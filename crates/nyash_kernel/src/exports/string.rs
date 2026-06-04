#[path = "string_helpers.rs"]
mod string_helpers;

use self::string_helpers::concat::substring::{
    substring_concat_path_from_parts, SubstringConcatPath,
};
use self::string_helpers::concat::{concat3_fallback, concat_pair_fallback};
use self::string_helpers::materialize::{concat_two_str, string_handle_from_owned_with_site};
use self::string_helpers::*;
use self::string_helpers::{concat3_fast_cache_lookup, concat3_fast_cache_store};
use self::string_helpers::{concat_const_suffix_fallback, insert_const_mid_fallback};
pub(crate) use self::string_helpers::{
    string_is_empty_from_handle, string_len_from_handle, to_owned_string_handle_arg,
};

use crate::exports::string_route_policy::compat_fallback_allowed;
use crate::exports::string_search::{
    compare_string_pair_hh, empty_needle_indexof, empty_needle_lastindexof, find_substr_byte_index,
    rfind_substr_byte_index, search_string_pair_hh,
};
use crate::exports::string_view::clamp_i64_range;
use crate::exports::string_view::{
    borrowed_substring_plan_from_handle, resolve_string_span_pair_from_handles,
    BorrowedSubstringPlan,
};
use crate::hako_forward_bridge;
use crate::observe;
use crate::plugin::{issue_fresh_handle, StringPublishSite};
use crate::string_debug;
use crate::string_trace;
use nyash_rust::runtime::host_handles as handles;

// Thin ABI export surface only.
// String ownership and policy live above this layer; keep these exports as
// stable entrypoints into Rust glue, not as a semantic owner.

// String.len_h(handle) -> i64
#[export_name = "nyash.string.len_h"]
pub extern "C" fn nyash_string_len_h(handle: i64) -> i64 {
    string_helpers::string_len_export_impl(handle)
}

// String.len_fast_h(handle) -> i64
#[export_name = "nyash.string.len_fast_h"]
pub extern "C" fn nyash_string_len_fast_h(handle: i64) -> i64 {
    string_helpers::string_len_fast_export_impl(handle)
}

// FAST-path helper: compute string length from raw pointer (i8*) with mode (reserved)
// Exported under both ABI names (including `nyrt_string_length`).
#[export_name = "nyrt_string_length"]
pub extern "C" fn nyrt_string_length(ptr: *const i8, mode: i64) -> i64 {
    string_helpers::string_length_from_ptr(ptr, mode)
}

// String.charCodeAt_h(handle, idx) -> i64 (byte-based; -1 if OOB)
#[export_name = "nyash.string.charCodeAt_h"]
pub extern "C" fn nyash_string_charcode_at_h_export(handle: i64, idx: i64) -> i64 {
    string_helpers::string_charcode_at_export_impl(handle, idx)
}

// String.concat_hh(lhs_h, rhs_h) -> handle
#[export_name = "nyash.string.concat_hh"]
pub extern "C" fn nyash_string_concat_hh_export(a_h: i64, b_h: i64) -> i64 {
    observe::record_str_concat2_route_enter();
    if let Some(v) = hako_forward_bridge::call_string_dispatch(
        hako_forward_bridge::string_ops::CONCAT_HH,
        a_h,
        b_h,
        0,
    ) {
        observe::record_str_concat2_route_dispatch_hit();
        return v;
    }
    if !compat_fallback_allowed() {
        return hako_forward_bridge::hook_miss_freeze_handle("string.concat_hh");
    }
    concat_pair_fallback(a_h, b_h)
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

// String.substring_concat_hhii(lhs_h, rhs_h, start_i64, end_i64) -> handle
#[export_name = "nyash.string.substring_concat_hhii"]
pub extern "C" fn nyash_string_substring_concat_hhii_export(
    a_h: i64,
    b_h: i64,
    start: i64,
    end: i64,
) -> i64 {
    if let Some(path) = handles::with_text_read_session(|session| {
        session
            .str_pair(a_h as u64, b_h as u64, |a, b| {
                substring_concat_path_from_parts(&[a_h, b_h], &[a, b], start, end)
            })
            .flatten()
    }) {
        return match path {
            SubstringConcatPath::ReturnEmpty => string_helpers::shared_empty_string_handle(),
            SubstringConcatPath::SinglePiece { handle, start, end } => {
                string_helpers::substring_fast_route(handle, start, end)
            }
            SubstringConcatPath::Owned(text) => {
                if text.is_empty() {
                    string_helpers::shared_empty_string_handle()
                } else {
                    string_handle_from_owned_with_site(
                        text,
                        StringPublishSite::StringSubstringConcatHhii,
                    )
                }
            }
        };
    }
    let concat_h = concat_pair_fallback(a_h, b_h);
    string_helpers::substring_fast_route(concat_h, start, end)
}

// String.substring_concat3_hhhii(a_h, b_h, c_h, start_i64, end_i64) -> handle
#[export_name = "nyash.string.substring_concat3_hhhii"]
pub extern "C" fn nyash_string_substring_concat3_hhhii_export(
    a_h: i64,
    b_h: i64,
    c_h: i64,
    start: i64,
    end: i64,
) -> i64 {
    if let Some(path) = handles::with_text_read_session(|session| {
        session
            .str3(a_h as u64, b_h as u64, c_h as u64, |a, b, c| {
                substring_concat_path_from_parts(&[a_h, b_h, c_h], &[a, b, c], start, end)
            })
            .flatten()
    }) {
        return match path {
            SubstringConcatPath::ReturnEmpty => string_helpers::shared_empty_string_handle(),
            SubstringConcatPath::SinglePiece { handle, start, end } => {
                string_helpers::substring_fast_route(handle, start, end)
            }
            SubstringConcatPath::Owned(text) => {
                if text.is_empty() {
                    string_helpers::shared_empty_string_handle()
                } else {
                    string_handle_from_owned_with_site(
                        text,
                        StringPublishSite::StringSubstringConcatHhii,
                    )
                }
            }
        };
    }
    let concat_h = concat3_fallback(a_h, b_h, c_h);
    string_helpers::substring_fast_route(concat_h, start, end)
}

// Runtime-private piecewise subrange helper for publication-boundary corridors.
// This is not a public MIR surface; pure-first injects it only under a
// proof-bearing rewrite target.
#[export_name = "nyash.string.piecewise_subrange_hsiii"]
pub extern "C" fn nyash_string_piecewise_subrange_hsiii_export(
    source_h: i64,
    middle_ptr: *const i8,
    split: i64,
    start: i64,
    end: i64,
) -> i64 {
    piecewise::piecewise_subrange_hsiii_fallback(source_h, middle_ptr, split, start, end)
}

// Runtime-private direct-kernel slot seam.
// Caller owns the slot and must publish or clear it before the boundary escapes.
#[export_name = "nyash.string.kernel_slot_piecewise_subrange_hsiii"]
pub extern "C" fn nyash_string_kernel_slot_piecewise_subrange_hsiii_export(
    slot: *mut crate::plugin::KernelTextSlot,
    source_h: i64,
    middle_ptr: *const i8,
    split: i64,
    start: i64,
    end: i64,
) -> i64 {
    unsafe {
        let slot = &mut *slot;
        i64::from(piecewise::piecewise_subrange_hsiii_into_slot(
            slot, source_h, middle_ptr, split, start, end,
        ))
    }
}

// Runtime-private direct-kernel slot seam.
#[export_name = "nyash.string.kernel_slot_capture_h"]
pub extern "C" fn nyash_string_kernel_slot_capture_h_export(
    slot: *mut crate::plugin::KernelTextSlot,
    source_h: i64,
) -> i64 {
    with_kernel_text_slot_mut(slot, |slot| {
        slot.clear();
        let Some(text) = crate::plugin::owned_string_from_handle(source_h) else {
            return 0;
        };
        crate::plugin::freeze_owned_string_into_slot(slot, text);
        1
    })
    .unwrap_or(0)
}

// Runtime-private direct-kernel slot seam.
#[export_name = "nyash.string.kernel_slot_concat_hh"]
pub extern "C" fn nyash_string_kernel_slot_concat_hh_export(
    slot: *mut crate::plugin::KernelTextSlot,
    a_h: i64,
    b_h: i64,
) -> i64 {
    unsafe {
        string_helpers::with_kernel_text_slot_mut(&mut *slot, |slot| {
            let owned = if let Some(text) = handles::with_text_read_session(|session| {
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
                text
            } else if let Some((a_span, b_span)) = resolve_string_span_pair_from_handles(a_h, b_h) {
                let a = a_span.as_text();
                let b = b_span.as_text();
                if a.is_empty() {
                    b.to_string()
                } else if b.is_empty() {
                    a.to_string()
                } else {
                    concat_two_str(a.as_str(), b.as_str())
                }
            } else {
                let lhs = to_owned_string_handle_arg(a_h);
                let rhs = to_owned_string_handle_arg(b_h);
                concat_two_str(lhs.as_str(), rhs.as_str())
            };
            crate::plugin::freeze_owned_string_into_slot(slot, owned);
            1
        })
        .unwrap_or(0)
    }
}

// Runtime-private direct-kernel slot seam.
#[export_name = "nyash.string.kernel_slot_concat_hs"]
pub extern "C" fn nyash_string_kernel_slot_concat_hs_export(
    slot: *mut crate::plugin::KernelTextSlot,
    a_h: i64,
    suffix_ptr: *const i8,
) -> i64 {
    unsafe {
        string_helpers::with_kernel_text_slot_mut(&mut *slot, |slot| {
            i64::from(string_helpers::concat_const_suffix_into_slot(
                slot, a_h, suffix_ptr,
            ))
        })
        .unwrap_or(0)
    }
}

// Runtime-private direct-kernel slot seam.
#[export_name = "nyash.string.kernel_slot_insert_hsi"]
pub extern "C" fn nyash_string_kernel_slot_insert_hsi_export(
    slot: *mut crate::plugin::KernelTextSlot,
    source_h: i64,
    middle_ptr: *const i8,
    split: i64,
) -> i64 {
    unsafe {
        string_helpers::with_kernel_text_slot_mut(&mut *slot, |slot| {
            i64::from(string_helpers::insert_const_mid_into_slot(
                slot, source_h, middle_ptr, split,
            ))
        })
        .unwrap_or(0)
    }
}

// Runtime-private direct-kernel slot seam.
#[export_name = "nyash.string.kernel_slot_piecewise_subrange_ssiii"]
pub extern "C" fn nyash_string_kernel_slot_piecewise_subrange_ssiii_export(
    out: *mut crate::plugin::KernelTextSlot,
    source: *const crate::plugin::KernelTextSlot,
    middle_ptr: *const i8,
    split: i64,
    start: i64,
    end: i64,
) -> i64 {
    unsafe {
        with_kernel_text_slot_mut(&mut *out, |out| {
            source
                .as_ref()
                .map(|source| {
                    i64::from(
                        string_helpers::piecewise::piecewise_subrange_kernel_text_slot_into_slot(
                            out, source, middle_ptr, split, start, end,
                        ),
                    )
                })
                .unwrap_or_else(|| {
                    out.clear();
                    0
                })
        })
        .unwrap_or(0)
    }
}

// Runtime-private direct-kernel slot seam.
#[export_name = "nyash.string.kernel_slot_substring_hii_in_place"]
pub extern "C" fn nyash_string_kernel_slot_substring_hii_in_place_export(
    slot: *mut crate::plugin::KernelTextSlot,
    start: i64,
    end: i64,
) -> i64 {
    unsafe {
        with_kernel_text_slot_mut(&mut *slot, |slot| {
            i64::from(
                string_helpers::piecewise::substring_kernel_text_slot_in_place(slot, start, end),
            )
        })
        .unwrap_or(0)
    }
}

// Runtime-private direct-kernel publish boundary.
#[export_name = "nyash.string.kernel_slot_publish_h"]
pub extern "C" fn nyash_string_kernel_slot_publish_h_export(
    slot: *mut crate::plugin::KernelTextSlot,
) -> i64 {
    unsafe {
        crate::plugin::publish_kernel_text_slot(&mut *slot)
            .unwrap_or_else(string_helpers::shared_empty_string_handle)
    }
}

// Runtime-private publish.text adapter (reason=explicit_api_replay, repr=stable_view).
#[export_name = "nyash.string.substring_publish_explicit_api_view_hii"]
pub extern "C" fn nyash_string_substring_publish_explicit_api_view_hii_export(
    h: i64,
    start: i64,
    end: i64,
) -> i64 {
    if h <= 0 {
        return 0;
    }

    if let Some(hit) = substring_view_arc_cache_lookup(h, start, end) {
        match hit {
            SubstringViewCacheHit::Handle(handle) => {
                return handle;
            }
            SubstringViewCacheHit::Reissue { result_obj, len } => {
                observe::record_birth_placement_borrow_view();
                observe::record_birth_backend_publish_reason_explicit_api();
                let handle = issue_fresh_handle(result_obj);
                if handle > 0 {
                    string_len_fast_cache_store(handle, len);
                    substring_view_arc_cache_refresh_handle(h, start, end, handle);
                }
                return handle;
            }
        }
    }

    let Some(plan) = borrowed_substring_plan_from_handle(h, start, end, true) else {
        return string_helpers::shared_empty_string_handle();
    };
    match plan {
        BorrowedSubstringPlan::ReturnHandle => {
            substring_fast_cache_store(h, start, end, true, h);
            h
        }
        BorrowedSubstringPlan::ReturnEmpty => {
            let result = string_helpers::shared_empty_string_handle();
            if result > 0 {
                substring_fast_cache_store(h, start, end, true, result);
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
                return string_helpers::shared_empty_string_handle();
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
                substring_fast_cache_store(h, start, end, true, result);
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
            let result_obj: std::sync::Arc<dyn nyash_rust::box_trait::NyashBox> =
                std::sync::Arc::new(span.into_view_box());
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

// Runtime-private publish.text adapter (repr=stable_owned, reason=explicit_api_replay).
#[export_name = "nyash.string.substring_concat3_publish_explicit_api_owned_hhhii"]
pub extern "C" fn nyash_string_substring_concat3_publish_explicit_api_owned_hhhii_export(
    a_h: i64,
    b_h: i64,
    c_h: i64,
    start: i64,
    end: i64,
) -> i64 {
    concat3_substring_publish_owned_with_reason(
        a_h,
        b_h,
        c_h,
        start,
        end,
        crate::plugin::PublishReason::ExplicitApi,
    )
}

// Runtime-private publish.text adapter (repr=stable_owned, reason=stable_object_demand).
#[export_name = "nyash.string.substring_concat3_publish_need_stable_owned_hhhii"]
pub extern "C" fn nyash_string_substring_concat3_publish_need_stable_owned_hhhii_export(
    a_h: i64,
    b_h: i64,
    c_h: i64,
    start: i64,
    end: i64,
) -> i64 {
    concat3_substring_publish_owned_with_reason(
        a_h,
        b_h,
        c_h,
        start,
        end,
        crate::plugin::PublishReason::NeedStableObject,
    )
}

// Runtime-private direct-kernel slot seam.
#[export_name = "nyash.string.kernel_slot_len_i"]
pub extern "C" fn nyash_string_kernel_slot_len_i_export(
    slot: *const crate::plugin::KernelTextSlot,
) -> i64 {
    unsafe {
        crate::plugin::with_kernel_text_slot_text(&*slot, |text| text.len() as i64).unwrap_or(0)
    }
}

// String.concat3_hhh(a_h, b_h, c_h) -> handle
#[export_name = "nyash.string.concat3_hhh"]
pub extern "C" fn nyash_string_concat3_hhh_export(a_h: i64, b_h: i64, c_h: i64) -> i64 {
    if let Some(cached) = concat3_fast_cache_lookup(a_h, b_h, c_h) {
        return cached;
    }
    if let Some(v) = hako_forward_bridge::call_string_dispatch(
        hako_forward_bridge::string_ops::CONCAT3_HHH,
        a_h,
        b_h,
        c_h,
    ) {
        concat3_fast_cache_store(a_h, b_h, c_h, v);
        return v;
    }
    if !compat_fallback_allowed() {
        return hako_forward_bridge::hook_miss_freeze_handle("string.concat3_hhh");
    }
    let v = concat3_fallback(a_h, b_h, c_h);
    if v > 0 {
        concat3_fast_cache_store(a_h, b_h, c_h, v);
    }
    v
}

// String.eq_hh(lhs_h, rhs_h) -> i64 (0/1)
#[export_name = "nyash.string.eq_hh"]
pub extern "C" fn nyash_string_eq_hh_export(a_h: i64, b_h: i64) -> i64 {
    if let Some(v) = hako_forward_bridge::call_string_dispatch(
        hako_forward_bridge::string_ops::EQ_HH,
        a_h,
        b_h,
        0,
    ) {
        return v;
    }
    if !compat_fallback_allowed() {
        return hako_forward_bridge::hook_miss_error_code("string.eq_hh");
    }
    let result = compare_string_pair_hh(a_h, b_h, |lhs, rhs| lhs == rhs);
    string_debug::stage1_string_debug_log_eq(a_h, b_h, result);
    result
}

// String.substring_hii(handle, start, end) -> handle
#[export_name = "nyash.string.substring_hii"]
pub extern "C" fn nyash_string_substring_hii_export(h: i64, start: i64, end: i64) -> i64 {
    string_helpers::substring_fast_route(h, start, end)
}

// String.substring_len_hii(handle, start, end) -> i64
// Internal borrowed-corridor helper for AOT lowering. This computes the
// resulting substring length without forcing view publication/materialization.
#[export_name = "nyash.string.substring_len_hii"]
pub extern "C" fn nyash_string_substring_len_hii_export(h: i64, start: i64, end: i64) -> i64 {
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

// String.indexOf_hh(haystack_h, needle_h) -> i64
#[export_name = "nyash.string.indexOf_hh"]
pub extern "C" fn nyash_string_indexof_hh_export(h: i64, n: i64) -> i64 {
    if let Some(v) = hako_forward_bridge::call_string_dispatch(
        hako_forward_bridge::string_ops::INDEXOF_HH,
        h,
        n,
        0,
    ) {
        return v;
    }
    if !compat_fallback_allowed() {
        return hako_forward_bridge::hook_miss_error_code("string.indexOf_hh");
    }
    search_string_pair_hh(h, n, empty_needle_indexof, find_substr_byte_index)
}

// String.lastIndexOf_hh(haystack_h, needle_h) -> i64
#[export_name = "nyash.string.lastIndexOf_hh"]
pub extern "C" fn nyash_string_lastindexof_hh_export(h: i64, n: i64) -> i64 {
    if let Some(v) = hako_forward_bridge::call_string_dispatch(
        hako_forward_bridge::string_ops::LASTINDEXOF_HH,
        h,
        n,
        0,
    ) {
        return v;
    }
    if !compat_fallback_allowed() {
        return hako_forward_bridge::hook_miss_error_code("string.lastIndexOf_hh");
    }
    search_string_pair_hh(h, n, empty_needle_lastindexof, rfind_substr_byte_index)
}

// String.contains_hh(haystack_h, needle_h) -> i64 (0/1)
#[export_name = "nyash.string.contains_hh"]
pub extern "C" fn nyash_string_contains_hh_export(h: i64, n: i64) -> i64 {
    if nyash_string_indexof_hh_export(h, n) >= 0 {
        1
    } else {
        0
    }
}

// String.lt_hh(lhs_h, rhs_h) -> i64 (0/1)
#[export_name = "nyash.string.lt_hh"]
pub extern "C" fn nyash_string_lt_hh_export(a_h: i64, b_h: i64) -> i64 {
    if let Some(v) = hako_forward_bridge::call_string_dispatch(
        hako_forward_bridge::string_ops::LT_HH,
        a_h,
        b_h,
        0,
    ) {
        return v;
    }
    if !compat_fallback_allowed() {
        return hako_forward_bridge::hook_miss_error_code("string.lt_hh");
    }
    compare_string_pair_hh(a_h, b_h, |lhs, rhs| lhs < rhs)
}

// Construct StringBox from two u64 words (little-endian) + length (<=16) and return handle
// export: nyash.string.from_u64x2(lo, hi, len) -> i64
#[export_name = "nyash.string.from_u64x2"]
pub extern "C" fn nyash_string_from_u64x2_export(lo: i64, hi: i64, len: i64) -> i64 {
    if let Some(v) = hako_forward_bridge::call_string_dispatch(
        hako_forward_bridge::string_ops::FROM_U64X2,
        lo,
        hi,
        len,
    ) {
        return v;
    }
    if !compat_fallback_allowed() {
        return hako_forward_bridge::hook_miss_freeze_handle("string.from_u64x2");
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
    string_handle_from_owned_with_site(s, StringPublishSite::Generic)
}
