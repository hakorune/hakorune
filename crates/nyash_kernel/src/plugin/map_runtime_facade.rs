use super::handle_cache::with_map_box;
use super::map_key_codec::map_key_string_from_any;
use super::map_probe::{map_probe_contains_any, map_probe_contains_i64};
use super::map_slot_load::{map_slot_load_any, map_slot_load_i64};
use super::map_slot_store::{map_slot_store_any, map_slot_store_i64_any};

// Runtime/compat forwarding only.
// Map semantic ownership lives in `.hako` (`MapCoreBox` / `MapStateCoreBox`);
// keep this module below the owner and above raw slot/probe/store leaves.

pub(super) fn map_runtime_clear(handle: i64) -> i64 {
    let _ = with_map_box(handle, |map| {
        map.clear_entries();
    });
    0
}

pub(super) fn map_runtime_delete_any(handle: i64, key_any: i64) -> i64 {
    let key_str = map_key_string_from_any(key_any);
    with_map_box(
        handle,
        |map| {
            if map.remove_key_str(&key_str) {
                1
            } else {
                0
            }
        },
    )
    .unwrap_or(0)
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
