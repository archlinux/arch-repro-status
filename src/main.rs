use anyhow::Result;
use arch_repro_status::args::Args;
use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::from_args();
    let results = arch_repro_status::run(args)?;
    for res in results {
        println!("{}", res);
    }
    Ok(())
}
