//! The one crate an app depends on. Each building block is a module behind a feature
//! (all on by default; use `default-features = false` and pick only what the app needs); the packages under `packages/` are its implementation.

/// Tokens, reset and shared stylesheet.
#[cfg(feature = "design")]
pub use bioxus_design as design;

/// Responsive shell and widgets.
#[cfg(feature = "widgets")]
pub use bioxus_widgets as widgets;

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
}
