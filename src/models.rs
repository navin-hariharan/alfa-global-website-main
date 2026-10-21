//! Team and Services records and how they are read from NocoDB rows.

use serde::Serialize;
use serde_json::{Map, Value};

/// A paragraph, or a run of bullet lines, inside a long biography.
#[derive(Clone, Debug, Serialize)]
pub struct Block {
    pub items: Vec<String>,
    pub is_list: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct TeamMember {
    pub name: String,
    pub role: String,
    pub image: Option<String>,
    pub short_bio: String,
    pub bio: Vec<Block>,
    pub linkedin: Option<String>,
    order: f64,
}

impl TeamMember {
    pub fn initials(&self) -> String {
        self.name
            .split_whitespace()
            .filter(|w| w.chars().next().is_some_and(|c| c.is_alphabetic()) && !w.ends_with('.'))
            .take(2)
            .filter_map(|w| w.chars().next())
            .collect::<String>()
            .to_uppercase()
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Service {
    pub slug: String,
    pub title: String,
    pub tagline: String,
    pub category: String,
    pub description: Vec<Block>,
    pub features: Vec<String>,
    pub image: Option<String>,
    pub gallery: Vec<String>,
    pub link: Option<String>,
    /// Every other column of the row, in table order, so nothing entered in
    /// NocoDB is left off the page.
    pub details: Vec<Detail>,
    order: f64,
}

/// One extra column of a service row: its heading and its content.
#[derive(Clone, Debug, Serialize)]
pub struct Detail {
    pub label: String,
    pub blocks: Vec<Block>,
    pub link: Option<String>,
}

impl Service {
    pub fn number(&self, index: &usize) -> String {
        format!("{index:02}")
    }

    /// Whether there is anything beyond the short text for "Read more" to show.
    pub fn has_more(&self) -> bool {
        !self.description.is_empty() || !self.features.is_empty() || !self.details.is_empty() || !self.gallery.is_empty() || self.link.is_some()
    }
}

// ---------------------------------------------------------------------------
// Reading NocoDB rows. Column names are matched loosely (case, spaces and
// punctuation are ignored) so the table can use "Full Bio", "full_bio", etc.
// ---------------------------------------------------------------------------

fn norm(key: &str) -> String {
    key.chars().filter(|c| c.is_alphanumeric()).flat_map(|c| c.to_lowercase()).collect()
}

struct Row<'a> {
    fields: Vec<(String, &'a Value)>,
    base_url: &'a str,
}

impl<'a> Row<'a> {
    fn new(map: &'a Map<String, Value>, base_url: &'a str) -> Self {
        Self { fields: map.iter().map(|(k, v)| (norm(k), v)).collect(), base_url }
    }

    fn raw(&self, keys: &[&str]) -> Option<&'a Value> {
        keys.iter().find_map(|k| self.fields.iter().find(|(f, v)| f == k && !v.is_null()).map(|(_, v)| *v))
    }

    fn text(&self, keys: &[&str]) -> Option<String> {
        let v = self.raw(keys)?;
        let s = match v {
            Value::String(s) => s.trim().to_string(),
            Value::Number(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Array(a) => a.iter().filter_map(|x| x.as_str()).collect::<Vec<_>>().join("\n"),
            _ => String::new(),
        };
        (!s.is_empty()).then_some(s)
    }

    fn number(&self, keys: &[&str]) -> Option<f64> {
        match self.raw(keys)? {
            Value::Number(n) => n.as_f64(),
            Value::String(s) => s.trim().parse().ok(),
            _ => None,
        }
    }

    /// `false` only when a visibility column exists and is switched off.
    fn visible(&self) -> bool {
        if let Some(v) = self.raw(&["show", "visible", "active", "published", "enabled", "display"]) {
            return match v {
                Value::Bool(b) => *b,
                Value::Number(n) => n.as_f64() != Some(0.0),
                Value::String(s) => !matches!(s.trim().to_lowercase().as_str(), "false" | "no" | "0" | "hidden"),
                _ => true,
            };
        }
        if let Some(Value::Bool(true)) = self.raw(&["hidden", "hide", "archived"]) {
            return false;
        }
        true
    }

    /// An image column may be a plain URL or a NocoDB attachment array.
    fn image(&self, keys: &[&str]) -> Option<String> {
        let v = self.raw(keys)?;
        let url = match v {
            Value::String(s) => Some(s.trim().to_string()),
            Value::Array(items) => items.iter().find_map(attachment_url),
            Value::Object(_) => attachment_url(v),
            _ => None,
        }?;
        if url.is_empty() {
            return None;
        }
        Some(self.absolute(url))
    }

    fn absolute(&self, url: String) -> String {
        if url.starts_with("http://") || url.starts_with("https://") || url.starts_with("data:") {
            url
        } else if url.starts_with('/') {
            format!("{}{}", self.base_url, url)
        } else {
            format!("{}/{}", self.base_url, url)
        }
    }
}

fn attachment_url(v: &Value) -> Option<String> {
    let o = v.as_object()?;
    ["signedUrl", "url", "signedPath", "path"]
        .iter()
        .find_map(|k| o.get(*k).and_then(|x| x.as_str()).filter(|s| !s.is_empty()).map(str::to_string))
}

/// Splits free text into paragraphs, grouping lines that start with a bullet.
pub fn blocks(text: &str) -> Vec<Block> {
    let mut out: Vec<Block> = Vec::new();
    for line in text.lines().map(str::trim).filter(|l| !l.is_empty()) {
        let cleaned = line.trim_start_matches('#').trim().replace("**", "").replace("__", "");
        let line = cleaned.as_str();
        if line.is_empty() {
            continue;
        }
        let bullet = ["- ", "• ", "* ", "– ", "✓ ", "✔ "].iter().find_map(|p| line.strip_prefix(p)).or_else(|| numbered(line));
        match bullet {
            Some(item) => match out.last_mut() {
                Some(b) if b.is_list => b.items.push(item.trim().to_string()),
                _ => out.push(Block { items: vec![item.trim().to_string()], is_list: true }),
            },
            None => out.push(Block { items: vec![line.to_string()], is_list: false }),
        }
    }
    out
}

/// "1. Item" or "2) Item" -> "Item".
fn numbered(line: &str) -> Option<&str> {
    let digits = line.chars().take_while(|c| c.is_ascii_digit()).count();
    if digits == 0 || digits > 2 {
        return None;
    }
    line[digits..].strip_prefix(". ").or_else(|| line[digits..].strip_prefix(") "))
}

fn list(text: &str) -> Vec<String> {
    let sep = if text.contains('\n') { '\n' } else if text.contains(';') { ';' } else { ',' };
    text.split(sep)
        .map(|s| s.trim().trim_start_matches(['-', '•', '*']).trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

/// The first one or two sentences of a long text, for card summaries.
fn summary(text: &str) -> String {
    let flat = text.split_whitespace().collect::<Vec<_>>().join(" ");
    let mut out = String::new();
    for sentence in flat.split_inclusive(". ") {
        if !out.is_empty() && out.len() + sentence.len() > 240 {
            break;
        }
        out.push_str(sentence);
        if out.len() > 140 {
            break;
        }
    }
    out.trim().to_string()
}

pub fn team_from_row(map: &Map<String, Value>, base_url: &str, index: usize) -> Option<TeamMember> {
    let r = Row::new(map, base_url);
    if !r.visible() {
        return None;
    }
    let name = r.text(&["name", "fullname", "membername", "title"])?;
    let role = r.text(&["role", "designation", "position", "jobtitle", "title"]).filter(|t| *t != name).unwrap_or_default();
    let full = r.text(&["fullbio", "bio", "biography", "profile", "about", "description", "details", "longbio"]);
    let short = r.text(&["shortbio", "summary", "short", "intro", "tagline", "excerpt", "shortdescription"]);
    let bio = blocks(full.as_deref().or(short.as_deref()).unwrap_or_default());
    let first_para = bio.iter().find(|b| !b.is_list).and_then(|b| b.items.first()).map(|p| summary(p));
    let short_bio = short.or(first_para).unwrap_or_default();
    Some(TeamMember {
        name,
        role,
        image: r.image(&["image", "photo", "picture", "avatar", "profileimage", "profilepicture", "headshot", "imageurl", "photourl"]),
        short_bio,
        bio,
        linkedin: r.text(&["linkedin", "linkedinurl", "linkedinprofile"]).filter(|u| u.starts_with("https://")),
        order: r.number(&["placement", "placementnumber", "placementno", "place", "order", "sortorder", "sort", "displayorder", "sequence", "rank", "priority", "serialno", "sno"]).unwrap_or(1e6 + index as f64),
    })
}

const SERVICE_TITLE: &[&str] = &["title", "name", "service", "servicename", "servicetitle", "heading"];
const SERVICE_TAGLINE: &[&str] = &["shorttext", "tagline", "subtitle", "subheading", "shortdescription", "shortdesc", "short", "summary", "intro", "lead", "headline", "excerpt"];
const SERVICE_CATEGORY: &[&str] = &["category", "sector", "type", "group", "vertical", "industry"];
const SERVICE_DESC: &[&str] = &["maintext", "description", "desc", "longtext", "fulltext", "maincontent", "bodytext", "details", "content", "body", "text", "longdescription", "fulldescription", "overview", "about"];
const SERVICE_FEATURES: &[&str] = &["features", "highlights", "points", "bullets", "keypoints", "items", "offerings", "keyfeatures"];
/// Icon columns from older tables are read and ignored; the page shows logos instead.
const SERVICE_ICON: &[&str] = &["icon", "iconname"];
const SERVICE_IMAGE: &[&str] = &["image", "photo", "picture", "iconimage", "cover", "coverimage", "imageurl", "banner", "thumbnail"];
const SERVICE_LINK: &[&str] = &["link", "url", "website", "learnmore", "cta", "ctalink", "page"];
const ORDER_KEYS: &[&str] = &["placement", "placementnumber", "placementno", "place", "order", "sortorder", "sort", "displayorder", "sequence", "rank", "priority", "serialno", "sno"];
const FLAG_KEYS: &[&str] = &["show", "visible", "active", "published", "enabled", "display", "hidden", "hide", "archived"];

/// NocoDB bookkeeping columns that never belong on the page.
fn is_system(key: &str) -> bool {
    matches!(key, "id" | "ncrecordid" | "ncrecordhash" | "createdat" | "updatedat" | "createdby" | "updatedby" | "ncorder" | "slug")
        || key.starts_with("nc")
}

fn humanize(key: &str) -> String {
    let words: Vec<String> = key
        .replace(['_', '-'], " ")
        .split_whitespace()
        .map(|w| {
            if w.chars().any(|c| c.is_uppercase()) {
                w.to_string()
            } else {
                let mut c = w.chars();
                c.next().map(|f| f.to_uppercase().collect::<String>() + c.as_str()).unwrap_or_default()
            }
        })
        .collect();
    words.join(" ")
}

pub fn slugify(text: &str) -> String {
    let mut out = String::new();
    for c in text.chars() {
        if c.is_alphanumeric() {
            out.extend(c.to_lowercase());
        } else if !out.ends_with('-') && !out.is_empty() {
            out.push('-');
        }
    }
    out.trim_end_matches('-').to_string()
}

fn is_url(s: &str) -> bool {
    (s.starts_with("https://") || s.starts_with("http://")) && !s.contains(char::is_whitespace)
}

fn is_image_attachment(v: &Value) -> bool {
    let Some(o) = v.as_object() else { return false };
    let mime = o.get("mimetype").or_else(|| o.get("mimeType")).and_then(|m| m.as_str()).unwrap_or("");
    let name = o.get("title").or_else(|| o.get("path")).or_else(|| o.get("url")).and_then(|m| m.as_str()).unwrap_or("").to_lowercase();
    mime.starts_with("image/") || [".png", ".jpg", ".jpeg", ".webp", ".gif", ".svg", ".avif"].iter().any(|e| name.ends_with(e))
}

/// Text of a linked record, lookup or select option.
fn object_text(v: &Value) -> Option<String> {
    let o = v.as_object()?;
    ["title", "Title", "name", "Name", "value", "label"]
        .iter()
        .find_map(|k| o.get(*k))
        .and_then(|x| match x {
            Value::String(s) => Some(s.trim().to_string()),
            Value::Number(n) => Some(n.to_string()),
            _ => None,
        })
        .filter(|s| !s.is_empty())
}

pub fn service_from_row(map: &Map<String, Value>, base_url: &str, index: usize) -> Option<Service> {
    let r = Row::new(map, base_url);
    if !r.visible() {
        return None;
    }
    let title = r.text(SERVICE_TITLE)?;
    let description_text = r.text(SERVICE_DESC).unwrap_or_default();
    let tagline = r.text(SERVICE_TAGLINE).filter(|t| *t != description_text).unwrap_or_default();
    let features = r.text(SERVICE_FEATURES).map(|t| list(&t)).unwrap_or_default();
    let link = r.text(SERVICE_LINK).filter(|u| is_url(u) || u.starts_with('/'));

    // The first key that matched in each list is "used"; everything else
    // becomes an extra detail block.
    let used: Vec<&str> = [SERVICE_TITLE, SERVICE_TAGLINE, SERVICE_CATEGORY, SERVICE_DESC, SERVICE_FEATURES, SERVICE_ICON, SERVICE_IMAGE, SERVICE_LINK, ORDER_KEYS]
        .iter()
        .filter_map(|keys| keys.iter().find(|k| r.raw(&[**k]).is_some()).copied())
        .chain(FLAG_KEYS.iter().copied())
        .collect();

    let mut gallery = Vec::new();
    let mut details = Vec::new();
    for (key, value) in map {
        let n = norm(key);
        if used.contains(&n.as_str()) || is_system(&n) || value.is_null() {
            continue;
        }
        let label = humanize(key);
        match value {
            Value::String(t) if is_url(t.trim()) => details.push(Detail { label, blocks: vec![], link: Some(t.trim().to_string()) }),
            Value::String(t) => {
                let b = blocks(t);
                if !b.is_empty() {
                    details.push(Detail { label, blocks: b, link: None });
                }
            }
            Value::Number(num) => details.push(Detail { label, blocks: vec![Block { items: vec![num.to_string()], is_list: false }], link: None }),
            Value::Bool(b) => details.push(Detail { label, blocks: vec![Block { items: vec![if *b { "Yes" } else { "No" }.into()], is_list: false }], link: None }),
            Value::Array(items) => {
                let mut texts = Vec::new();
                for item in items {
                    if is_image_attachment(item) {
                        if let Some(u) = attachment_url(item) {
                            gallery.push(r.absolute(u));
                        }
                    } else if let Some(t) = item.as_str().map(str::trim).filter(|t| !t.is_empty()) {
                        texts.push(t.to_string());
                    } else if let Some(t) = object_text(item) {
                        texts.push(t);
                    }
                }
                if !texts.is_empty() {
                    details.push(Detail { label, blocks: vec![Block { items: texts, is_list: true }], link: None });
                }
            }
            Value::Object(_) => {
                if let Some(t) = object_text(value) {
                    details.push(Detail { label, blocks: vec![Block { items: vec![t], is_list: false }], link: None });
                }
            }
            Value::Null => {}
        }
    }

    // Extra image attachments on the main image column go to the gallery too.
    let image = r.image(SERVICE_IMAGE);
    if let Some(Value::Array(items)) = r.raw(SERVICE_IMAGE) {
        gallery.extend(items.iter().skip(1).filter_map(attachment_url).map(|u| r.absolute(u)));
    }

    let mut service = Service {
        slug: slugify(&title),
        title,
        tagline,
        category: r.text(SERVICE_CATEGORY).unwrap_or_default(),
        description: blocks(&description_text),
        features,
        image,
        gallery,
        link,
        details,
        order: r.number(ORDER_KEYS).unwrap_or(1e6 + index as f64),
    };
    if service.tagline.is_empty() {
        // No short text column: lead with the opening of the main text.
        if let Some(first) = service.description.iter().find(|b| !b.is_list).and_then(|b| b.items.first()) {
            service.tagline = summary(first);
        }
    }
    Some(service)
}

pub fn sort_team(v: &mut [TeamMember]) {
    v.sort_by(|a, b| a.order.total_cmp(&b.order));
}

pub fn sort_services(v: &mut [Service]) {
    v.sort_by(|a, b| a.order.total_cmp(&b.order));
    // Anchors must be unique even if two services share a title.
    let mut seen = std::collections::HashSet::new();
    for (i, s) in v.iter_mut().enumerate() {
        if s.slug.is_empty() || !seen.insert(s.slug.clone()) {
            s.slug = format!("{}-{}", if s.slug.is_empty() { "service" } else { &s.slug }, i + 1);
            seen.insert(s.slug.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn reads_loose_column_names_and_attachments() {
        let row = json!({
            "Full Name": "Jane Doe",
            "Designation": "Director",
            "Photo": [{"title": "jane.jpg", "signedPath": "download/abc/jane.jpg"}],
            "Full_Bio": "First paragraph.\n- one\n- two\nLast.",
            "Sort Order": 2
        });
        let m = team_from_row(row.as_object().unwrap(), "https://db.example.com", 0).unwrap();
        assert_eq!(m.name, "Jane Doe");
        assert_eq!(m.role, "Director");
        assert_eq!(m.image.as_deref(), Some("https://db.example.com/download/abc/jane.jpg"));
        assert_eq!(m.bio.len(), 3);
        assert!(m.bio[1].is_list);
        assert_eq!(m.short_bio, "First paragraph.");
        assert_eq!(m.initials(), "JD");
    }

    #[test]
    fn hidden_rows_are_skipped() {
        let row = json!({"Name": "Hidden", "Show": false});
        assert!(team_from_row(row.as_object().unwrap(), "", 0).is_none());
    }

    #[test]
    fn service_features_split() {
        let row = json!({"Title": "Cyber Security", "Description": "Protects", "Features": "A, B, C"});
        let s = service_from_row(row.as_object().unwrap(), "", 0).unwrap();
        assert_eq!(s.features, vec!["A", "B", "C"]);
    }

    #[test]
    fn every_service_column_is_kept_and_placement_sorts() {
        let row = json!({
            "Id": 7,
            "Title": "Maritime",
            "Short Text": "Owning vessels.",
            "Main Text": "Through Dhanveir Groups Shipping.\n1. One\n2. Two",
            "Placement": 2,
            "Approach": "Patient capital.",
            "Sectors": ["Dry bulk", "Chartering"],
            "Gallery": [{"title": "a.png", "mimetype": "image/png", "signedPath": "dl/a.png"}],
            "Brochure": "https://example.com/b.pdf",
            "CreatedAt": "2026-01-01"
        });
        let s = service_from_row(row.as_object().unwrap(), "https://db", 0).unwrap();
        assert_eq!(s.order, 2.0);
        assert_eq!(s.tagline, "Owning vessels.");
        assert_eq!(s.description.len(), 2);
        assert!(s.description[1].is_list);
        let labels: Vec<_> = s.details.iter().map(|d| d.label.as_str()).collect();
        assert_eq!(labels, vec!["Approach", "Sectors", "Brochure"]);
        assert_eq!(s.gallery, vec!["https://db/dl/a.png"]);

        let mut v = vec![s.clone(), service_from_row(json!({"Title": "First", "Placement": 1}).as_object().unwrap(), "", 1).unwrap()];
        sort_services(&mut v);
        assert_eq!(v[0].title, "First");
    }
}
