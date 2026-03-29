use super::handle_cache::with_map_box;
use super::map_probe::map_probe_contains_any;
use super::map_slot_store::map_slot_store_any;
use super::value_codec::{any_arg_to_box, runtime_i64_from_box_ref};

#[inline(always)]
pub(super) fn runtime_data_map_get_hh(handle: i64, key_any: i64) -> i64 {
    with_map_box(handle, |map| {
        let key_str = any_arg_to_box(key_any).to_string_box().value;
        let value = map.get_opt_key_str(&key_str)?;
        Some(runtime_i64_from_box_ref(value.as_ref()))
    })
    .flatten()
    .unwrap_or(0)
}

#[inline(always)]
pub(super) fn runtime_data_map_set_hhh(handle: i64, key_any: i64, val_any: i64) -> i64 {
    map_slot_store_any(handle, key_any, val_any)
}

#[inline(always)]
pub(super) fn runtime_data_map_has_hh(handle: i64, key_any: i64) -> i64 {
    map_probe_contains_any(handle, key_any)
}
