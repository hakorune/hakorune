use super::array_slot_load::{array_slot_has_index, array_slot_load_encoded_i64};
use super::array_slot_store::{
    array_slot_rmw_add1_i64, array_slot_store_any, array_slot_store_i64,
    array_slot_store_kernel_text_slot, array_slot_store_string_handle,
};
use super::array_string_slot::{
    array_string_concat_const_suffix_by_index_into_slot,
    array_string_concat_const_suffix_by_index_store_same_slot,
    array_string_concat_const_suffix_by_index_store_same_slot_len,
    array_string_insert_const_mid_by_index_into_slot,
    array_string_insert_const_mid_by_index_store_same_slot,
    array_string_insert_const_mid_by_index_store_same_slot_len,
    array_string_insert_const_mid_subrange_by_index_store_same_slot,
    array_string_insert_const_mid_subrange_by_index_store_same_slot_len,
    array_string_insert_const_mid_subrange_len_by_index_store_same_slot_len,
    array_string_insert_const_mid_subrange_len_region_store_len,
};
use super::KernelTextSlot;

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

// phase-151x visibility lock:
// current runtime facade spelling for canonical `store.array.str`.
// Keep semantic ownership above this layer and treat this as index-backed
// forwarding only.
pub(super) fn array_runtime_store_array_string(handle: i64, idx: i64, value_h: i64) -> i64 {
    array_slot_store_string_handle(handle, idx, value_h)
}

pub(super) fn array_runtime_store_array_kernel_text_slot(
    handle: i64,
    idx: i64,
    slot: &mut KernelTextSlot,
) -> i64 {
    array_slot_store_kernel_text_slot(handle, idx, slot)
}

pub(super) fn array_runtime_concat_const_suffix_idx_into_slot(
    slot: &mut KernelTextSlot,
    handle: i64,
    idx: i64,
    suffix_ptr: *const i8,
) -> i64 {
    array_string_concat_const_suffix_by_index_into_slot(slot, handle, idx, suffix_ptr)
}

pub(super) fn array_runtime_concat_const_suffix_idx_store_same_slot(
    handle: i64,
    idx: i64,
    suffix_ptr: *const i8,
) -> i64 {
    array_string_concat_const_suffix_by_index_store_same_slot(handle, idx, suffix_ptr)
}

pub(super) fn array_runtime_concat_const_suffix_idx_store_same_slot_len(
    handle: i64,
    idx: i64,
    suffix_ptr: *const i8,
    suffix_len: i64,
) -> i64 {
    array_string_concat_const_suffix_by_index_store_same_slot_len(
        handle, idx, suffix_ptr, suffix_len,
    )
}

pub(super) fn array_runtime_insert_const_mid_idx_into_slot(
    slot: &mut KernelTextSlot,
    handle: i64,
    idx: i64,
    middle_ptr: *const i8,
    split: i64,
) -> i64 {
    array_string_insert_const_mid_by_index_into_slot(slot, handle, idx, middle_ptr, split)
}

pub(super) fn array_runtime_insert_const_mid_idx_store_same_slot(
    handle: i64,
    idx: i64,
    middle_ptr: *const i8,
    split: i64,
) -> i64 {
    array_string_insert_const_mid_by_index_store_same_slot(handle, idx, middle_ptr, split)
}

pub(super) fn array_runtime_insert_const_mid_idx_store_same_slot_len(
    handle: i64,
    idx: i64,
    middle_ptr: *const i8,
    middle_len: i64,
    split: i64,
) -> i64 {
    array_string_insert_const_mid_by_index_store_same_slot_len(
        handle, idx, middle_ptr, middle_len, split,
    )
}

pub(super) fn array_runtime_insert_const_mid_subrange_idx_store_same_slot(
    handle: i64,
    idx: i64,
    middle_ptr: *const i8,
    split: i64,
    start: i64,
    end: i64,
) -> i64 {
    array_string_insert_const_mid_subrange_by_index_store_same_slot(
        handle, idx, middle_ptr, split, start, end,
    )
}

pub(super) fn array_runtime_insert_const_mid_subrange_idx_store_same_slot_len(
    handle: i64,
    idx: i64,
    middle_ptr: *const i8,
    middle_len: i64,
    split: i64,
    start: i64,
    end: i64,
) -> i64 {
    array_string_insert_const_mid_subrange_by_index_store_same_slot_len(
        handle, idx, middle_ptr, middle_len, split, start, end,
    )
}

pub(super) fn array_runtime_insert_const_mid_subrange_len_idx_store_same_slot_len(
    handle: i64,
    idx: i64,
    middle_ptr: *const i8,
    middle_len: i64,
) -> i64 {
    array_string_insert_const_mid_subrange_len_by_index_store_same_slot_len(
        handle, idx, middle_ptr, middle_len,
    )
}

pub(super) fn array_runtime_insert_const_mid_subrange_len_region_store_len(
    handle: i64,
    loop_bound: i64,
    row_modulus: i64,
    middle_ptr: *const i8,
    middle_len: i64,
) -> i64 {
    array_string_insert_const_mid_subrange_len_region_store_len(
        handle,
        loop_bound,
        row_modulus,
        middle_ptr,
        middle_len,
    )
}

pub(super) fn array_runtime_has_idx(handle: i64, idx: i64) -> i64 {
    array_slot_has_index(handle, idx)
}

pub(super) fn array_runtime_rmw_add1_idx(handle: i64, idx: i64) -> i64 {
    array_slot_rmw_add1_i64(handle, idx)
}
