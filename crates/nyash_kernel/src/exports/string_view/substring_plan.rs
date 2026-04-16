use crate::exports::string_birth_placement::{substring_retention_class, RetainedForm};
use crate::exports::string_trace;
use crate::observe;
use nyash_rust::{
    box_trait::{NyashBox, StringBox},
    runtime::host_handles as handles,
};
use std::sync::Arc;

use super::{clamp_i64_range, clamp_usize_range, BorrowedSubstringPlan, StringSpan, StringViewBox};

#[inline(always)]
fn emit_borrowed_substring_plan_trace(
    handle: i64,
    start: i64,
    end: i64,
    view_enabled: bool,
    result: &str,
    reason: &str,
    source_kind: &str,
    span_len: usize,
) {
    if !string_trace::enabled() {
        return;
    }
    string_trace::emit(
        "carrier",
        result,
        reason,
        format_args!(
            "handle={} start={} end={} view_enabled={} source_kind={} span_len={}",
            handle, start, end, view_enabled, source_kind, span_len
        ),
    );
}

#[inline(always)]
fn finalize_borrowed_substring_plan(
    handle: i64,
    start: i64,
    end: i64,
    view_enabled: bool,
    source_kind: &str,
    plan: BorrowedSubstringPlan,
    reason: &str,
    span_len: usize,
) -> Option<BorrowedSubstringPlan> {
    match &plan {
        BorrowedSubstringPlan::ReturnHandle => {
            observe::record_str_substring_route_slow_plan_return_handle();
            emit_borrowed_substring_plan_trace(
                handle,
                start,
                end,
                view_enabled,
                "return_handle",
                reason,
                source_kind,
                span_len,
            );
        }
        BorrowedSubstringPlan::ReturnEmpty => {
            observe::record_str_substring_route_slow_plan_return_empty();
            emit_borrowed_substring_plan_trace(
                handle,
                start,
                end,
                view_enabled,
                "return_empty",
                reason,
                source_kind,
                span_len,
            );
        }
        BorrowedSubstringPlan::FreezeSpan(_) => {
            observe::record_str_substring_route_slow_plan_freeze_span();
            emit_borrowed_substring_plan_trace(
                handle,
                start,
                end,
                view_enabled,
                "freeze_span",
                reason,
                source_kind,
                span_len,
            );
        }
        BorrowedSubstringPlan::ViewSpan(_) => {
            observe::record_str_substring_route_slow_plan_view_span();
            emit_borrowed_substring_plan_trace(
                handle,
                start,
                end,
                view_enabled,
                "view_span",
                reason,
                source_kind,
                span_len,
            );
        }
    }
    Some(plan)
}

#[inline(always)]
fn substring_plan_from_string_box(
    handle: i64,
    start: i64,
    end: i64,
    view_enabled: bool,
    obj: &Arc<dyn NyashBox>,
    sb: &StringBox,
) -> Option<BorrowedSubstringPlan> {
    let (st_rel, en_rel) = clamp_i64_range(sb.value.len(), start, end);
    if st_rel == 0 && en_rel == sb.value.len() {
        return finalize_borrowed_substring_plan(
            handle,
            start,
            end,
            view_enabled,
            "StringBox",
            BorrowedSubstringPlan::ReturnHandle,
            "root_full_span",
            en_rel.saturating_sub(st_rel),
        );
    }
    if st_rel == en_rel {
        return finalize_borrowed_substring_plan(
            handle,
            start,
            end,
            view_enabled,
            "StringBox",
            BorrowedSubstringPlan::ReturnEmpty,
            "root_empty_span",
            0,
        );
    }
    if sb.value.get(st_rel..en_rel).is_none() {
        return finalize_borrowed_substring_plan(
            handle,
            start,
            end,
            view_enabled,
            "StringBox",
            BorrowedSubstringPlan::ReturnEmpty,
            "root_out_of_range",
            en_rel.saturating_sub(st_rel),
        );
    }
    let span_len = en_rel - st_rel;
    let span = StringSpan {
        base_handle: handle,
        base_obj: obj.clone(),
        start: st_rel,
        end: en_rel,
    };
    match substring_retention_class(view_enabled, span_len) {
        RetainedForm::RetainView => finalize_borrowed_substring_plan(
            handle,
            start,
            end,
            view_enabled,
            "StringBox",
            BorrowedSubstringPlan::ViewSpan(span),
            "retain_view",
            span_len,
        ),
        RetainedForm::MustFreeze(_) | RetainedForm::KeepTransient => {
            finalize_borrowed_substring_plan(
                handle,
                start,
                end,
                view_enabled,
                "StringBox",
                BorrowedSubstringPlan::FreezeSpan(span),
                "must_freeze_or_keep",
                span_len,
            )
        }
        RetainedForm::ReturnHandle => finalize_borrowed_substring_plan(
            handle,
            start,
            end,
            view_enabled,
            "StringBox",
            BorrowedSubstringPlan::ReturnHandle,
            "placement_return_handle",
            span_len,
        ),
    }
}

#[inline(always)]
fn substring_plan_from_view_box(
    handle: i64,
    start: i64,
    end: i64,
    view_enabled: bool,
    view: &StringViewBox,
) -> Option<BorrowedSubstringPlan> {
    let base_sb = view.base_obj.as_any().downcast_ref::<StringBox>()?;
    let (parent_st, parent_en) = clamp_usize_range(base_sb.value.len(), view.start, view.end);
    let parent_len = parent_en.saturating_sub(parent_st);
    let (st_rel, en_rel) = clamp_i64_range(parent_len, start, end);
    if st_rel == 0 && en_rel == parent_len {
        return finalize_borrowed_substring_plan(
            handle,
            start,
            end,
            view_enabled,
            "StringViewBox",
            BorrowedSubstringPlan::ReturnHandle,
            "view_full_span",
            en_rel.saturating_sub(st_rel),
        );
    }
    if st_rel == en_rel {
        return finalize_borrowed_substring_plan(
            handle,
            start,
            end,
            view_enabled,
            "StringViewBox",
            BorrowedSubstringPlan::ReturnEmpty,
            "view_empty_span",
            0,
        );
    }
    let abs_st = parent_st.saturating_add(st_rel);
    let abs_en = parent_st.saturating_add(en_rel);
    if base_sb.value.get(abs_st..abs_en).is_none() {
        return finalize_borrowed_substring_plan(
            handle,
            start,
            end,
            view_enabled,
            "StringViewBox",
            BorrowedSubstringPlan::ReturnEmpty,
            "view_out_of_range",
            abs_en.saturating_sub(abs_st),
        );
    }
    let span_len = abs_en - abs_st;
    let span = StringSpan {
        base_handle: view.base_handle,
        base_obj: view.base_obj.clone(),
        start: abs_st,
        end: abs_en,
    };
    match substring_retention_class(view_enabled, span_len) {
        RetainedForm::RetainView => finalize_borrowed_substring_plan(
            handle,
            start,
            end,
            view_enabled,
            "StringViewBox",
            BorrowedSubstringPlan::ViewSpan(span),
            "retain_view",
            span_len,
        ),
        RetainedForm::MustFreeze(_) | RetainedForm::KeepTransient => {
            finalize_borrowed_substring_plan(
                handle,
                start,
                end,
                view_enabled,
                "StringViewBox",
                BorrowedSubstringPlan::FreezeSpan(span),
                "must_freeze_or_keep",
                span_len,
            )
        }
        RetainedForm::ReturnHandle => finalize_borrowed_substring_plan(
            handle,
            start,
            end,
            view_enabled,
            "StringViewBox",
            BorrowedSubstringPlan::ReturnHandle,
            "placement_return_handle",
            span_len,
        ),
    }
}

#[inline(always)]
pub(super) fn borrowed_substring_plan_from_live_object(
    handle: i64,
    start: i64,
    end: i64,
    view_enabled: bool,
    obj: &Arc<dyn NyashBox>,
) -> Option<BorrowedSubstringPlan> {
    if let Some(sb) = obj.as_any().downcast_ref::<StringBox>() {
        return substring_plan_from_string_box(handle, start, end, view_enabled, obj, sb);
    }
    if let Some(view) = obj.as_any().downcast_ref::<StringViewBox>() {
        return substring_plan_from_view_box(handle, start, end, view_enabled, view);
    }
    None
}

#[inline(always)]
pub(super) fn borrowed_substring_plan_from_handle(
    handle: i64,
    start: i64,
    end: i64,
    view_enabled: bool,
) -> Option<BorrowedSubstringPlan> {
    if handle <= 0 {
        return None;
    }
    handles::with_handle(handle as u64, |obj| {
        let Some(obj) = obj else {
            return None;
        };
        borrowed_substring_plan_from_live_object(handle, start, end, view_enabled, obj)
    })
}
