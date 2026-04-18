use std::sync::{Mutex, OnceLock};

use super::*;
use crate::observe::contract;
use crate::test_support::with_env_var;

fn test_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

#[test]
fn tls_store_array_str_counters_flush_current_thread() {
    with_env_var("NYASH_PERF_COUNTERS", "1", || {
        let _guard = test_lock().lock().expect("observe test lock");

        let before = snapshot();
        store_array_str_enter();
        store_array_str_cache_probe(CacheProbeKind::Hit);
        store_array_str_retarget_hit();
        let after = snapshot();

        assert_eq!(
            contract::STORE_ARRAY_STR_TOTAL_FIELD.read(&after)
                - contract::STORE_ARRAY_STR_TOTAL_FIELD.read(&before),
            1
        );
        assert_eq!(
            contract::STORE_ARRAY_STR_CACHE_HIT_FIELD.read(&after)
                - contract::STORE_ARRAY_STR_CACHE_HIT_FIELD.read(&before),
            1
        );
        assert_eq!(
            contract::STORE_ARRAY_STR_RETARGET_HIT_FIELD.read(&after)
                - contract::STORE_ARRAY_STR_RETARGET_HIT_FIELD.read(&before),
            1
        );
    });
}

#[test]
fn tls_const_suffix_counters_flush_current_thread() {
    with_env_var("NYASH_PERF_COUNTERS", "1", || {
        let _guard = test_lock().lock().expect("observe test lock");

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
    });
}

#[test]
fn tls_birth_backend_counters_flush_current_thread() {
    with_env_var("NYASH_PERF_COUNTERS", "1", || {
        let _guard = test_lock().lock().expect("observe test lock");

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
        birth_backend_carrier_kind_stable_box();
        birth_backend_carrier_kind_source_keep();
        birth_backend_carrier_kind_owned_bytes();
        birth_backend_carrier_kind_handle();
        birth_backend_publish_reason_external_boundary();
        birth_backend_publish_reason_need_stable_object();
        birth_backend_publish_reason_generic_fallback();
        birth_backend_publish_reason_explicit_api();
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
        assert_eq!(after[113] - before[113], 1);
        assert_eq!(after[114] - before[114], 1);
        assert_eq!(after[115] - before[115], 1);
        assert_eq!(after[116] - before[116], 1);
        assert_eq!(after[117] - before[117], 1);
        assert_eq!(after[118] - before[118], 1);
        assert_eq!(after[119] - before[119], 1);
        assert_eq!(after[120] - before[120], 1);
    });
}

#[test]
fn tls_phase137x_evidence_counters_flush_current_thread() {
    with_env_var("NYASH_PERF_COUNTERS", "1", || {
        let _guard = test_lock().lock().expect("observe test lock");

        let before = snapshot();
        store_array_str_lookup_registry_slot_read();
        store_array_str_lookup_caller_latest_fresh_tag();
        birth_backend_site_string_concat_hh_materialize_owned(18);
        birth_backend_site_string_concat_hh_objectize_box();
        birth_backend_site_string_concat_hh_publish_handle();
        birth_backend_site_string_substring_concat_hhii_materialize_owned(7);
        birth_backend_site_string_substring_concat_hhii_objectize_box();
        birth_backend_site_string_substring_concat_hhii_publish_handle();
        let after = snapshot();

        assert_eq!(
            contract::STORE_ARRAY_STR_LOOKUP_REGISTRY_SLOT_READ_FIELD.read(&after)
                - contract::STORE_ARRAY_STR_LOOKUP_REGISTRY_SLOT_READ_FIELD.read(&before),
            1
        );
        assert_eq!(
            contract::STORE_ARRAY_STR_LOOKUP_CALLER_LATEST_FRESH_TAG_FIELD.read(&after)
                - contract::STORE_ARRAY_STR_LOOKUP_CALLER_LATEST_FRESH_TAG_FIELD.read(&before),
            1
        );
        assert_eq!(after[123] - before[123], 1);
        assert_eq!(after[124] - before[124], 18);
        assert_eq!(after[125] - before[125], 1);
        assert_eq!(after[126] - before[126], 1);
        assert_eq!(after[127] - before[127], 1);
        assert_eq!(after[128] - before[128], 7);
        assert_eq!(after[129] - before[129], 1);
        assert_eq!(after[130] - before[130], 1);
    });
}

#[test]
fn tls_phase137x_whole_site_counters_flush_current_thread() {
    with_env_var("NYASH_PERF_COUNTERS", "1", || {
        let _guard = test_lock().lock().expect("observe test lock");

        let before = snapshot();
        birth_backend_site_const_suffix_materialize_owned(11);
        birth_backend_site_const_suffix_objectize_box();
        birth_backend_site_const_suffix_publish_handle();
        birth_backend_site_freeze_text_plan_pieces3_materialize_owned(19);
        birth_backend_site_freeze_text_plan_pieces3_objectize_box();
        birth_backend_site_freeze_text_plan_pieces3_publish_handle();
        let after = snapshot();

        assert_eq!(after[131] - before[131], 1);
        assert_eq!(after[132] - before[132], 11);
        assert_eq!(after[133] - before[133], 1);
        assert_eq!(after[134] - before[134], 1);
        assert_eq!(after[135] - before[135], 1);
        assert_eq!(after[136] - before[136], 19);
        assert_eq!(after[137] - before[137], 1);
        assert_eq!(after[138] - before[138], 1);
    });
}

#[test]
fn tls_slot_publish_boundary_counters_flush_current_thread() {
    with_env_var("NYASH_PERF_COUNTERS", "1", || {
        let _guard = test_lock().lock().expect("observe test lock");

        let before = snapshot();
        birth_backend_publish_boundary_slot_publish_handle();
        birth_backend_publish_boundary_slot_objectize_stable_box();
        birth_backend_publish_boundary_slot_empty();
        birth_backend_publish_boundary_slot_already_published();
        borrowed_alias_encode_live_source_hit();
        borrowed_alias_encode_live_source_hit_array_get_index();
        borrowed_alias_encode_live_source_hit_map_runtime_data_get_any();
        borrowed_alias_encode_cached_handle_hit();
        borrowed_alias_encode_cached_handle_hit_array_get_index();
        borrowed_alias_encode_cached_handle_hit_map_runtime_data_get_any();
        let after = snapshot();

        assert_eq!(
            contract::BIRTH_BACKEND_PUBLISH_BOUNDARY_SLOT_PUBLISH_HANDLE_TOTAL_FIELD.read(&after)
                - contract::BIRTH_BACKEND_PUBLISH_BOUNDARY_SLOT_PUBLISH_HANDLE_TOTAL_FIELD
                    .read(&before),
            1
        );
        assert_eq!(
            contract::BIRTH_BACKEND_PUBLISH_BOUNDARY_SLOT_OBJECTIZE_STABLE_BOX_TOTAL_FIELD
                .read(&after)
                - contract::BIRTH_BACKEND_PUBLISH_BOUNDARY_SLOT_OBJECTIZE_STABLE_BOX_TOTAL_FIELD
                    .read(&before),
            1
        );
        assert_eq!(
            contract::BIRTH_BACKEND_PUBLISH_BOUNDARY_SLOT_EMPTY_FIELD.read(&after)
                - contract::BIRTH_BACKEND_PUBLISH_BOUNDARY_SLOT_EMPTY_FIELD.read(&before),
            1
        );
        assert_eq!(
            contract::BIRTH_BACKEND_PUBLISH_BOUNDARY_SLOT_ALREADY_PUBLISHED_FIELD.read(&after)
                - contract::BIRTH_BACKEND_PUBLISH_BOUNDARY_SLOT_ALREADY_PUBLISHED_FIELD
                    .read(&before),
            1
        );
        assert_eq!(
            contract::BORROWED_ALIAS_ENCODE_LIVE_SOURCE_HIT_FIELD.read(&after)
                - contract::BORROWED_ALIAS_ENCODE_LIVE_SOURCE_HIT_FIELD.read(&before),
            1
        );
        assert_eq!(
            contract::BORROWED_ALIAS_ENCODE_LIVE_SOURCE_HIT_ARRAY_GET_INDEX_FIELD.read(&after)
                - contract::BORROWED_ALIAS_ENCODE_LIVE_SOURCE_HIT_ARRAY_GET_INDEX_FIELD
                    .read(&before),
            1
        );
        assert_eq!(
            contract::BORROWED_ALIAS_ENCODE_LIVE_SOURCE_HIT_MAP_RUNTIME_DATA_GET_ANY_FIELD
                .read(&after)
                - contract::BORROWED_ALIAS_ENCODE_LIVE_SOURCE_HIT_MAP_RUNTIME_DATA_GET_ANY_FIELD
                    .read(&before),
            1
        );
        assert_eq!(
            contract::BORROWED_ALIAS_ENCODE_CACHED_HANDLE_HIT_FIELD.read(&after)
                - contract::BORROWED_ALIAS_ENCODE_CACHED_HANDLE_HIT_FIELD.read(&before),
            1
        );
        assert_eq!(
            contract::BORROWED_ALIAS_ENCODE_CACHED_HANDLE_HIT_ARRAY_GET_INDEX_FIELD.read(&after)
                - contract::BORROWED_ALIAS_ENCODE_CACHED_HANDLE_HIT_ARRAY_GET_INDEX_FIELD
                    .read(&before),
            1
        );
        assert_eq!(
            contract::BORROWED_ALIAS_ENCODE_CACHED_HANDLE_HIT_MAP_RUNTIME_DATA_GET_ANY_FIELD
                .read(&after)
                - contract::BORROWED_ALIAS_ENCODE_CACHED_HANDLE_HIT_MAP_RUNTIME_DATA_GET_ANY_FIELD
                    .read(&before),
            1
        );
    });
}

#[test]
fn tls_string_route_counters_flush_current_thread() {
    with_env_var("NYASH_PERF_COUNTERS", "1", || {
        let _guard = test_lock().lock().expect("observe test lock");

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
    });
}

#[test]
fn tls_substring_route_counters_flush_current_thread() {
    with_env_var("NYASH_PERF_COUNTERS", "1", || {
        let _guard = test_lock().lock().expect("observe test lock");

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
    });
}

#[test]
fn tls_piecewise_subrange_counters_flush_current_thread() {
    with_env_var("NYASH_PERF_COUNTERS", "1", || {
        let _guard = test_lock().lock().expect("observe test lock");

        let before = snapshot();
        piecewise_subrange_enter();
        piecewise_subrange_single_session_hit();
        piecewise_subrange_prefix_middle();
        piecewise_subrange_enter();
        piecewise_subrange_fallback_insert();
        piecewise_subrange_enter();
        piecewise_subrange_empty_return();
        let after = snapshot();

        assert_eq!(after[102] - before[102], 3);
        assert_eq!(after[103] - before[103], 1);
        assert_eq!(after[104] - before[104], 1);
        assert_eq!(after[105] - before[105], 1);
        assert_eq!(after[109] - before[109], 1);
    });
}
