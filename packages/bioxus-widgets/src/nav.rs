//! Navigation model. No router, no renderer: apps map their own routes to item ids.

use bioxus_design::form_factor::FormFactor;

/// One destination in the app's primary navigation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NavItem {
    pub id: String,
    pub label: String,
    /// Short glyph before the label (an emoji or a letter); decorative, hidden from assistive tech.
    pub icon: Option<String>,
}

impl NavItem {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon: None,
        }
    }

    #[must_use]
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

/// Where the primary navigation sits.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NavLayout {
    BottomBar,
    Rail,
    Sidebar,
}

impl NavLayout {
    /// The layout the stylesheet picks on its own at this form factor.
    #[must_use]
    pub fn for_form_factor(form_factor: FormFactor) -> Self {
        match form_factor {
            FormFactor::Phone => Self::BottomBar,
            FormFactor::Tablet => Self::Rail,
            FormFactor::Desktop => Self::Sidebar,
        }
    }

    /// Value of the `data-layout` attribute that forces this layout.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BottomBar => "bottom",
            Self::Rail => "rail",
            Self::Sidebar => "sidebar",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_form_factor_has_its_own_layout() {
        assert_eq!(
            NavLayout::for_form_factor(FormFactor::Phone),
            NavLayout::BottomBar
        );
        assert_eq!(
            NavLayout::for_form_factor(FormFactor::Tablet),
            NavLayout::Rail
        );
        assert_eq!(
            NavLayout::for_form_factor(FormFactor::Desktop),
            NavLayout::Sidebar
        );
    }

    #[test]
    fn attribute_values_are_distinct_and_match_the_css() {
        let names =
            [NavLayout::BottomBar, NavLayout::Rail, NavLayout::Sidebar].map(NavLayout::as_str);
        for (i, n) in names.iter().enumerate() {
            assert!(
                crate::STYLESHEET.contains(&format!("[data-layout=\"{n}\"]")),
                "css misses {n}"
            );
            assert!(!names[..i].contains(n), "duplicate {n}");
        }
    }
}
