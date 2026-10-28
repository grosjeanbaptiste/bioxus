//! The viewer as a Dioxus component: the page of [`crate::page`] in a frame.

use crate::{page, Part};
use dioxus::prelude::*;

/// Stylesheet of the viewer; inject after the design one.
pub const STYLESHEET: &str = include_str!("viewer3d.css");

#[derive(Clone, PartialEq, Props)]
pub struct Viewer3dProps {
    /// Accessible name of the frame, and the heading inside it.
    pub title: String,
    pub parts: Vec<Part>,
}

/// Orbit, zoom and show or hide each part. The scene lives in a frame, so it works on every
/// renderer without JavaScript glue.
#[component]
pub fn Viewer3d(props: Viewer3dProps) -> Element {
    let srcdoc = page(&props.title, &props.parts);
    rsx! { iframe { class: "bx-viewer3d", title: "{props.title}", srcdoc: "{srcdoc}" } }
}
