//! Live NocoDB client for the Team and Services tables.
//!
//! Every page view reads the tables straight from NocoDB. Concurrent requests
//! share one fetch, and `NOCODB_CACHE_SECONDS` can add a short cache if the
//! database ever needs shielding. If NocoDB cannot be reached, the last list
//! it returned is shown; before the first success the pages show an empty
//! state. There is no bundled copy of the data.

use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use serde_json::{Map, Value};
use tokio::sync::{Mutex, RwLock};

use crate::{
    config::Config,
    models::{self, Service, TeamMember},
};

const MAX_PAGES: usize = 20;
/// After a failed fetch, visitors get the last good list for this long
/// instead of each waiting on an unreachable database.
const ERROR_BACKOFF: Duration = Duration::from_secs(15);

struct Slot<T> {
    data: Arc<Vec<T>>,
    fetched_at: Option<Instant>,
    loaded: bool,
    last_error: Option<String>,
}

/// One table: its last result, plus a gate so that only one fetch runs at a time.
struct Table<T> {
    id: String,
    label: &'static str,
    slot: RwLock<Slot<T>>,
    gate: Mutex<()>,
    parse: fn(&Map<String, Value>, &str, usize) -> Option<T>,
    sort: fn(&mut [T]),
}

impl<T> Table<T> {
    fn new(id: String, label: &'static str, parse: fn(&Map<String, Value>, &str, usize) -> Option<T>, sort: fn(&mut [T])) -> Self {
        Self {
            id,
            label,
            slot: RwLock::new(Slot { data: Arc::new(Vec::new()), fetched_at: None, loaded: false, last_error: None }),
            gate: Mutex::new(()),
            parse,
            sort,
        }
    }
}

pub struct NocoDb {
    http: reqwest::Client,
    base_url: Option<String>,
    token: Option<String>,
    base_id: String,
    ttl: Duration,
    team: Table<TeamMember>,
    services: Table<Service>,
}

impl NocoDb {
    pub fn new(cfg: &Config) -> Self {
        let http = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .connect_timeout(Duration::from_secs(5))
            .pool_idle_timeout(Duration::from_secs(90))
            .user_agent(concat!("alfa-site/", env!("CARGO_PKG_VERSION")))
            .build()
            .expect("could not build the HTTP client");
        Self {
            http,
            base_url: cfg.nocodb_url.clone(),
            token: cfg.nocodb_token.clone(),
            base_id: cfg.nocodb_base_id.clone(),
            ttl: cfg.cache_ttl,
            team: Table::new(cfg.nocodb_team_table.clone(), "team members", models::team_from_row, models::sort_team),
            services: Table::new(cfg.nocodb_services_table.clone(), "services", models::service_from_row, models::sort_services),
        }
    }

    pub fn is_configured(&self) -> bool {
        self.base_url.is_some() && self.token.is_some()
    }

    /// Opens the connection to NocoDB early so the first visitor is quicker.
    pub fn warm_up(self: &Arc<Self>) {
        let me = self.clone();
        tokio::spawn(async move {
            let _ = tokio::join!(me.team(), me.services());
        });
    }

    pub async fn team(&self) -> Arc<Vec<TeamMember>> {
        self.load(&self.team).await
    }

    pub async fn services(&self) -> Arc<Vec<Service>> {
        self.load(&self.services).await
    }

    /// Live state of both tables, for `/api/status`.
    pub async fn status(&self) -> Value {
        async fn one<T>(t: &Table<T>) -> Value {
            let s = t.slot.read().await;
            serde_json::json!({
                "loaded": s.loaded,
                "count": s.data.len(),
                "seconds_since_fetch": s.fetched_at.map(|f| f.elapsed().as_secs()),
                "last_error": s.last_error,
            })
        }
        serde_json::json!({
            "configured": self.is_configured(),
            "cache_seconds": self.ttl.as_secs(),
            "team": one(&self.team).await,
            "services": one(&self.services).await,
        })
    }

    async fn load<T>(&self, table: &Table<T>) -> Arc<Vec<T>> {
        if !self.is_configured() {
            return Arc::new(Vec::new());
        }
        let asked = Instant::now();
        {
            let s = table.slot.read().await;
            let hold = if s.last_error.is_some() { self.ttl.max(ERROR_BACKOFF) } else { self.ttl };
            if s.fetched_at.is_some_and(|f| f.elapsed() < hold) {
                return s.data.clone();
            }
        }
        let _gate = table.gate.lock().await;
        {
            // Another request fetched while this one waited: use its result.
            let s = table.slot.read().await;
            if s.fetched_at.is_some_and(|f| f >= asked) {
                return s.data.clone();
            }
        }
        let base = self.base_url.clone().unwrap_or_default();
        let result = self.records(&table.id).await.map(|rows| {
            let mut v: Vec<T> = rows.iter().enumerate().filter_map(|(i, r)| (table.parse)(r, &base, i)).collect();
            (table.sort)(&mut v);
            v
        });
        let mut s = table.slot.write().await;
        match result {
            Ok(items) => {
                tracing::debug!("loaded {} {} from NocoDB", items.len(), table.label);
                s.data = Arc::new(items);
                s.loaded = true;
                s.last_error = None;
            }
            Err(e) => {
                tracing::warn!("could not load {} from NocoDB: {e}", table.label);
                s.last_error = Some(e);
            }
        }
        s.fetched_at = Some(Instant::now());
        s.data.clone()
    }

    /// Fetches every record of a table, following NocoDB's pagination.
    async fn records(&self, table: &str) -> Result<Vec<Map<String, Value>>, String> {
        let base = self.base_url.as_deref().ok_or("NOCODB_URL is not set")?;
        let token = self.token.as_deref().ok_or("NOCODB_API_TOKEN is not set")?;
        let mut url = format!("{base}/api/v3/data/{}/{table}/records?pageSize=100", self.base_id);
        let mut out = Vec::new();

        for _ in 0..MAX_PAGES {
            let resp = self
                .http
                .get(&url)
                .header("xc-token", token)
                .header("accept", "application/json")
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let status = resp.status();
            if !status.is_success() {
                return Err(format!("HTTP {status} from {}", url.replace(base, "")));
            }
            let body: Value = resp.json().await.map_err(|e| e.to_string())?;

            // v3 answers { records: [{ id, fields: {...} }], next }, v2 answers { list: [...] }.
            let rows = body.get("records").or_else(|| body.get("list")).and_then(Value::as_array).cloned().unwrap_or_default();
            for row in rows {
                let fields = match row.get("fields") {
                    Some(Value::Object(f)) => f.clone(),
                    _ => row.as_object().cloned().unwrap_or_default(),
                };
                out.push(fields);
            }

            match body.get("next").and_then(Value::as_str).filter(|n| !n.is_empty()) {
                Some(next) if next.starts_with("http") => url = next.to_string(),
                Some(next) if next.starts_with('/') => url = format!("{base}{next}"),
                _ => break,
            }
        }
        Ok(out)
    }
}
