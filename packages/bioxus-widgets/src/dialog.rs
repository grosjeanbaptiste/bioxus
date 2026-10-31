//! Modal dialog on the native `<dialog>` element.

use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct DialogProps {
    pub open: bool,
    pub title: String,
    pub on_close: EventHandler<()>,
    pub children: Element,
}

#[component]
pub fn Dialog(props: DialogProps) -> Element {
    let on_close = props.on_close;
    rsx! {
        dialog { class: "bx-dialog", open: props.open, oncancel: move |_| on_close.call(()),
            h2 { "{props.title}" }
            {props.children}
            button { class: "bx-nav-item", onclick: move |_| on_close.call(()), "Close" }
        }
    }
}

/// A question with a confirm and a cancel button; `danger` styles the confirm one as destructive.
#[component]
pub fn ConfirmDialog(
    open: bool,
    title: String,
    confirm_label: String,
    cancel_label: String,
    #[props(default)] danger: bool,
    on_confirm: EventHandler<()>,
    on_cancel: EventHandler<()>,
    children: Element,
) -> Element {
    let confirm_class = if danger {
        "bx-btn bx-danger"
    } else {
        "bx-btn bx-btn-primary"
    };
    rsx! {
        dialog { class: "bx-dialog", open, oncancel: move |_| on_cancel.call(()),
            h2 { "{title}" }
            {children}
            button { class: "bx-btn", onclick: move |_| on_cancel.call(()), "{cancel_label}" }
            button { class: confirm_class, onclick: move |_| on_confirm.call(()), "{confirm_label}" }
        }
    }
}
