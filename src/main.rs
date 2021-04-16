use arch_repro_status::args::Args;
use structopt::StructOpt;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let args = Args::from_args();
    match arch_repro_status::run(args) {
        Ok(results) => {
            for res in results {
                println!("{}", res);
            }
        }
        Err(e) => {
            log::error!("{}", e);
        }
    }
}
