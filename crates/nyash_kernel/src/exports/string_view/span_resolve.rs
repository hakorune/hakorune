use super::super::string_span_cache::{
    string_span_cache_get, string_span_cache_get_pair, string_span_cache_get_triplet,
    string_span_cache_put,
};
use nyash_rust::{
    box_trait::{NyashBox, StringBox},
    runtime::host_handles as handles,
};
use std::sync::Arc;

use super::{clamp_usize_range, StringSpan, StringViewBox};

#[inline(always)]
pub(super) fn resolve_string_span_from_obj(
    handle: i64,
    obj: Arc<dyn NyashBox>,
) -> Option<StringSpan> {
    if let Some(sb) = obj.as_any().downcast_ref::<StringBox>() {
        let len = sb.value.len();
        return Some(StringSpan {
            base_handle: handle,
            base_obj: obj,
            start: 0,
            end: len,
        });
    }
    if let Some(view) = obj.as_any().downcast_ref::<StringViewBox>() {
        return resolve_string_span_from_view(view);
    }
    None
}

#[inline(always)]
fn resolve_string_span_from_handle_uncached(handle: i64) -> Option<StringSpan> {
    if handle <= 0 {
        return None;
    }
    let obj = handles::get(handle as u64)?;
    resolve_string_span_from_obj(handle, obj)
}

#[inline(always)]
pub(super) fn resolve_string_span_from_handle_nocache(handle: i64) -> Option<StringSpan> {
    resolve_string_span_from_handle_uncached(handle)
}

#[inline(always)]
fn resolve_string_span_from_handle_with_epoch(handle: i64, drop_epoch: u64) -> Option<StringSpan> {
    if handle <= 0 {
        return None;
    }
    if let Some(span) = string_span_cache_get(handle, drop_epoch) {
        return Some(span);
    }
    let span = resolve_string_span_from_handle_uncached(handle)?;
    string_span_cache_put(handle, drop_epoch, &span);
    Some(span)
}

#[inline(always)]
pub(super) fn resolve_string_span_from_handle(handle: i64) -> Option<StringSpan> {
    resolve_string_span_from_handle_with_epoch(handle, handles::drop_epoch())
}

#[inline(always)]
pub(super) fn resolve_string_span_pair_from_handles(
    a_h: i64,
    b_h: i64,
) -> Option<(StringSpan, StringSpan)> {
    if a_h <= 0 || b_h <= 0 {
        return None;
    }
    if a_h == b_h {
        let span = resolve_string_span_from_handle(a_h)?;
        return Some((span.clone(), span));
    }
    let drop_epoch = handles::drop_epoch();
    let (a_cached, b_cached) = string_span_cache_get_pair(a_h, b_h, drop_epoch);
    match (a_cached, b_cached) {
        (Some(a_span), Some(b_span)) => return Some((a_span, b_span)),
        (Some(a_span), None) => {
            let b_span = resolve_string_span_from_handle_with_epoch(b_h, drop_epoch)?;
            return Some((a_span, b_span));
        }
        (None, Some(b_span)) => {
            let a_span = resolve_string_span_from_handle_with_epoch(a_h, drop_epoch)?;
            return Some((a_span, b_span));
        }
        (None, None) => {}
    }

    let (a_obj, b_obj) = handles::get_pair(a_h as u64, b_h as u64);
    let a_obj = a_obj?;
    let b_obj = b_obj?;
    let a_span = resolve_string_span_from_obj(a_h, a_obj)?;
    let b_span = resolve_string_span_from_obj(b_h, b_obj)?;
    string_span_cache_put(a_h, drop_epoch, &a_span);
    string_span_cache_put(b_h, drop_epoch, &b_span);
    Some((a_span, b_span))
}

#[inline(always)]
pub(super) fn resolve_string_span_triplet_from_handles(
    a_h: i64,
    b_h: i64,
    c_h: i64,
) -> Option<(StringSpan, StringSpan, StringSpan)> {
    if a_h <= 0 || b_h <= 0 || c_h <= 0 {
        return None;
    }
    let drop_epoch = handles::drop_epoch();
    let (a_cached, b_cached, c_cached) = string_span_cache_get_triplet(a_h, b_h, c_h, drop_epoch);

    let a_span = match a_cached {
        Some(span) => span,
        None => {
            let span = resolve_string_span_from_handle_uncached(a_h)?;
            string_span_cache_put(a_h, drop_epoch, &span);
            span
        }
    };
    let b_span = if b_h == a_h {
        a_span.clone()
    } else {
        match b_cached {
            Some(span) => span,
            None => {
                let span = resolve_string_span_from_handle_uncached(b_h)?;
                string_span_cache_put(b_h, drop_epoch, &span);
                span
            }
        }
    };
    let c_span = if c_h == a_h {
        a_span.clone()
    } else if c_h == b_h {
        b_span.clone()
    } else {
        match c_cached {
            Some(span) => span,
            None => {
                let span = resolve_string_span_from_handle_uncached(c_h)?;
                string_span_cache_put(c_h, drop_epoch, &span);
                span
            }
        }
    };
    Some((a_span, b_span, c_span))
}

#[inline(always)]
pub(super) fn resolve_string_span_from_view(view: &StringViewBox) -> Option<StringSpan> {
    if let Some(base_sb) = view.base_obj.as_any().downcast_ref::<StringBox>() {
        let (st, en) = clamp_usize_range(base_sb.value.len(), view.start, view.end);
        return Some(StringSpan {
            base_handle: view.base_handle,
            base_obj: view.base_obj.clone(),
            start: st,
            end: en,
        });
    }

    if view.base_handle <= 0 {
        return None;
    }
    let base_obj = handles::get(view.base_handle as u64)?;

    if let Some(base_sb) = base_obj.as_any().downcast_ref::<StringBox>() {
        let (st, en) = clamp_usize_range(base_sb.value.len(), view.start, view.end);
        return Some(StringSpan {
            base_handle: view.base_handle,
            base_obj,
            start: st,
            end: en,
        });
    }

    if let Some(parent_view) = base_obj.as_any().downcast_ref::<StringViewBox>() {
        let parent_span = resolve_string_span_from_view(parent_view)?;
        let (st_rel, en_rel) = clamp_usize_range(parent_span.len(), view.start, view.end);
        return Some(StringSpan {
            base_handle: parent_span.base_handle,
            base_obj: parent_span.base_obj.clone(),
            start: parent_span.start + st_rel,
            end: parent_span.start + en_rel,
        });
    }
    None
}

#[inline(always)]
pub(super) fn string_len_from_handle(handle: i64) -> Option<i64> {
    resolve_string_span_from_handle(handle).map(|span| span.len() as i64)
}

#[inline(always)]
pub(super) fn string_is_empty_from_handle(handle: i64) -> Option<bool> {
    resolve_string_span_from_handle(handle).map(|span| span.len() == 0)
}
