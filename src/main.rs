use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let results = arch_check_repro::run()?;
    for res in results {
        println!("{}", res);
    }
    Ok(())
}
