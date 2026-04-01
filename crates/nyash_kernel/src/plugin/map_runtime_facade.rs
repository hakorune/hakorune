use super::handle_cache::{map_lookup_cache_hit, with_map_box, with_map_lookup_cached};
use super::map_key_codec::map_key_string_from_any;
use super::map_probe::{map_probe_contains_any, map_probe_contains_i64, map_probe_contains_str};
use super::map_slot_load::{map_slot_load_any, map_slot_load_i64};
use super::map_slot_store::{map_slot_store_any, map_slot_store_i64_any};
use super::value_codec::runtime_i64_from_box_ref;

// Shared RawMap substrate facade.
// This stays below MapCoreBox and above the map slot/probe/store leaves.

// Observer facade.
pub(super) fn map_runtime_entry_count(handle: i64) -> i64 {
    with_map_box(handle, |map| map.entry_count_i64()).unwrap_or(0)
}

pub(super) fn map_runtime_cap(handle: i64) -> i64 {
    with_map_box(handle, |map| map.capacity_i64()).unwrap_or(0)
}

// Probe/load/store substrate facade.
pub(super) fn map_runtime_probe_i64(handle: i64, key_i64: i64) -> i64 {
    map_probe_contains_i64(handle, key_i64)
}

pub(super) fn map_runtime_probe_any(handle: i64, key_any: i64) -> i64 {
    map_probe_contains_any(handle, key_any)
}

pub(super) fn map_runtime_load_i64(handle: i64, key_i64: i64) -> i64 {
    map_slot_load_i64(handle, key_i64)
}

pub(super) fn map_runtime_load_any(handle: i64, key_any: i64) -> i64 {
    map_slot_load_any(handle, key_any)
}

pub(super) fn map_runtime_store_i64_any(handle: i64, key_i64: i64, val_any: i64) -> i64 {
    map_slot_store_i64_any(handle, key_i64, val_any)
}

pub(super) fn map_runtime_store_any(handle: i64, key_any: i64, val_any: i64) -> i64 {
    map_slot_store_any(handle, key_any, val_any)
}

// RuntimeData-facing facade.
pub(super) fn map_runtime_data_get_any_key(handle: i64, key_any: i64) -> i64 {
    let key_str = map_key_string_from_any(key_any);
    with_map_lookup_cached(handle, &key_str, |map| {
        let value = map.get_opt_key_str(&key_str);
        let present = value.is_some();
        let out = value
            .as_ref()
            .map(|value| runtime_i64_from_box_ref(value.as_ref()))
            .unwrap_or(0);
        (out, present)
    })
    .map(|(value, present)| if present { value } else { 0 })
    .unwrap_or(0)
}

pub(super) fn map_runtime_data_set_any_key(handle: i64, key_any: i64, val_any: i64) -> i64 {
    map_runtime_store_any(handle, key_any, val_any)
}

pub(super) fn map_runtime_data_has_any_key(handle: i64, key_any: i64) -> i64 {
    let key_str = map_key_string_from_any(key_any);
    if let Some((_, present)) = map_lookup_cache_hit(handle, &key_str) {
        return if present { 1 } else { 0 };
    }
    map_probe_contains_str(handle, &key_str)
}
