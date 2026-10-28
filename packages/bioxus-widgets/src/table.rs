//! A plain data table; cells are text. Scrolls horizontally on narrow screens.

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
