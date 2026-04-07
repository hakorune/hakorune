use super::handle_cache::{clear_map_lookup_cache, with_map_box};
use super::map_key_codec::map_key_string_from_any;
use super::map_probe::{map_probe_contains_any, map_probe_contains_i64};
use super::map_slot_load::{map_slot_load_any, map_slot_load_i64};
use super::map_slot_store::{map_slot_store_any, map_slot_store_i64_any};
use super::value_codec::{runtime_i64_from_box_ref_caller, BorrowedAliasEncodeCaller};
use nyash_rust::{boxes::map_box::MapBox, runtime::host_handles as handles};

// Runtime/compat forwarding only.
// Map semantic ownership lives in `.hako` (`MapCoreBox` / `MapStateCoreBox`);
// keep this module below the owner and above raw slot/probe/store leaves.

// Observer facade.
pub(super) fn map_runtime_entry_count(handle: i64) -> i64 {
    with_map_box(handle, |map| map.entry_count_i64()).unwrap_or(0)
}

pub(super) fn map_runtime_cap(handle: i64) -> i64 {
    with_map_box(handle, |map| map.capacity_i64()).unwrap_or(0)
}

pub(super) fn map_runtime_clear(handle: i64) -> i64 {
    if with_map_box(handle, |map| {
        map.get_data().write().unwrap().clear();
    })
    .is_some()
    {
        clear_map_lookup_cache();
    }
    0
}

pub(super) fn map_runtime_delete_any(handle: i64, key_any: i64) -> i64 {
    clear_map_lookup_cache();
    let key_str = map_key_string_from_any(key_any);
    with_map_box(handle, |map| {
        let removed = map.get_data().write().unwrap().remove(&key_str);
        if removed.is_some() {
            1
        } else {
            0
        }
    })
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

// RuntimeData-facing facade. This is compat/runtime forwarding, not owner logic.
#[inline(never)]
fn with_runtime_map_box<R>(handle: i64, f: impl FnOnce(&MapBox) -> R) -> Option<R> {
    if handle <= 0 {
        return None;
    }
    let obj = handles::get(handle as u64)?;
    let map = obj.as_any().downcast_ref::<MapBox>()?;
    Some(f(map))
}

#[inline(never)]
pub(super) fn map_runtime_data_get_any_key(handle: i64, key_any: i64) -> i64 {
    let key_str = if key_any <= 0 {
        key_any.to_string()
    } else {
        map_key_string_from_any(key_any)
    };
    with_runtime_map_box(handle, |map| {
        map.get_opt_key_str(&key_str)
            .as_ref()
            .map(|value| {
                runtime_i64_from_box_ref_caller(
                    value.as_ref(),
                    BorrowedAliasEncodeCaller::MapRuntimeDataGetAnyKey,
                )
            })
            .unwrap_or(0)
    })
    .unwrap_or(0)
}

#[inline(never)]
pub(super) fn map_runtime_data_set_any_key(handle: i64, key_any: i64, val_any: i64) -> i64 {
    let key_str = if key_any <= 0 {
        key_any.to_string()
    } else {
        map_key_string_from_any(key_any)
    };
    with_runtime_map_box(handle, |map| {
        let value_box = super::value_codec::any_arg_to_box(val_any);
        map.insert_key_str(key_str, value_box);
        1
    })
    .unwrap_or(0)
}

#[inline(never)]
pub(super) fn map_runtime_data_has_any_key(handle: i64, key_any: i64) -> i64 {
    let key_str = if key_any <= 0 {
        key_any.to_string()
    } else {
        map_key_string_from_any(key_any)
    };
    with_runtime_map_box(
        handle,
        |map| {
            if map.contains_key_str(&key_str) {
                1
            } else {
                0
            }
        },
    )
    .unwrap_or(0)
}
