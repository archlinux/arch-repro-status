//! Command-line argument parser.

use structopt::clap::AppSettings;
use structopt::StructOpt;

/// Command-line arguments to parse.
#[derive(Debug, StructOpt)]
#[structopt(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    global_settings(&[
        AppSettings::ColorAuto,
        AppSettings::ColoredHelp,
        AppSettings::DeriveDisplayOrder,
    ])
)]
pub struct Args {
    /// Sets the username of the maintainer.
    #[structopt(short, long, env = "MAINTAINER", value_name = "MAINTAINER")]
    pub maintainer: String,
    /// Sets the address of the rebuilderd instance.
    #[structopt(
        short,
        long,
        env = "REBUILDERD",
        value_name = "URL",
        default_value = "https://reproducible.archlinux.org"
    )]
    pub rebuilderd: String,
}
