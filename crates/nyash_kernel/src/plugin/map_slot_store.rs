use super::handle_cache::with_map_box;
use super::value_codec::{any_arg_to_box, int_arg_to_box};

#[inline(always)]
pub(super) fn map_slot_store_i64_any(handle: i64, key_i64: i64, val_any: i64) -> i64 {
    with_map_box(handle, |map| {
        let key_str = int_arg_to_box(key_i64).to_string_box().value;
        let value_box = any_arg_to_box(val_any);
        map.insert_key_str(key_str, value_box);
        1
    })
    .unwrap_or(0)
}

#[inline(always)]
pub(super) fn map_slot_store_any(handle: i64, key_any: i64, val_any: i64) -> i64 {
    with_map_box(handle, |map| {
        let key_str = any_arg_to_box(key_any).to_string_box().value;
        let value_box = any_arg_to_box(val_any);
        map.insert_key_str(key_str, value_box);
        1
    })
    .unwrap_or(0)
}
