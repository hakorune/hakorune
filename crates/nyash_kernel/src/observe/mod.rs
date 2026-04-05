#[cfg(feature = "perf-observe")]
pub(crate) mod contract;

#[cfg(feature = "perf-observe")]
mod backend;
#[cfg(feature = "perf-observe")]
mod config;
#[cfg(feature = "perf-observe")]
mod sink;

#[cfg(feature = "perf-observe")]
mod real {
    pub(crate) use super::backend::CacheProbeKind;

    #[inline(always)]
    pub(crate) fn enabled() -> bool {
        super::config::enabled()
    }

    #[inline(always)]
    pub(crate) fn record_store_array_str_enter() {
        super::backend::store_array_str_enter();
    }

    #[inline(always)]
    pub(crate) fn record_store_array_str_cache_probe(kind: CacheProbeKind) {
        super::backend::store_array_str_cache_probe(kind);
    }

    #[inline(always)]
    pub(crate) fn record_store_array_str_retarget_hit() {
        super::backend::store_array_str_retarget_hit();
    }

    #[inline(always)]
    pub(crate) fn record_store_array_str_source_store() {
        super::backend::store_array_str_source_store();
    }

    #[inline(always)]
    pub(crate) fn record_store_array_str_non_string_source() {
        super::backend::store_array_str_non_string_source();
    }

    #[inline(always)]
    pub(crate) fn record_const_suffix_enter() {
        super::backend::const_suffix_enter();
    }

    #[inline(always)]
    pub(crate) fn record_const_suffix_cached_handle_hit() {
        super::backend::const_suffix_cached_handle_hit();
    }

    #[inline(always)]
    pub(crate) fn record_const_suffix_text_cache_reload() {
        super::backend::const_suffix_text_cache_reload();
    }

    #[inline(always)]
    pub(crate) fn record_const_suffix_freeze_fallback() {
        super::backend::const_suffix_freeze_fallback();
    }

    pub(crate) fn flush() {
        if super::config::enabled() {
            super::sink::emit_summary_to_stderr();
        }
    }
}

#[cfg(not(feature = "perf-observe"))]
mod real {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub(crate) enum CacheProbeKind {
        Hit,
        MissHandle,
        MissDropEpoch,
    }

    #[inline(always)]
    pub(crate) fn enabled() -> bool {
        false
    }

    #[inline(always)]
    pub(crate) fn record_store_array_str_enter() {}

    #[inline(always)]
    pub(crate) fn record_store_array_str_cache_probe(_kind: CacheProbeKind) {}

    #[inline(always)]
    pub(crate) fn record_store_array_str_retarget_hit() {}

    #[inline(always)]
    pub(crate) fn record_store_array_str_source_store() {}

    #[inline(always)]
    pub(crate) fn record_store_array_str_non_string_source() {}

    #[inline(always)]
    pub(crate) fn record_const_suffix_enter() {}

    #[inline(always)]
    pub(crate) fn record_const_suffix_cached_handle_hit() {}

    #[inline(always)]
    pub(crate) fn record_const_suffix_text_cache_reload() {}

    #[inline(always)]
    pub(crate) fn record_const_suffix_freeze_fallback() {}

    #[inline(always)]
    pub(crate) fn flush() {}
}

pub(crate) use real::*;
