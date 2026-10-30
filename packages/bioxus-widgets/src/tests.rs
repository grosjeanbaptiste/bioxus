use super::*;
use dioxus::prelude::*;

#[component]
fn Host(layout: Option<NavLayout>, active: String) -> Element {
    let items = vec![
        NavItem::new("home", "Home").with_icon("H"),
        NavItem::new("log", "Log"),
    ];
    rsx! {
        AppShell { items, active, on_select: |_| {}, layout, title: "MyApp".to_string(), "content" }
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
    let ok = |t: &str| bioxus_design::TOKENS.contains(&t) || ["gutter", "columns"].contains(&t);
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

#[test]
fn title_names_the_nav_and_icons_are_decorative() {
    let html = render(None, "home");
    assert!(html.contains("aria-label=\"MyApp\"") && html.contains("class=\"bx-nav-title\""));
    assert_eq!(
        html.matches("bx-nav-icon").count(),
        1,
        "only the item with an icon gets one"
    );
    assert!(html.contains("aria-hidden=\"true\""));
}

#[component]
fn Bricks(running: bool, selected: bool, hinted: bool) -> Element {
    rsx! {
        TopBar { action: rsx! { button { class: "bx-btn", "⚙" } }, b { "App" } span { "3 / 10" } }
        Panel { title: "Pairs",
            Section { title: "Mine", p { "x" } }
        }
        BusyButton { label: "Install", busy_label: "Installing…", running, disabled: false, onclick: |_| {} }
        Notice { tone: Tone::Danger, title: "Wrong", "expected: x" }
        Chip { selected, hinted, onclick: |_| {}, "B1" }
    }
}

fn render_bricks(running: bool, selected: bool, hinted: bool) -> String {
    let props = BricksProps {
        running,
        selected,
        hinted,
    };
    let mut dom = VirtualDom::new_with_props(Bricks, props);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

#[test]
fn top_bar_puts_the_action_last() {
    let html = render_bricks(false, false, false);
    let bar = &html[html.find("bx-topbar").unwrap()..];
    assert!(bar.find("3 / 10").unwrap() < bar.find("bx-topbar-action").unwrap());
}

#[test]
fn panel_titles_itself_and_its_sections() {
    let html = render_bricks(false, false, false);
    assert!(html.contains("<h2>Pairs</h2>") && html.contains("class=\"bx-section-title\">Mine<"));
}

#[test]
fn busy_button_shows_a_spinner_and_the_busy_label_only_while_running() {
    let idle = render_bricks(false, false, false);
    assert!(idle.contains("aria-busy=\"false\"") && idle.contains(">Install<"));
    assert!(!idle.contains("bx-spinner"));
    let busy = render_bricks(true, false, false);
    assert!(busy.contains("aria-busy=\"true\"") && busy.contains("bx-spinner"));
    assert!(busy.contains("Installing…") && !busy.contains(">Install<"));
}

#[test]
fn notice_carries_its_tone_and_is_announced_when_danger() {
    let html = render_bricks(false, false, false);
    assert!(html.contains("class=\"bx-notice bx-danger\"") && html.contains("role=\"alert\""));
    assert!(html.contains("class=\"bx-notice-title\">Wrong<"));
}

#[test]
fn chip_states_are_attributes() {
    let plain = render_bricks(false, false, false);
    assert!(plain.contains("aria-pressed=\"false\"") && !plain.contains("data-hinted"));
    let both = render_bricks(false, true, true);
    assert!(both.contains("aria-pressed=\"true\"") && both.contains("data-hinted=\"true\""));
}

#[test]
fn button_classes_are_styled() {
    for class in [
        ".bx-btn ",
        ".bx-btn-primary",
        ".bx-option",
        ".bx-grid",
        ".bx-spinner",
    ] {
        assert!(STYLESHEET.contains(class), "{class}");
    }
}

#[component]
fn MoreForms(error: Option<String>, open: bool) -> Element {
    let options = vec![
        ("eur".to_string(), "Euro".to_string()),
        ("usd".to_string(), "Dollar".to_string()),
    ];
    rsx! {
        SelectField { label: "Currency", value: "usd", options, on_change: |_| {}, error: error.clone() }
        TextAreaField { label: "Notes", value: "a <b>", on_input: |_| {}, error }
        Collapsible { summary: "Journal", open, "inside" }
    }
}

fn render_more_forms(error: Option<&str>, open: bool) -> String {
    let props = MoreFormsProps {
        error: error.map(Into::into),
        open,
    };
    let mut dom = VirtualDom::new_with_props(MoreForms, props);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

#[test]
fn a_select_marks_only_the_current_option() {
    let html = render_more_forms(None, false);

    assert!(html.contains("Currency") && html.contains("<select"));
    assert!(html.contains("value=\"eur\"") && html.contains("Dollar"));
    assert_eq!(html.matches("selected").count(), 1, "{html}");
    let usd = html.find("value=\"usd\"").expect("the usd option");
    assert!(
        html[usd..].find("selected").expect("selected")
            < html[usd..].find("</option>").expect("end")
    );
}

#[test]
fn a_text_area_escapes_its_value() {
    let html = render_more_forms(None, false);

    assert!(html.contains("Notes") && html.contains("<textarea"));
    assert!(!html.contains("a <b>"), "{html}");
}

#[test]
fn select_and_text_area_announce_an_error() {
    let html = render_more_forms(Some("wrong"), false);

    assert_eq!(html.matches("role=\"alert\"").count(), 2, "{html}");
    assert_eq!(html.matches("aria-invalid=\"true\"").count(), 2, "{html}");
}

#[test]
fn a_valid_select_and_text_area_announce_nothing() {
    let html = render_more_forms(None, false);

    assert!(
        !html.contains("role=\"alert\"") && !html.contains("aria-invalid"),
        "{html}"
    );
}

#[test]
fn a_collapsible_is_native_and_follows_its_open_prop() {
    let closed = render_more_forms(None, false);
    let open = render_more_forms(None, true);

    assert!(closed.contains("<summary") && closed.contains("Journal") && closed.contains("inside"));
    assert!(!closed.contains(" open"), "{closed}");
    assert!(open.contains(" open"), "{open}");
}
