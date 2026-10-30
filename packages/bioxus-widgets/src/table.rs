//! Data tables: [`Table`] when cells are text, [`DataTable`] and [`TableRow`] for rich cells
//! or selectable rows. Both scroll horizontally on narrow screens.

use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct TableProps {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

#[component]
pub fn Table(props: TableProps) -> Element {
    rsx! {
        div { class: "bx-table-wrap",
            table { class: "bx-table",
                thead { tr { for c in props.columns.iter() { th { scope: "col", "{c}" } } } }
                tbody {
                    for (i, row) in props.rows.iter().enumerate() {
                        tr { key: "{i}", for cell in row.iter() { td { "{cell}" } } }
                    }
                }
            }
        }
    }
}

/// A table whose rows the app writes: put [`TableRow`]s or plain `tr`s in it.
#[component]
pub fn DataTable(columns: Vec<String>, children: Element) -> Element {
    rsx! {
        div { class: "bx-table-wrap",
            table { class: "bx-table",
                thead { tr { for c in columns.iter() { th { scope: "col", "{c}" } } } }
                tbody { {children} }
            }
        }
    }
}

/// A row; with `on_select` it answers to the click and to the Enter key.
#[component]
pub fn TableRow(
    #[props(default)] selected: bool,
    #[props(default)] on_select: Option<EventHandler<()>>,
    children: Element,
) -> Element {
    rsx! {
        tr {
            class: on_select.is_some().then_some("bx-row-action"),
            tabindex: on_select.is_some().then_some(0),
            "aria-selected": selected.then_some("true"),
            onclick: move |_| {
                if let Some(select) = on_select {
                    select.call(());
                }
            },
            onkeydown: move |e| {
                if e.key() == Key::Enter {
                    if let Some(select) = on_select {
                        select.call(());
                    }
                }
            },
            {children}
        }
    }
}
