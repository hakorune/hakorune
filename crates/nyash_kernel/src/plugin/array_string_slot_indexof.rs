use super::super::array_guard::valid_handle_idx;
use super::super::array_handle_cache::with_array_box;
use super::array_string_slot_helpers::{
    array_text_read_ref_demand, string_indexof_fast_str, with_compiler_const_utf8_ptr_len,
    CachedNeedle, ARRAY_STRING_INDEXOF_NEEDLE_CACHE,
};
use crate::exports::string_view::resolve_string_span_from_handle;
use nyash_rust::runtime::host_handles as handles;

#[inline(always)]
fn with_cached_needle_str<R>(needle_h: i64, f: impl FnOnce(&str) -> R) -> R {
    let drop_epoch = handles::drop_epoch();
    ARRAY_STRING_INDEXOF_NEEDLE_CACHE.with(|slot| {
        if let Some(cached) = slot.borrow().as_ref() {
            if cached.handle == needle_h && cached.drop_epoch == drop_epoch {
                return f(cached.value.as_str());
            }
        }
        let value = resolve_string_span_from_handle(needle_h)
            .map(|span| span.as_str().to_owned())
            .unwrap_or_default();
        *slot.borrow_mut() = Some(CachedNeedle {
            handle: needle_h,
            drop_epoch,
            value,
        });
        let borrowed = slot.borrow();
        let cached = borrowed
            .as_ref()
            .expect("[array/string_indexof] needle cache just initialized");
        f(cached.value.as_str())
    })
}

#[inline(always)]
pub(in super::super) fn array_string_len_by_index(handle: i64, idx: i64) -> i64 {
    let _demand = array_text_read_ref_demand();
    if !valid_handle_idx(handle, idx) {
        return 0;
    }
    with_array_box(handle, |arr| arr.slot_text_len_raw(idx).unwrap_or(0)).unwrap_or(0)
}

#[inline(always)]
pub(in super::super) fn array_string_indexof_by_index(handle: i64, idx: i64, needle_h: i64) -> i64 {
    let _demand = array_text_read_ref_demand();
    with_cached_needle_str(needle_h, |needle| {
        array_string_indexof_by_index_str(handle, idx, needle)
    })
}

#[inline(always)]
pub(in super::super) fn array_string_indexof_by_index_const_utf8(
    handle: i64,
    idx: i64,
    needle_ptr: *const i8,
    needle_len: i64,
) -> i64 {
    let _demand = array_text_read_ref_demand();
    with_compiler_const_utf8_ptr_len(needle_ptr, needle_len, |needle| {
        array_string_indexof_by_index_str(handle, idx, needle)
    })
    .unwrap_or(-1)
}

#[inline(always)]
fn array_string_indexof_by_index_str(handle: i64, idx: i64, needle: &str) -> i64 {
    if !valid_handle_idx(handle, idx) {
        return if needle.is_empty() { 0 } else { -1 };
    }
    if needle.is_empty() {
        return 0;
    }
    with_array_box(handle, |arr| {
        arr.slot_with_text_raw(idx, |hay| string_indexof_fast_str(hay, needle))
            .unwrap_or(-1)
    })
    .unwrap_or(-1)
}
