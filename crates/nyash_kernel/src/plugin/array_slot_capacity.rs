use super::array_guard::{valid_handle, valid_handle_idx};
use super::array_handle_cache::with_array_box;

#[inline(always)]
pub(super) fn array_slot_cap_i64(handle: i64) -> i64 {
    if !valid_handle(handle) {
        return 0;
    }
    with_array_box(handle, |arr| arr.capacity() as i64).unwrap_or(0)
}

#[inline(always)]
pub(super) fn array_slot_reserve_i64(handle: i64, additional: i64) -> i64 {
    if !valid_handle_idx(handle, additional) {
        return 0;
    }
    with_array_box(handle, |arr| {
        if arr.slot_reserve_capacity_raw(additional as usize) {
            1
        } else {
            0
        }
    })
    .unwrap_or(0)
}

#[inline(always)]
pub(super) fn array_slot_grow_i64(handle: i64, target_capacity: i64) -> i64 {
    if !valid_handle_idx(handle, target_capacity) {
        return 0;
    }
    with_array_box(handle, |arr| {
        if arr.slot_grow_capacity_raw(target_capacity as usize) {
            1
        } else {
            0
        }
    })
    .unwrap_or(0)
}
