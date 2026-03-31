use super::array_guard::valid_handle;
use super::array_handle_cache::with_array_box;
use super::value_codec::{any_arg_to_box_with_profile, CodecProfile};

#[inline(always)]
pub(super) fn array_slot_append_any(handle: i64, val_any: i64) -> i64 {
    if !valid_handle(handle) {
        return 0;
    }
    with_array_box(handle, |arr| {
        arr.slot_append_box_raw(any_arg_to_box_with_profile(
            val_any,
            CodecProfile::ArrayFastBorrowString,
        ))
    })
    .unwrap_or(0)
}
