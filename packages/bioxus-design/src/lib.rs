//! Design system shared by the Dioxus apps: tokens, reset, light and dark.
//! Units are `rem` only. An app overrides a token by redefining it after [`STYLESHEET`].

/// The shared stylesheet, to inject once at the app root.
pub const STYLESHEET: &str = include_str!("theme.css");

/// Tokens every app can rely on; each exists in both colour schemes.
pub const TOKENS: [&str; 9] = [
    "bg", "surface", "fg", "muted", "dim", "border", "accent", "warn", "danger",
];

#[cfg(test)]
mod tests {
    use super::{STYLESHEET, TOKENS};

    /// Text of the `@media (prefers-color-scheme: dark)` block.
    fn dark_block() -> &'static str {
        let start = STYLESHEET
            .find("prefers-color-scheme: dark")
            .expect("dark scheme block is missing");
        &STYLESHEET[start..]
    }

    #[test]
    fn light_scheme_defines_every_token() {
        let light = &STYLESHEET[..STYLESHEET.find("@media").expect("no @media block")];
        for t in TOKENS {
            assert!(light.contains(&format!("--{t}:")), "light misses --{t}");
        }
    }

    #[test]
    fn dark_scheme_redefines_every_token() {
        for t in TOKENS {
            assert!(
                dark_block().contains(&format!("--{t}:")),
                "dark misses --{t}"
            );
        }
    }

    #[test]
    fn uses_no_px_units() {
        assert!(!STYLESHEET.contains("px"));
    }

    #[test]
    fn resets_box_sizing_and_paints_the_page() {
        assert!(STYLESHEET.contains("box-sizing: border-box"));
        assert!(STYLESHEET.contains("background: var(--bg)"));
    }
}
