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

        assert_eq!(
            contract::CONST_SUFFIX_TOTAL_FIELD.read(&after)
                - contract::CONST_SUFFIX_TOTAL_FIELD.read(&before),
            1
        );
        assert_eq!(
            contract::CONST_SUFFIX_CACHED_HANDLE_HIT_FIELD.read(&after)
                - contract::CONST_SUFFIX_CACHED_HANDLE_HIT_FIELD.read(&before),
            1
        );
        assert_eq!(
            contract::CONST_SUFFIX_TEXT_CACHE_RELOAD_FIELD.read(&after)
                - contract::CONST_SUFFIX_TEXT_CACHE_RELOAD_FIELD.read(&before),
            1
        );
        assert_eq!(
            contract::CONST_SUFFIX_FREEZE_FALLBACK_FIELD.read(&after)
                - contract::CONST_SUFFIX_FREEZE_FALLBACK_FIELD.read(&before),
            1
        );
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

        assert_eq!(
            contract::BIRTH_PLACEMENT_FREEZE_OWNED_FIELD.read(&after)
                - contract::BIRTH_PLACEMENT_FREEZE_OWNED_FIELD.read(&before),
            1
        );
        assert_eq!(
            contract::BIRTH_PLACEMENT_FRESH_HANDLE_FIELD.read(&after)
                - contract::BIRTH_PLACEMENT_FRESH_HANDLE_FIELD.read(&before),
            1
        );
        assert_eq!(
            contract::BIRTH_BACKEND_FREEZE_TEXT_PLAN_PIECES2_FIELD.read(&after)
                - contract::BIRTH_BACKEND_FREEZE_TEXT_PLAN_PIECES2_FIELD.read(&before),
            1
        );
        assert_eq!(
            contract::BIRTH_BACKEND_STRING_BOX_NEW_TOTAL_FIELD.read(&after)
                - contract::BIRTH_BACKEND_STRING_BOX_NEW_TOTAL_FIELD.read(&before),
            1
        );
        assert_eq!(
            contract::BIRTH_BACKEND_STRING_BOX_NEW_BYTES_FIELD.read(&after)
                - contract::BIRTH_BACKEND_STRING_BOX_NEW_BYTES_FIELD.read(&before),
            18
        );
        assert_eq!(
            contract::BIRTH_BACKEND_STRING_BOX_CTOR_TOTAL_FIELD.read(&after)
                - contract::BIRTH_BACKEND_STRING_BOX_CTOR_TOTAL_FIELD.read(&before),
            1
        );
        assert_eq!(
            contract::BIRTH_BACKEND_STRING_BOX_CTOR_BYTES_FIELD.read(&after)
                - contract::BIRTH_BACKEND_STRING_BOX_CTOR_BYTES_FIELD.read(&before),
            18
        );
        assert_eq!(
            contract::BIRTH_BACKEND_ARC_WRAP_TOTAL_FIELD.read(&after)
                - contract::BIRTH_BACKEND_ARC_WRAP_TOTAL_FIELD.read(&before),
            1
        );
        assert_eq!(
            contract::BIRTH_BACKEND_OBJECTIZE_STABLE_BOX_NOW_TOTAL_FIELD.read(&after)
                - contract::BIRTH_BACKEND_OBJECTIZE_STABLE_BOX_NOW_TOTAL_FIELD.read(&before),
            1
        );
        assert_eq!(
            contract::BIRTH_BACKEND_OBJECTIZE_STABLE_BOX_NOW_BYTES_FIELD.read(&after)
                - contract::BIRTH_BACKEND_OBJECTIZE_STABLE_BOX_NOW_BYTES_FIELD.read(&before),
            18
        );
        assert_eq!(
            contract::BIRTH_BACKEND_HANDLE_ISSUE_TOTAL_FIELD.read(&after)
                - contract::BIRTH_BACKEND_HANDLE_ISSUE_TOTAL_FIELD.read(&before),
            1
        );
        assert_eq!(
            contract::BIRTH_BACKEND_ISSUE_FRESH_HANDLE_TOTAL_FIELD.read(&after)
                - contract::BIRTH_BACKEND_ISSUE_FRESH_HANDLE_TOTAL_FIELD.read(&before),
            1
        );
        assert_eq!(
            contract::BIRTH_BACKEND_MATERIALIZE_OWNED_TOTAL_FIELD.read(&after)
                - contract::BIRTH_BACKEND_MATERIALIZE_OWNED_TOTAL_FIELD.read(&before),
            1
        );
        assert_eq!(
            contract::BIRTH_BACKEND_MATERIALIZE_OWNED_BYTES_FIELD.read(&after)
                - contract::BIRTH_BACKEND_MATERIALIZE_OWNED_BYTES_FIELD.read(&before),
            18
        );
        assert_eq!(
            contract::BIRTH_BACKEND_GC_ALLOC_CALLED_FIELD.read(&after)
                - contract::BIRTH_BACKEND_GC_ALLOC_CALLED_FIELD.read(&before),
            1
        );
        assert_eq!(
            contract::BIRTH_BACKEND_GC_ALLOC_BYTES_FIELD.read(&after)
                - contract::BIRTH_BACKEND_GC_ALLOC_BYTES_FIELD.read(&before),
            18
        );
        assert_eq!(
            contract::BIRTH_BACKEND_GC_ALLOC_SKIPPED_FIELD.read(&after)
                - contract::BIRTH_BACKEND_GC_ALLOC_SKIPPED_FIELD.read(&before),
            1
        );
        assert_eq!(
            contract::BIRTH_BACKEND_CARRIER_KIND_STABLE_BOX_FIELD.read(&after)
                - contract::BIRTH_BACKEND_CARRIER_KIND_STABLE_BOX_FIELD.read(&before),
            1
        );
        assert_eq!(
            contract::BIRTH_BACKEND_CARRIER_KIND_SOURCE_KEEP_FIELD.read(&after)
                - contract::BIRTH_BACKEND_CARRIER_KIND_SOURCE_KEEP_FIELD.read(&before),
            1
        );
        assert_eq!(
            contract::BIRTH_BACKEND_CARRIER_KIND_OWNED_BYTES_FIELD.read(&after)
                - contract::BIRTH_BACKEND_CARRIER_KIND_OWNED_BYTES_FIELD.read(&before),
            1
        );
        assert_eq!(
            contract::BIRTH_BACKEND_CARRIER_KIND_HANDLE_FIELD.read(&after)
                - contract::BIRTH_BACKEND_CARRIER_KIND_HANDLE_FIELD.read(&before),
            1
        );
        assert_eq!(
            contract::BIRTH_BACKEND_PUBLISH_REASON_EXTERNAL_BOUNDARY_FIELD.read(&after)
                - contract::BIRTH_BACKEND_PUBLISH_REASON_EXTERNAL_BOUNDARY_FIELD.read(&before),
            1
        );
        assert_eq!(
            contract::BIRTH_BACKEND_PUBLISH_REASON_NEED_STABLE_OBJECT_FIELD.read(&after)
                - contract::BIRTH_BACKEND_PUBLISH_REASON_NEED_STABLE_OBJECT_FIELD.read(&before),
            1
        );
        assert_eq!(
            contract::BIRTH_BACKEND_PUBLISH_REASON_GENERIC_FALLBACK_FIELD.read(&after)
                - contract::BIRTH_BACKEND_PUBLISH_REASON_GENERIC_FALLBACK_FIELD.read(&before),
            1
        );
        assert_eq!(
            contract::BIRTH_BACKEND_PUBLISH_REASON_EXPLICIT_API_FIELD.read(&after)
                - contract::BIRTH_BACKEND_PUBLISH_REASON_EXPLICIT_API_FIELD.read(&before),
            1
        );
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
        assert_eq!(
            contract::BIRTH_BACKEND_SITE_STRING_CONCAT_HH_MATERIALIZE_OWNED_TOTAL_FIELD
                .read(&after)
                - contract::BIRTH_BACKEND_SITE_STRING_CONCAT_HH_MATERIALIZE_OWNED_TOTAL_FIELD
                    .read(&before),
            1
        );
        assert_eq!(
            contract::BIRTH_BACKEND_SITE_STRING_CONCAT_HH_MATERIALIZE_OWNED_BYTES_FIELD
                .read(&after)
                - contract::BIRTH_BACKEND_SITE_STRING_CONCAT_HH_MATERIALIZE_OWNED_BYTES_FIELD
                    .read(&before),
            18
        );
        assert_eq!(
            contract::BIRTH_BACKEND_SITE_STRING_CONCAT_HH_OBJECTIZE_BOX_TOTAL_FIELD.read(&after)
                - contract::BIRTH_BACKEND_SITE_STRING_CONCAT_HH_OBJECTIZE_BOX_TOTAL_FIELD
                    .read(&before),
            1
        );
        assert_eq!(
            contract::BIRTH_BACKEND_SITE_STRING_CONCAT_HH_PUBLISH_HANDLE_TOTAL_FIELD.read(&after)
                - contract::BIRTH_BACKEND_SITE_STRING_CONCAT_HH_PUBLISH_HANDLE_TOTAL_FIELD
                    .read(&before),
            1
        );
        assert_eq!(
            contract::BIRTH_BACKEND_SITE_STRING_SUBSTRING_CONCAT_HHII_MATERIALIZE_OWNED_TOTAL_FIELD
                .read(&after)
                - contract::BIRTH_BACKEND_SITE_STRING_SUBSTRING_CONCAT_HHII_MATERIALIZE_OWNED_TOTAL_FIELD
                    .read(&before),
            1
        );
        assert_eq!(
            contract::BIRTH_BACKEND_SITE_STRING_SUBSTRING_CONCAT_HHII_MATERIALIZE_OWNED_BYTES_FIELD
                .read(&after)
                - contract::BIRTH_BACKEND_SITE_STRING_SUBSTRING_CONCAT_HHII_MATERIALIZE_OWNED_BYTES_FIELD
                    .read(&before),
            7
        );
        assert_eq!(
            contract::BIRTH_BACKEND_SITE_STRING_SUBSTRING_CONCAT_HHII_OBJECTIZE_BOX_TOTAL_FIELD
                .read(&after)
                - contract::BIRTH_BACKEND_SITE_STRING_SUBSTRING_CONCAT_HHII_OBJECTIZE_BOX_TOTAL_FIELD
                    .read(&before),
            1
        );
        assert_eq!(
            contract::BIRTH_BACKEND_SITE_STRING_SUBSTRING_CONCAT_HHII_PUBLISH_HANDLE_TOTAL_FIELD
                .read(&after)
                - contract::BIRTH_BACKEND_SITE_STRING_SUBSTRING_CONCAT_HHII_PUBLISH_HANDLE_TOTAL_FIELD
                    .read(&before),
            1
        );
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

        assert_eq!(
            contract::BIRTH_BACKEND_SITE_CONST_SUFFIX_MATERIALIZE_OWNED_TOTAL_FIELD.read(&after)
                - contract::BIRTH_BACKEND_SITE_CONST_SUFFIX_MATERIALIZE_OWNED_TOTAL_FIELD
                    .read(&before),
            1
        );
        assert_eq!(
            contract::BIRTH_BACKEND_SITE_CONST_SUFFIX_MATERIALIZE_OWNED_BYTES_FIELD.read(&after)
                - contract::BIRTH_BACKEND_SITE_CONST_SUFFIX_MATERIALIZE_OWNED_BYTES_FIELD
                    .read(&before),
            11
        );
        assert_eq!(
            contract::BIRTH_BACKEND_SITE_CONST_SUFFIX_OBJECTIZE_BOX_TOTAL_FIELD.read(&after)
                - contract::BIRTH_BACKEND_SITE_CONST_SUFFIX_OBJECTIZE_BOX_TOTAL_FIELD.read(&before),
            1
        );
        assert_eq!(
            contract::BIRTH_BACKEND_SITE_CONST_SUFFIX_PUBLISH_HANDLE_TOTAL_FIELD.read(&after)
                - contract::BIRTH_BACKEND_SITE_CONST_SUFFIX_PUBLISH_HANDLE_TOTAL_FIELD
                    .read(&before),
            1
        );
        assert_eq!(
            contract::BIRTH_BACKEND_SITE_FREEZE_TEXT_PLAN_PIECES3_MATERIALIZE_OWNED_TOTAL_FIELD
                .read(&after)
                - contract::BIRTH_BACKEND_SITE_FREEZE_TEXT_PLAN_PIECES3_MATERIALIZE_OWNED_TOTAL_FIELD
                    .read(&before),
            1
        );
        assert_eq!(
            contract::BIRTH_BACKEND_SITE_FREEZE_TEXT_PLAN_PIECES3_MATERIALIZE_OWNED_BYTES_FIELD
                .read(&after)
                - contract::BIRTH_BACKEND_SITE_FREEZE_TEXT_PLAN_PIECES3_MATERIALIZE_OWNED_BYTES_FIELD
                    .read(&before),
            19
        );
        assert_eq!(
            contract::BIRTH_BACKEND_SITE_FREEZE_TEXT_PLAN_PIECES3_OBJECTIZE_BOX_TOTAL_FIELD
                .read(&after)
                - contract::BIRTH_BACKEND_SITE_FREEZE_TEXT_PLAN_PIECES3_OBJECTIZE_BOX_TOTAL_FIELD
                    .read(&before),
            1
        );
        assert_eq!(
            contract::BIRTH_BACKEND_SITE_FREEZE_TEXT_PLAN_PIECES3_PUBLISH_HANDLE_TOTAL_FIELD
                .read(&after)
                - contract::BIRTH_BACKEND_SITE_FREEZE_TEXT_PLAN_PIECES3_PUBLISH_HANDLE_TOTAL_FIELD
                    .read(&before),
            1
        );
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

        assert_eq!(
            contract::STR_CONCAT2_ROUTE_TOTAL_FIELD.read(&after)
                - contract::STR_CONCAT2_ROUTE_TOTAL_FIELD.read(&before),
            1
        );
        assert_eq!(
            contract::STR_CONCAT2_ROUTE_FAST_STR_OWNED_FIELD.read(&after)
                - contract::STR_CONCAT2_ROUTE_FAST_STR_OWNED_FIELD.read(&before),
            1
        );
        assert_eq!(
            contract::STR_LEN_ROUTE_TOTAL_FIELD.read(&after)
                - contract::STR_LEN_ROUTE_TOTAL_FIELD.read(&before),
            2
        );
        assert_eq!(
            contract::STR_LEN_ROUTE_FAST_STR_HIT_FIELD.read(&after)
                - contract::STR_LEN_ROUTE_FAST_STR_HIT_FIELD.read(&before),
            1
        );
        assert_eq!(
            contract::STR_LEN_ROUTE_LATEST_FRESH_HANDLE_FAST_STR_HIT_FIELD.read(&after)
                - contract::STR_LEN_ROUTE_LATEST_FRESH_HANDLE_FAST_STR_HIT_FIELD.read(&before),
            1
        );
        assert_eq!(
            contract::STR_LEN_ROUTE_LATEST_FRESH_HANDLE_FALLBACK_HIT_FIELD.read(&after)
                - contract::STR_LEN_ROUTE_LATEST_FRESH_HANDLE_FALLBACK_HIT_FIELD.read(&before),
            1
        );
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

        assert_eq!(
            contract::STR_SUBSTRING_ROUTE_TOTAL_FIELD.read(&after)
                - contract::STR_SUBSTRING_ROUTE_TOTAL_FIELD.read(&before),
            1
        );
        assert_eq!(
            contract::STR_SUBSTRING_ROUTE_VIEW_ARC_CACHE_MISS_FIELD.read(&after)
                - contract::STR_SUBSTRING_ROUTE_VIEW_ARC_CACHE_MISS_FIELD.read(&before),
            1
        );
        assert_eq!(
            contract::STR_SUBSTRING_ROUTE_FAST_CACHE_HIT_FIELD.read(&after)
                - contract::STR_SUBSTRING_ROUTE_FAST_CACHE_HIT_FIELD.read(&before),
            1
        );
        assert_eq!(
            contract::STR_SUBSTRING_ROUTE_DISPATCH_HIT_FIELD.read(&after)
                - contract::STR_SUBSTRING_ROUTE_DISPATCH_HIT_FIELD.read(&before),
            1
        );
        assert_eq!(
            contract::STR_SUBSTRING_ROUTE_SLOW_PLAN_FIELD.read(&after)
                - contract::STR_SUBSTRING_ROUTE_SLOW_PLAN_FIELD.read(&before),
            1
        );
        assert_eq!(
            contract::STR_SUBSTRING_ROUTE_SLOW_PLAN_RETURN_HANDLE_FIELD.read(&after)
                - contract::STR_SUBSTRING_ROUTE_SLOW_PLAN_RETURN_HANDLE_FIELD.read(&before),
            1
        );
        assert_eq!(
            contract::STR_SUBSTRING_ROUTE_SLOW_PLAN_RETURN_EMPTY_FIELD.read(&after)
                - contract::STR_SUBSTRING_ROUTE_SLOW_PLAN_RETURN_EMPTY_FIELD.read(&before),
            1
        );
        assert_eq!(
            contract::STR_SUBSTRING_ROUTE_SLOW_PLAN_FREEZE_SPAN_FIELD.read(&after)
                - contract::STR_SUBSTRING_ROUTE_SLOW_PLAN_FREEZE_SPAN_FIELD.read(&before),
            1
        );
        assert_eq!(
            contract::STR_SUBSTRING_ROUTE_SLOW_PLAN_VIEW_SPAN_FIELD.read(&after)
                - contract::STR_SUBSTRING_ROUTE_SLOW_PLAN_VIEW_SPAN_FIELD.read(&before),
            1
        );
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

        assert_eq!(
            contract::PIECEWISE_SUBRANGE_TOTAL_FIELD.read(&after)
                - contract::PIECEWISE_SUBRANGE_TOTAL_FIELD.read(&before),
            3
        );
        assert_eq!(
            contract::PIECEWISE_SUBRANGE_SINGLE_SESSION_HIT_FIELD.read(&after)
                - contract::PIECEWISE_SUBRANGE_SINGLE_SESSION_HIT_FIELD.read(&before),
            1
        );
        assert_eq!(
            contract::PIECEWISE_SUBRANGE_FALLBACK_INSERT_FIELD.read(&after)
                - contract::PIECEWISE_SUBRANGE_FALLBACK_INSERT_FIELD.read(&before),
            1
        );
        assert_eq!(
            contract::PIECEWISE_SUBRANGE_EMPTY_RETURN_FIELD.read(&after)
                - contract::PIECEWISE_SUBRANGE_EMPTY_RETURN_FIELD.read(&before),
            1
        );
        assert_eq!(
            contract::PIECEWISE_SUBRANGE_PREFIX_MIDDLE_FIELD.read(&after)
                - contract::PIECEWISE_SUBRANGE_PREFIX_MIDDLE_FIELD.read(&before),
            1
        );
    });
}
