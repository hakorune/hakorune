use super::array_guard::valid_handle_idx;
use super::array_handle_cache::{array_get_index_encoded_i64, with_array_box};
use super::value_demand::ARRAY_GENERIC_GET_ENCODED;

#[inline(always)]
pub(super) fn array_slot_load_encoded_i64(handle: i64, idx: i64) -> i64 {
    let _demand = ARRAY_GENERIC_GET_ENCODED;
    array_get_index_encoded_i64(handle, idx).unwrap_or(0)
}

#[inline(always)]
pub(super) fn array_slot_has_index(handle: i64, idx: i64) -> i64 {
    if !valid_handle_idx(handle, idx) {
        return 0;
    }
    with_array_box(handle, |arr| if arr.has_index_i64(idx) { 1 } else { 0 }).unwrap_or(0)
}
