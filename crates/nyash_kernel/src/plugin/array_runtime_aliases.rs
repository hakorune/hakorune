use super::array_compat::append_integer_raw;
use super::array_runtime_any::{
    array_runtime_get_any_key, array_runtime_has_any_key, array_runtime_set_any_key,
};
use super::array_runtime_facade::{
    array_runtime_get_idx, array_runtime_has_idx, array_runtime_set_idx_any,
    array_runtime_set_idx_i64, array_runtime_set_idx_string_handle,
};
use super::array_runtime_substrate::array_runtime_push_any;

// Historical/compat array ABI aliases.
// Keep these exported names separate from the forwarding core so `.hako` owner
// cutover can treat compat aliases as a shrink-only surface.

#[export_name = "nyash.array.get_hh"]
pub extern "C" fn nyash_array_get_hh_alias(handle: i64, key_any: i64) -> i64 {
    array_runtime_get_any_key(handle, key_any)
}

#[export_name = "nyash.array.set_hhh"]
pub extern "C" fn nyash_array_set_hhh_alias(handle: i64, key_any: i64, val_any: i64) -> i64 {
    array_runtime_set_any_key(handle, key_any, val_any)
}

#[export_name = "nyash.array.has_hh"]
pub extern "C" fn nyash_array_has_hh_alias(handle: i64, key_any: i64) -> i64 {
    array_runtime_has_any_key(handle, key_any)
}

#[export_name = "nyash.array.push_hh"]
pub extern "C" fn nyash_array_push_hh_alias(handle: i64, val_any: i64) -> i64 {
    array_runtime_push_any(handle, val_any)
}

#[export_name = "nyash.array.push_hi"]
pub extern "C" fn nyash_array_push_hi_alias(handle: i64, value_i64: i64) -> i64 {
    append_integer_raw(handle, value_i64)
}

#[export_name = "nyash.array.get_hi"]
pub extern "C" fn nyash_array_get_hi_alias(handle: i64, idx: i64) -> i64 {
    array_runtime_get_idx(handle, idx)
}

#[export_name = "nyash.array.set_hih"]
pub extern "C" fn nyash_array_set_hih_alias(handle: i64, idx: i64, val_any: i64) -> i64 {
    array_runtime_set_idx_any(handle, idx, val_any)
}

#[export_name = "nyash.array.set_hii"]
pub extern "C" fn nyash_array_set_hii_alias(handle: i64, idx: i64, value_i64: i64) -> i64 {
    array_runtime_set_idx_i64(handle, idx, value_i64)
}

#[export_name = "nyash.array.set_his"]
pub extern "C" fn nyash_array_set_his_alias(handle: i64, idx: i64, value_h: i64) -> i64 {
    array_runtime_set_idx_string_handle(handle, idx, value_h)
}

#[export_name = "nyash.array.has_hi"]
pub extern "C" fn nyash_array_has_hi_alias(handle: i64, idx: i64) -> i64 {
    array_runtime_has_idx(handle, idx)
}
