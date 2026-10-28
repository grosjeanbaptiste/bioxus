//! The one crate an app depends on. Each building block is a module behind a feature
//! (all on by default); the packages under `packages/` are its implementation.

/// Tokens, reset and shared stylesheet.
#[cfg(feature = "design")]
pub use bioxus_design as design;

#[cfg(all(test, feature = "design"))]
mod tests {
    #[test]
    fn exposes_the_design_system() {
        assert_eq!(super::design::STYLESHEET, bioxus_design::STYLESHEET);
        assert!(super::design::TOKENS.contains(&"accent"));
    }
}
