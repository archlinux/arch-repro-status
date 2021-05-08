use arch_repro_status::args::Args;
use std::env;
use structopt::StructOpt;

#[tokio::main]
async fn main() {
    let args = Args::from_args();
    if args.debug {
        env::set_var("RUST_LOG", "debug");
    } else if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();
    arch_repro_status::run(args).unwrap_or_else(|e| log::error!("{}", e))
}
