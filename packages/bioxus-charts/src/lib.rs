//! Charts for Dioxus apps. Geometry is pure ([`scale`]); components only draw it.

pub mod bar;
pub mod line;
pub mod scale;

pub use bar::{Bar, BarChart};
pub use bioxus_widgets::Tone;
pub use line::LineChart;

/// Stylesheet of the charts; inject after the design and widgets ones.
pub const STYLESHEET: &str = include_str!("charts.css");

#[cfg(test)]
mod tests;
