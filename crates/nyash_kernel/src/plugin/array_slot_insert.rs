use super::array_handle_cache::with_array_box;
use super::handle_cache::valid_handle;
use super::value_codec::{any_arg_to_box_with_profile, CodecProfile};

/// Generic ArrayElementWrite insert entry used by the typed LLVM consumer.
/// The published MIR row owns the receiver/index/value identity; this helper
/// only performs the existing ArrayBox storage operation.
#[inline(always)]
pub(super) fn array_slot_insert_any(handle: i64, idx: i64, val_any: i64) -> i64 {
    if !valid_handle(handle) || idx < 0 {
        return 0;
    }
    with_array_box(handle, |arr| {
        let value = any_arg_to_box_with_profile(val_any, CodecProfile::ArrayFastBorrowString);
        if arr.slot_insert_box_raw(idx, value) {
            1
        } else {
            0
        }
    })
    .unwrap_or(0)
}
