// String export helper logic split out from string.rs.

use super::super::string_birth_placement::{
    concat3_retention_class, concat_suffix_retention_class, insert_middle_retention_class,
    RetainedForm,
};
use super::super::string_debug::{
    jit_trace_len_enabled, stage1_string_debug_log_concat_materialize, stage1_string_debug_log_eq,
    substring_view_enabled,
};
use super::super::string_plan::{
    concat_const_suffix_plan_from_handle, insert_const_mid_plan_from_handle, TextPiece, TextPlan,
};
use super::super::string_search::{
    compare_string_pair_hh, empty_needle_indexof, empty_needle_lastindexof, find_substr_byte_index,
    rfind_substr_byte_index, search_string_pair_hh,
};
use super::super::string_trace;
use super::super::string_view::{
    borrowed_substring_plan_from_handle, resolve_string_span_from_handle,
    resolve_string_span_pair_from_handles, resolve_string_span_triplet_from_handles,
    string_is_empty_from_handle as string_is_empty_impl, string_len_from_handle as string_len_impl,
    BorrowedSubstringPlan, StringSpan, StringViewBox,
};
use crate::hako_forward_bridge;
use crate::observe;
use crate::plugin::{issue_fresh_handle_from_arc, materialize_owned_string};
use nyash_rust::box_trait::{NyashBox, StringBox};
use nyash_rust::runtime::host_handles as handles;
use std::{
    cell::{Cell, RefCell},
    ffi::CStr,
    ptr,
    sync::Arc,
    thread::LocalKey,
};

// Native string substrate helpers.
// These routines stay below semantic ownership and keep raw copy/search/materialize
// fast paths in Rust unless a source-backed replacement proves safe.
// They serve the thin ABI facade and VM wrappers; they do not own route policy.

#[derive(Default)]
struct ConstSuffixTextCache {
    ptr: Cell<usize>,
    text: RefCell<Option<String>>,
}

#[derive(Default)]
struct ConcatPairFastCache {
    drop_epoch: Cell<u64>,
    lhs_handle: Cell<i64>,
    rhs_handle: Cell<i64>,
    result: RefCell<Option<Arc<dyn NyashBox>>>,
}

#[derive(Default)]
struct Concat3FastCache {
    drop_epoch: Cell<u64>,
    a_handle: Cell<i64>,
    b_handle: Cell<i64>,
    c_handle: Cell<i64>,
    result_handle: Cell<i64>,
}

#[derive(Default)]
struct ConcatConstSuffixFastCache {
    drop_epoch: Cell<u64>,
    source_handle: Cell<i64>,
    suffix_ptr: Cell<usize>,
    result_handle: Cell<i64>,
}

#[derive(Default)]
struct SubstringFastCache {
    drop_epoch: Cell<u64>,
    source_handle: Cell<i64>,
    start: Cell<i64>,
    end: Cell<i64>,
    view_enabled: Cell<bool>,
    result_handle: Cell<i64>,
    source_handle2: Cell<i64>,
    start2: Cell<i64>,
    end2: Cell<i64>,
    view_enabled2: Cell<bool>,
    result_handle2: Cell<i64>,
}

#[derive(Default)]
struct StringLenFastCache {
    drop_epoch: Cell<u64>,
    handle: Cell<i64>,
    len: Cell<i64>,
    handle2: Cell<i64>,
    len2: Cell<i64>,
}

thread_local! {
    static CONST_SUFFIX_TEXT_CACHE: ConstSuffixTextCache = const { ConstSuffixTextCache {
        ptr: Cell::new(0),
        text: RefCell::new(None),
    } };
    static CONCAT_PAIR_FAST_CACHE: ConcatPairFastCache = const { ConcatPairFastCache {
        drop_epoch: Cell::new(0),
        lhs_handle: Cell::new(0),
        rhs_handle: Cell::new(0),
        result: RefCell::new(None),
    } };
    static CONCAT3_FAST_CACHE: Concat3FastCache = const { Concat3FastCache {
        drop_epoch: Cell::new(0),
        a_handle: Cell::new(0),
        b_handle: Cell::new(0),
        c_handle: Cell::new(0),
        result_handle: Cell::new(0),
    } };
    static CONCAT_CONST_SUFFIX_FAST_CACHE: ConcatConstSuffixFastCache = const { ConcatConstSuffixFastCache {
        drop_epoch: Cell::new(0),
        source_handle: Cell::new(0),
        suffix_ptr: Cell::new(0),
        result_handle: Cell::new(0),
    } };
    static SUBSTRING_FAST_CACHE: SubstringFastCache = const { SubstringFastCache {
        drop_epoch: Cell::new(0),
        source_handle: Cell::new(0),
        start: Cell::new(0),
        end: Cell::new(0),
        view_enabled: Cell::new(false),
        result_handle: Cell::new(0),
        source_handle2: Cell::new(0),
        start2: Cell::new(0),
        end2: Cell::new(0),
        view_enabled2: Cell::new(false),
        result_handle2: Cell::new(0),
    } };
    static STRING_LEN_FAST_CACHE: StringLenFastCache = const { StringLenFastCache {
        drop_epoch: Cell::new(0),
        handle: Cell::new(0),
        len: Cell::new(0),
        handle2: Cell::new(0),
        len2: Cell::new(0),
    } };
}

#[inline(always)]
fn with_cached_const_suffix_text<R>(ptr: *const i8, f: impl FnOnce(&str) -> R) -> R {
    if ptr.is_null() {
        return f("");
    }
    let addr = ptr as usize;
    CONST_SUFFIX_TEXT_CACHE.with(|cache| {
        if cache.ptr.get() != addr || cache.text.borrow().is_none() {
            let bytes = unsafe { CStr::from_ptr(ptr) }.to_bytes();
            let text = String::from_utf8_lossy(bytes).into_owned();
            observe::record_const_suffix_text_cache_reload();
            cache.ptr.set(addr);
            *cache.text.borrow_mut() = Some(text);
        }
        let text_ref = cache.text.borrow();
        f(text_ref.as_deref().unwrap_or(""))
    })
}

#[inline(always)]
fn concat_pair_fast_cache_lookup(a_h: i64, b_h: i64) -> Option<Arc<dyn NyashBox>> {
    let drop_epoch = handles::drop_epoch();
    CONCAT_PAIR_FAST_CACHE.with(|cache| {
        if cache.drop_epoch.get() != drop_epoch
            || cache.lhs_handle.get() != a_h
            || cache.rhs_handle.get() != b_h
        {
            return None;
        }
        let result = cache.result.borrow();
        result.as_ref().cloned()
    })
}

#[inline(always)]
fn concat_pair_fast_cache_store(a_h: i64, b_h: i64, result: Arc<dyn NyashBox>) {
    let drop_epoch = handles::drop_epoch();
    CONCAT_PAIR_FAST_CACHE.with(|cache| {
        cache.drop_epoch.set(drop_epoch);
        cache.lhs_handle.set(a_h);
        cache.rhs_handle.set(b_h);
        *cache.result.borrow_mut() = Some(result);
    });
}

#[inline(always)]
fn concat3_fast_cache_lookup(a_h: i64, b_h: i64, c_h: i64) -> Option<i64> {
    let drop_epoch = handles::drop_epoch();
    CONCAT3_FAST_CACHE.with(|cache| {
        if cache.drop_epoch.get() == drop_epoch
            && cache.a_handle.get() == a_h
            && cache.b_handle.get() == b_h
            && cache.c_handle.get() == c_h
            && cache.result_handle.get() > 0
        {
            Some(cache.result_handle.get())
        } else {
            None
        }
    })
}

#[inline(always)]
fn concat3_fast_cache_store(a_h: i64, b_h: i64, c_h: i64, result_handle: i64) {
    let drop_epoch = handles::drop_epoch();
    CONCAT3_FAST_CACHE.with(|cache| {
        cache.drop_epoch.set(drop_epoch);
        cache.a_handle.set(a_h);
        cache.b_handle.set(b_h);
        cache.c_handle.set(c_h);
        cache.result_handle.set(result_handle);
    });
}

#[inline(always)]
fn concat_const_suffix_fast_cache_lookup(source_handle: i64, suffix_ptr: *const i8) -> Option<i64> {
    let drop_epoch = handles::drop_epoch();
    CONCAT_CONST_SUFFIX_FAST_CACHE.with(|cache| {
        if cache.drop_epoch.get() == drop_epoch
            && cache.source_handle.get() == source_handle
            && cache.suffix_ptr.get() == suffix_ptr as usize
            && cache.result_handle.get() > 0
        {
            Some(cache.result_handle.get())
        } else {
            None
        }
    })
}

#[inline(always)]
fn concat_const_suffix_fast_cache_store(
    source_handle: i64,
    suffix_ptr: *const i8,
    result_handle: i64,
) {
    let drop_epoch = handles::drop_epoch();
    CONCAT_CONST_SUFFIX_FAST_CACHE.with(|cache| {
        cache.drop_epoch.set(drop_epoch);
        cache.source_handle.set(source_handle);
        cache.suffix_ptr.set(suffix_ptr as usize);
        cache.result_handle.set(result_handle);
    });
}

#[inline(always)]
fn substring_fast_cache_lookup(
    source_handle: i64,
    start: i64,
    end: i64,
    view_enabled: bool,
) -> Option<i64> {
    let drop_epoch = handles::drop_epoch();
    SUBSTRING_FAST_CACHE.with(|cache| {
        if cache.drop_epoch.get() == drop_epoch {
            if cache.source_handle.get() == source_handle
                && cache.start.get() == start
                && cache.end.get() == end
                && cache.view_enabled.get() == view_enabled
                && cache.result_handle.get() > 0
            {
                return Some(cache.result_handle.get());
            }
            if cache.source_handle2.get() == source_handle
                && cache.start2.get() == start
                && cache.end2.get() == end
                && cache.view_enabled2.get() == view_enabled
                && cache.result_handle2.get() > 0
            {
                return Some(cache.result_handle2.get());
            }
        }
        None
    })
}

#[inline(always)]
fn substring_fast_cache_store(
    source_handle: i64,
    start: i64,
    end: i64,
    view_enabled: bool,
    result_handle: i64,
) {
    let drop_epoch = handles::drop_epoch();
    SUBSTRING_FAST_CACHE.with(|cache| {
        cache.drop_epoch.set(drop_epoch);
        cache.source_handle2.set(cache.source_handle.get());
        cache.start2.set(cache.start.get());
        cache.end2.set(cache.end.get());
        cache.view_enabled2.set(cache.view_enabled.get());
        cache.result_handle2.set(cache.result_handle.get());
        cache.source_handle.set(source_handle);
        cache.start.set(start);
        cache.end.set(end);
        cache.view_enabled.set(view_enabled);
        cache.result_handle.set(result_handle);
    });
}

#[inline(always)]
fn string_len_fast_cache_lookup(handle: i64) -> Option<i64> {
    let drop_epoch = handles::drop_epoch();
    STRING_LEN_FAST_CACHE.with(|cache| {
        if cache.drop_epoch.get() == drop_epoch {
            if cache.handle.get() == handle {
                Some(cache.len.get())
            } else if cache.handle2.get() == handle {
                Some(cache.len2.get())
            } else {
                None
            }
        } else {
            None
        }
    })
}

#[inline(always)]
fn string_len_fast_cache_store(handle: i64, len: i64) {
    let drop_epoch = handles::drop_epoch();
    STRING_LEN_FAST_CACHE.with(|cache| {
        cache.drop_epoch.set(drop_epoch);
        cache.handle2.set(cache.handle.get());
        cache.len2.set(cache.len.get());
        cache.handle.set(handle);
        cache.len.set(len);
    });
}

pub(crate) fn string_len_from_handle(handle: i64) -> Option<i64> {
    if handle <= 0 {
        observe::record_str_len_route_miss();
        trace_observer_resolution(
            "observer",
            handle,
            "none",
            "invalid_handle",
            format_args!("invalid_handle"),
        );
        return None;
    }
    if let Some(cached) = string_len_fast_cache_lookup(handle) {
        observe::record_str_len_route_fast_str_hit();
        if observe::len_route_matches_latest_fresh_handle(handle) {
            observe::record_str_len_route_latest_fresh_handle_fast_str_hit();
        }
        trace_observer_resolution(
            "observer",
            handle,
            "fast_hit",
            "len_handle_cache",
            format_args!("len={}", cached),
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
        trace_observer_resolution(
            "observer",
            handle,
            "fast_hit",
            "live_object_fast",
            format_args!("len={}", view_len),
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
        trace_observer_resolution(
            "observer",
            handle,
            "fast_hit",
            "as_str_fast",
            format_args!("len={}", fast_len.unwrap_or_default()),
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
    trace_observer_resolution(
        "observer",
        handle,
        if fallback.is_some() {
            "fallback_hit"
        } else {
            "fallback_miss"
        },
        "string_len_impl",
        format_args!("len={}", fallback.unwrap_or_default()),
    );
    fallback
}

pub(crate) fn string_is_empty_from_handle(handle: i64) -> Option<bool> {
    if handle <= 0 {
        trace_observer_resolution(
            "observer",
            handle,
            "none",
            "invalid_handle",
            format_args!("invalid_handle"),
        );
        return None;
    }
    if let Some(view_len) = string_len_fast_cache_lookup(handle) {
        let empty = view_len == 0;
        trace_observer_resolution(
            "observer",
            handle,
            "fast_hit",
            "live_object_fast",
            format_args!("empty={}", empty),
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
        trace_observer_resolution(
            "observer",
            handle,
            "fast_hit",
            "live_object_fast",
            format_args!("empty={}", empty),
        );
        return Some(empty);
    }
    let fast_len = handles::with_text_read_session(|session| {
        session.str_handle(handle as u64, |text| text.len() as i64)
    });
    if fast_len.is_some() {
        let empty = fast_len.unwrap_or_default() == 0;
        string_len_fast_cache_store(handle, fast_len.unwrap_or_default());
        trace_observer_resolution(
            "observer",
            handle,
            "fast_hit",
            "as_str_fast",
            format_args!("empty={}", empty),
        );
        return Some(empty);
    }
    let fallback = string_is_empty_impl(handle);
    trace_observer_resolution(
        "observer",
        handle,
        if fallback.is_some() {
            "fallback_hit"
        } else {
            "fallback_miss"
        },
        "string_is_empty_impl",
        format_args!("empty={}", fallback.unwrap_or(false)),
    );
    fallback
}

#[inline(always)]
pub(super) fn string_handle_from_owned(value: String) -> i64 {
    let len = value.len();
    if len == 0 {
        return shared_empty_string_handle();
    }
    observe::record_birth_placement_fresh_handle();
    let handle = materialize_owned_string(value);
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
    observe::record_birth_placement_freeze_owned();
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
    string_handle_from_owned(plan.into_owned())
}

#[inline(always)]
fn concat_two_str(a: &str, b: &str) -> String {
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
fn concat_three_str(a: &str, b: &str, c: &str) -> String {
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
fn shared_empty_string_handle() -> i64 {
    #[cfg(test)]
    {
        string_handle_from_owned(String::new())
    }
    #[cfg(not(test))]
    {
        static HANDLE: std::sync::OnceLock<i64> = std::sync::OnceLock::new();
        *HANDLE.get_or_init(|| {
            handles::to_handle_arc(std::sync::Arc::new(nyash_rust::box_trait::StringBox::new(
                String::new(),
            ))) as i64
        })
    }
}

#[inline(always)]
fn text_plan_shape(plan: &TextPlan<'_>) -> &'static str {
    match plan {
        TextPlan::View1(_) => "view1",
        TextPlan::Pieces2 { .. } => "pieces2",
        TextPlan::Pieces3 { .. } => "pieces3",
        TextPlan::Pieces4 { .. } => "pieces4",
        TextPlan::OwnedTmp(_) => "owned_tmp",
    }
}

#[inline(always)]
fn text_plan_piece_count(plan: &TextPlan<'_>) -> usize {
    match plan {
        TextPlan::View1(_) => 1,
        TextPlan::Pieces2 { .. } => 2,
        TextPlan::Pieces3 { .. } => 3,
        TextPlan::Pieces4 { .. } => 4,
        TextPlan::OwnedTmp(_) => 1,
    }
}

#[inline(always)]
fn text_plan_total_len(plan: &TextPlan<'_>) -> usize {
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
    extra: impl std::fmt::Display,
) {
    if !string_trace::enabled() {
        return;
    }
    string_trace::emit(
        stage,
        result,
        reason,
        format_args!("handle={} {}", handle, extra),
    );
}

fn concat_to_string_handle(parts: &[&str]) -> i64 {
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

enum Concat3Plan<'a> {
    ReuseHandle(i64),
    Materialize(TextPlan<'a>),
}

#[inline(always)]
fn freeze_concat3_plan<'a>(plan: Concat3Plan<'a>) -> i64 {
    match plan {
        Concat3Plan::ReuseHandle(handle) => {
            observe::record_birth_placement_return_handle();
            if string_trace::enabled() {
                string_trace::emit(
                    "sink",
                    "reuse_handle",
                    "concat3_reuse",
                    format_args!("handle={}", handle),
                );
            }
            handle
        }
        Concat3Plan::Materialize(value) => {
            if string_trace::enabled() {
                string_trace::emit(
                    "sink",
                    "freeze_plan",
                    "concat3_materialize",
                    format_args!(
                        "plan_shape={} piece_count={} total_len={}",
                        text_plan_shape(&value),
                        text_plan_piece_count(&value),
                        text_plan_total_len(&value)
                    ),
                );
            }
            freeze_text_plan(value)
        }
    }
}

#[inline(always)]
fn concat3_plan_from_parts<'a>(
    a_h: i64,
    b_h: i64,
    c_h: i64,
    a: &'a str,
    b: &'a str,
    c: &'a str,
    allow_handle_reuse: bool,
) -> Concat3Plan<'a> {
    let placement =
        concat3_retention_class(a.is_empty(), b.is_empty(), c.is_empty(), allow_handle_reuse);
    debug_assert!(!matches!(placement, RetainedForm::RetainView));
    if a.is_empty() {
        if b.is_empty() {
            return if allow_handle_reuse {
                Concat3Plan::ReuseHandle(c_h)
            } else {
                Concat3Plan::Materialize(TextPlan::from_two(
                    TextPiece::Inline(b),
                    TextPiece::Inline(c),
                ))
            };
        }
        if c.is_empty() {
            return if allow_handle_reuse {
                Concat3Plan::ReuseHandle(b_h)
            } else {
                Concat3Plan::Materialize(TextPlan::from_two(
                    TextPiece::Inline(b),
                    TextPiece::Inline(c),
                ))
            };
        }
        return Concat3Plan::Materialize(TextPlan::from_two(
            TextPiece::Inline(b),
            TextPiece::Inline(c),
        ));
    }
    if b.is_empty() {
        if c.is_empty() {
            return if allow_handle_reuse {
                Concat3Plan::ReuseHandle(a_h)
            } else {
                Concat3Plan::Materialize(TextPlan::from_two(
                    TextPiece::Inline(a),
                    TextPiece::Inline(c),
                ))
            };
        }
        return Concat3Plan::Materialize(TextPlan::from_two(
            TextPiece::Inline(a),
            TextPiece::Inline(c),
        ));
    }
    if c.is_empty() {
        return Concat3Plan::Materialize(TextPlan::from_two(
            TextPiece::Inline(a),
            TextPiece::Inline(b),
        ));
    }
    Concat3Plan::Materialize(TextPlan::from_three(
        TextPiece::Inline(a),
        TextPiece::Inline(b),
        TextPiece::Inline(c),
    ))
}

enum ConcatFastPath {
    ReuseHandle(i64),
    Owned(String),
}

#[inline(always)]
fn concat3_plan_from_fast_str(a_h: i64, b_h: i64, c_h: i64) -> Option<i64> {
    if a_h <= 0 || b_h <= 0 || c_h <= 0 {
        return None;
    }
    let plan = handles::with_text_read_session(|session| {
        session.str3(a_h as u64, b_h as u64, c_h as u64, |a, b, c| {
            let placement = concat3_retention_class(a.is_empty(), b.is_empty(), c.is_empty(), true);
            debug_assert!(!matches!(placement, RetainedForm::RetainView));
            if a.is_empty() {
                if b.is_empty() {
                    return ConcatFastPath::ReuseHandle(c_h);
                }
                if c.is_empty() {
                    return ConcatFastPath::ReuseHandle(b_h);
                }
                return ConcatFastPath::Owned(concat_two_str(b, c));
            }
            if b.is_empty() {
                if c.is_empty() {
                    return ConcatFastPath::ReuseHandle(a_h);
                }
                return ConcatFastPath::Owned(concat_two_str(a, c));
            }
            if c.is_empty() {
                return ConcatFastPath::Owned(concat_two_str(a, b));
            }
            ConcatFastPath::Owned(concat_three_str(a, b, c))
        })
    })?;
    Some(match plan {
        ConcatFastPath::ReuseHandle(handle) => handle,
        ConcatFastPath::Owned(text) => string_handle_from_owned(text),
    })
}

#[inline(always)]
fn concat3_plan_from_spans(a_h: i64, b_h: i64, c_h: i64) -> Option<i64> {
    if a_h <= 0 || b_h <= 0 || c_h <= 0 {
        return None;
    }
    let Some((a_span, b_span, c_span)) = resolve_string_span_triplet_from_handles(a_h, b_h, c_h)
    else {
        return None;
    };
    if a_span.span_bytes_len() == 0 {
        if b_span.span_bytes_len() == 0 {
            return Some(c_h);
        }
        if c_span.span_bytes_len() == 0 {
            return Some(b_h);
        }
        return Some(freeze_concat3_plan(Concat3Plan::Materialize(
            TextPlan::from_two(TextPiece::Span(b_span), TextPiece::Span(c_span)),
        )));
    }
    if b_span.span_bytes_len() == 0 {
        if c_span.span_bytes_len() == 0 {
            return Some(a_h);
        }
        return Some(freeze_concat3_plan(Concat3Plan::Materialize(
            TextPlan::from_two(TextPiece::Span(a_span), TextPiece::Span(c_span)),
        )));
    }
    if c_span.span_bytes_len() == 0 {
        return Some(freeze_concat3_plan(Concat3Plan::Materialize(
            TextPlan::from_two(TextPiece::Span(a_span), TextPiece::Span(b_span)),
        )));
    }
    Some(freeze_concat3_plan(Concat3Plan::Materialize(
        TextPlan::from_three(
            TextPiece::Span(a_span),
            TextPiece::Span(b_span),
            TextPiece::Span(c_span),
        ),
    )))
}

#[inline(always)]
pub(crate) fn to_owned_string_handle_arg(h: i64) -> String {
    resolve_string_span_from_handle(h)
        .map(|span| span.as_str().to_string())
        .unwrap_or_default()
}

#[inline(always)]
fn concat_pair_from_spans(a_h: i64, b_h: i64) -> Option<i64> {
    let (a_span, b_span) = resolve_string_span_pair_from_handles(a_h, b_h)?;
    let a = a_span.as_str();
    let b = b_span.as_str();
    if a.is_empty() {
        observe::record_str_concat2_route_span_return_handle();
        observe::record_birth_placement_return_handle();
        return Some(b_h);
    }
    if b.is_empty() {
        observe::record_str_concat2_route_span_return_handle();
        observe::record_birth_placement_return_handle();
        return Some(a_h);
    }
    observe::record_str_concat2_route_span_freeze();
    Some(freeze_text_plan(TextPlan::from_two(
        TextPiece::Span(a_span),
        TextPiece::Span(b_span),
    )))
}

#[inline(always)]
fn concat_pair_from_fast_str(a_h: i64, b_h: i64) -> Option<i64> {
    if a_h <= 0 || b_h <= 0 {
        return None;
    }
    if let Some(cached) = concat_pair_fast_cache_lookup(a_h, b_h) {
        observe::record_str_concat2_route_fast_str_owned();
        observe::record_birth_placement_fresh_handle();
        return Some(issue_fresh_handle_from_arc(cached));
    }
    let plan = handles::with_text_read_session(|session| {
        session.str_pair(a_h as u64, b_h as u64, |a, b| {
            if a.is_empty() {
                return ConcatFastPath::ReuseHandle(b_h);
            }
            if b.is_empty() {
                return ConcatFastPath::ReuseHandle(a_h);
            }
            ConcatFastPath::Owned(concat_two_str(a, b))
        })
    })?;
    Some(match plan {
        ConcatFastPath::ReuseHandle(handle) => {
            observe::record_str_concat2_route_fast_str_return_handle();
            observe::record_birth_placement_return_handle();
            handle
        }
        ConcatFastPath::Owned(text) => {
            observe::record_str_concat2_route_fast_str_owned();
            let handle = string_handle_from_owned(text);
            if handle > 0 {
                if let Some(result) = handles::get(handle as u64) {
                    concat_pair_fast_cache_store(a_h, b_h, result);
                }
            }
            handle
        }
    })
}

#[inline(always)]
fn concat_pair_with_materialize(a_h: i64, b_h: i64) -> i64 {
    observe::record_str_concat2_route_materialize_fallback();
    let lhs = to_owned_string_handle_arg(a_h);
    let rhs = to_owned_string_handle_arg(b_h);
    let out = concat_to_string_handle(&[lhs.as_str(), rhs.as_str()]);
    stage1_string_debug_log_concat_materialize(a_h, b_h, out);
    out
}

#[inline(always)]
fn concat_pair_fallback(a_h: i64, b_h: i64) -> i64 {
    if let Some(out) = concat_pair_from_fast_str(a_h, b_h) {
        return out;
    }
    if let Some(out) = concat_pair_from_spans(a_h, b_h) {
        return out;
    }
    concat_pair_with_materialize(a_h, b_h)
}

#[inline(always)]
fn execute_concat2_freeze_from_text(a_h: i64, suffix: &str, placement: RetainedForm) -> i64 {
    observe::record_const_suffix_freeze_fallback();
    match placement {
        RetainedForm::ReturnHandle => {
            observe::record_birth_placement_return_handle();
            a_h
        }
        RetainedForm::KeepTransient | RetainedForm::MustFreeze(_) => {
            freeze_text_plan(concat_const_suffix_plan_from_handle(a_h, suffix))
        }
        RetainedForm::RetainView => unreachable!("concat_hs cannot retain a view"),
    }
}

#[inline(always)]
fn execute_const_suffix_contract(a_h: i64, suffix_ptr: *const i8) -> i64 {
    // phase-151x visibility lock:
    // `.hako const_suffix -> thaw.str + lit.str + str.concat2 + freeze.str`
    // is the public reading. This function is only the current Rust executor.
    if suffix_ptr.is_null() {
        return a_h;
    }
    observe::record_const_suffix_enter();
    with_cached_const_suffix_text(suffix_ptr, |suffix| {
        let suffix_is_empty = suffix.is_empty();
        let placement = concat_suffix_retention_class(suffix_is_empty);
        if matches!(placement, RetainedForm::ReturnHandle) {
            observe::record_const_suffix_empty_return();
            observe::record_birth_placement_return_handle();
            return a_h;
        }
        if let Some(hit) = concat_const_suffix_fast_cache_lookup(a_h, suffix_ptr) {
            observe::record_const_suffix_cached_fast_str_hit();
            observe::record_birth_placement_return_handle();
            return hit;
        }
        if let Some(handle) = handles::with_text_read_session(|session| {
            session.str_handle(a_h as u64, |lhs| {
                if let Some(hit) = concat_const_suffix_fast_cache_lookup(a_h, suffix_ptr) {
                    observe::record_const_suffix_cached_fast_str_hit();
                    observe::record_birth_placement_return_handle();
                    return hit;
                }
                observe::record_const_suffix_freeze_fallback();
                let out = concat_two_str(lhs, suffix);
                let handle = super::super::string_const_handle_from_text(&out);
                if handle > 0 {
                    concat_const_suffix_fast_cache_store(a_h, suffix_ptr, handle);
                }
                handle
            })
        }) {
            return handle;
        }
        execute_concat2_freeze_from_text(a_h, suffix, placement)
    })
}

#[inline(always)]
fn concat_const_suffix_fallback(a_h: i64, suffix_ptr: *const i8) -> i64 {
    // phase-149x: keep `concat_hs` as the current concrete executor path, but
    // read this route as `.hako const_suffix -> thaw.str + lit.str + str.concat2 + freeze.str`.
    execute_const_suffix_contract(a_h, suffix_ptr)
}

#[inline(always)]
fn insert_const_mid_fallback(source_h: i64, middle_ptr: *const i8, split: i64) -> i64 {
    #[derive(Default)]
    struct ConstCStringCache {
        ptr: Cell<usize>,
        handle: Cell<i64>,
        text: RefCell<Option<String>>,
    }

    fn with_cached_const_text<R>(
        cache: &'static LocalKey<ConstCStringCache>,
        ptr: *const i8,
        f: impl FnOnce(&str) -> R,
    ) -> R {
        if ptr.is_null() {
            return f("");
        }
        let addr = ptr as usize;
        cache.with(|cache| {
            if cache.ptr.get() != addr || cache.text.borrow().is_none() {
                let bytes = unsafe { CStr::from_ptr(ptr) }.to_bytes();
                let text = String::from_utf8_lossy(bytes).into_owned();
                cache.ptr.set(addr);
                *cache.text.borrow_mut() = Some(text);
            }
            let text_ref = cache.text.borrow();
            f(text_ref.as_deref().unwrap_or(""))
        })
    }

    thread_local! {
        static CONST_INSERT_TEXT_CACHE: ConstCStringCache = const { ConstCStringCache {
            ptr: Cell::new(0),
            handle: Cell::new(0),
            text: RefCell::new(None),
        } };
    }

    with_cached_const_text(&CONST_INSERT_TEXT_CACHE, middle_ptr, |middle| {
        let source_is_empty = string_is_empty_from_handle(source_h) == Some(true);
        match insert_middle_retention_class(source_is_empty, middle.is_empty()) {
            RetainedForm::ReturnHandle => source_h,
            RetainedForm::KeepTransient | RetainedForm::MustFreeze(_) => {
                if source_is_empty {
                    let addr = middle_ptr as usize;
                    CONST_INSERT_TEXT_CACHE.with(|cache| {
                        if cache.ptr.get() == addr {
                            let cached = cache.handle.get();
                            if cached > 0 {
                                return cached;
                            }
                        }
                        let handle = super::super::string_const_handle_from_text(middle);
                        if handle > 0 {
                            cache.ptr.set(addr);
                            cache.handle.set(handle);
                        }
                        handle
                    })
                } else {
                    freeze_text_plan(insert_const_mid_plan_from_handle(source_h, middle, split))
                }
            }
            RetainedForm::RetainView => {
                unreachable!("insert_hsi cannot retain a view")
            }
        }
    })
}

#[inline(always)]
fn concat3_fallback(a_h: i64, b_h: i64, c_h: i64) -> i64 {
    if let Some(plan) = concat3_plan_from_fast_str(a_h, b_h, c_h) {
        return plan;
    }
    if let Some(plan) = concat3_plan_from_spans(a_h, b_h, c_h) {
        return plan;
    }

    let a = to_owned_string_handle_arg(a_h);
    let b = to_owned_string_handle_arg(b_h);
    let c = to_owned_string_handle_arg(c_h);
    freeze_concat3_plan(concat3_plan_from_parts(
        a_h,
        b_h,
        c_h,
        a.as_str(),
        b.as_str(),
        c.as_str(),
        false,
    ))
}

#[inline(always)]
fn hako_string_dispatch(op: i64, a0: i64, a1: i64, a2: i64) -> Option<i64> {
    hako_forward_bridge::call_string_dispatch(op, a0, a1, a2)
}

#[inline(always)]
fn allow_rust_string_fallback() -> bool {
    hako_forward_bridge::rust_fallback_allowed()
}

#[inline(always)]
fn hook_miss_scalar_error(route: &str) -> i64 {
    hako_forward_bridge::hook_miss_error_code(route)
}

#[inline(always)]
fn hook_miss_freeze_handle(route: &str) -> i64 {
    hako_forward_bridge::hook_miss_freeze_handle(route)
}

#[inline(always)]
fn dispatch_or_fallback_concat_hh(a_h: i64, b_h: i64) -> i64 {
    observe::record_str_concat2_route_enter();
    if let Some(v) = hako_string_dispatch(hako_forward_bridge::string_ops::CONCAT_HH, a_h, b_h, 0) {
        observe::record_str_concat2_route_dispatch_hit();
        return v;
    }
    if !allow_rust_string_fallback() {
        return hook_miss_freeze_handle("string.concat_hh");
    }
    concat_pair_fallback(a_h, b_h)
}

#[inline(always)]
fn dispatch_or_fallback_concat3_hhh(a_h: i64, b_h: i64, c_h: i64) -> i64 {
    if let Some(cached) = concat3_fast_cache_lookup(a_h, b_h, c_h) {
        return cached;
    }
    if let Some(v) =
        hako_string_dispatch(hako_forward_bridge::string_ops::CONCAT3_HHH, a_h, b_h, c_h)
    {
        concat3_fast_cache_store(a_h, b_h, c_h, v);
        return v;
    }
    if !allow_rust_string_fallback() {
        return hook_miss_freeze_handle("string.concat3_hhh");
    }
    let v = concat3_fallback(a_h, b_h, c_h);
    if v > 0 {
        concat3_fast_cache_store(a_h, b_h, c_h, v);
    }
    v
}

pub(super) fn string_len_export_impl(handle: i64) -> i64 {
    observe::record_str_len_route_enter();
    if let Some(v) = hako_string_dispatch(hako_forward_bridge::string_ops::LEN_H, handle, 0, 0) {
        observe::record_str_len_route_dispatch_hit();
        return v;
    }
    if !allow_rust_string_fallback() {
        return hook_miss_scalar_error("string.len_h");
    }
    if jit_trace_len_enabled() {
        let present = if handle > 0 {
            handles::get(handle as u64).is_some()
        } else {
            false
        };
        eprintln!(
            "[AOT-LEN_H] string.len_h handle={} present={}",
            handle, present
        );
    }
    string_len_from_handle(handle).unwrap_or(0)
}

pub(super) fn string_length_from_ptr(ptr: *const i8, _mode: i64) -> i64 {
    if ptr.is_null() {
        return 0;
    }
    let c = unsafe { CStr::from_ptr(ptr) };
    c.to_bytes().len() as i64
}

pub(super) fn string_charcode_at_export_impl(handle: i64, idx: i64) -> i64 {
    if let Some(v) = hako_string_dispatch(
        hako_forward_bridge::string_ops::CHARCODE_AT_H,
        handle,
        idx,
        0,
    ) {
        return v;
    }
    if !allow_rust_string_fallback() {
        return hook_miss_scalar_error("string.charCodeAt_h");
    }
    if idx < 0 {
        return -1;
    }
    if let Some(span) = resolve_string_span_from_handle(handle) {
        let s = span.as_str();
        let i = idx as usize;
        if i < s.len() {
            return s.as_bytes()[i] as i64;
        }
    }
    -1
}

pub(super) fn string_concat_hh_export_impl(a_h: i64, b_h: i64) -> i64 {
    dispatch_or_fallback_concat_hh(a_h, b_h)
}

pub(super) fn string_concat_hs_export_impl(a_h: i64, suffix_ptr: *const i8) -> i64 {
    concat_const_suffix_fallback(a_h, suffix_ptr)
}

pub(super) fn string_insert_hsi_export_impl(
    source_h: i64,
    middle_ptr: *const i8,
    split: i64,
) -> i64 {
    insert_const_mid_fallback(source_h, middle_ptr, split)
}

pub(super) fn string_concat3_hhh_export_impl(a_h: i64, b_h: i64, c_h: i64) -> i64 {
    dispatch_or_fallback_concat3_hhh(a_h, b_h, c_h)
}

pub(super) fn string_eq_hh_export_impl(a_h: i64, b_h: i64) -> i64 {
    if let Some(v) = hako_string_dispatch(hako_forward_bridge::string_ops::EQ_HH, a_h, b_h, 0) {
        return v;
    }
    if !allow_rust_string_fallback() {
        return hook_miss_scalar_error("string.eq_hh");
    }
    let result = compare_string_pair_hh(a_h, b_h, |lhs, rhs| lhs == rhs);
    stage1_string_debug_log_eq(a_h, b_h, result);
    result
}

pub(super) fn string_substring_hii_export_impl(h: i64, start: i64, end: i64) -> i64 {
    if h <= 0 {
        return 0;
    }
    let view_enabled = substring_view_enabled();
    let fallback_allowed = allow_rust_string_fallback();
    if fallback_allowed {
        if let Some(hit) = substring_fast_cache_lookup(h, start, end, view_enabled) {
            return hit;
        }
    }
    if let Some(v) = hako_string_dispatch(
        hako_forward_bridge::string_ops::SUBSTRING_HII,
        h,
        start,
        end,
    ) {
        substring_fast_cache_store(h, start, end, view_enabled, v);
        return v;
    }
    if !fallback_allowed {
        return hook_miss_freeze_handle("string.substring_hii");
    }
    let Some(plan) = borrowed_substring_plan_from_handle(h, start, end, view_enabled) else {
        return shared_empty_string_handle();
    };
    let result = match plan {
        BorrowedSubstringPlan::ReturnHandle => {
            observe::record_birth_placement_return_handle();
            h
        }
        BorrowedSubstringPlan::ReturnEmpty => shared_empty_string_handle(),
        BorrowedSubstringPlan::FreezeSpan(span) => string_handle_from_span(span),
        BorrowedSubstringPlan::ViewSpan(span) => {
            observe::record_birth_placement_borrow_view();
            handles::to_handle_arc(std::sync::Arc::new(span.into_view_box())) as i64
        }
    };
    if result > 0 {
        substring_fast_cache_store(h, start, end, view_enabled, result);
    }
    result
}

pub(super) fn string_indexof_hh_export_impl(h: i64, n: i64) -> i64 {
    if let Some(v) = hako_string_dispatch(hako_forward_bridge::string_ops::INDEXOF_HH, h, n, 0) {
        return v;
    }
    if !allow_rust_string_fallback() {
        return hook_miss_scalar_error("string.indexOf_hh");
    }
    search_string_pair_hh(h, n, empty_needle_indexof, find_substr_byte_index)
}

pub(super) fn string_lastindexof_hh_export_impl(h: i64, n: i64) -> i64 {
    if let Some(v) = hako_string_dispatch(hako_forward_bridge::string_ops::LASTINDEXOF_HH, h, n, 0)
    {
        return v;
    }
    if !allow_rust_string_fallback() {
        return hook_miss_scalar_error("string.lastIndexOf_hh");
    }
    search_string_pair_hh(h, n, empty_needle_lastindexof, rfind_substr_byte_index)
}

pub(super) fn string_lt_hh_export_impl(a_h: i64, b_h: i64) -> i64 {
    if let Some(v) = hako_string_dispatch(hako_forward_bridge::string_ops::LT_HH, a_h, b_h, 0) {
        return v;
    }
    if !allow_rust_string_fallback() {
        return hook_miss_scalar_error("string.lt_hh");
    }
    compare_string_pair_hh(a_h, b_h, |lhs, rhs| lhs < rhs)
}

pub(super) fn string_from_u64x2_export_impl(lo: i64, hi: i64, len: i64) -> i64 {
    if let Some(v) = hako_string_dispatch(hako_forward_bridge::string_ops::FROM_U64X2, lo, hi, len)
    {
        return v;
    }
    if !allow_rust_string_fallback() {
        return hook_miss_freeze_handle("string.from_u64x2");
    }
    let l = if len < 0 {
        0
    } else {
        core::cmp::min(len as usize, 16)
    };
    let mut bytes: Vec<u8> = Vec::with_capacity(l);
    let lo_u = lo as u64;
    let hi_u = hi as u64;
    for i in 0..l.min(8) {
        bytes.push(((lo_u >> (8 * i)) & 0xff) as u8);
    }
    for i in 0..l.saturating_sub(8) {
        bytes.push(((hi_u >> (8 * i)) & 0xff) as u8);
    }
    let s = String::from_utf8_lossy(&bytes).to_string();
    string_handle_from_owned(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nyash_rust::box_trait::{NyashBox, StringBox};
    use nyash_rust::runtime::host_handles as handles;
    use std::{ffi::CString, sync::Arc};

    #[test]
    fn concat_pair_fast_cache_invalidates_on_drop_epoch() {
        let lhs: Arc<dyn NyashBox> = Arc::new(StringBox::new("lhs-cache".to_string()));
        let rhs: Arc<dyn NyashBox> = Arc::new(StringBox::new("rhs-cache".to_string()));
        let result: Arc<dyn NyashBox> = Arc::new(StringBox::new("out-cache".to_string()));
        let lhs_h = handles::to_handle_arc(lhs) as i64;
        let rhs_h = handles::to_handle_arc(rhs) as i64;

        concat_pair_fast_cache_store(lhs_h, rhs_h, result.clone());
        assert!(concat_pair_fast_cache_lookup(lhs_h, rhs_h).is_some());

        handles::drop_handle(lhs_h as u64);
        assert!(concat_pair_fast_cache_lookup(lhs_h, rhs_h).is_none());
    }

    #[test]
    fn const_suffix_fast_cache_invalidates_on_drop_epoch() {
        let source: Arc<dyn NyashBox> = Arc::new(StringBox::new("source-cache".to_string()));
        let source_h = handles::to_handle_arc(source) as i64;
        let suffix = CString::new("xy").expect("CString");
        let suffix_ptr = suffix.as_ptr();

        concat_const_suffix_fast_cache_store(source_h, suffix_ptr, 77);
        assert_eq!(
            concat_const_suffix_fast_cache_lookup(source_h, suffix_ptr),
            Some(77)
        );

        handles::drop_handle(source_h as u64);
        assert_eq!(
            concat_const_suffix_fast_cache_lookup(source_h, suffix_ptr),
            None
        );
    }

    #[test]
    fn substring_fast_cache_invalidates_on_drop_epoch() {
        let source: Arc<dyn NyashBox> = Arc::new(StringBox::new("substring-cache".to_string()));
        let source_h = handles::to_handle_arc(source) as i64;

        substring_fast_cache_store(source_h, 2, 6, false, 88);
        assert_eq!(substring_fast_cache_lookup(source_h, 2, 6, false), Some(88));

        handles::drop_handle(source_h as u64);
        assert_eq!(substring_fast_cache_lookup(source_h, 2, 6, false), None);
    }

    #[test]
    fn substring_fast_cache_keeps_two_recent_slices_hot() {
        let source: Arc<dyn NyashBox> = Arc::new(StringBox::new("substring-cache".to_string()));
        let source_h = handles::to_handle_arc(source) as i64;

        substring_fast_cache_store(source_h, 0, 4, true, 101);
        substring_fast_cache_store(source_h, 4, 8, true, 202);

        assert_eq!(substring_fast_cache_lookup(source_h, 0, 4, true), Some(101));
        assert_eq!(substring_fast_cache_lookup(source_h, 4, 8, true), Some(202));
    }

    #[test]
    fn string_len_fast_cache_keeps_two_recent_handles_hot() {
        let a: Arc<dyn NyashBox> = Arc::new(StringBox::new("abcd".to_string()));
        let b: Arc<dyn NyashBox> = Arc::new(StringBox::new("ef".to_string()));
        let a_h = handles::to_handle_arc(a) as i64;
        let b_h = handles::to_handle_arc(b) as i64;

        string_len_fast_cache_store(a_h, 4);
        string_len_fast_cache_store(b_h, 2);

        assert_eq!(string_len_fast_cache_lookup(a_h), Some(4));
        assert_eq!(string_len_fast_cache_lookup(b_h), Some(2));
    }
}
