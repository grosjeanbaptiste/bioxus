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
    /// Text of the button that opens the drawer of `secondary` items on the phone bottom bar.
    /// Without it, secondary items stay in the bar like the others.
    #[props(default)]
    pub more_label: Option<String>,
    pub children: Element,
}

/// The heading to show before item `i`: its section, when the previous item is in another.
fn section_starting_at(items: &[NavItem], i: usize) -> Option<&str> {
    let section = items[i].section.as_deref()?;
    let previous = i.checked_sub(1).and_then(|p| items[p].section.as_deref());
    (previous != Some(section)).then_some(section)
}

/// One navigation button; `select` receives the id of the item pressed.
fn item_button(item: &NavItem, active: &str, select: EventHandler<String>) -> Element {
    let id = item.id.clone();
    rsx! {
        button {
            class: "bx-nav-item",
            disabled: item.disabled,
            "aria-current": (item.id == active).then_some("page"),
            onclick: move |_| select.call(id.clone()),
            if let Some(icon) = &item.icon {
                span { class: "bx-nav-icon", "aria-hidden": "true", "{icon}" }
            }
            span { class: "bx-nav-label", "{item.label}" }
        }
    }
}

/// Navigation plus content: bottom bar on phones, rail on tablets, sidebar on desktops.
/// With `more_label`, the `secondary` items leave the phone bottom bar for a drawer; the
/// rail and the sidebar list them after the others, so nothing is hidden there.
#[component]
pub fn AppShell(props: AppShellProps) -> Element {
    let layout = props.layout.map(NavLayout::as_str);
    let on_select = props.on_select;
    let mut open = use_signal(|| false);
    let select = use_callback(move |id: String| {
        open.set(false);
        on_select.call(id);
    });
    let drawer = props
        .more_label
        .clone()
        .filter(|_| props.items.iter().any(|i| i.secondary));
    let in_drawer = |item: &NavItem| drawer.is_some() && item.secondary;
    let active_inside = props
        .items
        .iter()
        .any(|i| in_drawer(i) && i.id == props.active);
    rsx! {
        div { class: "bx-shell", "data-layout": layout,
            nav { class: "bx-nav", "aria-label": props.title.clone().unwrap_or_else(|| "Primary".into()),
                if let Some(title) = &props.title {
                    p { class: "bx-nav-title", "{title}" }
                }
                for (i, item) in props.items.iter().enumerate().filter(|(_, it)| !in_drawer(it)) {
                    Fragment { key: "{item.id}",
                        if let Some(section) = section_starting_at(&props.items, i) {
                            p { class: "bx-nav-section", "{section}" }
                        }
                        {item_button(item, &props.active, select)}
                    }
                }
                if let Some(label) = &drawer {
                    button {
                        class: "bx-nav-item bx-nav-more-toggle",
                        "aria-expanded": if open() { "true" } else { "false" },
                        "data-active": active_inside.then_some("true"),
                        onclick: move |_| open.set(!open()),
                        span { class: "bx-nav-label", "{label}" }
                    }
                    div { class: "bx-nav-more-list", hidden: !open(),
                        for (i, item) in props.items.iter().enumerate().filter(|(_, it)| in_drawer(it)) {
                            Fragment { key: "{item.id}",
                                if let Some(section) = section_starting_at(&props.items, i) {
                                    p { class: "bx-nav-section", "{section}" }
                                }
                                {item_button(item, &props.active, select)}
                            }
                        }
                    }
                }
            }
            main { class: "bx-main", {props.children} }
        }
    }
}
