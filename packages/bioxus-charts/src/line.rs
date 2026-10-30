//! A line through a series, stretched to the width of its container.

use crate::scale::polyline;
use dioxus::prelude::*;

const WIDTH: f64 = 800.0;
const HEIGHT: f64 = 280.0;

#[derive(Clone, PartialEq, Props)]
pub struct LineChartProps {
    /// Accessible name of the whole chart.
    pub title: String,
    pub values: Vec<f64>,
    /// Axis label under the first value (a date, a period: the app decides).
    #[props(default)]
    pub start: String,
    /// Axis label under the last value.
    #[props(default)]
    pub end: String,
}

#[component]
pub fn LineChart(props: LineChartProps) -> Element {
    let points = polyline(&props.values, WIDTH, HEIGHT);
    if points.is_empty() {
        return rsx! { p { class: "bx-line-empty", "∅ {props.title}" } };
    }
    rsx! {
        div { class: "bx-line",
            svg {
                role: "img",
                "aria-label": "{props.title}",
                view_box: "0 0 {WIDTH} {HEIGHT}",
                preserve_aspect_ratio: "none",
                polyline { points }
            }
            div { class: "bx-line-axis",
                span { "{props.start}" }
                span { "{props.end}" }
            }
        }
    }
}
