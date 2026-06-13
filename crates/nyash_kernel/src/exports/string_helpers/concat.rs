#[path = "concat/common.rs"]
pub(super) mod common;
#[path = "concat/const_adapter.rs"]
pub(super) mod const_adapter;
#[path = "concat/piecewise.rs"]
pub(crate) mod piecewise;
#[path = "concat/substring.rs"]
pub(crate) mod substring;

use super::cache::{concat_pair_fast_cache_lookup, concat_pair_fast_cache_store};
use super::materialize::{
    concat_two_str, freeze_text_plan_with_site, string_handle_from_owned_with_site,
    to_owned_string_handle_arg,
};
use crate::exports::string_birth_placement::{concat3_retention_class, RetainedForm};
use crate::exports::string_debug::stage1_string_debug_log_concat_materialize;
use crate::exports::string_plan::{TextPiece, TextPlan};
use crate::exports::string_trace;
use crate::exports::string_view::{
    resolve_string_span_pair_from_handles, resolve_string_span_triplet_from_handles,
};
use crate::observe;
use crate::plugin::{issue_fresh_handle, StringPublishSite};
use nyash_rust::runtime::host_handles as handles;

enum ConcatFastPath {
    ReuseHandle(i64),
    Owned(String),
}

#[inline(always)]
pub(crate) fn concat_pair_fallback(a_h: i64, b_h: i64) -> i64 {
    if a_h > 0 && b_h > 0 {
        if let Some(cached) = concat_pair_fast_cache_lookup(a_h, b_h) {
            observe::record_str_concat2_route_fast_str_owned();
            observe::record_birth_placement_fresh_handle();
            return issue_fresh_handle(cached);
        }
        if let Some(out) = handles::with_text_read_session(|session| {
            session.str_pair(a_h as u64, b_h as u64, |a, b| {
                if a.is_empty() {
                    return ConcatFastPath::ReuseHandle(b_h);
                }
                if b.is_empty() {
                    return ConcatFastPath::ReuseHandle(a_h);
                }
                ConcatFastPath::Owned(concat_two_str(a, b))
            })
        })
        .map(|plan| match plan {
            ConcatFastPath::ReuseHandle(handle) => {
                observe::record_str_concat2_route_fast_str_return_handle();
                observe::record_birth_placement_return_handle();
                handle
            }
            ConcatFastPath::Owned(text) => {
                observe::record_str_concat2_route_fast_str_owned();
                let handle =
                    string_handle_from_owned_with_site(text, StringPublishSite::StringConcatHh);
                if handle > 0 {
                    if let Some(result) = handles::with_handle(handle as u64, |obj| obj.cloned()) {
                        concat_pair_fast_cache_store(a_h, b_h, result);
                    }
                }
                handle
            }
        }) {
            return out;
        }
    }
    if let Some((a_span, b_span)) = resolve_string_span_pair_from_handles(a_h, b_h) {
        let a = a_span.as_text();
        let b = b_span.as_text();
        if a.is_empty() {
            observe::record_str_concat2_route_span_return_handle();
            observe::record_birth_placement_return_handle();
            return b_h;
        }
        if b.is_empty() {
            observe::record_str_concat2_route_span_return_handle();
            observe::record_birth_placement_return_handle();
            return a_h;
        }
        observe::record_str_concat2_route_span_freeze();
        return freeze_text_plan_with_site(
            TextPlan::from_two(TextPiece::Span(a_span), TextPiece::Span(b_span)),
            StringPublishSite::Generic,
        );
    }
    observe::record_str_concat2_route_materialize_fallback();
    let lhs = to_owned_string_handle_arg(a_h);
    let rhs = to_owned_string_handle_arg(b_h);
    let out = string_handle_from_owned_with_site(
        concat_two_str(lhs.as_str(), rhs.as_str()),
        StringPublishSite::Generic,
    );
    stage1_string_debug_log_concat_materialize(a_h, b_h, out);
    out
}

#[inline(always)]
pub(crate) fn concat3_fallback(a_h: i64, b_h: i64, c_h: i64) -> i64 {
    let freeze_plan = |value: TextPlan<'_>| -> i64 {
        if string_trace::enabled() {
            let (shape, piece_count, total_len) = value.trace_shape();
            string_trace::emit(
                "sink",
                "freeze_plan",
                "concat3_materialize",
                format_args!(
                    "plan_shape={} piece_count={} total_len={}",
                    shape, piece_count, total_len
                ),
            );
        }
        freeze_text_plan_with_site(value, StringPublishSite::Generic)
    };
    if a_h > 0 && b_h > 0 && c_h > 0 {
        if let Some(plan) = handles::with_text_read_session(|session| {
            session.str3(a_h as u64, b_h as u64, c_h as u64, |a, b, c| {
                let placement =
                    concat3_retention_class(a.is_empty(), b.is_empty(), c.is_empty(), true);
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
                let total = a.len() + b.len() + c.len();
                let mut out = String::with_capacity(total);
                unsafe {
                    let buf = out.as_mut_vec();
                    buf.set_len(total);
                    let a_len = a.len();
                    let b_len = b.len();
                    std::ptr::copy_nonoverlapping(a.as_ptr(), buf.as_mut_ptr(), a_len);
                    std::ptr::copy_nonoverlapping(b.as_ptr(), buf.as_mut_ptr().add(a_len), b_len);
                    std::ptr::copy_nonoverlapping(
                        c.as_ptr(),
                        buf.as_mut_ptr().add(a_len + b_len),
                        c.len(),
                    );
                }
                ConcatFastPath::Owned(out)
            })
        })
        .map(|plan| match plan {
            ConcatFastPath::ReuseHandle(handle) => handle,
            ConcatFastPath::Owned(text) => {
                string_handle_from_owned_with_site(text, StringPublishSite::StringConcatHh)
            }
        }) {
            return plan;
        }
    }
    if a_h > 0 && b_h > 0 && c_h > 0 {
        if let Some((a_span, b_span, c_span)) =
            resolve_string_span_triplet_from_handles(a_h, b_h, c_h)
        {
            if a_span.span_bytes_len() == 0 {
                if b_span.span_bytes_len() == 0 {
                    return c_h;
                }
                if c_span.span_bytes_len() == 0 {
                    return b_h;
                }
                return freeze_plan(TextPlan::from_two(
                    TextPiece::Span(b_span),
                    TextPiece::Span(c_span),
                ));
            }
            if b_span.span_bytes_len() == 0 {
                if c_span.span_bytes_len() == 0 {
                    return a_h;
                }
                return freeze_plan(TextPlan::from_two(
                    TextPiece::Span(a_span),
                    TextPiece::Span(c_span),
                ));
            }
            if c_span.span_bytes_len() == 0 {
                return freeze_plan(TextPlan::from_two(
                    TextPiece::Span(a_span),
                    TextPiece::Span(b_span),
                ));
            }
            return freeze_plan(TextPlan::from_three(
                TextPiece::Span(a_span),
                TextPiece::Span(b_span),
                TextPiece::Span(c_span),
            ));
        }
    }

    let a = to_owned_string_handle_arg(a_h);
    let b = to_owned_string_handle_arg(b_h);
    let c = to_owned_string_handle_arg(c_h);
    let placement = concat3_retention_class(a.is_empty(), b.is_empty(), c.is_empty(), false);
    debug_assert!(!matches!(placement, RetainedForm::RetainView));
    let plan = if a.is_empty() {
        if b.is_empty() {
            TextPlan::from_two(TextPiece::Inline(b.as_str()), TextPiece::Inline(c.as_str()))
        } else if c.is_empty() {
            TextPlan::from_two(TextPiece::Inline(b.as_str()), TextPiece::Inline(c.as_str()))
        } else {
            TextPlan::from_two(TextPiece::Inline(b.as_str()), TextPiece::Inline(c.as_str()))
        }
    } else if b.is_empty() {
        if c.is_empty() {
            TextPlan::from_two(TextPiece::Inline(a.as_str()), TextPiece::Inline(c.as_str()))
        } else {
            TextPlan::from_two(TextPiece::Inline(a.as_str()), TextPiece::Inline(c.as_str()))
        }
    } else if c.is_empty() {
        TextPlan::from_two(TextPiece::Inline(a.as_str()), TextPiece::Inline(b.as_str()))
    } else {
        TextPlan::from_three(
            TextPiece::Inline(a.as_str()),
            TextPiece::Inline(b.as_str()),
            TextPiece::Inline(c.as_str()),
        )
    };
    freeze_plan(plan)
}
