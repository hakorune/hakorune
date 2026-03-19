use super::handle_helpers::with_array_box;
pub(super) use super::array_string_slot::array_set_by_index_string_handle_value;
use super::value_codec::{decode_array_fast_value, ArrayFastDecodedValue};

pub(super) fn array_set_by_index(handle: i64, idx: i64, val_any: i64) -> i64 {
    if handle <= 0 || idx < 0 {
        return 0;
    }
    match decode_array_fast_value(val_any) {
        ArrayFastDecodedValue::ImmediateI64(v) => array_set_by_index_i64_value(handle, idx, v),
        ArrayFastDecodedValue::Boxed(value) => with_array_box(handle, |arr| {
            if arr.try_set_index_i64(idx, value) {
                1
            } else {
                0
            }
        })
        .unwrap_or(0),
    }
}

pub(super) fn array_set_by_index_i64_value(handle: i64, idx: i64, value_i64: i64) -> i64 {
    if handle <= 0 || idx < 0 {
        return 0;
    }
    with_array_box(handle, |arr| {
        if arr.try_set_index_i64_integer(idx, value_i64) {
            1
        } else {
            0
        }
    })
    .unwrap_or(0)
}
