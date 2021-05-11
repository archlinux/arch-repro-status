//! Package data from the Arch Linux website.

use bytesize::ByteSize;
use colored::*;
use std::convert::TryInto;
use std::fmt;

/// JSON endpoint to use for searching packages.
pub const ARCHWEB_ENDPOINT: &str = "https://archlinux.org/packages/search/json";

/// Search result from archlinux.org
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    pub version: i64,
    pub limit: i64,
    pub valid: bool,
    pub results: Vec<ArchwebPackage>,
    #[serde(rename = "num_pages")]
    pub num_pages: Option<i64>,
    pub page: Option<i64>,
}

/// Package data that archlinux.org provides.
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchwebPackage {
    pub pkgname: String,
    pub pkgbase: String,
    pub repo: String,
    pub arch: String,
    pub pkgver: String,
    pub pkgrel: String,
    pub epoch: i64,
    pub pkgdesc: String,
    pub url: String,
    pub filename: String,
    #[serde(rename = "compressed_size")]
    pub compressed_size: i64,
    #[serde(rename = "installed_size")]
    pub installed_size: i64,
    #[serde(rename = "build_date")]
    pub build_date: String,
    #[serde(rename = "last_update")]
    pub last_update: String,
    #[serde(rename = "flag_date")]
    pub flag_date: Option<String>,
    pub maintainers: Vec<String>,
    pub packager: String,
    pub groups: Vec<::serde_json::Value>,
    pub licenses: Vec<String>,
    pub conflicts: Vec<::serde_json::Value>,
    pub provides: Vec<::serde_json::Value>,
    pub replaces: Vec<String>,
    pub depends: Vec<String>,
    pub optdepends: Vec<String>,
    pub makedepends: Vec<String>,
    pub checkdepends: Vec<::serde_json::Value>,
}

impl fmt::Display for ArchwebPackage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&format!("\t{:16}: {}\n", "Name".cyan(), self.pkgname))?;
        f.write_str(&format!(
            "\t{:16}: {}{}-{}\n",
            "Version".cyan(),
            if self.epoch != 0 {
                format!("{}:", self.epoch)
            } else {
                String::new()
            },
            self.pkgver,
            self.pkgrel
        ))?;
        f.write_str(&format!("\t{:16}: {}\n", "Architecture".cyan(), self.arch))?;
        f.write_str(&format!("\t{:16}: {}\n", "Repository".cyan(), self.repo))?;
        f.write_str(&format!(
            "\t{:16}: {}\n",
            "Description".cyan(),
            self.pkgdesc
        ))?;
        f.write_str(&format!("\t{:16}: {}\n", "Upstream URL".cyan(), self.url))?;
        f.write_str(&format!(
            "\t{:16}: {}\n",
            "License(s)".cyan(),
            self.licenses.join(", ")
        ))?;
        f.write_str(&format!(
            "\t{:16}: {}\n",
            "Maintainer(s)".cyan(),
            self.maintainers.join(", ")
        ))?;
        f.write_str(&format!(
            "\t{:16}: {}\n",
            "Package Size".cyan(),
            ByteSize(self.compressed_size.try_into().unwrap_or_default()).to_string_as(true)
        ))?;
        f.write_str(&format!(
            "\t{:16}: {}\n",
            "Installed Size".cyan(),
            ByteSize(self.installed_size.try_into().unwrap_or_default()).to_string_as(true)
        ))?;
        f.write_str(&format!(
            "\t{:16}: {}\n",
            "Last Packager".cyan(),
            self.packager
        ))?;
        f.write_str(&format!(
            "\t{:16}: {}\n",
            "Build Date".cyan(),
            self.build_date
        ))?;
        f.write_str(&format!(
            "\t{:16}: {}\n",
            "Last Updated".cyan(),
            self.last_update
        ))?;
        if let Some(date) = &self.flag_date {
            f.write_str(&format!("\t{:16}: {}\n", "Flag Date".red(), date))?;
        }
        f.write_str(&format!(
            "\t{:16}: https://archlinux.org/packages/{}/{}/{}/\n",
            "Package URL".cyan(),
            self.repo,
            self.arch,
            self.pkgbase
        ))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_archweb_package_info() {
        let package = ArchwebPackage {
            pkgname: String::from("test"),
            pkgbase: String::from("test_"),
            repo: String::from("foo"),
            arch: String::from("i686"),
            pkgver: String::from("1.2.3"),
            pkgrel: String::from("10"),
            epoch: 2,
            pkgdesc: String::from("A test package"),
            url: String::from("example.com"),
            compressed_size: 660662,
            installed_size: 678620,
            build_date: String::from("2000"),
            last_update: String::from("2000"),
            flag_date: Some(String::from("today")),
            maintainers: vec![String::from("orhun"), String::from("nuhro")],
            packager: String::from("orhun"),
            licenses: vec![String::from("MIT"), String::from("GPL")],
            ..ArchwebPackage::default()
        };
        println!("{}", package);
        assert_eq!(
            "\tName            : test\
            \n\tVersion         : 2:1.2.3-10\
            \n\tArchitecture    : i686\
            \n\tRepository      : foo\
            \n\tDescription     : A test package\
            \n\tUpstream URL    : example.com\
            \n\tLicense(s)      : MIT, GPL\
            \n\tMaintainer(s)   : orhun, nuhro\
            \n\tPackage Size    : 645.2 kiB\
            \n\tInstalled Size  : 662.7 kiB\
            \n\tLast Packager   : orhun\
            \n\tBuild Date      : 2000\
            \n\tLast Updated    : 2000\
            \n\tFlag Date       : today\
            \n\tPackage URL     : https://archlinux.org/packages/foo/i686/test_/\n",
            package.to_string()
        );
    }
}
