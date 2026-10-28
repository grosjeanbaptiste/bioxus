//! A 3D viewer for glTF binaries. [`page`] builds a standalone page (pure, no UI dependency);
//! the `component` feature adds [`Viewer3d`], which shows that page in place.

use base64::Engine as _;

#[cfg(feature = "component")]
mod component;

#[cfg(feature = "component")]
pub use component::{Viewer3d, STYLESHEET};

/// The Three.js release the page loads; pinned so a page renders the same next year.
pub const THREE_VERSION: &str = "0.170.0";

/// One part of the scene: a name and its glTF binary (`.glb`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Part {
    pub name: String,
    pub glb: Vec<u8>,
}

impl Part {
    pub fn new(name: impl Into<String>, glb: Vec<u8>) -> Self {
        Self {
            name: name.into(),
            glb,
        }
    }
}

/// A standalone HTML page that shows `parts` with Three.js. Every binary is embedded, so the
/// page can be opened or sent as it is; only Three.js itself is fetched, from jsDelivr.
#[must_use]
pub fn page(title: &str, parts: &[Part]) -> String {
    let parts: Vec<_> = parts
        .iter()
        .map(|p| {
            let data = base64::engine::general_purpose::STANDARD.encode(&p.glb);
            serde_json::json!([p.name, data])
        })
        .collect();
    let parts = serde_json::Value::Array(parts)
        .to_string()
        .replace("</", "<\\/");
    PAGE.replace(
        "__CDN__",
        &format!("https://cdn.jsdelivr.net/npm/three@{THREE_VERSION}"),
    )
    .replace("__TITLE__", &escape_html(title))
    .replace("__PARTS__", &parts)
}

fn escape_html(text: &str) -> String {
    let mut escaped = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#x27;"),
            other => escaped.push(other),
        }
    }
    escaped
}

const PAGE: &str = include_str!("page.html");

#[cfg(test)]
mod tests;
