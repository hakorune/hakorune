// String-related C ABI exports.

use super::string_birth_placement::{
    concat3_retention_class, concat_suffix_retention_class, insert_middle_retention_class,
    TextRetentionClass,
};
use super::string_debug::{
    jit_trace_len_enabled, stage1_string_debug_log_concat_materialize, stage1_string_debug_log_eq,
    substring_view_enabled,
};
use super::string_plan::{
    concat_const_suffix_plan_from_handle, insert_const_mid_plan_from_handle, TextPiece, TextPlan,
};
use super::string_search::{
    compare_string_pair_hh, empty_needle_indexof, empty_needle_lastindexof, find_substr_byte_index,
    rfind_substr_byte_index, search_string_pair_hh,
};
use super::string_view::{
    borrowed_substring_plan_from_handle, resolve_string_span_from_handle,
    resolve_string_span_pair_from_handles, resolve_string_span_triplet_from_handles,
    string_is_empty_from_handle as string_is_empty_impl, string_len_from_handle as string_len_impl,
    BorrowedSubstringPlan,
};
use crate::hako_forward_bridge;
use crate::plugin::materialize_owned_string;
use nyash_rust::runtime::host_handles as handles;
use std::{
    cell::{Cell, RefCell},
    ffi::CStr,
    ptr,
    thread::LocalKey,
};

pub(crate) fn string_len_from_handle(handle: i64) -> Option<i64> {
    if handle <= 0 {
        return None;
    }
    let fast_len = handles::with_handle(handle as u64, |obj| {
        obj.and_then(|boxed| boxed.as_ref().as_str_fast().map(|s| s.len() as i64))
    });
    if fast_len.is_some() {
        return fast_len;
    }
    string_len_impl(handle)
}

pub(crate) fn string_is_empty_from_handle(handle: i64) -> Option<bool> {
    if handle <= 0 {
        return None;
    }
    let fast_empty = handles::with_handle(handle as u64, |obj| {
        obj.and_then(|boxed| boxed.as_ref().as_str_fast().map(str::is_empty))
    });
    if fast_empty.is_some() {
        return fast_empty;
    }
    string_is_empty_impl(handle)
}

fn string_handle_from_owned(value: String) -> i64 {
    materialize_owned_string(value)
}

#[inline(always)]
fn freeze_text_plan<'a>(plan: TextPlan<'a>) -> i64 {
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
        Concat3Plan::ReuseHandle(handle) => handle,
        Concat3Plan::Materialize(value) => freeze_text_plan(value),
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
    debug_assert!(!matches!(placement, TextRetentionClass::RetainView));
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
    let plan = handles::with_str3(a_h as u64, b_h as u64, c_h as u64, |a, b, c| {
        let placement =
            concat3_retention_class(a.is_empty(), b.is_empty(), c.is_empty(), true);
        debug_assert!(!matches!(placement, TextRetentionClass::RetainView));
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
    })?;
    Some(match plan {
        ConcatFastPath::ReuseHandle(handle) => handle,
        ConcatFastPath::Owned(text) => freeze_text_plan(TextPlan::from_owned(text)),
    })
}

#[inline(always)]
fn concat3_plan_from_spans(a_h: i64, b_h: i64, c_h: i64, allow_handle_reuse: bool) -> Option<i64> {
    if a_h <= 0 || b_h <= 0 || c_h <= 0 {
        return None;
    }
    let Some((a_span, b_span, c_span)) =
        resolve_string_span_triplet_from_handles(a_h, b_h, c_h)
    else {
        return None;
    };
    let a = a_span.as_str();
    let b = b_span.as_str();
    let c = c_span.as_str();
    let placement =
        concat3_retention_class(a.is_empty(), b.is_empty(), c.is_empty(), allow_handle_reuse);
    debug_assert!(!matches!(placement, TextRetentionClass::RetainView));
    if a.is_empty() {
        if b.is_empty() {
            return if allow_handle_reuse {
                Some(c_h)
            } else {
                Some(freeze_concat3_plan(Concat3Plan::Materialize(
                    TextPlan::from_two(TextPiece::Span(b_span), TextPiece::Inline(c)),
                )))
            };
        }
        if c.is_empty() {
            return Some(if allow_handle_reuse {
                b_h
            } else {
                freeze_concat3_plan(Concat3Plan::Materialize(TextPlan::from_two(
                    TextPiece::Span(b_span),
                    TextPiece::Inline(c),
                )))
            });
        }
        return Some(freeze_concat3_plan(Concat3Plan::Materialize(
            TextPlan::from_two(TextPiece::Span(b_span), TextPiece::Span(c_span)),
        )));
    }
    if b.is_empty() {
        if c.is_empty() {
            return Some(if allow_handle_reuse {
                a_h
            } else {
                freeze_concat3_plan(Concat3Plan::Materialize(TextPlan::from_two(
                    TextPiece::Inline(a),
                    TextPiece::Inline(c),
                )))
            });
        }
        return Some(freeze_concat3_plan(Concat3Plan::Materialize(
            TextPlan::from_two(TextPiece::Span(a_span), TextPiece::Span(c_span)),
        )));
    }
    if c.is_empty() {
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
        return Some(b_h);
    }
    if b.is_empty() {
        return Some(a_h);
    }
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
    let plan = handles::with_str_pair(a_h as u64, b_h as u64, |a, b| {
        if a.is_empty() {
            return ConcatFastPath::ReuseHandle(b_h);
        }
        if b.is_empty() {
            return ConcatFastPath::ReuseHandle(a_h);
        }
        ConcatFastPath::Owned(concat_two_str(a, b))
    })?;
    Some(match plan {
        ConcatFastPath::ReuseHandle(handle) => handle,
        ConcatFastPath::Owned(text) => freeze_text_plan(TextPlan::from_owned(text)),
    })
}

#[inline(always)]
fn concat_pair_with_materialize(a_h: i64, b_h: i64) -> i64 {
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
fn concat_const_suffix_from_handle(a_h: i64, suffix: &str) -> i64 {
    match concat_suffix_retention_class(suffix.is_empty()) {
        TextRetentionClass::ReturnHandle => a_h,
        TextRetentionClass::KeepTransient | TextRetentionClass::MustFreeze(_) => {
            freeze_text_plan(concat_const_suffix_plan_from_handle(a_h, suffix))
        }
        TextRetentionClass::RetainView => unreachable!("concat_hs cannot retain a view"),
    }
}

#[inline(always)]
fn concat_const_suffix_fallback(a_h: i64, suffix_ptr: *const i8) -> i64 {
    #[derive(Default)]
    struct ConstCStringCache {
        ptr: Cell<usize>,
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
        static CONST_SUFFIX_TEXT_CACHE: ConstCStringCache = const { ConstCStringCache {
            ptr: Cell::new(0),
            text: RefCell::new(None),
        } };
    }

    if suffix_ptr.is_null() {
        return a_h;
    }
    with_cached_const_text(&CONST_SUFFIX_TEXT_CACHE, suffix_ptr, |suffix| {
        concat_const_suffix_from_handle(a_h, suffix)
    })
}

#[inline(always)]
fn insert_const_mid_fallback(source_h: i64, middle_ptr: *const i8, split: i64) -> i64 {
    #[derive(Default)]
    struct ConstCStringCache {
        ptr: Cell<usize>,
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
            text: RefCell::new(None),
        } };
    }

    with_cached_const_text(&CONST_INSERT_TEXT_CACHE, middle_ptr, |middle| {
        let source_is_empty = string_is_empty_from_handle(source_h) == Some(true);
        match insert_middle_retention_class(source_is_empty, middle.is_empty()) {
            TextRetentionClass::ReturnHandle => source_h,
            TextRetentionClass::KeepTransient | TextRetentionClass::MustFreeze(_) => {
                if source_is_empty {
                    super::nyash_box_from_i8_string_const(middle_ptr)
                } else {
                    freeze_text_plan(insert_const_mid_plan_from_handle(source_h, middle, split))
                }
            }
            TextRetentionClass::RetainView => {
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
    if let Some(plan) = concat3_plan_from_spans(a_h, b_h, c_h, true) {
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
    if let Some(v) = hako_string_dispatch(hako_forward_bridge::string_ops::CONCAT_HH, a_h, b_h, 0) {
        return v;
    }
    if !allow_rust_string_fallback() {
        return hook_miss_freeze_handle("string.concat_hh");
    }
    concat_pair_fallback(a_h, b_h)
}

#[inline(always)]
fn dispatch_or_fallback_concat3_hhh(a_h: i64, b_h: i64, c_h: i64) -> i64 {
    if let Some(v) =
        hako_string_dispatch(hako_forward_bridge::string_ops::CONCAT3_HHH, a_h, b_h, c_h)
    {
        return v;
    }
    if !allow_rust_string_fallback() {
        return hook_miss_freeze_handle("string.concat3_hhh");
    }
    concat3_fallback(a_h, b_h, c_h)
}

// String.len_h(handle) -> i64
#[export_name = "nyash.string.len_h"]
pub extern "C" fn nyash_string_len_h(handle: i64) -> i64 {
    if let Some(v) = hako_string_dispatch(hako_forward_bridge::string_ops::LEN_H, handle, 0, 0) {
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

// FAST-path helper: compute string length from raw pointer (i8*) with mode (reserved)
// Exported as both legacy name (nyash.string.length_si) and neutral name (nyrt_string_length)
#[export_name = "nyrt_string_length"]
pub extern "C" fn nyrt_string_length(ptr: *const i8, _mode: i64) -> i64 {
    use std::ffi::CStr;
    if ptr.is_null() {
        return 0;
    }
    // Safety: pointer is expected to point to a null-terminated UTF-8 byte string
    let c = unsafe { CStr::from_ptr(ptr) };
    match c.to_bytes().len() {
        n => n as i64,
    }
}

// String.charCodeAt_h(handle, idx) -> i64 (byte-based; -1 if OOB)
#[export_name = "nyash.string.charCodeAt_h"]
pub extern "C" fn nyash_string_charcode_at_h_export(handle: i64, idx: i64) -> i64 {
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

// String.concat_hh(lhs_h, rhs_h) -> handle
#[export_name = "nyash.string.concat_hh"]
pub extern "C" fn nyash_string_concat_hh_export(a_h: i64, b_h: i64) -> i64 {
    dispatch_or_fallback_concat_hh(a_h, b_h)
}

// String.concat_hs(lhs_h, const_suffix_ptr) -> handle
#[export_name = "nyash.string.concat_hs"]
pub extern "C" fn nyash_string_concat_hs_export(a_h: i64, suffix_ptr: *const i8) -> i64 {
    concat_const_suffix_fallback(a_h, suffix_ptr)
}

// String.insert_hsi(source_h, const_middle_ptr, split_i64) -> handle
#[export_name = "nyash.string.insert_hsi"]
pub extern "C" fn nyash_string_insert_hsi_export(
    source_h: i64,
    middle_ptr: *const i8,
    split: i64,
) -> i64 {
    insert_const_mid_fallback(source_h, middle_ptr, split)
}

// String.concat3_hhh(a_h, b_h, c_h) -> handle
#[export_name = "nyash.string.concat3_hhh"]
pub extern "C" fn nyash_string_concat3_hhh_export(a_h: i64, b_h: i64, c_h: i64) -> i64 {
    dispatch_or_fallback_concat3_hhh(a_h, b_h, c_h)
}

// String.eq_hh(lhs_h, rhs_h) -> i64 (0/1)
#[export_name = "nyash.string.eq_hh"]
pub extern "C" fn nyash_string_eq_hh_export(a_h: i64, b_h: i64) -> i64 {
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

// String.substring_hii(handle, start, end) -> handle
#[export_name = "nyash.string.substring_hii"]
pub extern "C" fn nyash_string_substring_hii_export(h: i64, start: i64, end: i64) -> i64 {
    if let Some(v) = hako_string_dispatch(
        hako_forward_bridge::string_ops::SUBSTRING_HII,
        h,
        start,
        end,
    ) {
        return v;
    }
    if !allow_rust_string_fallback() {
        return hook_miss_freeze_handle("string.substring_hii");
    }
    if h <= 0 {
        return 0;
    }
    let view_enabled = substring_view_enabled();
    let Some(plan) = borrowed_substring_plan_from_handle(h, start, end, view_enabled) else {
        return shared_empty_string_handle();
    };
    match plan {
        BorrowedSubstringPlan::ReturnHandle => h,
        BorrowedSubstringPlan::ReturnEmpty => shared_empty_string_handle(),
        BorrowedSubstringPlan::FreezePlan(plan) => freeze_text_plan(plan),
        BorrowedSubstringPlan::ViewSpan(span) => {
            handles::to_handle_arc(std::sync::Arc::new(span.into_view_box())) as i64
        }
    }
}

// String.indexOf_hh(haystack_h, needle_h) -> i64
#[export_name = "nyash.string.indexOf_hh"]
pub extern "C" fn nyash_string_indexof_hh_export(h: i64, n: i64) -> i64 {
    if let Some(v) = hako_string_dispatch(hako_forward_bridge::string_ops::INDEXOF_HH, h, n, 0) {
        return v;
    }
    if !allow_rust_string_fallback() {
        return hook_miss_scalar_error("string.indexOf_hh");
    }
    search_string_pair_hh(h, n, empty_needle_indexof, find_substr_byte_index)
}

// String.lastIndexOf_hh(haystack_h, needle_h) -> i64
#[export_name = "nyash.string.lastIndexOf_hh"]
pub extern "C" fn nyash_string_lastindexof_hh_export(h: i64, n: i64) -> i64 {
    if let Some(v) = hako_string_dispatch(hako_forward_bridge::string_ops::LASTINDEXOF_HH, h, n, 0)
    {
        return v;
    }
    if !allow_rust_string_fallback() {
        return hook_miss_scalar_error("string.lastIndexOf_hh");
    }
    search_string_pair_hh(h, n, empty_needle_lastindexof, rfind_substr_byte_index)
}

// String.lt_hh(lhs_h, rhs_h) -> i64 (0/1)
#[export_name = "nyash.string.lt_hh"]
pub extern "C" fn nyash_string_lt_hh_export(a_h: i64, b_h: i64) -> i64 {
    if let Some(v) = hako_string_dispatch(hako_forward_bridge::string_ops::LT_HH, a_h, b_h, 0) {
        return v;
    }
    if !allow_rust_string_fallback() {
        return hook_miss_scalar_error("string.lt_hh");
    }
    compare_string_pair_hh(a_h, b_h, |lhs, rhs| lhs < rhs)
}

// Construct StringBox from two u64 words (little-endian) + length (<=16) and return handle
// export: nyash.string.from_u64x2(lo, hi, len) -> i64
#[export_name = "nyash.string.from_u64x2"]
pub extern "C" fn nyash_string_from_u64x2_export(lo: i64, hi: i64, len: i64) -> i64 {
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
