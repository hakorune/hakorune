use super::cache::{string_len_fast_cache_lookup, string_len_fast_cache_store};
use crate::exports::string_plan::TextPlan;
use crate::exports::string_trace;
use crate::exports::string_view::{
    resolve_string_span_from_handle, string_is_empty_from_handle as string_is_empty_impl,
    string_len_from_handle as string_len_impl, StringSpan, StringViewBox,
};
use crate::observe;
use crate::plugin::{
    materialize_owned_string_generic_fallback, materialize_owned_string_generic_fallback_for_site,
    StringPublishSite,
};
use nyash_rust::box_trait::StringBox;
use nyash_rust::runtime::host_handles as handles;
use std::ptr;

#[inline(always)]
pub(crate) fn string_len_from_handle(handle: i64) -> Option<i64> {
    if handle <= 0 {
        observe::record_str_len_route_miss();
        trace_observer_resolution("observer", handle, "none", "invalid_handle", || {
            "invalid_handle".to_string()
        });
        return None;
    }
    let trace_enabled = string_trace::enabled();
    if let Some(cached) = string_len_fast_cache_lookup(handle) {
        observe::record_str_len_route_fast_str_hit();
        if observe::len_route_matches_latest_fresh_handle(handle) {
            observe::record_str_len_route_latest_fresh_handle_fast_str_hit();
        }
        trace_observer_resolution_enabled(
            trace_enabled,
            "observer",
            handle,
            "fast_hit",
            "len_handle_cache",
            || format!("len={}", cached),
        );
        return Some(cached);
    }
    if let Some(view_len) = handles::with_handle(handle as u64, |obj| {
        let Some(obj) = obj else {
            return None;
        };
        if let Some(sb) = obj.as_any().downcast_ref::<StringBox>() {
            return Some(sb.value.len() as i64);
        }
        let view = obj.as_any().downcast_ref::<StringViewBox>()?;
        Some(view.end.saturating_sub(view.start) as i64)
    }) {
        observe::record_str_len_route_fast_str_hit();
        if observe::len_route_matches_latest_fresh_handle(handle) {
            observe::record_str_len_route_latest_fresh_handle_fast_str_hit();
        }
        string_len_fast_cache_store(handle, view_len);
        trace_observer_resolution_enabled(
            trace_enabled,
            "observer",
            handle,
            "fast_hit",
            "live_object_fast",
            || format!("len={}", view_len),
        );
        return Some(view_len);
    }
    let fast_len = handles::with_text_read_session(|session| {
        session.str_handle(handle as u64, |text| text.len() as i64)
    });
    if fast_len.is_some() {
        observe::record_str_len_route_fast_str_hit();
        if observe::len_route_matches_latest_fresh_handle(handle) {
            observe::record_str_len_route_latest_fresh_handle_fast_str_hit();
        }
        string_len_fast_cache_store(handle, fast_len.unwrap_or_default());
        trace_observer_resolution_enabled(
            trace_enabled,
            "observer",
            handle,
            "fast_hit",
            "as_str_fast",
            || format!("len={}", fast_len.unwrap_or_default()),
        );
        return fast_len;
    }
    let fallback = string_len_impl(handle);
    if fallback.is_some() {
        observe::record_str_len_route_fallback_hit();
        if observe::len_route_matches_latest_fresh_handle(handle) {
            observe::record_str_len_route_latest_fresh_handle_fallback_hit();
        }
        string_len_fast_cache_store(handle, fallback.unwrap_or_default());
    } else {
        observe::record_str_len_route_miss();
    }
    trace_observer_resolution_enabled(
        trace_enabled,
        "observer",
        handle,
        if fallback.is_some() {
            "fallback_hit"
        } else {
            "fallback_miss"
        },
        "string_len_impl",
        || format!("len={}", fallback.unwrap_or_default()),
    );
    fallback
}

#[inline(always)]
pub(crate) fn string_is_empty_from_handle(handle: i64) -> Option<bool> {
    if handle <= 0 {
        trace_observer_resolution("observer", handle, "none", "invalid_handle", || {
            "invalid_handle".to_string()
        });
        return None;
    }
    let trace_enabled = string_trace::enabled();
    if let Some(view_len) = string_len_fast_cache_lookup(handle) {
        let empty = view_len == 0;
        trace_observer_resolution_enabled(
            trace_enabled,
            "observer",
            handle,
            "fast_hit",
            "live_object_fast",
            || format!("empty={}", empty),
        );
        return Some(empty);
    }
    if let Some(view_len) = handles::with_handle(handle as u64, |obj| {
        let Some(obj) = obj else {
            return None;
        };
        if let Some(sb) = obj.as_any().downcast_ref::<StringBox>() {
            return Some(sb.value.len() as i64);
        }
        let view = obj.as_any().downcast_ref::<StringViewBox>()?;
        Some(view.end.saturating_sub(view.start) as i64)
    }) {
        let empty = view_len == 0;
        string_len_fast_cache_store(handle, view_len);
        trace_observer_resolution_enabled(
            trace_enabled,
            "observer",
            handle,
            "fast_hit",
            "live_object_fast",
            || format!("empty={}", empty),
        );
        return Some(empty);
    }
    let fast_len = handles::with_text_read_session(|session| {
        session.str_handle(handle as u64, |text| text.len() as i64)
    });
    if fast_len.is_some() {
        let empty = fast_len.unwrap_or_default() == 0;
        string_len_fast_cache_store(handle, fast_len.unwrap_or_default());
        trace_observer_resolution_enabled(
            trace_enabled,
            "observer",
            handle,
            "fast_hit",
            "as_str_fast",
            || format!("empty={}", empty),
        );
        return Some(empty);
    }
    let fallback = string_is_empty_impl(handle);
    trace_observer_resolution_enabled(
        trace_enabled,
        "observer",
        handle,
        if fallback.is_some() {
            "fallback_hit"
        } else {
            "fallback_miss"
        },
        "string_is_empty_impl",
        || format!("empty={}", fallback.unwrap_or(false)),
    );
    fallback
}

#[inline(always)]
pub(super) fn string_handle_from_owned(value: String) -> i64 {
    string_handle_from_owned_with_site(value, StringPublishSite::Generic)
}

#[inline(always)]
pub(super) fn string_handle_from_owned_concat_hh(value: String) -> i64 {
    string_handle_from_owned_with_site(value, StringPublishSite::StringConcatHh)
}

#[inline(always)]
pub(super) fn string_handle_from_owned_substring_concat_hhii(value: String) -> i64 {
    string_handle_from_owned_with_site(value, StringPublishSite::StringSubstringConcatHhii)
}

#[inline(always)]
pub(super) fn string_handle_from_owned_const_suffix(value: String) -> i64 {
    string_handle_from_owned_with_site(value, StringPublishSite::ConstSuffix)
}

#[inline(always)]
fn string_handle_from_owned_with_site(value: String, site: StringPublishSite) -> i64 {
    let len = value.len();
    if len == 0 {
        return shared_empty_string_handle();
    }
    observe::record_birth_placement_fresh_handle();
    let handle = match site {
        StringPublishSite::Generic => materialize_owned_string_generic_fallback(value),
        _ => materialize_owned_string_generic_fallback_for_site(value, site),
    };
    string_len_fast_cache_store(handle, len as i64);
    if string_trace::enabled() {
        string_trace::emit(
            "sink",
            "fresh_handle",
            "materialize_owned_string",
            format_args!("source=owned len={} handle={}", len, handle),
        );
    }
    handle
}

#[inline(always)]
pub(super) fn string_handle_from_span(span: StringSpan) -> i64 {
    let source = span.as_str();
    if source.is_empty() {
        if string_trace::enabled() {
            string_trace::emit(
                "sink",
                "shared_empty",
                "span_empty",
                format_args!(
                    "source=span len=0 base_handle={} range={}..{}",
                    span.base_handle(),
                    span.start(),
                    span.end()
                ),
            );
        }
        return shared_empty_string_handle();
    }
    observe::record_birth_placement_materialize_owned();
    let len = source.len();
    let mut out = String::with_capacity(len);
    unsafe {
        let buf = out.as_mut_vec();
        buf.set_len(len);
        ptr::copy_nonoverlapping(source.as_ptr(), buf.as_mut_ptr(), len);
    }
    let handle = string_handle_from_owned(out);
    if string_trace::enabled() {
        string_trace::emit(
            "sink",
            "fresh_handle",
            "span_materialize",
            format_args!(
                "source=span len={} base_handle={} range={}..{} handle={}",
                len,
                span.base_handle(),
                span.start(),
                span.end(),
                handle
            ),
        );
    }
    handle
}

#[inline(always)]
pub(super) fn freeze_text_plan<'a>(plan: TextPlan<'a>) -> i64 {
    freeze_text_plan_with_site(plan, StringPublishSite::Generic)
}

#[inline(always)]
pub(super) fn freeze_text_plan_with_site<'a>(plan: TextPlan<'a>, site: StringPublishSite) -> i64 {
    observe::record_birth_placement_freeze_owned();
    let pieces3_site = matches!(&plan, TextPlan::Pieces3 { .. });
    match &plan {
        TextPlan::View1(_) => observe::record_birth_backend_freeze_text_plan_view1(),
        TextPlan::Pieces2 { .. } => observe::record_birth_backend_freeze_text_plan_pieces2(),
        TextPlan::Pieces3 { .. } => observe::record_birth_backend_freeze_text_plan_pieces3(),
        TextPlan::Pieces4 { .. } => observe::record_birth_backend_freeze_text_plan_pieces4(),
        TextPlan::OwnedTmp(_) => observe::record_birth_backend_freeze_text_plan_owned_tmp(),
    }
    if string_trace::enabled() {
        let piece_count = text_plan_piece_count(&plan);
        let total_len = text_plan_total_len(&plan);
        string_trace::emit(
            "sink",
            "freeze_plan",
            "freeze_text_plan",
            format_args!(
                "plan_shape={} piece_count={} total_len={}",
                text_plan_shape(&plan),
                piece_count,
                total_len
            ),
        );
    }
    let site = match site {
        StringPublishSite::Generic if pieces3_site => StringPublishSite::FreezeTextPlanPieces3,
        other => other,
    };
    string_handle_from_owned_with_site(plan.into_owned(), site)
}

#[inline(always)]
pub(super) fn concat_two_str(a: &str, b: &str) -> String {
    let a_len = a.len();
    let total = a_len + b.len();
    let mut out = String::with_capacity(total);
    unsafe {
        let buf = out.as_mut_vec();
        buf.set_len(total);
        ptr::copy_nonoverlapping(a.as_ptr(), buf.as_mut_ptr(), a_len);
        ptr::copy_nonoverlapping(b.as_ptr(), buf.as_mut_ptr().add(a_len), b.len());
    }
    out
}

#[inline(always)]
pub(super) fn concat_three_str(a: &str, b: &str, c: &str) -> String {
    let a_len = a.len();
    let b_len = b.len();
    let total = a_len + b_len + c.len();
    let mut out = String::with_capacity(total);
    unsafe {
        let buf = out.as_mut_vec();
        buf.set_len(total);
        ptr::copy_nonoverlapping(a.as_ptr(), buf.as_mut_ptr(), a_len);
        ptr::copy_nonoverlapping(b.as_ptr(), buf.as_mut_ptr().add(a_len), b_len);
        ptr::copy_nonoverlapping(c.as_ptr(), buf.as_mut_ptr().add(a_len + b_len), c.len());
    }
    out
}

#[inline(always)]
pub(super) fn shared_empty_string_handle() -> i64 {
    #[cfg(test)]
    {
        string_handle_from_owned(String::new())
    }
    #[cfg(not(test))]
    {
        static HANDLE: std::sync::OnceLock<i64> = std::sync::OnceLock::new();
        *HANDLE.get_or_init(|| {
            handles::to_handle_arc(std::sync::Arc::new(StringBox::new(String::new()))) as i64
        })
    }
}

#[inline(always)]
pub(super) fn text_plan_shape(plan: &TextPlan<'_>) -> &'static str {
    match plan {
        TextPlan::View1(_) => "view1",
        TextPlan::Pieces2 { .. } => "pieces2",
        TextPlan::Pieces3 { .. } => "pieces3",
        TextPlan::Pieces4 { .. } => "pieces4",
        TextPlan::OwnedTmp(_) => "owned_tmp",
    }
}

#[inline(always)]
pub(super) fn text_plan_piece_count(plan: &TextPlan<'_>) -> usize {
    match plan {
        TextPlan::View1(_) => 1,
        TextPlan::Pieces2 { .. } => 2,
        TextPlan::Pieces3 { .. } => 3,
        TextPlan::Pieces4 { .. } => 4,
        TextPlan::OwnedTmp(_) => 1,
    }
}

#[inline(always)]
pub(super) fn text_plan_total_len(plan: &TextPlan<'_>) -> usize {
    match plan {
        TextPlan::View1(span) => span.len(),
        TextPlan::Pieces2 { total_len, .. }
        | TextPlan::Pieces3 { total_len, .. }
        | TextPlan::Pieces4 { total_len, .. } => *total_len,
        TextPlan::OwnedTmp(text) => text.len(),
    }
}

#[inline(always)]
fn trace_observer_resolution(
    stage: &str,
    handle: i64,
    result: &str,
    reason: &str,
    extra: impl FnOnce() -> String,
) {
    trace_observer_resolution_enabled(
        string_trace::enabled(),
        stage,
        handle,
        result,
        reason,
        extra,
    );
}

#[inline(always)]
pub(super) fn trace_observer_resolution_enabled(
    trace_enabled: bool,
    stage: &str,
    handle: i64,
    result: &str,
    reason: &str,
    extra: impl FnOnce() -> String,
) {
    if !trace_enabled {
        return;
    }
    string_trace::emit(
        stage,
        result,
        reason,
        format_args!("handle={} {}", handle, extra()),
    );
}

pub(super) fn concat_to_string_handle(parts: &[&str]) -> i64 {
    match parts.len() {
        0 => return string_handle_from_owned(String::new()),
        1 => return string_handle_from_owned(parts[0].to_string()),
        2 => return string_handle_from_owned(concat_two_str(parts[0], parts[1])),
        3 => return string_handle_from_owned(concat_three_str(parts[0], parts[1], parts[2])),
        _ => {}
    }
    let mut total = 0usize;
    for p in parts {
        total += p.len();
    }
    let mut out = String::with_capacity(total);
    for p in parts {
        out.push_str(p);
    }
    string_handle_from_owned(out)
}

#[inline(always)]
pub(crate) fn to_owned_string_handle_arg(h: i64) -> String {
    resolve_string_span_from_handle(h)
        .map(|span| span.as_str().to_string())
        .unwrap_or_default()
}
