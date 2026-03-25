use super::array_string_slot::array_set_by_index_string_handle_value;
use super::handle_cache::with_array_box;
use super::value_codec::{decode_array_fast_value, ArrayFastDecodedValue};

#[inline(always)]
pub(super) fn array_slot_store_any(handle: i64, idx: i64, val_any: i64) -> i64 {
    if handle <= 0 || idx < 0 {
        return 0;
    }
    match decode_array_fast_value(val_any) {
        ArrayFastDecodedValue::ImmediateI64(v) => array_slot_store_i64(handle, idx, v),
        ArrayFastDecodedValue::Boxed(value) => with_array_box(handle, |arr| {
            if arr.slot_store_box_raw(idx, value) {
                1
            } else {
                0
            }
        })
        .unwrap_or(0),
    }
}

#[inline(always)]
pub(super) fn array_slot_store_i64(handle: i64, idx: i64, value_i64: i64) -> i64 {
    if handle <= 0 || idx < 0 {
        return 0;
    }
    with_array_box(handle, |arr| {
        if arr.slot_store_i64_raw(idx, value_i64) {
            1
        } else {
            0
        }
    })
    .unwrap_or(0)
}

#[inline(always)]
pub(super) fn array_slot_store_string_handle(handle: i64, idx: i64, value_h: i64) -> i64 {
    array_set_by_index_string_handle_value(handle, idx, value_h)
}
