use super::handle_helpers::with_map_box;
use super::value_codec::{any_arg_to_box, bool_box_to_i64, int_arg_to_box};

#[inline(always)]
pub(super) fn map_probe_contains_i64(handle: i64, key_i64: i64) -> i64 {
    with_map_box(handle, |map| {
        let out = map.has(int_arg_to_box(key_i64));
        bool_box_to_i64(out.as_ref()).unwrap_or(0)
    })
    .unwrap_or(0)
}

#[inline(always)]
pub(super) fn map_probe_contains_any(handle: i64, key_any: i64) -> i64 {
    with_map_box(handle, |map| {
        let key_box = any_arg_to_box(key_any);
        let out = map.has(key_box);
        bool_box_to_i64(out.as_ref()).unwrap_or(0)
    })
    .unwrap_or(0)
}
