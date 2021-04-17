use arch_repro_status::args::Args;
use std::env;
use structopt::StructOpt;

#[tokio::main]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();
    let args = Args::from_args();
    arch_repro_status::run(args).unwrap_or_else(|e| log::error!("{}", e))
}
