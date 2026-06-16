use super::handle_cache::with_map_box;
use super::map_key_codec::{map_key_string_from_any, map_key_string_from_i64};
use super::map_probe::{map_probe_contains_any, map_probe_contains_i64};
use super::map_slot_load::{map_scalar_load_i64, map_slot_load_str};
use super::map_slot_mutate::{map_slot_clear, map_slot_delete_any};
use super::map_slot_store::{map_slot_store_any, map_slot_store_i64_any};
pub(super) fn map_entry_count_raw(handle: i64) -> i64 {
    static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    let enabled = crate::env_flags::flag_on_cached(&ENABLED, "NYASH_LLVM_MAP_DEBUG");
    if enabled {
        eprintln!("[MAP] entry_count_i64(handle={})", handle);
    }
    let size = with_map_box(handle, |map| map.entry_count_i64()).unwrap_or(0);
    if enabled {
        eprintln!("[MAP] entry_count_i64 => {}", size);
    }
    size
}

// entry_count_i64: raw observer (handle) -> i64
#[export_name = "nyash.map.entry_count_i64"]
pub extern "C" fn nyash_map_entry_count_i64(handle: i64) -> i64 {
    map_entry_count_raw(handle)
}

#[export_name = "nyash.map.cap_h"]
pub extern "C" fn nyash_map_cap_h(handle: i64) -> i64 {
    with_map_box(handle, |map| map.capacity_i64()).unwrap_or(0)
}

#[export_name = "nyash.map.keys_h"]
pub extern "C" fn nyash_map_keys_h(handle: i64) -> i64 {
    with_map_box(handle, |map| {
        let keys = map.keys();
        let keys: std::sync::Arc<dyn nyash_rust::box_trait::NyashBox> = std::sync::Arc::from(keys);
        nyash_rust::runtime::host_handles::to_handle_arc(keys) as i64
    })
    .unwrap_or(0)
}

#[export_name = "nyash.map.clear_h"]
pub extern "C" fn nyash_map_clear_h(handle: i64) -> i64 {
    map_slot_clear(handle)
}

#[export_name = "nyash.map.delete_hh"]
pub extern "C" fn nyash_map_delete_hh_alias(handle: i64, key_any: i64) -> i64 {
    map_slot_delete_any(handle, key_any)
}

// Mainline map aliases used by collection-owner routing and adapter defaults.
#[export_name = "nyash.map.slot_load_hi"]
pub extern "C" fn nyash_map_slot_load_hi_alias(handle: i64, key_i64: i64) -> i64 {
    let key_str = map_key_string_from_i64(key_i64);
    map_slot_load_str(handle, &key_str)
}

#[export_name = "nyash.map.slot_load_hh"]
pub extern "C" fn nyash_map_slot_load_hh_alias(handle: i64, key_any: i64) -> i64 {
    let key_str = map_key_string_from_any(key_any);
    map_slot_load_str(handle, &key_str)
}

#[export_name = "nyash.map.scalar_load_hi"]
pub extern "C" fn nyash_map_scalar_load_hi_alias(handle: i64, key_i64: i64) -> i64 {
    map_scalar_load_i64(handle, key_i64)
}

#[export_name = "nyash.map.local_i64_get_hi"]
pub extern "C" fn nyash_map_local_i64_get_hi_alias(handle: i64, key_i64: i64) -> i64 {
    map_scalar_load_i64(handle, key_i64)
}

#[export_name = "nyash.map.slot_store_hih"]
pub extern "C" fn nyash_map_slot_store_hih_alias(handle: i64, key_i64: i64, val_any: i64) -> i64 {
    map_slot_store_i64_any(handle, key_i64, val_any)
}

#[export_name = "nyash.map.slot_store_hhh"]
pub extern "C" fn nyash_map_slot_store_hhh_alias(handle: i64, key_any: i64, val_any: i64) -> i64 {
    map_slot_store_any(handle, key_any, val_any)
}

#[export_name = "nyash.map.probe_hi"]
pub extern "C" fn nyash_map_probe_hi_alias(handle: i64, key_i64: i64) -> i64 {
    map_probe_contains_i64(handle, key_i64)
}

#[export_name = "nyash.map.probe_hh"]
pub extern "C" fn nyash_map_probe_hh_alias(handle: i64, key_any: i64) -> i64 {
    map_probe_contains_any(handle, key_any)
}
