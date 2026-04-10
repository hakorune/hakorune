use super::array_guard::valid_handle;
use super::array_handle_cache::with_array_box;
use super::value_codec::{any_arg_to_box_with_profile, CodecProfile};

#[inline(always)]
pub(super) fn array_slot_append_any(handle: i64, val_any: i64) -> i64 {
    if !valid_handle(handle) {
        return 0;
    }
    with_array_box(handle, |arr| {
        let value = any_arg_to_box_with_profile(val_any, CodecProfile::ArrayFastBorrowString);
        if let Some(i64_value) = value.as_i64_fast() {
            let idx = arr.len() as i64;
            if arr.slot_store_i64_raw(idx, i64_value) {
                idx + 1
            } else {
                0
            }
        } else if let Some(bool_value) = value.as_bool_fast() {
            let idx = arr.len() as i64;
            if arr.slot_store_bool_raw(idx, bool_value) {
                idx + 1
            } else {
                0
            }
        } else if let Some(f64_value) = value.as_f64_fast() {
            let idx = arr.len() as i64;
            if arr.slot_store_f64_raw(idx, f64_value) {
                idx + 1
            } else {
                0
            }
        } else {
            arr.slot_append_box_raw(value)
        }
    })
    .unwrap_or(0)
}
