//! Command-line argument parser.

use clap::{AppSettings, Parser};
use rebuilderd_common::Status;
use std::path::PathBuf;

/// Command-line arguments to parse.
#[derive(Debug, Parser)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    global_setting = AppSettings::DeriveDisplayOrder,
    rename_all_env = "screaming-snake"
)]
pub struct Args {
    /// Disables logging.
    #[clap(short, long)]
    pub quiet: bool,
    /// Increases the logging verbosity.
    #[clap(short, long, parse(from_occurrences), alias = "debug")]
    pub verbose: u8,
    /// Checks all of the packages on the system.
    #[clap(short, long)]
    pub all: bool,
    /// Sets the username of the maintainer.
    #[clap(short, long, value_name = "MAINTAINER", env)]
    pub maintainer: Option<String>,
    /// Sets the address of the rebuilderd instance.
    #[clap(
        short,
        long,
        value_name = "URL",
        default_value = "https://reproducible.archlinux.org",
        env
    )]
    pub rebuilderd: String,
    /// Sets the path to the pacman database.
    #[clap(
        short = 'b',
        long,
        value_name = "PATH",
        default_value = "/var/lib/pacman",
        env
    )]
    pub dbpath: String,
    /// Sets the repositories to query.
    #[clap(
        long,
        value_name = "REPO",
        default_value = "core,extra,community,multilib",
        use_value_delimiter = true
    )]
    pub repos: Vec<String>,
    /// Sets the filter for package status.
    #[clap(
        short,
        long,
        value_name = "STATUS",
        possible_values = &["GOOD", "BAD", "UNKWN"],
        env
    )]
    pub filter: Option<Status>,
    /// Views the build log or diffoscope of the interactively selected package.
    #[clap(short, long)]
    pub inspect: bool,
    /// Sets the pager for viewing files.
    #[clap(short, long, value_name = "PAGER", default_value = "less", env)]
    pub pager: String,
    /// Sets the cache directory for log files.
    #[clap(short, long, value_name = "DIR", env)]
    pub cache_dir: Option<PathBuf>,
}
