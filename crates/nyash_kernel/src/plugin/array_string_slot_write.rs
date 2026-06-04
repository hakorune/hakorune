use super::super::array_handle_cache::with_array_box;
use super::super::array_text_write_txn::{
    with_array_text_slot_update, with_array_text_slot_update_resident_first,
    ArrayTextWriteTxnOutcome,
};
use super::super::handle_cache::valid_handle_idx;
use super::super::value_codec::{KernelTextSlot, OwnedText};
use super::array_string_slot_helpers::{
    array_text_owned_cell_demand, array_text_read_ref_demand, with_compiler_const_utf8_ptr_len,
    with_cstr_utf8_ptr,
};
use crate::exports::string_view::clamp_i64_range;
use crate::observe;

pub(in super::super) fn array_string_concat_const_suffix_by_index_store_same_slot_len(
    handle: i64,
    idx: i64,
    suffix_ptr: *const i8,
    suffix_len: i64,
) -> i64 {
    if !valid_handle_idx(handle, idx) || suffix_ptr.is_null() {
        return 0;
    }
    with_compiler_const_utf8_ptr_len(suffix_ptr, suffix_len, |suffix| {
        array_string_concat_const_suffix_by_index_store_same_slot_text(handle, idx, suffix)
    })
    .unwrap_or(0)
}

#[inline(always)]
pub(in super::super) fn array_string_concat_const_suffix_by_index_store_same_slot_text(
    handle: i64,
    idx: i64,
    suffix: &str,
) -> i64 {
    let _read_demand = array_text_read_ref_demand();
    let _output_demand = array_text_owned_cell_demand();
    let observe_enabled = observe::enabled();
    observe::record_store_array_str_enter();
    with_array_text_slot_update(handle, idx, |value| {
        if !suffix.is_empty() {
            value.push_str(suffix);
        }
        if observe_enabled {
            observe::record_store_array_str_existing_slot();
            observe::record_store_array_str_source_store();
        }
        1
    })
    .unwrap_or(0)
}

pub(crate) fn with_array_kernel_text_slot_str<R>(
    slot: *mut KernelTextSlot,
    handle: i64,
    idx: i64,
    input_ptr: *const i8,
    f: impl FnOnce(&mut KernelTextSlot, &str) -> R,
) -> Option<R> {
    with_cstr_utf8_ptr(input_ptr, |input| {
        let Some(slot) = (unsafe { slot.as_mut() }) else {
            return None;
        };
        slot.clear();
        if !valid_handle_idx(handle, idx) {
            return None;
        }
        let _read_demand = array_text_read_ref_demand();
        let _output_demand = array_text_owned_cell_demand();
        Some(f(slot, input))
    })
    .flatten()
}

pub(in super::super) fn array_kernel_slot_concat_his(
    slot: *mut KernelTextSlot,
    handle: i64,
    idx: i64,
    suffix_ptr: *const i8,
) -> i64 {
    with_array_kernel_text_slot_str(slot, handle, idx, suffix_ptr, |slot, suffix| {
        with_array_box(handle, |arr| {
            arr.slot_with_text_raw(idx, |source| {
                let mut out = String::with_capacity(source.len() + suffix.len());
                out.push_str(source);
                out.push_str(suffix);
                slot.replace_owned_bytes(OwnedText::from_string(out));
                1
            })
        })
        .flatten()
        .unwrap_or(0)
    })
    .unwrap_or(0)
}

pub(in super::super) fn array_kernel_slot_insert_hisi(
    slot: *mut KernelTextSlot,
    handle: i64,
    idx: i64,
    middle_ptr: *const i8,
    split: i64,
) -> i64 {
    with_array_kernel_text_slot_str(slot, handle, idx, middle_ptr, |slot, middle| {
        with_array_box(handle, |arr| {
            arr.slot_with_text_raw(idx, |source| {
                let out = if source.is_empty() {
                    middle.to_owned()
                } else if middle.is_empty() {
                    source.to_owned()
                } else {
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
                        std::ptr::copy_nonoverlapping(
                            prefix.as_ptr(),
                            buf.as_mut_ptr().add(cursor),
                            prefix.len(),
                        );
                        cursor += prefix.len();
                        std::ptr::copy_nonoverlapping(
                            middle.as_ptr(),
                            buf.as_mut_ptr().add(cursor),
                            middle.len(),
                        );
                        cursor += middle.len();
                        std::ptr::copy_nonoverlapping(
                            suffix.as_ptr(),
                            buf.as_mut_ptr().add(cursor),
                            suffix.len(),
                        );
                    }
                    out
                };
                slot.replace_owned_bytes(OwnedText::from_string(out));
                1
            })
        })
        .flatten()
        .unwrap_or(0)
    })
    .unwrap_or(0)
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
    let piece_start = 0usize;
    let piece_end = piece_start.saturating_add(prefix.len());
    let start = slice_start.max(piece_start);
    let end = slice_end.min(piece_end);
    if start < end {
        out.push_str(prefix.get(start - piece_start..end - piece_start)?);
    }
    let piece_start = prefix_len;
    let piece_end = piece_start.saturating_add(middle_len);
    let start = slice_start.max(piece_start);
    let end = slice_end.min(piece_end);
    if start < end {
        out.push_str(middle.get(start - piece_start..end - piece_start)?);
    }
    let piece_start = prefix_len.saturating_add(middle_len);
    let piece_end = piece_start.saturating_add(suffix.len());
    let start = slice_start.max(piece_start);
    let end = slice_end.min(piece_end);
    if start < end {
        out.push_str(suffix.get(start - piece_start..end - piece_start)?);
    }
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
        array_string_insert_const_mid_by_index_store_same_slot_text(handle, idx, middle, split)
    })
    .unwrap_or(0)
}

#[inline(always)]
pub(in super::super) fn array_string_insert_const_mid_by_index_store_same_slot_text(
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
        if value.is_empty() {
            value.push_str(middle);
        } else if !middle.is_empty() {
            let split = split.clamp(0, value.len() as i64) as usize;
            if value.is_char_boundary(split) {
                value.insert_str(split, middle);
            } else {
                let source = value.as_str();
                if source.is_empty() {
                    *value = middle.to_owned();
                } else {
                    let prefix = source.get(0..split).unwrap_or("");
                    let suffix = source.get(split..).unwrap_or("");
                    let total = prefix.len() + middle.len() + suffix.len();
                    let mut out = String::with_capacity(total);
                    // Equivalent to `(source[..split] + middle + source[split..])[1..source_len + 1]`.
                    unsafe {
                        let buf = out.as_mut_vec();
                        buf.set_len(total);
                        let mut cursor = 0usize;
                        std::ptr::copy_nonoverlapping(
                            prefix.as_ptr(),
                            buf.as_mut_ptr().add(cursor),
                            prefix.len(),
                        );
                        cursor += prefix.len();
                        std::ptr::copy_nonoverlapping(
                            middle.as_ptr(),
                            buf.as_mut_ptr().add(cursor),
                            middle.len(),
                        );
                        cursor += middle.len();
                        std::ptr::copy_nonoverlapping(
                            suffix.as_ptr(),
                            buf.as_mut_ptr().add(cursor),
                            suffix.len(),
                        );
                    }
                    *value = out;
                }
            }
        }
        if observe_enabled {
            observe::record_store_array_str_existing_slot();
            observe::record_store_array_str_source_store();
        }
        1
    })
    .unwrap_or(0)
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
        array_string_insert_const_mid_subrange_by_index_store_same_slot_text(
            handle, idx, middle, split, start, end,
        )
    })
    .unwrap_or(0)
}

#[inline(always)]
pub(in super::super) fn array_string_insert_const_mid_subrange_by_index_store_same_slot_text(
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
        let _read_demand = array_text_read_ref_demand();
        let _output_demand = array_text_owned_cell_demand();
        let observe_enabled = observe::enabled();
        observe::record_store_array_str_enter();
        let outcome = with_array_text_slot_update_resident_first(handle, idx, |value| {
            let source_len = value.len();
            if source_len != 0 && middle.len() == 2 {
                let split_start = source_len / 2;
                if split_start != 0
                    && split_start < source_len
                    && value.is_char_boundary(1)
                    && value.is_char_boundary(split_start)
                    && value.is_char_boundary(source_len - 1)
                    && middle.is_char_boundary(1)
                {
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
                    if observe_enabled {
                        observe::record_store_array_str_existing_slot();
                        observe::record_store_array_str_source_store();
                    }
                    return value.len() as i64;
                }
            }
            let source_len = value.len();
            let split = (source_len / 2) as i64;
            let start = 1;
            let end = source_len as i64 + 1;
            if !try_update_insert_const_mid_subrange_same_len_in_place(
                value, middle, split, start, end,
            ) {
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
        with_array_box(handle, |arr| {
            arr.slot_text_region_update_sum_raw(loop_bound, row_modulus, |value| {
                let observe_enabled = observe::enabled();
                let source_len = value.len();
                if source_len != 0 && middle.len() == 2 {
                    let split_start = source_len / 2;
                    if split_start != 0
                        && split_start < source_len
                        && value.is_char_boundary(1)
                        && value.is_char_boundary(split_start)
                        && value.is_char_boundary(source_len - 1)
                        && middle.is_char_boundary(1)
                    {
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
                            std::ptr::copy_nonoverlapping(
                                middle.as_ptr(),
                                ptr.add(split_start - 1),
                                2,
                            );
                        }
                        if observe_enabled {
                            observe::record_store_array_str_existing_slot();
                            observe::record_store_array_str_source_store();
                        }
                        return Some(value.len() as i64);
                    }
                }
                let source_len = value.len();
                let split = (source_len / 2) as i64;
                let start = 1;
                let end = source_len as i64 + 1;
                if !try_update_insert_const_mid_subrange_same_len_in_place(
                    value, middle, split, start, end,
                ) {
                    let Some(next) = materialize_insert_const_mid_subrange_for_array_slot(
                        value.as_str(),
                        middle,
                        split,
                        start,
                        end,
                    ) else {
                        return None;
                    };
                    *value = next;
                }
                if observe_enabled {
                    observe::record_store_array_str_existing_slot();
                    observe::record_store_array_str_source_store();
                }
                Some(value.len() as i64)
            })
        })
        .flatten()
        .unwrap_or(0)
    })
    .unwrap_or(0)
}
