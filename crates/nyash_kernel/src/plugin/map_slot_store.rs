use super::handle_cache::with_map_box;
use super::map_key_codec::{map_key_string_from_any, map_key_string_from_i64};
use super::value_codec::{any_arg_to_box_with_profile, CodecProfile};
use super::value_demand::MAP_VALUE_STORE_ANY;

#[inline(always)]
pub(super) fn map_slot_store_i64_any(handle: i64, key_i64: i64, val_any: i64) -> i64 {
    let key_str = map_key_string_from_i64(key_i64);
    map_slot_store_str_any(handle, key_str, val_any)
}

#[inline(always)]
pub(super) fn map_slot_store_any(handle: i64, key_any: i64, val_any: i64) -> i64 {
    let key_str = map_key_string_from_any(key_any);
    map_slot_store_str_any(handle, key_str, val_any)
}

#[inline(always)]
pub(super) fn map_slot_store_str_any(handle: i64, key_str: String, val_any: i64) -> i64 {
    let _value_demand = MAP_VALUE_STORE_ANY;
    with_map_box(handle, |map| {
        let value_box = any_arg_to_box_with_profile(val_any, CodecProfile::MapValueBorrowString);
        map.insert_key_str(key_str, value_box);
        1
    })
    .unwrap_or(0)
}
