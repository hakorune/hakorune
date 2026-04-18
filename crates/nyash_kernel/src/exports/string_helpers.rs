// String export helper logic split out from string.rs.

#[path = "string_helpers/cache.rs"]
mod cache;
#[path = "string_helpers/concat.rs"]
mod concat;
#[path = "string_helpers/materialize.rs"]
mod materialize;

#[cfg(test)]
#[path = "string_helpers/tests.rs"]
mod tests;

use crate::exports::string_debug::{
    jit_trace_len_enabled, jit_trace_len_state_init, jit_trace_len_state_raw,
    stage1_string_debug_log_eq, substring_route_policy, SubstringRoutePolicy,
};
use crate::exports::string_search::{
    compare_string_pair_hh, empty_needle_indexof, empty_needle_lastindexof, find_substr_byte_index,
    rfind_substr_byte_index, search_string_pair_hh,
};
use crate::exports::string_view::{
    borrowed_substring_plan_from_handle, clamp_i64_range, resolve_string_span_from_handle,
    BorrowedSubstringPlan,
};
use crate::hako_forward_bridge;
use crate::observe;
use crate::plugin::{
    freeze_owned_string_into_slot, issue_fresh_handle_from_arc, owned_string_from_handle,
    publish_kernel_text_slot, KernelTextSlot,
};
use nyash_rust::box_trait::NyashBox;
use nyash_rust::runtime::host_handles as handles;
use std::{ffi::CStr, sync::Arc};

use self::cache::{
    concat3_fast_cache_lookup, concat3_fast_cache_store, string_len_fast_cache_lookup,
    string_len_fast_cache_store, substring_fast_cache_lookup, substring_fast_cache_store,
    substring_view_arc_cache_lookup, substring_view_arc_cache_refresh_handle,
    substring_view_arc_cache_store, SubstringViewCacheHit,
};
use self::concat::{
    concat3_fallback, concat3_substring_fallback, concat_const_suffix_fallback,
    concat_const_suffix_into_slot, concat_pair_fallback, concat_pair_into_slot,
    concat_pair_substring_fallback, insert_const_mid_fallback, piecewise_subrange_hsiii_fallback,
    piecewise_subrange_hsiii_into_slot, piecewise_subrange_kernel_text_slot_into_slot,
    substring_kernel_text_slot_in_place,
};
use self::materialize::{
    shared_empty_string_handle, string_handle_from_owned, string_handle_from_span,
    trace_observer_resolution_enabled,
};

pub(crate) use self::materialize::{
    string_is_empty_from_handle, string_len_from_handle, to_owned_string_handle_arg,
};

// Native string substrate helpers.
// These routines stay below semantic ownership and keep raw copy/search/materialize
// fast paths in Rust unless a source-backed replacement proves safe.
// They serve the thin ABI facade and VM wrappers; they do not own route policy.

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

#[cold]
#[inline(never)]
fn trace_len_fast_hit(handle: i64, cached: i64) {
    trace_observer_resolution_enabled(
        true,
        "observer",
        handle,
        "fast_hit",
        "len_handle_cache",
        || format!("len={}", cached),
    );
}

#[cold]
#[inline(never)]
fn string_len_export_slow_path(handle: i64) -> i64 {
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

#[inline(always)]
pub(super) fn string_len_export_impl(handle: i64) -> i64 {
    let dispatch_raw = hako_forward_bridge::string_len_dispatch_probe_raw();
    if dispatch_raw != 0 {
        let dispatch: hako_forward_bridge::HakoStringDispatchFn =
            unsafe { std::mem::transmute(dispatch_raw) };
        let v = dispatch(hako_forward_bridge::string_ops::LEN_H, handle, 0, 0);
        observe::record_str_len_route_dispatch_hit();
        return v;
    }
    if let Some(cached) = string_len_fast_cache_lookup(handle) {
        observe::record_str_len_route_fast_str_hit();
        if observe::len_route_matches_latest_fresh_handle(handle) {
            observe::record_str_len_route_latest_fresh_handle_fast_str_hit();
        }
        let trace_state = jit_trace_len_state_raw();
        if trace_state == 0 {
            return cached;
        }
        if trace_state == 1 {
            return string_len_trace_fast_return(handle, cached);
        }
        return string_len_fast_return_unknown_trace_state(handle, cached);
    }
    string_len_export_slow_path(handle)
}

#[inline(never)]
fn string_len_trace_fast_return(handle: i64, cached: i64) -> i64 {
    trace_len_fast_hit(handle, cached);
    cached
}

#[inline(never)]
fn string_len_fast_return_unknown_trace_state(handle: i64, cached: i64) -> i64 {
    if jit_trace_len_state_init() != 0 {
        string_len_trace_fast_return(handle, cached)
    } else {
        cached
    }
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

pub(super) fn string_substring_concat_hhii_export_impl(
    a_h: i64,
    b_h: i64,
    start: i64,
    end: i64,
) -> i64 {
    concat_pair_substring_fallback(a_h, b_h, start, end)
}

pub(super) fn string_substring_concat3_hhhii_export_impl(
    a_h: i64,
    b_h: i64,
    c_h: i64,
    start: i64,
    end: i64,
) -> i64 {
    concat3_substring_fallback(a_h, b_h, c_h, start, end)
}

pub(super) fn string_piecewise_subrange_hsiii_export_impl(
    source_h: i64,
    middle_ptr: *const i8,
    split: i64,
    start: i64,
    end: i64,
) -> i64 {
    piecewise_subrange_hsiii_fallback(source_h, middle_ptr, split, start, end)
}

pub(super) fn string_piecewise_subrange_hsiii_into_slot_export_impl(
    slot: *mut KernelTextSlot,
    source_h: i64,
    middle_ptr: *const i8,
    split: i64,
    start: i64,
    end: i64,
) -> i64 {
    let Some(slot) = (unsafe { slot.as_mut() }) else {
        return 0;
    };
    i64::from(piecewise_subrange_hsiii_into_slot(
        slot, source_h, middle_ptr, split, start, end,
    ))
}

#[inline(always)]
fn string_handle_into_slot(slot: &mut KernelTextSlot, source_h: i64) -> bool {
    slot.clear();
    let Some(text) = owned_string_from_handle(source_h) else {
        return false;
    };
    freeze_owned_string_into_slot(slot, text);
    true
}

pub(super) fn string_handle_into_slot_export_impl(slot: *mut KernelTextSlot, source_h: i64) -> i64 {
    let Some(slot) = (unsafe { slot.as_mut() }) else {
        return 0;
    };
    i64::from(string_handle_into_slot(slot, source_h))
}

pub(super) fn string_concat_hh_into_slot_export_impl(
    slot: *mut KernelTextSlot,
    a_h: i64,
    b_h: i64,
) -> i64 {
    let Some(slot) = (unsafe { slot.as_mut() }) else {
        return 0;
    };
    i64::from(concat_pair_into_slot(slot, a_h, b_h))
}

pub(super) fn string_concat_hs_into_slot_export_impl(
    slot: *mut KernelTextSlot,
    a_h: i64,
    suffix_ptr: *const i8,
) -> i64 {
    let Some(slot) = (unsafe { slot.as_mut() }) else {
        return 0;
    };
    i64::from(concat_const_suffix_into_slot(slot, a_h, suffix_ptr))
}

pub(super) fn string_piecewise_subrange_kernel_text_slot_into_slot_export_impl(
    out: *mut KernelTextSlot,
    source: *const KernelTextSlot,
    middle_ptr: *const i8,
    split: i64,
    start: i64,
    end: i64,
) -> i64 {
    let Some(out) = (unsafe { out.as_mut() }) else {
        return 0;
    };
    let Some(source) = (unsafe { source.as_ref() }) else {
        out.clear();
        return 0;
    };
    i64::from(piecewise_subrange_kernel_text_slot_into_slot(
        out, source, middle_ptr, split, start, end,
    ))
}

pub(super) fn string_kernel_text_slot_substring_hii_in_place_export_impl(
    slot: *mut KernelTextSlot,
    start: i64,
    end: i64,
) -> i64 {
    let Some(slot) = (unsafe { slot.as_mut() }) else {
        return 0;
    };
    i64::from(substring_kernel_text_slot_in_place(slot, start, end))
}

pub(super) fn string_publish_kernel_text_slot_h_export_impl(slot: *mut KernelTextSlot) -> i64 {
    let Some(slot) = (unsafe { slot.as_mut() }) else {
        return 0;
    };
    publish_kernel_text_slot(slot).unwrap_or_else(shared_empty_string_handle)
}

pub(super) fn string_kernel_text_slot_len_i_export_impl(slot: *const KernelTextSlot) -> i64 {
    let Some(slot) = (unsafe { slot.as_ref() }) else {
        return 0;
    };
    crate::plugin::with_kernel_text_slot_text(slot, |text| text.len() as i64).unwrap_or(0)
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

#[inline(always)]
pub(super) fn string_substring_hii_export_impl(h: i64, start: i64, end: i64) -> i64 {
    if h <= 0 {
        return 0;
    }
    observe::record_str_substring_route_enter();
    let SubstringRoutePolicy {
        view_enabled,
        fallback_allowed,
    } = substring_route_policy();
    if fallback_allowed {
        if view_enabled {
            if let Some(hit) = substring_view_arc_cache_lookup(h, start, end) {
                match hit {
                    SubstringViewCacheHit::Handle(handle) => {
                        observe::record_str_substring_route_view_arc_cache_handle_hit();
                        return handle;
                    }
                    SubstringViewCacheHit::Reissue { result_obj, len } => {
                        observe::record_str_substring_route_view_arc_cache_reissue_hit();
                        observe::record_birth_placement_borrow_view();
                        let handle = issue_fresh_handle_from_arc(result_obj);
                        if handle > 0 {
                            string_len_fast_cache_store(handle, len);
                            substring_view_arc_cache_refresh_handle(h, start, end, handle);
                        }
                        return handle;
                    }
                }
            }
            observe::record_str_substring_route_view_arc_cache_miss();
        }
        if let Some(hit) = substring_fast_cache_lookup(h, start, end, view_enabled) {
            observe::record_str_substring_route_fast_cache_hit();
            return hit;
        }
    }
    let dispatch_raw = hako_forward_bridge::string_dispatch_raw();
    if dispatch_raw != 0 {
        observe::record_str_substring_route_dispatch_hit();
        let dispatch: hako_forward_bridge::HakoStringDispatchFn =
            unsafe { std::mem::transmute(dispatch_raw) };
        let v = dispatch(
            hako_forward_bridge::string_ops::SUBSTRING_HII,
            h,
            start,
            end,
        );
        substring_fast_cache_store(h, start, end, view_enabled, v);
        return v;
    }
    if !fallback_allowed {
        return hook_miss_freeze_handle("string.substring_hii");
    }
    observe::record_str_substring_route_slow_plan();
    let Some(plan) = borrowed_substring_plan_from_handle(h, start, end, view_enabled) else {
        return shared_empty_string_handle();
    };
    match plan {
        BorrowedSubstringPlan::ReturnHandle => {
            observe::record_birth_placement_return_handle();
            substring_fast_cache_store(h, start, end, view_enabled, h);
            h
        }
        BorrowedSubstringPlan::ReturnEmpty => {
            let result = shared_empty_string_handle();
            if result > 0 {
                substring_fast_cache_store(h, start, end, view_enabled, result);
            }
            result
        }
        BorrowedSubstringPlan::FreezeSpan(span) => {
            let result = string_handle_from_span(span);
            if result > 0 {
                substring_fast_cache_store(h, start, end, view_enabled, result);
            }
            result
        }
        BorrowedSubstringPlan::ViewSpan {
            span,
            source_box_id,
        } => {
            observe::record_birth_placement_borrow_view();
            let len = span.len() as i64;
            let result_obj: Arc<dyn NyashBox> = Arc::new(span.into_view_box());
            let handle = issue_fresh_handle_from_arc(result_obj.clone());
            if handle > 0 {
                string_len_fast_cache_store(handle, len);
                substring_view_arc_cache_store(
                    h,
                    source_box_id,
                    start,
                    end,
                    len,
                    result_obj,
                    handle,
                );
            }
            handle
        }
    }
}

#[inline(always)]
pub(super) fn string_substring_len_hii_export_impl(h: i64, start: i64, end: i64) -> i64 {
    if h <= 0 {
        return 0;
    }
    handles::with_text_read_session_ready(|session| {
        session
            .str_handle(h as u64, |text| {
                let (start, end) = clamp_i64_range(text.len(), start, end);
                end.saturating_sub(start) as i64
            })
            .unwrap_or(0)
    })
    .unwrap_or(0)
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
