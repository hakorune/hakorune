use super::handle_cache::{clear_map_lookup_cache, with_map_box};
use super::map_key_codec::{map_key_string_from_any, map_key_string_from_i64};
use super::value_codec::any_arg_to_box;

#[inline(always)]
pub(super) fn map_slot_store_i64_any(handle: i64, key_i64: i64, val_any: i64) -> i64 {
    clear_map_lookup_cache();
    with_map_box(handle, |map| {
        let key_str = map_key_string_from_i64(key_i64);
        let value_box = any_arg_to_box(val_any);
        map.insert_key_str(key_str, value_box);
        1
    })
    .unwrap_or(0)
}

#[inline(always)]
pub(super) fn map_slot_store_any(handle: i64, key_any: i64, val_any: i64) -> i64 {
    clear_map_lookup_cache();
    let key_str = map_key_string_from_any(key_any);
    with_map_box(handle, |map| {
        let value_box = any_arg_to_box(val_any);
        map.insert_key_str(key_str, value_box);
        1
    })
    .unwrap_or(0)
}
