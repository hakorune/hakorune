#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MapKeyDomain {
    CanonicalI64(i64),
    Text(String),
}

impl MapKeyDomain {
    pub fn from_i64(value: i64) -> Self {
        Self::CanonicalI64(value)
    }

    pub fn from_text(text: &str) -> Self {
        match parse_canonical_i64_text(text) {
            Some(value) => Self::CanonicalI64(value),
            None => Self::Text(text.to_string()),
        }
    }

    /// Checked native preparation; completes allocation before child evaluation.
    pub fn try_from_text(text: &str) -> Result<Self, std::collections::TryReserveError> {
        if let Some(value) = parse_canonical_i64_text(text) {
            return Ok(Self::CanonicalI64(value));
        }
        let mut owned = String::new();
        owned.try_reserve_exact(text.len())?;
        owned.push_str(text);
        Ok(Self::Text(owned))
    }

    pub fn public_text(&self) -> String {
        match self {
            Self::CanonicalI64(value) => value.to_string(),
            Self::Text(value) => value.clone(),
        }
    }
}

fn parse_canonical_i64_text(text: &str) -> Option<i64> {
    if text == "0" { return Some(0); }
    let digits = text.strip_prefix('-').unwrap_or(text).as_bytes();
    if !matches!(digits.first(), Some(b'1'..=b'9'))
        || !digits.iter().all(u8::is_ascii_digit) { return None; }
    text.parse::<i64>().ok()
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
