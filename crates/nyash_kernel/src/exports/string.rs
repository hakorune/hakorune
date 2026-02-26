// String-related C ABI exports.

use super::string_view::{
    clamp_i64_range, resolve_string_span_from_handle, resolve_string_span_from_obj,
    resolve_string_span_pair_from_handles, string_is_empty_from_handle as string_is_empty_impl,
    string_len_from_handle as string_len_impl, string_view_from_span_range,
};
use memchr::{memchr, memmem, memrchr};
use nyash_rust::{
    box_trait::{NyashBox, StringBox},
    runtime::host_handles as handles,
};
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

pub(crate) fn string_len_from_handle(handle: i64) -> Option<i64> {
    string_len_impl(handle)
}

pub(crate) fn string_is_empty_from_handle(handle: i64) -> Option<bool> {
    string_is_empty_impl(handle)
}

fn substring_view_enabled() -> bool {
    static SUBSTRING_VIEW_ENABLED: OnceLock<bool> = OnceLock::new();
    env_flag_cached(&SUBSTRING_VIEW_ENABLED, "NYASH_LLVM_FAST")
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
    let merged = with_string_pair_span(a_h, b_h, |a, b| {
        let mut out = String::with_capacity(a.len() + b.len());
        out.push_str(a);
        out.push_str(b);
        out
    })?;
    Some(string_handle_from_owned(merged))
}

#[inline(always)]
fn concat_pair_from_fast_str(a_h: i64, b_h: i64) -> Option<i64> {
    let merged = with_string_pair_fast_str(a_h, b_h, |a, b| {
        let mut out = String::with_capacity(a.len() + b.len());
        out.push_str(a);
        out.push_str(b);
        out
    })?;
    Some(string_handle_from_owned(merged))
}

#[inline(always)]
fn concat_pair_with_materialize(a_h: i64, b_h: i64) -> i64 {
    let lhs = to_owned_string_handle_arg(a_h);
    let rhs = to_owned_string_handle_arg(b_h);
    concat_to_string_handle(&[lhs.as_str(), rhs.as_str()])
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
    // Hot path: resolve all 3 handles once and reuse the same objects for
    // both direct String route and StringView-compatible fallback route.
    if a_h > 0 && b_h > 0 && c_h > 0 {
        if let Some(out) = handles::with_str3(a_h as u64, b_h as u64, c_h as u64, |a, b, c| {
            let mut out = String::with_capacity(a.len() + b.len() + c.len());
            out.push_str(a);
            out.push_str(b);
            out.push_str(c);
            out
        }) {
            return string_handle_from_owned(out);
        }

        let (a_obj, b_obj, c_obj) = handles::get3(a_h as u64, b_h as u64, c_h as u64);
        if let (Some(a_obj), Some(b_obj), Some(c_obj)) = (a_obj, b_obj, c_obj) {
            if let (Some(a_span), Some(b_span), Some(c_span)) = (
                resolve_string_span_from_obj(a_h, a_obj),
                resolve_string_span_from_obj(b_h, b_obj),
                resolve_string_span_from_obj(c_h, c_obj),
            ) {
                return concat_to_string_handle(&[
                    a_span.as_str(),
                    b_span.as_str(),
                    c_span.as_str(),
                ]);
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
    compare_string_pair_hh(a_h, b_h, |lhs, rhs| lhs == rhs)
}

// String.substring_hii(handle, start, end) -> handle
#[export_name = "nyash.string.substring_hii"]
pub extern "C" fn nyash_string_substring_hii_export(h: i64, start: i64, end: i64) -> i64 {
    if h <= 0 {
        return 0;
    }
    let Some(span) = resolve_string_span_from_handle(h) else {
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
    search_string_pair_hh(h, n, empty_needle_indexof, find_substr_byte_index)
}

// String.lastIndexOf_hh(haystack_h, needle_h) -> i64
#[export_name = "nyash.string.lastIndexOf_hh"]
pub extern "C" fn nyash_string_lastindexof_hh_export(h: i64, n: i64) -> i64 {
    search_string_pair_hh(h, n, empty_needle_lastindexof, rfind_substr_byte_index)
}

// String.lt_hh(lhs_h, rhs_h) -> i64 (0/1)
#[export_name = "nyash.string.lt_hh"]
pub extern "C" fn nyash_string_lt_hh_export(a_h: i64, b_h: i64) -> i64 {
    compare_string_pair_hh(a_h, b_h, |lhs, rhs| lhs < rhs)
}

// Construct StringBox from two u64 words (little-endian) + length (<=16) and return handle
// export: nyash.string.from_u64x2(lo, hi, len) -> i64
#[export_name = "nyash.string.from_u64x2"]
pub extern "C" fn nyash_string_from_u64x2_export(lo: i64, hi: i64, len: i64) -> i64 {
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
