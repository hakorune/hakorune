use super::array_runtime_facade::{
    array_runtime_get_any_key, array_runtime_has_any_key, array_runtime_push_any,
    array_runtime_set_any_key,
};

#[inline(always)]
pub(super) fn runtime_data_array_get_hh(handle: i64, key_any: i64) -> i64 {
    array_runtime_get_any_key(handle, key_any)
}

#[inline(always)]
pub(super) fn runtime_data_array_set_hhh(handle: i64, key_any: i64, val_any: i64) -> i64 {
    array_runtime_set_any_key(handle, key_any, val_any)
}

#[inline(always)]
pub(super) fn runtime_data_array_has_hh(handle: i64, key_any: i64) -> i64 {
    array_runtime_has_any_key(handle, key_any)
}

#[inline(always)]
pub(super) fn runtime_data_array_push_hh(handle: i64, val_any: i64) -> i64 {
    array_runtime_push_any(handle, val_any)
}
