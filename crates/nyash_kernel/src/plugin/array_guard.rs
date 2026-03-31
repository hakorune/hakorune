#[inline(always)]
pub(super) fn valid_handle(handle: i64) -> bool {
    handle > 0
}

#[inline(always)]
pub(super) fn valid_handle_idx(handle: i64, idx: i64) -> bool {
    handle > 0 && idx >= 0
}

