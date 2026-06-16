// Prototype vocabulary kept for kernel-side route planning notes. The storage
// truth currently lives in `nyash-rust::boxes::map_key_domain`.
#![allow(dead_code)]

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum MapKeyDomain {
    CanonicalI64(i64),
    Text(String),
}

impl MapKeyDomain {
    pub(crate) fn from_i64(value: i64) -> Self {
        Self::CanonicalI64(value)
    }

    pub(crate) fn from_text(text: &str) -> Self {
        match parse_canonical_i64_text(text) {
            Some(value) => Self::CanonicalI64(value),
            None => Self::Text(text.to_string()),
        }
    }

    pub(crate) fn public_text(&self) -> String {
        match self {
            Self::CanonicalI64(value) => value.to_string(),
            Self::Text(value) => value.clone(),
        }
    }
}

fn parse_canonical_i64_text(text: &str) -> Option<i64> {
    let value = text.parse::<i64>().ok()?;
    (value.to_string() == text).then_some(value)
}

#[cfg(test)]
mod tests {
    use super::MapKeyDomain;

    #[test]
    fn i64_and_canonical_text_share_domain() {
        for value in [0, 1, -1, i64::MIN, i64::MAX] {
            let from_i64 = MapKeyDomain::from_i64(value);
            let text = value.to_string();
            let from_text = MapKeyDomain::from_text(&text);
            assert_eq!(from_i64, from_text);
            assert_eq!(from_i64.public_text(), text);
        }
    }

    #[test]
    fn noncanonical_numeric_text_stays_text() {
        for text in ["", "+1", "01", "-0", " 1", "1 ", "9223372036854775808"] {
            let key = MapKeyDomain::from_text(text);
            assert_eq!(key, MapKeyDomain::Text(text.to_string()));
            assert_eq!(key.public_text(), text);
        }
    }

    #[test]
    fn canonical_text_keeps_expected_alias_examples() {
        assert_eq!(MapKeyDomain::from_i64(1), MapKeyDomain::from_text("1"));
        assert_ne!(MapKeyDomain::from_i64(1), MapKeyDomain::from_text("01"));
        assert_ne!(MapKeyDomain::from_i64(0), MapKeyDomain::from_text("-0"));
    }
}
