//! Shared helpers for diagnostic-only allocator provider registry reports.
//!
//! These helpers keep the reserved provider-id set and TOML list readers in one
//! place while the report implementations live in narrower modules.

pub(crate) const OWNER_PATH: &str = "src/runtime/allocator_provider_registry.rs";

pub(crate) const EXPECTED_PROVIDER_IDS: &[&str] = &[
    "native_system_malloc",
    "native_mimalloc",
    "hako_model_allocator",
    "debug_guarded_allocator",
];

pub(crate) fn string_list_matches(value: Option<&toml::Value>, expected: &[&str]) -> bool {
    let Some(items) = value.and_then(toml::Value::as_array) else {
        return false;
    };
    let actual: Vec<&str> = items.iter().filter_map(toml::Value::as_str).collect();
    actual == expected
}

pub(crate) fn string_list(value: Option<&toml::Value>) -> Vec<&str> {
    value
        .and_then(toml::Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(toml::Value::as_str)
        .filter(|item| !item.is_empty())
        .collect()
}
