impl ThreadCounters {
    #[inline(always)]
    fn bump(cell: &Cell<u64>) {
        cell.set(cell.get() + 1);
    }

    #[inline(always)]
    fn store_array_str_enter(&self) {
        Self::bump(&self.store_array_str_total);
    }

    #[inline(always)]
    fn store_array_str_cache_probe(&self, kind: CacheProbeKind) {
        match kind {
            CacheProbeKind::Hit => Self::bump(&self.store_array_str_cache_hit),
            CacheProbeKind::MissHandle => Self::bump(&self.store_array_str_cache_miss_handle),
            CacheProbeKind::MissDropEpoch => Self::bump(&self.store_array_str_cache_miss_epoch),
        }
    }

    #[inline(always)]
    fn store_array_str_retarget_hit(&self) {
        Self::bump(&self.store_array_str_retarget_hit);
    }

    #[inline(always)]
    fn store_array_str_latest_fresh_retarget_hit(&self) {
        Self::bump(&self.store_array_str_latest_fresh_retarget_hit);
    }

    #[inline(always)]
    fn store_array_str_source_store(&self) {
        Self::bump(&self.store_array_str_source_store);
    }

    #[inline(always)]
    fn store_array_str_latest_fresh_source_store(&self) {
        Self::bump(&self.store_array_str_latest_fresh_source_store);
    }

    #[inline(always)]
    fn store_array_str_non_string_source(&self) {
        Self::bump(&self.store_array_str_non_string_source);
    }

    #[inline(always)]
    fn store_array_str_existing_slot(&self) {
        Self::bump(&self.store_array_str_existing_slot);
    }

    #[inline(always)]
    fn store_array_str_append_slot(&self) {
        Self::bump(&self.store_array_str_append_slot);
    }

    #[inline(always)]
    fn store_array_str_source_string_box(&self) {
        Self::bump(&self.store_array_str_source_string_box);
    }

    #[inline(always)]
    fn store_array_str_source_string_view(&self) {
        Self::bump(&self.store_array_str_source_string_view);
    }

    #[inline(always)]
    fn store_array_str_source_missing(&self) {
        Self::bump(&self.store_array_str_source_missing);
    }

    #[inline(always)]
    fn store_array_str_plan_source_kind_string_like(&self) {
        Self::bump(&self.store_array_str_plan_source_kind_string_like);
    }

    #[inline(always)]
    fn store_array_str_plan_source_kind_other_object(&self) {
        Self::bump(&self.store_array_str_plan_source_kind_other_object);
    }

    #[inline(always)]
    fn store_array_str_plan_source_kind_missing(&self) {
        Self::bump(&self.store_array_str_plan_source_kind_missing);
    }

    #[inline(always)]
    fn store_array_str_plan_slot_kind_borrowed_alias(&self) {
        Self::bump(&self.store_array_str_plan_slot_kind_borrowed_alias);
    }

    #[inline(always)]
    fn store_array_str_plan_slot_kind_other(&self) {
        Self::bump(&self.store_array_str_plan_slot_kind_other);
    }

    #[inline(always)]
    fn store_array_str_plan_action_retarget_alias(&self) {
        Self::bump(&self.store_array_str_plan_action_retarget_alias);
    }

    #[inline(always)]
    fn store_array_str_plan_action_store_from_source(&self) {
        Self::bump(&self.store_array_str_plan_action_store_from_source);
    }

    #[inline(always)]
    fn store_array_str_plan_action_need_stable_object(&self) {
        Self::bump(&self.store_array_str_plan_action_need_stable_object);
    }

    #[inline(always)]
    fn store_array_str_reason_source_kind_via_object(&self) {
        Self::bump(&self.store_array_str_reason_source_kind_via_object);
    }

    #[inline(always)]
    fn store_array_str_reason_retarget_keep_source_arc(&self) {
        Self::bump(&self.store_array_str_reason_retarget_keep_source_arc);
    }

    #[inline(always)]
    fn store_array_str_reason_retarget_keep_source_arc_ptr_eq_hit(&self) {
        Self::bump(&self.store_array_str_reason_retarget_keep_source_arc_ptr_eq_hit);
    }

    #[inline(always)]
    fn store_array_str_reason_retarget_keep_source_arc_ptr_eq_miss(&self) {
        Self::bump(&self.store_array_str_reason_retarget_keep_source_arc_ptr_eq_miss);
    }

    #[inline(always)]
    fn store_array_str_reason_retarget_alias_update(&self) {
        Self::bump(&self.store_array_str_reason_retarget_alias_update);
    }

    #[inline(always)]
    fn store_array_str_lookup_registry_slot_read(&self) {
        Self::bump(&self.store_array_str_lookup_registry_slot_read);
    }

    #[inline(always)]
    fn store_array_str_lookup_caller_latest_fresh_tag(&self) {
        Self::bump(&self.store_array_str_lookup_caller_latest_fresh_tag);
    }

    #[inline(always)]
    fn const_suffix_enter(&self) {
        Self::bump(&self.const_suffix_total);
    }

    #[inline(always)]
    fn const_suffix_cached_handle_hit(&self) {
        Self::bump(&self.const_suffix_cached_handle_hit);
    }

    #[inline(always)]
    fn const_suffix_text_cache_reload(&self) {
        Self::bump(&self.const_suffix_text_cache_reload);
    }

    #[inline(always)]
    fn const_suffix_freeze_fallback(&self) {
        Self::bump(&self.const_suffix_freeze_fallback);
    }

    #[inline(always)]
    fn const_suffix_empty_return(&self) {
        Self::bump(&self.const_suffix_empty_return);
    }

    #[inline(always)]
    fn const_suffix_cached_fast_str_hit(&self) {
        Self::bump(&self.const_suffix_cached_fast_str_hit);
    }

    #[inline(always)]
    fn const_suffix_cached_span_hit(&self) {
        Self::bump(&self.const_suffix_cached_span_hit);
    }

    #[inline(always)]
    fn birth_placement_return_handle(&self) {
        Self::bump(&self.birth_placement_return_handle);
    }

    #[inline(always)]
    fn birth_placement_borrow_view(&self) {
        Self::bump(&self.birth_placement_borrow_view);
    }

    #[inline(always)]
    fn birth_placement_freeze_owned(&self) {
        Self::bump(&self.birth_placement_freeze_owned);
    }

    #[inline(always)]
    fn birth_placement_fresh_handle(&self) {
        Self::bump(&self.birth_placement_fresh_handle);
    }

    #[inline(always)]
    fn birth_placement_materialize_owned(&self) {
        Self::bump(&self.birth_placement_materialize_owned);
    }

    #[inline(always)]
    fn birth_placement_store_from_source(&self) {
        Self::bump(&self.birth_placement_store_from_source);
    }

    #[inline(always)]
    fn birth_backend_freeze_text_plan_view1(&self) {
        Self::bump(&self.birth_backend_freeze_text_plan_total);
        Self::bump(&self.birth_backend_freeze_text_plan_view1);
    }

    #[inline(always)]
    fn birth_backend_freeze_text_plan_pieces2(&self) {
        Self::bump(&self.birth_backend_freeze_text_plan_total);
        Self::bump(&self.birth_backend_freeze_text_plan_pieces2);
    }

    #[inline(always)]
    fn birth_backend_freeze_text_plan_pieces3(&self) {
        Self::bump(&self.birth_backend_freeze_text_plan_total);
        Self::bump(&self.birth_backend_freeze_text_plan_pieces3);
    }

    #[inline(always)]
    fn birth_backend_freeze_text_plan_pieces4(&self) {
        Self::bump(&self.birth_backend_freeze_text_plan_total);
        Self::bump(&self.birth_backend_freeze_text_plan_pieces4);
    }

    #[inline(always)]
    fn birth_backend_freeze_text_plan_owned_tmp(&self) {
        Self::bump(&self.birth_backend_freeze_text_plan_total);
        Self::bump(&self.birth_backend_freeze_text_plan_owned_tmp);
    }

    #[inline(always)]
    fn birth_backend_string_box_new(&self, bytes: u64) {
        Self::bump(&self.birth_backend_string_box_new_total);
        self.birth_backend_string_box_new_bytes
            .set(self.birth_backend_string_box_new_bytes.get() + bytes);
    }

    #[inline(always)]
    fn birth_backend_string_box_ctor(&self, bytes: u64) {
        Self::bump(&self.birth_backend_string_box_ctor_total);
        self.birth_backend_string_box_ctor_bytes
            .set(self.birth_backend_string_box_ctor_bytes.get() + bytes);
    }

    #[inline(always)]
    fn birth_backend_arc_wrap(&self) {
        Self::bump(&self.birth_backend_arc_wrap_total);
    }

    #[inline(always)]
    fn birth_backend_objectize_stable_box_now(&self, bytes: u64) {
        Self::bump(&self.birth_backend_objectize_stable_box_now_total);
        self.birth_backend_objectize_stable_box_now_bytes
            .set(self.birth_backend_objectize_stable_box_now_bytes.get() + bytes);
    }

    #[inline(always)]
    fn birth_backend_handle_issue(&self) {
        Self::bump(&self.birth_backend_handle_issue_total);
    }

    #[inline(always)]
    fn birth_backend_issue_fresh_handle(&self) {
        Self::bump(&self.birth_backend_issue_fresh_handle_total);
    }

    #[inline(always)]
    fn birth_backend_materialize_owned(&self, bytes: u64) {
        Self::bump(&self.birth_backend_materialize_owned_total);
        self.birth_backend_materialize_owned_bytes
            .set(self.birth_backend_materialize_owned_bytes.get() + bytes);
    }

    #[inline(always)]
    fn birth_backend_gc_alloc(&self, bytes: u64) {
        Self::bump(&self.birth_backend_gc_alloc_called);
        self.birth_backend_gc_alloc_bytes
            .set(self.birth_backend_gc_alloc_bytes.get() + bytes);
    }

    #[inline(always)]
    fn birth_backend_gc_alloc_skipped(&self) {
        Self::bump(&self.birth_backend_gc_alloc_skipped);
    }

    #[inline(always)]
    fn birth_backend_carrier_kind_stable_box(&self) {
        Self::bump(&self.birth_backend_carrier_kind_stable_box);
    }

    #[inline(always)]
    fn birth_backend_carrier_kind_source_keep(&self) {
        Self::bump(&self.birth_backend_carrier_kind_source_keep);
    }

    #[inline(always)]
    fn birth_backend_carrier_kind_owned_bytes(&self) {
        Self::bump(&self.birth_backend_carrier_kind_owned_bytes);
    }

    #[inline(always)]
    fn birth_backend_carrier_kind_handle(&self) {
        Self::bump(&self.birth_backend_carrier_kind_handle);
    }

    #[inline(always)]
    fn birth_backend_publish_reason_external_boundary(&self) {
        Self::bump(&self.birth_backend_publish_reason_external_boundary);
    }

    #[inline(always)]
    fn birth_backend_publish_reason_need_stable_object(&self) {
        Self::bump(&self.birth_backend_publish_reason_need_stable_object);
    }

    #[inline(always)]
    fn birth_backend_publish_reason_generic_fallback(&self) {
        Self::bump(&self.birth_backend_publish_reason_generic_fallback);
    }

    #[inline(always)]
    fn birth_backend_publish_reason_explicit_api(&self) {
        Self::bump(&self.birth_backend_publish_reason_explicit_api);
    }

    #[inline(always)]
    fn birth_backend_publish_boundary_slot_publish_handle(&self) {
        Self::bump(&self.birth_backend_publish_boundary_slot_publish_handle_total);
    }

    #[inline(always)]
    fn birth_backend_publish_boundary_slot_objectize_stable_box(&self) {
        Self::bump(&self.birth_backend_publish_boundary_slot_objectize_stable_box_total);
    }

    #[inline(always)]
    fn birth_backend_publish_boundary_slot_empty(&self) {
        Self::bump(&self.birth_backend_publish_boundary_slot_empty);
    }

    #[inline(always)]
    fn birth_backend_publish_boundary_slot_already_published(&self) {
        Self::bump(&self.birth_backend_publish_boundary_slot_already_published);
    }

    #[inline(always)]
    fn birth_backend_site_string_concat_hh_materialize_owned(&self, bytes: u64) {
        Self::bump(&self.birth_backend_site_string_concat_hh_materialize_owned_total);
        self.birth_backend_site_string_concat_hh_materialize_owned_bytes
            .set(self.birth_backend_site_string_concat_hh_materialize_owned_bytes.get() + bytes);
    }

    #[inline(always)]
    fn birth_backend_site_string_concat_hh_objectize_box(&self) {
        Self::bump(&self.birth_backend_site_string_concat_hh_objectize_box_total);
    }

    #[inline(always)]
    fn birth_backend_site_string_concat_hh_publish_handle(&self) {
        Self::bump(&self.birth_backend_site_string_concat_hh_publish_handle_total);
    }

    #[inline(always)]
    fn birth_backend_site_string_substring_concat_hhii_materialize_owned(&self, bytes: u64) {
        Self::bump(
            &self.birth_backend_site_string_substring_concat_hhii_materialize_owned_total,
        );
        self.birth_backend_site_string_substring_concat_hhii_materialize_owned_bytes.set(
            self.birth_backend_site_string_substring_concat_hhii_materialize_owned_bytes.get()
                + bytes,
        );
    }

    #[inline(always)]
    fn birth_backend_site_string_substring_concat_hhii_objectize_box(&self) {
        Self::bump(&self.birth_backend_site_string_substring_concat_hhii_objectize_box_total);
    }

    #[inline(always)]
    fn birth_backend_site_string_substring_concat_hhii_publish_handle(&self) {
        Self::bump(&self.birth_backend_site_string_substring_concat_hhii_publish_handle_total);
    }

    #[inline(always)]
    fn birth_backend_site_const_suffix_materialize_owned(&self, bytes: u64) {
        Self::bump(&self.birth_backend_site_const_suffix_materialize_owned_total);
        self.birth_backend_site_const_suffix_materialize_owned_bytes
            .set(self.birth_backend_site_const_suffix_materialize_owned_bytes.get() + bytes);
    }

    #[inline(always)]
    fn birth_backend_site_const_suffix_objectize_box(&self) {
        Self::bump(&self.birth_backend_site_const_suffix_objectize_box_total);
    }

    #[inline(always)]
    fn birth_backend_site_const_suffix_publish_handle(&self) {
        Self::bump(&self.birth_backend_site_const_suffix_publish_handle_total);
    }

    #[inline(always)]
    fn birth_backend_site_freeze_text_plan_pieces3_materialize_owned(&self, bytes: u64) {
        Self::bump(&self.birth_backend_site_freeze_text_plan_pieces3_materialize_owned_total);
        self.birth_backend_site_freeze_text_plan_pieces3_materialize_owned_bytes.set(
            self.birth_backend_site_freeze_text_plan_pieces3_materialize_owned_bytes.get() + bytes,
        );
    }

    #[inline(always)]
    fn birth_backend_site_freeze_text_plan_pieces3_objectize_box(&self) {
        Self::bump(&self.birth_backend_site_freeze_text_plan_pieces3_objectize_box_total);
    }

    #[inline(always)]
    fn birth_backend_site_freeze_text_plan_pieces3_publish_handle(&self) {
        Self::bump(&self.birth_backend_site_freeze_text_plan_pieces3_publish_handle_total);
    }

    #[inline(always)]
    fn str_concat2_route_enter(&self) {
        Self::bump(&self.str_concat2_route_total);
    }

    #[inline(always)]
    fn str_concat2_route_dispatch_hit(&self) {
        Self::bump(&self.str_concat2_route_dispatch_hit);
    }

    #[inline(always)]
    fn str_concat2_route_fast_str_owned(&self) {
        Self::bump(&self.str_concat2_route_fast_str_owned);
    }

    #[inline(always)]
    fn str_concat2_route_fast_str_return_handle(&self) {
        Self::bump(&self.str_concat2_route_fast_str_return_handle);
    }

    #[inline(always)]
    fn str_concat2_route_span_freeze(&self) {
        Self::bump(&self.str_concat2_route_span_freeze);
    }

    #[inline(always)]
    fn str_concat2_route_span_return_handle(&self) {
        Self::bump(&self.str_concat2_route_span_return_handle);
    }

    #[inline(always)]
    fn str_concat2_route_materialize_fallback(&self) {
        Self::bump(&self.str_concat2_route_materialize_fallback);
    }

    #[inline(always)]
    fn str_len_route_enter(&self) {
        Self::bump(&self.str_len_route_total);
    }

    #[inline(always)]
    fn str_len_route_dispatch_hit(&self) {
        Self::bump(&self.str_len_route_total);
        Self::bump(&self.str_len_route_dispatch_hit);
    }

    #[inline(always)]
    fn str_len_route_fast_str_hit(&self) {
        Self::bump(&self.str_len_route_total);
        Self::bump(&self.str_len_route_fast_str_hit);
    }

    #[inline(always)]
    fn str_len_route_fallback_hit(&self) {
        Self::bump(&self.str_len_route_total);
        Self::bump(&self.str_len_route_fallback_hit);
    }

    #[inline(always)]
    fn str_len_route_miss(&self) {
        Self::bump(&self.str_len_route_total);
        Self::bump(&self.str_len_route_miss);
    }

    #[inline(always)]
    fn str_len_route_latest_fresh_handle_fast_str_hit(&self) {
        Self::bump(&self.str_len_route_latest_fresh_handle_fast_str_hit);
    }

    #[inline(always)]
    fn str_len_route_latest_fresh_handle_fallback_hit(&self) {
        Self::bump(&self.str_len_route_latest_fresh_handle_fallback_hit);
    }

    #[inline(always)]
    fn str_substring_route_enter(&self) {
        Self::bump(&self.str_substring_route_total);
    }

    #[inline(always)]
    fn str_substring_route_view_arc_cache_handle_hit(&self) {
        Self::bump(&self.str_substring_route_view_arc_cache_handle_hit);
    }

    #[inline(always)]
    fn str_substring_route_view_arc_cache_reissue_hit(&self) {
        Self::bump(&self.str_substring_route_view_arc_cache_reissue_hit);
    }

    #[inline(always)]
    fn str_substring_route_view_arc_cache_miss(&self) {
        Self::bump(&self.str_substring_route_view_arc_cache_miss);
    }

    #[inline(always)]
    fn str_substring_route_fast_cache_hit(&self) {
        Self::bump(&self.str_substring_route_fast_cache_hit);
    }

    #[inline(always)]
    fn str_substring_route_dispatch_hit(&self) {
        Self::bump(&self.str_substring_route_dispatch_hit);
    }

    #[inline(always)]
    fn str_substring_route_slow_plan(&self) {
        Self::bump(&self.str_substring_route_slow_plan);
    }

    #[inline(always)]
    fn str_substring_route_slow_plan_return_handle(&self) {
        Self::bump(&self.str_substring_route_slow_plan_return_handle);
    }

    #[inline(always)]
    fn str_substring_route_slow_plan_return_empty(&self) {
        Self::bump(&self.str_substring_route_slow_plan_return_empty);
    }

    #[inline(always)]
    fn str_substring_route_slow_plan_freeze_span(&self) {
        Self::bump(&self.str_substring_route_slow_plan_freeze_span);
    }

    #[inline(always)]
    fn str_substring_route_slow_plan_view_span(&self) {
        Self::bump(&self.str_substring_route_slow_plan_view_span);
    }

    #[inline(always)]
    fn piecewise_subrange_enter(&self) {
        Self::bump(&self.piecewise_subrange_total);
    }

    #[inline(always)]
    fn piecewise_subrange_single_session_hit(&self) {
        Self::bump(&self.piecewise_subrange_single_session_hit);
    }

    #[inline(always)]
    fn piecewise_subrange_fallback_insert(&self) {
        Self::bump(&self.piecewise_subrange_fallback_insert);
    }

    #[inline(always)]
    fn piecewise_subrange_empty_return(&self) {
        Self::bump(&self.piecewise_subrange_empty_return);
    }

    #[inline(always)]
    fn piecewise_subrange_prefix_only(&self) {
        Self::bump(&self.piecewise_subrange_prefix_only);
    }

    #[inline(always)]
    fn piecewise_subrange_middle_only(&self) {
        Self::bump(&self.piecewise_subrange_middle_only);
    }

    #[inline(always)]
    fn piecewise_subrange_suffix_only(&self) {
        Self::bump(&self.piecewise_subrange_suffix_only);
    }

    #[inline(always)]
    fn piecewise_subrange_prefix_middle(&self) {
        Self::bump(&self.piecewise_subrange_prefix_middle);
    }

    #[inline(always)]
    fn piecewise_subrange_middle_suffix(&self) {
        Self::bump(&self.piecewise_subrange_middle_suffix);
    }

    #[inline(always)]
    fn piecewise_subrange_prefix_suffix(&self) {
        Self::bump(&self.piecewise_subrange_prefix_suffix);
    }

    #[inline(always)]
    fn piecewise_subrange_all_three(&self) {
        Self::bump(&self.piecewise_subrange_all_three);
    }

    #[inline(always)]
    fn borrowed_alias_to_string_box(&self) {
        Self::bump(&self.borrowed_alias_to_string_box);
    }

    #[inline(always)]
    fn borrowed_alias_equals(&self) {
        Self::bump(&self.borrowed_alias_equals);
    }

    #[inline(always)]
    fn borrowed_alias_clone_box(&self) {
        Self::bump(&self.borrowed_alias_clone_box);
    }

    #[inline(always)]
    fn borrowed_alias_to_string_box_latest_fresh(&self) {
        Self::bump(&self.borrowed_alias_to_string_box_latest_fresh);
    }

    #[inline(always)]
    fn borrowed_alias_equals_latest_fresh(&self) {
        Self::bump(&self.borrowed_alias_equals_latest_fresh);
    }

    #[inline(always)]
    fn borrowed_alias_clone_box_latest_fresh(&self) {
        Self::bump(&self.borrowed_alias_clone_box_latest_fresh);
    }

    #[inline(always)]
    fn borrowed_alias_borrowed_source_fast(&self) {
        Self::bump(&self.borrowed_alias_borrowed_source_fast);
    }

    #[inline(always)]
    fn borrowed_alias_as_str_fast(&self) {
        Self::bump(&self.borrowed_alias_as_str_fast);
    }

    #[inline(always)]
    fn borrowed_alias_as_str_fast_live_source(&self) {
        Self::bump(&self.borrowed_alias_as_str_fast_live_source);
    }

    #[inline(always)]
    fn borrowed_alias_as_str_fast_stale_source(&self) {
        Self::bump(&self.borrowed_alias_as_str_fast_stale_source);
    }

    #[inline(always)]
    fn borrowed_alias_array_len_by_index_latest_fresh(&self) {
        Self::bump(&self.borrowed_alias_array_len_by_index_latest_fresh);
    }

    #[inline(always)]
    fn borrowed_alias_array_indexof_by_index_latest_fresh(&self) {
        Self::bump(&self.borrowed_alias_array_indexof_by_index_latest_fresh);
    }

    #[inline(always)]
    fn borrowed_alias_encode_epoch_hit(&self) {
        Self::bump(&self.borrowed_alias_encode_epoch_hit);
    }

    #[inline(always)]
    fn borrowed_alias_encode_ptr_eq_hit(&self) {
        Self::bump(&self.borrowed_alias_encode_ptr_eq_hit);
    }

    #[inline(always)]
    fn borrowed_alias_encode_to_handle_arc(&self) {
        Self::bump(&self.borrowed_alias_encode_to_handle_arc);
    }

    #[inline(always)]
    fn borrowed_alias_encode_to_handle_arc_array_get_index(&self) {
        Self::bump(&self.borrowed_alias_encode_to_handle_arc_array_get_index);
    }

    #[inline(always)]
    fn borrowed_alias_encode_to_handle_arc_map_runtime_data_get_any(&self) {
        Self::bump(&self.borrowed_alias_encode_to_handle_arc_map_runtime_data_get_any);
    }

    #[inline(always)]
    fn mark_latest_fresh_handle(&self, handle: i64) {
        self.latest_fresh_handle.set(handle);
    }

    #[inline(always)]
    fn matches_latest_fresh_handle(&self, handle: i64) -> bool {
        handle > 0 && self.latest_fresh_handle.get() == handle
    }

    fn flush_into_global(&self) {
        flush_cell(&self.store_array_str_total, &GLOBAL.store_array_str_total);
        flush_cell(
            &self.store_array_str_cache_hit,
            &GLOBAL.store_array_str_cache_hit,
        );
        flush_cell(
            &self.store_array_str_cache_miss_handle,
            &GLOBAL.store_array_str_cache_miss_handle,
        );
        flush_cell(
            &self.store_array_str_cache_miss_epoch,
            &GLOBAL.store_array_str_cache_miss_epoch,
        );
        flush_cell(
            &self.store_array_str_retarget_hit,
            &GLOBAL.store_array_str_retarget_hit,
        );
        flush_cell(
            &self.store_array_str_latest_fresh_retarget_hit,
            &GLOBAL.store_array_str_latest_fresh_retarget_hit,
        );
        flush_cell(
            &self.store_array_str_source_store,
            &GLOBAL.store_array_str_source_store,
        );
        flush_cell(
            &self.store_array_str_latest_fresh_source_store,
            &GLOBAL.store_array_str_latest_fresh_source_store,
        );
        flush_cell(
            &self.store_array_str_non_string_source,
            &GLOBAL.store_array_str_non_string_source,
        );
        flush_cell(
            &self.store_array_str_existing_slot,
            &GLOBAL.store_array_str_existing_slot,
        );
        flush_cell(
            &self.store_array_str_append_slot,
            &GLOBAL.store_array_str_append_slot,
        );
        flush_cell(
            &self.store_array_str_source_string_box,
            &GLOBAL.store_array_str_source_string_box,
        );
        flush_cell(
            &self.store_array_str_source_string_view,
            &GLOBAL.store_array_str_source_string_view,
        );
        flush_cell(
            &self.store_array_str_source_missing,
            &GLOBAL.store_array_str_source_missing,
        );
        flush_cell(
            &self.store_array_str_plan_source_kind_string_like,
            &GLOBAL.store_array_str_plan_source_kind_string_like,
        );
        flush_cell(
            &self.store_array_str_plan_source_kind_other_object,
            &GLOBAL.store_array_str_plan_source_kind_other_object,
        );
        flush_cell(
            &self.store_array_str_plan_source_kind_missing,
            &GLOBAL.store_array_str_plan_source_kind_missing,
        );
        flush_cell(
            &self.store_array_str_plan_slot_kind_borrowed_alias,
            &GLOBAL.store_array_str_plan_slot_kind_borrowed_alias,
        );
        flush_cell(
            &self.store_array_str_plan_slot_kind_other,
            &GLOBAL.store_array_str_plan_slot_kind_other,
        );
        flush_cell(
            &self.store_array_str_plan_action_retarget_alias,
            &GLOBAL.store_array_str_plan_action_retarget_alias,
        );
        flush_cell(
            &self.store_array_str_plan_action_store_from_source,
            &GLOBAL.store_array_str_plan_action_store_from_source,
        );
        flush_cell(
            &self.store_array_str_plan_action_need_stable_object,
            &GLOBAL.store_array_str_plan_action_need_stable_object,
        );
        flush_cell(
            &self.store_array_str_reason_source_kind_via_object,
            &GLOBAL.store_array_str_reason_source_kind_via_object,
        );
        flush_cell(
            &self.store_array_str_reason_retarget_keep_source_arc,
            &GLOBAL.store_array_str_reason_retarget_keep_source_arc,
        );
        flush_cell(
            &self.store_array_str_reason_retarget_keep_source_arc_ptr_eq_hit,
            &GLOBAL.store_array_str_reason_retarget_keep_source_arc_ptr_eq_hit,
        );
        flush_cell(
            &self.store_array_str_reason_retarget_keep_source_arc_ptr_eq_miss,
            &GLOBAL.store_array_str_reason_retarget_keep_source_arc_ptr_eq_miss,
        );
        flush_cell(
            &self.store_array_str_reason_retarget_alias_update,
            &GLOBAL.store_array_str_reason_retarget_alias_update,
        );
        flush_cell(
            &self.store_array_str_lookup_registry_slot_read,
            &GLOBAL.store_array_str_lookup_registry_slot_read,
        );
        flush_cell(
            &self.store_array_str_lookup_caller_latest_fresh_tag,
            &GLOBAL.store_array_str_lookup_caller_latest_fresh_tag,
        );
        flush_cell(&self.const_suffix_total, &GLOBAL.const_suffix_total);
        flush_cell(
            &self.const_suffix_cached_handle_hit,
            &GLOBAL.const_suffix_cached_handle_hit,
        );
        flush_cell(
            &self.const_suffix_text_cache_reload,
            &GLOBAL.const_suffix_text_cache_reload,
        );
        flush_cell(
            &self.const_suffix_freeze_fallback,
            &GLOBAL.const_suffix_freeze_fallback,
        );
        flush_cell(
            &self.const_suffix_empty_return,
            &GLOBAL.const_suffix_empty_return,
        );
        flush_cell(
            &self.const_suffix_cached_fast_str_hit,
            &GLOBAL.const_suffix_cached_fast_str_hit,
        );
        flush_cell(
            &self.const_suffix_cached_span_hit,
            &GLOBAL.const_suffix_cached_span_hit,
        );
        flush_cell(
            &self.birth_placement_return_handle,
            &GLOBAL.birth_placement_return_handle,
        );
        flush_cell(
            &self.birth_placement_borrow_view,
            &GLOBAL.birth_placement_borrow_view,
        );
        flush_cell(
            &self.birth_placement_freeze_owned,
            &GLOBAL.birth_placement_freeze_owned,
        );
        flush_cell(
            &self.birth_placement_fresh_handle,
            &GLOBAL.birth_placement_fresh_handle,
        );
        flush_cell(
            &self.birth_placement_materialize_owned,
            &GLOBAL.birth_placement_materialize_owned,
        );
        flush_cell(
            &self.birth_placement_store_from_source,
            &GLOBAL.birth_placement_store_from_source,
        );
        flush_cell(
            &self.birth_backend_freeze_text_plan_total,
            &GLOBAL.birth_backend_freeze_text_plan_total,
        );
        flush_cell(
            &self.birth_backend_freeze_text_plan_view1,
            &GLOBAL.birth_backend_freeze_text_plan_view1,
        );
        flush_cell(
            &self.birth_backend_freeze_text_plan_pieces2,
            &GLOBAL.birth_backend_freeze_text_plan_pieces2,
        );
        flush_cell(
            &self.birth_backend_freeze_text_plan_pieces3,
            &GLOBAL.birth_backend_freeze_text_plan_pieces3,
        );
        flush_cell(
            &self.birth_backend_freeze_text_plan_pieces4,
            &GLOBAL.birth_backend_freeze_text_plan_pieces4,
        );
        flush_cell(
            &self.birth_backend_freeze_text_plan_owned_tmp,
            &GLOBAL.birth_backend_freeze_text_plan_owned_tmp,
        );
        flush_cell(
            &self.birth_backend_string_box_new_total,
            &GLOBAL.birth_backend_string_box_new_total,
        );
        flush_cell(
            &self.birth_backend_string_box_new_bytes,
            &GLOBAL.birth_backend_string_box_new_bytes,
        );
        flush_cell(
            &self.birth_backend_string_box_ctor_total,
            &GLOBAL.birth_backend_string_box_ctor_total,
        );
        flush_cell(
            &self.birth_backend_string_box_ctor_bytes,
            &GLOBAL.birth_backend_string_box_ctor_bytes,
        );
        flush_cell(
            &self.birth_backend_arc_wrap_total,
            &GLOBAL.birth_backend_arc_wrap_total,
        );
        flush_cell(
            &self.birth_backend_objectize_stable_box_now_total,
            &GLOBAL.birth_backend_objectize_stable_box_now_total,
        );
        flush_cell(
            &self.birth_backend_objectize_stable_box_now_bytes,
            &GLOBAL.birth_backend_objectize_stable_box_now_bytes,
        );
        flush_cell(
            &self.birth_backend_handle_issue_total,
            &GLOBAL.birth_backend_handle_issue_total,
        );
        flush_cell(
            &self.birth_backend_issue_fresh_handle_total,
            &GLOBAL.birth_backend_issue_fresh_handle_total,
        );
        flush_cell(
            &self.birth_backend_materialize_owned_total,
            &GLOBAL.birth_backend_materialize_owned_total,
        );
        flush_cell(
            &self.birth_backend_materialize_owned_bytes,
            &GLOBAL.birth_backend_materialize_owned_bytes,
        );
        flush_cell(
            &self.birth_backend_gc_alloc_called,
            &GLOBAL.birth_backend_gc_alloc_called,
        );
        flush_cell(
            &self.birth_backend_gc_alloc_bytes,
            &GLOBAL.birth_backend_gc_alloc_bytes,
        );
        flush_cell(
            &self.birth_backend_gc_alloc_skipped,
            &GLOBAL.birth_backend_gc_alloc_skipped,
        );
        flush_cell(
            &self.birth_backend_carrier_kind_stable_box,
            &GLOBAL.birth_backend_carrier_kind_stable_box,
        );
        flush_cell(
            &self.birth_backend_carrier_kind_source_keep,
            &GLOBAL.birth_backend_carrier_kind_source_keep,
        );
        flush_cell(
            &self.birth_backend_carrier_kind_owned_bytes,
            &GLOBAL.birth_backend_carrier_kind_owned_bytes,
        );
        flush_cell(
            &self.birth_backend_carrier_kind_handle,
            &GLOBAL.birth_backend_carrier_kind_handle,
        );
        flush_cell(
            &self.birth_backend_publish_reason_external_boundary,
            &GLOBAL.birth_backend_publish_reason_external_boundary,
        );
        flush_cell(
            &self.birth_backend_publish_reason_need_stable_object,
            &GLOBAL.birth_backend_publish_reason_need_stable_object,
        );
        flush_cell(
            &self.birth_backend_publish_reason_generic_fallback,
            &GLOBAL.birth_backend_publish_reason_generic_fallback,
        );
        flush_cell(
            &self.birth_backend_publish_reason_explicit_api,
            &GLOBAL.birth_backend_publish_reason_explicit_api,
        );
        flush_cell(
            &self.birth_backend_publish_boundary_slot_publish_handle_total,
            &GLOBAL.birth_backend_publish_boundary_slot_publish_handle_total,
        );
        flush_cell(
            &self.birth_backend_publish_boundary_slot_objectize_stable_box_total,
            &GLOBAL.birth_backend_publish_boundary_slot_objectize_stable_box_total,
        );
        flush_cell(
            &self.birth_backend_publish_boundary_slot_empty,
            &GLOBAL.birth_backend_publish_boundary_slot_empty,
        );
        flush_cell(
            &self.birth_backend_publish_boundary_slot_already_published,
            &GLOBAL.birth_backend_publish_boundary_slot_already_published,
        );
        flush_cell(
            &self.birth_backend_site_string_concat_hh_materialize_owned_total,
            &GLOBAL.birth_backend_site_string_concat_hh_materialize_owned_total,
        );
        flush_cell(
            &self.birth_backend_site_string_concat_hh_materialize_owned_bytes,
            &GLOBAL.birth_backend_site_string_concat_hh_materialize_owned_bytes,
        );
        flush_cell(
            &self.birth_backend_site_string_concat_hh_objectize_box_total,
            &GLOBAL.birth_backend_site_string_concat_hh_objectize_box_total,
        );
        flush_cell(
            &self.birth_backend_site_string_concat_hh_publish_handle_total,
            &GLOBAL.birth_backend_site_string_concat_hh_publish_handle_total,
        );
        flush_cell(
            &self.birth_backend_site_string_substring_concat_hhii_materialize_owned_total,
            &GLOBAL.birth_backend_site_string_substring_concat_hhii_materialize_owned_total,
        );
        flush_cell(
            &self.birth_backend_site_string_substring_concat_hhii_materialize_owned_bytes,
            &GLOBAL.birth_backend_site_string_substring_concat_hhii_materialize_owned_bytes,
        );
        flush_cell(
            &self.birth_backend_site_string_substring_concat_hhii_objectize_box_total,
            &GLOBAL.birth_backend_site_string_substring_concat_hhii_objectize_box_total,
        );
        flush_cell(
            &self.birth_backend_site_string_substring_concat_hhii_publish_handle_total,
            &GLOBAL.birth_backend_site_string_substring_concat_hhii_publish_handle_total,
        );
        flush_cell(
            &self.birth_backend_site_const_suffix_materialize_owned_total,
            &GLOBAL.birth_backend_site_const_suffix_materialize_owned_total,
        );
        flush_cell(
            &self.birth_backend_site_const_suffix_materialize_owned_bytes,
            &GLOBAL.birth_backend_site_const_suffix_materialize_owned_bytes,
        );
        flush_cell(
            &self.birth_backend_site_const_suffix_objectize_box_total,
            &GLOBAL.birth_backend_site_const_suffix_objectize_box_total,
        );
        flush_cell(
            &self.birth_backend_site_const_suffix_publish_handle_total,
            &GLOBAL.birth_backend_site_const_suffix_publish_handle_total,
        );
        flush_cell(
            &self.birth_backend_site_freeze_text_plan_pieces3_materialize_owned_total,
            &GLOBAL.birth_backend_site_freeze_text_plan_pieces3_materialize_owned_total,
        );
        flush_cell(
            &self.birth_backend_site_freeze_text_plan_pieces3_materialize_owned_bytes,
            &GLOBAL.birth_backend_site_freeze_text_plan_pieces3_materialize_owned_bytes,
        );
        flush_cell(
            &self.birth_backend_site_freeze_text_plan_pieces3_objectize_box_total,
            &GLOBAL.birth_backend_site_freeze_text_plan_pieces3_objectize_box_total,
        );
        flush_cell(
            &self.birth_backend_site_freeze_text_plan_pieces3_publish_handle_total,
            &GLOBAL.birth_backend_site_freeze_text_plan_pieces3_publish_handle_total,
        );
        flush_cell(
            &self.str_concat2_route_total,
            &GLOBAL.str_concat2_route_total,
        );
        flush_cell(
            &self.str_concat2_route_dispatch_hit,
            &GLOBAL.str_concat2_route_dispatch_hit,
        );
        flush_cell(
            &self.str_concat2_route_fast_str_owned,
            &GLOBAL.str_concat2_route_fast_str_owned,
        );
        flush_cell(
            &self.str_concat2_route_fast_str_return_handle,
            &GLOBAL.str_concat2_route_fast_str_return_handle,
        );
        flush_cell(
            &self.str_concat2_route_span_freeze,
            &GLOBAL.str_concat2_route_span_freeze,
        );
        flush_cell(
            &self.str_concat2_route_span_return_handle,
            &GLOBAL.str_concat2_route_span_return_handle,
        );
        flush_cell(
            &self.str_concat2_route_materialize_fallback,
            &GLOBAL.str_concat2_route_materialize_fallback,
        );
        flush_cell(&self.str_len_route_total, &GLOBAL.str_len_route_total);
        flush_cell(
            &self.str_len_route_dispatch_hit,
            &GLOBAL.str_len_route_dispatch_hit,
        );
        flush_cell(
            &self.str_len_route_fast_str_hit,
            &GLOBAL.str_len_route_fast_str_hit,
        );
        flush_cell(
            &self.str_len_route_fallback_hit,
            &GLOBAL.str_len_route_fallback_hit,
        );
        flush_cell(&self.str_len_route_miss, &GLOBAL.str_len_route_miss);
        flush_cell(
            &self.str_len_route_latest_fresh_handle_fast_str_hit,
            &GLOBAL.str_len_route_latest_fresh_handle_fast_str_hit,
        );
        flush_cell(
            &self.str_len_route_latest_fresh_handle_fallback_hit,
            &GLOBAL.str_len_route_latest_fresh_handle_fallback_hit,
        );
        flush_cell(
            &self.str_substring_route_total,
            &GLOBAL.str_substring_route_total,
        );
        flush_cell(
            &self.str_substring_route_view_arc_cache_handle_hit,
            &GLOBAL.str_substring_route_view_arc_cache_handle_hit,
        );
        flush_cell(
            &self.str_substring_route_view_arc_cache_reissue_hit,
            &GLOBAL.str_substring_route_view_arc_cache_reissue_hit,
        );
        flush_cell(
            &self.str_substring_route_view_arc_cache_miss,
            &GLOBAL.str_substring_route_view_arc_cache_miss,
        );
        flush_cell(
            &self.str_substring_route_fast_cache_hit,
            &GLOBAL.str_substring_route_fast_cache_hit,
        );
        flush_cell(
            &self.str_substring_route_dispatch_hit,
            &GLOBAL.str_substring_route_dispatch_hit,
        );
        flush_cell(
            &self.str_substring_route_slow_plan,
            &GLOBAL.str_substring_route_slow_plan,
        );
        flush_cell(
            &self.str_substring_route_slow_plan_return_handle,
            &GLOBAL.str_substring_route_slow_plan_return_handle,
        );
        flush_cell(
            &self.str_substring_route_slow_plan_return_empty,
            &GLOBAL.str_substring_route_slow_plan_return_empty,
        );
        flush_cell(
            &self.str_substring_route_slow_plan_freeze_span,
            &GLOBAL.str_substring_route_slow_plan_freeze_span,
        );
        flush_cell(
            &self.str_substring_route_slow_plan_view_span,
            &GLOBAL.str_substring_route_slow_plan_view_span,
        );
        flush_cell(
            &self.piecewise_subrange_total,
            &GLOBAL.piecewise_subrange_total,
        );
        flush_cell(
            &self.piecewise_subrange_single_session_hit,
            &GLOBAL.piecewise_subrange_single_session_hit,
        );
        flush_cell(
            &self.piecewise_subrange_fallback_insert,
            &GLOBAL.piecewise_subrange_fallback_insert,
        );
        flush_cell(
            &self.piecewise_subrange_empty_return,
            &GLOBAL.piecewise_subrange_empty_return,
        );
        flush_cell(
            &self.piecewise_subrange_prefix_only,
            &GLOBAL.piecewise_subrange_prefix_only,
        );
        flush_cell(
            &self.piecewise_subrange_middle_only,
            &GLOBAL.piecewise_subrange_middle_only,
        );
        flush_cell(
            &self.piecewise_subrange_suffix_only,
            &GLOBAL.piecewise_subrange_suffix_only,
        );
        flush_cell(
            &self.piecewise_subrange_prefix_middle,
            &GLOBAL.piecewise_subrange_prefix_middle,
        );
        flush_cell(
            &self.piecewise_subrange_middle_suffix,
            &GLOBAL.piecewise_subrange_middle_suffix,
        );
        flush_cell(
            &self.piecewise_subrange_prefix_suffix,
            &GLOBAL.piecewise_subrange_prefix_suffix,
        );
        flush_cell(
            &self.piecewise_subrange_all_three,
            &GLOBAL.piecewise_subrange_all_three,
        );
        flush_cell(
            &self.borrowed_alias_to_string_box,
            &GLOBAL.borrowed_alias_to_string_box,
        );
        flush_cell(&self.borrowed_alias_equals, &GLOBAL.borrowed_alias_equals);
        flush_cell(
            &self.borrowed_alias_clone_box,
            &GLOBAL.borrowed_alias_clone_box,
        );
        flush_cell(
            &self.borrowed_alias_to_string_box_latest_fresh,
            &GLOBAL.borrowed_alias_to_string_box_latest_fresh,
        );
        flush_cell(
            &self.borrowed_alias_equals_latest_fresh,
            &GLOBAL.borrowed_alias_equals_latest_fresh,
        );
        flush_cell(
            &self.borrowed_alias_clone_box_latest_fresh,
            &GLOBAL.borrowed_alias_clone_box_latest_fresh,
        );
        flush_cell(
            &self.borrowed_alias_borrowed_source_fast,
            &GLOBAL.borrowed_alias_borrowed_source_fast,
        );
        flush_cell(
            &self.borrowed_alias_as_str_fast,
            &GLOBAL.borrowed_alias_as_str_fast,
        );
        flush_cell(
            &self.borrowed_alias_as_str_fast_live_source,
            &GLOBAL.borrowed_alias_as_str_fast_live_source,
        );
        flush_cell(
            &self.borrowed_alias_as_str_fast_stale_source,
            &GLOBAL.borrowed_alias_as_str_fast_stale_source,
        );
        flush_cell(
            &self.borrowed_alias_array_len_by_index_latest_fresh,
            &GLOBAL.borrowed_alias_array_len_by_index_latest_fresh,
        );
        flush_cell(
            &self.borrowed_alias_array_indexof_by_index_latest_fresh,
            &GLOBAL.borrowed_alias_array_indexof_by_index_latest_fresh,
        );
        flush_cell(
            &self.borrowed_alias_encode_epoch_hit,
            &GLOBAL.borrowed_alias_encode_epoch_hit,
        );
        flush_cell(
            &self.borrowed_alias_encode_ptr_eq_hit,
            &GLOBAL.borrowed_alias_encode_ptr_eq_hit,
        );
        flush_cell(
            &self.borrowed_alias_encode_to_handle_arc,
            &GLOBAL.borrowed_alias_encode_to_handle_arc,
        );
        flush_cell(
            &self.borrowed_alias_encode_to_handle_arc_array_get_index,
            &GLOBAL.borrowed_alias_encode_to_handle_arc_array_get_index,
        );
        flush_cell(
            &self.borrowed_alias_encode_to_handle_arc_map_runtime_data_get_any,
            &GLOBAL.borrowed_alias_encode_to_handle_arc_map_runtime_data_get_any,
        );
    }
}

impl Drop for ThreadCounters {
    fn drop(&mut self) {
        self.flush_into_global();
    }
}

#[inline(always)]
fn flush_cell(local: &Cell<u64>, global: &AtomicU64) {
    let value = local.replace(0);
    if value != 0 {
        global.fetch_add(value, Ordering::Relaxed);
    }
}

thread_local! {
    static TLS_COUNTERS: ThreadCounters = const { ThreadCounters::new() };
}

#[inline(always)]
fn with_tls(f: impl FnOnce(&ThreadCounters)) {
    if config::enabled() {
        TLS_COUNTERS.with(f);
    }
}

fn flush_current_thread() {
    TLS_COUNTERS.with(ThreadCounters::flush_into_global);
}
