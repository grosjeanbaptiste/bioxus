//! Responsive shell and widgets for Dioxus apps. Renderer-agnostic: no router, no JS,
//! no storage. Colours and spacing come from `bioxus-design` tokens only.

pub mod badge;
pub mod controls;
pub mod dialog;
pub mod field;
pub mod nav;
pub mod shell;
pub mod stat;
pub mod table;

pub use badge::{Badge, Tone};
pub use controls::{BusyButton, Chip, Collapsible, Notice, Panel, Section, Spinner, TopBar};
pub use dialog::Dialog;
pub use field::{CheckField, InputField, InputKind, SelectField, TextAreaField};
pub use nav::{NavItem, NavLayout};
pub use shell::AppShell;
pub use stat::Stat;
pub use table::{DataTable, Table, TableRow};

/// Stylesheet of the widgets; inject after `bioxus_design::STYLESHEET`.
pub const STYLESHEET: &str = include_str!("widgets.css");

#[cfg(test)]
mod tests;
