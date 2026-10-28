//! Looks a key up through the locale chain, then the fallback locale.

use crate::catalog::Catalog;
use crate::interpolate::fill;
use crate::locale::Locale;
use std::collections::HashMap;

/// Called with (locale, key) each time a key is found nowhere. The default writes a
/// warning on stderr; an app can route it to its own logger or fail tests on it.
pub type MissingHandler = Box<dyn Fn(&Locale, &str) + Send + Sync>;

pub struct Translator {
    catalogs: HashMap<Locale, Catalog>,
    fallback: Locale,
    on_missing: MissingHandler,
}

impl Translator {
    #[must_use]
    pub fn new(fallback: Locale) -> Self {
        let on_missing: MissingHandler = Box::new(|l, k| {
            eprintln!(
                "warn: bioxus-locale: no `{k}` in `{}` or its fallbacks",
                l.as_str()
            )
        });
        Self {
            catalogs: HashMap::new(),
            fallback,
            on_missing,
        }
    }

    #[must_use]
    pub fn on_missing(mut self, handler: MissingHandler) -> Self {
        self.on_missing = handler;
        self
    }

    /// Add a catalog, or override the existing one key by key.
    pub fn add(&mut self, locale: Locale, catalog: Catalog) {
        self.catalogs.entry(locale).or_default().merge(catalog);
    }

    /// The raw string for `key`, or `None`. Does not call the missing handler.
    #[must_use]
    pub fn lookup(&self, locale: &Locale, key: &str) -> Option<&str> {
        locale
            .chain()
            .into_iter()
            .chain(self.fallback.chain())
            .find_map(|l| self.catalogs.get(&l)?.get(key))
    }

    /// Translate `key` with `{name}` arguments. A missing key returns the key itself,
    /// visibly, after calling the missing handler.
    #[must_use]
    pub fn t(&self, locale: &Locale, key: &str, args: &[(&str, &str)]) -> String {
        match self.lookup(locale, key) {
            Some(template) => fill(template, args),
            None => {
                (self.on_missing)(locale, key);
                key.to_owned()
            }
        }
    }
}
