//! Alfa Global Group website.
//!
//! A small Axum server that renders every page on the server with Askama
//! templates, serves the static assets, and pulls the Team and Services lists
//! live from NocoDB.

mod config;
mod contact;
mod content;
mod icons;
mod models;
mod nocodb;
mod pages;

use std::{net::SocketAddr, sync::Arc, time::Duration};

use axum::{
    http::{header, HeaderValue},
    routing::get,
    Router,
};
use tower_http::{
    compression::CompressionLayer,
    services::{ServeDir, ServeFile},
    set_header::SetResponseHeaderLayer,
    trace::TraceLayer,
};
use tracing_subscriber::EnvFilter;

use crate::{config::Config, nocodb::NocoDb};

#[derive(Clone)]
pub struct AppState {
    pub cfg: Arc<Config>,
    pub noco: Arc<NocoDb>,
}

#[tokio::main]
async fn main() {
    // `.env` is optional: in Docker the variables come from `env_file`.
    let _ = dotenvy::dotenv();

    // `alfa-site healthcheck` is used by the Docker HEALTHCHECK, since the
    // runtime image has no curl or shell.
    if std::env::args().nth(1).as_deref() == Some("healthcheck") {
        std::process::exit(healthcheck().await);
    }

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "alfa_site=info,tower_http=warn".into()))
        .compact()
        .init();

    let cfg = Arc::new(Config::from_env());
    let noco = Arc::new(NocoDb::new(&cfg));
    if noco.is_configured() {
        tracing::info!("NocoDB enabled at {}", cfg.nocodb_url.as_deref().unwrap_or_default());
        noco.warm_up();
    } else {
        tracing::warn!("NOCODB_URL or NOCODB_API_TOKEN is not set; the Team and Services pages will be empty");
    }

    let state = AppState { cfg: cfg.clone(), noco };

    let static_files = Router::new()
        .nest_service("/static", ServeDir::new(&cfg.static_dir))
        .route_service("/favicon.ico", ServeFile::new(cfg.static_dir.join("favicon.ico")))
        .layer(SetResponseHeaderLayer::overriding(
            header::CACHE_CONTROL,
            HeaderValue::from_static("public, max-age=604800"),
        ));

    let app = pages::routes()
        .merge(contact::routes())
        .route("/healthz", get(|| async { "ok" }))
        .with_state(state)
        .merge(static_files)
        .layer(CompressionLayer::new())
        .layer(SetResponseHeaderLayer::if_not_present(header::X_CONTENT_TYPE_OPTIONS, HeaderValue::from_static("nosniff")))
        .layer(SetResponseHeaderLayer::if_not_present(header::X_FRAME_OPTIONS, HeaderValue::from_static("SAMEORIGIN")))
        .layer(SetResponseHeaderLayer::if_not_present(
            header::REFERRER_POLICY,
            HeaderValue::from_static("strict-origin-when-cross-origin"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            header::CONTENT_SECURITY_POLICY,
            HeaderValue::from_static(
                "default-src 'self'; img-src 'self' https: data:; style-src 'self' 'unsafe-inline' https://fonts.googleapis.com; \
                 font-src 'self' https://fonts.gstatic.com; script-src 'self'; frame-src https://www.openstreetmap.org; \
                 form-action 'self'; base-uri 'self'; object-src 'none'",
            ),
        ))
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], cfg.port));
    let listener = tokio::net::TcpListener::bind(addr).await.expect("could not bind the HTTP port");
    tracing::info!("Alfa Global website listening on http://{addr}");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("server error");
}

async fn shutdown_signal() {
    let ctrl_c = async {
        let _ = tokio::signal::ctrl_c().await;
    };
    #[cfg(unix)]
    let terminate = async {
        if let Ok(mut s) = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
            s.recv().await;
        }
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    tokio::select! { _ = ctrl_c => {}, _ = terminate => {} }
}

async fn healthcheck() -> i32 {
    let port = std::env::var("PORT").ok().and_then(|p| p.parse::<u16>().ok()).unwrap_or(8080);
    let client = reqwest::Client::builder().timeout(Duration::from_secs(3)).build();
    match client {
        Ok(c) => match c.get(format!("http://127.0.0.1:{port}/healthz")).send().await {
            Ok(r) if r.status().is_success() => 0,
            _ => 1,
        },
        Err(_) => 1,
    }
}
