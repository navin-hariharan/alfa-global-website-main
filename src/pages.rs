//! Page handlers and their Askama templates.

use std::sync::Arc;

use askama::Template;
use axum::{
    extract::{Path, State},
    http::{header, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::get,
    Json, Router,
};

use crate::{
    content::{self, Company, COMPANIES},
    icons,
    models::{Service, TeamMember},
    AppState,
};

pub const ASSET_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Shared by every page: SEO fields, the active path and navigation data.
pub struct PageMeta {
    pub title: String,
    pub description: String,
    pub path: String,
    pub canonical: String,
    pub companies: &'static [Company],
    pub v: &'static str,
}

impl PageMeta {
    pub fn new(state: &AppState, title: &str, description: &str, path: &str) -> Self {
        Self {
            title: title.to_string(),
            description: description.to_string(),
            path: path.to_string(),
            canonical: format!("{}{}", state.cfg.site_url, path),
            companies: COMPANIES,
            v: ASSET_VERSION,
        }
    }

    pub fn icon(&self, name: &str) -> String {
        icons::svg(name)
    }

    pub fn is(&self, path: &str) -> bool {
        self.path == path
    }

    pub fn in_group(&self) -> bool {
        COMPANIES.iter().any(|c| self.path == format!("/{}", c.slug))
    }

    pub fn year(&self) -> i32 {
        time::OffsetDateTime::now_utc().year()
    }
}

pub fn render<T: Template>(t: T) -> Response {
    match t.render() {
        Ok(html) => Html(html).into_response(),
        Err(e) => {
            tracing::error!("template error: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response()
        }
    }
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(home))
        .route("/about", get(about))
        .route("/services", get(services))
        .route("/strategy", get(strategy))
        .route("/global-access", get(global_access))
        .route("/team", get(team))
        .route("/team/live", get(team_live))
        .route("/services/live", get(services_live))
        .route("/privacy", get(privacy))
        .route("/terms", get(terms))
        .route("/api/team", get(api_team))
        .route("/api/services", get(api_services))
        .route("/api/status", get(api_status))
        .route("/robots.txt", get(robots))
        .route("/sitemap.xml", get(sitemap))
        .route("/:slug", get(company))
        .fallback(not_found)
}

// ---------------------------------------------------------------------------

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTpl {
    page: PageMeta,
    team: Arc<Vec<TeamMember>>,
}

async fn home(State(st): State<AppState>) -> Response {
    let team = st.noco.team().await;
    render(HomeTpl {
        page: PageMeta::new(
            &st,
            "Alfa Global Group | Beyond capital. Building futures.",
            "Alfa Global Group is a diversified strategic investment and holding company that deploys its own capital to build, acquire and scale enterprises across agriculture, maritime, infrastructure and technology.",
            "/",
        ),
        team,
    })
}

#[derive(Template)]
#[template(path = "about.html")]
struct AboutTpl {
    page: PageMeta,
}

async fn about(State(st): State<AppState>) -> Response {
    render(AboutTpl {
        page: PageMeta::new(
            &st,
            "About | Alfa Global Group",
            "A diversified strategic investment and holding company founded on a vision of global transformation, innovation, and lasting impact.",
            "/about",
        ),
    })
}

#[derive(Template)]
#[template(path = "services.html")]
struct ServicesTpl {
    page: PageMeta,
    services: Arc<Vec<Service>>,
}

async fn services(State(st): State<AppState>) -> Response {
    let services = st.noco.services().await;
    render(ServicesTpl {
        page: PageMeta::new(
            &st,
            "Services | Alfa Global Group",
            "The enterprises of Alfa Global Group across financial technology, strategic holdings and maritime, agriculture and natural resources, and cyber security.",
            "/services",
        ),
        services,
    })
}

/// The services list alone, polled by open Services pages for live updates.
#[derive(Template)]
#[template(path = "partials/services_live.html")]
struct ServicesLiveTpl {
    page: PageMeta,
    services: Arc<Vec<Service>>,
}

async fn services_live(State(st): State<AppState>) -> Response {
    let services = st.noco.services().await;
    no_store(render(ServicesLiveTpl { page: PageMeta::new(&st, "", "", "/services"), services }))
}

fn no_store(mut r: Response) -> Response {
    r.headers_mut().insert(header::CACHE_CONTROL, header::HeaderValue::from_static("no-store"));
    r
}

#[derive(Template)]
#[template(path = "strategy.html")]
struct StrategyTpl {
    page: PageMeta,
}

async fn strategy(State(st): State<AppState>) -> Response {
    render(StrategyTpl {
        page: PageMeta::new(
            &st,
            "Strategy | Alfa Global Group",
            "A unified approach that combines innovation, disciplined risk management, and long-term vision across every enterprise of the Group.",
            "/strategy",
        ),
    })
}

#[derive(Template)]
#[template(path = "global_access.html")]
struct GlobalTpl {
    page: PageMeta,
}

async fn global_access(State(st): State<AppState>) -> Response {
    render(GlobalTpl {
        page: PageMeta::new(
            &st,
            "Global Access | Alfa Global Group",
            "Connecting the Group's enterprises with international markets, partners and cross-border opportunities.",
            "/global-access",
        ),
    })
}

#[derive(Template)]
#[template(path = "team.html")]
struct TeamTpl {
    page: PageMeta,
    team: Arc<Vec<TeamMember>>,
}

async fn team(State(st): State<AppState>) -> Response {
    let team = st.noco.team().await;
    render(TeamTpl {
        page: PageMeta::new(
            &st,
            "Leadership | Alfa Global Group",
            "The leadership of Alfa Global Group: decades of combined experience across capital markets, maritime operations, cyber security, law, and enterprise building.",
            "/team",
        ),
        team,
    })
}

/// The team grid alone, polled by open Team pages for live updates.
#[derive(Template)]
#[template(path = "partials/team_live.html")]
struct TeamLiveTpl {
    page: PageMeta,
    team: Arc<Vec<TeamMember>>,
}

async fn team_live(State(st): State<AppState>) -> Response {
    let team = st.noco.team().await;
    no_store(render(TeamLiveTpl { page: PageMeta::new(&st, "", "", "/team"), team }))
}

#[derive(Template)]
#[template(path = "privacy.html")]
struct PrivacyTpl {
    page: PageMeta,
}

async fn privacy(State(st): State<AppState>) -> Response {
    render(PrivacyTpl { page: PageMeta::new(&st, "Privacy Policy | Alfa Global Group", "How Alfa Global Group collects, uses and protects personal information.", "/privacy") })
}

#[derive(Template)]
#[template(path = "terms.html")]
struct TermsTpl {
    page: PageMeta,
}

async fn terms(State(st): State<AppState>) -> Response {
    render(TermsTpl { page: PageMeta::new(&st, "Terms of Service | Alfa Global Group", "The terms that govern the use of the Alfa Global Group website.", "/terms") })
}

#[derive(Template)]
#[template(path = "company.html")]
struct CompanyTpl {
    page: PageMeta,
    company: &'static Company,
    others: Vec<&'static Company>,
}

async fn company(State(st): State<AppState>, Path(slug): Path<String>) -> Response {
    let Some(c) = content::company(&slug) else {
        return not_found(State(st)).await;
    };
    let others = COMPANIES.iter().filter(|o| o.slug != c.slug).collect();
    render(CompanyTpl { page: PageMeta::new(&st, &format!("{} | Alfa Global Group", c.name), c.summary, &format!("/{}", c.slug)), company: c, others })
}

#[derive(Template)]
#[template(path = "not_found.html")]
struct NotFoundTpl {
    page: PageMeta,
}

pub async fn not_found(State(st): State<AppState>) -> Response {
    let mut r = render(NotFoundTpl { page: PageMeta::new(&st, "Page not found | Alfa Global Group", "The page you were looking for could not be found.", "/404") });
    *r.status_mut() = StatusCode::NOT_FOUND;
    r
}

// --- JSON and crawler endpoints --------------------------------------------

async fn api_team(State(st): State<AppState>) -> Json<Vec<TeamMember>> {
    Json(st.noco.team().await.as_ref().clone())
}

async fn api_services(State(st): State<AppState>) -> Json<Vec<Service>> {
    Json(st.noco.services().await.as_ref().clone())
}

async fn api_status(State(st): State<AppState>) -> Json<serde_json::Value> {
    Json(st.noco.status().await)
}

async fn robots(State(st): State<AppState>) -> impl IntoResponse {
    ([(header::CONTENT_TYPE, "text/plain")], format!("User-agent: *\nAllow: /\nSitemap: {}/sitemap.xml\n", st.cfg.site_url))
}

async fn sitemap(State(st): State<AppState>) -> impl IntoResponse {
    let mut paths: Vec<String> = ["/", "/about", "/services", "/strategy", "/global-access", "/team", "/contact", "/privacy", "/terms"]
        .iter()
        .map(|p| p.to_string())
        .collect();
    paths.extend(COMPANIES.iter().map(|c| format!("/{}", c.slug)));
    let urls: String = paths.iter().map(|p| format!("<url><loc>{}{}</loc></url>", st.cfg.site_url, p)).collect();
    (
        [(header::CONTENT_TYPE, "application/xml")],
        format!(r#"<?xml version="1.0" encoding="UTF-8"?><urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">{urls}</urlset>"#),
    )
}
