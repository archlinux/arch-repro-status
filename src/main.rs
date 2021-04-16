use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let results = arch_repro_status::run()?;
    for res in results {
        println!("{}", res);
    }
    Ok(())
}
