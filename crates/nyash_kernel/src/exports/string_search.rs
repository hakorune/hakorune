use super::string_view::{resolve_string_span_from_handle, resolve_string_span_pair_from_handles};
use crate::plugin::TextRef;
use memchr::{memchr, memmem, memrchr};
use nyash_rust::runtime::host_handles as handles;

#[inline(always)]
pub(crate) fn bool_to_i64(value: bool) -> i64 {
    if value {
        1
    } else {
        0
    }
}

#[inline(always)]
pub(crate) fn empty_needle_indexof(_hay: &str) -> i64 {
    0
}

#[inline(always)]
pub(crate) fn empty_needle_lastindexof(hay: &str) -> i64 {
    hay.len() as i64
}

#[inline(always)]
pub(crate) fn find_substr_byte_index(hay: &str, needle: &str) -> Option<usize> {
    let hay_b = hay.as_bytes();
    let nee_b = needle.as_bytes();
    match nee_b.len() {
        0 => Some(0),
        1 => memchr(nee_b[0], hay_b),
        2 | 3 | 4 => find_substr_byte_index_small(hay_b, nee_b),
        _ => memmem::find(hay_b, nee_b),
    }
}

#[inline(always)]
fn find_substr_byte_index_small(hay_b: &[u8], nee_b: &[u8]) -> Option<usize> {
    let first = nee_b[0];
    let needle_len = nee_b.len();
    let mut offset = 0usize;
    let mut search = hay_b;

    while let Some(pos) = memchr(first, search) {
        let idx = offset + pos;
        let end = idx + needle_len;
        if end <= hay_b.len() && &hay_b[idx..end] == nee_b {
            return Some(idx);
        }
        offset = idx + 1;
        if offset >= hay_b.len() {
            return None;
        }
        search = &hay_b[offset..];
    }

    None
}

#[inline(always)]
pub(crate) fn rfind_substr_byte_index(hay: &str, needle: &str) -> Option<usize> {
    let hay_b = hay.as_bytes();
    let nee_b = needle.as_bytes();
    match nee_b.len() {
        0 => Some(hay_b.len()),
        1 => memrchr(nee_b[0], hay_b),
        2 | 3 | 4 => rfind_substr_byte_index_small(hay_b, nee_b),
        _ => memmem::rfind(hay_b, nee_b),
    }
}

#[inline(always)]
fn rfind_substr_byte_index_small(hay_b: &[u8], nee_b: &[u8]) -> Option<usize> {
    let first = nee_b[0];
    let needle_len = nee_b.len();
    let mut search = hay_b;

    while let Some(pos) = memrchr(first, search) {
        let end = pos + needle_len;
        if end <= search.len() && &search[pos..end] == nee_b {
            return Some(pos + (hay_b.len() - search.len()));
        }
        if pos == 0 {
            break;
        }
        search = &search[..pos];
    }

    None
}

pub(crate) fn with_string_pair_fast_str<R>(
    a_h: i64,
    b_h: i64,
    f: impl FnOnce(TextRef<'_>, TextRef<'_>) -> R,
) -> Option<R> {
    if a_h <= 0 || b_h <= 0 {
        return None;
    }
    handles::with_str_pair(a_h as u64, b_h as u64, |a, b| {
        f(TextRef::new(a), TextRef::new(b))
    })
}

pub(crate) fn with_string_pair_span<R>(
    a_h: i64,
    b_h: i64,
    f: impl FnOnce(TextRef<'_>, TextRef<'_>) -> R,
) -> Option<R> {
    let (a_span, b_span) = resolve_string_span_pair_from_handles(a_h, b_h)?;
    Some(f(a_span.as_text(), b_span.as_text()))
}

pub(crate) fn with_string_pair_lossy_span<R>(
    a_h: i64,
    b_h: i64,
    f: impl FnOnce(TextRef<'_>, TextRef<'_>) -> R,
) -> R {
    let empty = TextRef::new("");
    let a_span = resolve_string_span_from_handle(a_h);
    let b_span = resolve_string_span_from_handle(b_h);
    let a = a_span.as_ref().map(|span| span.as_text()).unwrap_or(empty);
    let b = b_span.as_ref().map(|span| span.as_text()).unwrap_or(empty);
    f(a, b)
}

pub(crate) fn with_lossy_string_pair<R>(
    a_h: i64,
    b_h: i64,
    f: impl FnOnce(TextRef<'_>, TextRef<'_>) -> R,
) -> R {
    let mut f_opt = Some(f);
    if let Some(out) = with_string_pair_fast_str(a_h, b_h, |a, b| {
        let f = f_opt
            .take()
            .expect("[string/export] with_lossy_string_pair closure missing (fast)");
        f(a, b)
    }) {
        return out;
    }
    if let Some(out) = with_string_pair_span(a_h, b_h, |a, b| {
        let f = f_opt
            .take()
            .expect("[string/export] with_lossy_string_pair closure missing (span)");
        f(a, b)
    }) {
        return out;
    }
    with_string_pair_lossy_span(a_h, b_h, |a, b| {
        let f = f_opt
            .take()
            .expect("[string/export] with_lossy_string_pair closure missing (lossy)");
        f(a, b)
    })
}

#[inline(always)]
pub(crate) fn search_string_pair_hh(
    hay_h: i64,
    needle_h: i64,
    empty_result: fn(&str) -> i64,
    search: fn(&str, &str) -> Option<usize>,
) -> i64 {
    let eval = |hay: TextRef<'_>, nee: TextRef<'_>| -> i64 {
        if nee.is_empty() {
            return empty_result(hay.as_str());
        }
        search(hay.as_str(), nee.as_str())
            .map(|pos| pos as i64)
            .unwrap_or(-1)
    };

    with_lossy_string_pair(hay_h, needle_h, |hay, nee| eval(hay, nee))
}

#[inline(always)]
pub(crate) fn compare_string_pair_hh(lhs_h: i64, rhs_h: i64, cmp: fn(&str, &str) -> bool) -> i64 {
    with_lossy_string_pair(lhs_h, rhs_h, |lhs, rhs| {
        bool_to_i64(cmp(lhs.as_str(), rhs.as_str()))
    })
}
