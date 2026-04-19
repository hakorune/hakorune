#[path = "concat/const_adapter.rs"]
mod const_adapter;
#[path = "concat/piecewise.rs"]
mod piecewise;
#[path = "concat/substring.rs"]
mod substring;

use super::cache::{concat_pair_fast_cache_lookup, concat_pair_fast_cache_store};
use super::materialize::{
    concat_three_str, concat_to_string_handle, concat_two_str, freeze_text_plan,
    string_handle_from_owned, string_handle_from_owned_concat_hh, text_plan_piece_count,
    text_plan_shape, text_plan_total_len, to_owned_string_handle_arg,
};
use crate::exports::string_birth_placement::{concat3_retention_class, RetainedForm};
use crate::exports::string_debug::stage1_string_debug_log_concat_materialize;
use crate::exports::string_plan::{TextPiece, TextPlan};
use crate::exports::string_trace;
use crate::exports::string_view::{
    resolve_string_span_pair_from_handles, resolve_string_span_triplet_from_handles,
};
use crate::observe;
use crate::plugin::{
    freeze_owned_string_into_slot, issue_fresh_handle_from_arc, KernelTextSlot, TextRef,
};
use nyash_rust::runtime::host_handles as handles;

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
pub(super) fn concat_const_suffix_fallback(a_h: i64, suffix_ptr: *const i8) -> i64 {
    const_adapter::concat_const_suffix_fallback(a_h, suffix_ptr)
}

#[inline(always)]
pub(super) fn concat_const_suffix_into_slot(
    slot: &mut KernelTextSlot,
    a_h: i64,
    suffix_ptr: *const i8,
) -> bool {
    const_adapter::concat_const_suffix_into_slot(slot, a_h, suffix_ptr)
}

#[inline(always)]
pub(super) fn insert_const_mid_fallback(source_h: i64, middle_ptr: *const i8, split: i64) -> i64 {
    const_adapter::insert_const_mid_fallback(source_h, middle_ptr, split)
}

#[inline(always)]
pub(super) fn insert_const_mid_into_slot(
    slot: &mut KernelTextSlot,
    source_h: i64,
    middle_ptr: *const i8,
    split: i64,
) -> bool {
    const_adapter::insert_const_mid_into_slot(slot, source_h, middle_ptr, split)
}

#[inline(always)]
pub(super) fn piecewise_subrange_hsiii_fallback(
    source_h: i64,
    middle_ptr: *const i8,
    split: i64,
    start: i64,
    end: i64,
) -> i64 {
    piecewise::piecewise_subrange_hsiii_fallback(source_h, middle_ptr, split, start, end)
}

#[inline(always)]
pub(super) fn piecewise_subrange_hsiii_into_slot(
    out: &mut KernelTextSlot,
    source_h: i64,
    middle_ptr: *const i8,
    split: i64,
    start: i64,
    end: i64,
) -> bool {
    piecewise::piecewise_subrange_hsiii_into_slot(out, source_h, middle_ptr, split, start, end)
}

#[inline(always)]
pub(super) fn piecewise_subrange_kernel_text_slot_into_slot(
    out: &mut KernelTextSlot,
    source: &KernelTextSlot,
    middle_ptr: *const i8,
    split: i64,
    start: i64,
    end: i64,
) -> bool {
    piecewise::piecewise_subrange_kernel_text_slot_into_slot(
        out, source, middle_ptr, split, start, end,
    )
}

#[inline(always)]
pub(super) fn substring_kernel_text_slot_into_slot(
    out: &mut KernelTextSlot,
    source: &KernelTextSlot,
    start: i64,
    end: i64,
) -> bool {
    piecewise::substring_kernel_text_slot_into_slot(out, source, start, end)
}

#[inline(always)]
pub(super) fn substring_kernel_text_slot_in_place(
    slot: &mut KernelTextSlot,
    start: i64,
    end: i64,
) -> bool {
    piecewise::substring_kernel_text_slot_in_place(slot, start, end)
}

#[inline(always)]
fn concat_pair_owned_for_slot(a_h: i64, b_h: i64) -> String {
    if let Some(text) = handles::with_text_read_session(|session| {
        session.str_pair(a_h as u64, b_h as u64, |a, b| {
            if a.is_empty() {
                return b.to_owned();
            }
            if b.is_empty() {
                return a.to_owned();
            }
            concat_two_str(a, b)
        })
    }) {
        return text;
    }
    if let Some((a_span, b_span)) = resolve_string_span_pair_from_handles(a_h, b_h) {
        let a = a_span.as_text();
        let b = b_span.as_text();
        if a.is_empty() {
            return b.to_string();
        }
        if b.is_empty() {
            return a.to_string();
        }
        return concat_two_str(a.as_str(), b.as_str());
    }
    let lhs = to_owned_string_handle_arg(a_h);
    let rhs = to_owned_string_handle_arg(b_h);
    concat_two_str(lhs.as_str(), rhs.as_str())
}

#[inline(always)]
pub(super) fn concat_pair_into_slot(slot: &mut KernelTextSlot, a_h: i64, b_h: i64) -> bool {
    freeze_owned_string_into_slot(slot, concat_pair_owned_for_slot(a_h, b_h));
    true
}

#[inline(always)]
pub(super) fn concat_pair_substring_fallback(a_h: i64, b_h: i64, start: i64, end: i64) -> i64 {
    substring::concat_pair_substring_fallback(a_h, b_h, start, end)
}

#[inline(always)]
pub(super) fn concat3_substring_fallback(
    a_h: i64,
    b_h: i64,
    c_h: i64,
    start: i64,
    end: i64,
) -> i64 {
    substring::concat3_substring_fallback(a_h, b_h, c_h, start, end)
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
    let a = a_span.as_text();
    let b = b_span.as_text();
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
            let handle = string_handle_from_owned_concat_hh(text);
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
    let a = TextRef::new(a.as_str());
    let b = TextRef::new(b.as_str());
    let c = TextRef::new(c.as_str());
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
