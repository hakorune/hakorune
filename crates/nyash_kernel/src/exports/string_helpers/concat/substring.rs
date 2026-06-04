use crate::exports::string_view::clamp_i64_range;
use nyash_rust::runtime::host_handles as handles;

use super::super::materialize::{shared_empty_string_handle, to_owned_string_handle_arg};
use crate::plugin::{freeze_owned_bytes_with_site, publish_owned_bytes_with_reason_and_site};
use crate::plugin::{PublishReason, StringPublishSite};

pub(in crate::exports::string) enum SubstringConcatPath {
    ReturnEmpty,
    SinglePiece { handle: i64, start: i64, end: i64 },
    Owned(String),
}

#[inline(always)]
pub(in crate::exports::string) fn substring_concat_path_from_parts(
    handles: &[i64],
    parts: &[&str],
    start: i64,
    end: i64,
) -> Option<SubstringConcatPath> {
    if handles.iter().any(|&handle| handle <= 0) {
        return None;
    }
    let total_len = parts
        .iter()
        .fold(0usize, |acc, part| acc.saturating_add(part.len()));
    let (slice_start, slice_end) = clamp_i64_range(total_len, start, end);
    if slice_start == slice_end {
        return Some(SubstringConcatPath::ReturnEmpty);
    }

    let mut cursor = 0usize;
    for (idx, part) in parts.iter().enumerate() {
        let piece_start = cursor;
        let piece_end = cursor.saturating_add(part.len());
        if slice_start >= piece_start && slice_end <= piece_end {
            return Some(SubstringConcatPath::SinglePiece {
                handle: handles[idx],
                start: slice_start.saturating_sub(piece_start) as i64,
                end: slice_end.saturating_sub(piece_start) as i64,
            });
        }
        cursor = piece_end;
        if cursor >= slice_end {
            break;
        }
    }

    super::common::substring_owned_from_parts(parts, slice_start, slice_end)
        .map(SubstringConcatPath::Owned)
}

#[inline(always)]
pub(in crate::exports::string) fn concat3_substring_publish_owned_with_reason(
    a_h: i64,
    b_h: i64,
    c_h: i64,
    start: i64,
    end: i64,
    reason: PublishReason,
) -> i64 {
    let owned = handles::with_text_read_session(|session| {
        session
            .str3(a_h as u64, b_h as u64, c_h as u64, |a, b, c| {
                substring_concat_path_from_parts(&[a_h, b_h, c_h], &[a, b, c], start, end)
            })
            .flatten()
    })
    .and_then(|path| match path {
        SubstringConcatPath::ReturnEmpty => Some(String::new()),
        SubstringConcatPath::SinglePiece { handle, start, end } => {
            if handle <= 0 {
                return None;
            }
            let slice_owned = |text: &str| {
                let (slice_start, slice_end) = clamp_i64_range(text.len(), start, end);
                text.get(slice_start..slice_end).map(str::to_owned)
            };
            handles::with_text_read_session(|session| {
                session.str_handle(handle as u64, slice_owned)
            })
            .flatten()
            .or_else(|| {
                let owned = to_owned_string_handle_arg(handle);
                slice_owned(owned.as_str())
            })
        }
        SubstringConcatPath::Owned(text) => Some(text),
    })
    .or_else(|| {
        let concat_h = super::concat3_fallback(a_h, b_h, c_h);
        if concat_h <= 0 {
            return None;
        }
        let slice_owned = |text: &str| {
            let (slice_start, slice_end) = clamp_i64_range(text.len(), start, end);
            text.get(slice_start..slice_end).map(str::to_owned)
        };
        handles::with_text_read_session(|session| session.str_handle(concat_h as u64, slice_owned))
            .flatten()
            .or_else(|| {
                let owned = to_owned_string_handle_arg(concat_h);
                slice_owned(owned.as_str())
            })
    });

    match owned {
        Some(text) if text.is_empty() => shared_empty_string_handle(),
        Some(text) => publish_owned_bytes_with_reason_and_site(
            freeze_owned_bytes_with_site(text, StringPublishSite::StringSubstringConcatHhii),
            reason,
            StringPublishSite::StringSubstringConcatHhii,
        ),
        None => shared_empty_string_handle(),
    }
}
