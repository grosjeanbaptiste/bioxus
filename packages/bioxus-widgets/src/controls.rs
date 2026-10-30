//! Small building blocks: top bar, panel, section, busy button, notice, chip.
//! Plain buttons, options and grids are CSS classes (`bx-btn`, `bx-btn-primary`,
//! `bx-option`, `bx-grid`) so they keep every native attribute.

use crate::badge::Tone;
use dioxus::prelude::*;

/// A native fold; the browser handles the toggle and the keyboard.
#[component]
pub fn Collapsible(summary: String, #[props(default)] open: bool, children: Element) -> Element {
    rsx! {
        details { class: "bx-collapsible", open,
            summary { "{summary}" }
            {children}
        }
    }
}

/// A bar of short items; `action` sits at the end.
#[component]
pub fn TopBar(action: Element, children: Element) -> Element {
    rsx! {
        div { class: "bx-topbar",
            {children}
            span { class: "bx-topbar-action", {action} }
        }
    }
}

/// A surface card, titled when `title` is given.
#[component]
pub fn Panel(#[props(default)] title: Option<String>, children: Element) -> Element {
    rsx! {
        section { class: "bx-panel",
            if let Some(title) = title {
                h2 { "{title}" }
            }
            {children}
        }
    }
}

/// A titled group inside a panel.
#[component]
pub fn Section(title: String, children: Element) -> Element {
    rsx! {
        div { class: "bx-section",
            h3 { class: "bx-section-title", "{title}" }
            {children}
        }
    }
}

/// Decorative spinning ring; pair it with a text that says what runs.
#[component]
pub fn Spinner() -> Element {
    rsx! { span { class: "bx-spinner", "aria-hidden": "true" } }
}

/// Button for a long action: spinner and `busy_label` while `running`.
#[component]
pub fn BusyButton(
    label: String,
    busy_label: String,
    running: bool,
    disabled: bool,
    onclick: EventHandler<()>,
    #[props(default = "bx-btn".to_string())] class: String,
) -> Element {
    rsx! {
        button {
            class: "{class} bx-busy",
            disabled,
            "aria-busy": if running { "true" } else { "false" },
            onclick: move |_| onclick.call(()),
            if running {
                Spinner {}
                "{busy_label}"
            } else {
                "{label}"
            }
        }
    }
}

/// A bordered message in a tone: a verdict, a result, an error. Danger is announced.
#[component]
pub fn Notice(#[props(default)] tone: Tone, title: String, children: Element) -> Element {
    let class = match tone.modifier() {
        "" => "bx-notice".to_string(),
        m => format!("bx-notice {m}"),
    };
    rsx! {
        div { class, role: (tone == Tone::Danger).then_some("alert"),
            span { class: "bx-notice-title", "{title}" }
            {children}
        }
    }
}

/// A compact toggle in a row of choices. `hinted` marks a suggested value.
#[component]
pub fn Chip(
    selected: bool,
    #[props(default)] hinted: bool,
    onclick: EventHandler<()>,
    children: Element,
) -> Element {
    rsx! {
        button {
            class: "bx-chip",
            "aria-pressed": if selected { "true" } else { "false" },
            "data-hinted": hinted.then_some("true"),
            onclick: move |_| onclick.call(()),
            {children}
        }
    }
}
