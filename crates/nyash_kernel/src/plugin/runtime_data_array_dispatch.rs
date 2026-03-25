use super::value_codec::{
    any_arg_to_box_with_profile, any_arg_to_index, runtime_i64_from_box_ref, CodecProfile,
};
use nyash_rust::boxes::array::ArrayBox;

#[inline(always)]
fn resolve_array_index_key(arr: &ArrayBox, key_any: i64) -> Option<i64> {
    if key_any >= 0 && key_any <= arr.len() as i64 {
        return Some(key_any);
    }
    any_arg_to_index(key_any)
}

#[inline(always)]
pub(super) fn runtime_data_array_get_hh(arr: &ArrayBox, key_any: i64) -> i64 {
    let Some(idx) = resolve_array_index_key(arr, key_any) else {
        return 0;
    };
    if idx < 0 {
        return 0;
    }
    let items = arr.items.read();
    items
        .get(idx as usize)
        .map(|item| runtime_i64_from_box_ref(item.as_ref()))
        .unwrap_or(0)
}

#[inline(always)]
pub(super) fn runtime_data_array_set_hhh(arr: &ArrayBox, key_any: i64, val_any: i64) -> i64 {
    let Some(idx) = resolve_array_index_key(arr, key_any) else {
        return 0;
    };
    let value = any_arg_to_box_with_profile(val_any, CodecProfile::ArrayFastBorrowString);
    if arr.slot_store_box_raw(idx, value) {
        1
    } else {
        0
    }
}

#[inline(always)]
pub(super) fn runtime_data_array_has_hh(arr: &ArrayBox, key_any: i64) -> i64 {
    let Some(idx) = resolve_array_index_key(arr, key_any) else {
        return 0;
    };
    if arr.has_index_i64(idx) {
        1
    } else {
        0
    }
}

#[inline(always)]
pub(super) fn runtime_data_array_push_hh(arr: &ArrayBox, val_any: i64) -> i64 {
    arr.slot_append_box_raw(any_arg_to_box_with_profile(
        val_any,
        CodecProfile::ArrayFastBorrowString,
    ))
}
