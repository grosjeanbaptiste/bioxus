//! The iOS icon set: 13 PNGs named `icon_<n>x<n>.png`, RGB without alpha, the layout `ll`
//! (`stores/ios/AppIcon.appiconset`) and `actrack` (`ui/mobile/assets/icons`) already use,
//! and which their `embed`/`build_appicon` scripts read. `dx bundle` writes no AppIcon
//! entry, so without a set iOS shows a white square.

use super::IconError;
use std::path::{Path, PathBuf};

/// Pixel sizes the App Store and devices need.
pub const SIZES: [u32; 13] = [20, 29, 40, 58, 60, 76, 80, 87, 120, 152, 167, 180, 1024];
/// A complete set: (pixel size, PNG bytes) for every entry of [`SIZES`].
pub type IconSet = Vec<(u32, Vec<u8>)>;

/// Where an app repo may keep its set, first match wins.
pub const SET_DIRS: [&str; 3] = [
    "stores/ios/AppIcon.appiconset",
    "ui/mobile/assets/icons",
    "assets/icons",
];

macro_rules! default_set {
    ($($n:literal),*) => {
        [$(($n, include_bytes!(concat!("../../assets/ios/icon_", $n, "x", $n, ".png")) as &[u8])),*]
    };
}
/// The BIOXUS set, used when the app has no icon of its own.
pub const DEFAULT_SET: [(u32, &[u8]); 13] =
    default_set!(20, 29, 40, 58, 60, 76, 80, 87, 120, 152, 167, 180, 1024);

#[must_use]
pub fn file_name(size: u32) -> String {
    format!("icon_{size}x{size}.png")
}

/// Width and height from a PNG header, or `None` if it is not a PNG.
fn png_dimensions(b: &[u8]) -> Option<(u32, u32)> {
    let sig = b.starts_with(b"\x89PNG\r\n\x1a\n") && b.get(12..16) == Some(b"IHDR");
    let be = |i: usize| {
        b.get(i..i + 4)
            .and_then(|x| x.try_into().ok())
            .map(u32::from_be_bytes)
    };
    if sig {
        Some((be(16)?, be(20)?))
    } else {
        None
    }
}

fn read_set(dir: &Path) -> Result<Option<IconSet>, IconError> {
    let mut found = Vec::new();
    let mut missing = Vec::new();
    for size in SIZES {
        let path = dir.join(file_name(size));
        match std::fs::read(&path) {
            Ok(bytes) if png_dimensions(&bytes) == Some((size, size)) => found.push((size, bytes)),
            Ok(_) => return Err(IconError::Invalid { path }),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => missing.push(size),
            Err(e) => {
                return Err(IconError::Unreadable {
                    path,
                    reason: e.to_string(),
                })
            }
        }
    }
    match (found.is_empty(), missing.is_empty()) {
        (true, _) => Ok(None),
        (false, true) => Ok(Some(found)),
        (false, false) => Err(IconError::IncompleteSet {
            dir: dir.to_path_buf(),
            missing,
        }),
    }
}

/// The app's complete set, or BIOXUS's when the app has no icon at all. An app that has a
/// master icon but no set is an error: its own icon must not be silently replaced.
pub fn resolve_set(root: &Path) -> Result<IconSet, IconError> {
    for dir in SET_DIRS {
        if let Some(set) = read_set(&root.join(dir))? {
            return Ok(set);
        }
    }
    if let Some(master) = super::CANDIDATES
        .iter()
        .map(|c| root.join(c))
        .find(|p| p.is_file())
    {
        return Err(IconError::NoSetForMaster { master });
    }
    Ok(DEFAULT_SET.iter().map(|(n, b)| (*n, b.to_vec())).collect())
}

/// For a `build.rs`: write the resolved set into `out_dir/AppIcon.appiconset/`.
pub fn emit_set(root: &Path, out_dir: &Path) -> Result<PathBuf, IconError> {
    let dir = out_dir.join("AppIcon.appiconset");
    let fail = |path: &Path, e: std::io::Error| IconError::Unreadable {
        path: path.to_path_buf(),
        reason: e.to_string(),
    };
    std::fs::create_dir_all(&dir).map_err(|e| fail(&dir, e))?;
    for (size, bytes) in resolve_set(root)? {
        let path = dir.join(file_name(size));
        std::fs::write(&path, bytes).map_err(|e| fail(&path, e))?;
    }
    Ok(dir)
}

#[cfg(test)]
mod tests;
