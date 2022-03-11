use arch_repro_status::args::Args;
use clap::{ArgEnum, CommandFactory};
use clap_complete::Shell;
use std::env;

/// Shell completions can be created with `cargo run --bin completions`
/// in a directory specified by the environment variable OUT_DIR.
fn main() -> Result<(), std::io::Error> {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR is not set");
    let mut app = Args::command();
    for &shell in Shell::value_variants() {
        clap_complete::generate_to(shell, &mut app, env!("CARGO_PKG_NAME"), &out_dir)?;
    }
    println!("Completion scripts are generated in {:?}", out_dir);
    Ok(())
}
