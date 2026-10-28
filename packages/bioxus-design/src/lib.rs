//! Design system shared by the Dioxus apps: tokens, reset, light and dark.
//! Ready-made themes ([`THEMES`]) and form factors ([`form_factor`]). Units are `rem` only. An app overrides a token by redefining it after [`STYLESHEET`].

pub mod form_factor;
pub mod icon;

/// The shared stylesheet, to inject once at the app root.
pub const STYLESHEET: &str = concat!(include_str!("theme.css"), include_str!("themes.css"));

/// Ready-made themes, picked by setting `data-theme="<name>"` on the root element.
/// The default look needs no attribute. Each theme has a light and a dark variant.
pub const THEMES: [&str; 3] = ["ocean", "forest", "sunset"];

/// Tokens every app can rely on; each exists in both colour schemes.
pub const TOKENS: [&str; 9] = [
    "bg", "surface", "fg", "muted", "dim", "border", "accent", "warn", "danger",
];

#[cfg(test)]
mod tests {
    use super::{STYLESHEET, THEMES, TOKENS};

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
    fn every_theme_has_a_light_and_a_dark_rule() {
        for name in THEMES {
            let rule = format!("[data-theme=\"{name}\"]");
            assert_eq!(STYLESHEET.matches(&rule).count(), 2, "theme {name}");
        }
    }

    #[test]
    fn themes_only_override_known_tokens() {
        let themes = &STYLESHEET[STYLESHEET.find("[data-theme").expect("no themes")..];
        for decl in themes.split("--").skip(1) {
            let name = decl.split(':').next().expect("split yields one item");
            assert!(TOKENS.contains(&name), "unknown token --{name}");
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
