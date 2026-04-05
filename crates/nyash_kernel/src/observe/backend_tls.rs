use std::cell::Cell;
use std::sync::atomic::{AtomicU64, Ordering};

use super::config;

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
    const_suffix_total: AtomicU64,
    const_suffix_cached_handle_hit: AtomicU64,
    const_suffix_text_cache_reload: AtomicU64,
    const_suffix_freeze_fallback: AtomicU64,
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
            const_suffix_total: AtomicU64::new(0),
            const_suffix_cached_handle_hit: AtomicU64::new(0),
            const_suffix_text_cache_reload: AtomicU64::new(0),
            const_suffix_freeze_fallback: AtomicU64::new(0),
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
    const_suffix_total: Cell<u64>,
    const_suffix_cached_handle_hit: Cell<u64>,
    const_suffix_text_cache_reload: Cell<u64>,
    const_suffix_freeze_fallback: Cell<u64>,
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
            const_suffix_total: Cell::new(0),
            const_suffix_cached_handle_hit: Cell::new(0),
            const_suffix_text_cache_reload: Cell::new(0),
            const_suffix_freeze_fallback: Cell::new(0),
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

fn flush_current_thread() {
    TLS_COUNTERS.with(ThreadCounters::flush_into_global);
}

pub(crate) fn snapshot() -> [u64; 11] {
    flush_current_thread();
    [
        GLOBAL.store_array_str_total.load(Ordering::Relaxed),
        GLOBAL.store_array_str_cache_hit.load(Ordering::Relaxed),
        GLOBAL.store_array_str_cache_miss_handle.load(Ordering::Relaxed),
        GLOBAL.store_array_str_cache_miss_epoch.load(Ordering::Relaxed),
        GLOBAL.store_array_str_retarget_hit.load(Ordering::Relaxed),
        GLOBAL.store_array_str_source_store.load(Ordering::Relaxed),
        GLOBAL.store_array_str_non_string_source.load(Ordering::Relaxed),
        GLOBAL.const_suffix_total.load(Ordering::Relaxed),
        GLOBAL.const_suffix_cached_handle_hit.load(Ordering::Relaxed),
        GLOBAL.const_suffix_text_cache_reload.load(Ordering::Relaxed),
        GLOBAL.const_suffix_freeze_fallback.load(Ordering::Relaxed),
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

        assert_eq!(after[7] - before[7], 1);
        assert_eq!(after[8] - before[8], 1);
        assert_eq!(after[9] - before[9], 1);
        assert_eq!(after[10] - before[10], 1);
    }
}
