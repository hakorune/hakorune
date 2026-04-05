use super::array_guard::valid_handle;
use super::array_compat::{append_integer_raw, nyash_array_length_h};
use super::array_slot_append::array_slot_append_any;
use super::array_slot_capacity::{array_slot_cap_i64, array_slot_grow_i64, array_slot_reserve_i64};
use super::array_slot_load::{array_slot_has_index, array_slot_load_encoded_i64};
use super::array_slot_store::{
    array_slot_rmw_add1_i64, array_slot_store_any, array_slot_store_i64, array_slot_store_string_handle,
};
use super::array_string_slot::{array_string_indexof_by_index, array_string_len_by_index};
use super::value_codec::any_arg_to_index;

// Runtime/compat forwarding only.
// Array semantic ownership lives in `.hako` (`ArrayCoreBox` / `ArrayStateCoreBox`);
// keep this module limited to handle/index coercion and stable host ABI forwarding.

// Shared fail-safe guards for runtime-only array routes.
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

#[inline(always)]
fn with_runtime_handle_or_zero(handle: i64, f: impl FnOnce() -> i64) -> i64 {
    if !valid_handle(handle) {
        return 0;
    }
    f()
}

// Any-key runtime facade.
// Used by RuntimeData-style dispatch when the key stays in `any` form.
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

// Handle-only runtime facade.
pub(super) fn array_runtime_push_any(handle: i64, val_any: i64) -> i64 {
    with_runtime_handle_or_zero(handle, || array_slot_append_any(handle, val_any))
}

// Index-backed slot facade.
pub(super) fn array_runtime_get_idx(handle: i64, idx: i64) -> i64 {
    array_slot_load_encoded_i64(handle, idx)
}

pub(super) fn array_runtime_set_idx_any(handle: i64, idx: i64, val_any: i64) -> i64 {
    array_slot_store_any(handle, idx, val_any)
}

pub(super) fn array_runtime_set_idx_i64(handle: i64, idx: i64, value_i64: i64) -> i64 {
    array_slot_store_i64(handle, idx, value_i64)
}

pub(super) fn array_runtime_set_idx_string_handle(handle: i64, idx: i64, value_h: i64) -> i64 {
    array_slot_store_string_handle(handle, idx, value_h)
}

pub(super) fn array_runtime_has_idx(handle: i64, idx: i64) -> i64 {
    array_slot_has_index(handle, idx)
}

pub(super) fn array_runtime_rmw_add1_idx(handle: i64, idx: i64) -> i64 {
    array_slot_rmw_add1_i64(handle, idx)
}

// Length/capacity facade.
pub(super) fn array_runtime_len(handle: i64) -> i64 {
    nyash_array_length_h(handle)
}

pub(super) fn array_runtime_cap(handle: i64) -> i64 {
    array_slot_cap_i64(handle)
}

pub(super) fn array_runtime_reserve(handle: i64, additional: i64) -> i64 {
    array_slot_reserve_i64(handle, additional)
}

pub(super) fn array_runtime_grow(handle: i64, target_capacity: i64) -> i64 {
    array_slot_grow_i64(handle, target_capacity)
}

// String slot facade.
pub(super) fn array_runtime_string_len_at(handle: i64, idx: i64) -> i64 {
    array_string_len_by_index(handle, idx)
}

pub(super) fn array_runtime_string_indexof_at(handle: i64, idx: i64, needle_h: i64) -> i64 {
    array_string_indexof_by_index(handle, idx, needle_h)
}

// ABI exports for runtime-only aliases. These are intentionally non-owning and
// should shrink over time as `.hako` semantic owners absorb visible policy.
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

// RuntimeData mono-route aliases with integer-key contract.
// These routes are selected by lowering when key VID is proven i64/non-negative.
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
