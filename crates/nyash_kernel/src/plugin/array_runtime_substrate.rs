use super::array_compat::nyash_array_length_h;
use super::array_direct_i64_buffer::direct_array_i64_push_i64;
use super::array_slot_append::array_slot_append_any;
use super::array_slot_capacity::{array_slot_cap_i64, array_slot_grow_i64, array_slot_reserve_i64};
use super::array_string_slot::{
    array_string_indexof_by_index, array_string_indexof_by_index_const_utf8,
    array_string_len_by_index,
};

// Mainline substrate-side Array forwarding.
// Keep daily `.hako` owner routes separate from compat/runtime facade code so
// semantic ownership can sit above this capability layer.

#[inline(always)]
fn with_runtime_handle_or_zero(handle: i64, f: impl FnOnce() -> i64) -> i64 {
    if !super::array_guard::valid_handle(handle) {
        return 0;
    }
    f()
}

pub(super) fn array_runtime_push_any(handle: i64, val_any: i64) -> i64 {
    // DirectArrayI64 handles are exact-lane storage, not public ArrayBox
    // handles. Their producer/lowering route passes raw i64 values; decoding
    // through the public value codec can mistake small positive integers for
    // unrelated host handles already present in the process.
    if let Some(new_len) = direct_array_i64_push_i64(handle, val_any) {
        return new_len;
    }
    with_runtime_handle_or_zero(handle, || array_slot_append_any(handle, val_any))
}

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

pub(super) fn array_runtime_string_len_at(handle: i64, idx: i64) -> i64 {
    array_string_len_by_index(handle, idx)
}

pub(super) fn array_runtime_string_indexof_at(handle: i64, idx: i64, needle_h: i64) -> i64 {
    array_string_indexof_by_index(handle, idx, needle_h)
}

pub(super) fn array_runtime_string_indexof_const_utf8_at(
    handle: i64,
    idx: i64,
    needle_ptr: *const i8,
    needle_len: i64,
) -> i64 {
    array_string_indexof_by_index_const_utf8(handle, idx, needle_ptr, needle_len)
}
