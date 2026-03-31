use super::handle_cache::with_map_box;
use super::map_key_codec::{map_key_string_from_i64, map_key_with_any_str_ref};

#[inline(always)]
pub(super) fn map_probe_contains_i64(handle: i64, key_i64: i64) -> i64 {
    with_map_box(handle, |map| {
        let key_str = map_key_string_from_i64(key_i64);
        if map.contains_key_str(&key_str) {
            1
        } else {
            0
        }
    })
    .unwrap_or(0)
}

#[inline(always)]
pub(super) fn map_probe_contains_any(handle: i64, key_any: i64) -> i64 {
    map_key_with_any_str_ref(key_any, |key_str| {
        with_map_box(handle, |map| {
            if map.contains_key_str(key_str) {
                1
            } else {
                0
            }
        })
        .unwrap_or(0)
    })
}
