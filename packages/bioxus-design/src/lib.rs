//! Design system shared by the Dioxus apps: tokens, reset, light and dark.
//! Ready-made themes ([`THEMES`]) and form factors ([`form_factor`]). Units are `rem` only. An app overrides a token by redefining it after [`STYLESHEET`].
//! The scheme follows the system unless an element carries `data-scheme="light|dark"`, which forces it for the whole page.

pub mod form_factor;
pub mod icon;

/// The shared stylesheet, to inject once at the app root.
pub const STYLESHEET: &str = concat!(include_str!("theme.css"), include_str!("themes.css"));

/// Ready-made themes, picked by setting `data-theme="<name>"` on the root element.
/// The default look needs no attribute. Each theme has a light and a dark variant, the dark one also under a forced `data-scheme="dark"`.
pub const THEMES: [&str; 3] = ["ocean", "forest", "sunset"];

/// Tokens every app can rely on; each exists in both colour schemes.
pub const TOKENS: [&str; 11] = [
    "bg",
    "surface",
    "fg",
    "muted",
    "dim",
    "border",
    "accent",
    "on-accent",
    "ok",
    "warn",
    "danger",
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
    fn every_theme_has_a_light_a_dark_and_a_forced_dark_rule() {
        for name in THEMES {
            let rule = format!("[data-theme=\"{name}\"]");
            // light, system dark, forced dark
            assert_eq!(STYLESHEET.matches(&rule).count(), 3, "theme {name}");
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

    /// Text of the first rule that forces `scheme`, up to its closing brace.
    fn forced_rule(scheme: &str) -> &'static str {
        let start = STYLESHEET
            .find(&format!(":root:where([data-scheme=\"{scheme}\"]"))
            .unwrap_or_else(|| panic!("no forced {scheme} rule"));
        let rest = &STYLESHEET[start..];
        &rest[..rest.find('}').expect("unclosed rule")]
    }

    #[test]
    fn forced_dark_defines_every_token() {
        let rule = forced_rule("dark");
        assert!(rule.contains("color-scheme: dark"));
        for t in TOKENS {
            assert!(
                rule.contains(&format!("--{t}:")),
                "forced dark misses --{t}"
            );
        }
    }

    #[test]
    fn forced_light_wins_over_a_dark_system() {
        assert!(forced_rule("light").contains("color-scheme: light"));
        assert!(
            dark_block().contains(":not([data-scheme=\"light\"], :has([data-scheme=\"light\"]))")
        );
    }

    /// An app's plain `:root { --x: … }` after the stylesheet must win, so every
    /// scheme condition sits in `:where()`, which adds no specificity.
    #[test]
    fn scheme_conditions_add_no_specificity() {
        for line in STYLESHEET.lines().filter(|l| l.contains("data-scheme")) {
            let cond = line.find("data-scheme").expect("filtered on it");
            assert!(line[..cond].contains(":where("), "{line}");
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
