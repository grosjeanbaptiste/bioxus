use super::*;
use dioxus::prelude::*;

#[component]
fn Host(more_label: Option<String>, active: String, with_secondary: bool) -> Element {
    let mut items = vec![NavItem::new("a", "Alpha"), NavItem::new("b", "Beta")];
    if with_secondary {
        items.push(NavItem::new("c", "Gamma").secondary());
        items.push(NavItem::new("d", "Delta").secondary());
    }
    rsx! { AppShell { items, active, on_select: |_| {}, more_label, "content" } }
}

fn render(more: Option<&str>, active: &str, with_secondary: bool) -> String {
    let props = HostProps {
        more_label: more.map(Into::into),
        active: active.into(),
        with_secondary,
    };
    let mut dom = VirtualDom::new_with_props(Host, props);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

#[test]
fn without_a_label_secondary_items_stay_in_the_bar() {
    let html = render(None, "a", true);
    assert!(!html.contains("bx-nav-more"));
    for name in ["Alpha", "Beta", "Gamma", "Delta"] {
        assert!(html.contains(name), "missing {name}");
    }
}

#[test]
fn without_secondary_items_there_is_no_drawer_even_with_a_label() {
    assert!(!render(Some("More"), "a", false).contains("bx-nav-more"));
}

#[test]
fn secondary_items_sit_in_a_closed_drawer_after_the_toggle() {
    let html = render(Some("More"), "a", true);
    assert!(html.contains("aria-expanded=\"false\"") && html.contains(">More<"));
    let toggle = html.find("bx-nav-more-toggle").unwrap_or(usize::MAX);
    let list = html.find("bx-nav-more-list").unwrap_or(usize::MAX);
    assert!(toggle < list, "toggle comes before its list");
    assert!(html[list..].contains("hidden"), "the drawer starts closed");
    assert!(
        html.find("Beta") < Some(toggle),
        "primary items stay in the bar"
    );
    assert!(html.find("Gamma") > Some(list) && html.find("Delta") > Some(list));
}

#[test]
fn the_toggle_shows_when_the_active_item_is_in_the_drawer() {
    assert!(render(Some("More"), "c", true).contains("data-active=\"true\""));
    assert!(!render(Some("More"), "a", true).contains("data-active"));
}

#[test]
fn the_active_secondary_item_is_still_marked_current_once() {
    let html = render(Some("More"), "d", true);
    assert_eq!(html.matches("aria-current=\"page\"").count(), 1);
}
