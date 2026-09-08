#[cfg(any(feature = "perf-observe", feature = "perf-trace"))]
pub(crate) mod contract;

#[cfg(feature = "perf-observe")]
mod backend;
#[cfg(feature = "perf-observe")]
mod config;
#[cfg(feature = "perf-observe")]
mod sink;
#[cfg(feature = "perf-trace")]
mod trace;

#[cfg(feature = "perf-observe")]
#[path = "real_perf_observe.rs"]
mod real;
#[cfg(not(feature = "perf-observe"))]
#[path = "real_no_perf_observe.rs"]
mod real;

pub(crate) use real::*;

#[cfg(all(test, feature = "perf-observe"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct BorrowedAliasEncodeSnapshot {
    pub live_source_hit: u64,
    pub live_source_hit_array_get_index: u64,
    pub live_source_hit_map_runtime_data_get_any: u64,
    pub cached_handle_hit: u64,
    pub cached_handle_hit_array_get_index: u64,
    pub cached_handle_hit_map_runtime_data_get_any: u64,
    pub fallback_to_handle_arc: u64,
    pub fallback_to_handle_arc_array_get_index: u64,
    pub fallback_to_handle_arc_map_runtime_data_get_any: u64,
}

#[cfg(all(test, feature = "perf-observe"))]
pub(crate) fn borrowed_alias_encode_snapshot_for_tests() -> BorrowedAliasEncodeSnapshot {
    let snapshot = backend::snapshot();
    let [live_source_hit, live_source_hit_array_get_index, live_source_hit_map_runtime_data_get_any, cached_handle_hit, cached_handle_hit_array_get_index, cached_handle_hit_map_runtime_data_get_any, fallback_to_handle_arc, fallback_to_handle_arc_array_get_index, fallback_to_handle_arc_map_runtime_data_get_any] =
        contract::BORROWED_ALIAS_ENCODE_SNAPSHOT_FIELDS.map(|field| field.read(&snapshot));

    // Keep plugin tests off the raw TLS snapshot layout. The field set and
    // order are owned by observe::contract.
    BorrowedAliasEncodeSnapshot {
        live_source_hit,
        live_source_hit_array_get_index,
        live_source_hit_map_runtime_data_get_any,
        cached_handle_hit,
        cached_handle_hit_array_get_index,
        cached_handle_hit_map_runtime_data_get_any,
        fallback_to_handle_arc,
        fallback_to_handle_arc_array_get_index,
        fallback_to_handle_arc_map_runtime_data_get_any,
    }
}

#[cfg(feature = "perf-trace")]
pub(crate) fn flush_trace() {
    trace::flush();
}

#[cfg(not(feature = "perf-trace"))]
pub(crate) fn flush_trace() {}
