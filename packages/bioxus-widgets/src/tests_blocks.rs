use super::*;
use dioxus::prelude::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

fn path() -> Vec<NavItem> {
    vec![
        NavItem::new("a", "Home"),
        NavItem::new("b", "Docs"),
        NavItem::new("c", "Page"),
    ]
}

#[test]
fn initials_take_two_capitals_and_survive_edge_cases() {
    assert_eq!(display::initials("ada lovelace byron"), "AL");
    assert_eq!(display::initials("  grace "), "G");
    assert_eq!(display::initials(""), "");
    assert_eq!(display::initials("élan"), "É");
}

#[test]
fn avatar_shows_the_picture_or_labelled_initials() {
    let with = render(|| rsx! { Avatar { name: "Ada Lovelace", src: "a.png".to_string() } });
    assert!(with.contains("<img") && with.contains("alt=\"Ada Lovelace\""));
    let without = render(|| rsx! { Avatar { name: "Ada Lovelace" } });
    assert!(without.contains("aria-label=\"Ada Lovelace\"") && without.contains(">AL<"));
}

#[test]
fn empty_state_and_key_value_row_render_their_text() {
    let html = render(|| {
        rsx! {
            EmptyState { title: "Nothing", action: rsx! { button { "Add" } }, "Add a first one" }
            KeyValueRow { label: "Plan", value: "Free" }
        }
    });
    for needle in [
        "Nothing",
        "Add a first one",
        "<button>Add</button>",
        "<dt>Plan</dt>",
        "<dd>Free</dd>",
    ] {
        assert!(html.contains(needle), "missing {needle}");
    }
}

#[test]
fn status_carries_a_symbol_and_words_for_every_tone() {
    let symbols = [Tone::Neutral, Tone::Ok, Tone::Warn, Tone::Danger].map(Tone::symbol);
    for (i, a) in symbols.iter().enumerate() {
        assert!(
            symbols[i + 1..].iter().all(|b| a != b),
            "tones share a symbol"
        );
    }
    let html = render(|| rsx! { StatusIndicator { tone: Tone::Danger, label: "Failed" } });
    assert!(html.contains("bx-status bx-danger") && html.contains("role=\"status\""));
    assert!(html.contains("aria-hidden=\"true\"") && html.contains("Failed"));
}

#[test]
fn settings_card_shows_its_description_only_when_given() {
    let with = render(
        || rsx! { SettingsCard { title: "Sync", description: "Every hour".to_string(), "body" } },
    );
    assert!(with.contains("Every hour") && with.contains("bx-settings"));
    let without = render(|| rsx! { SettingsCard { title: "Sync", "body" } });
    assert!(!without.contains("bx-hint"));
}

#[test]
fn breadcrumb_links_all_but_the_last_item() {
    let html =
        render(|| rsx! { Breadcrumb { label: "You are here", items: path(), on_select: |_| {} } });
    assert_eq!(html.matches("<button").count(), 2);
    assert_eq!(html.matches("aria-current=\"page\"").count(), 1);
    assert!(html.contains("aria-label=\"You are here\""));
    assert!(html.find("aria-current").unwrap() > html.find("Docs").unwrap());
}

#[test]
fn breadcrumb_of_nothing_renders_no_item_and_does_not_panic() {
    let html = render(|| rsx! { Breadcrumb { label: "x", items: vec![], on_select: |_| {} } });
    assert!(!html.contains("<li"));
}

#[test]
fn tabs_mark_only_the_active_one_selected() {
    let html =
        render(|| rsx! { Tabs { label: "Views", items: path(), active: "b", on_select: |_| {} } });
    assert_eq!(html.matches("aria-selected=\"true\"").count(), 1);
    assert_eq!(html.matches("role=\"tab\"").count(), 3);
    assert!(html.contains("role=\"tablist\""));
}

#[test]
fn a_disabled_tab_is_not_selectable_and_an_unknown_active_selects_none() {
    let html = render(|| {
        rsx! {
            Tabs { label: "v", items: vec![NavItem::new("a", "A").disabled()], active: "zz", on_select: |_| {} }
        }
    });
    assert!(html.contains("disabled") && !html.contains("aria-selected=\"true\""));
}

#[test]
fn switch_is_a_checkbox_with_the_switch_role() {
    let html = render(|| rsx! { SwitchField { label: "Sync", checked: true, on_change: |_| {} } });
    assert!(
        html.contains("type=\"checkbox\"")
            && html.contains("role=\"switch\"")
            && html.contains("checked")
    );
}

#[test]
fn confirm_dialog_has_both_buttons_and_styles_danger() {
    let ask = |danger: bool| {
        let mut dom = VirtualDom::new_with_props(Ask, AskProps { danger });
        dom.rebuild_in_place();
        dioxus_ssr::render(&dom)
    };
    let html = ask(true);
    assert!(html.contains("open") && html.contains("Keep") && html.contains("Delete"));
    assert!(html.contains("bx-btn bx-danger") && !ask(false).contains("bx-btn bx-danger"));
}

#[component]
fn Ask(danger: bool) -> Element {
    rsx! {
        ConfirmDialog {
            open: true, title: "Sure?", confirm_label: "Delete", cancel_label: "Keep",
            danger, on_confirm: |_| {}, on_cancel: |_| {}, "body"
        }
    }
}
