use super::super::backend;
use super::super::contract;

pub(crate) fn emit_summary_to_stderr() {
    let snapshot = backend::snapshot();
    eprintln!(
        "[perf/counter][{}] total={} {}={} {}={} {}={} {}={} {}={} {}={}",
        contract::STORE_ARRAY_STR,
        snapshot[0],
        contract::STORE_ARRAY_STR_CACHE_HIT,
        snapshot[1],
        contract::STORE_ARRAY_STR_CACHE_MISS_HANDLE,
        snapshot[2],
        contract::STORE_ARRAY_STR_CACHE_MISS_EPOCH,
        snapshot[3],
        contract::STORE_ARRAY_STR_RETARGET_HIT,
        snapshot[4],
        contract::STORE_ARRAY_STR_SOURCE_STORE,
        snapshot[5],
        contract::STORE_ARRAY_STR_NON_STRING_SOURCE,
        snapshot[6],
    );
    eprintln!(
        "[perf/counter][{}] total={} {}={} {}={} {}={}",
        contract::CONST_SUFFIX,
        snapshot[7],
        contract::CONST_SUFFIX_CACHED_HANDLE_HIT,
        snapshot[8],
        contract::CONST_SUFFIX_TEXT_CACHE_RELOAD,
        snapshot[9],
        contract::CONST_SUFFIX_FREEZE_FALLBACK,
        snapshot[10],
    );
}
