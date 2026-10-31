//! Read-only blocks: empty state, key/value row, avatar, status indicator, settings card.

use crate::badge::Tone;
use dioxus::prelude::*;

/// Shown where a list or screen has nothing yet; `action` sits under the hint.
#[component]
pub fn EmptyState(
    title: String,
    #[props(default)] action: Option<Element>,
    children: Element,
) -> Element {
    rsx! {
        div { class: "bx-empty",
            h2 { "{title}" }
            {children}
            if let Some(action) = action {
                {action}
            }
        }
    }
}

/// A label and its value on one row.
#[component]
pub fn KeyValueRow(label: String, value: String) -> Element {
    rsx! { dl { class: "bx-kv", dt { "{label}" } dd { "{value}" } } }
}

/// Up to two capitals taken from the first letters of the first two words.
#[must_use]
pub fn initials(name: &str) -> String {
    name.split_whitespace()
        .filter_map(|word| word.chars().next())
        .take(2)
        .flat_map(char::to_uppercase)
        .collect()
}

/// A person or account: the picture when `src` is given, else their initials.
#[component]
pub fn Avatar(name: String, #[props(default)] src: Option<String>) -> Element {
    rsx! {
        match src {
            Some(src) => rsx! { img { class: "bx-avatar", src, alt: "{name}" } },
            None => rsx! { span { class: "bx-avatar", role: "img", "aria-label": "{name}", "{initials(&name)}" } },
        }
    }
}

impl Tone {
    /// Text symbol that carries the tone without colour (check, bang, cross, dot).
    #[must_use]
    pub const fn symbol(self) -> &'static str {
        match self {
            Self::Neutral => "\u{2022}",
            Self::Ok => "\u{2713}",
            Self::Warn => "!",
            Self::Danger => "\u{2715}",
        }
    }
}

/// A state as symbol plus words, so it never relies on colour alone.
#[component]
pub fn StatusIndicator(#[props(default)] tone: Tone, label: String) -> Element {
    let class = format!("bx-status {}", tone.modifier());
    rsx! {
        span { class, role: "status",
            span { "aria-hidden": "true", "{tone.symbol()}" }
            " {label}"
        }
    }
}

/// A titled group of settings with an optional explanation.
#[component]
pub fn SettingsCard(
    title: String,
    #[props(default)] description: Option<String>,
    children: Element,
) -> Element {
    rsx! {
        section { class: "bx-panel bx-settings",
            h3 { class: "bx-section-title", "{title}" }
            if let Some(description) = description {
                p { class: "bx-hint", "{description}" }
            }
            {children}
        }
    }
}
