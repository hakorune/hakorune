// String export helper logic split out from string.rs.

#[path = "string_helpers/cache.rs"]
mod cache;
#[path = "string_helpers/concat.rs"]
pub(crate) mod concat;
#[path = "string_helpers/materialize.rs"]
pub(crate) mod materialize;

#[cfg(test)]
#[path = "string_helpers/tests.rs"]
mod tests;

use crate::exports::string_route_policy::{
    compat_fallback_allowed, substring_route_policy, SubstringRoutePolicy,
};
use crate::exports::string_trace;
use crate::exports::string_view::{borrowed_substring_plan_from_handle, BorrowedSubstringPlan};
use crate::hako_forward_bridge;
use crate::observe;
use crate::plugin::{issue_fresh_handle, KernelTextSlot, StringPublishSite};
use nyash_rust::box_trait::NyashBox;
use nyash_rust::runtime::host_handles as handles;
use std::sync::Arc;

pub(crate) use self::cache::{
    string_len_fast_cache_lookup, string_len_fast_cache_store, substring_fast_cache_lookup,
    substring_fast_cache_store, substring_len_fast_cache_lookup,
    substring_len_fast_cache_store, substring_view_arc_cache_lookup,
    substring_view_arc_cache_refresh_handle, substring_view_arc_cache_store, SubstringViewCacheHit,
};
pub(crate) use self::concat::const_adapter::concat_const_suffix_into_slot;
pub(crate) use self::concat::const_adapter::insert_const_mid_into_slot;
pub(super) use self::concat::piecewise;
pub(super) use self::concat::substring::concat3_substring_publish_owned_with_reason;
pub(crate) use self::materialize::shared_empty_string_handle;
use self::materialize::string_handle_from_owned_with_site;
pub(crate) use self::materialize::trace_observer_resolution_enabled;

pub(crate) use self::cache::{concat3_fast_cache_lookup, concat3_fast_cache_store};
pub(crate) use self::concat::const_adapter::{
    concat_const_suffix_fallback, insert_const_mid_fallback,
};

pub(crate) use self::materialize::{
    string_is_empty_from_handle, string_len_from_handle, to_owned_string_handle_arg,
};
use crate::c_string::c_string_bytes;

// Native string helper routines.
// These stay below semantic ownership and keep raw copy/search/materialize
// fast paths in Rust unless a source-backed replacement proves safe.
// They serve the ABI surface and VM wrappers; they do not own route policy.

#[inline(always)]
pub(crate) fn with_kernel_text_slot_mut<T>(
    slot: *mut KernelTextSlot,
    f: impl FnOnce(&mut KernelTextSlot) -> T,
) -> Option<T> {
    unsafe { slot.as_mut().map(f) }
}

#[inline(always)]
pub(super) fn string_length_from_ptr(ptr: *const i8, _mode: i64) -> i64 {
    if ptr.is_null() {
        return 0;
    }
    c_string_bytes(ptr).len() as i64
}

#[inline(always)]
pub(super) fn string_len_export_impl(handle: i64) -> i64 {
    let dispatch_raw = hako_forward_bridge::string_dispatch_raw();
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
        if !crate::env_flags::jit_trace_len_enabled() {
            return cached;
        }
        trace_len_fast_hit(handle, cached);
        return cached;
    }
    string_len_export_slow_path(handle)
}

#[inline(always)]
pub(super) fn string_len_fast_export_impl(handle: i64) -> i64 {
    if handle > 0 {
        if let Some(fast_len) = handles::with_text_read_session_ready(|session| {
            session.str_handle(handle as u64, |text| text.len() as i64)
        })
        .flatten()
        {
            return fast_len;
        }
    }
    string_len_export_impl(handle)
}

#[inline(always)]
pub(super) fn string_charcode_at_export_impl(handle: i64, idx: i64) -> i64 {
    if let Some(v) = hako_forward_bridge::call_string_dispatch(
        hako_forward_bridge::string_ops::CHARCODE_AT_H,
        handle,
        idx,
        0,
    ) {
        return v;
    }
    if !compat_fallback_allowed() {
        return hako_forward_bridge::hook_miss_error_code("string.charCodeAt_h");
    }
    if idx < 0 {
        return -1;
    }
    if let Some(span) = crate::exports::string_view::resolve_string_span_from_handle(handle) {
        let s = span.as_text();
        let i = idx as usize;
        if i < s.len() {
            return s.as_bytes()[i] as i64;
        }
    }
    -1
}

#[inline(always)]
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
    if !compat_fallback_allowed() {
        return hako_forward_bridge::hook_miss_error_code("string.len_h");
    }
    if crate::env_flags::jit_trace_len_enabled() {
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
pub(crate) fn substring_fast_route(h: i64, start: i64, end: i64) -> i64 {
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
                        let handle = issue_fresh_handle(result_obj);
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
        return hako_forward_bridge::hook_miss_freeze_handle("string.substring_hii");
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
            let source = span.as_text();
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
                std::ptr::copy_nonoverlapping(source.as_ptr(), buf.as_mut_ptr(), len);
            }
            let result = string_handle_from_owned_with_site(out, StringPublishSite::Generic);
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
                        result
                    ),
                );
            }
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
            observe::record_birth_backend_publish_reason_explicit_api();
            let result_obj: Arc<dyn NyashBox> = Arc::new(span.into_view_box());
            let handle = issue_fresh_handle(result_obj.clone());
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
