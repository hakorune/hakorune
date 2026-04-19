use super::super::array_guard::valid_handle_idx;
use super::super::value_codec::KernelTextSlot;
use super::array_string_slot_helpers::{
    array_text_owned_cell_demand, array_text_read_ref_demand,
    record_borrowed_alias_string_read_latest_fresh, with_compiler_const_utf8_ptr_len,
};
use crate::exports::string_view::clamp_i64_range;
use crate::observe;
use nyash_rust::box_trait::StringBox;
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
        let idx = idx as usize;
        arr.with_items_read(|items| {
            let Some(item) = items.get(idx) else {
                return 0;
            };
            let Some(source) = item.as_str_fast() else {
                return 0;
            };
            let mut out = String::with_capacity(source.len() + suffix.len());
            out.push_str(source);
            out.push_str(suffix);
            slot.replace_owned_string(out);
            1
        })
    })
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
    super::super::array_handle_cache::with_array_box(handle, |arr| {
        let idx = idx as usize;
        arr.with_items_write(|items| {
            if idx >= items.len() {
                return 0;
            }
            let item = &mut items[idx];
            if let Some(value) = item.as_any_mut().downcast_mut::<StringBox>() {
                append_const_suffix_to_string_box_value(&mut value.value, suffix);
                if observe_enabled {
                    observe::record_store_array_str_existing_slot();
                    observe::record_store_array_str_source_store();
                }
                return 1;
            }
            let Some(mut value) = item.as_str_fast().map(str::to_owned) else {
                return 0;
            };
            append_const_suffix_to_string_box_value(&mut value, suffix);
            *item = Box::new(StringBox::new(value));
            if observe_enabled {
                observe::record_store_array_str_existing_slot();
                observe::record_store_array_str_source_store();
            }
            1
        })
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
    let observe_enabled = observe::enabled();
    super::super::array_handle_cache::with_array_box(handle, |arr| {
        let idx = idx as usize;
        arr.with_items_read(|items| {
            let Some(item) = items.get(idx) else {
                return 0;
            };
            record_borrowed_alias_string_read_latest_fresh(observe_enabled, item.as_ref(), false);
            let Some(source) = item.as_str_fast() else {
                return 0;
            };
            slot.replace_owned_string(materialize_insert_const_mid_for_array_slot(
                source, middle, split,
            ));
            1
        })
    })
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
    super::super::array_handle_cache::with_array_box(handle, |arr| {
        let idx = idx as usize;
        arr.with_items_write(|items| {
            if idx >= items.len() {
                return 0;
            }
            let item = &mut items[idx];
            if let Some(value) = item.as_any_mut().downcast_mut::<StringBox>() {
                insert_const_mid_into_string_box_value(&mut value.value, middle, split);
                if observe_enabled {
                    observe::record_store_array_str_existing_slot();
                    observe::record_store_array_str_source_store();
                }
                return 1;
            }
            let Some(mut value) = item.as_str_fast().map(str::to_owned) else {
                return 0;
            };
            insert_const_mid_into_string_box_value(&mut value, middle, split);
            *item = Box::new(StringBox::new(value));
            if observe_enabled {
                observe::record_store_array_str_existing_slot();
                observe::record_store_array_str_source_store();
            }
            1
        })
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
    super::super::array_handle_cache::with_array_box(handle, |arr| {
        let idx = idx as usize;
        arr.with_items_write(|items| {
            if idx >= items.len() {
                return 0;
            }
            let item = &mut items[idx];
            if let Some(value) = item.as_any_mut().downcast_mut::<StringBox>() {
                let Some(next) = materialize_insert_const_mid_subrange_for_array_slot(
                    value.value.as_str(),
                    middle,
                    split,
                    start,
                    end,
                ) else {
                    return 0;
                };
                value.value = next;
                if observe_enabled {
                    observe::record_store_array_str_existing_slot();
                    observe::record_store_array_str_source_store();
                }
                return 1;
            }
            let Some(next) = item.as_str_fast().and_then(|source| {
                materialize_insert_const_mid_subrange_for_array_slot(
                    source, middle, split, start, end,
                )
            }) else {
                return 0;
            };
            *item = Box::new(StringBox::new(next));
            if observe_enabled {
                observe::record_store_array_str_existing_slot();
                observe::record_store_array_str_source_store();
            }
            1
        })
    })
    .unwrap_or(0)
}
