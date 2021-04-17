//! Common package data.

use crate::archweb::ArchwebPackage;
use rebuilderd_common::Status;

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
