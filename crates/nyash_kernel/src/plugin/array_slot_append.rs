use super::handle_helpers::with_array_box;
use super::value_codec::{any_arg_to_box_with_profile, CodecProfile};

#[inline(always)]
pub(super) fn array_slot_append_any(handle: i64, val_any: i64) -> i64 {
    if handle <= 0 {
        return 0;
    }
    with_array_box(handle, |arr| {
        let _ = arr.push(any_arg_to_box_with_profile(
            val_any,
            CodecProfile::ArrayFastBorrowString,
        ));
        arr.len() as i64
    })
    .unwrap_or(0)
}
