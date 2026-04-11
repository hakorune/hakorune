use super::cache::{
    concat_const_suffix_fast_cache_lookup, concat_const_suffix_fast_cache_store,
    concat_pair_fast_cache_lookup, concat_pair_fast_cache_store, with_cached_const_suffix_text,
};
use super::materialize::{
    concat_three_str, concat_to_string_handle, concat_two_str, freeze_text_plan,
    shared_empty_string_handle, string_handle_from_owned, string_is_empty_from_handle,
    text_plan_piece_count, text_plan_shape, text_plan_total_len, to_owned_string_handle_arg,
};
use crate::exports::string_birth_placement::{
    concat3_retention_class, concat_suffix_retention_class, insert_middle_retention_class,
    RetainedForm,
};
use crate::exports::string_debug::stage1_string_debug_log_concat_materialize;
use crate::exports::string_literal_handle_from_text;
use crate::exports::string_plan::{
    concat_const_suffix_plan_from_handle, insert_const_mid_plan_from_handle, TextPiece, TextPlan,
};
use crate::exports::string_trace;
use crate::exports::string_view::{
    clamp_i64_range, resolve_string_span_pair_from_handles,
    resolve_string_span_triplet_from_handles,
};
use crate::observe;
use crate::plugin::issue_fresh_handle_from_arc;
use nyash_rust::runtime::host_handles as handles;
use std::{
    cell::{Cell, RefCell},
    ffi::CStr,
    thread::LocalKey,
};

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
            super::string_substring_hii_export_impl(handle, start, end)
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
pub(super) fn concat_pair_fallback(a_h: i64, b_h: i64) -> i64 {
    if let Some(out) = concat_pair_from_fast_str(a_h, b_h) {
        return out;
    }
    if let Some(out) = concat_pair_from_spans(a_h, b_h) {
        return out;
    }
    concat_pair_with_materialize(a_h, b_h)
}

#[inline(always)]
pub(super) fn concat_pair_substring_fallback(a_h: i64, b_h: i64, start: i64, end: i64) -> i64 {
    if let Some(path) = concat_pair_substring_path(a_h, b_h, start, end) {
        return freeze_concat_substring_path(path);
    }
    let concat_h = concat_pair_fallback(a_h, b_h);
    super::string_substring_hii_export_impl(concat_h, start, end)
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
        if let Some(plan) = handles::with_text_read_session(|session| {
            session.str_handle(a_h as u64, |lhs| {
                if let Some(hit) = concat_const_suffix_fast_cache_lookup(a_h, suffix_ptr) {
                    observe::record_const_suffix_cached_fast_str_hit();
                    observe::record_birth_placement_return_handle();
                    return ConcatFastPath::ReuseHandle(hit);
                }
                observe::record_const_suffix_freeze_fallback();
                ConcatFastPath::Owned(concat_two_str(lhs, suffix))
            })
        }) {
            return match plan {
                ConcatFastPath::ReuseHandle(handle) => handle,
                ConcatFastPath::Owned(text) => {
                    let handle = string_handle_from_owned(text);
                    if handle > 0 {
                        concat_const_suffix_fast_cache_store(a_h, suffix_ptr, handle);
                    }
                    handle
                }
            };
        }
        execute_concat2_freeze_from_text(a_h, suffix, placement)
    })
}

#[inline(always)]
pub(super) fn concat_const_suffix_fallback(a_h: i64, suffix_ptr: *const i8) -> i64 {
    // phase-149x: keep `concat_hs` as the current concrete executor path, but
    // read this route as `.hako const_suffix -> thaw.str + lit.str + str.concat2 + freeze.str`.
    execute_const_suffix_contract(a_h, suffix_ptr)
}

#[inline(always)]
pub(super) fn insert_const_mid_fallback(source_h: i64, middle_ptr: *const i8, split: i64) -> i64 {
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
                        let handle = string_literal_handle_from_text(middle);
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
pub(super) fn concat3_fallback(a_h: i64, b_h: i64, c_h: i64) -> i64 {
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
    let concat_h = concat3_fallback(a_h, b_h, c_h);
    super::string_substring_hii_export_impl(concat_h, start, end)
}
