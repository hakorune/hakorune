use super::super::backend;
use super::super::contract;

pub(crate) fn emit_summary_to_stderr() {
    let snapshot = backend::snapshot();
    eprintln!(
        "[perf/counter][{}] total={} {}={} {}={} {}={} {}={} {}={} {}={} {}={} {}={} {}={} {}={} {}={}",
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
        contract::STORE_ARRAY_STR_EXISTING_SLOT,
        snapshot[7],
        contract::STORE_ARRAY_STR_APPEND_SLOT,
        snapshot[8],
        contract::STORE_ARRAY_STR_SOURCE_STRING_BOX,
        snapshot[9],
        contract::STORE_ARRAY_STR_SOURCE_STRING_VIEW,
        snapshot[10],
        contract::STORE_ARRAY_STR_SOURCE_MISSING,
        snapshot[11],
    );
    eprintln!(
        "[perf/counter][{}] total={} {}={} {}={} {}={} {}={} {}={} {}={}",
        contract::CONST_SUFFIX,
        snapshot[12],
        contract::CONST_SUFFIX_CACHED_HANDLE_HIT,
        snapshot[13],
        contract::CONST_SUFFIX_TEXT_CACHE_RELOAD,
        snapshot[14],
        contract::CONST_SUFFIX_FREEZE_FALLBACK,
        snapshot[15],
        contract::CONST_SUFFIX_EMPTY_RETURN,
        snapshot[16],
        contract::CONST_SUFFIX_CACHED_FAST_STR_HIT,
        snapshot[17],
        contract::CONST_SUFFIX_CACHED_SPAN_HIT,
        snapshot[18],
    );
}
