//! Command-line argument parser.

use rebuilderd_common::Status;
use std::path::PathBuf;
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
    ]),
    rename_all_env = "screaming-snake"
)]
pub struct Args {
    /// Disables logging
    #[structopt(short, long)]
    pub quiet: bool,
    /// Increases the logging verbosity
    #[structopt(short, long, parse(from_occurrences), alias = "debug")]
    pub verbose: u8,
    /// Sets the username of the maintainer.
    #[structopt(short, long, value_name = "MAINTAINER", env)]
    pub maintainer: Option<String>,
    /// Sets the address of the rebuilderd instance.
    #[structopt(
        short,
        long,
        value_name = "URL",
        default_value = "https://reproducible.archlinux.org",
        env
    )]
    pub rebuilderd: String,
    /// Sets the path to the pacman database.
    #[structopt(
        short = "b",
        long,
        value_name = "PATH",
        default_value = "/var/lib/pacman",
        env
    )]
    pub dbpath: String,
    /// Sets the repositories to query.
    #[structopt(
        long,
        value_name = "REPO",
        default_value = "core,extra,community,multilib",
        use_delimiter = true
    )]
    pub repos: Vec<String>,
    /// Sets the filter for package status.
    #[structopt(
        short,
        long,
        value_name = "STATUS",
        possible_values = &["GOOD", "BAD", "UNKWN"],
        env
    )]
    pub filter: Option<Status>,
    /// Views the build log or diffoscope of the interactively selected package.
    #[structopt(short, long)]
    pub inspect: bool,
    /// Sets the pager for viewing files.
    #[structopt(short, long, value_name = "PAGER", default_value = "less", env)]
    pub pager: String,
    /// Sets the cache directory for log files.
    #[structopt(short, long, value_name = "DIR", env)]
    pub cache_dir: Option<PathBuf>,
}
