use super::map_probe::map_probe_contains_any;
use super::map_slot_load::map_slot_load_any;
use super::map_slot_store::map_slot_store_any;

#[inline(always)]
pub(super) fn runtime_data_map_get_hh(handle: i64, key_any: i64) -> i64 {
    map_slot_load_any(handle, key_any)
}

#[inline(always)]
pub(super) fn runtime_data_map_set_hhh(handle: i64, key_any: i64, val_any: i64) -> i64 {
    map_slot_store_any(handle, key_any, val_any)
}

#[inline(always)]
pub(super) fn runtime_data_map_has_hh(handle: i64, key_any: i64) -> i64 {
    map_probe_contains_any(handle, key_any)
}
