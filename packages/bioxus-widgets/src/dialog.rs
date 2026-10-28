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
