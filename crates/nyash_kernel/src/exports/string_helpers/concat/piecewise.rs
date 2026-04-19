use crate::exports::string_view::clamp_i64_range;
use crate::observe;
use crate::plugin::{
    freeze_owned_string_into_slot, publish_kernel_text_slot, with_kernel_text_slot_text,
    KernelTextSlot, KernelTextSlotState,
};
use nyash_rust::runtime::host_handles as handles;

use super::super::materialize::shared_empty_string_handle;
use super::const_adapter::{insert_const_mid_fallback, with_insert_middle_text};

#[inline(always)]
fn substring_owned_from_parts(parts: &[&str], start: usize, end: usize) -> Option<String> {
    if end <= start {
        return Some(String::new());
    }
    let mut out = String::with_capacity(end.saturating_sub(start));
    let mut cursor = 0usize;
    for part in parts {
        let part_len = part.len();
        let piece_start = cursor;
        let piece_end = cursor.saturating_add(part_len);
        let slice_start = start.max(piece_start);
        let slice_end = end.min(piece_end);
        if slice_start < slice_end {
            let local_start = slice_start.saturating_sub(piece_start);
            let local_end = slice_end.saturating_sub(piece_start);
            let slice = part.get(local_start..local_end)?;
            out.push_str(slice);
        }
        cursor = piece_end;
        if cursor >= end {
            break;
        }
    }
    Some(out)
}

#[inline(always)]
fn overlaps(start: usize, end: usize, piece_start: usize, piece_end: usize) -> bool {
    start < piece_end && piece_start < end
}

#[cfg_attr(feature = "perf-observe", inline(never))]
#[cfg_attr(not(feature = "perf-observe"), inline(always))]
fn materialize_piecewise_all_three(
    total_len: usize,
    prefix: &str,
    middle: &str,
    suffix: &str,
) -> String {
    let mut out = String::with_capacity(total_len);
    unsafe {
        let bytes = out.as_mut_vec();
        bytes.set_len(total_len);
        let mut cursor = 0usize;
        std::ptr::copy_nonoverlapping(
            prefix.as_ptr(),
            bytes.as_mut_ptr().add(cursor),
            prefix.len(),
        );
        cursor += prefix.len();
        std::ptr::copy_nonoverlapping(
            middle.as_ptr(),
            bytes.as_mut_ptr().add(cursor),
            middle.len(),
        );
        cursor += middle.len();
        std::ptr::copy_nonoverlapping(
            suffix.as_ptr(),
            bytes.as_mut_ptr().add(cursor),
            suffix.len(),
        );
    }
    out
}

#[inline(always)]
fn record_piecewise_shape(prefix_hit: bool, middle_hit: bool, suffix_hit: bool) {
    match (prefix_hit, middle_hit, suffix_hit) {
        (true, false, false) => observe::record_piecewise_subrange_prefix_only(),
        (false, true, false) => observe::record_piecewise_subrange_middle_only(),
        (false, false, true) => observe::record_piecewise_subrange_suffix_only(),
        (true, true, false) => observe::record_piecewise_subrange_prefix_middle(),
        (false, true, true) => observe::record_piecewise_subrange_middle_suffix(),
        (true, false, true) => observe::record_piecewise_subrange_prefix_suffix(),
        (true, true, true) => observe::record_piecewise_subrange_all_three(),
        (false, false, false) => {}
    }
}

#[inline(always)]
fn substring_borrowed_text_into_slot(
    out: &mut KernelTextSlot,
    text: &str,
    start: i64,
    end: i64,
) -> bool {
    let (slice_start, slice_end) = clamp_i64_range(text.len(), start, end);
    if slice_start == slice_end {
        out.clear();
        return true;
    }
    let Some(slice) = text.get(slice_start..slice_end) else {
        out.clear();
        return false;
    };
    freeze_owned_string_into_slot(out, slice.to_string());
    true
}

#[inline(always)]
fn with_kernel_text_slot_source_text<R>(
    slot: &KernelTextSlot,
    f: impl FnOnce(&str) -> R,
) -> Option<R> {
    match slot.state() {
        KernelTextSlotState::Empty => Some(f("")),
        KernelTextSlotState::OwnedBytes | KernelTextSlotState::DeferredConstSuffix => {
            with_kernel_text_slot_text(slot, |text| f(text.as_str()))
        }
        KernelTextSlotState::Published => None,
    }
}

#[inline(always)]
fn piecewise_subrange_borrowed_text_into_slot(
    out: &mut KernelTextSlot,
    source: &str,
    middle: &str,
    split: i64,
    start: i64,
    end: i64,
) -> Option<()> {
    let (split_start, _) = clamp_i64_range(source.len(), split, split);
    let prefix = source.get(..split_start).unwrap_or("");
    let suffix = source.get(split_start..).unwrap_or("");
    let prefix_len = prefix.len();
    let middle_len = middle.len();
    let suffix_len = suffix.len();
    let total_len = prefix
        .len()
        .saturating_add(middle_len)
        .saturating_add(suffix_len);
    let (slice_start, slice_end) = clamp_i64_range(total_len, start, end);
    if slice_start == slice_end {
        observe::record_piecewise_subrange_empty_return();
        out.clear();
        return Some(());
    }

    let middle_start = prefix_len;
    let middle_end = middle_start.saturating_add(middle_len);
    let suffix_start = middle_end;
    let suffix_end = suffix_start.saturating_add(suffix_len);
    let prefix_hit = overlaps(slice_start, slice_end, 0, prefix_len);
    let middle_hit = overlaps(slice_start, slice_end, middle_start, middle_end);
    let suffix_hit = overlaps(slice_start, slice_end, suffix_start, suffix_end);
    if prefix_hit && middle_hit && suffix_hit {
        let prefix_slice = prefix.get(slice_start..prefix_len)?;
        let suffix_slice = suffix.get(..slice_end.saturating_sub(suffix_start))?;
        record_piecewise_shape(true, true, true);
        freeze_owned_string_into_slot(
            out,
            materialize_piecewise_all_three(
                slice_end.saturating_sub(slice_start),
                prefix_slice,
                middle,
                suffix_slice,
            ),
        );
        return Some(());
    }
    let text = substring_owned_from_parts(&[prefix, middle, suffix], slice_start, slice_end)?;
    record_piecewise_shape(prefix_hit, middle_hit, suffix_hit);
    freeze_owned_string_into_slot(out, text);
    Some(())
}

#[cfg_attr(feature = "perf-observe", inline(never))]
#[cfg_attr(not(feature = "perf-observe"), inline(always))]
fn with_piecewise_borrowed_inputs<R>(
    source_h: i64,
    middle_ptr: *const i8,
    f: impl FnOnce(&str, &str) -> Option<R>,
) -> Option<R> {
    if source_h <= 0 {
        return None;
    }
    with_insert_middle_text(middle_ptr, |middle| {
        handles::with_text_read_session_ready(|session| {
            session.str_handle(source_h as u64, |source| f(source, middle))
        })
        .flatten()
        .flatten()
    })
}

#[cfg_attr(feature = "perf-observe", inline(never))]
#[cfg_attr(not(feature = "perf-observe"), inline(always))]
fn publish_kernel_text_slot_boundary(slot: &mut KernelTextSlot) -> Option<i64> {
    publish_kernel_text_slot(slot)
}

#[inline(always)]
pub(super) fn piecewise_subrange_hsiii_into_slot(
    out: &mut KernelTextSlot,
    source_h: i64,
    middle_ptr: *const i8,
    split: i64,
    start: i64,
    end: i64,
) -> bool {
    out.clear();
    with_piecewise_borrowed_inputs(source_h, middle_ptr, |source, middle| {
        observe::record_piecewise_subrange_single_session_hit();
        piecewise_subrange_borrowed_text_into_slot(out, source, middle, split, start, end)
    })
    .is_some()
}

#[inline(always)]
pub(super) fn piecewise_subrange_kernel_text_slot_into_slot(
    out: &mut KernelTextSlot,
    source: &KernelTextSlot,
    middle_ptr: *const i8,
    split: i64,
    start: i64,
    end: i64,
) -> bool {
    out.clear();
    with_insert_middle_text(middle_ptr, |middle| {
        with_kernel_text_slot_source_text(source, |text| {
            piecewise_subrange_borrowed_text_into_slot(out, text, middle, split, start, end)
        })
        .flatten()
    })
    .is_some()
}

#[inline(always)]
pub(super) fn substring_kernel_text_slot_into_slot(
    out: &mut KernelTextSlot,
    source: &KernelTextSlot,
    start: i64,
    end: i64,
) -> bool {
    out.clear();
    with_kernel_text_slot_source_text(source, |text| {
        substring_borrowed_text_into_slot(out, text, start, end)
    })
    .unwrap_or(false)
}

#[inline(always)]
pub(super) fn substring_kernel_text_slot_in_place(
    slot: &mut KernelTextSlot,
    start: i64,
    end: i64,
) -> bool {
    if slot.state() == KernelTextSlotState::Empty {
        slot.clear();
        return true;
    }
    let Some(bytes) = slot.take_materialized_owned_bytes() else {
        slot.clear();
        return false;
    };
    let text = bytes.as_str();
    let (slice_start, slice_end) = clamp_i64_range(text.len(), start, end);
    if slice_start == slice_end {
        slot.clear();
        return true;
    }
    if slice_start == 0 && slice_end == text.len() {
        slot.replace_owned_bytes(bytes);
        return true;
    }
    let Some(slice) = text.get(slice_start..slice_end) else {
        slot.clear();
        return false;
    };
    freeze_owned_string_into_slot(slot, slice.to_string());
    true
}

#[inline(always)]
pub(super) fn piecewise_subrange_hsiii_fallback(
    source_h: i64,
    middle_ptr: *const i8,
    split: i64,
    start: i64,
    end: i64,
) -> i64 {
    observe::record_piecewise_subrange_enter();
    let mut slot = KernelTextSlot::empty();
    // Phase-137x keeps the carrier local to this executor first.
    // The next slot-transport card may thread it across same-corridor consumers.
    if piecewise_subrange_hsiii_into_slot(&mut slot, source_h, middle_ptr, split, start, end) {
        return publish_kernel_text_slot_boundary(&mut slot)
            .unwrap_or_else(shared_empty_string_handle);
    }
    with_insert_middle_text(middle_ptr, |_middle| {
        observe::record_piecewise_subrange_fallback_insert();
        let inserted_h = insert_const_mid_fallback(source_h, middle_ptr, split);
        super::super::string_substring_hii_export_impl(inserted_h, start, end)
    })
}
