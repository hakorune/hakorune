use crate::exports::string_view::clamp_i64_range;
use crate::observe;
use nyash_rust::runtime::host_handles as handles;

use super::const_adapter::{insert_const_mid_fallback, with_insert_middle_text};
use super::super::materialize::{shared_empty_string_handle, string_handle_from_owned};

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

#[inline(always)]
fn record_piecewise_shape(
    prefix_hit: bool,
    middle_hit: bool,
    suffix_hit: bool,
) {
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
fn piecewise_subrange_from_source(
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
    let suffix_len = suffix.len();
    let total_len = prefix
        .len()
        .saturating_add(middle_len)
        .saturating_add(suffix_len);
    let (slice_start, slice_end) = clamp_i64_range(total_len, start, end);
    if slice_start == slice_end {
        observe::record_piecewise_subrange_empty_return();
        return Some(String::new());
    }
    let prefix_hit = overlaps(slice_start, slice_end, 0, prefix_len);
    let middle_start = prefix_len;
    let middle_end = middle_start.saturating_add(middle_len);
    let suffix_start = middle_end;
    let suffix_end = suffix_start.saturating_add(suffix_len);
    let middle_hit = overlaps(slice_start, slice_end, middle_start, middle_end);
    let suffix_hit = overlaps(slice_start, slice_end, suffix_start, suffix_end);
    let text = substring_owned_from_parts(&[prefix, middle, suffix], slice_start, slice_end)?;
    record_piecewise_shape(prefix_hit, middle_hit, suffix_hit);
    Some(text)
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
    with_insert_middle_text(middle_ptr, |middle| {
        if source_h > 0 {
            if let Some(text) = handles::with_text_read_session(|session| {
                session.str_handle(source_h as u64, |source| {
                    piecewise_subrange_from_source(source, middle, split, start, end)
                })
            }) {
                observe::record_piecewise_subrange_single_session_hit();
                return match text {
                    Some(text) if text.is_empty() => shared_empty_string_handle(),
                    Some(text) => string_handle_from_owned(text),
                    None => shared_empty_string_handle(),
                };
            }
        }
        observe::record_piecewise_subrange_fallback_insert();
        let inserted_h = insert_const_mid_fallback(source_h, middle_ptr, split);
        super::super::string_substring_hii_export_impl(inserted_h, start, end)
    })
}
