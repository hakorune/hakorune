use super::array_guard::valid_handle;
use super::array_runtime_facade::{
    array_runtime_get_idx, array_runtime_has_idx, array_runtime_set_idx_any,
};
use super::value_codec::any_arg_to_index;

// RuntimeData-style any-key Array routes.
// Keep any-key coercion separate from the slot/substrate forwarding core so
// `.hako` owner cutover can treat these routes as a shrink-only runtime shell.

#[inline(always)]
fn with_runtime_index_or_zero(handle: i64, key_any: i64, f: impl FnOnce(i64) -> i64) -> i64 {
    if !valid_handle(handle) {
        return 0;
    }
    let Some(idx) = any_arg_to_index(key_any) else {
        return 0;
    };
    f(idx)
}

pub(super) fn array_runtime_get_any_key(handle: i64, key_any: i64) -> i64 {
    with_runtime_index_or_zero(handle, key_any, |idx| array_runtime_get_idx(handle, idx))
}

pub(super) fn array_runtime_set_any_key(handle: i64, key_any: i64, val_any: i64) -> i64 {
    with_runtime_index_or_zero(handle, key_any, |idx| {
        array_runtime_set_idx_any(handle, idx, val_any)
    })
}

pub(super) fn array_runtime_has_any_key(handle: i64, key_any: i64) -> i64 {
    with_runtime_index_or_zero(handle, key_any, |idx| array_runtime_has_idx(handle, idx))
}
