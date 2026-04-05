use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::OnceLock;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CacheProbeKind {
    Hit,
    MissHandle,
    MissDropEpoch,
}

static PERF_COUNTERS_ENABLED: OnceLock<bool> = OnceLock::new();

static STORE_ARRAY_STR_TOTAL: AtomicU64 = AtomicU64::new(0);
static STORE_ARRAY_STR_CACHE_HIT: AtomicU64 = AtomicU64::new(0);
static STORE_ARRAY_STR_CACHE_MISS_HANDLE: AtomicU64 = AtomicU64::new(0);
static STORE_ARRAY_STR_CACHE_MISS_EPOCH: AtomicU64 = AtomicU64::new(0);
static STORE_ARRAY_STR_RETARGET_HIT: AtomicU64 = AtomicU64::new(0);
static STORE_ARRAY_STR_SOURCE_STORE: AtomicU64 = AtomicU64::new(0);
static STORE_ARRAY_STR_NON_STRING_SOURCE: AtomicU64 = AtomicU64::new(0);

static CONST_SUFFIX_TOTAL: AtomicU64 = AtomicU64::new(0);
static CONST_SUFFIX_CACHED_HANDLE_HIT: AtomicU64 = AtomicU64::new(0);
static CONST_SUFFIX_TEXT_CACHE_RELOAD: AtomicU64 = AtomicU64::new(0);
static CONST_SUFFIX_FREEZE_FALLBACK: AtomicU64 = AtomicU64::new(0);

#[inline(always)]
fn parse_enabled(value: &str) -> bool {
    matches!(value, "1" | "on" | "true" | "yes")
}

pub(crate) fn enabled() -> bool {
    #[cfg(test)]
    {
        std::env::var("NYASH_PERF_COUNTERS")
            .ok()
            .as_deref()
            .is_some_and(parse_enabled)
    }
    #[cfg(not(test))]
    {
        *PERF_COUNTERS_ENABLED.get_or_init(|| {
            std::env::var("NYASH_PERF_COUNTERS")
                .ok()
                .as_deref()
                .is_some_and(parse_enabled)
        })
    }
}

#[inline(always)]
fn bump(counter: &AtomicU64) {
    if enabled() {
        counter.fetch_add(1, Ordering::Relaxed);
    }
}

#[inline(always)]
pub(crate) fn record_store_array_str_enter() {
    bump(&STORE_ARRAY_STR_TOTAL);
}

#[inline(always)]
pub(crate) fn record_store_array_str_cache_probe(kind: CacheProbeKind) {
    if !enabled() {
        return;
    }
    match kind {
        CacheProbeKind::Hit => bump(&STORE_ARRAY_STR_CACHE_HIT),
        CacheProbeKind::MissHandle => bump(&STORE_ARRAY_STR_CACHE_MISS_HANDLE),
        CacheProbeKind::MissDropEpoch => bump(&STORE_ARRAY_STR_CACHE_MISS_EPOCH),
    }
}

#[inline(always)]
pub(crate) fn record_store_array_str_retarget_hit() {
    bump(&STORE_ARRAY_STR_RETARGET_HIT);
}

#[inline(always)]
pub(crate) fn record_store_array_str_source_store() {
    bump(&STORE_ARRAY_STR_SOURCE_STORE);
}

#[inline(always)]
pub(crate) fn record_store_array_str_non_string_source() {
    bump(&STORE_ARRAY_STR_NON_STRING_SOURCE);
}

#[inline(always)]
pub(crate) fn record_const_suffix_enter() {
    bump(&CONST_SUFFIX_TOTAL);
}

#[inline(always)]
pub(crate) fn record_const_suffix_cached_handle_hit() {
    bump(&CONST_SUFFIX_CACHED_HANDLE_HIT);
}

#[inline(always)]
pub(crate) fn record_const_suffix_text_cache_reload() {
    bump(&CONST_SUFFIX_TEXT_CACHE_RELOAD);
}

#[inline(always)]
pub(crate) fn record_const_suffix_freeze_fallback() {
    bump(&CONST_SUFFIX_FREEZE_FALLBACK);
}

pub(crate) fn emit_summary_to_stderr() {
    if !enabled() {
        return;
    }
    eprintln!(
        "[perf/counter][store.array.str] total={} cache_hit={} cache_miss_handle={} cache_miss_epoch={} retarget_hit={} source_store={} non_string_source={}",
        STORE_ARRAY_STR_TOTAL.load(Ordering::Relaxed),
        STORE_ARRAY_STR_CACHE_HIT.load(Ordering::Relaxed),
        STORE_ARRAY_STR_CACHE_MISS_HANDLE.load(Ordering::Relaxed),
        STORE_ARRAY_STR_CACHE_MISS_EPOCH.load(Ordering::Relaxed),
        STORE_ARRAY_STR_RETARGET_HIT.load(Ordering::Relaxed),
        STORE_ARRAY_STR_SOURCE_STORE.load(Ordering::Relaxed),
        STORE_ARRAY_STR_NON_STRING_SOURCE.load(Ordering::Relaxed),
    );
    eprintln!(
        "[perf/counter][const_suffix] total={} cached_handle_hit={} text_cache_reload={} freeze_fallback={}",
        CONST_SUFFIX_TOTAL.load(Ordering::Relaxed),
        CONST_SUFFIX_CACHED_HANDLE_HIT.load(Ordering::Relaxed),
        CONST_SUFFIX_TEXT_CACHE_RELOAD.load(Ordering::Relaxed),
        CONST_SUFFIX_FREEZE_FALLBACK.load(Ordering::Relaxed),
    );
}
