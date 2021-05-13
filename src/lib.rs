//! A CLI tool for querying the [reproducibility] status of the Arch Linux packages
//! using data from a [rebuilderd] instance such as [reproducible.archlinux.org].
//!
//! [reproducibility]: https://reproducible-builds.org/
//! [rebuilderd]: https://wiki.archlinux.org/index.php/Rebuilderd
//! [reproducible.archlinux.org]: https://reproducible.archlinux.org/

pub mod archweb;
pub mod args;
pub mod error;
pub mod package;

use alpm::Alpm;
use archweb::{ArchwebPackage, SearchResult, ARCHWEB_ENDPOINT};
use args::Args;
use colored::*;
use console::{Style, Term};
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Confirm, Select};
use error::ReproStatusError;
use futures::{executor, future};
use package::{LogType, Package};
use rebuilderd_common::{PkgRelease as RebuilderdPackage, Status};
use reqwest::Client as HttpClient;
use std::convert::TryInto;
use std::fs;
use std::io::{self, Write};
use std::process::Command;

/// User agent that will be used for requests.
static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

/// Fetches the packages of the specified maintainer from archlinux.org
async fn fetch_archweb_packages<'a>(
    client: &'a HttpClient,
    maintainer: &'a str,
) -> Result<Vec<ArchwebPackage>, ReproStatusError> {
    let url = format!("{}/?maintainer={}", ARCHWEB_ENDPOINT, maintainer);
    let response = client
        .get(&url)
        .send()
        .await?
        .json::<SearchResult>()
        .await?;
    let mut results = response.results;
    if let (Some(page), Some(num_pages)) = (response.page, response.num_pages) {
        for page in (page + 1)..=num_pages {
            results.extend(
                client
                    .get(&format!("{}&page={}", &url, page))
                    .send()
                    .await?
                    .json::<SearchResult>()
                    .await?
                    .results,
            )
        }
    }
    Ok(results)
}

/// Fetches the packages from the specified rebuilderd instance.
async fn fetch_rebuilderd_packages<'a>(
    client: &'a HttpClient,
    rebuilder: &'a str,
) -> Result<Vec<RebuilderdPackage>, ReproStatusError> {
    Ok(client
        .get(format!("{}/api/v0/pkgs/list?distro=archlinux", rebuilder))
        .send()
        .await?
        .json()
        .await?)
}

/// Fetches the package logs from the specified rebuilderd instance.
async fn fetch_rebuilderd_logs<'a>(
    client: &'a HttpClient,
    rebuilder: &'a str,
    build_id: i32,
    log_type: LogType,
) -> Result<String, ReproStatusError> {
    Ok(client
        .get(&format!(
            "{}/api/v0/builds/{}/{}",
            rebuilder,
            build_id,
            match log_type {
                LogType::Build => "log",
                LogType::Diffoscope => "diffoscope",
            }
        ))
        .send()
        .await?
        .text()
        .await?)
}

/// Presents an interactive selection dialog for providing
/// options for selecting a package and operation.
///
/// Possible operations are: showing the build logs and diffoscope.
/// It fetches the logs from rebuilderd and shows them via specified pager.
async fn inspect_packages<'a>(
    mut packages: Vec<Package>,
    default_selection: i32,
    client: &'a HttpClient,
    args: &'a Args,
) -> Result<Option<i32>, ReproStatusError> {
    if let Some(filter) = args.filter {
        packages = packages
            .into_iter()
            .filter(|pkg| pkg.status == filter)
            .collect();
    }
    let items = packages
        .iter()
        .map(|pkg| pkg.to_string())
        .collect::<Vec<String>>();
    if let Some(index) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select package to inspect")
        .default(default_selection.try_into().unwrap_or_default())
        .paged(true)
        .items(&items)
        .interact_on_opt(&Term::stderr())
        .map_or(None, |v| v)
    {
        let operation = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select operation")
            .default(0)
            .items(&["show build log", "show diffoscope", "show package info"])
            .interact_on_opt(&Term::stderr())?;
        if let Some(2) = operation {
            println!("\n{}", packages[index].data);
            Confirm::with_theme(&ColorfulTheme {
                hint_style: Style::new().for_stderr().hidden(),
                prompt_prefix: console::style("❯".to_string()).for_stderr().green(),
                prompt_suffix: console::style(String::new()).for_stderr().hidden(),
                ..ColorfulTheme::default()
            })
            .with_prompt("Press Enter to continue")
            .wait_for_newline(true)
            .show_default(false)
            .interact_on_opt(&Term::stderr())?;
            return Ok(Some(index.try_into().unwrap_or_default()));
        }
        let log_type = match operation {
            Some(0) => LogType::Build,
            _ => LogType::Diffoscope,
        };
        let path = packages[index].get_log_path(log_type, args.cache_dir.as_ref().cloned())?;
        if path.exists() {
            log::debug!("Hit cache for {:?}", path);
        } else {
            let logs =
                fetch_rebuilderd_logs(client, &args.rebuilderd, packages[index].build_id, log_type)
                    .await?;
            fs::write(&path, logs)?;
        }
        match Command::new(&args.pager).arg(path).spawn() {
            Ok(mut child) => {
                child.wait()?;
                Ok(Some(index.try_into().unwrap_or_default()))
            }
            Err(e) => Err(ReproStatusError::IoError(e)),
        }
    } else {
        Ok(None)
    }
}

/// Prints the status of the packages to the specified output.
fn print_results<Output: Write>(
    packages: Vec<Package>,
    is_local: bool,
    filter: Option<Status>,
    output: &mut Output,
) -> Result<(), ReproStatusError> {
    let mut negatives = 0;
    for pkg in &packages {
        if pkg.status == Status::Bad {
            negatives += 1;
        }
        if let Some(filter) = filter {
            if pkg.status != filter {
                continue;
            }
        }
        writeln!(
            output,
            "[{}] {}",
            match pkg.status {
                Status::Good => "+".green(),
                Status::Bad => "-".red(),
                Status::Unknown => "?".yellow(),
            },
            pkg
        )?;
    }
    if packages.is_empty() {
        log::warn!("No packages found.")
    } else {
        match negatives {
            0 => log::info!("All packages are reproducible!"),
            1 => log::info!(
                "1/{} package is {} reproducible. Almost there.",
                packages.len(),
                "not".bold(),
            ),
            _ => log::info!(
                "{}/{} packages are {} reproducible.",
                negatives,
                packages.len(),
                "not".bold(),
            ),
        }
        log::info!(
            "Your {} {:.1}% reproducible.",
            String::from(if is_local {
                "system is"
            } else {
                "packages are"
            }),
            ((packages.len() - negatives) as f64 / packages.len() as f64) * 100.
        )
    }
    Ok(())
}

/// Returns the reproducibility results of an individual maintainer's packages.
fn get_maintainer_packages<'a>(
    maintainer: &'a str,
    client: &'a HttpClient,
    args: &'a Args,
) -> Result<Vec<Package>, ReproStatusError> {
    let (archweb, rebuilderd) = executor::block_on(future::try_join(
        fetch_archweb_packages(&client, maintainer),
        fetch_rebuilderd_packages(&client, &args.rebuilderd),
    ))?;
    let mut packages = Vec::new();
    for pkg in archweb {
        packages.push(match rebuilderd.iter().find(|p| p.name == pkg.pkgname) {
            Some(p) => Package {
                data: pkg,
                status: p.status,
                build_id: p.build_id.unwrap_or_default(),
            },
            None => Package {
                data: pkg,
                status: Status::Unknown,
                build_id: 0,
            },
        })
    }
    Ok(packages)
}

/// Returns the reproducibility results of the locally installed packages.
fn get_user_packages<'a>(
    client: &'a HttpClient,
    args: &'a Args,
) -> Result<Vec<Package>, ReproStatusError> {
    let rebuilderd = executor::block_on(fetch_rebuilderd_packages(&client, &args.rebuilderd))?;
    log::debug!("querying packages from local database: {}", args.dbpath);
    let pacman = Alpm::new("/", &args.dbpath)?;
    let mut packages = Vec::new();
    for pkg in pacman.localdb().pkgs() {
        packages.push(match rebuilderd.iter().find(|p| p.name == pkg.name()) {
            Some(p) => Package {
                data: ArchwebPackage::from(pkg),
                status: p.status,
                build_id: p.build_id.unwrap_or_default(),
            },
            None => Package {
                data: ArchwebPackage::from(pkg),
                status: Status::Unknown,
                build_id: 0,
            },
        });
    }
    Ok(packages)
}

/// Runs `arch-repro-status` and prints the results/shows dialogues.
pub fn run(args: Args) -> Result<(), ReproStatusError> {
    let client = HttpClient::builder().user_agent(APP_USER_AGENT).build()?;
    let packages = if let Some(ref maintainer) = args.maintainer {
        get_maintainer_packages(&maintainer, &client, &args)
    } else {
        get_user_packages(&client, &args)
    }?;
    if args.inspect {
        ctrlc::set_handler(move || Term::stdout().show_cursor().expect("failed to show cursor"))?;
        let mut default_selection = Some(0);
        while let Some(selection) = default_selection {
            default_selection = executor::block_on(inspect_packages(
                packages.clone(),
                selection,
                &client,
                &args,
            ))?;
        }
        Ok(())
    } else {
        print_results(
            packages,
            args.maintainer.is_none(),
            args.filter,
            &mut io::stdout(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use pretty_assertions::assert_eq;
    use std::str;

    /// Rebuilderd instance to use for testing.
    const REBUILDERD_URL: &str = "https://reproducible.archlinux.org";

    #[tokio::test]
    async fn test_fetch_archweb_packages() -> Result<()> {
        let client = HttpClient::new();
        assert_eq!(0, fetch_archweb_packages(&client, "xyz").await?.len());
        // assuming jelle will maintain packages for eternity <3
        assert!(!fetch_archweb_packages(&client, "jelle").await?.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_fetch_rebuilderd_packages() -> Result<()> {
        let client = HttpClient::new();
        assert!(!fetch_rebuilderd_packages(&client, REBUILDERD_URL)
            .await?
            .is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_fetch_rebuilderd_logs() -> Result<()> {
        let client = HttpClient::new();
        assert_eq!(
            "Not found\n",
            fetch_rebuilderd_logs(&client, REBUILDERD_URL, 0, LogType::Build).await?
        );
        Ok(())
    }

    #[test]
    fn test_print_results() -> Result<()> {
        let mut output = Vec::new();
        print_results(
            vec![
                Package {
                    data: ArchwebPackage {
                        pkgname: String::from("test"),
                        pkgver: String::from("0.1"),
                        pkgrel: String::from("2"),
                        ..ArchwebPackage::default()
                    },
                    status: Status::Good,
                    build_id: 0,
                },
                Package {
                    data: ArchwebPackage {
                        pkgname: String::from("xyz"),
                        pkgver: String::from("0.4"),
                        pkgrel: String::from("1"),
                        ..ArchwebPackage::default()
                    },
                    status: Status::Bad,
                    build_id: 0,
                },
            ],
            None,
            &mut output,
        )?;
        assert_eq!(
            "[+] test 0.1-2 GOOD \n[-] xyz 0.4-1 BAD  \n",
            str::from_utf8(&output)?
        );
        Ok(())
    }
}
