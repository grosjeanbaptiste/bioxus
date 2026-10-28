use super::*;
use dioxus::prelude::*;

#[component]
fn Host(max: Option<f64>) -> Element {
    let bars = vec![
        Bar::new("a", 5.0, "5"),
        Bar {
            tone: Tone::Danger,
            ..Bar::new("b", 10.0, "10")
        },
        Bar::new("c", f64::NAN, "?"),
    ];
    rsx! { BarChart { title: "Spread", bars, max } }
}

fn render(max: Option<f64>) -> String {
    let mut dom = VirtualDom::new_with_props(Host, HostProps { max });
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

#[test]
fn widths_follow_the_largest_bar() {
    let html = render(None);
    assert!(html.contains("width: 50.00%;") && html.contains("width: 100.00%;"));
}

#[test]
fn an_explicit_max_rescales_and_a_nan_bar_is_empty() {
    let html = render(Some(20.0));
    assert!(html.contains("width: 25.00%;") && html.contains("width: 0.00%;"));
}

#[test]
fn the_chart_is_named_and_tones_become_classes() {
    let html = render(None);
    assert!(html.contains("role=\"img\"") && html.contains("aria-label=\"Spread\""));
    assert!(html.contains("bx-bar-fill bx-danger") && html.contains("title=\"a: 5\""));
}

#[test]
fn an_empty_chart_renders_no_rows() {
    #[component]
    fn Empty() -> Element {
        rsx! { BarChart { title: "t", bars: vec![] } }
    }
    let mut dom = VirtualDom::new(Empty);
    dom.rebuild_in_place();
    assert!(!dioxus_ssr::render(&dom).contains("bx-bar\""));
}

#[test]
fn styles_use_only_design_tokens_and_no_px() {
    for t in STYLESHEET.split("var(--").skip(1) {
        let name = t.split(')').next().unwrap();
        assert!(
            bioxus_design::TOKENS.contains(&name),
            "unknown token --{name}"
        );
    }
    assert!(!STYLESHEET.contains("px"));
}
