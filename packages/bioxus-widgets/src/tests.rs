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
