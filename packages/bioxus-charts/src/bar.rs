//! Horizontal proportional bars, one row per entry.

use crate::scale::{fill, max_of};
use bioxus_widgets::Tone;
use dioxus::prelude::*;

/// One row of a [`BarChart`].
#[derive(Clone, Debug, PartialEq)]
pub struct Bar {
    pub label: String,
    pub value: f64,
    /// Text shown at the end of the row (units, grouping, percent: the app decides).
    pub display: String,
    pub tone: Tone,
}

impl Bar {
    pub fn new(label: impl Into<String>, value: f64, display: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value,
            display: display.into(),
            tone: Tone::Neutral,
        }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct BarChartProps {
    /// Accessible name of the whole chart.
    pub title: String,
    pub bars: Vec<Bar>,
    /// Scale ceiling; defaults to the largest bar.
    #[props(default)]
    pub max: Option<f64>,
}

#[component]
pub fn BarChart(props: BarChartProps) -> Element {
    let values: Vec<f64> = props.bars.iter().map(|b| b.value).collect();
    let max = props.max.unwrap_or_else(|| max_of(&values));
    rsx! {
        div { class: "bx-bars", role: "img", "aria-label": "{props.title}",
            for (i, bar) in props.bars.iter().enumerate() {
                div { key: "{i}", class: "bx-bar", title: "{bar.label}: {bar.display}",
                    span { class: "bx-bar-label", "{bar.label}" }
                    span { class: "bx-bar-track",
                        span {
                            class: "bx-bar-fill {bar.tone.modifier()}",
                            style: "width: {fill(bar.value, max) * 100.0:.2}%;",
                        }
                    }
                    span { class: "bx-bar-value", "{bar.display}" }
                }
            }
        }
    }
}
