use crate::archweb::{ArchwebPackage, SearchResult, ARCHWEB_ENDPOINT};
use crate::error::ReproStatusError;
use crate::package::LogType;
use rebuilderd_common::PkgRelease as RebuilderdPackage;
use reqwest::Client as HttpClient;

/// Fetches the packages of the specified maintainer from archlinux.org
pub async fn fetch_archweb_packages<'a>(
    client: &'a HttpClient,
    maintainer: &'a str,
) -> Result<Vec<ArchwebPackage>, ReproStatusError> {
    let url = format!("{}/?maintainer={}", ARCHWEB_ENDPOINT, maintainer);
    let response = client
        .get(&url)
        .send()
        .await?
        .json::<SearchResult>()
        .await?;
    let mut results = response.results;
    if let (Some(page), Some(num_pages)) = (response.page, response.num_pages) {
        for page in (page + 1)..=num_pages {
            results.extend(
                client
                    .get(&format!("{}&page={}", &url, page))
                    .send()
                    .await?
                    .json::<SearchResult>()
                    .await?
                    .results,
            )
        }
    }
    Ok(results)
}

/// Fetches the packages from the specified rebuilderd instance.
pub async fn fetch_rebuilderd_packages<'a>(
    client: &'a HttpClient,
    rebuilder: &'a str,
) -> Result<Vec<RebuilderdPackage>, ReproStatusError> {
    Ok(client
        .get(format!("{}/api/v0/pkgs/list?distro=archlinux", rebuilder))
        .send()
        .await?
        .json()
        .await?)
}

/// Fetches the package logs from the specified rebuilderd instance.
pub async fn fetch_rebuilderd_logs<'a>(
    client: &'a HttpClient,
    rebuilder: &'a str,
    build_id: i32,
    log_type: LogType,
) -> Result<String, ReproStatusError> {
    Ok(client
        .get(&format!(
            "{}/api/v0/builds/{}/{}",
            rebuilder,
            build_id,
            match log_type {
                LogType::Build => "log",
                LogType::Diffoscope => "diffoscope",
            }
        ))
        .send()
        .await?
        .text()
        .await?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use pretty_assertions::assert_eq;

    /// Rebuilderd instance to use for testing.
    const REBUILDERD_URL: &str = "https://reproducible.archlinux.org";

    #[tokio::test]
    async fn test_fetch_archweb_packages() -> Result<()> {
        let client = HttpClient::new();
        assert_eq!(0, fetch_archweb_packages(&client, "xyz").await?.len());
        assert!(!fetch_archweb_packages(&client, "jelle").await?.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_fetch_rebuilderd_packages() -> Result<()> {
        let client = HttpClient::new();
        assert!(!fetch_rebuilderd_packages(&client, REBUILDERD_URL)
            .await?
            .is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_fetch_rebuilderd_logs() -> Result<()> {
        let client = HttpClient::new();
        assert_eq!(
            "Not found\n",
            fetch_rebuilderd_logs(&client, REBUILDERD_URL, 0, LogType::Build).await?
        );
        Ok(())
    }
}
