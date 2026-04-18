use super::map_probe::{map_probe_contains_any, map_probe_contains_i64};
use super::map_slot_load::{map_slot_load_any, map_slot_load_i64};
use super::map_slot_mutate::{map_slot_clear, map_slot_delete_any};
use super::map_slot_store::{map_slot_store_any, map_slot_store_i64_any};

// Runtime/compat forwarding only.
// Map semantic ownership lives in `.hako` (`MapCoreBox` / `MapStateCoreBox`);
// keep this module below the owner and above raw slot/probe/store leaves.

pub(super) fn map_runtime_clear(handle: i64) -> i64 {
    map_slot_clear(handle)
}

pub(super) fn map_runtime_delete_any(handle: i64, key_any: i64) -> i64 {
    map_slot_delete_any(handle, key_any)
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
