use super::handle_cache::array_get_index_encoded_i64;

#[inline(always)]
pub(super) fn array_slot_load_encoded_i64(handle: i64, idx: i64) -> i64 {
    array_get_index_encoded_i64(handle, idx).unwrap_or(0)
}

#[inline(always)]
pub(super) fn array_slot_has_index(handle: i64, idx: i64) -> i64 {
    if handle <= 0 || idx < 0 {
        return 0;
    }
    super::handle_cache::with_array_box(handle, |arr| if arr.has_index_i64(idx) { 1 } else { 0 })
        .unwrap_or(0)
}
