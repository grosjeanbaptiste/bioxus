use super::*;
use std::sync::{Arc, Mutex};

fn loc(t: &str) -> Locale {
    Locale::new(t).unwrap()
}

fn translator() -> (Translator, Arc<Mutex<Vec<String>>>) {
    let missing = Arc::new(Mutex::new(Vec::new()));
    let sink = Arc::clone(&missing);
    let mut t = Translator::new(loc("en")).on_missing(Box::new(move |l, k| {
        sink.lock().unwrap().push(format!("{}:{k}", l.as_str()))
    }));
    t.add(
        loc("en"),
        Catalog::from_pairs([("hi", "Hello {name}"), ("only_en", "English only")]),
    );
    t.add(loc("fr"), Catalog::from_pairs([("hi", "Bonjour {name}")]));
    t.add(loc("fr-be"), Catalog::from_pairs([("hi", "Salut {name}")]));
    (t, missing)
}

#[test]
fn the_most_specific_locale_wins() {
    let (t, _) = translator();
    assert_eq!(t.t(&loc("fr-BE"), "hi", &[("name", "Ana")]), "Salut Ana");
    assert_eq!(t.t(&loc("fr-ca"), "hi", &[("name", "Ana")]), "Bonjour Ana");
}

#[test]
fn an_untranslated_key_falls_back_without_complaint() {
    let (t, missing) = translator();
    assert_eq!(t.t(&loc("fr"), "only_en", &[]), "English only");
    assert!(missing.lock().unwrap().is_empty());
}

#[test]
fn an_unknown_locale_uses_the_fallback_locale() {
    let (t, _) = translator();
    assert_eq!(t.t(&loc("ja"), "hi", &[("name", "Ana")]), "Hello Ana");
}

#[test]
fn a_missing_key_is_returned_visibly_and_reported() {
    let (t, missing) = translator();
    assert_eq!(t.t(&loc("fr-be"), "nope", &[]), "nope");
    assert_eq!(*missing.lock().unwrap(), ["fr-be:nope"]);
}

#[test]
fn adding_a_catalog_twice_overrides_key_by_key() {
    let (mut t, _) = translator();
    t.add(loc("fr"), Catalog::from_pairs([("hi", "Coucou {name}")]));
    assert_eq!(t.t(&loc("fr"), "hi", &[("name", "A")]), "Coucou A");
    assert_eq!(t.lookup(&loc("fr"), "only_en"), Some("English only"));
}

#[cfg(feature = "json")]
#[test]
fn json_catalogs_load_and_translate() {
    let mut t = Translator::new(loc("en"));
    t.add(
        loc("en"),
        Catalog::from_json(r#"{"menu":{"open":"Open {n}"}}"#).unwrap(),
    );
    assert_eq!(t.t(&loc("en"), "menu.open", &[("n", "2")]), "Open 2");
}
