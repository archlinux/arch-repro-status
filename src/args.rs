//! Command-line argument parser.

use rebuilderd_common::Status;
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
    /// Sets the filter for package status.
    #[structopt(short, long, env = "FILTER", value_name = "STATUS")]
    pub filter: Option<Status>,
    /// Views the build log or diffoscope of the interactively selected package.
    #[structopt(short, long)]
    pub inspect: bool,
    /// Sets the pager for viewing files.
    #[structopt(
        short,
        long,
        env = "PAGER",
        value_name = "PAGER",
        default_value = "less"
    )]
    pub pager: String,
}
