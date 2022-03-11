use arch_repro_status::args::Args;
use clap::Parser;
use log::Level;
use std::env;

/// Levels of logging.
const LOG_LEVELS: &[Level] = &[Level::Warn, Level::Info, Level::Debug, Level::Trace];

#[tokio::main]
async fn main() {
    let args = Args::parse();
    if env::var_os("RUST_LOG").is_none() {
        let level_index = ((args.verbose + 1) * !args.quiet as u8) as usize;
        let level = LOG_LEVELS.get(level_index).unwrap_or(&Level::Trace);
        env::set_var("RUST_LOG", level.as_str());
    }
    pretty_env_logger::init();
    arch_repro_status::run(args).unwrap_or_else(|e| log::error!("{}", e))
}
