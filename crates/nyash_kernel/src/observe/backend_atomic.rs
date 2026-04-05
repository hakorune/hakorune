use std::sync::atomic::{AtomicU64, Ordering};

use super::config;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CacheProbeKind {
    Hit,
    MissHandle,
    MissDropEpoch,
}

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
fn bump(counter: &AtomicU64) {
    if config::enabled() {
        counter.fetch_add(1, Ordering::Relaxed);
    }
}

#[inline(always)]
pub(crate) fn store_array_str_enter() {
    bump(&STORE_ARRAY_STR_TOTAL);
}

#[inline(always)]
pub(crate) fn store_array_str_cache_probe(kind: CacheProbeKind) {
    if !config::enabled() {
        return;
    }
    match kind {
        CacheProbeKind::Hit => bump(&STORE_ARRAY_STR_CACHE_HIT),
        CacheProbeKind::MissHandle => bump(&STORE_ARRAY_STR_CACHE_MISS_HANDLE),
        CacheProbeKind::MissDropEpoch => bump(&STORE_ARRAY_STR_CACHE_MISS_EPOCH),
    }
}

#[inline(always)]
pub(crate) fn store_array_str_retarget_hit() {
    bump(&STORE_ARRAY_STR_RETARGET_HIT);
}

#[inline(always)]
pub(crate) fn store_array_str_source_store() {
    bump(&STORE_ARRAY_STR_SOURCE_STORE);
}

#[inline(always)]
pub(crate) fn store_array_str_non_string_source() {
    bump(&STORE_ARRAY_STR_NON_STRING_SOURCE);
}

#[inline(always)]
pub(crate) fn const_suffix_enter() {
    bump(&CONST_SUFFIX_TOTAL);
}

#[inline(always)]
pub(crate) fn const_suffix_cached_handle_hit() {
    bump(&CONST_SUFFIX_CACHED_HANDLE_HIT);
}

#[inline(always)]
pub(crate) fn const_suffix_text_cache_reload() {
    bump(&CONST_SUFFIX_TEXT_CACHE_RELOAD);
}

#[inline(always)]
pub(crate) fn const_suffix_freeze_fallback() {
    bump(&CONST_SUFFIX_FREEZE_FALLBACK);
}

pub(crate) fn snapshot() -> [u64; 11] {
    [
        STORE_ARRAY_STR_TOTAL.load(Ordering::Relaxed),
        STORE_ARRAY_STR_CACHE_HIT.load(Ordering::Relaxed),
        STORE_ARRAY_STR_CACHE_MISS_HANDLE.load(Ordering::Relaxed),
        STORE_ARRAY_STR_CACHE_MISS_EPOCH.load(Ordering::Relaxed),
        STORE_ARRAY_STR_RETARGET_HIT.load(Ordering::Relaxed),
        STORE_ARRAY_STR_SOURCE_STORE.load(Ordering::Relaxed),
        STORE_ARRAY_STR_NON_STRING_SOURCE.load(Ordering::Relaxed),
        CONST_SUFFIX_TOTAL.load(Ordering::Relaxed),
        CONST_SUFFIX_CACHED_HANDLE_HIT.load(Ordering::Relaxed),
        CONST_SUFFIX_TEXT_CACHE_RELOAD.load(Ordering::Relaxed),
        CONST_SUFFIX_FREEZE_FALLBACK.load(Ordering::Relaxed),
    ]
}
