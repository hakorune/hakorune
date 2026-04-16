use crate::exports::string_view::clamp_i64_range;
use nyash_rust::runtime::host_handles as handles;

use super::super::materialize::{shared_empty_string_handle, string_handle_from_owned};

enum ConcatSubstringPath {
    ReturnEmpty,
    SinglePiece { handle: i64, start: i64, end: i64 },
    Owned(String),
}

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
fn concat_pair_substring_path(
    a_h: i64,
    b_h: i64,
    start: i64,
    end: i64,
) -> Option<ConcatSubstringPath> {
    if a_h <= 0 || b_h <= 0 {
        return None;
    }
    handles::with_text_read_session(|session| {
        session.str_pair(a_h as u64, b_h as u64, |a, b| {
            let a_len = a.len();
            let total_len = a_len.saturating_add(b.len());
            let (slice_start, slice_end) = clamp_i64_range(total_len, start, end);
            if slice_start == slice_end {
                return ConcatSubstringPath::ReturnEmpty;
            }
            if slice_end <= a_len {
                return ConcatSubstringPath::SinglePiece {
                    handle: a_h,
                    start: slice_start as i64,
                    end: slice_end as i64,
                };
            }
            if slice_start >= a_len {
                let local_start = slice_start.saturating_sub(a_len);
                let local_end = slice_end.saturating_sub(a_len);
                return ConcatSubstringPath::SinglePiece {
                    handle: b_h,
                    start: local_start as i64,
                    end: local_end as i64,
                };
            }
            match substring_owned_from_parts(&[a, b], slice_start, slice_end) {
                Some(text) if text.is_empty() => ConcatSubstringPath::ReturnEmpty,
                Some(text) => ConcatSubstringPath::Owned(text),
                None => ConcatSubstringPath::ReturnEmpty,
            }
        })
    })
}

#[inline(always)]
fn concat3_substring_path(
    a_h: i64,
    b_h: i64,
    c_h: i64,
    start: i64,
    end: i64,
) -> Option<ConcatSubstringPath> {
    if a_h <= 0 || b_h <= 0 || c_h <= 0 {
        return None;
    }
    handles::with_text_read_session(|session| {
        session.str3(a_h as u64, b_h as u64, c_h as u64, |a, b, c| {
            let a_len = a.len();
            let ab_len = a_len.saturating_add(b.len());
            let total_len = ab_len.saturating_add(c.len());
            let (slice_start, slice_end) = clamp_i64_range(total_len, start, end);
            if slice_start == slice_end {
                return ConcatSubstringPath::ReturnEmpty;
            }
            if slice_end <= a_len {
                return ConcatSubstringPath::SinglePiece {
                    handle: a_h,
                    start: slice_start as i64,
                    end: slice_end as i64,
                };
            }
            if slice_start >= ab_len {
                let local_start = slice_start.saturating_sub(ab_len);
                let local_end = slice_end.saturating_sub(ab_len);
                return ConcatSubstringPath::SinglePiece {
                    handle: c_h,
                    start: local_start as i64,
                    end: local_end as i64,
                };
            }
            if slice_start >= a_len && slice_end <= ab_len {
                let local_start = slice_start.saturating_sub(a_len);
                let local_end = slice_end.saturating_sub(a_len);
                return ConcatSubstringPath::SinglePiece {
                    handle: b_h,
                    start: local_start as i64,
                    end: local_end as i64,
                };
            }
            match substring_owned_from_parts(&[a, b, c], slice_start, slice_end) {
                Some(text) if text.is_empty() => ConcatSubstringPath::ReturnEmpty,
                Some(text) => ConcatSubstringPath::Owned(text),
                None => ConcatSubstringPath::ReturnEmpty,
            }
        })
    })
}

#[inline(always)]
fn freeze_concat_substring_path(path: ConcatSubstringPath) -> i64 {
    match path {
        ConcatSubstringPath::ReturnEmpty => shared_empty_string_handle(),
        ConcatSubstringPath::SinglePiece { handle, start, end } => {
            super::super::string_substring_hii_export_impl(handle, start, end)
        }
        ConcatSubstringPath::Owned(text) => {
            if text.is_empty() {
                shared_empty_string_handle()
            } else {
                string_handle_from_owned(text)
            }
        }
    }
}

#[inline(always)]
pub(super) fn concat_pair_substring_fallback(a_h: i64, b_h: i64, start: i64, end: i64) -> i64 {
    if let Some(path) = concat_pair_substring_path(a_h, b_h, start, end) {
        return freeze_concat_substring_path(path);
    }
    let concat_h = super::concat_pair_fallback(a_h, b_h);
    super::super::string_substring_hii_export_impl(concat_h, start, end)
}

#[inline(always)]
pub(super) fn concat3_substring_fallback(
    a_h: i64,
    b_h: i64,
    c_h: i64,
    start: i64,
    end: i64,
) -> i64 {
    if let Some(path) = concat3_substring_path(a_h, b_h, c_h, start, end) {
        return freeze_concat_substring_path(path);
    }
    let concat_h = super::concat3_fallback(a_h, b_h, c_h);
    super::super::string_substring_hii_export_impl(concat_h, start, end)
}
