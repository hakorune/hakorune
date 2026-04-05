use super::array_string_slot::array_string_store_handle_at;
use super::array_guard::valid_handle_idx;
use super::array_handle_cache::with_array_box;
use super::value_codec::{decode_array_fast_value, ArrayFastDecodedValue};

#[inline(always)]
fn array_slot_store_box(handle: i64, idx: i64, value: Box<dyn nyash_rust::box_trait::NyashBox>) -> i64 {
    with_array_box(handle, |arr| {
        if arr.slot_store_box_raw(idx, value) {
            1
        } else {
            0
        }
    })
    .unwrap_or(0)
}

#[inline(always)]
pub(super) fn array_slot_store_any(handle: i64, idx: i64, val_any: i64) -> i64 {
    if !valid_handle_idx(handle, idx) {
        return 0;
    }
    match decode_array_fast_value(val_any) {
        ArrayFastDecodedValue::ImmediateI64(v) => array_slot_store_i64(handle, idx, v),
        ArrayFastDecodedValue::Boxed(value) => array_slot_store_box(handle, idx, value),
    }
}

#[inline(always)]
pub(super) fn array_slot_store_i64(handle: i64, idx: i64, value_i64: i64) -> i64 {
    if !valid_handle_idx(handle, idx) {
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
    // phase-151x visibility lock:
    // executor leaf for the current concrete `nyash.array.set_his` path that
    // reads as canonical `store.array.str`.
    array_string_store_handle_at(handle, idx, value_h)
}

#[inline(always)]
pub(super) fn array_slot_rmw_add1_i64(handle: i64, idx: i64) -> i64 {
    if !valid_handle_idx(handle, idx) {
        return 0;
    }
    with_array_box(handle, |arr| arr.slot_rmw_add1_i64_raw(idx))
        .flatten()
        .unwrap_or(0)
}
