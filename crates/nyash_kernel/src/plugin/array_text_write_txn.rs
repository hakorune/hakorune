use super::array_handle_cache::with_array_box;

pub(crate) enum ArrayTextWriteTxnOutcome<R> {
    Resident(R),
    Fallback(R),
}

#[inline(always)]
pub(crate) fn with_array_text_slot_update<R>(
    handle: i64,
    idx: i64,
    f: impl FnOnce(&mut String) -> R,
) -> Option<R> {
    with_array_box(handle, |arr| arr.slot_update_text_raw(idx, f)).flatten()
}

#[inline(always)]
pub(crate) fn with_array_text_slot_update_resident_first<R>(
    handle: i64,
    idx: i64,
    f: impl FnOnce(&mut String) -> R,
) -> Option<ArrayTextWriteTxnOutcome<R>> {
    with_array_box(handle, |arr| {
        arr.slot_update_text_resident_first_raw(idx, f)
            .map(|(out, resident)| {
                if resident {
                    ArrayTextWriteTxnOutcome::Resident(out)
                } else {
                    ArrayTextWriteTxnOutcome::Fallback(out)
                }
            })
    })
    .flatten()
}
