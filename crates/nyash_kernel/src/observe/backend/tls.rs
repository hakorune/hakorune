use std::cell::Cell;
use std::sync::atomic::{AtomicU64, Ordering};

use super::super::config;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CacheProbeKind {
    Hit,
    MissHandle,
    MissDropEpoch,
}

struct GlobalCounters {
    store_array_str_total: AtomicU64,
    store_array_str_cache_hit: AtomicU64,
    store_array_str_cache_miss_handle: AtomicU64,
    store_array_str_cache_miss_epoch: AtomicU64,
    store_array_str_retarget_hit: AtomicU64,
    store_array_str_source_store: AtomicU64,
    store_array_str_non_string_source: AtomicU64,
    store_array_str_existing_slot: AtomicU64,
    store_array_str_append_slot: AtomicU64,
    store_array_str_source_string_box: AtomicU64,
    store_array_str_source_string_view: AtomicU64,
    store_array_str_source_missing: AtomicU64,
    const_suffix_total: AtomicU64,
    const_suffix_cached_handle_hit: AtomicU64,
    const_suffix_text_cache_reload: AtomicU64,
    const_suffix_freeze_fallback: AtomicU64,
    const_suffix_empty_return: AtomicU64,
    const_suffix_cached_fast_str_hit: AtomicU64,
    const_suffix_cached_span_hit: AtomicU64,
    birth_placement_return_handle: AtomicU64,
    birth_placement_borrow_view: AtomicU64,
    birth_placement_freeze_owned: AtomicU64,
    birth_placement_fresh_handle: AtomicU64,
    birth_placement_materialize_owned: AtomicU64,
    birth_placement_store_from_source: AtomicU64,
    birth_backend_freeze_text_plan_total: AtomicU64,
    birth_backend_freeze_text_plan_view1: AtomicU64,
    birth_backend_freeze_text_plan_pieces2: AtomicU64,
    birth_backend_freeze_text_plan_pieces3: AtomicU64,
    birth_backend_freeze_text_plan_pieces4: AtomicU64,
    birth_backend_freeze_text_plan_owned_tmp: AtomicU64,
    birth_backend_materialize_owned_total: AtomicU64,
    birth_backend_materialize_owned_bytes: AtomicU64,
    birth_backend_gc_alloc_called: AtomicU64,
    birth_backend_gc_alloc_bytes: AtomicU64,
}

impl GlobalCounters {
    const fn new() -> Self {
        Self {
            store_array_str_total: AtomicU64::new(0),
            store_array_str_cache_hit: AtomicU64::new(0),
            store_array_str_cache_miss_handle: AtomicU64::new(0),
            store_array_str_cache_miss_epoch: AtomicU64::new(0),
            store_array_str_retarget_hit: AtomicU64::new(0),
            store_array_str_source_store: AtomicU64::new(0),
            store_array_str_non_string_source: AtomicU64::new(0),
            store_array_str_existing_slot: AtomicU64::new(0),
            store_array_str_append_slot: AtomicU64::new(0),
            store_array_str_source_string_box: AtomicU64::new(0),
            store_array_str_source_string_view: AtomicU64::new(0),
            store_array_str_source_missing: AtomicU64::new(0),
            const_suffix_total: AtomicU64::new(0),
            const_suffix_cached_handle_hit: AtomicU64::new(0),
            const_suffix_text_cache_reload: AtomicU64::new(0),
            const_suffix_freeze_fallback: AtomicU64::new(0),
            const_suffix_empty_return: AtomicU64::new(0),
            const_suffix_cached_fast_str_hit: AtomicU64::new(0),
            const_suffix_cached_span_hit: AtomicU64::new(0),
            birth_placement_return_handle: AtomicU64::new(0),
            birth_placement_borrow_view: AtomicU64::new(0),
            birth_placement_freeze_owned: AtomicU64::new(0),
            birth_placement_fresh_handle: AtomicU64::new(0),
            birth_placement_materialize_owned: AtomicU64::new(0),
            birth_placement_store_from_source: AtomicU64::new(0),
            birth_backend_freeze_text_plan_total: AtomicU64::new(0),
            birth_backend_freeze_text_plan_view1: AtomicU64::new(0),
            birth_backend_freeze_text_plan_pieces2: AtomicU64::new(0),
            birth_backend_freeze_text_plan_pieces3: AtomicU64::new(0),
            birth_backend_freeze_text_plan_pieces4: AtomicU64::new(0),
            birth_backend_freeze_text_plan_owned_tmp: AtomicU64::new(0),
            birth_backend_materialize_owned_total: AtomicU64::new(0),
            birth_backend_materialize_owned_bytes: AtomicU64::new(0),
            birth_backend_gc_alloc_called: AtomicU64::new(0),
            birth_backend_gc_alloc_bytes: AtomicU64::new(0),
        }
    }
}

static GLOBAL: GlobalCounters = GlobalCounters::new();

struct ThreadCounters {
    store_array_str_total: Cell<u64>,
    store_array_str_cache_hit: Cell<u64>,
    store_array_str_cache_miss_handle: Cell<u64>,
    store_array_str_cache_miss_epoch: Cell<u64>,
    store_array_str_retarget_hit: Cell<u64>,
    store_array_str_source_store: Cell<u64>,
    store_array_str_non_string_source: Cell<u64>,
    store_array_str_existing_slot: Cell<u64>,
    store_array_str_append_slot: Cell<u64>,
    store_array_str_source_string_box: Cell<u64>,
    store_array_str_source_string_view: Cell<u64>,
    store_array_str_source_missing: Cell<u64>,
    const_suffix_total: Cell<u64>,
    const_suffix_cached_handle_hit: Cell<u64>,
    const_suffix_text_cache_reload: Cell<u64>,
    const_suffix_freeze_fallback: Cell<u64>,
    const_suffix_empty_return: Cell<u64>,
    const_suffix_cached_fast_str_hit: Cell<u64>,
    const_suffix_cached_span_hit: Cell<u64>,
    birth_placement_return_handle: Cell<u64>,
    birth_placement_borrow_view: Cell<u64>,
    birth_placement_freeze_owned: Cell<u64>,
    birth_placement_fresh_handle: Cell<u64>,
    birth_placement_materialize_owned: Cell<u64>,
    birth_placement_store_from_source: Cell<u64>,
    birth_backend_freeze_text_plan_total: Cell<u64>,
    birth_backend_freeze_text_plan_view1: Cell<u64>,
    birth_backend_freeze_text_plan_pieces2: Cell<u64>,
    birth_backend_freeze_text_plan_pieces3: Cell<u64>,
    birth_backend_freeze_text_plan_pieces4: Cell<u64>,
    birth_backend_freeze_text_plan_owned_tmp: Cell<u64>,
    birth_backend_materialize_owned_total: Cell<u64>,
    birth_backend_materialize_owned_bytes: Cell<u64>,
    birth_backend_gc_alloc_called: Cell<u64>,
    birth_backend_gc_alloc_bytes: Cell<u64>,
}

impl ThreadCounters {
    const fn new() -> Self {
        Self {
            store_array_str_total: Cell::new(0),
            store_array_str_cache_hit: Cell::new(0),
            store_array_str_cache_miss_handle: Cell::new(0),
            store_array_str_cache_miss_epoch: Cell::new(0),
            store_array_str_retarget_hit: Cell::new(0),
            store_array_str_source_store: Cell::new(0),
            store_array_str_non_string_source: Cell::new(0),
            store_array_str_existing_slot: Cell::new(0),
            store_array_str_append_slot: Cell::new(0),
            store_array_str_source_string_box: Cell::new(0),
            store_array_str_source_string_view: Cell::new(0),
            store_array_str_source_missing: Cell::new(0),
            const_suffix_total: Cell::new(0),
            const_suffix_cached_handle_hit: Cell::new(0),
            const_suffix_text_cache_reload: Cell::new(0),
            const_suffix_freeze_fallback: Cell::new(0),
            const_suffix_empty_return: Cell::new(0),
            const_suffix_cached_fast_str_hit: Cell::new(0),
            const_suffix_cached_span_hit: Cell::new(0),
            birth_placement_return_handle: Cell::new(0),
            birth_placement_borrow_view: Cell::new(0),
            birth_placement_freeze_owned: Cell::new(0),
            birth_placement_fresh_handle: Cell::new(0),
            birth_placement_materialize_owned: Cell::new(0),
            birth_placement_store_from_source: Cell::new(0),
            birth_backend_freeze_text_plan_total: Cell::new(0),
            birth_backend_freeze_text_plan_view1: Cell::new(0),
            birth_backend_freeze_text_plan_pieces2: Cell::new(0),
            birth_backend_freeze_text_plan_pieces3: Cell::new(0),
            birth_backend_freeze_text_plan_pieces4: Cell::new(0),
            birth_backend_freeze_text_plan_owned_tmp: Cell::new(0),
            birth_backend_materialize_owned_total: Cell::new(0),
            birth_backend_materialize_owned_bytes: Cell::new(0),
            birth_backend_gc_alloc_called: Cell::new(0),
            birth_backend_gc_alloc_bytes: Cell::new(0),
        }
    }

    #[inline(always)]
    fn bump(cell: &Cell<u64>) {
        cell.set(cell.get() + 1);
    }

    #[inline(always)]
    fn store_array_str_enter(&self) {
        Self::bump(&self.store_array_str_total);
    }

    #[inline(always)]
    fn store_array_str_cache_probe(&self, kind: CacheProbeKind) {
        match kind {
            CacheProbeKind::Hit => Self::bump(&self.store_array_str_cache_hit),
            CacheProbeKind::MissHandle => Self::bump(&self.store_array_str_cache_miss_handle),
            CacheProbeKind::MissDropEpoch => Self::bump(&self.store_array_str_cache_miss_epoch),
        }
    }

    #[inline(always)]
    fn store_array_str_retarget_hit(&self) {
        Self::bump(&self.store_array_str_retarget_hit);
    }

    #[inline(always)]
    fn store_array_str_source_store(&self) {
        Self::bump(&self.store_array_str_source_store);
    }

    #[inline(always)]
    fn store_array_str_non_string_source(&self) {
        Self::bump(&self.store_array_str_non_string_source);
    }

    #[inline(always)]
    fn store_array_str_existing_slot(&self) {
        Self::bump(&self.store_array_str_existing_slot);
    }

    #[inline(always)]
    fn store_array_str_append_slot(&self) {
        Self::bump(&self.store_array_str_append_slot);
    }

    #[inline(always)]
    fn store_array_str_source_string_box(&self) {
        Self::bump(&self.store_array_str_source_string_box);
    }

    #[inline(always)]
    fn store_array_str_source_string_view(&self) {
        Self::bump(&self.store_array_str_source_string_view);
    }

    #[inline(always)]
    fn store_array_str_source_missing(&self) {
        Self::bump(&self.store_array_str_source_missing);
    }

    #[inline(always)]
    fn const_suffix_enter(&self) {
        Self::bump(&self.const_suffix_total);
    }

    #[inline(always)]
    fn const_suffix_cached_handle_hit(&self) {
        Self::bump(&self.const_suffix_cached_handle_hit);
    }

    #[inline(always)]
    fn const_suffix_text_cache_reload(&self) {
        Self::bump(&self.const_suffix_text_cache_reload);
    }

    #[inline(always)]
    fn const_suffix_freeze_fallback(&self) {
        Self::bump(&self.const_suffix_freeze_fallback);
    }

    #[inline(always)]
    fn const_suffix_empty_return(&self) {
        Self::bump(&self.const_suffix_empty_return);
    }

    #[inline(always)]
    fn const_suffix_cached_fast_str_hit(&self) {
        Self::bump(&self.const_suffix_cached_fast_str_hit);
    }

    #[inline(always)]
    fn const_suffix_cached_span_hit(&self) {
        Self::bump(&self.const_suffix_cached_span_hit);
    }

    #[inline(always)]
    fn birth_placement_return_handle(&self) {
        Self::bump(&self.birth_placement_return_handle);
    }

    #[inline(always)]
    fn birth_placement_borrow_view(&self) {
        Self::bump(&self.birth_placement_borrow_view);
    }

    #[inline(always)]
    fn birth_placement_freeze_owned(&self) {
        Self::bump(&self.birth_placement_freeze_owned);
    }

    #[inline(always)]
    fn birth_placement_fresh_handle(&self) {
        Self::bump(&self.birth_placement_fresh_handle);
    }

    #[inline(always)]
    fn birth_placement_materialize_owned(&self) {
        Self::bump(&self.birth_placement_materialize_owned);
    }

    #[inline(always)]
    fn birth_placement_store_from_source(&self) {
        Self::bump(&self.birth_placement_store_from_source);
    }

    #[inline(always)]
    fn birth_backend_freeze_text_plan_view1(&self) {
        Self::bump(&self.birth_backend_freeze_text_plan_total);
        Self::bump(&self.birth_backend_freeze_text_plan_view1);
    }

    #[inline(always)]
    fn birth_backend_freeze_text_plan_pieces2(&self) {
        Self::bump(&self.birth_backend_freeze_text_plan_total);
        Self::bump(&self.birth_backend_freeze_text_plan_pieces2);
    }

    #[inline(always)]
    fn birth_backend_freeze_text_plan_pieces3(&self) {
        Self::bump(&self.birth_backend_freeze_text_plan_total);
        Self::bump(&self.birth_backend_freeze_text_plan_pieces3);
    }

    #[inline(always)]
    fn birth_backend_freeze_text_plan_pieces4(&self) {
        Self::bump(&self.birth_backend_freeze_text_plan_total);
        Self::bump(&self.birth_backend_freeze_text_plan_pieces4);
    }

    #[inline(always)]
    fn birth_backend_freeze_text_plan_owned_tmp(&self) {
        Self::bump(&self.birth_backend_freeze_text_plan_total);
        Self::bump(&self.birth_backend_freeze_text_plan_owned_tmp);
    }

    #[inline(always)]
    fn birth_backend_materialize_owned(&self, bytes: u64) {
        Self::bump(&self.birth_backend_materialize_owned_total);
        Self::bump(&self.birth_backend_gc_alloc_called);
        self.birth_backend_materialize_owned_bytes
            .set(self.birth_backend_materialize_owned_bytes.get() + bytes);
        self.birth_backend_gc_alloc_bytes
            .set(self.birth_backend_gc_alloc_bytes.get() + bytes);
    }

    fn flush_into_global(&self) {
        flush_cell(&self.store_array_str_total, &GLOBAL.store_array_str_total);
        flush_cell(
            &self.store_array_str_cache_hit,
            &GLOBAL.store_array_str_cache_hit,
        );
        flush_cell(
            &self.store_array_str_cache_miss_handle,
            &GLOBAL.store_array_str_cache_miss_handle,
        );
        flush_cell(
            &self.store_array_str_cache_miss_epoch,
            &GLOBAL.store_array_str_cache_miss_epoch,
        );
        flush_cell(
            &self.store_array_str_retarget_hit,
            &GLOBAL.store_array_str_retarget_hit,
        );
        flush_cell(
            &self.store_array_str_source_store,
            &GLOBAL.store_array_str_source_store,
        );
        flush_cell(
            &self.store_array_str_non_string_source,
            &GLOBAL.store_array_str_non_string_source,
        );
        flush_cell(
            &self.store_array_str_existing_slot,
            &GLOBAL.store_array_str_existing_slot,
        );
        flush_cell(
            &self.store_array_str_append_slot,
            &GLOBAL.store_array_str_append_slot,
        );
        flush_cell(
            &self.store_array_str_source_string_box,
            &GLOBAL.store_array_str_source_string_box,
        );
        flush_cell(
            &self.store_array_str_source_string_view,
            &GLOBAL.store_array_str_source_string_view,
        );
        flush_cell(
            &self.store_array_str_source_missing,
            &GLOBAL.store_array_str_source_missing,
        );
        flush_cell(&self.const_suffix_total, &GLOBAL.const_suffix_total);
        flush_cell(
            &self.const_suffix_cached_handle_hit,
            &GLOBAL.const_suffix_cached_handle_hit,
        );
        flush_cell(
            &self.const_suffix_text_cache_reload,
            &GLOBAL.const_suffix_text_cache_reload,
        );
        flush_cell(
            &self.const_suffix_freeze_fallback,
            &GLOBAL.const_suffix_freeze_fallback,
        );
        flush_cell(
            &self.const_suffix_empty_return,
            &GLOBAL.const_suffix_empty_return,
        );
        flush_cell(
            &self.const_suffix_cached_fast_str_hit,
            &GLOBAL.const_suffix_cached_fast_str_hit,
        );
        flush_cell(
            &self.const_suffix_cached_span_hit,
            &GLOBAL.const_suffix_cached_span_hit,
        );
        flush_cell(
            &self.birth_placement_return_handle,
            &GLOBAL.birth_placement_return_handle,
        );
        flush_cell(
            &self.birth_placement_borrow_view,
            &GLOBAL.birth_placement_borrow_view,
        );
        flush_cell(
            &self.birth_placement_freeze_owned,
            &GLOBAL.birth_placement_freeze_owned,
        );
        flush_cell(
            &self.birth_placement_fresh_handle,
            &GLOBAL.birth_placement_fresh_handle,
        );
        flush_cell(
            &self.birth_placement_materialize_owned,
            &GLOBAL.birth_placement_materialize_owned,
        );
        flush_cell(
            &self.birth_placement_store_from_source,
            &GLOBAL.birth_placement_store_from_source,
        );
        flush_cell(
            &self.birth_backend_freeze_text_plan_total,
            &GLOBAL.birth_backend_freeze_text_plan_total,
        );
        flush_cell(
            &self.birth_backend_freeze_text_plan_view1,
            &GLOBAL.birth_backend_freeze_text_plan_view1,
        );
        flush_cell(
            &self.birth_backend_freeze_text_plan_pieces2,
            &GLOBAL.birth_backend_freeze_text_plan_pieces2,
        );
        flush_cell(
            &self.birth_backend_freeze_text_plan_pieces3,
            &GLOBAL.birth_backend_freeze_text_plan_pieces3,
        );
        flush_cell(
            &self.birth_backend_freeze_text_plan_pieces4,
            &GLOBAL.birth_backend_freeze_text_plan_pieces4,
        );
        flush_cell(
            &self.birth_backend_freeze_text_plan_owned_tmp,
            &GLOBAL.birth_backend_freeze_text_plan_owned_tmp,
        );
        flush_cell(
            &self.birth_backend_materialize_owned_total,
            &GLOBAL.birth_backend_materialize_owned_total,
        );
        flush_cell(
            &self.birth_backend_materialize_owned_bytes,
            &GLOBAL.birth_backend_materialize_owned_bytes,
        );
        flush_cell(
            &self.birth_backend_gc_alloc_called,
            &GLOBAL.birth_backend_gc_alloc_called,
        );
        flush_cell(
            &self.birth_backend_gc_alloc_bytes,
            &GLOBAL.birth_backend_gc_alloc_bytes,
        );
    }
}

impl Drop for ThreadCounters {
    fn drop(&mut self) {
        self.flush_into_global();
    }
}

#[inline(always)]
fn flush_cell(local: &Cell<u64>, global: &AtomicU64) {
    let value = local.replace(0);
    if value != 0 {
        global.fetch_add(value, Ordering::Relaxed);
    }
}

thread_local! {
    static TLS_COUNTERS: ThreadCounters = const { ThreadCounters::new() };
}

#[inline(always)]
fn with_tls(f: impl FnOnce(&ThreadCounters)) {
    if config::enabled() {
        TLS_COUNTERS.with(f);
    }
}

#[inline(always)]
pub(crate) fn store_array_str_enter() {
    with_tls(ThreadCounters::store_array_str_enter);
}

#[inline(always)]
pub(crate) fn store_array_str_cache_probe(kind: CacheProbeKind) {
    with_tls(|tls| tls.store_array_str_cache_probe(kind));
}

#[inline(always)]
pub(crate) fn store_array_str_retarget_hit() {
    with_tls(ThreadCounters::store_array_str_retarget_hit);
}

#[inline(always)]
pub(crate) fn store_array_str_source_store() {
    with_tls(ThreadCounters::store_array_str_source_store);
}

#[inline(always)]
pub(crate) fn store_array_str_non_string_source() {
    with_tls(ThreadCounters::store_array_str_non_string_source);
}

#[inline(always)]
pub(crate) fn store_array_str_existing_slot() {
    with_tls(ThreadCounters::store_array_str_existing_slot);
}

#[inline(always)]
pub(crate) fn store_array_str_append_slot() {
    with_tls(ThreadCounters::store_array_str_append_slot);
}

#[inline(always)]
pub(crate) fn store_array_str_source_string_box() {
    with_tls(ThreadCounters::store_array_str_source_string_box);
}

#[inline(always)]
pub(crate) fn store_array_str_source_string_view() {
    with_tls(ThreadCounters::store_array_str_source_string_view);
}

#[inline(always)]
pub(crate) fn store_array_str_source_missing() {
    with_tls(ThreadCounters::store_array_str_source_missing);
}

#[inline(always)]
pub(crate) fn const_suffix_enter() {
    with_tls(ThreadCounters::const_suffix_enter);
}

#[inline(always)]
pub(crate) fn const_suffix_cached_handle_hit() {
    with_tls(ThreadCounters::const_suffix_cached_handle_hit);
}

#[inline(always)]
pub(crate) fn const_suffix_text_cache_reload() {
    with_tls(ThreadCounters::const_suffix_text_cache_reload);
}

#[inline(always)]
pub(crate) fn const_suffix_freeze_fallback() {
    with_tls(ThreadCounters::const_suffix_freeze_fallback);
}

#[inline(always)]
pub(crate) fn const_suffix_empty_return() {
    with_tls(ThreadCounters::const_suffix_empty_return);
}

#[inline(always)]
pub(crate) fn const_suffix_cached_fast_str_hit() {
    with_tls(ThreadCounters::const_suffix_cached_fast_str_hit);
}

#[inline(always)]
pub(crate) fn const_suffix_cached_span_hit() {
    with_tls(ThreadCounters::const_suffix_cached_span_hit);
}

#[inline(always)]
pub(crate) fn birth_placement_return_handle() {
    with_tls(ThreadCounters::birth_placement_return_handle);
}

#[inline(always)]
pub(crate) fn birth_placement_borrow_view() {
    with_tls(ThreadCounters::birth_placement_borrow_view);
}

#[inline(always)]
pub(crate) fn birth_placement_freeze_owned() {
    with_tls(ThreadCounters::birth_placement_freeze_owned);
}

#[inline(always)]
pub(crate) fn birth_placement_fresh_handle() {
    with_tls(ThreadCounters::birth_placement_fresh_handle);
}

#[inline(always)]
pub(crate) fn birth_placement_materialize_owned() {
    with_tls(ThreadCounters::birth_placement_materialize_owned);
}

#[inline(always)]
pub(crate) fn birth_placement_store_from_source() {
    with_tls(ThreadCounters::birth_placement_store_from_source);
}

#[inline(always)]
pub(crate) fn birth_backend_freeze_text_plan_view1() {
    with_tls(ThreadCounters::birth_backend_freeze_text_plan_view1);
}

#[inline(always)]
pub(crate) fn birth_backend_freeze_text_plan_pieces2() {
    with_tls(ThreadCounters::birth_backend_freeze_text_plan_pieces2);
}

#[inline(always)]
pub(crate) fn birth_backend_freeze_text_plan_pieces3() {
    with_tls(ThreadCounters::birth_backend_freeze_text_plan_pieces3);
}

#[inline(always)]
pub(crate) fn birth_backend_freeze_text_plan_pieces4() {
    with_tls(ThreadCounters::birth_backend_freeze_text_plan_pieces4);
}

#[inline(always)]
pub(crate) fn birth_backend_freeze_text_plan_owned_tmp() {
    with_tls(ThreadCounters::birth_backend_freeze_text_plan_owned_tmp);
}

#[inline(always)]
pub(crate) fn birth_backend_materialize_owned(bytes: u64) {
    with_tls(|tls| tls.birth_backend_materialize_owned(bytes));
}

fn flush_current_thread() {
    TLS_COUNTERS.with(ThreadCounters::flush_into_global);
}

pub(crate) fn snapshot() -> [u64; 35] {
    flush_current_thread();
    [
        GLOBAL.store_array_str_total.load(Ordering::Relaxed),
        GLOBAL.store_array_str_cache_hit.load(Ordering::Relaxed),
        GLOBAL.store_array_str_cache_miss_handle.load(Ordering::Relaxed),
        GLOBAL.store_array_str_cache_miss_epoch.load(Ordering::Relaxed),
        GLOBAL.store_array_str_retarget_hit.load(Ordering::Relaxed),
        GLOBAL.store_array_str_source_store.load(Ordering::Relaxed),
        GLOBAL.store_array_str_non_string_source.load(Ordering::Relaxed),
        GLOBAL.store_array_str_existing_slot.load(Ordering::Relaxed),
        GLOBAL.store_array_str_append_slot.load(Ordering::Relaxed),
        GLOBAL.store_array_str_source_string_box.load(Ordering::Relaxed),
        GLOBAL.store_array_str_source_string_view.load(Ordering::Relaxed),
        GLOBAL.store_array_str_source_missing.load(Ordering::Relaxed),
        GLOBAL.const_suffix_total.load(Ordering::Relaxed),
        GLOBAL.const_suffix_cached_handle_hit.load(Ordering::Relaxed),
        GLOBAL.const_suffix_text_cache_reload.load(Ordering::Relaxed),
        GLOBAL.const_suffix_freeze_fallback.load(Ordering::Relaxed),
        GLOBAL.const_suffix_empty_return.load(Ordering::Relaxed),
        GLOBAL.const_suffix_cached_fast_str_hit.load(Ordering::Relaxed),
        GLOBAL.const_suffix_cached_span_hit.load(Ordering::Relaxed),
        GLOBAL.birth_placement_return_handle.load(Ordering::Relaxed),
        GLOBAL.birth_placement_borrow_view.load(Ordering::Relaxed),
        GLOBAL.birth_placement_freeze_owned.load(Ordering::Relaxed),
        GLOBAL.birth_placement_fresh_handle.load(Ordering::Relaxed),
        GLOBAL.birth_placement_materialize_owned.load(Ordering::Relaxed),
        GLOBAL.birth_placement_store_from_source.load(Ordering::Relaxed),
        GLOBAL.birth_backend_freeze_text_plan_total.load(Ordering::Relaxed),
        GLOBAL.birth_backend_freeze_text_plan_view1.load(Ordering::Relaxed),
        GLOBAL.birth_backend_freeze_text_plan_pieces2.load(Ordering::Relaxed),
        GLOBAL.birth_backend_freeze_text_plan_pieces3.load(Ordering::Relaxed),
        GLOBAL.birth_backend_freeze_text_plan_pieces4.load(Ordering::Relaxed),
        GLOBAL.birth_backend_freeze_text_plan_owned_tmp.load(Ordering::Relaxed),
        GLOBAL.birth_backend_materialize_owned_total.load(Ordering::Relaxed),
        GLOBAL.birth_backend_materialize_owned_bytes.load(Ordering::Relaxed),
        GLOBAL.birth_backend_gc_alloc_called.load(Ordering::Relaxed),
        GLOBAL.birth_backend_gc_alloc_bytes.load(Ordering::Relaxed),
    ]
}

#[cfg(test)]
mod tests {
    use std::sync::{Mutex, OnceLock};

    use super::*;

    fn test_lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
    }

    #[test]
    fn tls_store_array_str_counters_flush_current_thread() {
        let _guard = test_lock().lock().expect("observe test lock");
        std::env::set_var("NYASH_PERF_COUNTERS", "1");

        let before = snapshot();
        store_array_str_enter();
        store_array_str_cache_probe(CacheProbeKind::Hit);
        store_array_str_retarget_hit();
        let after = snapshot();

        assert_eq!(after[0] - before[0], 1);
        assert_eq!(after[1] - before[1], 1);
        assert_eq!(after[4] - before[4], 1);
    }

    #[test]
    fn tls_const_suffix_counters_flush_current_thread() {
        let _guard = test_lock().lock().expect("observe test lock");
        std::env::set_var("NYASH_PERF_COUNTERS", "1");

        let before = snapshot();
        const_suffix_enter();
        const_suffix_cached_handle_hit();
        const_suffix_text_cache_reload();
        const_suffix_freeze_fallback();
        let after = snapshot();

        assert_eq!(after[12] - before[12], 1);
        assert_eq!(after[13] - before[13], 1);
        assert_eq!(after[14] - before[14], 1);
        assert_eq!(after[15] - before[15], 1);
    }

    #[test]
    fn tls_birth_backend_counters_flush_current_thread() {
        let _guard = test_lock().lock().expect("observe test lock");
        std::env::set_var("NYASH_PERF_COUNTERS", "1");

        let before = snapshot();
        birth_placement_freeze_owned();
        birth_placement_fresh_handle();
        birth_backend_freeze_text_plan_pieces2();
        birth_backend_materialize_owned(18);
        let after = snapshot();

        assert_eq!(after[21] - before[21], 1);
        assert_eq!(after[22] - before[22], 1);
        assert_eq!(after[25] - before[25], 1);
        assert_eq!(after[31] - before[31], 1);
        assert_eq!(after[32] - before[32], 18);
        assert_eq!(after[33] - before[33], 1);
        assert_eq!(after[34] - before[34], 18);
    }
}
