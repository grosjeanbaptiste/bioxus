use super::*;

fn part(name: &str, bytes: &[u8]) -> Part {
    Part::new(name, bytes.to_vec())
}

#[test]
fn the_page_embeds_every_part_by_name() {
    let html = page(
        "Glasses",
        &[part("front", b"glTF-front"), part("pin", b"glTF-pin")],
    );

    assert!(html.contains(&base64::engine::general_purpose::STANDARD.encode(b"glTF-front")));
    assert!(html.contains("\"pin\""));
    assert!(html.contains("<title>Glasses</title>"));
}

#[test]
fn the_page_pins_the_threejs_version_it_loads() {
    let html = page("x", &[]);

    assert!(html.contains(&format!("three@{THREE_VERSION}/build/three.module.js")));
    assert!(html.contains(&format!("three@{THREE_VERSION}/examples/jsm/")));
}

#[test]
fn a_title_cannot_inject_markup() {
    let html = page("<script>x</script> & co", &[]);

    assert!(!html.contains("<script>x</script>"));
    assert!(html.contains("&lt;script&gt;x&lt;/script&gt; &amp; co"));
}

#[test]
fn a_part_name_cannot_close_the_script() {
    let html = page("t", &[part("a</script><b>", b"x")]);

    assert!(!html.contains("a</script><b>"));
}

#[test]
fn no_part_still_makes_a_page() {
    let html = page("empty", &[]);

    assert!(html.starts_with("<!doctype html>") && html.contains("const PARTS = [];"));
}

#[test]
fn the_page_has_no_placeholder_left() {
    let html = page("t", &[part("p", b"x")]);

    assert!(!html.contains("__"), "an unfilled placeholder remains");
}

#[cfg(feature = "component")]
mod component {
    use super::*;
    use dioxus::prelude::*;

    #[component]
    fn Host() -> Element {
        rsx! { Viewer3d { title: "A \"quoted\" <part>", parts: vec![part("front", b"x")] } }
    }

    fn render() -> String {
        let mut dom = VirtualDom::new(Host);
        dom.rebuild_in_place();
        dioxus_ssr::render(&dom)
    }

    #[test]
    fn the_viewer_is_a_named_frame_that_holds_the_page() {
        let html = render();

        assert!(html.contains("<iframe") && html.contains("class=\"bx-viewer3d\""));
        assert!(html.contains("title=\"A &quot;quoted&quot; &lt;part&gt;\""));
        assert!(html.contains("srcdoc="), "the page travels in srcdoc");
        assert!(html.contains("three@"), "the page is the one `page` builds");
    }

    #[test]
    fn the_stylesheet_sizes_the_frame_in_rem_only() {
        assert!(STYLESHEET.contains(".bx-viewer3d"));
        assert!(!STYLESHEET.contains("px"));
    }
}
