mod app;

/// Desktop gets a window icon (build.rs: ours, else BIOXUS's); web and mobile launch as is.
#[cfg(feature = "desktop")]
fn main() {
    use dioxus::desktop::{icon_from_memory, Config};
    // build.rs checked the file is a PNG, so decoding cannot fail short of a corrupt body.
    let icon = icon_from_memory::<dioxus::desktop::tao::window::Icon>(include_bytes!(concat!(
        env!("OUT_DIR"),
        "/app-icon.png"
    )))
    .expect("app icon PNG decodes");
    dioxus::LaunchBuilder::desktop()
        .with_cfg(Config::new().with_icon(icon))
        .launch(app::App);
}

#[cfg(not(feature = "desktop"))]
fn main() {
    dioxus::launch(app::App);
}
