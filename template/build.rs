//! Every app has an icon: this repo's own if it has one, else the BIOXUS icon.

use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);
    let out = PathBuf::from(std::env::var("OUT_DIR")?);
    for candidate in bioxus::design::icon::CANDIDATES {
        println!("cargo:rerun-if-changed={}", root.join(candidate).display());
    }
    bioxus::design::icon::emit_png(Path::new(&root), &out)?;
    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}
