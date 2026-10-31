//! The one crate an app depends on. Each building block is a module behind a feature
//! (all on by default; use `default-features = false` and pick only what the app needs); the packages under `packages/` are its implementation.

/// Tokens, reset and shared stylesheet.
#[cfg(feature = "design")]
pub use bioxus_design as design;

/// Responsive shell and widgets.
#[cfg(feature = "widgets")]
pub use bioxus_widgets as widgets;

/// Proportional bar charts.
#[cfg(feature = "charts")]
pub use bioxus_charts as charts;

/// Open-ended i18n.
#[cfg(feature = "locale")]
pub use bioxus_locale as locale;

/// Local storage port and adapters.
#[cfg(feature = "store")]
pub use bioxus_store as store;

/// Offline-first sync (hybrid logical clock, last-write-wins).
#[cfg(feature = "sync")]
pub use bioxus_sync as sync;

/// 3D viewer for glTF binaries.
#[cfg(feature = "viewer3d")]
pub use bioxus_viewer3d as viewer3d;

/// Lucide icons, one component per icon (`icons::Check { class: "bx-icon" }`); size them with CSS.
#[cfg(feature = "icons")]
pub use lucide_dioxus as icons;

/// Signals persisted in local or session storage.
#[cfg(feature = "persist")]
pub use dioxus_sdk_storage as persist;

/// Interval, timeout and debounce hooks.
#[cfg(feature = "time")]
pub use dioxus_sdk_time as time;

#[cfg(all(test, feature = "design"))]
mod tests {
    #[test]
    fn exposes_the_design_system() {
        assert_eq!(super::design::STYLESHEET, bioxus_design::STYLESHEET);
        assert!(super::design::TOKENS.contains(&"accent"));
    }

    #[cfg(feature = "widgets")]
    #[test]
    fn exposes_the_widgets() {
        assert!(super::widgets::STYLESHEET.contains(".bx-shell"));
    }

    #[cfg(feature = "charts")]
    #[test]
    fn exposes_the_charts() {
        assert!(super::charts::STYLESHEET.contains(".bx-bars"));
    }

    #[cfg(feature = "locale")]
    #[test]
    fn exposes_the_locale_api() {
        assert_eq!(super::locale::Locale::new("FR").unwrap().as_str(), "fr");
    }

    #[cfg(feature = "store")]
    #[test]
    fn exposes_the_store_port() {
        let store = super::store::MemoryStore::default();
        super::store::contract::block_on(super::store::contract::check(&store));
    }

    #[cfg(feature = "sync")]
    #[test]
    fn exposes_the_sync_engine() {
        let stamp = super::sync::Clock::new(3).tick(1).unwrap();
        assert_eq!(super::sync::clock::node_of(stamp), 3);
    }

    #[cfg(feature = "viewer3d")]
    #[test]
    fn exposes_the_viewer() {
        assert!(super::viewer3d::page("t", &[]).contains("three@"));
    }

    #[cfg(feature = "icons")]
    #[test]
    fn renders_an_icon() {
        use dioxus::prelude::*;
        fn app() -> Element {
            rsx! { super::icons::Check { class: "bx-icon".to_string() } }
        }
        let mut dom = VirtualDom::new(app);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("<svg") && html.contains("bx-icon"), "{html}");
    }

    #[cfg(feature = "persist")]
    #[test]
    fn exposes_the_storage_hooks() {
        assert!(std::any::type_name::<super::persist::LocalStorage>().contains("LocalStorage"));
    }

    #[cfg(feature = "time")]
    #[test]
    fn exposes_the_time_hooks() {
        assert!(std::any::type_name::<super::time::UseInterval>().contains("UseInterval"));
    }
}
