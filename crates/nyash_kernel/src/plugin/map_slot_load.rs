use super::handle_cache::with_map_box;
use super::value_codec::{any_arg_to_box, box_to_handle, int_arg_to_box};

#[inline(always)]
pub(super) fn map_slot_load_i64(handle: i64, key_i64: i64) -> i64 {
    with_map_box(handle, |map| {
        let key_str = int_arg_to_box(key_i64).to_string_box().value;
        let value = map.get_opt_key_str(&key_str)?;
        Some(box_to_handle(value))
    })
    .flatten()
    .unwrap_or(0)
}

#[inline(always)]
pub(super) fn map_slot_load_any(handle: i64, key_any: i64) -> i64 {
    with_map_box(handle, |map| {
        let key_str = any_arg_to_box(key_any).to_string_box().value;
        let value = map.get_opt_key_str(&key_str)?;
        Some(box_to_handle(value))
    })
    .flatten()
    .unwrap_or(0)
}
