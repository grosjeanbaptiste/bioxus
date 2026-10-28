use super::*;

fn scratch(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("bioxus-ios-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir); // fresh start; a missing dir is the normal case
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn write_set(dir: &Path, skip: &[u32]) {
    std::fs::create_dir_all(dir).unwrap();
    for (n, bytes) in DEFAULT_SET.iter().filter(|(n, _)| !skip.contains(n)) {
        std::fs::write(dir.join(file_name(*n)), bytes).unwrap();
    }
}

#[test]
fn the_default_set_has_every_size_exactly_and_no_alpha() {
    for (n, bytes) in DEFAULT_SET {
        assert_eq!(png_dimensions(bytes), Some((n, n)), "icon_{n}");
        assert_eq!(bytes[25], 2, "color type 2 = RGB without alpha, icon_{n}");
    }
}

#[test]
fn an_app_without_any_icon_gets_the_bioxus_set() {
    let set = resolve_set(&scratch("none")).unwrap();
    assert_eq!(set.iter().map(|(n, _)| *n).collect::<Vec<_>>(), SIZES);
}

#[test]
fn an_apps_own_set_is_used_from_either_convention() {
    for dir in ["stores/ios/AppIcon.appiconset", "ui/mobile/assets/icons"] {
        let root = scratch("own");
        write_set(&root.join(dir), &[]);
        let marked: Vec<u8> = [DEFAULT_SET[0].1, b"own"].concat(); // still a 20x20 PNG header
        std::fs::write(root.join(dir).join(file_name(20)), &marked).unwrap();
        let set = resolve_set(&root).unwrap();
        assert_eq!(
            set[0],
            (20, marked),
            "{dir}: the app's file wins over the default"
        );
    }
}

#[test]
fn a_partial_set_is_an_error_naming_the_missing_sizes() {
    let root = scratch("partial");
    write_set(&root.join("assets/icons"), &[167, 1024]);
    match resolve_set(&root) {
        Err(IconError::IncompleteSet { missing, .. }) => assert_eq!(missing, [167, 1024]),
        other => panic!("expected IncompleteSet, got {other:?}"),
    }
}

#[test]
fn a_wrong_sized_or_corrupt_file_is_an_error() {
    let root = scratch("wrong");
    write_set(&root.join("assets/icons"), &[]);
    std::fs::write(
        root.join("assets/icons").join(file_name(20)),
        DEFAULT_SET[12].1,
    )
    .unwrap();
    assert!(matches!(resolve_set(&root), Err(IconError::Invalid { .. })));
}

#[test]
fn a_master_icon_without_a_set_is_never_replaced_by_the_default() {
    let root = scratch("master");
    std::fs::write(root.join("icon.png"), DEFAULT_SET[12].1).unwrap();
    assert!(matches!(
        resolve_set(&root),
        Err(IconError::NoSetForMaster { .. })
    ));
}

#[test]
fn emit_writes_the_whole_set() {
    let root = scratch("emit");
    let dir = emit_set(&root, &root).unwrap();
    for n in SIZES {
        assert!(dir.join(file_name(n)).is_file(), "icon_{n}");
    }
}
