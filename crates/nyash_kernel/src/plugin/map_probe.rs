use super::handle_cache::with_map_box;
use super::value_codec::{any_arg_to_box, int_arg_to_box};

#[inline(always)]
pub(super) fn map_probe_contains_i64(handle: i64, key_i64: i64) -> i64 {
    with_map_box(handle, |map| {
        let key_str = int_arg_to_box(key_i64).to_string_box().value;
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
    with_map_box(handle, |map| {
        let key_str = any_arg_to_box(key_any).to_string_box().value;
        if map.contains_key_str(&key_str) {
            1
        } else {
            0
        }
    })
    .unwrap_or(0)
}
