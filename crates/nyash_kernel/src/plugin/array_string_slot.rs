use super::array_guard::valid_handle_idx;
use super::handle_cache::{cache_probe_kind, CacheProbeKind as HandleCacheProbeKind};
use super::value_codec::{
    is_string_handle_source, maybe_store_string_box_from_verified_source,
    try_retarget_borrowed_string_slot_verified,
};
use crate::perf_counters::{self, CacheProbeKind as PerfCacheProbeKind};
use crate::exports::string_view::resolve_string_span_from_handle;
use memchr::{memchr, memmem};
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
    if !valid_handle_idx(handle, idx) {
        return 0;
    }
    super::array_handle_cache::with_array_box(handle, |arr| {
        let idx = idx as usize;
        arr.with_items_read(|items| {
            let Some(item) = items.get(idx) else {
                return 0;
            };
            item.as_str_fast().map(|s| s.len() as i64).unwrap_or(0)
        })
    })
    .unwrap_or(0)
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
        if !valid_handle_idx(handle, idx) {
            return if needle.is_empty() { 0 } else { -1 };
        }
        if needle.is_empty() {
            return 0;
        }
        super::array_handle_cache::with_array_box(handle, |arr| {
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

#[inline(always)]
fn execute_store_array_str_slot(
    items: &mut Vec<Box<dyn nyash_rust::box_trait::NyashBox>>,
    idx: usize,
    value_h: i64,
    source_obj: Option<&std::sync::Arc<dyn nyash_rust::box_trait::NyashBox>>,
    drop_epoch: u64,
) -> i64 {
    if idx > items.len() {
        return 0;
    }
    let source_is_string = source_obj.is_some_and(is_string_handle_source);
    if idx < items.len() {
        if source_is_string {
            if let Some(value_obj) = source_obj {
                if try_retarget_borrowed_string_slot_verified(
                    &mut items[idx],
                    value_h,
                    value_obj,
                    drop_epoch,
                ) {
                    perf_counters::record_store_array_str_retarget_hit();
                    return 1;
                }
            }
        }
    }
    if source_is_string {
        perf_counters::record_store_array_str_source_store();
    } else {
        perf_counters::record_store_array_str_non_string_source();
    }
    let value = maybe_store_string_box_from_verified_source(
        value_h,
        source_obj,
        drop_epoch,
        source_is_string,
    );
    if idx < items.len() {
        items[idx] = value;
    } else {
        items.push(value);
    }
    1
}

#[inline(always)]
fn execute_store_array_str_contract(handle: i64, idx: i64, value_h: i64) -> i64 {
    if !valid_handle_idx(handle, idx) || value_h <= 0 {
        return 0;
    }
    let drop_epoch = handles::drop_epoch();
    perf_counters::record_store_array_str_enter();
    if perf_counters::enabled() {
        let kind = match cache_probe_kind(handle, drop_epoch) {
            HandleCacheProbeKind::Hit => PerfCacheProbeKind::Hit,
            HandleCacheProbeKind::MissHandle => PerfCacheProbeKind::MissHandle,
            HandleCacheProbeKind::MissDropEpoch => PerfCacheProbeKind::MissDropEpoch,
        };
        perf_counters::record_store_array_str_cache_probe(kind);
    }
    super::array_handle_cache::with_array_box(handle, |arr| {
        let idx = idx as usize;
        arr.with_items_write(|items| {
            handles::with_handle(value_h as u64, |source_obj| {
                execute_store_array_str_slot(items, idx, value_h, source_obj, drop_epoch)
            })
        })
    })
    .unwrap_or(0)
}

#[inline(always)]
pub(super) fn array_string_store_handle_at(handle: i64, idx: i64, value_h: i64) -> i64 {
    // phase-150x: keep array-string store semantics owned above this layer and
    // treat the Rust path as the executor for the canonical `store.array.str`
    // reading only.
    execute_store_array_str_contract(handle, idx, value_h)
}
