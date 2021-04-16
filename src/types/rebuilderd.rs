#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Package {
    pub name: String,
    pub version: String,
    pub status: String,
    pub distro: String,
    pub suite: String,
    pub architecture: String,
    pub url: String,
    #[serde(rename = "build_id")]
    pub build_id: Option<i64>,
    #[serde(rename = "built_at")]
    pub built_at: Option<String>,
    pub attestation: ::serde_json::Value,
    #[serde(rename = "next_retry")]
    pub next_retry: Option<String>,
}
