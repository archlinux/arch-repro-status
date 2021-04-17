//! Finds the reproducibility status of the packages of a maintainer
//! using package data from [archlinux.org] and a [rebuilderd] instance.
//!
//! [archlinux.org]: https://archlinux.org/packages
//! [rebuilderd]: https://wiki.archlinux.org/index.php/Rebuilderd

pub mod args;
pub mod error;
mod types;

use args::Args;
use colored::*;
use error::ReproStatusError;
use futures::executor;
use futures::future;
use rebuilderd_common::Status;
use reqwest::Client as HttpClient;
use std::io::{self, Write};
use types::archweb::{Package as ArchwebPackage, SearchResult};
use types::rebuilderd::Package as RebuilderdPackage;

/// JSON endpoint to use for searching packages.
const ARCHWEB_ENDPOINT: &str = "https://archlinux.org/packages/search/json";

/// Fetches the packages of the specified maintainer from archlinux.org
async fn fetch_archweb_packages<'a>(
    client: &'a HttpClient,
    maintainer: &'a str,
) -> Result<Vec<ArchwebPackage>, ReproStatusError> {
    Ok(client
        .get(&format!("{}/?maintainer={}", ARCHWEB_ENDPOINT, maintainer))
        .send()
        .await?
        .json::<SearchResult>()
        .await?
        .results)
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

/// Prints the status of the packages to the specified output.
fn print_results<Output: Write>(
    results: Vec<(Status, ArchwebPackage)>,
    filter: Option<Status>,
    output: &mut Output,
) -> Result<(), ReproStatusError> {
    let mut negatives = 0;
    for (status, pkg) in results {
        if status == Status::Bad {
            negatives += 1;
        }
        if let Some(filter) = filter {
            if status != filter {
                continue;
            }
        }
        writeln!(
            output,
            "[{}] {} {}-{} {}",
            match status {
                Status::Good => "+".green(),
                Status::Bad => "-".red(),
                Status::Unknown => "?".yellow(),
            },
            pkg.pkgname,
            pkg.pkgver,
            pkg.pkgrel,
            status.fancy()
        )?;
    }
    match negatives {
        0 => log::info!("All packages are reproducible!"),
        1 => log::info!("1 package is not reproducible."),
        _ => log::info!("{} packages are not reproducible.", negatives),
    }
    Ok(())
}

/// Runs `arch-repro-status` and prints the results.
pub fn run(args: Args) -> Result<(), ReproStatusError> {
    let client = HttpClient::builder().build()?;
    let (archweb, rebuilderd) = executor::block_on(future::try_join(
        fetch_archweb_packages(&client, &args.maintainer),
        fetch_rebuilderd_packages(&client, &args.rebuilderd),
    ))?;
    let mut results = Vec::new();
    for pkg in archweb {
        results.push((
            match rebuilderd.iter().find(|p| p.name == pkg.pkgname) {
                Some(p) => p.status,
                None => Status::Unknown,
            },
            pkg,
        ))
    }
    print_results(results, args.filter, &mut io::stdout())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use pretty_assertions::assert_eq;
    use std::str;

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
        assert!(
            !fetch_rebuilderd_packages(&client, "https://reproducible.archlinux.org")
                .await?
                .is_empty()
        );
        Ok(())
    }

    #[test]
    fn test_print_results() -> Result<()> {
        let mut output = Vec::new();
        print_results(
            vec![
                (
                    Status::Good,
                    ArchwebPackage {
                        pkgname: String::from("test"),
                        pkgver: String::from("0.1"),
                        pkgrel: String::from("2"),
                        ..ArchwebPackage::default()
                    },
                ),
                (
                    Status::Bad,
                    ArchwebPackage {
                        pkgname: String::from("xyz"),
                        pkgver: String::from("0.4"),
                        pkgrel: String::from("1"),
                        ..ArchwebPackage::default()
                    },
                ),
            ],
            None,
            &mut output,
        )?;
        assert_eq!(
            vec![
                "[\u{1b}[32m+\u{1b}[0m] test 0.1-2 \u{1b}[32mGOOD \u{1b}[0m",
                "[\u{1b}[31m-\u{1b}[0m] xyz 0.4-1 \u{1b}[31mBAD  \u{1b}[0m\n"
            ]
            .join("\n"),
            str::from_utf8(&output)?
        );
        Ok(())
    }
}
