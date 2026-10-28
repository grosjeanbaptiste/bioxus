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
}
