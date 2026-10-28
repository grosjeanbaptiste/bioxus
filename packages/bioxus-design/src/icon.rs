//! Every app has an app icon: the one its repo provides, else the BIOXUS icon.
//! A provided icon that is unreadable or not a real PNG/SVG is an error, never a silent
//! fallback to the default.

use std::path::{Path, PathBuf};

pub const DEFAULT_SVG: &str = include_str!("../assets/icon.svg");
pub const DEFAULT_PNG: &[u8] = include_bytes!("../assets/icon.png");

/// Where an app repo may keep its icon, first match wins. SVG before PNG.
pub const CANDIDATES: [&str; 4] = ["assets/icon.svg", "assets/icon.png", "icon.svg", "icon.png"];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Png,
    Svg,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Source {
    Provided(PathBuf),
    Default,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppIcon {
    pub source: Source,
    pub kind: Kind,
    pub bytes: Vec<u8>,
}

#[derive(Debug)]
pub enum IconError {
    Unreadable { path: PathBuf, reason: String },
    Invalid { path: PathBuf },
}

impl std::fmt::Display for IconError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unreadable { path, reason } => {
                write!(f, "app icon {} cannot be read: {reason}", path.display())
            }
            Self::Invalid { path } => {
                write!(f, "app icon {} is not a valid PNG or SVG", path.display())
            }
        }
    }
}

impl std::error::Error for IconError {}

fn kind_of(bytes: &[u8]) -> Option<Kind> {
    if bytes.starts_with(b"\x89PNG\r\n\x1a\n") {
        return Some(Kind::Png);
    }
    let head = String::from_utf8_lossy(&bytes[..bytes.len().min(2048)]);
    head.contains("<svg").then_some(Kind::Svg)
}

/// The icon of the app whose repo is `root`: its own, or BIOXUS's when it has none.
pub fn resolve(root: &Path) -> Result<AppIcon, IconError> {
    for rel in CANDIDATES {
        let path = root.join(rel);
        match std::fs::read(&path) {
            Ok(bytes) => {
                let kind =
                    kind_of(&bytes).ok_or_else(|| IconError::Invalid { path: path.clone() })?;
                return Ok(AppIcon {
                    source: Source::Provided(path),
                    kind,
                    bytes,
                });
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => {
                return Err(IconError::Unreadable {
                    path,
                    reason: e.to_string(),
                })
            }
        }
    }
    Ok(AppIcon {
        source: Source::Default,
        kind: Kind::Svg,
        bytes: DEFAULT_SVG.as_bytes().to_vec(),
    })
}

/// For a `build.rs`: write the resolved icon to `out_dir/app-icon.{png,svg}` and return its
/// path, so the app can `include_bytes!(concat!(env!("OUT_DIR"), "/app-icon.png"))`.
/// The default is written as PNG, the format window and bundle APIs want.
pub fn emit(root: &Path, out_dir: &Path) -> Result<PathBuf, IconError> {
    let icon = resolve(root)?;
    let (ext, bytes) = match (&icon.source, icon.kind) {
        (Source::Default, _) => ("png", DEFAULT_PNG.to_vec()),
        (_, Kind::Png) => ("png", icon.bytes),
        (_, Kind::Svg) => ("svg", icon.bytes),
    };
    let path = out_dir.join(format!("app-icon.{ext}"));
    std::fs::write(&path, bytes).map_err(|e| IconError::Unreadable {
        path: path.clone(),
        reason: e.to_string(),
    })?;
    Ok(path)
}

/// PNG bytes for window and bundle APIs, which cannot take an SVG: the app's own PNG if it
/// has one, else the BIOXUS PNG. An SVG-only app therefore still gets an icon.
pub fn resolve_png(root: &Path) -> Result<Vec<u8>, IconError> {
    for rel in CANDIDATES.iter().filter(|c| c.ends_with(".png")) {
        let path = root.join(rel);
        match std::fs::read(&path) {
            Ok(bytes) if kind_of(&bytes) == Some(Kind::Png) => return Ok(bytes),
            Ok(_) => return Err(IconError::Invalid { path }),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => {
                return Err(IconError::Unreadable {
                    path,
                    reason: e.to_string(),
                })
            }
        }
    }
    Ok(DEFAULT_PNG.to_vec())
}

/// For a `build.rs` of a desktop or mobile app: write [`resolve_png`] to `out_dir/app-icon.png`.
pub fn emit_png(root: &Path, out_dir: &Path) -> Result<PathBuf, IconError> {
    let path = out_dir.join("app-icon.png");
    std::fs::write(&path, resolve_png(root)?).map_err(|e| IconError::Unreadable {
        path: path.clone(),
        reason: e.to_string(),
    })?;
    Ok(path)
}

#[cfg(test)]
mod tests;
