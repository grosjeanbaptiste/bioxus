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

mod line {
    use crate::scale::polyline;
    use crate::LineChart;
    use dioxus::prelude::*;

    #[test]
    fn the_lowest_value_sits_at_the_bottom_and_the_highest_at_the_top() {
        assert_eq!(
            polyline(&[0.0, 10.0], 100.0, 50.0),
            "0.00,50.00 100.00,0.00"
        );
    }

    #[test]
    fn a_flat_series_and_a_single_point_stay_in_the_frame() {
        assert_eq!(
            polyline(&[3.0, 3.0], 100.0, 50.0),
            "0.00,25.00 100.00,25.00"
        );
        assert_eq!(polyline(&[3.0], 100.0, 50.0), "50.00,25.00");
    }

    #[test]
    fn a_non_finite_value_is_left_out_and_keeps_its_slot() {
        assert_eq!(
            polyline(&[0.0, f64::NAN, 10.0], 100.0, 50.0),
            "0.00,50.00 100.00,0.00"
        );
        assert_eq!(
            polyline(&[f64::INFINITY, 0.0, 10.0], 100.0, 50.0),
            "50.00,50.00 100.00,0.00"
        );
    }

    #[test]
    fn no_usable_value_gives_no_line() {
        assert_eq!(polyline(&[], 100.0, 50.0), "");
        assert_eq!(polyline(&[f64::NAN], 100.0, 50.0), "");
    }

    #[component]
    fn Host(values: Vec<f64>) -> Element {
        rsx! { LineChart { title: "Close", values, start: "2026-01", end: "2026-02" } }
    }

    fn render(values: Vec<f64>) -> String {
        let mut dom = VirtualDom::new_with_props(Host, HostProps { values });
        dom.rebuild_in_place();
        dioxus_ssr::render(&dom)
    }

    #[test]
    fn the_chart_is_named_scaled_and_labelled() {
        let html = render(vec![1.0, 2.0]);

        assert!(html.contains("role=\"img\"") && html.contains("aria-label=\"Close\""));
        assert!(
            html.contains("<polyline") && html.contains("viewBox"),
            "{html}"
        );
        assert!(html.contains("2026-01") && html.contains("2026-02"));
    }

    #[test]
    fn an_empty_series_says_so_instead_of_drawing_nothing() {
        let html = render(vec![f64::NAN]);

        assert!(!html.contains("<polyline"), "{html}");
        assert!(html.contains("bx-line-empty"), "{html}");
    }

    #[test]
    fn the_stylesheet_styles_the_line_in_tokens() {
        assert!(
            crate::STYLESHEET.contains(".bx-line") && crate::STYLESHEET.contains("var(--accent)")
        );
        assert!(!crate::STYLESHEET.contains("px"));
    }
}
