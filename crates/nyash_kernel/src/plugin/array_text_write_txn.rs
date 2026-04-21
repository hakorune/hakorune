use super::array_handle_cache::with_array_box;
use nyash_rust::boxes::array::ArrayBox;

pub(crate) enum ArrayTextWriteTxnOutcome<R> {
    Resident(R),
    Fallback(R),
}

pub(crate) struct ArrayTextWriteTxn<'a> {
    arr: &'a ArrayBox,
    idx: i64,
}

impl<'a> ArrayTextWriteTxn<'a> {
    #[inline(always)]
    fn new(arr: &'a ArrayBox, idx: i64) -> Self {
        Self { arr, idx }
    }

    #[inline(always)]
    fn update<R>(self, f: impl FnOnce(&mut String) -> R) -> Option<R> {
        self.arr.slot_update_text_raw(self.idx, f)
    }

    #[inline(always)]
    fn update_resident_first<R>(
        self,
        f: impl FnOnce(&mut String) -> R,
    ) -> Option<ArrayTextWriteTxnOutcome<R>> {
        self.arr
            .slot_update_text_resident_first_raw(self.idx, f)
            .map(|(out, resident)| {
                if resident {
                    ArrayTextWriteTxnOutcome::Resident(out)
                } else {
                    ArrayTextWriteTxnOutcome::Fallback(out)
                }
            })
    }
}

#[inline(always)]
pub(crate) fn with_array_text_write_txn<R>(
    handle: i64,
    idx: i64,
    f: impl FnOnce(ArrayTextWriteTxn<'_>) -> R,
) -> Option<R> {
    with_array_box(handle, |arr| f(ArrayTextWriteTxn::new(arr, idx)))
}

#[inline(always)]
pub(crate) fn with_array_text_slot_update<R>(
    handle: i64,
    idx: i64,
    f: impl FnOnce(&mut String) -> R,
) -> Option<R> {
    with_array_text_write_txn(handle, idx, |txn| txn.update(f)).flatten()
}

#[inline(always)]
pub(crate) fn with_array_text_slot_update_resident_first<R>(
    handle: i64,
    idx: i64,
    f: impl FnOnce(&mut String) -> R,
) -> Option<ArrayTextWriteTxnOutcome<R>> {
    with_array_text_write_txn(handle, idx, |txn| txn.update_resident_first(f)).flatten()
}
