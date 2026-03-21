use super::handle_helpers::with_map_box;
use super::value_codec::{any_arg_to_box, int_arg_to_box};

#[inline(always)]
pub(super) fn map_slot_store_i64_any(handle: i64, key_i64: i64, val_any: i64) -> i64 {
    with_map_box(handle, |map| {
        let key_box = int_arg_to_box(key_i64);
        let value_box = any_arg_to_box(val_any);
        let _ = map.set(key_box, value_box);
        1
    })
    .unwrap_or(0)
}

#[inline(always)]
pub(super) fn map_slot_store_any(handle: i64, key_any: i64, val_any: i64) -> i64 {
    with_map_box(handle, |map| {
        let key_box = any_arg_to_box(key_any);
        let value_box = any_arg_to_box(val_any);
        let _ = map.set(key_box, value_box);
        1
    })
    .unwrap_or(0)
}
