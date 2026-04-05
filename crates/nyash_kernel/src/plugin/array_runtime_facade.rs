use super::array_slot_load::{array_slot_has_index, array_slot_load_encoded_i64};
use super::array_slot_store::{
    array_slot_rmw_add1_i64, array_slot_store_any, array_slot_store_i64, array_slot_store_string_handle,
};

// Runtime/compat forwarding only.
// Array semantic ownership lives in `.hako` (`ArrayCoreBox` / `ArrayStateCoreBox`);
// keep this module limited to index-backed forwarding only.

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

// Current runtime facade spelling for the canonical `store.array.str` contract.
// Keep semantic ownership above this layer and treat this as index-backed
// forwarding only.
pub(super) fn array_runtime_store_array_string(handle: i64, idx: i64, value_h: i64) -> i64 {
    array_slot_store_string_handle(handle, idx, value_h)
}

pub(super) fn array_runtime_has_idx(handle: i64, idx: i64) -> i64 {
    array_slot_has_index(handle, idx)
}

pub(super) fn array_runtime_rmw_add1_idx(handle: i64, idx: i64) -> i64 {
    array_slot_rmw_add1_i64(handle, idx)
}
