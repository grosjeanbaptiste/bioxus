//! Open-ended i18n with no UI dependency: locales are tags, catalogs are key tables,
//! lookup follows `fr-be` -> `fr` -> fallback, and remote JSON overrides merge key by key.

pub mod catalog;
pub mod interpolate;
pub mod locale;
pub mod translator;

pub use catalog::{Catalog, CatalogError};
pub use locale::Locale;
pub use translator::Translator;

#[cfg(test)]
mod tests;
