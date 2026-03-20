use super::handle_helpers::{array_get_index_encoded_i64, with_array_box};

#[inline(always)]
pub(super) fn array_slot_load_encoded_i64(handle: i64, idx: i64) -> i64 {
    if let Some(out) = with_array_box(handle, |arr| {
        if idx < 0 {
            return None;
        }
        let items = arr.items.read();
        let item = items.get(idx as usize)?;
        if let Some(iv) = item.as_i64_fast() {
            return Some(iv);
        }
        if let Some(bv) = item.as_bool_fast() {
            return Some(if bv { 1 } else { 0 });
        }
        None
    })
    .flatten()
    {
        return out;
    }
    array_get_index_encoded_i64(handle, idx).unwrap_or(0)
}

#[inline(always)]
pub(super) fn array_slot_has_index(handle: i64, idx: i64) -> i64 {
    if handle <= 0 || idx < 0 {
        return 0;
    }
    with_array_box(handle, |arr| if arr.has_index_i64(idx) { 1 } else { 0 }).unwrap_or(0)
}
