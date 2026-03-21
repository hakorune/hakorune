use super::handle_helpers::with_map_box;
use super::value_codec::{any_arg_to_box, box_to_handle, int_arg_to_box};

#[inline(always)]
pub(super) fn map_slot_load_i64(handle: i64, key_i64: i64) -> i64 {
    with_map_box(handle, |map| {
        let value = map.get_opt(int_arg_to_box(key_i64))?;
        Some(box_to_handle(value))
    })
    .flatten()
    .unwrap_or(0)
}

#[inline(always)]
pub(super) fn map_slot_load_any(handle: i64, key_any: i64) -> i64 {
    with_map_box(handle, |map| {
        let key_box = any_arg_to_box(key_any);
        let value = map.get_opt(key_box)?;
        Some(box_to_handle(value))
    })
    .flatten()
    .unwrap_or(0)
}
