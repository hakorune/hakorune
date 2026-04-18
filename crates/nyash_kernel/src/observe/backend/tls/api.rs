macro_rules! tls_zero_arg_api {
    ($($name:ident => $method:ident,)+) => {
        $(
            #[inline(always)]
            pub(crate) fn $name() {
                with_tls(ThreadCounters::$method);
            }
        )+
    };
}

tls_zero_arg_api! {
    store_array_str_enter => store_array_str_enter,
    store_array_str_retarget_hit => store_array_str_retarget_hit,
    store_array_str_latest_fresh_retarget_hit => store_array_str_latest_fresh_retarget_hit,
    store_array_str_source_store => store_array_str_source_store,
    store_array_str_latest_fresh_source_store => store_array_str_latest_fresh_source_store,
    store_array_str_non_string_source => store_array_str_non_string_source,
    store_array_str_existing_slot => store_array_str_existing_slot,
    store_array_str_append_slot => store_array_str_append_slot,
    store_array_str_source_string_box => store_array_str_source_string_box,
    store_array_str_source_string_view => store_array_str_source_string_view,
    store_array_str_source_missing => store_array_str_source_missing,
    store_array_str_plan_source_kind_string_like => store_array_str_plan_source_kind_string_like,
    store_array_str_plan_source_kind_other_object => store_array_str_plan_source_kind_other_object,
    store_array_str_plan_source_kind_missing => store_array_str_plan_source_kind_missing,
    store_array_str_plan_slot_kind_borrowed_alias => store_array_str_plan_slot_kind_borrowed_alias,
    store_array_str_plan_slot_kind_other => store_array_str_plan_slot_kind_other,
    store_array_str_plan_action_retarget_alias => store_array_str_plan_action_retarget_alias,
    store_array_str_plan_action_store_from_source => store_array_str_plan_action_store_from_source,
    store_array_str_plan_action_need_stable_object => store_array_str_plan_action_need_stable_object,
    store_array_str_reason_source_kind_via_object => store_array_str_reason_source_kind_via_object,
    store_array_str_reason_retarget_keep_source_arc => store_array_str_reason_retarget_keep_source_arc,
    store_array_str_reason_retarget_keep_source_arc_ptr_eq_hit => store_array_str_reason_retarget_keep_source_arc_ptr_eq_hit,
    store_array_str_reason_retarget_keep_source_arc_ptr_eq_miss => store_array_str_reason_retarget_keep_source_arc_ptr_eq_miss,
    store_array_str_reason_retarget_alias_update => store_array_str_reason_retarget_alias_update,
    store_array_str_lookup_registry_slot_read => store_array_str_lookup_registry_slot_read,
    store_array_str_lookup_caller_latest_fresh_tag => store_array_str_lookup_caller_latest_fresh_tag,
    const_suffix_enter => const_suffix_enter,
    const_suffix_cached_handle_hit => const_suffix_cached_handle_hit,
    const_suffix_text_cache_reload => const_suffix_text_cache_reload,
    const_suffix_freeze_fallback => const_suffix_freeze_fallback,
    const_suffix_empty_return => const_suffix_empty_return,
    const_suffix_cached_fast_str_hit => const_suffix_cached_fast_str_hit,
    const_suffix_cached_span_hit => const_suffix_cached_span_hit,
    birth_placement_return_handle => birth_placement_return_handle,
    birth_placement_borrow_view => birth_placement_borrow_view,
    birth_placement_freeze_owned => birth_placement_freeze_owned,
    birth_placement_fresh_handle => birth_placement_fresh_handle,
    birth_placement_materialize_owned => birth_placement_materialize_owned,
    birth_placement_store_from_source => birth_placement_store_from_source,
    birth_backend_freeze_text_plan_view1 => birth_backend_freeze_text_plan_view1,
    birth_backend_freeze_text_plan_pieces2 => birth_backend_freeze_text_plan_pieces2,
    birth_backend_freeze_text_plan_pieces3 => birth_backend_freeze_text_plan_pieces3,
    birth_backend_freeze_text_plan_pieces4 => birth_backend_freeze_text_plan_pieces4,
    birth_backend_freeze_text_plan_owned_tmp => birth_backend_freeze_text_plan_owned_tmp,
    birth_backend_arc_wrap => birth_backend_arc_wrap,
    birth_backend_handle_issue => birth_backend_handle_issue,
    birth_backend_issue_fresh_handle => birth_backend_issue_fresh_handle,
    birth_backend_gc_alloc_skipped => birth_backend_gc_alloc_skipped,
    birth_backend_carrier_kind_stable_box => birth_backend_carrier_kind_stable_box,
    birth_backend_carrier_kind_source_keep => birth_backend_carrier_kind_source_keep,
    birth_backend_carrier_kind_owned_bytes => birth_backend_carrier_kind_owned_bytes,
    birth_backend_carrier_kind_handle => birth_backend_carrier_kind_handle,
    birth_backend_publish_reason_external_boundary => birth_backend_publish_reason_external_boundary,
    birth_backend_publish_reason_need_stable_object => birth_backend_publish_reason_need_stable_object,
    birth_backend_publish_reason_generic_fallback => birth_backend_publish_reason_generic_fallback,
    birth_backend_publish_reason_explicit_api => birth_backend_publish_reason_explicit_api,
    birth_backend_publish_boundary_slot_publish_handle => birth_backend_publish_boundary_slot_publish_handle,
    birth_backend_publish_boundary_slot_objectize_stable_box => birth_backend_publish_boundary_slot_objectize_stable_box,
    birth_backend_publish_boundary_slot_empty => birth_backend_publish_boundary_slot_empty,
    birth_backend_publish_boundary_slot_already_published => birth_backend_publish_boundary_slot_already_published,
    str_concat2_route_enter => str_concat2_route_enter,
    str_concat2_route_dispatch_hit => str_concat2_route_dispatch_hit,
    str_concat2_route_fast_str_owned => str_concat2_route_fast_str_owned,
    str_concat2_route_fast_str_return_handle => str_concat2_route_fast_str_return_handle,
    str_concat2_route_span_freeze => str_concat2_route_span_freeze,
    str_concat2_route_span_return_handle => str_concat2_route_span_return_handle,
    str_concat2_route_materialize_fallback => str_concat2_route_materialize_fallback,
    str_len_route_enter => str_len_route_enter,
    str_len_route_dispatch_hit => str_len_route_dispatch_hit,
    str_len_route_fast_str_hit => str_len_route_fast_str_hit,
    str_len_route_fallback_hit => str_len_route_fallback_hit,
    str_len_route_miss => str_len_route_miss,
    str_len_route_latest_fresh_handle_fast_str_hit => str_len_route_latest_fresh_handle_fast_str_hit,
    str_len_route_latest_fresh_handle_fallback_hit => str_len_route_latest_fresh_handle_fallback_hit,
    str_substring_route_enter => str_substring_route_enter,
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
    piecewise_subrange_enter => piecewise_subrange_enter,
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
    borrowed_alias_encode_epoch_hit => borrowed_alias_encode_epoch_hit,
    borrowed_alias_encode_cached_handle_hit => borrowed_alias_encode_cached_handle_hit,
    borrowed_alias_encode_ptr_eq_hit => borrowed_alias_encode_ptr_eq_hit,
    borrowed_alias_encode_to_handle_arc => borrowed_alias_encode_to_handle_arc,
    borrowed_alias_encode_to_handle_arc_array_get_index => borrowed_alias_encode_to_handle_arc_array_get_index,
    borrowed_alias_encode_to_handle_arc_map_runtime_data_get_any => borrowed_alias_encode_to_handle_arc_map_runtime_data_get_any,
}

#[inline(always)]
pub(crate) fn store_array_str_cache_probe(kind: CacheProbeKind) {
    with_tls(|tls| tls.store_array_str_cache_probe(kind));
}

#[inline(always)]
pub(crate) fn birth_backend_materialize_owned(bytes: u64) {
    with_tls(|tls| tls.birth_backend_materialize_owned(bytes));
}

#[inline(always)]
pub(crate) fn birth_backend_string_box_new(bytes: u64) {
    with_tls(|tls| tls.birth_backend_string_box_new(bytes));
}

#[inline(always)]
pub(crate) fn birth_backend_string_box_ctor(bytes: u64) {
    with_tls(|tls| tls.birth_backend_string_box_ctor(bytes));
}

#[inline(always)]
pub(crate) fn birth_backend_objectize_stable_box_now(bytes: u64) {
    with_tls(|tls| tls.birth_backend_objectize_stable_box_now(bytes));
}

#[inline(always)]
pub(crate) fn birth_backend_gc_alloc(bytes: u64) {
    with_tls(|tls| tls.birth_backend_gc_alloc(bytes));
}

#[inline(always)]
pub(crate) fn birth_backend_site_string_concat_hh_materialize_owned(bytes: u64) {
    with_tls(|tls| tls.birth_backend_site_string_concat_hh_materialize_owned(bytes));
}

#[inline(always)]
pub(crate) fn birth_backend_site_string_concat_hh_objectize_box() {
    with_tls(ThreadCounters::birth_backend_site_string_concat_hh_objectize_box);
}

#[inline(always)]
pub(crate) fn birth_backend_site_string_concat_hh_publish_handle() {
    with_tls(ThreadCounters::birth_backend_site_string_concat_hh_publish_handle);
}

#[inline(always)]
pub(crate) fn birth_backend_site_string_substring_concat_hhii_materialize_owned(bytes: u64) {
    with_tls(|tls| tls.birth_backend_site_string_substring_concat_hhii_materialize_owned(bytes));
}

#[inline(always)]
pub(crate) fn birth_backend_site_string_substring_concat_hhii_objectize_box() {
    with_tls(ThreadCounters::birth_backend_site_string_substring_concat_hhii_objectize_box);
}

#[inline(always)]
pub(crate) fn birth_backend_site_string_substring_concat_hhii_publish_handle() {
    with_tls(ThreadCounters::birth_backend_site_string_substring_concat_hhii_publish_handle);
}

#[inline(always)]
pub(crate) fn birth_backend_site_const_suffix_materialize_owned(bytes: u64) {
    with_tls(|tls| tls.birth_backend_site_const_suffix_materialize_owned(bytes));
}

#[inline(always)]
pub(crate) fn birth_backend_site_const_suffix_objectize_box() {
    with_tls(ThreadCounters::birth_backend_site_const_suffix_objectize_box);
}

#[inline(always)]
pub(crate) fn birth_backend_site_const_suffix_publish_handle() {
    with_tls(ThreadCounters::birth_backend_site_const_suffix_publish_handle);
}

#[inline(always)]
pub(crate) fn birth_backend_site_freeze_text_plan_pieces3_materialize_owned(bytes: u64) {
    with_tls(|tls| tls.birth_backend_site_freeze_text_plan_pieces3_materialize_owned(bytes));
}

#[inline(always)]
pub(crate) fn birth_backend_site_freeze_text_plan_pieces3_objectize_box() {
    with_tls(ThreadCounters::birth_backend_site_freeze_text_plan_pieces3_objectize_box);
}

#[inline(always)]
pub(crate) fn birth_backend_site_freeze_text_plan_pieces3_publish_handle() {
    with_tls(ThreadCounters::birth_backend_site_freeze_text_plan_pieces3_publish_handle);
}

#[inline(always)]
pub(crate) fn mark_latest_fresh_handle(handle: i64) {
    with_tls(|tls| tls.mark_latest_fresh_handle(handle));
}

#[inline(always)]
pub(crate) fn matches_latest_fresh_handle(handle: i64) -> bool {
    TLS_COUNTERS.with(|tls| tls.matches_latest_fresh_handle(handle))
}

macro_rules! load {
    ($field:ident) => {
        GLOBAL.$field.load(Ordering::Relaxed)
    };
}

pub(crate) fn snapshot() -> [u64; 144] {
    flush_current_thread();
    [
        load!(store_array_str_total),
        load!(store_array_str_cache_hit),
        load!(store_array_str_cache_miss_handle),
        load!(store_array_str_cache_miss_epoch),
        load!(store_array_str_retarget_hit),
        load!(store_array_str_latest_fresh_retarget_hit),
        load!(store_array_str_source_store),
        load!(store_array_str_latest_fresh_source_store),
        load!(store_array_str_non_string_source),
        load!(store_array_str_existing_slot),
        load!(store_array_str_append_slot),
        load!(store_array_str_source_string_box),
        load!(store_array_str_source_string_view),
        load!(store_array_str_source_missing),
        load!(const_suffix_total),
        load!(const_suffix_cached_handle_hit),
        load!(const_suffix_text_cache_reload),
        load!(const_suffix_freeze_fallback),
        load!(const_suffix_empty_return),
        load!(const_suffix_cached_fast_str_hit),
        load!(const_suffix_cached_span_hit),
        load!(birth_placement_return_handle),
        load!(birth_placement_borrow_view),
        load!(birth_placement_freeze_owned),
        load!(birth_placement_fresh_handle),
        load!(birth_placement_materialize_owned),
        load!(birth_placement_store_from_source),
        load!(birth_backend_freeze_text_plan_total),
        load!(birth_backend_freeze_text_plan_view1),
        load!(birth_backend_freeze_text_plan_pieces2),
        load!(birth_backend_freeze_text_plan_pieces3),
        load!(birth_backend_freeze_text_plan_pieces4),
        load!(birth_backend_freeze_text_plan_owned_tmp),
        load!(birth_backend_string_box_new_total),
        load!(birth_backend_string_box_new_bytes),
        load!(birth_backend_string_box_ctor_total),
        load!(birth_backend_string_box_ctor_bytes),
        load!(birth_backend_arc_wrap_total),
        load!(birth_backend_objectize_stable_box_now_total),
        load!(birth_backend_objectize_stable_box_now_bytes),
        load!(birth_backend_handle_issue_total),
        load!(birth_backend_issue_fresh_handle_total),
        load!(birth_backend_materialize_owned_total),
        load!(birth_backend_materialize_owned_bytes),
        load!(birth_backend_gc_alloc_called),
        load!(birth_backend_gc_alloc_bytes),
        load!(birth_backend_gc_alloc_skipped),
        load!(str_concat2_route_total),
        load!(str_concat2_route_dispatch_hit),
        load!(str_concat2_route_fast_str_owned),
        load!(str_concat2_route_fast_str_return_handle),
        load!(str_concat2_route_span_freeze),
        load!(str_concat2_route_span_return_handle),
        load!(str_concat2_route_materialize_fallback),
        load!(str_len_route_total),
        load!(str_len_route_dispatch_hit),
        load!(str_len_route_fast_str_hit),
        load!(str_len_route_fallback_hit),
        load!(str_len_route_miss),
        load!(str_len_route_latest_fresh_handle_fast_str_hit),
        load!(str_len_route_latest_fresh_handle_fallback_hit),
        load!(str_substring_route_total),
        load!(str_substring_route_view_arc_cache_handle_hit),
        load!(str_substring_route_view_arc_cache_reissue_hit),
        load!(str_substring_route_view_arc_cache_miss),
        load!(str_substring_route_fast_cache_hit),
        load!(str_substring_route_dispatch_hit),
        load!(str_substring_route_slow_plan),
        load!(str_substring_route_slow_plan_return_handle),
        load!(str_substring_route_slow_plan_return_empty),
        load!(str_substring_route_slow_plan_freeze_span),
        load!(str_substring_route_slow_plan_view_span),
        load!(borrowed_alias_to_string_box),
        load!(borrowed_alias_equals),
        load!(borrowed_alias_clone_box),
        load!(borrowed_alias_to_string_box_latest_fresh),
        load!(borrowed_alias_equals_latest_fresh),
        load!(borrowed_alias_clone_box_latest_fresh),
        load!(borrowed_alias_borrowed_source_fast),
        load!(borrowed_alias_as_str_fast),
        load!(borrowed_alias_as_str_fast_live_source),
        load!(borrowed_alias_as_str_fast_stale_source),
        load!(borrowed_alias_array_len_by_index_latest_fresh),
        load!(borrowed_alias_array_indexof_by_index_latest_fresh),
        load!(borrowed_alias_encode_epoch_hit),
        load!(borrowed_alias_encode_ptr_eq_hit),
        load!(borrowed_alias_encode_to_handle_arc),
        load!(borrowed_alias_encode_to_handle_arc_array_get_index),
        load!(borrowed_alias_encode_to_handle_arc_map_runtime_data_get_any),
        load!(store_array_str_plan_source_kind_string_like),
        load!(store_array_str_plan_source_kind_other_object),
        load!(store_array_str_plan_source_kind_missing),
        load!(store_array_str_plan_slot_kind_borrowed_alias),
        load!(store_array_str_plan_slot_kind_other),
        load!(store_array_str_plan_action_retarget_alias),
        load!(store_array_str_plan_action_store_from_source),
        load!(store_array_str_plan_action_need_stable_object),
        load!(store_array_str_reason_source_kind_via_object),
        load!(store_array_str_reason_retarget_keep_source_arc),
        load!(store_array_str_reason_retarget_keep_source_arc_ptr_eq_hit),
        load!(store_array_str_reason_retarget_keep_source_arc_ptr_eq_miss),
        load!(store_array_str_reason_retarget_alias_update),
        load!(piecewise_subrange_total),
        load!(piecewise_subrange_single_session_hit),
        load!(piecewise_subrange_fallback_insert),
        load!(piecewise_subrange_empty_return),
        load!(piecewise_subrange_prefix_only),
        load!(piecewise_subrange_middle_only),
        load!(piecewise_subrange_suffix_only),
        load!(piecewise_subrange_prefix_middle),
        load!(piecewise_subrange_middle_suffix),
        load!(piecewise_subrange_prefix_suffix),
        load!(piecewise_subrange_all_three),
        load!(birth_backend_carrier_kind_stable_box),
        load!(birth_backend_carrier_kind_source_keep),
        load!(birth_backend_carrier_kind_owned_bytes),
        load!(birth_backend_carrier_kind_handle),
        load!(birth_backend_publish_reason_external_boundary),
        load!(birth_backend_publish_reason_need_stable_object),
        load!(birth_backend_publish_reason_generic_fallback),
        load!(birth_backend_publish_reason_explicit_api),
        load!(store_array_str_lookup_registry_slot_read),
        load!(store_array_str_lookup_caller_latest_fresh_tag),
        load!(birth_backend_site_string_concat_hh_materialize_owned_total),
        load!(birth_backend_site_string_concat_hh_materialize_owned_bytes),
        load!(birth_backend_site_string_concat_hh_objectize_box_total),
        load!(birth_backend_site_string_concat_hh_publish_handle_total),
        load!(birth_backend_site_string_substring_concat_hhii_materialize_owned_total),
        load!(birth_backend_site_string_substring_concat_hhii_materialize_owned_bytes),
        load!(birth_backend_site_string_substring_concat_hhii_objectize_box_total),
        load!(birth_backend_site_string_substring_concat_hhii_publish_handle_total),
        load!(birth_backend_site_const_suffix_materialize_owned_total),
        load!(birth_backend_site_const_suffix_materialize_owned_bytes),
        load!(birth_backend_site_const_suffix_objectize_box_total),
        load!(birth_backend_site_const_suffix_publish_handle_total),
        load!(birth_backend_site_freeze_text_plan_pieces3_materialize_owned_total),
        load!(birth_backend_site_freeze_text_plan_pieces3_materialize_owned_bytes),
        load!(birth_backend_site_freeze_text_plan_pieces3_objectize_box_total),
        load!(birth_backend_site_freeze_text_plan_pieces3_publish_handle_total),
        load!(birth_backend_publish_boundary_slot_publish_handle_total),
        load!(birth_backend_publish_boundary_slot_objectize_stable_box_total),
        load!(birth_backend_publish_boundary_slot_empty),
        load!(birth_backend_publish_boundary_slot_already_published),
        load!(borrowed_alias_encode_cached_handle_hit),
    ]
}
