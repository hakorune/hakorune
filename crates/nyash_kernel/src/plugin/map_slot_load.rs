use super::handle_cache::with_map_box;
use super::map_key_codec::{map_key_string_from_any, map_key_string_from_i64};
use super::value_codec::box_to_handle;

#[inline(always)]
pub(super) fn map_slot_load_i64(handle: i64, key_i64: i64) -> i64 {
    with_map_box(handle, |map| {
        let key_str = map_key_string_from_i64(key_i64);
        let value = map.get_opt_key_str(&key_str)?;
        Some(box_to_handle(value))
    })
    .flatten()
    .unwrap_or(0)
}

#[inline(always)]
pub(super) fn map_slot_load_any(handle: i64, key_any: i64) -> i64 {
    let key_str = map_key_string_from_any(key_any);
    with_map_box(handle, |map| {
        let value = map.get_opt_key_str(&key_str)?;
        Some(box_to_handle(value))
    })
    .flatten()
    .unwrap_or(0)
}
