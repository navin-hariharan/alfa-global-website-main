//! Inline SVG icons (24×24, stroke based), rendered with `|safe` in templates.

const NAMES: &[(&str, &str)] = &[
    ("arrow", r#"<path d="M5 12h14M13 6l6 6-6 6"/>"#),
    ("check", r#"<path d="m5 12.5 4.5 4.5L19 7.5"/>"#),
    ("chip", r#"<rect x="6" y="6" width="12" height="12" rx="2.5"/><path d="M9.5 9.5h5v5h-5zM9 2.5V6M15 2.5V6M9 18v3.5M15 18v3.5M2.5 9H6M2.5 15H6M18 9h3.5M18 15h3.5"/>"#),
    ("ship", r#"<path d="M3 17.5c1.5 1.3 3 1.3 4.5 0s3-1.3 4.5 0 3 1.3 4.5 0 3-1.3 4.5 0"/><path d="m4.5 14.5-1-4.5h17l-1.8 4.5M7 10V6.5h10V10M12 3v3.5"/>"#),
    ("leaf", r#"<path d="M5 19c0-8 5-13.5 15-14-.5 10-6 15-14 15"/><path d="M5 19 13 11"/>"#),
    ("shield", r#"<path d="M12 3 4.5 6v5.5c0 4.6 3.2 8.4 7.5 9.5 4.3-1.1 7.5-4.9 7.5-9.5V6z"/><path d="m9 12 2.2 2.2L15.5 10"/>"#),
    ("building", r#"<path d="M4 21V5.5L12 3v18M12 8.5l8 2.5V21M2.5 21h19M7 8v.01M7 11.5v.01M7 15v.01M15.5 13.5v.01M15.5 17v.01"/>"#),
    ("handshake", r#"<path d="m11 7-3.2-2.2a2 2 0 0 0-2.3.1L2.5 7.5v7l3 .5 5 4.5a1.6 1.6 0 0 0 2.3-2.2"/><path d="m13 7 2.3-1.8a2 2 0 0 1 2.4 0l3.8 2.8v6.5l-3 .5-4.5 4"/><path d="M8.5 11.5 12 8l3.5 3.5a1.5 1.5 0 0 1-2.1 2.1L12 12.2"/>"#),
    ("layers", r#"<path d="m12 3 9 5-9 5-9-5z"/><path d="m3 13 9 5 9-5"/><path d="m3 17.5 9 5 9-5" opacity=".55"/>"#),
    ("spark", r#"<path d="M12 3v4M12 17v4M3 12h4M17 12h4M6 6l2.5 2.5M15.5 15.5 18 18M6 18l2.5-2.5M15.5 8.5 18 6"/>"#),
    ("star", r#"<path d="m12 3 2.7 5.6 6.1.9-4.4 4.3 1 6.1L12 17l-5.4 2.9 1-6.1-4.4-4.3 6.1-.9z"/>"#),
    ("bulb", r#"<path d="M9 18h6M10 21h4M12 3a6 6 0 0 0-3.6 10.8c.6.5 1.1 1.3 1.1 2.2h5c0-.9.5-1.7 1.1-2.2A6 6 0 0 0 12 3z"/>"#),
    ("bolt", r#"<path d="M13 2.5 4.5 13.5H12l-1 8 8.5-11H12z"/>"#),
    ("eye", r#"<path d="M2.5 12S6 5 12 5s9.5 7 9.5 7-3.5 7-9.5 7-9.5-7-9.5-7z"/><circle cx="12" cy="12" r="3"/>"#),
    ("users", r#"<circle cx="9" cy="8" r="3.5"/><path d="M2.5 20c.5-3.5 3.2-6 6.5-6s6 2.5 6.5 6"/><path d="M16 4.6a3.5 3.5 0 0 1 0 6.8M18 14.3c2 .8 3.3 2.9 3.5 5.7"/>"#),
    ("scale", r#"<path d="M12 3v18M7 21h10M5 7h14M5 7l-3 7a3.5 3.5 0 0 0 6 0zM19 7l-3 7a3.5 3.5 0 0 0 6 0z"/>"#),
    ("lock", r#"<rect x="4.5" y="10.5" width="15" height="10.5" rx="2.5"/><path d="M8 10.5V7.5a4 4 0 0 1 8 0v3"/>"#),
    ("coins", r#"<ellipse cx="9" cy="7" rx="6" ry="3"/><path d="M3 7v4c0 1.7 2.7 3 6 3s6-1.3 6-3V7"/><path d="M9 14v3c0 1.7 2.7 3 6 3s6-1.3 6-3v-4c0-1.6-2.4-2.9-5.4-3"/>"#),
    ("globe", r#"<circle cx="12" cy="12" r="9"/><path d="M3 12h18M12 3c2.5 2.6 3.8 5.6 3.8 9S14.5 18.4 12 21c-2.5-2.6-3.8-5.6-3.8-9S9.5 5.6 12 3z"/>"#),
    ("chart", r#"<path d="M3 20.5h18"/><path d="m4 16 5-5 4 3 7-8"/><path d="M15 6h5v5"/>"#),
    ("target", r#"<circle cx="12" cy="12" r="9"/><circle cx="12" cy="12" r="5"/><circle cx="12" cy="12" r="1.2"/>"#),
    ("compass", r#"<circle cx="12" cy="12" r="9"/><path d="m15.5 8.5-2 5-5 2 2-5z"/>"#),
    ("anchor", r#"<circle cx="12" cy="5" r="2"/><path d="M12 7v14M8 11h8M4.5 13a7.5 7.5 0 0 0 15 0"/>"#),
    ("search", r#"<circle cx="11" cy="11" r="6.5"/><path d="m20 20-4.2-4.2"/>"#),
    ("cog", r#"<circle cx="12" cy="12" r="3"/><path d="M12 2.5v3M12 18.5v3M4.6 4.6l2.1 2.1M17.3 17.3l2.1 2.1M2.5 12h3M18.5 12h3M4.6 19.4l2.1-2.1M17.3 6.7l2.1-2.1"/>"#),
    ("doc", r#"<path d="M14 3H7a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2V8z"/><path d="M14 3v5h5M9 13h6M9 17h4"/>"#),
    ("phone", r#"<path d="M5 3.5h3.3l1.7 4.3-2.2 1.4a11 11 0 0 0 5 5l1.4-2.2 4.3 1.7V17a2.5 2.5 0 0 1-2.5 2.5C9.4 19.5 4.5 14.6 4.5 6A2.5 2.5 0 0 1 5 3.5z"/>"#),
    ("mail", r#"<rect x="3" y="5" width="18" height="14" rx="2.5"/><path d="m3.5 6.5 8.5 6.5 8.5-6.5"/>"#),
    ("pin", r#"<path d="M12 21s-7-6.2-7-11.5a7 7 0 0 1 14 0C19 14.8 12 21 12 21z"/><circle cx="12" cy="9.5" r="2.5"/>"#),
    ("clock", r#"<circle cx="12" cy="12" r="9"/><path d="M12 7v5l3.5 2"/>"#),
    ("external", r#"<path d="M14 4h6v6M20 4l-9 9M18 14v4.5a1.5 1.5 0 0 1-1.5 1.5h-11A1.5 1.5 0 0 1 4 18.5v-11A1.5 1.5 0 0 1 5.5 6H10"/>"#),
    ("close", r#"<path d="M6 6l12 12M18 6 6 18"/>"#),
    ("chevron", r#"<path d="m6 9 6 6 6-6"/>"#),
    ("linkedin", r#"<rect x="3" y="3" width="18" height="18" rx="3"/><path d="M8 10.5V17M8 7.5v.01M12 17v-6.5M12 13.5c0-1.7 1-3 2.6-3s2.4 1.1 2.4 3V17"/>"#),
    ("whatsapp", r#"<path d="M4 20.5 5.2 17A8.5 8.5 0 1 1 8 19.6z"/><path d="M9 8.5c0 3.5 2.6 6.5 6.5 6.5l1-1.6-2-1-1 .9c-1.2-.4-2.4-1.6-2.8-2.8l.9-1-1-2z"/>"#),
];

/// The SVG markup for `name` (falls back to a neutral spark).
pub fn svg(name: &str) -> String {
    let body = NAMES.iter().find(|(n, _)| *n == name).map(|(_, b)| *b).or_else(|| NAMES.iter().find(|(n, _)| *n == "spark").map(|(_, b)| *b)).unwrap_or_default();
    format!(
        r#"<svg class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">{body}</svg>"#
    )
}
