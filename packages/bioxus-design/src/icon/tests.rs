use super::*;

fn scratch(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("bioxus-icon-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir); // fresh start; a missing dir is the normal case
    std::fs::create_dir_all(dir.join("assets")).unwrap();
    dir
}

#[test]
fn the_embedded_default_is_a_real_512_png_and_a_real_svg() {
    assert_eq!(kind_of(DEFAULT_PNG), Some(Kind::Png));
    assert_eq!(
        u32::from_be_bytes(DEFAULT_PNG[16..20].try_into().unwrap()),
        512
    );
    assert_eq!(kind_of(DEFAULT_SVG.as_bytes()), Some(Kind::Svg));
}

#[test]
fn an_app_without_an_icon_gets_the_bioxus_one() {
    let dir = scratch("none");
    let icon = resolve(&dir).unwrap();
    assert_eq!((icon.source, icon.kind), (Source::Default, Kind::Svg));
    assert_eq!(
        std::fs::read(emit(&dir, &dir).unwrap()).unwrap(),
        DEFAULT_PNG
    );
}

#[test]
fn a_provided_icon_wins_and_svg_is_preferred_to_png() {
    let dir = scratch("own");
    std::fs::write(dir.join("assets/icon.png"), DEFAULT_PNG).unwrap();
    std::fs::write(dir.join("icon.svg"), "<svg/>").unwrap();
    assert_eq!(
        resolve(&dir).unwrap().kind,
        Kind::Png,
        "assets/ comes before the root"
    );
    std::fs::write(dir.join("assets/icon.svg"), "<svg id='own'/>").unwrap();
    let icon = resolve(&dir).unwrap();
    assert_eq!(
        (icon.kind, icon.source),
        (Kind::Svg, Source::Provided(dir.join("assets/icon.svg")))
    );
    assert!(emit(&dir, &dir).unwrap().ends_with("app-icon.svg"));
}

#[test]
fn a_corrupt_provided_icon_is_an_error_not_a_silent_default() {
    let dir = scratch("bad");
    std::fs::write(dir.join("assets/icon.png"), b"not a png").unwrap();
    assert!(matches!(resolve(&dir), Err(IconError::Invalid { .. })));
    assert!(emit(&dir, &dir).is_err());
}

#[test]
fn an_unreadable_candidate_is_reported_not_skipped() {
    let dir = scratch("dir");
    std::fs::create_dir(dir.join("assets/icon.svg")).unwrap(); // a directory where a file should be
    assert!(matches!(resolve(&dir), Err(IconError::Unreadable { .. })));
}

#[test]
fn png_resolution_ignores_svg_and_falls_back_to_the_default() {
    let dir = scratch("png");
    std::fs::write(dir.join("assets/icon.svg"), "<svg id='own'/>").unwrap();
    assert_eq!(
        resolve_png(&dir).unwrap(),
        DEFAULT_PNG,
        "svg-only app still gets a png"
    );
    let own: Vec<u8> = [DEFAULT_PNG, b"own"].concat();
    std::fs::write(dir.join("icon.png"), &own).unwrap();
    assert_eq!(std::fs::read(emit_png(&dir, &dir).unwrap()).unwrap(), own);
}

#[test]
fn a_corrupt_png_is_an_error_even_when_an_svg_exists() {
    let dir = scratch("badpng");
    std::fs::write(dir.join("assets/icon.svg"), "<svg/>").unwrap();
    std::fs::write(dir.join("assets/icon.png"), b"junk").unwrap();
    assert!(matches!(resolve_png(&dir), Err(IconError::Invalid { .. })));
}

#[test]
fn the_1024_of_an_ios_set_counts_as_the_apps_own_icon() {
    let dir = scratch("set1024");
    let set = dir.join("ui/mobile/assets/icons");
    std::fs::create_dir_all(&set).unwrap();
    std::fs::write(set.join("icon_1024x1024.png"), ios::DEFAULT_SET[12].1).unwrap();
    assert_eq!(
        resolve(&dir).unwrap().source,
        Source::Provided(set.join("icon_1024x1024.png"))
    );
    assert_eq!(resolve_png(&dir).unwrap(), ios::DEFAULT_SET[12].1);
}
