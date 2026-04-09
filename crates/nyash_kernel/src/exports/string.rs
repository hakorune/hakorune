#[path = "string_helpers.rs"]
mod string_helpers;

use self::string_helpers::*;
pub(crate) use self::string_helpers::{
    string_is_empty_from_handle, string_len_from_handle, to_owned_string_handle_arg,
};

// Thin ABI facade only.
// String semantic ownership should live above this layer; keep exports here as
// stable entrypoints into Rust substrate/sink glue, not as a policy owner.

// String.len_h(handle) -> i64
#[export_name = "nyash.string.len_h"]
pub extern "C" fn nyash_string_len_h(handle: i64) -> i64 {
    string_len_export_impl(handle)
}

// FAST-path helper: compute string length from raw pointer (i8*) with mode (reserved)
// Exported as both legacy name (nyash.string.length_si) and neutral name (nyrt_string_length)
#[export_name = "nyrt_string_length"]
pub extern "C" fn nyrt_string_length(ptr: *const i8, mode: i64) -> i64 {
    string_length_from_ptr(ptr, mode)
}

// String.charCodeAt_h(handle, idx) -> i64 (byte-based; -1 if OOB)
#[export_name = "nyash.string.charCodeAt_h"]
pub extern "C" fn nyash_string_charcode_at_h_export(handle: i64, idx: i64) -> i64 {
    string_charcode_at_export_impl(handle, idx)
}

// String.concat_hh(lhs_h, rhs_h) -> handle
#[export_name = "nyash.string.concat_hh"]
pub extern "C" fn nyash_string_concat_hh_export(a_h: i64, b_h: i64) -> i64 {
    string_concat_hh_export_impl(a_h, b_h)
}

// String.concat_hs(lhs_h, const_suffix_ptr) -> handle
#[export_name = "nyash.string.concat_hs"]
pub extern "C" fn nyash_string_concat_hs_export(a_h: i64, suffix_ptr: *const i8) -> i64 {
    string_concat_hs_export_impl(a_h, suffix_ptr)
}

// String.insert_hsi(source_h, const_middle_ptr, split_i64) -> handle
#[export_name = "nyash.string.insert_hsi"]
pub extern "C" fn nyash_string_insert_hsi_export(
    source_h: i64,
    middle_ptr: *const i8,
    split: i64,
) -> i64 {
    string_insert_hsi_export_impl(source_h, middle_ptr, split)
}

// String.substring_concat_hhii(lhs_h, rhs_h, start_i64, end_i64) -> handle
#[export_name = "nyash.string.substring_concat_hhii"]
pub extern "C" fn nyash_string_substring_concat_hhii_export(
    a_h: i64,
    b_h: i64,
    start: i64,
    end: i64,
) -> i64 {
    string_substring_concat_hhii_export_impl(a_h, b_h, start, end)
}

// String.substring_concat3_hhhii(a_h, b_h, c_h, start_i64, end_i64) -> handle
#[export_name = "nyash.string.substring_concat3_hhhii"]
pub extern "C" fn nyash_string_substring_concat3_hhhii_export(
    a_h: i64,
    b_h: i64,
    c_h: i64,
    start: i64,
    end: i64,
) -> i64 {
    string_substring_concat3_hhhii_export_impl(a_h, b_h, c_h, start, end)
}

// String.concat3_hhh(a_h, b_h, c_h) -> handle
#[export_name = "nyash.string.concat3_hhh"]
pub extern "C" fn nyash_string_concat3_hhh_export(a_h: i64, b_h: i64, c_h: i64) -> i64 {
    string_concat3_hhh_export_impl(a_h, b_h, c_h)
}

// String.eq_hh(lhs_h, rhs_h) -> i64 (0/1)
#[export_name = "nyash.string.eq_hh"]
pub extern "C" fn nyash_string_eq_hh_export(a_h: i64, b_h: i64) -> i64 {
    string_eq_hh_export_impl(a_h, b_h)
}

// String.substring_hii(handle, start, end) -> handle
#[export_name = "nyash.string.substring_hii"]
pub extern "C" fn nyash_string_substring_hii_export(h: i64, start: i64, end: i64) -> i64 {
    string_substring_hii_export_impl(h, start, end)
}

// String.substring_len_hii(handle, start, end) -> i64
// Internal borrowed-corridor helper for AOT lowering. This computes the
// resulting substring length without forcing view publication/materialization.
#[export_name = "nyash.string.substring_len_hii"]
pub extern "C" fn nyash_string_substring_len_hii_export(h: i64, start: i64, end: i64) -> i64 {
    string_substring_len_hii_export_impl(h, start, end)
}

// String.indexOf_hh(haystack_h, needle_h) -> i64
#[export_name = "nyash.string.indexOf_hh"]
pub extern "C" fn nyash_string_indexof_hh_export(h: i64, n: i64) -> i64 {
    string_indexof_hh_export_impl(h, n)
}

// String.lastIndexOf_hh(haystack_h, needle_h) -> i64
#[export_name = "nyash.string.lastIndexOf_hh"]
pub extern "C" fn nyash_string_lastindexof_hh_export(h: i64, n: i64) -> i64 {
    string_lastindexof_hh_export_impl(h, n)
}

// String.lt_hh(lhs_h, rhs_h) -> i64 (0/1)
#[export_name = "nyash.string.lt_hh"]
pub extern "C" fn nyash_string_lt_hh_export(a_h: i64, b_h: i64) -> i64 {
    string_lt_hh_export_impl(a_h, b_h)
}

// Construct StringBox from two u64 words (little-endian) + length (<=16) and return handle
// export: nyash.string.from_u64x2(lo, hi, len) -> i64
#[export_name = "nyash.string.from_u64x2"]
pub extern "C" fn nyash_string_from_u64x2_export(lo: i64, hi: i64, len: i64) -> i64 {
    string_from_u64x2_export_impl(lo, hi, len)
}
