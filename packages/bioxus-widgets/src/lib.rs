//! Responsive shell and widgets for Dioxus apps. Renderer-agnostic: no router, no JS,
//! no storage. Colours and spacing come from `bioxus-design` tokens only.

pub mod badge;
pub mod nav;
pub mod shell;

pub use badge::{Badge, Tone};
pub use nav::{NavItem, NavLayout};
pub use shell::AppShell;

/// Stylesheet of the widgets; inject after `bioxus_design::STYLESHEET`.
pub const STYLESHEET: &str = include_str!("widgets.css");

#[cfg(test)]
mod tests;
