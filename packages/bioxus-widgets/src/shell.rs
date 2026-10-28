//! The responsive shell. The layout is chosen by CSS media queries, so it works on every
//! Dioxus renderer (web, desktop, mobile) without JavaScript; `layout` forces one.

use crate::nav::{NavItem, NavLayout};
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct AppShellProps {
    pub items: Vec<NavItem>,
    /// Id of the current destination.
    pub active: String,
    pub on_select: EventHandler<String>,
    /// Name shown above the items (visible on the sidebar layout) and used as the nav label.
    #[props(default)]
    pub title: Option<String>,
    /// Force a layout instead of following the viewport width.
    #[props(default)]
    pub layout: Option<NavLayout>,
    pub children: Element,
}

/// Navigation plus content: bottom bar on phones, rail on tablets, sidebar on desktops.
#[component]
pub fn AppShell(props: AppShellProps) -> Element {
    let layout = props.layout.map(NavLayout::as_str);
    let on_select = props.on_select;
    rsx! {
        div { class: "bx-shell", "data-layout": layout,
            nav { class: "bx-nav", "aria-label": props.title.clone().unwrap_or_else(|| "Primary".into()),
                if let Some(title) = &props.title {
                    p { class: "bx-nav-title", "{title}" }
                }
                for item in props.items.iter() {
                    button {
                        key: "{item.id}",
                        class: "bx-nav-item",
                        "aria-current": (item.id == props.active).then_some("page"),
                        onclick: {
                            let id = item.id.clone();
                            move |_| on_select.call(id.clone())
                        },
                        if let Some(icon) = &item.icon {
                            span { class: "bx-nav-icon", "aria-hidden": "true", "{icon}" }
                        }
                        span { class: "bx-nav-label", "{item.label}" }
                    }
                }
            }
            main { class: "bx-main", {props.children} }
        }
    }
}
