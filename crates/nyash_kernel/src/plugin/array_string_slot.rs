use super::value_codec::{
    string_handle_or_immediate_box_from_obj, try_retarget_borrowed_string_slot,
    try_retarget_borrowed_string_slot_with_source,
};
use crate::exports::string_view::resolve_string_span_from_handle;
use memchr::{memchr, memmem};
use nyash_rust::boxes::array::ArrayBox;
use nyash_rust::runtime::host_handles as handles;
use std::cell::RefCell;

struct CachedNeedle {
    handle: i64,
    drop_epoch: u64,
    value: String,
}

thread_local! {
    static ARRAY_STRING_INDEXOF_NEEDLE_CACHE: RefCell<Option<CachedNeedle>> = const { RefCell::new(None) };
}

pub(super) fn array_string_len_by_index(handle: i64, idx: i64) -> i64 {
    if handle <= 0 || idx < 0 {
        return 0;
    }
    handles::with_handle(handle as u64, |arr_obj| {
        let Some(obj) = arr_obj else {
            return 0;
        };
        let Some(arr) = obj.as_any().downcast_ref::<ArrayBox>() else {
            return 0;
        };
        let idx = idx as usize;
        arr.with_items_read(|items| {
            let Some(item) = items.get(idx) else {
                return 0;
            };
            item.as_str_fast().map(|s| s.len() as i64).unwrap_or(0)
        })
    })
}

#[inline(always)]
fn string_indexof_fast_str(hay: &str, needle: &str) -> i64 {
    if needle.is_empty() {
        return 0;
    }
    let hay_b = hay.as_bytes();
    let nee_b = needle.as_bytes();
    match nee_b.len() {
        1 => memchr(nee_b[0], hay_b).map(|pos| pos as i64).unwrap_or(-1),
        2 | 3 | 4 => string_indexof_fast_str_small(hay_b, nee_b),
        _ => memmem::find(hay_b, nee_b)
            .map(|pos| pos as i64)
            .unwrap_or(-1),
    }
}

#[inline(always)]
fn string_indexof_fast_str_small(hay_b: &[u8], nee_b: &[u8]) -> i64 {
    let first = nee_b[0];
    let needle_len = nee_b.len();
    let mut offset = 0usize;
    let mut search = hay_b;

    while let Some(pos) = memchr(first, search) {
        let idx = offset + pos;
        let end = idx + needle_len;
        if end <= hay_b.len() && &hay_b[idx..end] == nee_b {
            return idx as i64;
        }
        offset = idx + 1;
        if offset >= hay_b.len() {
            return -1;
        }
        search = &hay_b[offset..];
    }

    -1
}

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
pub(super) fn array_string_indexof_by_index(handle: i64, idx: i64, needle_h: i64) -> i64 {
    with_cached_needle_str(needle_h, |needle| {
        if handle <= 0 || idx < 0 {
            return if needle.is_empty() { 0 } else { -1 };
        }
        if needle.is_empty() {
            return 0;
        }
        super::handle_cache::with_array_box(handle, |arr| {
            let idx = idx as usize;
            arr.with_items_read(|items| {
                items
                    .get(idx)
                    .and_then(|item| item.as_str_fast())
                    .map(|hay| string_indexof_fast_str(hay, needle))
                    .unwrap_or(-1)
            })
        })
        .unwrap_or(-1)
    })
}

pub(super) fn array_set_by_index_string_handle_value(handle: i64, idx: i64, value_h: i64) -> i64 {
    if handle <= 0 || idx < 0 || value_h <= 0 {
        return 0;
    }
    let drop_epoch = handles::drop_epoch();
    let value_obj = super::handle_cache::object_from_handle_cached(value_h);
    super::handle_cache::with_array_box(handle, |arr| {
        let idx = idx as usize;
        arr.with_items_write(|items| {
            if idx < items.len() {
                if let Some(value_obj) = value_obj.as_ref() {
                    if try_retarget_borrowed_string_slot_with_source(
                        &mut items[idx],
                        value_h,
                        value_obj,
                        drop_epoch,
                    ) {
                        return 1;
                    }
                } else if try_retarget_borrowed_string_slot(&mut items[idx], value_h) {
                    return 1;
                }
                let value = string_handle_or_immediate_box_from_obj(
                    value_obj.as_ref(),
                    value_h,
                    drop_epoch,
                );
                items[idx] = value;
                return 1;
            }
            if idx == items.len() {
                let value = string_handle_or_immediate_box_from_obj(
                    value_obj.as_ref(),
                    value_h,
                    drop_epoch,
                );
                items.push(value);
                return 1;
            }
            0
        })
    })
    .unwrap_or(0)
}
