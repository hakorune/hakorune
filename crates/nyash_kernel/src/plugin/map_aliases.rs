use super::map_probe::{map_probe_contains_any, map_probe_contains_i64};
use super::map_slot_load::{map_slot_load_any, map_slot_load_i64};
use super::map_slot_mutate::{map_slot_clear, map_slot_delete_any};
use super::map_slot_store::{map_slot_store_any, map_slot_store_i64_any};

// entry_count_i64: raw observer (handle) -> i64
#[export_name = "nyash.map.entry_count_i64"]
pub extern "C" fn nyash_map_entry_count_i64(handle: i64) -> i64 {
    super::map_substrate::map_entry_count_raw(handle)
}

#[export_name = "nyash.map.cap_h"]
pub extern "C" fn nyash_map_cap_h(handle: i64) -> i64 {
    super::map_substrate::map_capacity_raw(handle)
}

#[export_name = "nyash.map.keys_h"]
pub extern "C" fn nyash_map_keys_h(handle: i64) -> i64 {
    super::map_substrate::map_keys_handle(handle)
}

#[export_name = "nyash.map.clear_h"]
pub extern "C" fn nyash_map_clear_h(handle: i64) -> i64 {
    map_slot_clear(handle)
}

#[export_name = "nyash.map.delete_hh"]
pub extern "C" fn nyash_map_delete_hh_alias(handle: i64, key_any: i64) -> i64 {
    map_slot_delete_any(handle, key_any)
}

// Mainline substrate aliases used by collection-owner cutover and adapter defaults.
#[export_name = "nyash.map.slot_load_hi"]
pub extern "C" fn nyash_map_slot_load_hi_alias(handle: i64, key_i64: i64) -> i64 {
    map_slot_load_i64(handle, key_i64)
}

#[export_name = "nyash.map.slot_load_hh"]
pub extern "C" fn nyash_map_slot_load_hh_alias(handle: i64, key_any: i64) -> i64 {
    map_slot_load_any(handle, key_any)
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
