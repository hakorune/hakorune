impl ThreadCounters {
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
    fn borrowed_alias_encode_live_source_hit(&self) {
        Self::bump(&self.borrowed_alias_encode_live_source_hit);
    }

    #[inline(always)]
    fn borrowed_alias_encode_live_source_hit_array_get_index(&self) {
        Self::bump(&self.borrowed_alias_encode_live_source_hit_array_get_index);
    }

    #[inline(always)]
    fn borrowed_alias_encode_live_source_hit_map_runtime_data_get_any(&self) {
        Self::bump(&self.borrowed_alias_encode_live_source_hit_map_runtime_data_get_any);
    }

    #[inline(always)]
    fn borrowed_alias_encode_epoch_hit(&self) {
        Self::bump(&self.borrowed_alias_encode_epoch_hit);
    }

    #[inline(always)]
    fn borrowed_alias_encode_cached_handle_hit(&self) {
        Self::bump(&self.borrowed_alias_encode_cached_handle_hit);
    }

    #[inline(always)]
    fn borrowed_alias_encode_cached_handle_hit_array_get_index(&self) {
        Self::bump(&self.borrowed_alias_encode_cached_handle_hit_array_get_index);
    }

    #[inline(always)]
    fn borrowed_alias_encode_cached_handle_hit_map_runtime_data_get_any(&self) {
        Self::bump(&self.borrowed_alias_encode_cached_handle_hit_map_runtime_data_get_any);
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
}
