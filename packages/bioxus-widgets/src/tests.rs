use super::*;
use dioxus::prelude::*;

#[component]
fn Host(layout: Option<NavLayout>, active: String) -> Element {
    let items = vec![NavItem::new("home", "Home"), NavItem::new("log", "Log")];
    rsx! {
        AppShell { items, active, on_select: |_| {}, layout, "content" }
        Badge { tone: Tone::Danger, "bad" }
    }
}

fn render(layout: Option<NavLayout>, active: &str) -> String {
    let mut dom = VirtualDom::new_with_props(
        Host,
        HostProps {
            layout,
            active: active.into(),
        },
    );
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

#[test]
fn renders_every_item_and_the_content() {
    let html = render(None, "home");
    for needle in ["Home", "Log", "content", "bx-main"] {
        assert!(html.contains(needle), "missing {needle}");
    }
}

#[test]
fn only_the_active_item_is_marked_current() {
    let html = render(None, "log");
    assert_eq!(html.matches("aria-current=\"page\"").count(), 1);
    assert!(html.find("aria-current").unwrap() > html.find("Home").unwrap());
}

#[test]
fn an_unknown_active_id_marks_nothing() {
    assert!(!render(None, "nope").contains("aria-current"));
}

#[test]
fn layout_is_responsive_unless_forced() {
    assert!(!render(None, "home").contains("data-layout"));
    assert!(render(Some(NavLayout::Rail), "home").contains("data-layout=\"rail\""));
}

#[test]
fn badge_carries_its_tone_class() {
    assert!(render(None, "home").contains("bx-badge bx-danger"));
}

#[test]
fn styles_use_only_design_tokens_and_no_px() {
    let ok = |t: &str| bioxus_design::TOKENS.contains(&t) || t == "gutter";
    for t in STYLESHEET.split("var(--").skip(1) {
        let name = t.split(')').next().unwrap();
        assert!(ok(name), "unknown token --{name}");
    }
    assert!(!STYLESHEET.contains("px"));
}

#[component]
fn Forms(error: Option<String>, open: bool) -> Element {
    rsx! {
        InputField { label: "Amount", value: "12", on_input: |_| {}, kind: InputKind::Number, error }
        Table { columns: vec!["A".into(), "B".into()], rows: vec![vec!["1".into(), "2".into()], vec!["3".into(), "4".into()]] }
        Dialog { open, title: "Sure?", on_close: |_| {}, "body" }
        Stat { label: "Total", value: "42" }
    }
}

fn render_forms(error: Option<&str>, open: bool) -> String {
    let props = FormsProps {
        error: error.map(Into::into),
        open,
    };
    let mut dom = VirtualDom::new_with_props(Forms, props);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

#[test]
fn input_uses_the_native_type_and_value() {
    let html = render_forms(None, false);
    assert!(html.contains("type=\"number\"") && html.contains("value=\"12\""));
    assert!(!html.contains("aria-invalid") && !html.contains("bx-error"));
}

#[test]
fn an_error_marks_the_input_invalid_and_is_announced() {
    let html = render_forms(Some("too big"), false);
    assert!(
        html.contains("aria-invalid=\"true\"")
            && html.contains("role=\"alert\"")
            && html.contains("too big")
    );
}

#[test]
fn table_renders_headers_and_every_cell() {
    let html = render_forms(None, false);
    assert_eq!(html.matches("<th ").count(), 2);
    assert_eq!(html.matches("<td>").count(), 4);
}

#[test]
fn dialog_is_open_only_when_asked() {
    assert!(render_forms(None, true).contains("<dialog class=\"bx-dialog\" open"));
    assert!(!render_forms(None, false).contains("<dialog class=\"bx-dialog\" open"));
}

#[test]
fn input_kinds_map_to_native_types() {
    assert_eq!(
        [InputKind::Text, InputKind::Number, InputKind::Date].map(InputKind::as_str),
        ["text", "number", "date"]
    );
}
