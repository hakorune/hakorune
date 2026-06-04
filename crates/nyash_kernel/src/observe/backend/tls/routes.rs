macro_rules! tls_bump_total_method {
    ($($name:ident => $field:ident,)+) => {
        $(
            #[inline(always)]
            fn $name(&self) {
                Self::bump(&self.$field);
            }
        )+
    };
}

macro_rules! tls_bump_method {
    ($($name:ident => $field:ident,)+) => {
        $(
            #[inline(always)]
            fn $name(&self) {
                Self::bump(&self.$field);
            }
        )+
    };
}

impl ThreadCounters {
    tls_bump_total_method! {
        str_concat2_route_enter => str_concat2_route_total,
        str_len_route_enter => str_len_route_total,
        str_substring_route_enter => str_substring_route_total,
        piecewise_subrange_enter => piecewise_subrange_total,
    }

    tls_bump_method! {
        str_concat2_route_dispatch_hit => str_concat2_route_dispatch_hit,
        str_concat2_route_fast_str_owned => str_concat2_route_fast_str_owned,
        str_concat2_route_fast_str_return_handle => str_concat2_route_fast_str_return_handle,
        str_concat2_route_span_freeze => str_concat2_route_span_freeze,
        str_concat2_route_span_return_handle => str_concat2_route_span_return_handle,
        str_concat2_route_materialize_fallback => str_concat2_route_materialize_fallback,
        str_len_route_dispatch_hit => str_len_route_dispatch_hit,
        str_len_route_fast_str_hit => str_len_route_fast_str_hit,
        str_len_route_fallback_hit => str_len_route_fallback_hit,
        str_len_route_miss => str_len_route_miss,
        str_len_route_latest_fresh_handle_fast_str_hit => str_len_route_latest_fresh_handle_fast_str_hit,
        str_len_route_latest_fresh_handle_fallback_hit => str_len_route_latest_fresh_handle_fallback_hit,
        str_substring_route_view_arc_cache_handle_hit => str_substring_route_view_arc_cache_handle_hit,
        str_substring_route_view_arc_cache_reissue_hit => str_substring_route_view_arc_cache_reissue_hit,
        str_substring_route_view_arc_cache_miss => str_substring_route_view_arc_cache_miss,
        str_substring_route_fast_cache_hit => str_substring_route_fast_cache_hit,
        str_substring_route_dispatch_hit => str_substring_route_dispatch_hit,
        str_substring_route_slow_plan => str_substring_route_slow_plan,
        str_substring_route_slow_plan_return_handle => str_substring_route_slow_plan_return_handle,
        str_substring_route_slow_plan_return_empty => str_substring_route_slow_plan_return_empty,
        str_substring_route_slow_plan_freeze_span => str_substring_route_slow_plan_freeze_span,
        str_substring_route_slow_plan_view_span => str_substring_route_slow_plan_view_span,
        piecewise_subrange_single_session_hit => piecewise_subrange_single_session_hit,
        piecewise_subrange_fallback_insert => piecewise_subrange_fallback_insert,
        piecewise_subrange_empty_return => piecewise_subrange_empty_return,
        piecewise_subrange_prefix_only => piecewise_subrange_prefix_only,
        piecewise_subrange_middle_only => piecewise_subrange_middle_only,
        piecewise_subrange_suffix_only => piecewise_subrange_suffix_only,
        piecewise_subrange_prefix_middle => piecewise_subrange_prefix_middle,
        piecewise_subrange_middle_suffix => piecewise_subrange_middle_suffix,
        piecewise_subrange_prefix_suffix => piecewise_subrange_prefix_suffix,
        piecewise_subrange_all_three => piecewise_subrange_all_three,
        borrowed_alias_to_string_box => borrowed_alias_to_string_box,
        borrowed_alias_equals => borrowed_alias_equals,
        borrowed_alias_clone_box => borrowed_alias_clone_box,
        borrowed_alias_to_string_box_latest_fresh => borrowed_alias_to_string_box_latest_fresh,
        borrowed_alias_equals_latest_fresh => borrowed_alias_equals_latest_fresh,
        borrowed_alias_clone_box_latest_fresh => borrowed_alias_clone_box_latest_fresh,
        borrowed_alias_borrowed_source_fast => borrowed_alias_borrowed_source_fast,
        borrowed_alias_as_str_fast => borrowed_alias_as_str_fast,
        borrowed_alias_as_str_fast_live_source => borrowed_alias_as_str_fast_live_source,
        borrowed_alias_as_str_fast_stale_source => borrowed_alias_as_str_fast_stale_source,
        borrowed_alias_array_len_by_index_latest_fresh => borrowed_alias_array_len_by_index_latest_fresh,
        borrowed_alias_array_indexof_by_index_latest_fresh => borrowed_alias_array_indexof_by_index_latest_fresh,
        borrowed_alias_encode_live_source_hit => borrowed_alias_encode_live_source_hit,
        borrowed_alias_encode_live_source_hit_array_get_index => borrowed_alias_encode_live_source_hit_array_get_index,
        borrowed_alias_encode_live_source_hit_map_runtime_data_get_any => borrowed_alias_encode_live_source_hit_map_runtime_data_get_any,
        borrowed_alias_encode_epoch_hit => borrowed_alias_encode_epoch_hit,
        borrowed_alias_encode_cached_handle_hit => borrowed_alias_encode_cached_handle_hit,
        borrowed_alias_encode_cached_handle_hit_array_get_index => borrowed_alias_encode_cached_handle_hit_array_get_index,
        borrowed_alias_encode_cached_handle_hit_map_runtime_data_get_any => borrowed_alias_encode_cached_handle_hit_map_runtime_data_get_any,
        borrowed_alias_encode_ptr_eq_hit => borrowed_alias_encode_ptr_eq_hit,
        borrowed_alias_encode_to_handle_arc => borrowed_alias_encode_to_handle_arc,
        borrowed_alias_encode_to_handle_arc_array_get_index => borrowed_alias_encode_to_handle_arc_array_get_index,
        borrowed_alias_encode_to_handle_arc_map_runtime_data_get_any => borrowed_alias_encode_to_handle_arc_map_runtime_data_get_any,
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
