use arch_repro_status::args::Args;
use colored::*;
use rebuilderd_common::Status;
use std::env;
use structopt::StructOpt;

#[tokio::main]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();
    match arch_repro_status::run(Args::from_args()) {
        Ok(results) => {
            let mut negatives = 0;
            for (status, pkg) in results {
                if status == Status::Bad {
                    negatives += 1;
                }
                println!(
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
                );
            }
            match negatives {
                0 => log::info!("All packages are reproducible!"),
                1 => log::info!("1 package is not reproducible."),
                _ => log::info!("{} packages are not reproducible.", negatives),
            }
        }
        Err(e) => {
            log::error!("{}", e);
        }
    }
}
