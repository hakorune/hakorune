use crate::exports::string_view::clamp_i64_range;
use crate::observe;
use crate::plugin::{
    freeze_owned_string_into_slot, publish_kernel_text_slot, with_kernel_text_slot_text,
    KernelTextSlot, KernelTextSlotState, TextRef,
};
use nyash_rust::runtime::host_handles as handles;

use super::super::materialize::shared_empty_string_handle;
use super::common::substring_owned_from_parts;
use super::const_adapter::{insert_const_mid_fallback, with_insert_middle_text};

#[inline(always)]
fn overlaps(start: usize, end: usize, piece_start: usize, piece_end: usize) -> bool {
    start < piece_end && piece_start < end
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
pub(crate) fn piecewise_subrange_hsiii_into_slot(
    out: &mut KernelTextSlot,
    source_h: i64,
    middle_ptr: *const i8,
    split: i64,
    start: i64,
    end: i64,
) -> bool {
    out.clear();
    if source_h <= 0 {
        return false;
    }
    with_insert_middle_text(middle_ptr, |middle| {
        handles::with_text_read_session_ready(|session| {
            session.str_handle(source_h as u64, |source| {
                observe::record_piecewise_subrange_single_session_hit();
                let source = TextRef::new(source);
                let middle = TextRef::new(middle);
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
                    let total = prefix_slice.len() + middle.len() + suffix_slice.len();
                    let mut out_text = String::with_capacity(total);
                    unsafe {
                        let buf = out_text.as_mut_vec();
                        buf.set_len(total);
                        let mut cursor = 0usize;
                        std::ptr::copy_nonoverlapping(
                            prefix_slice.as_ptr(),
                            buf.as_mut_ptr().add(cursor),
                            prefix_slice.len(),
                        );
                        cursor += prefix_slice.len();
                        std::ptr::copy_nonoverlapping(
                            middle.as_str().as_ptr(),
                            buf.as_mut_ptr().add(cursor),
                            middle.len(),
                        );
                        cursor += middle.len();
                        std::ptr::copy_nonoverlapping(
                            suffix_slice.as_ptr(),
                            buf.as_mut_ptr().add(cursor),
                            suffix_slice.len(),
                        );
                    }
                    freeze_owned_string_into_slot(out, out_text);
                    return Some(());
                }
                let text = substring_owned_from_parts(
                    &[prefix, middle.as_str(), suffix],
                    slice_start,
                    slice_end,
                )?;
                record_piecewise_shape(prefix_hit, middle_hit, suffix_hit);
                freeze_owned_string_into_slot(out, text);
                Some(())
            })
        })
        .flatten()
        .flatten()
    })
    .is_some()
}

#[inline(always)]
pub(crate) fn piecewise_subrange_kernel_text_slot_into_slot(
    out: &mut KernelTextSlot,
    source: &KernelTextSlot,
    middle_ptr: *const i8,
    split: i64,
    start: i64,
    end: i64,
) -> bool {
    out.clear();
    with_insert_middle_text(middle_ptr, |middle| match source.state() {
        KernelTextSlotState::Empty => {
            let source = TextRef::new("");
            let middle = TextRef::new(middle);
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
                let total = prefix_slice.len() + middle.len() + suffix_slice.len();
                let mut out_text = String::with_capacity(total);
                unsafe {
                    let buf = out_text.as_mut_vec();
                    buf.set_len(total);
                    let mut cursor = 0usize;
                    std::ptr::copy_nonoverlapping(
                        prefix_slice.as_ptr(),
                        buf.as_mut_ptr().add(cursor),
                        prefix_slice.len(),
                    );
                    cursor += prefix_slice.len();
                    std::ptr::copy_nonoverlapping(
                        middle.as_str().as_ptr(),
                        buf.as_mut_ptr().add(cursor),
                        middle.len(),
                    );
                    cursor += middle.len();
                    std::ptr::copy_nonoverlapping(
                        suffix_slice.as_ptr(),
                        buf.as_mut_ptr().add(cursor),
                        suffix_slice.len(),
                    );
                }
                freeze_owned_string_into_slot(out, out_text);
                return Some(());
            }
            let text = substring_owned_from_parts(
                &[prefix, middle.as_str(), suffix],
                slice_start,
                slice_end,
            )?;
            record_piecewise_shape(prefix_hit, middle_hit, suffix_hit);
            freeze_owned_string_into_slot(out, text);
            Some(())
        }
        KernelTextSlotState::OwnedBytes | KernelTextSlotState::DeferredConstSuffix => {
            with_kernel_text_slot_text(source, |text| {
                let source = text;
                let middle = TextRef::new(middle);
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
                    let total = prefix_slice.len() + middle.len() + suffix_slice.len();
                    let mut out_text = String::with_capacity(total);
                    unsafe {
                        let buf = out_text.as_mut_vec();
                        buf.set_len(total);
                        let mut cursor = 0usize;
                        std::ptr::copy_nonoverlapping(
                            prefix_slice.as_ptr(),
                            buf.as_mut_ptr().add(cursor),
                            prefix_slice.len(),
                        );
                        cursor += prefix_slice.len();
                        std::ptr::copy_nonoverlapping(
                            middle.as_str().as_ptr(),
                            buf.as_mut_ptr().add(cursor),
                            middle.len(),
                        );
                        cursor += middle.len();
                        std::ptr::copy_nonoverlapping(
                            suffix_slice.as_ptr(),
                            buf.as_mut_ptr().add(cursor),
                            suffix_slice.len(),
                        );
                    }
                    freeze_owned_string_into_slot(out, out_text);
                    return Some(());
                }
                let text = substring_owned_from_parts(
                    &[prefix, middle.as_str(), suffix],
                    slice_start,
                    slice_end,
                )?;
                record_piecewise_shape(prefix_hit, middle_hit, suffix_hit);
                freeze_owned_string_into_slot(out, text);
                Some(())
            })
            .flatten()
        }
        KernelTextSlotState::Published => None,
    })
    .is_some()
}

#[inline(always)]
pub(crate) fn substring_kernel_text_slot_in_place(
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
    let text = TextRef::new(bytes.as_str());
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
pub(crate) fn piecewise_subrange_hsiii_fallback(
    source_h: i64,
    middle_ptr: *const i8,
    split: i64,
    start: i64,
    end: i64,
) -> i64 {
    observe::record_piecewise_subrange_enter();
    let mut slot = KernelTextSlot::empty();
    if piecewise_subrange_hsiii_into_slot(&mut slot, source_h, middle_ptr, split, start, end) {
        return publish_kernel_text_slot(&mut slot).unwrap_or_else(shared_empty_string_handle);
    }
    with_insert_middle_text(middle_ptr, |_middle| {
        observe::record_piecewise_subrange_fallback_insert();
        let inserted_h = insert_const_mid_fallback(source_h, middle_ptr, split);
        super::super::substring_fast_route(inserted_h, start, end)
    })
}
