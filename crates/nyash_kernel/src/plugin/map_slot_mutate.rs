use super::handle_cache::with_map_box;
use super::map_key_codec::map_key_string_from_any;

#[inline(always)]
pub(super) fn map_slot_clear(handle: i64) -> i64 {
    let _ = with_map_box(handle, |map| {
        map.clear_entries();
    });
    0
}

#[inline(always)]
pub(super) fn map_slot_delete_any(handle: i64, key_any: i64) -> i64 {
    let key_str = map_key_string_from_any(key_any);
    with_map_box(
        handle,
        |map| {
            if map.remove_key_str(&key_str) {
                1
            } else {
                0
            }
        },
    )
    .unwrap_or(0)
}
