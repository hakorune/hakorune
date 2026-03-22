use super::handle_cache::with_array_box;

#[inline(always)]
pub(super) fn array_slot_reserve_i64(handle: i64, additional: i64) -> i64 {
    if handle <= 0 || additional < 0 {
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
    if handle <= 0 || target_capacity < 0 {
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
