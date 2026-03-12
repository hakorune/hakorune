// String-related C ABI exports.

use super::string_view::{
    clamp_i64_range, resolve_string_span_from_handle, resolve_string_span_from_handle_nocache,
    resolve_string_span_from_obj, resolve_string_span_pair_from_handles,
    string_is_empty_from_handle as string_is_empty_impl, string_len_from_handle as string_len_impl,
    string_view_from_span_range, StringViewBox,
};
use crate::hako_forward_bridge;
use memchr::{memchr, memmem, memrchr};
use nyash_rust::{
    box_trait::{NyashBox, StringBox},
    runtime::host_handles as handles,
};
use std::ptr;
use std::sync::{Arc, OnceLock};

fn env_flag_cached(_cell: &'static OnceLock<bool>, key: &str) -> bool {
    #[cfg(test)]
    {
        std::env::var(key).ok().as_deref() == Some("1")
    }
    #[cfg(not(test))]
    {
        *_cell.get_or_init(|| std::env::var(key).ok().as_deref() == Some("1"))
    }
}

fn env_flag_default_on_cached(_cell: &'static OnceLock<bool>, key: &str) -> bool {
    #[cfg(test)]
    {
        match std::env::var(key).ok().as_deref() {
            Some("0") => false,
            Some("off") => false,
            Some("false") => false,
            Some(_) => true,
            None => true,
        }
    }
    #[cfg(not(test))]
    {
        *_cell.get_or_init(|| match std::env::var(key).ok().as_deref() {
            Some("0") => false,
            Some("off") => false,
            Some("false") => false,
            Some(_) => true,
            None => true,
        })
    }
}

fn stage1_string_debug_enabled() -> bool {
    static STAGE1_STRING_DEBUG: OnceLock<bool> = OnceLock::new();
    env_flag_cached(&STAGE1_STRING_DEBUG, "STAGE1_CLI_DEBUG")
}

fn stage1_string_handle_debug(handle: i64) -> (bool, usize, String) {
    if let Some(span) = resolve_string_span_from_handle_nocache(handle) {
        let s = span.as_str();
        let preview = if s.len() <= 48 {
            s.to_string()
        } else {
            s[..48].to_string()
        };
        return (true, s.len(), preview);
    }
    (false, 0, String::new())
}

fn stage1_string_debug_log_eq(a_h: i64, b_h: i64, result: i64) {
    if !stage1_string_debug_enabled() {
        return;
    }
    let (a_ok, a_len, a_preview) = stage1_string_handle_debug(a_h);
    let (b_ok, b_len, b_preview) = stage1_string_handle_debug(b_h);
    eprintln!(
        "[stage1/string_export] op=eq lhs={} lhs_ok={} lhs_len={} lhs_preview={:?} rhs={} rhs_ok={} rhs_len={} rhs_preview={:?} result={}",
        a_h, a_ok, a_len, a_preview, b_h, b_ok, b_len, b_preview, result
    );
}

fn stage1_string_debug_log_concat_materialize(a_h: i64, b_h: i64, out_h: i64) {
    if !stage1_string_debug_enabled() {
        return;
    }
    let (a_ok, a_len, a_preview) = stage1_string_handle_debug(a_h);
    let (b_ok, b_len, b_preview) = stage1_string_handle_debug(b_h);
    let (out_ok, out_len, out_preview) = stage1_string_handle_debug(out_h);
    eprintln!(
        "[stage1/string_export] op=concat_materialize lhs={} lhs_ok={} lhs_len={} lhs_preview={:?} rhs={} rhs_ok={} rhs_len={} rhs_preview={:?} out={} out_ok={} out_len={} out_preview={:?}",
        a_h,
        a_ok,
        a_len,
        a_preview,
        b_h,
        b_ok,
        b_len,
        b_preview,
        out_h,
        out_ok,
        out_len,
        out_preview
    );
}

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

fn substring_view_enabled() -> bool {
    static SUBSTRING_VIEW_ENABLED: OnceLock<bool> = OnceLock::new();
    env_flag_default_on_cached(&SUBSTRING_VIEW_ENABLED, "NYASH_LLVM_FAST")
}

fn jit_trace_len_enabled() -> bool {
    static JIT_TRACE_LEN_ENABLED: OnceLock<bool> = OnceLock::new();
    env_flag_cached(&JIT_TRACE_LEN_ENABLED, "NYASH_JIT_TRACE_LEN")
}

fn string_handle_from_owned(value: String) -> i64 {
    nyash_rust::runtime::global_hooks::gc_alloc(value.len() as u64);
    let arc: Arc<dyn NyashBox> = Arc::new(StringBox::new(value));
    handles::to_handle_arc(arc) as i64
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
        static HANDLE: OnceLock<i64> = OnceLock::new();
        *HANDLE.get_or_init(|| {
            let arc: Arc<dyn NyashBox> = Arc::new(StringBox::new(String::new()));
            handles::to_handle_arc(arc) as i64
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

fn to_owned_string_handle_arg(h: i64) -> String {
    resolve_string_span_from_handle(h)
        .map(|span| span.as_str().to_string())
        .unwrap_or_default()
}

#[inline(always)]
fn with_string_pair_fast_str<R>(a_h: i64, b_h: i64, f: impl FnOnce(&str, &str) -> R) -> Option<R> {
    if a_h <= 0 || b_h <= 0 {
        return None;
    }
    handles::with_str_pair(a_h as u64, b_h as u64, f)
}

#[inline(always)]
fn with_string_pair_span<R>(a_h: i64, b_h: i64, f: impl FnOnce(&str, &str) -> R) -> Option<R> {
    let (a_span, b_span) = resolve_string_span_pair_from_handles(a_h, b_h)?;
    Some(f(a_span.as_str(), b_span.as_str()))
}

#[inline(always)]
fn with_string_pair_lossy_span<R>(a_h: i64, b_h: i64, f: impl FnOnce(&str, &str) -> R) -> R {
    let a_span = resolve_string_span_from_handle(a_h);
    let b_span = resolve_string_span_from_handle(b_h);
    let a = a_span.as_ref().map(|span| span.as_str()).unwrap_or("");
    let b = b_span.as_ref().map(|span| span.as_str()).unwrap_or("");
    f(a, b)
}

#[inline(always)]
fn with_lossy_string_pair<R>(a_h: i64, b_h: i64, f: impl FnOnce(&str, &str) -> R) -> R {
    let mut f_opt = Some(f);
    if let Some(out) = with_string_pair_fast_str(a_h, b_h, |a, b| {
        let f = f_opt
            .take()
            .expect("[string/export] with_lossy_string_pair closure missing (fast)");
        f(a, b)
    }) {
        return out;
    }
    if let Some(out) = with_string_pair_span(a_h, b_h, |a, b| {
        let f = f_opt
            .take()
            .expect("[string/export] with_lossy_string_pair closure missing (span)");
        f(a, b)
    }) {
        return out;
    }
    with_string_pair_lossy_span(a_h, b_h, |a, b| {
        let f = f_opt
            .take()
            .expect("[string/export] with_lossy_string_pair closure missing (lossy)");
        f(a, b)
    })
}

#[inline(always)]
fn concat_pair_from_spans(a_h: i64, b_h: i64) -> Option<i64> {
    let out = with_string_pair_span(a_h, b_h, |a, b| {
        if a.is_empty() {
            return Ok(b_h);
        }
        if b.is_empty() {
            return Ok(a_h);
        }
        Err(concat_two_str(a, b))
    })?;
    match out {
        Ok(h) => Some(h),
        Err(merged) => Some(string_handle_from_owned(merged)),
    }
}

#[inline(always)]
fn concat_pair_from_fast_str(a_h: i64, b_h: i64) -> Option<i64> {
    if a_h <= 0 || b_h <= 0 {
        return None;
    }
    let out = handles::with_pair(a_h as u64, b_h as u64, |a_obj, b_obj| {
        let a_obj = a_obj?;
        let b_obj = b_obj?;
        if let (Some(a_sb), Some(b_sb)) = (
            a_obj.as_any().downcast_ref::<StringBox>(),
            b_obj.as_any().downcast_ref::<StringBox>(),
        ) {
            if a_sb.value.is_empty() {
                return Some(Ok(b_h));
            }
            if b_sb.value.is_empty() {
                return Some(Ok(a_h));
            }
            return Some(Err(concat_two_str(
                a_sb.value.as_str(),
                b_sb.value.as_str(),
            )));
        }
        let a = a_obj.as_ref().as_str_fast()?;
        let b = b_obj.as_ref().as_str_fast()?;
        if a.is_empty() {
            return Some(Ok(b_h));
        }
        if b.is_empty() {
            return Some(Ok(a_h));
        }
        Some(Err(concat_two_str(a, b)))
    })?;
    match out {
        Ok(h) => Some(h),
        Err(merged) => Some(string_handle_from_owned(merged)),
    }
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
fn find_substr_byte_index(hay: &str, needle: &str) -> Option<usize> {
    let hay_b = hay.as_bytes();
    let nee_b = needle.as_bytes();
    match nee_b.len() {
        0 => Some(0),
        1 => memchr(nee_b[0], hay_b),
        _ => memmem::find(hay_b, nee_b),
    }
}

#[inline(always)]
fn bool_to_i64(value: bool) -> i64 {
    if value {
        1
    } else {
        0
    }
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
fn empty_needle_indexof(_hay: &str) -> i64 {
    0
}

#[inline(always)]
fn empty_needle_lastindexof(hay: &str) -> i64 {
    hay.len() as i64
}

#[inline(always)]
fn search_string_pair_hh(
    hay_h: i64,
    needle_h: i64,
    empty_result: fn(&str) -> i64,
    search: fn(&str, &str) -> Option<usize>,
) -> i64 {
    let eval = |hay: &str, nee: &str| -> i64 {
        if nee.is_empty() {
            return empty_result(hay);
        }
        search(hay, nee).map(|pos| pos as i64).unwrap_or(-1)
    };

    with_lossy_string_pair(hay_h, needle_h, |hay, nee| eval(hay, nee))
}

#[inline(always)]
fn compare_string_pair_hh(lhs_h: i64, rhs_h: i64, cmp: fn(&str, &str) -> bool) -> i64 {
    with_lossy_string_pair(lhs_h, rhs_h, |lhs, rhs| bool_to_i64(cmp(lhs, rhs)))
}

#[inline(always)]
fn rfind_substr_byte_index(hay: &str, needle: &str) -> Option<usize> {
    let hay_b = hay.as_bytes();
    let nee_b = needle.as_bytes();
    match nee_b.len() {
        0 => Some(hay_b.len()),
        1 => memrchr(nee_b[0], hay_b),
        _ => memmem::rfind(hay_b, nee_b),
    }
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
    if let Some(v) = hako_string_dispatch(hako_forward_bridge::string_ops::CONCAT_HH, a_h, b_h, 0) {
        return v;
    }
    if !allow_rust_string_fallback() {
        return hook_miss_freeze_handle("string.concat_hh");
    }
    if let Some(out) = concat_pair_from_fast_str(a_h, b_h) {
        return out;
    }
    if let Some(out) = concat_pair_from_spans(a_h, b_h) {
        return out;
    }

    concat_pair_with_materialize(a_h, b_h)
}

// String.concat3_hhh(a_h, b_h, c_h) -> handle
#[export_name = "nyash.string.concat3_hhh"]
pub extern "C" fn nyash_string_concat3_hhh_export(a_h: i64, b_h: i64, c_h: i64) -> i64 {
    if let Some(v) =
        hako_string_dispatch(hako_forward_bridge::string_ops::CONCAT3_HHH, a_h, b_h, c_h)
    {
        return v;
    }
    if !allow_rust_string_fallback() {
        return hook_miss_freeze_handle("string.concat3_hhh");
    }
    // Hot path: resolve all 3 handles once and reuse the same objects for
    // both direct String route and StringView-compatible fallback route.
    if a_h > 0 && b_h > 0 && c_h > 0 {
        if let Some(out) = handles::with_str3(a_h as u64, b_h as u64, c_h as u64, |a, b, c| {
            if a.is_empty() {
                if b.is_empty() {
                    return Ok(c_h);
                }
                if c.is_empty() {
                    return Ok(b_h);
                }
                return Err(concat_two_str(b, c));
            }
            if b.is_empty() {
                if c.is_empty() {
                    return Ok(a_h);
                }
                return Err(concat_two_str(a, c));
            }
            if c.is_empty() {
                return Err(concat_two_str(a, b));
            }
            Err(concat_three_str(a, b, c))
        }) {
            return match out {
                Ok(h) => h,
                Err(merged) => string_handle_from_owned(merged),
            };
        }

        let (a_obj, b_obj, c_obj) = handles::get3(a_h as u64, b_h as u64, c_h as u64);
        if let (Some(a_obj), Some(b_obj), Some(c_obj)) = (a_obj, b_obj, c_obj) {
            if let (Some(a_span), Some(b_span), Some(c_span)) = (
                resolve_string_span_from_obj(a_h, a_obj),
                resolve_string_span_from_obj(b_h, b_obj),
                resolve_string_span_from_obj(c_h, c_obj),
            ) {
                let a = a_span.as_str();
                let b = b_span.as_str();
                let c = c_span.as_str();
                if a.is_empty() {
                    if b.is_empty() {
                        return c_h;
                    }
                    if c.is_empty() {
                        return b_h;
                    }
                    return string_handle_from_owned(concat_two_str(b, c));
                }
                if b.is_empty() {
                    if c.is_empty() {
                        return a_h;
                    }
                    return string_handle_from_owned(concat_two_str(a, c));
                }
                if c.is_empty() {
                    return string_handle_from_owned(concat_two_str(a, b));
                }
                return concat_to_string_handle(&[a, b, c]);
            }
        }
    }

    let a = to_owned_string_handle_arg(a_h);
    let b = to_owned_string_handle_arg(b_h);
    let c = to_owned_string_handle_arg(c_h);
    concat_to_string_handle(&[a.as_str(), b.as_str(), c.as_str()])
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
    enum BorrowedSubstringPlan {
        ReturnHandle,
        ReturnEmpty,
        Materialize(String),
        CreateView {
            base_handle: i64,
            base_obj: Arc<dyn NyashBox>,
            start: usize,
            end: usize,
        },
        Fallback,
    }
    if let Some(plan) = handles::with_handle(h as u64, |obj| {
        let Some(obj) = obj else {
            return None;
        };
        if let Some(sb) = obj.as_any().downcast_ref::<StringBox>() {
            let (st_rel, en_rel) = clamp_i64_range(sb.value.len(), start, end);
            if st_rel == 0 && en_rel == sb.value.len() {
                return Some(BorrowedSubstringPlan::ReturnHandle);
            }
            if st_rel == en_rel {
                return Some(BorrowedSubstringPlan::ReturnEmpty);
            }
            let Some(sub_slice) = sb.value.get(st_rel..en_rel) else {
                // Preserve legacy byte-range contract: invalid UTF-8 boundary slice => empty string.
                return Some(BorrowedSubstringPlan::ReturnEmpty);
            };
            if !substring_view_enabled() {
                return Some(BorrowedSubstringPlan::Materialize(sub_slice.to_string()));
            }
            return Some(BorrowedSubstringPlan::CreateView {
                base_handle: h,
                base_obj: obj.clone(),
                start: st_rel,
                end: en_rel,
            });
        }
        if let Some(view) = obj.as_any().downcast_ref::<StringViewBox>() {
            let Some(base_sb) = view.base_obj.as_any().downcast_ref::<StringBox>() else {
                return Some(BorrowedSubstringPlan::Fallback);
            };
            let mut parent_st = view.start.min(base_sb.value.len());
            let mut parent_en = view.end.min(base_sb.value.len());
            if parent_en < parent_st {
                std::mem::swap(&mut parent_st, &mut parent_en);
            }
            let parent_len = parent_en.saturating_sub(parent_st);
            let (st_rel, en_rel) = clamp_i64_range(parent_len, start, end);
            if st_rel == 0 && en_rel == parent_len {
                return Some(BorrowedSubstringPlan::ReturnHandle);
            }
            if st_rel == en_rel {
                return Some(BorrowedSubstringPlan::ReturnEmpty);
            }
            let abs_st = parent_st.saturating_add(st_rel);
            let abs_en = parent_st.saturating_add(en_rel);
            let Some(sub_slice) = base_sb.value.get(abs_st..abs_en) else {
                return Some(BorrowedSubstringPlan::ReturnEmpty);
            };
            if !substring_view_enabled() {
                return Some(BorrowedSubstringPlan::Materialize(sub_slice.to_string()));
            }
            return Some(BorrowedSubstringPlan::CreateView {
                base_handle: view.base_handle,
                base_obj: view.base_obj.clone(),
                start: abs_st,
                end: abs_en,
            });
        }
        Some(BorrowedSubstringPlan::Fallback)
    }) {
        match plan {
            BorrowedSubstringPlan::ReturnHandle => return h,
            BorrowedSubstringPlan::ReturnEmpty => return shared_empty_string_handle(),
            BorrowedSubstringPlan::Materialize(value) => return string_handle_from_owned(value),
            BorrowedSubstringPlan::CreateView {
                base_handle,
                base_obj,
                start,
                end,
            } => {
                let view = StringViewBox::new(base_handle, base_obj, start, end);
                let arc: Arc<dyn NyashBox> = Arc::new(view);
                return handles::to_handle_arc(arc) as i64;
            }
            BorrowedSubstringPlan::Fallback => {}
        }
    }
    let Some(span) = resolve_string_span_from_handle_nocache(h) else {
        return shared_empty_string_handle();
    };
    let span_len = span.len();
    let (st_rel, en_rel) = clamp_i64_range(span.len(), start, end);
    if st_rel == 0 && en_rel == span_len {
        return h;
    }
    if st_rel == en_rel {
        return shared_empty_string_handle();
    }
    let sub_opt = span.as_str().get(st_rel..en_rel);
    let sub_slice = sub_opt.unwrap_or("");

    if !substring_view_enabled() {
        return string_handle_from_owned(sub_slice.to_string());
    }

    if sub_opt.is_none() {
        // Preserve legacy byte-range contract: invalid UTF-8 boundary slice => empty string.
        return shared_empty_string_handle();
    }

    let view = string_view_from_span_range(&span, st_rel, en_rel);
    let arc: Arc<dyn NyashBox> = Arc::new(view);
    handles::to_handle_arc(arc) as i64
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
