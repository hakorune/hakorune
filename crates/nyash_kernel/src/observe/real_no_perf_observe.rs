#![allow(dead_code)]

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CacheProbeKind {
    Hit,
    MissHandle,
    MissDropEpoch,
}

macro_rules! noop_unit_fns0 {
    ($($name:ident),* $(,)?) => {
        $(#[inline(always)] pub(crate) fn $name() {})*
    };
}

macro_rules! noop_unit_fns1 {
    ($($name:ident : $ty:ty),* $(,)?) => {
        $(#[inline(always)] pub(crate) fn $name(_: $ty) {})*
    };
}

macro_rules! noop_bool_fns0 {
    ($($name:ident),* $(,)?) => {
        $(#[inline(always)] pub(crate) fn $name() -> bool { false })*
    };
}

macro_rules! noop_bool_fns1 {
    ($($name:ident : $ty:ty),* $(,)?) => {
        $(#[inline(always)] pub(crate) fn $name(_: $ty) -> bool { false })*
    };
}

noop_bool_fns0!(enabled, bypass_gc_alloc_enabled);
noop_unit_fns1!(record_store_array_str_cache_probe: CacheProbeKind);

noop_unit_fns0!(
    record_store_array_str_enter,
    record_store_array_str_retarget_hit,
    record_store_array_str_latest_fresh_retarget_hit,
    record_store_array_str_source_store,
    record_store_array_str_latest_fresh_source_store,
    record_store_array_str_non_string_source,
    record_store_array_str_existing_slot,
    record_store_array_str_append_slot,
    record_store_array_str_source_string_box,
    record_store_array_str_source_string_view,
    record_store_array_str_source_missing,
    record_store_array_str_plan_source_kind_string_like,
    record_store_array_str_plan_source_kind_other_object,
    record_store_array_str_plan_source_kind_missing,
    record_store_array_str_plan_slot_kind_borrowed_alias,
    record_store_array_str_plan_slot_kind_other,
    record_store_array_str_plan_action_retarget_alias,
    record_store_array_str_plan_action_store_from_source,
    record_store_array_str_plan_action_need_stable_object,
    record_store_array_str_reason_source_kind_via_object,
    record_store_array_str_reason_retarget_keep_source_arc,
    record_store_array_str_reason_retarget_keep_source_arc_ptr_eq_hit,
    record_store_array_str_reason_retarget_keep_source_arc_ptr_eq_miss,
    record_store_array_str_reason_retarget_alias_update,
    record_store_array_str_lookup_registry_slot_read,
    record_store_array_str_lookup_caller_latest_fresh_tag,
    record_store_array_str_update_text_resident_hit,
    record_store_array_str_update_text_resident_miss,
    record_store_array_str_update_text_fallback_hit,
    record_store_array_str_update_text_fallback_miss,
    record_const_suffix_enter,
    record_const_suffix_cached_handle_hit,
    record_const_suffix_text_cache_reload,
    record_const_suffix_freeze_fallback,
    record_const_suffix_empty_return,
    record_const_suffix_cached_fast_str_hit,
    record_const_suffix_cached_span_hit,
    record_birth_placement_return_handle,
    record_birth_placement_borrow_view,
    record_birth_placement_freeze_owned,
    record_birth_placement_fresh_handle,
    record_birth_placement_materialize_owned,
    record_birth_placement_store_from_source,
    record_birth_backend_freeze_text_plan_view1,
    record_birth_backend_freeze_text_plan_pieces2,
    record_birth_backend_freeze_text_plan_pieces3,
    record_birth_backend_freeze_text_plan_pieces4,
    record_birth_backend_freeze_text_plan_owned_tmp,
    record_birth_backend_arc_wrap,
    record_birth_backend_handle_issue,
    record_birth_backend_issue_fresh_handle,
    record_birth_backend_gc_alloc_skipped,
    record_birth_backend_carrier_kind_stable_box,
    record_birth_backend_carrier_kind_source_keep,
    record_birth_backend_carrier_kind_owned_bytes,
    record_birth_backend_carrier_kind_handle,
    record_birth_backend_publish_reason_external_boundary,
    record_birth_backend_publish_reason_need_stable_object,
    record_birth_backend_publish_reason_generic_fallback,
    record_birth_backend_publish_reason_explicit_api,
    record_birth_backend_publish_boundary_slot_publish_handle,
    record_birth_backend_publish_boundary_slot_objectize_stable_box,
    record_birth_backend_publish_boundary_slot_empty,
    record_birth_backend_publish_boundary_slot_already_published,
    record_birth_backend_site_string_concat_hh_objectize_box,
    record_birth_backend_site_string_concat_hh_publish_handle,
    record_birth_backend_site_string_substring_concat_hhii_objectize_box,
    record_birth_backend_site_string_substring_concat_hhii_publish_handle,
    record_birth_backend_site_const_suffix_objectize_box,
    record_birth_backend_site_const_suffix_publish_handle,
    record_birth_backend_site_freeze_text_plan_pieces3_objectize_box,
    record_birth_backend_site_freeze_text_plan_pieces3_publish_handle,
    record_str_concat2_route_enter,
    record_str_concat2_route_dispatch_hit,
    record_str_concat2_route_fast_str_owned,
    record_str_concat2_route_fast_str_return_handle,
    record_str_concat2_route_span_freeze,
    record_str_concat2_route_span_return_handle,
    record_str_concat2_route_materialize_fallback,
    record_str_len_route_enter,
    record_str_len_route_dispatch_hit,
    record_str_len_route_fast_str_hit,
    record_str_len_route_latest_fresh_handle_fast_str_hit,
    record_str_len_route_fallback_hit,
    record_str_len_route_latest_fresh_handle_fallback_hit,
    record_str_len_route_miss,
    record_str_substring_route_enter,
    record_str_substring_route_view_arc_cache_handle_hit,
    record_str_substring_route_view_arc_cache_reissue_hit,
    record_str_substring_route_view_arc_cache_miss,
    record_str_substring_route_fast_cache_hit,
    record_str_substring_route_dispatch_hit,
    record_str_substring_route_slow_plan,
    record_str_substring_route_slow_plan_return_handle,
    record_str_substring_route_slow_plan_return_empty,
    record_str_substring_route_slow_plan_freeze_span,
    record_str_substring_route_slow_plan_view_span,
    record_piecewise_subrange_enter,
    record_piecewise_subrange_single_session_hit,
    record_piecewise_subrange_fallback_insert,
    record_piecewise_subrange_empty_return,
    record_piecewise_subrange_prefix_only,
    record_piecewise_subrange_middle_only,
    record_piecewise_subrange_suffix_only,
    record_piecewise_subrange_prefix_middle,
    record_piecewise_subrange_middle_suffix,
    record_piecewise_subrange_prefix_suffix,
    record_piecewise_subrange_all_three,
    record_borrowed_alias_to_string_box,
    record_borrowed_alias_equals,
    record_borrowed_alias_clone_box,
    record_borrowed_alias_to_string_box_latest_fresh,
    record_borrowed_alias_equals_latest_fresh,
    record_borrowed_alias_clone_box_latest_fresh,
    record_borrowed_alias_borrowed_source_fast,
    record_borrowed_alias_as_str_fast,
    record_borrowed_alias_as_str_fast_live_source,
    record_borrowed_alias_as_str_fast_stale_source,
    record_borrowed_alias_array_len_by_index_latest_fresh,
    record_borrowed_alias_array_indexof_by_index_latest_fresh,
    record_borrowed_alias_encode_live_source_hit,
    record_borrowed_alias_encode_live_source_hit_array_get_index,
    record_borrowed_alias_encode_live_source_hit_map_runtime_data_get_any,
    record_borrowed_alias_encode_epoch_hit,
    record_borrowed_alias_encode_cached_handle_hit,
    record_borrowed_alias_encode_cached_handle_hit_array_get_index,
    record_borrowed_alias_encode_cached_handle_hit_map_runtime_data_get_any,
    record_borrowed_alias_encode_ptr_eq_hit,
    record_borrowed_alias_encode_to_handle_arc,
    record_borrowed_alias_encode_to_handle_arc_array_get_index,
    record_borrowed_alias_encode_to_handle_arc_map_runtime_data_get_any,
);

noop_unit_fns1!(
    record_birth_backend_materialize_owned: usize,
    record_birth_backend_string_box_new: usize,
    record_birth_backend_string_box_ctor: usize,
    record_birth_backend_objectize_stable_box_now: usize,
    record_birth_backend_gc_alloc: usize,
    record_birth_backend_site_string_concat_hh_materialize_owned: usize,
    record_birth_backend_site_string_substring_concat_hhii_materialize_owned: usize,
    record_birth_backend_site_const_suffix_materialize_owned: usize,
    record_birth_backend_site_freeze_text_plan_pieces3_materialize_owned: usize,
    mark_latest_fresh_handle: i64,
);

noop_bool_fns1!(len_route_matches_latest_fresh_handle: i64);

pub(crate) fn flush() {}

// Note: perf-observe is compiled out in this mode, so these helpers intentionally do nothing.
