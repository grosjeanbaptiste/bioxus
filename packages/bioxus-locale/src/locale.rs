//! A locale is an open tag (`fr`, `fr-BE`, `pt-BR`, ...), not a closed enum: an app adds
//! a language by shipping a catalog, with no change here.

/// A language tag, compared case-insensitively and stored lowercase with `-` separators.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Locale(String);

impl Locale {
    /// Normalise `tag` (`fr_BE`, `FR-be` -> `fr-be`). An empty or blank tag is rejected.
    pub fn new(tag: &str) -> Option<Self> {
        let tag = tag.trim().replace('_', "-").to_lowercase();
        (!tag.is_empty()).then_some(Self(tag))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// From most to least specific: `fr-be-x` -> `fr-be-x`, `fr-be`, `fr`.
    #[must_use]
    pub fn chain(&self) -> Vec<Locale> {
        let mut out = vec![self.clone()];
        let mut tag = self.0.as_str();
        while let Some((parent, _)) = tag.rsplit_once('-') {
            out.push(Self(parent.to_owned()));
            tag = parent;
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tags_are_normalised() {
        assert_eq!(Locale::new(" FR_be ").unwrap().as_str(), "fr-be");
    }

    #[test]
    fn blank_tags_are_rejected() {
        assert!(Locale::new("").is_none() && Locale::new("  ").is_none());
    }

    #[test]
    fn chain_goes_from_specific_to_general() {
        let chain: Vec<_> = Locale::new("zh-hant-tw")
            .unwrap()
            .chain()
            .iter()
            .map(|l| l.as_str().to_owned())
            .collect();
        assert_eq!(chain, ["zh-hant-tw", "zh-hant", "zh"]);
    }

    #[test]
    fn a_bare_language_has_a_chain_of_one() {
        assert_eq!(Locale::new("fr").unwrap().chain().len(), 1);
    }
}
