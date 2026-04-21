use super::super::array_guard::valid_handle_idx;
use super::super::array_text_write_txn::{
    with_array_text_slot_update, with_array_text_slot_update_resident_first,
    ArrayTextWriteTxnOutcome,
};
use super::super::value_codec::KernelTextSlot;
use super::array_string_slot_helpers::{
    array_text_owned_cell_demand, array_text_read_ref_demand, with_compiler_const_utf8_ptr_len,
};
use crate::exports::string_view::clamp_i64_range;
use crate::observe;
use std::ffi::CStr;

pub(in super::super) fn array_string_concat_const_suffix_by_index_into_slot(
    slot: &mut KernelTextSlot,
    handle: i64,
    idx: i64,
    suffix_ptr: *const i8,
) -> i64 {
    slot.clear();
    if !valid_handle_idx(handle, idx) || suffix_ptr.is_null() {
        return 0;
    }
    let Ok(suffix) = (unsafe { CStr::from_ptr(suffix_ptr) }).to_str() else {
        return 0;
    };
    array_string_concat_const_suffix_by_index_into_slot_str(slot, handle, idx, suffix)
}

fn array_string_concat_const_suffix_by_index_into_slot_str(
    slot: &mut KernelTextSlot,
    handle: i64,
    idx: i64,
    suffix: &str,
) -> i64 {
    let _read_demand = array_text_read_ref_demand();
    let _output_demand = array_text_owned_cell_demand();
    slot.clear();
    super::super::array_handle_cache::with_array_box(handle, |arr| {
        arr.slot_with_text_raw(idx, |source| {
            let mut out = String::with_capacity(source.len() + suffix.len());
            out.push_str(source);
            out.push_str(suffix);
            slot.replace_owned_string(out);
            1
        })
    })
    .flatten()
    .unwrap_or(0)
}

#[inline(always)]
fn append_const_suffix_to_string_box_value(value: &mut String, suffix: &str) {
    if suffix.is_empty() {
        return;
    }
    value.push_str(suffix);
}

pub(in super::super) fn array_string_concat_const_suffix_by_index_store_same_slot(
    handle: i64,
    idx: i64,
    suffix_ptr: *const i8,
) -> i64 {
    if !valid_handle_idx(handle, idx) || suffix_ptr.is_null() {
        return 0;
    }
    let Ok(suffix) = (unsafe { CStr::from_ptr(suffix_ptr) }).to_str() else {
        return 0;
    };
    array_string_concat_const_suffix_by_index_store_same_slot_str(handle, idx, suffix)
}

pub(in super::super) fn array_string_concat_const_suffix_by_index_store_same_slot_len(
    handle: i64,
    idx: i64,
    suffix_ptr: *const i8,
    suffix_len: i64,
) -> i64 {
    if !valid_handle_idx(handle, idx) {
        return 0;
    }
    with_compiler_const_utf8_ptr_len(suffix_ptr, suffix_len, |suffix| {
        array_string_concat_const_suffix_by_index_store_same_slot_str(handle, idx, suffix)
    })
    .unwrap_or(0)
}

fn array_string_concat_const_suffix_by_index_store_same_slot_str(
    handle: i64,
    idx: i64,
    suffix: &str,
) -> i64 {
    let _read_demand = array_text_read_ref_demand();
    let _output_demand = array_text_owned_cell_demand();
    let observe_enabled = observe::enabled();
    observe::record_store_array_str_enter();
    with_array_text_slot_update(handle, idx, |value| {
        append_const_suffix_to_string_box_value(value, suffix);
        if observe_enabled {
            observe::record_store_array_str_existing_slot();
            observe::record_store_array_str_source_store();
        }
        1
    })
    .unwrap_or(0)
}

#[inline(always)]
fn materialize_insert_const_mid_for_array_slot(source: &str, middle: &str, split: i64) -> String {
    if source.is_empty() {
        return middle.to_owned();
    }
    if middle.is_empty() {
        return source.to_owned();
    }
    let split = split.clamp(0, source.len() as i64) as usize;
    let prefix = source.get(0..split).unwrap_or("");
    let suffix = source.get(split..).unwrap_or("");
    let total = prefix.len() + middle.len() + suffix.len();
    let mut out = String::with_capacity(total);
    // Equivalent to `(source[..split] + middle + source[split..])[1..source_len + 1]`.
    unsafe {
        let buf = out.as_mut_vec();
        buf.set_len(total);
        let mut cursor = 0usize;
        std::ptr::copy_nonoverlapping(prefix.as_ptr(), buf.as_mut_ptr().add(cursor), prefix.len());
        cursor += prefix.len();
        std::ptr::copy_nonoverlapping(middle.as_ptr(), buf.as_mut_ptr().add(cursor), middle.len());
        cursor += middle.len();
        std::ptr::copy_nonoverlapping(suffix.as_ptr(), buf.as_mut_ptr().add(cursor), suffix.len());
    }
    out
}

#[inline(always)]
fn push_piece_overlap(
    out: &mut String,
    piece: &str,
    piece_start: usize,
    slice_start: usize,
    slice_end: usize,
) -> Option<()> {
    let piece_end = piece_start.saturating_add(piece.len());
    let start = slice_start.max(piece_start);
    let end = slice_end.min(piece_end);
    if start < end {
        out.push_str(piece.get(start - piece_start..end - piece_start)?);
    }
    Some(())
}

#[inline(always)]
fn materialize_insert_const_mid_subrange_for_array_slot(
    source: &str,
    middle: &str,
    split: i64,
    start: i64,
    end: i64,
) -> Option<String> {
    let (split_start, _) = clamp_i64_range(source.len(), split, split);
    let prefix = source.get(..split_start).unwrap_or("");
    let suffix = source.get(split_start..).unwrap_or("");
    let prefix_len = prefix.len();
    let middle_len = middle.len();
    let total_len = prefix_len
        .saturating_add(middle_len)
        .saturating_add(suffix.len());
    let (slice_start, slice_end) = clamp_i64_range(total_len, start, end);
    if slice_start == slice_end {
        return Some(String::new());
    }
    let mut out = String::with_capacity(slice_end.saturating_sub(slice_start));
    push_piece_overlap(&mut out, prefix, 0, slice_start, slice_end)?;
    push_piece_overlap(&mut out, middle, prefix_len, slice_start, slice_end)?;
    push_piece_overlap(
        &mut out,
        suffix,
        prefix_len.saturating_add(middle_len),
        slice_start,
        slice_end,
    )?;
    Some(out)
}

#[inline(always)]
fn try_update_insert_const_mid_subrange_same_len_in_place(
    value: &mut String,
    middle: &str,
    split: i64,
    start: i64,
    end: i64,
) -> bool {
    let source_len = value.len();
    let middle_len = middle.len();
    if source_len == 0 || middle_len != 2 {
        return false;
    }
    let (split_start, _) = clamp_i64_range(source_len, split, split);
    let total_len = source_len.saturating_add(middle_len);
    let (slice_start, slice_end) = clamp_i64_range(total_len, start, end);
    if slice_start != 1 || slice_end != source_len + 1 {
        return false;
    }
    if split_start == 0 || split_start >= source_len {
        return false;
    }
    if !value.is_char_boundary(slice_start)
        || !value.is_char_boundary(split_start)
        || !value.is_char_boundary(source_len - 1)
        || !middle.is_char_boundary(1)
    {
        return false;
    }
    unsafe {
        let bytes = value.as_mut_vec();
        let ptr = bytes.as_mut_ptr();
        let prefix_shift_len = split_start - 1;
        let suffix_shift_len = source_len - split_start - 1;
        if suffix_shift_len != 0 {
            std::ptr::copy(
                ptr.add(split_start),
                ptr.add(split_start + middle_len - 1),
                suffix_shift_len,
            );
        }
        if prefix_shift_len != 0 {
            std::ptr::copy(ptr.add(1), ptr, prefix_shift_len);
        }
        std::ptr::copy_nonoverlapping(middle.as_ptr(), ptr.add(split_start - 1), middle_len);
    }
    true
}

#[inline(always)]
fn try_update_insert_const_mid_subrange_len_fast(value: &mut String, middle: &str) -> bool {
    let source_len = value.len();
    if source_len == 0 || middle.len() != 2 {
        return false;
    }
    let split_start = source_len / 2;
    if split_start == 0 || split_start >= source_len {
        return false;
    }
    if !value.is_char_boundary(1)
        || !value.is_char_boundary(split_start)
        || !value.is_char_boundary(source_len - 1)
        || !middle.is_char_boundary(1)
    {
        return false;
    }
    unsafe {
        let bytes = value.as_mut_vec();
        let ptr = bytes.as_mut_ptr();
        let prefix_shift_len = split_start - 1;
        let suffix_shift_len = source_len - split_start - 1;
        if suffix_shift_len != 0 {
            std::ptr::copy(
                ptr.add(split_start),
                ptr.add(split_start + 1),
                suffix_shift_len,
            );
        }
        if prefix_shift_len != 0 {
            std::ptr::copy(ptr.add(1), ptr, prefix_shift_len);
        }
        std::ptr::copy_nonoverlapping(middle.as_ptr(), ptr.add(split_start - 1), 2);
    }
    true
}

#[inline(always)]
fn insert_const_mid_into_string_box_value(value: &mut String, middle: &str, split: i64) {
    if value.is_empty() {
        value.push_str(middle);
        return;
    }
    if middle.is_empty() {
        return;
    }
    let split = split.clamp(0, value.len() as i64) as usize;
    if value.is_char_boundary(split) {
        value.insert_str(split, middle);
        return;
    }
    *value = materialize_insert_const_mid_for_array_slot(value.as_str(), middle, split as i64);
}

pub(in super::super) fn array_string_insert_const_mid_by_index_into_slot(
    slot: &mut KernelTextSlot,
    handle: i64,
    idx: i64,
    middle_ptr: *const i8,
    split: i64,
) -> i64 {
    slot.clear();
    if !valid_handle_idx(handle, idx) || middle_ptr.is_null() {
        return 0;
    }
    let Ok(middle) = (unsafe { CStr::from_ptr(middle_ptr) }).to_str() else {
        return 0;
    };
    array_string_insert_const_mid_by_index_into_slot_str(slot, handle, idx, middle, split)
}

fn array_string_insert_const_mid_by_index_into_slot_str(
    slot: &mut KernelTextSlot,
    handle: i64,
    idx: i64,
    middle: &str,
    split: i64,
) -> i64 {
    let _read_demand = array_text_read_ref_demand();
    let _output_demand = array_text_owned_cell_demand();
    slot.clear();
    super::super::array_handle_cache::with_array_box(handle, |arr| {
        arr.slot_with_text_raw(idx, |source| {
            slot.replace_owned_string(materialize_insert_const_mid_for_array_slot(
                source, middle, split,
            ));
            1
        })
    })
    .flatten()
    .unwrap_or(0)
}

pub(in super::super) fn array_string_insert_const_mid_by_index_store_same_slot(
    handle: i64,
    idx: i64,
    middle_ptr: *const i8,
    split: i64,
) -> i64 {
    if !valid_handle_idx(handle, idx) || middle_ptr.is_null() {
        return 0;
    }
    let Ok(middle) = (unsafe { CStr::from_ptr(middle_ptr) }).to_str() else {
        return 0;
    };
    array_string_insert_const_mid_by_index_store_same_slot_str(handle, idx, middle, split)
}

pub(in super::super) fn array_string_insert_const_mid_by_index_store_same_slot_len(
    handle: i64,
    idx: i64,
    middle_ptr: *const i8,
    middle_len: i64,
    split: i64,
) -> i64 {
    if !valid_handle_idx(handle, idx) {
        return 0;
    }
    with_compiler_const_utf8_ptr_len(middle_ptr, middle_len, |middle| {
        array_string_insert_const_mid_by_index_store_same_slot_str(handle, idx, middle, split)
    })
    .unwrap_or(0)
}

fn array_string_insert_const_mid_by_index_store_same_slot_str(
    handle: i64,
    idx: i64,
    middle: &str,
    split: i64,
) -> i64 {
    let _read_demand = array_text_read_ref_demand();
    let _output_demand = array_text_owned_cell_demand();
    let observe_enabled = observe::enabled();
    observe::record_store_array_str_enter();
    with_array_text_slot_update(handle, idx, |value| {
        insert_const_mid_into_string_box_value(value, middle, split);
        if observe_enabled {
            observe::record_store_array_str_existing_slot();
            observe::record_store_array_str_source_store();
        }
        1
    })
    .unwrap_or(0)
}

pub(in super::super) fn array_string_insert_const_mid_subrange_by_index_store_same_slot(
    handle: i64,
    idx: i64,
    middle_ptr: *const i8,
    split: i64,
    start: i64,
    end: i64,
) -> i64 {
    if !valid_handle_idx(handle, idx) || middle_ptr.is_null() {
        return 0;
    }
    let Ok(middle) = (unsafe { CStr::from_ptr(middle_ptr) }).to_str() else {
        return 0;
    };
    array_string_insert_const_mid_subrange_by_index_store_same_slot_str(
        handle, idx, middle, split, start, end,
    )
}

pub(in super::super) fn array_string_insert_const_mid_subrange_by_index_store_same_slot_len(
    handle: i64,
    idx: i64,
    middle_ptr: *const i8,
    middle_len: i64,
    split: i64,
    start: i64,
    end: i64,
) -> i64 {
    if !valid_handle_idx(handle, idx) {
        return 0;
    }
    with_compiler_const_utf8_ptr_len(middle_ptr, middle_len, |middle| {
        array_string_insert_const_mid_subrange_by_index_store_same_slot_str(
            handle, idx, middle, split, start, end,
        )
    })
    .unwrap_or(0)
}

pub(in super::super) fn array_string_insert_const_mid_subrange_len_by_index_store_same_slot_len(
    handle: i64,
    idx: i64,
    middle_ptr: *const i8,
    middle_len: i64,
) -> i64 {
    if !valid_handle_idx(handle, idx) {
        return 0;
    }
    with_compiler_const_utf8_ptr_len(middle_ptr, middle_len, |middle| {
        array_string_insert_const_mid_subrange_len_by_index_store_same_slot_str(handle, idx, middle)
    })
    .unwrap_or(0)
}

pub(in super::super) fn array_string_insert_const_mid_subrange_len_region_store_len(
    handle: i64,
    loop_bound: i64,
    row_modulus: i64,
    middle_ptr: *const i8,
    middle_len: i64,
) -> i64 {
    if handle <= 0 || loop_bound < 0 || row_modulus <= 0 {
        return 0;
    }
    with_compiler_const_utf8_ptr_len(middle_ptr, middle_len, |middle| {
        super::super::array_handle_cache::with_array_box(handle, |arr| {
            arr.slot_text_region_update_sum_raw(loop_bound, row_modulus, |value| {
                Some(update_insert_const_mid_subrange_len_value(
                    value,
                    middle,
                    observe::enabled(),
                ))
            })
        })
        .flatten()
        .unwrap_or(0)
    })
    .unwrap_or(0)
}

fn array_string_insert_const_mid_subrange_by_index_store_same_slot_str(
    handle: i64,
    idx: i64,
    middle: &str,
    split: i64,
    start: i64,
    end: i64,
) -> i64 {
    let _read_demand = array_text_read_ref_demand();
    let _output_demand = array_text_owned_cell_demand();
    let observe_enabled = observe::enabled();
    observe::record_store_array_str_enter();
    with_array_text_slot_update(handle, idx, |value| {
        if try_update_insert_const_mid_subrange_same_len_in_place(value, middle, split, start, end)
        {
            if observe_enabled {
                observe::record_store_array_str_existing_slot();
                observe::record_store_array_str_source_store();
            }
            return 1;
        }
        let Some(next) = materialize_insert_const_mid_subrange_for_array_slot(
            value.as_str(),
            middle,
            split,
            start,
            end,
        ) else {
            return 0;
        };
        *value = next;
        if observe_enabled {
            observe::record_store_array_str_existing_slot();
            observe::record_store_array_str_source_store();
        }
        1
    })
    .unwrap_or(0)
}

fn array_string_insert_const_mid_subrange_len_by_index_store_same_slot_str(
    handle: i64,
    idx: i64,
    middle: &str,
) -> i64 {
    let _read_demand = array_text_read_ref_demand();
    let _output_demand = array_text_owned_cell_demand();
    let observe_enabled = observe::enabled();
    observe::record_store_array_str_enter();
    let outcome = with_array_text_slot_update_resident_first(handle, idx, |value| {
        update_insert_const_mid_subrange_len_value(value, middle, observe_enabled)
    });
    match outcome {
        Some(ArrayTextWriteTxnOutcome::Resident(out)) => {
            if observe_enabled {
                observe::record_store_array_str_update_text_resident_hit();
            }
            out
        }
        Some(ArrayTextWriteTxnOutcome::Fallback(out)) => {
            if observe_enabled {
                observe::record_store_array_str_update_text_resident_miss();
                observe::record_store_array_str_update_text_fallback_hit();
            }
            out
        }
        None => {
            if observe_enabled {
                observe::record_store_array_str_update_text_resident_miss();
                observe::record_store_array_str_update_text_fallback_miss();
            }
            0
        }
    }
}

#[inline(always)]
fn update_insert_const_mid_subrange_len_value(
    value: &mut String,
    middle: &str,
    observe_enabled: bool,
) -> i64 {
    if try_update_insert_const_mid_subrange_len_fast(value, middle) {
        if observe_enabled {
            observe::record_store_array_str_existing_slot();
            observe::record_store_array_str_source_store();
        }
        return value.len() as i64;
    }
    update_insert_const_mid_subrange_len_value_slow(value, middle, observe_enabled)
}

#[cold]
#[inline(never)]
fn update_insert_const_mid_subrange_len_value_slow(
    value: &mut String,
    middle: &str,
    observe_enabled: bool,
) -> i64 {
    let source_len = value.len();
    let split = (source_len / 2) as i64;
    let start = 1;
    let end = source_len as i64 + 1;
    if !try_update_insert_const_mid_subrange_same_len_in_place(value, middle, split, start, end) {
        let Some(next) = materialize_insert_const_mid_subrange_for_array_slot(
            value.as_str(),
            middle,
            split,
            start,
            end,
        ) else {
            return 0;
        };
        *value = next;
    }
    if observe_enabled {
        observe::record_store_array_str_existing_slot();
        observe::record_store_array_str_source_store();
    }
    value.len() as i64
}
