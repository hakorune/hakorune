//! Shared helpers for allocator provider diagnostic TOML readers.
//!
//! This module owns small parsing predicates used by diagnostic-only allocator
//! provider reports. It does not select providers, consume proofs, prepare
//! rollback, open activation gates, install hooks, or replace the process
//! allocator.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct DiagnosticFactCheck {
    pub present: bool,
    pub name: &'static str,
}

pub(crate) fn text_field_matches(value: &toml::Value, key: &str, expected: &str) -> bool {
    value.get(key).and_then(toml::Value::as_str) == Some(expected)
}

pub(crate) fn bool_field_false(value: &toml::Value, key: &str) -> bool {
    value.get(key).and_then(toml::Value::as_bool) == Some(false)
}

pub(crate) fn nonempty_text_field<'a>(value: &'a toml::Value, key: &str) -> Option<&'a str> {
    let text = value.get(key)?.as_str()?;
    if text.is_empty() {
        None
    } else {
        Some(text)
    }
}

pub(crate) fn string_list_contains_all(value: Option<&toml::Value>, required: &[&str]) -> bool {
    let Some(items) = value.and_then(toml::Value::as_array) else {
        return false;
    };
    required.iter().all(|required| {
        items
            .iter()
            .filter_map(toml::Value::as_str)
            .any(|item| item == *required)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_value(text: &str) -> toml::Value {
        toml::from_str(text).expect("test TOML should parse")
    }

    #[test]
    fn allocator_provider_toml_helpers_match_text_and_false_bool_fields() {
        let value = parse_value(
            r#"
            status = "reserved"
            active = false
            "#,
        );

        assert!(text_field_matches(&value, "status", "reserved"));
        assert!(!text_field_matches(&value, "status", "active"));
        assert!(bool_field_false(&value, "active"));
        assert!(!bool_field_false(&value, "missing"));
    }

    #[test]
    fn allocator_provider_toml_helpers_require_nonempty_text() {
        let value = parse_value(
            r#"
            owner = "src/runtime/example.rs"
            empty = ""
            "#,
        );

        assert_eq!(
            nonempty_text_field(&value, "owner"),
            Some("src/runtime/example.rs")
        );
        assert_eq!(nonempty_text_field(&value, "empty"), None);
        assert_eq!(nonempty_text_field(&value, "missing"), None);
    }

    #[test]
    fn allocator_provider_toml_helpers_match_required_string_list_members() {
        let value = parse_value(
            r#"
            required = ["a", "b", "c"]
            "#,
        );

        assert!(string_list_contains_all(value.get("required"), &["a", "c"]));
        assert!(!string_list_contains_all(
            value.get("required"),
            &["a", "missing"]
        ));
        assert!(!string_list_contains_all(value.get("missing"), &["a"]));
    }
}
