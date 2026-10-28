//! Key -> string table of one locale.

use std::collections::HashMap;

/// Why a catalog could not be loaded. Never swallowed: callers decide what to do.
#[derive(Debug, PartialEq, Eq)]
pub enum CatalogError {
    /// The text is not valid JSON.
    Parse(String),
    /// The JSON is valid but a value is neither a string nor an object of strings.
    NotAString { key: String },
}

impl std::fmt::Display for CatalogError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parse(e) => write!(f, "catalog is not valid JSON: {e}"),
            Self::NotAString { key } => write!(f, "catalog value at `{key}` is not a string"),
        }
    }
}

impl std::error::Error for CatalogError {}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Catalog(pub(crate) HashMap<String, String>);

impl Catalog {
    pub fn from_pairs<'a>(pairs: impl IntoIterator<Item = (&'a str, &'a str)>) -> Self {
        Self(
            pairs
                .into_iter()
                .map(|(k, v)| (k.to_owned(), v.to_owned()))
                .collect(),
        )
    }

    #[must_use]
    pub fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).map(String::as_str)
    }

    /// Key-by-key override: `other` wins where it has the key, everything else stays.
    pub fn merge(&mut self, other: Catalog) {
        self.0.extend(other.0);
    }

    /// Parse a JSON object; nested objects become dotted keys (`{"a":{"b":"x"}}` -> `a.b`).
    #[cfg(feature = "json")]
    pub fn from_json(text: &str) -> Result<Self, CatalogError> {
        let value: serde_json::Value =
            serde_json::from_str(text).map_err(|e| CatalogError::Parse(e.to_string()))?;
        let mut map = HashMap::new();
        flatten("", &value, &mut map)?;
        Ok(Self(map))
    }
}

#[cfg(feature = "json")]
fn flatten(
    prefix: &str,
    value: &serde_json::Value,
    out: &mut HashMap<String, String>,
) -> Result<(), CatalogError> {
    match value {
        serde_json::Value::String(s) if !prefix.is_empty() => {
            out.insert(prefix.to_owned(), s.clone());
            Ok(())
        }
        serde_json::Value::Object(map) => {
            for (k, v) in map {
                let key = if prefix.is_empty() {
                    k.clone()
                } else {
                    format!("{prefix}.{k}")
                };
                flatten(&key, v, out)?;
            }
            Ok(())
        }
        _ => Err(CatalogError::NotAString {
            key: prefix.to_owned(),
        }),
    }
}

#[cfg(all(test, feature = "json"))]
mod tests {
    use super::*;

    #[test]
    fn flat_and_nested_json_flatten_to_dotted_keys() {
        let c = Catalog::from_json(r#"{"ok":"OK","menu":{"file":{"open":"Open"}}}"#).unwrap();
        assert_eq!(
            (c.get("ok"), c.get("menu.file.open")),
            (Some("OK"), Some("Open"))
        );
    }

    #[test]
    fn invalid_json_is_an_error_not_an_empty_catalog() {
        assert!(matches!(
            Catalog::from_json("{oops"),
            Err(CatalogError::Parse(_))
        ));
    }

    #[test]
    fn a_non_string_value_names_its_key() {
        let e = Catalog::from_json(r#"{"a":{"n":3}}"#).unwrap_err();
        assert_eq!(e, CatalogError::NotAString { key: "a.n".into() });
        assert!(Catalog::from_json("3").is_err() && Catalog::from_json(r#"["x"]"#).is_err());
    }

    #[test]
    fn merge_overrides_key_by_key_and_keeps_the_rest() {
        let mut base = Catalog::from_pairs([("a", "1"), ("b", "2")]);
        base.merge(Catalog::from_pairs([("b", "9"), ("c", "3")]));
        assert_eq!(
            (base.get("a"), base.get("b"), base.get("c")),
            (Some("1"), Some("9"), Some("3"))
        );
    }
}
