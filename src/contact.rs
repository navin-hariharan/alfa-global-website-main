//! The contact page and its enquiry form. Enquiries are appended, one JSON
//! object per line, to `ENQUIRIES_FILE` (a Docker volume in production).

use askama::Template;
use axum::{
    extract::{Query, State},
    response::{Redirect, Response},
    routing::get,
    Form, Router,
};
use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;

use crate::{
    pages::{render, PageMeta},
    AppState,
};

pub const TOPICS: &[&str] = &[
    "Strategic Partnership / Joint Venture",
    "Business or Project Proposal",
    "Group Company Enquiry",
    "Careers",
    "Other",
];

pub fn routes() -> Router<AppState> {
    Router::new().route("/contact", get(page).post(submit))
}

#[derive(Template)]
#[template(path = "contact.html")]
struct ContactTpl {
    page: PageMeta,
    status: String,
    topics: &'static [&'static str],
}

#[derive(Deserialize)]
struct StatusQuery {
    status: Option<String>,
}

async fn page(State(st): State<AppState>, Query(q): Query<StatusQuery>) -> Response {
    render(ContactTpl {
        page: PageMeta::new(
            &st,
            "Contact | Alfa Global Group",
            "Enquiries regarding strategic partnerships, joint ventures, and enterprise proposals.",
            "/contact",
        ),
        status: q.status.unwrap_or_default(),
        topics: TOPICS,
    })
}

#[derive(Deserialize)]
struct Enquiry {
    name: String,
    email: String,
    #[serde(default)]
    phone: String,
    #[serde(default)]
    topic: String,
    message: String,
    /// Honeypot: real visitors never see or fill this field.
    #[serde(default)]
    company_website: String,
}

#[derive(Serialize)]
struct Stored<'a> {
    received_at: String,
    name: &'a str,
    email: &'a str,
    phone: &'a str,
    topic: &'a str,
    message: &'a str,
}

fn clip(s: &str, max: usize) -> &str {
    let s = s.trim();
    match s.char_indices().nth(max) {
        Some((i, _)) => &s[..i],
        None => s,
    }
}

async fn submit(State(st): State<AppState>, Form(f): Form<Enquiry>) -> Redirect {
    if !f.company_website.is_empty() {
        return Redirect::to("/contact?status=sent#enquiry");
    }
    let email = clip(&f.email, 200);
    let valid = !f.name.trim().is_empty()
        && !f.message.trim().is_empty()
        && email.contains('@')
        && email.rsplit('@').next().is_some_and(|d| d.contains('.'));
    if !valid {
        return Redirect::to("/contact?status=invalid#enquiry");
    }

    let record = Stored {
        received_at: time::OffsetDateTime::now_utc().format(&time::format_description::well_known::Rfc3339).unwrap_or_default(),
        name: clip(&f.name, 200),
        email,
        phone: clip(&f.phone, 40),
        topic: clip(&f.topic, 100),
        message: clip(&f.message, 5000),
    };

    match save(&st, &record).await {
        Ok(()) => {
            tracing::info!("new enquiry from {} ({})", record.name, record.topic);
            Redirect::to("/contact?status=sent#enquiry")
        }
        Err(e) => {
            tracing::error!("could not save enquiry: {e}");
            Redirect::to("/contact?status=error#enquiry")
        }
    }
}

async fn save(st: &AppState, record: &Stored<'_>) -> std::io::Result<()> {
    let path = &st.cfg.enquiries_file;
    if let Some(dir) = path.parent().filter(|d| !d.as_os_str().is_empty()) {
        tokio::fs::create_dir_all(dir).await?;
    }
    let mut line = serde_json::to_string(record).map_err(std::io::Error::other)?;
    line.push('\n');
    let mut file = tokio::fs::OpenOptions::new().create(true).append(true).open(path).await?;
    file.write_all(line.as_bytes()).await
}
