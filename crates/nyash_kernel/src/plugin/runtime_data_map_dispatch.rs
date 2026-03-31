use super::handle_cache::{map_lookup_cache_hit, with_map_lookup_cached};
use super::map_key_codec::map_key_string_from_any;
use super::map_probe::map_probe_contains_str;
use super::map_slot_store::map_slot_store_any;
use super::value_codec::runtime_i64_from_box_ref;

#[inline(always)]
pub(super) fn runtime_data_map_get_hh(handle: i64, key_any: i64) -> i64 {
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

#[inline(always)]
pub(super) fn runtime_data_map_set_hhh(handle: i64, key_any: i64, val_any: i64) -> i64 {
    map_slot_store_any(handle, key_any, val_any)
}

#[inline(always)]
pub(super) fn runtime_data_map_has_hh(handle: i64, key_any: i64) -> i64 {
    let key_str = map_key_string_from_any(key_any);
    if let Some((_, present)) = map_lookup_cache_hit(handle, &key_str) {
        return if present { 1 } else { 0 };
    }
    map_probe_contains_str(handle, &key_str)
}
