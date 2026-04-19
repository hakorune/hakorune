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
    pub(crate) fn bypass_gc_alloc_enabled() -> bool {
        false
    }

    #[inline(always)]
    pub(crate) fn record_store_array_str_enter() {}

    #[inline(always)]
    pub(crate) fn record_store_array_str_cache_probe(_kind: CacheProbeKind) {}

    #[inline(always)]
    pub(crate) fn record_store_array_str_retarget_hit() {}

    #[inline(always)]
    pub(crate) fn record_store_array_str_latest_fresh_retarget_hit() {}

    #[inline(always)]
    pub(crate) fn record_store_array_str_source_store() {}

    #[inline(always)]
    pub(crate) fn record_store_array_str_latest_fresh_source_store() {}

    #[inline(always)]
    pub(crate) fn record_store_array_str_non_string_source() {}

    #[inline(always)]
    pub(crate) fn record_store_array_str_existing_slot() {}

    #[inline(always)]
    pub(crate) fn record_store_array_str_append_slot() {}

    #[inline(always)]
    pub(crate) fn record_store_array_str_source_string_box() {}

    #[inline(always)]
    pub(crate) fn record_store_array_str_source_string_view() {}

    #[inline(always)]
    pub(crate) fn record_store_array_str_source_missing() {}

    #[inline(always)]
    pub(crate) fn record_store_array_str_plan_source_kind_string_like() {}

    #[inline(always)]
    pub(crate) fn record_store_array_str_plan_source_kind_other_object() {}

    #[inline(always)]
    pub(crate) fn record_store_array_str_plan_source_kind_missing() {}

    #[inline(always)]
    pub(crate) fn record_store_array_str_plan_slot_kind_borrowed_alias() {}

    #[inline(always)]
    pub(crate) fn record_store_array_str_plan_slot_kind_other() {}

    #[inline(always)]
    pub(crate) fn record_store_array_str_plan_action_retarget_alias() {}

    #[inline(always)]
    pub(crate) fn record_store_array_str_plan_action_store_from_source() {}

    #[inline(always)]
    pub(crate) fn record_store_array_str_plan_action_need_stable_object() {}

    #[inline(always)]
    pub(crate) fn record_store_array_str_reason_source_kind_via_object() {}

    #[inline(always)]
    pub(crate) fn record_store_array_str_reason_retarget_keep_source_arc() {}

    #[inline(always)]
    pub(crate) fn record_store_array_str_reason_retarget_keep_source_arc_ptr_eq_hit() {}

    #[inline(always)]
    pub(crate) fn record_store_array_str_reason_retarget_keep_source_arc_ptr_eq_miss() {}

    #[inline(always)]
    pub(crate) fn record_store_array_str_reason_retarget_alias_update() {}

    #[inline(always)]
    pub(crate) fn record_store_array_str_lookup_registry_slot_read() {}

    #[inline(always)]
    pub(crate) fn record_store_array_str_lookup_caller_latest_fresh_tag() {}

    #[inline(always)]
    pub(crate) fn record_const_suffix_enter() {}

    #[inline(always)]
    pub(crate) fn record_const_suffix_cached_handle_hit() {}

    #[inline(always)]
    pub(crate) fn record_const_suffix_text_cache_reload() {}

    #[inline(always)]
    pub(crate) fn record_const_suffix_freeze_fallback() {}

    #[inline(always)]
    pub(crate) fn record_const_suffix_empty_return() {}

    #[inline(always)]
    pub(crate) fn record_const_suffix_cached_fast_str_hit() {}

    #[inline(always)]
    pub(crate) fn record_const_suffix_cached_span_hit() {}

    #[inline(always)]
    pub(crate) fn record_birth_placement_return_handle() {}

    #[inline(always)]
    pub(crate) fn record_birth_placement_borrow_view() {}

    #[inline(always)]
    pub(crate) fn record_birth_placement_freeze_owned() {}

    #[inline(always)]
    pub(crate) fn record_birth_placement_fresh_handle() {}

    #[inline(always)]
    pub(crate) fn record_birth_placement_materialize_owned() {}

    #[inline(always)]
    pub(crate) fn record_birth_placement_store_from_source() {}

    #[inline(always)]
    pub(crate) fn record_birth_backend_freeze_text_plan_view1() {}

    #[inline(always)]
    pub(crate) fn record_birth_backend_freeze_text_plan_pieces2() {}

    #[inline(always)]
    pub(crate) fn record_birth_backend_freeze_text_plan_pieces3() {}

    #[inline(always)]
    pub(crate) fn record_birth_backend_freeze_text_plan_pieces4() {}

    #[inline(always)]
    pub(crate) fn record_birth_backend_freeze_text_plan_owned_tmp() {}

    #[inline(always)]
    pub(crate) fn record_birth_backend_materialize_owned(_bytes: usize) {}

    #[inline(always)]
    pub(crate) fn record_birth_backend_string_box_new(_bytes: usize) {}

    #[inline(always)]
    pub(crate) fn record_birth_backend_string_box_ctor(_bytes: usize) {}

    #[inline(always)]
    pub(crate) fn record_birth_backend_arc_wrap() {}

    #[inline(always)]
    pub(crate) fn record_birth_backend_objectize_stable_box_now(_bytes: usize) {}

    #[inline(always)]
    pub(crate) fn record_birth_backend_handle_issue() {}

    #[inline(always)]
    pub(crate) fn record_birth_backend_issue_fresh_handle() {}

    #[inline(always)]
    pub(crate) fn record_birth_backend_gc_alloc(_bytes: usize) {}

    #[inline(always)]
    pub(crate) fn record_birth_backend_gc_alloc_skipped() {}

    #[inline(always)]
    pub(crate) fn record_birth_backend_carrier_kind_stable_box() {}

    #[inline(always)]
    pub(crate) fn record_birth_backend_carrier_kind_source_keep() {}

    #[inline(always)]
    pub(crate) fn record_birth_backend_carrier_kind_owned_bytes() {}

    #[inline(always)]
    pub(crate) fn record_birth_backend_carrier_kind_handle() {}

    #[inline(always)]
    pub(crate) fn record_birth_backend_publish_reason_external_boundary() {}

    #[inline(always)]
    pub(crate) fn record_birth_backend_publish_reason_need_stable_object() {}

    #[inline(always)]
    pub(crate) fn record_birth_backend_publish_reason_generic_fallback() {}

    #[inline(always)]
    pub(crate) fn record_birth_backend_publish_reason_explicit_api() {}

    #[inline(always)]
    pub(crate) fn record_birth_backend_publish_boundary_slot_publish_handle() {}

    #[inline(always)]
    pub(crate) fn record_birth_backend_publish_boundary_slot_objectize_stable_box() {}

    #[inline(always)]
    pub(crate) fn record_birth_backend_publish_boundary_slot_empty() {}

    #[inline(always)]
    pub(crate) fn record_birth_backend_publish_boundary_slot_already_published() {}

    #[inline(always)]
    pub(crate) fn record_birth_backend_site_string_concat_hh_materialize_owned(_bytes: usize) {}

    #[inline(always)]
    pub(crate) fn record_birth_backend_site_string_concat_hh_objectize_box() {}

    #[inline(always)]
    pub(crate) fn record_birth_backend_site_string_concat_hh_publish_handle() {}

    #[inline(always)]
    pub(crate) fn record_birth_backend_site_string_substring_concat_hhii_materialize_owned(
        _bytes: usize,
    ) {
    }

    #[inline(always)]
    pub(crate) fn record_birth_backend_site_string_substring_concat_hhii_objectize_box() {}

    #[inline(always)]
    pub(crate) fn record_birth_backend_site_string_substring_concat_hhii_publish_handle() {}

    #[inline(always)]
    pub(crate) fn record_birth_backend_site_const_suffix_materialize_owned(_bytes: usize) {}

    #[inline(always)]
    pub(crate) fn record_birth_backend_site_const_suffix_objectize_box() {}

    #[inline(always)]
    pub(crate) fn record_birth_backend_site_const_suffix_publish_handle() {}

    #[inline(always)]
    pub(crate) fn record_birth_backend_site_freeze_text_plan_pieces3_materialize_owned(
        _bytes: usize,
    ) {
    }

    #[inline(always)]
    pub(crate) fn record_birth_backend_site_freeze_text_plan_pieces3_objectize_box() {}

    #[inline(always)]
    pub(crate) fn record_birth_backend_site_freeze_text_plan_pieces3_publish_handle() {}

    #[inline(always)]
    pub(crate) fn record_str_concat2_route_enter() {}

    #[inline(always)]
    pub(crate) fn record_str_concat2_route_dispatch_hit() {}

    #[inline(always)]
    pub(crate) fn record_str_concat2_route_fast_str_owned() {}

    #[inline(always)]
    pub(crate) fn record_str_concat2_route_fast_str_return_handle() {}

    #[inline(always)]
    pub(crate) fn record_str_concat2_route_span_freeze() {}

    #[inline(always)]
    pub(crate) fn record_str_concat2_route_span_return_handle() {}

    #[inline(always)]
    pub(crate) fn record_str_concat2_route_materialize_fallback() {}

    #[inline(always)]
    pub(crate) fn record_str_len_route_enter() {}

    #[inline(always)]
    pub(crate) fn record_str_len_route_dispatch_hit() {}

    #[inline(always)]
    pub(crate) fn record_str_len_route_fast_str_hit() {}

    #[inline(always)]
    pub(crate) fn record_str_len_route_latest_fresh_handle_fast_str_hit() {}

    #[inline(always)]
    pub(crate) fn record_str_len_route_fallback_hit() {}

    #[inline(always)]
    pub(crate) fn record_str_len_route_latest_fresh_handle_fallback_hit() {}

    #[inline(always)]
    pub(crate) fn record_str_len_route_miss() {}

    #[inline(always)]
    pub(crate) fn record_str_substring_route_enter() {}

    #[inline(always)]
    pub(crate) fn record_str_substring_route_view_arc_cache_handle_hit() {}

    #[inline(always)]
    pub(crate) fn record_str_substring_route_view_arc_cache_reissue_hit() {}

    #[inline(always)]
    pub(crate) fn record_str_substring_route_view_arc_cache_miss() {}

    #[inline(always)]
    pub(crate) fn record_str_substring_route_fast_cache_hit() {}

    #[inline(always)]
    pub(crate) fn record_str_substring_route_dispatch_hit() {}

    #[inline(always)]
    pub(crate) fn record_str_substring_route_slow_plan() {}

    #[inline(always)]
    pub(crate) fn record_str_substring_route_slow_plan_return_handle() {}

    #[inline(always)]
    pub(crate) fn record_str_substring_route_slow_plan_return_empty() {}

    #[inline(always)]
    pub(crate) fn record_str_substring_route_slow_plan_freeze_span() {}

    #[inline(always)]
    pub(crate) fn record_str_substring_route_slow_plan_view_span() {}

    #[inline(always)]
    pub(crate) fn record_piecewise_subrange_enter() {}

    #[inline(always)]
    pub(crate) fn record_piecewise_subrange_single_session_hit() {}

    #[inline(always)]
    pub(crate) fn record_piecewise_subrange_fallback_insert() {}

    #[inline(always)]
    pub(crate) fn record_piecewise_subrange_empty_return() {}

    #[inline(always)]
    pub(crate) fn record_piecewise_subrange_prefix_only() {}

    #[inline(always)]
    pub(crate) fn record_piecewise_subrange_middle_only() {}

    #[inline(always)]
    pub(crate) fn record_piecewise_subrange_suffix_only() {}

    #[inline(always)]
    pub(crate) fn record_piecewise_subrange_prefix_middle() {}

    #[inline(always)]
    pub(crate) fn record_piecewise_subrange_middle_suffix() {}

    #[inline(always)]
    pub(crate) fn record_piecewise_subrange_prefix_suffix() {}

    #[inline(always)]
    pub(crate) fn record_piecewise_subrange_all_three() {}

    #[inline(always)]
    pub(crate) fn record_borrowed_alias_to_string_box() {}

    #[inline(always)]
    pub(crate) fn record_borrowed_alias_equals() {}

    #[inline(always)]
    pub(crate) fn record_borrowed_alias_clone_box() {}

    #[inline(always)]
    pub(crate) fn record_borrowed_alias_to_string_box_latest_fresh() {}

    #[inline(always)]
    pub(crate) fn record_borrowed_alias_equals_latest_fresh() {}

    #[inline(always)]
    pub(crate) fn record_borrowed_alias_clone_box_latest_fresh() {}

    #[inline(always)]
    pub(crate) fn record_borrowed_alias_borrowed_source_fast() {}

    #[inline(always)]
    pub(crate) fn record_borrowed_alias_as_str_fast() {}

    #[inline(always)]
    pub(crate) fn record_borrowed_alias_as_str_fast_live_source() {}

    #[inline(always)]
    pub(crate) fn record_borrowed_alias_as_str_fast_stale_source() {}

    #[inline(always)]
    pub(crate) fn record_borrowed_alias_array_len_by_index_latest_fresh() {}

    #[inline(always)]
    pub(crate) fn record_borrowed_alias_array_indexof_by_index_latest_fresh() {}

    #[inline(always)]
    pub(crate) fn record_borrowed_alias_encode_live_source_hit() {}

    #[inline(always)]
    pub(crate) fn record_borrowed_alias_encode_live_source_hit_array_get_index() {}

    #[inline(always)]
    pub(crate) fn record_borrowed_alias_encode_live_source_hit_map_runtime_data_get_any() {}

    #[inline(always)]
    pub(crate) fn record_borrowed_alias_encode_epoch_hit() {}

    #[inline(always)]
    pub(crate) fn record_borrowed_alias_encode_cached_handle_hit() {}

    #[inline(always)]
    pub(crate) fn record_borrowed_alias_encode_cached_handle_hit_array_get_index() {}

    #[inline(always)]
    pub(crate) fn record_borrowed_alias_encode_cached_handle_hit_map_runtime_data_get_any() {}

    #[inline(always)]
    pub(crate) fn record_borrowed_alias_encode_ptr_eq_hit() {}

    #[inline(always)]
    pub(crate) fn record_borrowed_alias_encode_to_handle_arc() {}

    #[inline(always)]
    pub(crate) fn record_borrowed_alias_encode_to_handle_arc_array_get_index() {}

    #[inline(always)]
    pub(crate) fn record_borrowed_alias_encode_to_handle_arc_map_runtime_data_get_any() {}

    #[inline(always)]
    pub(crate) fn mark_latest_fresh_handle(_handle: i64) {}

    #[inline(always)]
    pub(crate) fn len_route_matches_latest_fresh_handle(_handle: i64) -> bool {
        false
    }

    #[inline(always)]
    pub(crate) fn flush() {}
