use std::{env, path::PathBuf, time::Duration};

/// Runtime settings, read once from the environment (see `.env.example`).
pub struct Config {
    pub port: u16,
    pub static_dir: PathBuf,
    pub nocodb_url: Option<String>,
    pub nocodb_token: Option<String>,
    pub nocodb_base_id: String,
    pub nocodb_team_table: String,
    pub nocodb_services_table: String,
    pub cache_ttl: Duration,
    pub enquiries_file: PathBuf,
    pub site_url: String,
}

fn var(key: &str) -> Option<String> {
    env::var(key).ok().map(|v| v.trim().to_string()).filter(|v| !v.is_empty())
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            port: var("PORT").and_then(|p| p.parse().ok()).unwrap_or(8080),
            static_dir: var("STATIC_DIR").map(PathBuf::from).unwrap_or_else(|| PathBuf::from("static")),
            // Accept either the server address or a full API URL pasted from NocoDB.
            nocodb_url: var("NOCODB_URL").map(|u| u.split("/api/").next().unwrap_or_default().trim_end_matches('/').to_string()),
            nocodb_token: var("NOCODB_API_TOKEN"),
            nocodb_base_id: var("NOCODB_BASE_ID").unwrap_or_else(|| "p1uzmrjrxr8s8x9".into()),
            nocodb_team_table: var("NOCODB_TEAM_TABLE").unwrap_or_else(|| "mly0l1dxwu1role".into()),
            nocodb_services_table: var("NOCODB_SERVICES_TABLE").unwrap_or_else(|| "moaw8ss95jywuuy".into()),
            cache_ttl: Duration::from_secs(var("NOCODB_CACHE_SECONDS").and_then(|s| s.parse().ok()).unwrap_or(0)),
            enquiries_file: var("ENQUIRIES_FILE").map(PathBuf::from).unwrap_or_else(|| PathBuf::from("data/enquiries.jsonl")),
            site_url: var("SITE_URL").unwrap_or_else(|| "https://alfaglobalwealth.com".into()).trim_end_matches('/').to_string(),
        }
    }
}
