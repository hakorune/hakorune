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

    assert_eq!(after[14] - before[14], 1);
    assert_eq!(after[15] - before[15], 1);
    assert_eq!(after[16] - before[16], 1);
    assert_eq!(after[17] - before[17], 1);
}

#[test]
fn tls_birth_backend_counters_flush_current_thread() {
    let _guard = test_lock().lock().expect("observe test lock");
    std::env::set_var("NYASH_PERF_COUNTERS", "1");

    let before = snapshot();
    birth_placement_freeze_owned();
    birth_placement_fresh_handle();
    birth_backend_freeze_text_plan_pieces2();
    birth_backend_string_box_new(18);
    birth_backend_string_box_ctor(18);
    birth_backend_arc_wrap();
    birth_backend_objectize_stable_box_now(18);
    birth_backend_handle_issue();
    birth_backend_issue_fresh_handle();
    birth_backend_materialize_owned(18);
    birth_backend_gc_alloc(18);
    birth_backend_gc_alloc_skipped();
    let after = snapshot();

    assert_eq!(after[23] - before[23], 1);
    assert_eq!(after[24] - before[24], 1);
    assert_eq!(after[29] - before[29], 1);
    assert_eq!(after[33] - before[33], 1);
    assert_eq!(after[34] - before[34], 18);
    assert_eq!(after[35] - before[35], 1);
    assert_eq!(after[36] - before[36], 18);
    assert_eq!(after[37] - before[37], 1);
    assert_eq!(after[38] - before[38], 1);
    assert_eq!(after[39] - before[39], 18);
    assert_eq!(after[40] - before[40], 1);
    assert_eq!(after[41] - before[41], 1);
    assert_eq!(after[42] - before[42], 1);
    assert_eq!(after[43] - before[43], 18);
    assert_eq!(after[44] - before[44], 1);
    assert_eq!(after[45] - before[45], 18);
    assert_eq!(after[46] - before[46], 1);
}

#[test]
fn tls_string_route_counters_flush_current_thread() {
    let _guard = test_lock().lock().expect("observe test lock");
    std::env::set_var("NYASH_PERF_COUNTERS", "1");

    let before = snapshot();
    str_concat2_route_enter();
    str_concat2_route_fast_str_owned();
    str_len_route_enter();
    str_len_route_fast_str_hit();
    mark_latest_fresh_handle(77);
    assert!(matches_latest_fresh_handle(77));
    str_len_route_latest_fresh_handle_fast_str_hit();
    str_len_route_latest_fresh_handle_fallback_hit();
    let after = snapshot();

    assert_eq!(after[47] - before[47], 1);
    assert_eq!(after[49] - before[49], 1);
    assert_eq!(after[54] - before[54], 2);
    assert_eq!(after[56] - before[56], 1);
    assert_eq!(after[59] - before[59], 1);
    assert_eq!(after[60] - before[60], 1);
}

#[test]
fn tls_substring_route_counters_flush_current_thread() {
    let _guard = test_lock().lock().expect("observe test lock");
    std::env::set_var("NYASH_PERF_COUNTERS", "1");

    let before = snapshot();
    str_substring_route_enter();
    str_substring_route_view_arc_cache_miss();
    str_substring_route_fast_cache_hit();
    str_substring_route_dispatch_hit();
    str_substring_route_slow_plan();
    str_substring_route_slow_plan_return_handle();
    str_substring_route_slow_plan_return_empty();
    str_substring_route_slow_plan_freeze_span();
    str_substring_route_slow_plan_view_span();
    let after = snapshot();

    assert_eq!(after[61] - before[61], 1);
    assert_eq!(after[64] - before[64], 1);
    assert_eq!(after[65] - before[65], 1);
    assert_eq!(after[66] - before[66], 1);
    assert_eq!(after[67] - before[67], 1);
    assert_eq!(after[68] - before[68], 1);
    assert_eq!(after[69] - before[69], 1);
    assert_eq!(after[70] - before[70], 1);
    assert_eq!(after[71] - before[71], 1);
}
