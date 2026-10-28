//! `{name}` placeholders. `{{` and `}}` are literal braces. A placeholder with no value
//! is kept as written, so a missing argument is visible instead of silently blank.

/// Fill `template` from `args` (name, value).
#[must_use]
pub fn fill(template: &str, args: &[(&str, &str)]) -> String {
    let mut out = String::with_capacity(template.len());
    let mut rest = template;
    while let Some(i) = rest.find(['{', '}']) {
        out.push_str(&rest[..i]);
        rest = &rest[i..];
        if rest.starts_with("{{") || rest.starts_with("}}") {
            out.push_str(&rest[..1]);
            rest = &rest[2..];
        } else if let Some(end) = rest.starts_with('{').then(|| rest.find('}')).flatten() {
            let name = &rest[1..end];
            match args.iter().find(|(k, _)| *k == name) {
                Some((_, v)) => out.push_str(v),
                None => out.push_str(&rest[..=end]),
            }
            rest = &rest[end + 1..];
        } else {
            out.push_str(&rest[..1]);
            rest = &rest[1..];
        }
    }
    out.push_str(rest);
    out
}

#[cfg(test)]
mod tests {
    use super::fill;

    #[test]
    fn replaces_named_placeholders() {
        assert_eq!(
            fill("Hi {name}, {n} new", &[("name", "Ana"), ("n", "3")]),
            "Hi Ana, 3 new"
        );
    }

    #[test]
    fn a_missing_argument_stays_visible() {
        assert_eq!(fill("Hi {name}", &[]), "Hi {name}");
    }

    #[test]
    fn doubled_braces_are_literal() {
        assert_eq!(fill("{{x}} {a}", &[("a", "1")]), "{x} 1");
    }

    #[test]
    fn stray_and_unterminated_braces_pass_through() {
        assert_eq!(fill("a } b { c", &[]), "a } b { c");
        assert_eq!(fill("{open", &[]), "{open");
    }

    #[test]
    fn values_are_not_reinterpreted() {
        assert_eq!(fill("{a}", &[("a", "{b}")]), "{b}");
    }

    #[test]
    fn handles_multibyte_text() {
        assert_eq!(fill("é{a}ü", &[("a", "→")]), "é→ü");
    }
}
