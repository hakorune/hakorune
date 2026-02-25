// RuntimeDataBox-compatible dynamic dispatch helpers.
//
// These exports bridge RuntimeDataBox method calls in AOT/LLVM to concrete
// host boxes (ArrayBox/MapBox) without relying on static box-name guesses.

use super::handle_helpers::with_array_or_map;
use super::value_codec::{
    any_arg_to_box, any_arg_to_box_with_profile, any_arg_to_index, bool_box_to_i64,
    box_to_runtime_i64, runtime_i64_from_box_ref, CodecProfile,
};
use nyash_rust::boxes::array::ArrayBox;

#[inline(always)]
fn resolve_array_index_key(arr: &ArrayBox, key_any: i64) -> Option<i64> {
    // Keep positive immediates stable on array routes when key is in
    // immediate index range, even if an unrelated live handle shares the id.
    if key_any >= 0 && key_any <= arr.len() as i64 {
        return Some(key_any);
    }
    any_arg_to_index(key_any)
}

// nyash.runtime_data.get_hh(recv_h, key_any) -> value_handle (or 0)
#[export_name = "nyash.runtime_data.get_hh"]
pub extern "C" fn nyash_runtime_data_get_hh(recv_h: i64, key_any: i64) -> i64 {
    with_array_or_map(
        recv_h,
        |arr| {
            let Some(idx) = resolve_array_index_key(arr, key_any) else {
                return 0;
            };
            if idx < 0 {
                // Keep legacy contract: negative array index resolves to immediate 0.
                return 0;
            }
            let idx_usize = idx as usize;
            let items = arr.items.read();
            items
                .get(idx_usize)
                .map(|item| runtime_i64_from_box_ref(item.as_ref()))
                .unwrap_or(0)
        },
        |map| {
            let key_box = any_arg_to_box(key_any);
            map.get_opt(key_box).map(box_to_runtime_i64).unwrap_or(0)
        },
    )
    .unwrap_or(0)
}

// nyash.runtime_data.set_hhh(recv_h, key_any, val_any) -> 0/1
#[export_name = "nyash.runtime_data.set_hhh"]
pub extern "C" fn nyash_runtime_data_set_hhh(recv_h: i64, key_any: i64, val_any: i64) -> i64 {
    with_array_or_map(
        recv_h,
        |arr| {
            let Some(idx) = resolve_array_index_key(arr, key_any) else {
                return 0;
            };
            let value = any_arg_to_box_with_profile(val_any, CodecProfile::ArrayFastBorrowString);
            if arr.try_set_index_i64(idx, value) {
                1
            } else {
                0
            }
        },
        |map| {
            let key_box = any_arg_to_box(key_any);
            let val_box = any_arg_to_box(val_any);
            let _ = map.set(key_box, val_box);
            1
        },
    )
    .unwrap_or(0)
}

// nyash.runtime_data.has_hh(recv_h, key_any) -> 0/1
#[export_name = "nyash.runtime_data.has_hh"]
pub extern "C" fn nyash_runtime_data_has_hh(recv_h: i64, key_any: i64) -> i64 {
    with_array_or_map(
        recv_h,
        |arr| {
            let Some(idx) = resolve_array_index_key(arr, key_any) else {
                return 0;
            };
            if arr.has_index_i64(idx) {
                1
            } else {
                0
            }
        },
        |map| {
            let key_box = any_arg_to_box(key_any);
            let out = map.has(key_box);
            bool_box_to_i64(out.as_ref()).unwrap_or(0)
        },
    )
    .unwrap_or(0)
}

// nyash.runtime_data.push_hh(recv_h, val_any) -> new_len (array) / 0
#[export_name = "nyash.runtime_data.push_hh"]
pub extern "C" fn nyash_runtime_data_push_hh(recv_h: i64, val_any: i64) -> i64 {
    with_array_or_map(
        recv_h,
        |arr| {
            let _ = arr.push(any_arg_to_box_with_profile(
                val_any,
                CodecProfile::ArrayFastBorrowString,
            ));
            arr.len() as i64
        },
        |_map| 0,
    )
    .unwrap_or(0)
}
