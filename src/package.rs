//! Common package data.

use crate::archweb::ArchwebPackage;
use crate::error::ReproStatusError;
use rebuilderd_common::Status;
use std::env;
use std::fmt;
use std::fs;
use std::path::PathBuf;

/// Type of logs that rebuilderd provides.
#[derive(Debug, Copy, Clone)]
pub enum LogType {
    /// Build logs.
    Build,
    /// Diffoscope logs.
    Diffoscope,
}

impl fmt::Display for LogType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

/// Package that consists of archweb data
/// and essential information from rebuilderd.
#[derive(Debug, Clone)]
pub struct Package {
    /// Package data from the Arch Linux website.
    pub data: ArchwebPackage,
    /// Reproducibility status of the package.
    pub status: Status,
    /// Rebuilderd build ID.
    pub build_id: i32,
}

impl Default for Package {
    fn default() -> Self {
        Self {
            data: ArchwebPackage::default(),
            status: Status::Unknown,
            build_id: 0,
        }
    }
}

impl fmt::Display for Package {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {}{}-{} {}",
            self.data.pkgname,
            if self.data.epoch != 0 {
                format!("{}:", self.data.epoch)
            } else {
                String::new()
            },
            self.data.pkgver,
            self.data.pkgrel,
            self.status.fancy()
        )
    }
}

impl Package {
    /// Returns the path to save logs based on the log type and build ID.
    pub fn get_log_path(&self, log_type: LogType) -> Result<PathBuf, ReproStatusError> {
        let path = env::temp_dir()
            .join(env!("CARGO_PKG_NAME"))
            .join(format!("{}_{}.log", self.build_id, log_type,));
        if !path.exists() {
            fs::create_dir_all(match path.parent() {
                Some(parent) => parent,
                None => path.as_path(),
            })?;
        }
        Ok(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_get_log_path() -> Result<()> {
        let package = Package {
            data: ArchwebPackage {
                pkgname: String::from("test"),
                pkgver: String::from("0.0.1"),
                pkgrel: String::from("1"),
                ..ArchwebPackage::default()
            },
            status: Status::Good,
            build_id: 0,
        };
        let path = package.get_log_path(LogType::Diffoscope)?;
        assert_eq!(
            format!("/tmp/{}/0_diffoscope.log", env!("CARGO_PKG_NAME")),
            path.to_string_lossy()
        );
        Ok(())
    }
}
