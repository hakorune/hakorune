use super::value_codec::{any_arg_to_box, bool_box_to_i64, box_to_runtime_i64};
use nyash_rust::boxes::map_box::MapBox;

#[inline(always)]
pub(super) fn runtime_data_map_get_hh(map: &MapBox, key_any: i64) -> i64 {
    let key_box = any_arg_to_box(key_any);
    map.get_opt(key_box).map(box_to_runtime_i64).unwrap_or(0)
}

#[inline(always)]
pub(super) fn runtime_data_map_set_hhh(map: &MapBox, key_any: i64, val_any: i64) -> i64 {
    let key_box = any_arg_to_box(key_any);
    let val_box = any_arg_to_box(val_any);
    let _ = map.set(key_box, val_box);
    1
}

#[inline(always)]
pub(super) fn runtime_data_map_has_hh(map: &MapBox, key_any: i64) -> i64 {
    let key_box = any_arg_to_box(key_any);
    let out = map.has(key_box);
    bool_box_to_i64(out.as_ref()).unwrap_or(0)
}
