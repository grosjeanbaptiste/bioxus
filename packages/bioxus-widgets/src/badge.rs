//! A small status badge whose colour comes from the design tokens.

use dioxus::prelude::*;

/// Meaning of a badge; each maps to one design token.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Tone {
    #[default]
    Neutral,
    Ok,
    Warn,
    Danger,
}

impl Tone {
    #[must_use]
    pub const fn class(self) -> &'static str {
        match self {
            Self::Neutral => "bx-badge",
            Self::Ok => "bx-badge bx-ok",
            Self::Warn => "bx-badge bx-warn",
            Self::Danger => "bx-badge bx-danger",
        }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct BadgeProps {
    #[props(default)]
    pub tone: Tone,
    pub children: Element,
}

#[component]
pub fn Badge(props: BadgeProps) -> Element {
    rsx! { span { class: props.tone.class(), {props.children} } }
}
