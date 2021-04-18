use std::{env, str::FromStr};
use structopt::clap::Shell;
use structopt::StructOpt;

use arch_repro_status::args::Args;

/// Shell completions can be created with `cargo run --bin completions`
/// in a directory specified by the environment variable OUT_DIR.
fn main() {
    //  https://doc.rust-lang.org/cargo/reference/environment-variables.html
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let mut app = Args::clap();
    for variant in Shell::variants()
        .iter()
        .filter_map(|v| Shell::from_str(v).ok())
    {
        app.gen_completions(env!("CARGO_PKG_NAME"), variant, &out_dir);
    }
    println!("Completion scripts generated in '{}'", out_dir);
}
