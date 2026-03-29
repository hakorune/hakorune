use super::string_view::StringSpan;
use nyash_rust::config::env::StringSpanCachePolicyMode;
use std::cell::RefCell;

trait StringSpanCachePolicy {
    const SLOTS: usize;
    const MAX_SPAN_BYTES: usize;

    fn should_admit(handle: i64, span: &StringSpan) -> bool;
    fn should_promote(hit_idx: usize) -> bool;
}

struct DefaultStringSpanCachePolicy;

impl StringSpanCachePolicy for DefaultStringSpanCachePolicy {
    const SLOTS: usize = 2;
    const MAX_SPAN_BYTES: usize = 256;

    #[inline(always)]
    fn should_admit(handle: i64, span: &StringSpan) -> bool {
        string_span_cache_enabled() && handle > 0 && span.span_bytes_len() <= Self::MAX_SPAN_BYTES
    }

    #[inline(always)]
    fn should_promote(hit_idx: usize) -> bool {
        hit_idx > 0
    }
}

type ActiveStringSpanCachePolicy = DefaultStringSpanCachePolicy;

// Keep small-to-medium spans in the TLS cache; larger strings are common
// in kilo/text loops and benefit from avoiding repeated handle map lookups.
const STRING_SPAN_CACHE_SLOTS: usize = ActiveStringSpanCachePolicy::SLOTS;
const _: [(); 2] = [(); STRING_SPAN_CACHE_SLOTS];

#[inline(always)]
fn active_string_span_cache_policy_mode() -> StringSpanCachePolicyMode {
    nyash_rust::config::env::string_span_cache_policy_mode()
}

#[inline(always)]
fn string_span_cache_enabled() -> bool {
    matches!(
        active_string_span_cache_policy_mode(),
        StringSpanCachePolicyMode::On
    )
}

#[derive(Clone)]
struct StringSpanCacheEntry {
    handle: i64,
    span: StringSpan,
}

struct StringSpanCacheState {
    drop_epoch: u64,
    slots: [Option<StringSpanCacheEntry>; STRING_SPAN_CACHE_SLOTS],
}

impl StringSpanCacheState {
    const fn new() -> Self {
        Self {
            drop_epoch: 0,
            slots: [None, None],
        }
    }

    fn ensure_epoch(&mut self, drop_epoch: u64) {
        if self.drop_epoch != drop_epoch {
            self.drop_epoch = drop_epoch;
            self.slots = [None, None];
        }
    }
}

thread_local! {
    // Tiny per-thread cache to keep small stable handle working sets hot.
    static STRING_SPAN_CACHE: RefCell<StringSpanCacheState> = const { RefCell::new(StringSpanCacheState::new()) };
}

pub(super) fn string_span_cache_get(handle: i64, drop_epoch: u64) -> Option<StringSpan> {
    if !string_span_cache_enabled() {
        return None;
    }
    STRING_SPAN_CACHE.with(|cache| {
        let mut state = cache.borrow_mut();
        state.ensure_epoch(drop_epoch);
        string_span_cache_lookup_promote(&mut state.slots, handle)
    })
}

pub(super) fn string_span_cache_get_pair(
    a_h: i64,
    b_h: i64,
    drop_epoch: u64,
) -> (Option<StringSpan>, Option<StringSpan>) {
    if !string_span_cache_enabled() {
        return (None, None);
    }
    STRING_SPAN_CACHE.with(|cache| {
        let mut state = cache.borrow_mut();
        state.ensure_epoch(drop_epoch);
        let slots = &mut state.slots;
        let a_span = string_span_cache_lookup_promote(slots, a_h);
        let b_span = if a_h == b_h {
            a_span.clone()
        } else {
            string_span_cache_lookup_promote(slots, b_h)
        };
        (a_span, b_span)
    })
}

pub(super) fn string_span_cache_get_triplet(
    a_h: i64,
    b_h: i64,
    c_h: i64,
    drop_epoch: u64,
) -> (Option<StringSpan>, Option<StringSpan>, Option<StringSpan>) {
    if !string_span_cache_enabled() {
        return (None, None, None);
    }
    STRING_SPAN_CACHE.with(|cache| {
        let mut state = cache.borrow_mut();
        state.ensure_epoch(drop_epoch);
        let slots = &mut state.slots;
        let a_span = string_span_cache_lookup_promote(slots, a_h);
        let b_span = if b_h == a_h {
            a_span.clone()
        } else {
            string_span_cache_lookup_promote(slots, b_h)
        };
        let c_span = if c_h == a_h {
            a_span.clone()
        } else if c_h == b_h {
            b_span.clone()
        } else {
            string_span_cache_lookup_promote(slots, c_h)
        };
        (a_span, b_span, c_span)
    })
}

#[inline(always)]
fn string_span_cache_lookup_promote(
    slots: &mut [Option<StringSpanCacheEntry>; STRING_SPAN_CACHE_SLOTS],
    handle: i64,
) -> Option<StringSpan> {
    if let Some(entry) = slots[0].as_ref() {
        if entry.handle == handle {
            return Some(entry.span.clone());
        }
    }
    for idx in 1..STRING_SPAN_CACHE_SLOTS {
        let Some(entry) = slots[idx].as_ref() else {
            continue;
        };
        if entry.handle == handle {
            let span = entry.span.clone();
            if ActiveStringSpanCachePolicy::should_promote(idx) {
                slots.swap(0, idx);
            }
            return Some(span);
        }
    }
    None
}

#[inline(always)]
fn string_span_cache_insert_front(
    slots: &mut [Option<StringSpanCacheEntry>; STRING_SPAN_CACHE_SLOTS],
    entry: StringSpanCacheEntry,
) {
    if slots[0]
        .as_ref()
        .is_some_and(|cached| cached.handle == entry.handle)
    {
        slots[0] = Some(entry);
        return;
    }
    if let Some(idx) = (1..STRING_SPAN_CACHE_SLOTS).find(|&idx| {
        slots[idx]
            .as_ref()
            .is_some_and(|cached| cached.handle == entry.handle)
    }) {
        slots[idx] = Some(entry);
        slots.swap(0, idx);
        return;
    }

    for idx in (1..STRING_SPAN_CACHE_SLOTS).rev() {
        slots[idx] = slots[idx - 1].take();
    }
    slots[0] = Some(entry);
}

pub(super) fn string_span_cache_put(handle: i64, drop_epoch: u64, span: &StringSpan) {
    if !ActiveStringSpanCachePolicy::should_admit(handle, span) {
        return;
    }
    STRING_SPAN_CACHE.with(|cache| {
        let mut state = cache.borrow_mut();
        state.ensure_epoch(drop_epoch);
        let slots = &mut state.slots;
        let entry = StringSpanCacheEntry {
            handle,
            span: span.clone(),
        };
        string_span_cache_insert_front(slots, entry);
    });
}
