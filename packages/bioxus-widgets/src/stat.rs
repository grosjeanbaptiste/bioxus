//! A labelled figure.

use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct StatProps {
    pub label: String,
    pub value: String,
}

#[component]
pub fn Stat(props: StatProps) -> Element {
    rsx! { div { class: "bx-stat", span { class: "bx-label", "{props.label}" } strong { "{props.value}" } } }
}
