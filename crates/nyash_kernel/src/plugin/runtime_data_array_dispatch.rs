use super::array_runtime_any::{
    array_runtime_get_any_key, array_runtime_has_any_key, array_runtime_set_any_key,
};
use super::array_runtime_substrate::array_runtime_push_any;

// Array branch for RuntimeDataBox facade dispatch.
// Keep RuntimeDataBox itself protocol-only by isolating Array-specific routing
// away from the top-level runtime_data dispatch shell.

#[inline(always)]
pub(super) fn runtime_data_array_get_any(recv_h: i64, key_any: i64) -> i64 {
    array_runtime_get_any_key(recv_h, key_any)
}

#[inline(always)]
pub(super) fn runtime_data_array_set_any(recv_h: i64, key_any: i64, val_any: i64) -> i64 {
    array_runtime_set_any_key(recv_h, key_any, val_any)
}

#[inline(always)]
pub(super) fn runtime_data_array_has_any(recv_h: i64, key_any: i64) -> i64 {
    array_runtime_has_any_key(recv_h, key_any)
}

#[inline(always)]
pub(super) fn runtime_data_array_push_any(recv_h: i64, val_any: i64) -> i64 {
    array_runtime_push_any(recv_h, val_any)
}
