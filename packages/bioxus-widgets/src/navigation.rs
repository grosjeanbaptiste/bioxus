//! Breadcrumb and tabs. Like `AppShell` they own no route: the app maps item ids.

use crate::nav::NavItem;
use dioxus::prelude::*;

/// The path to the current place; every item but the last is a button.
#[component]
pub fn Breadcrumb(label: String, items: Vec<NavItem>, on_select: EventHandler<String>) -> Element {
    let last = items.len().saturating_sub(1);
    rsx! {
        nav { class: "bx-breadcrumb", "aria-label": "{label}",
            ol {
                for (i, item) in items.into_iter().enumerate() {
                    li {
                        if i == last {
                            span { "aria-current": "page", "{item.label}" }
                        } else {
                            button {
                                class: "bx-link",
                                disabled: item.disabled,
                                onclick: move |_| on_select.call(item.id.clone()),
                                "{item.label}"
                            }
                        }
                    }
                }
            }
        }
    }
}

/// A row of tabs; the app shows the panel of the `active` one.
#[component]
pub fn Tabs(
    label: String,
    items: Vec<NavItem>,
    active: String,
    on_select: EventHandler<String>,
) -> Element {
    rsx! {
        div { class: "bx-tabs", role: "tablist", "aria-label": "{label}",
            for item in items {
                button {
                    class: "bx-tab",
                    role: "tab",
                    "aria-selected": if item.id == active { "true" } else { "false" },
                    disabled: item.disabled,
                    onclick: move |_| on_select.call(item.id.clone()),
                    "{item.label}"
                }
            }
        }
    }
}
