pub(crate) const STORE_ARRAY_STR: &str = "store.array.str";
pub(crate) const CONST_SUFFIX: &str = "const_suffix";
pub(crate) const BIRTH_PLACEMENT: &str = "birth.placement";
pub(crate) const BIRTH_BACKEND: &str = "birth.backend";
pub(crate) const STR_CONCAT2_ROUTE: &str = "str.concat2.route";
pub(crate) const STR_LEN_ROUTE: &str = "str.len.route";
pub(crate) const STR_SUBSTRING_ROUTE: &str = "str.substring.route";
pub(crate) const PIECEWISE_SUBRANGE: &str = "piecewise_subrange";
pub(crate) const STABLE_BOX_DEMAND: &str = "stable_box_demand";
pub(crate) const BORROWED_ALIAS: &str = "borrowed.alias";

pub(crate) const STORE_ARRAY_STR_CACHE_HIT: &str = "cache_hit";
pub(crate) const STORE_ARRAY_STR_CACHE_MISS_HANDLE: &str = "cache_miss_handle";
pub(crate) const STORE_ARRAY_STR_CACHE_MISS_EPOCH: &str = "cache_miss_epoch";
pub(crate) const STORE_ARRAY_STR_RETARGET_HIT: &str = "retarget_hit";
pub(crate) const STORE_ARRAY_STR_SOURCE_STORE: &str = "source_store";
pub(crate) const STORE_ARRAY_STR_NON_STRING_SOURCE: &str = "non_string_source";
pub(crate) const STORE_ARRAY_STR_LATEST_FRESH_RETARGET_HIT: &str = "latest_fresh_retarget_hit";
pub(crate) const STORE_ARRAY_STR_LATEST_FRESH_SOURCE_STORE: &str = "latest_fresh_source_store";
pub(crate) const STORE_ARRAY_STR_EXISTING_SLOT: &str = "existing_slot";
pub(crate) const STORE_ARRAY_STR_APPEND_SLOT: &str = "append_slot";
pub(crate) const STORE_ARRAY_STR_SOURCE_STRING_BOX: &str = "source_string_box";
pub(crate) const STORE_ARRAY_STR_SOURCE_STRING_VIEW: &str = "source_string_view";
pub(crate) const STORE_ARRAY_STR_SOURCE_MISSING: &str = "source_missing";
pub(crate) const STORE_ARRAY_STR_PLAN_SOURCE_KIND_STRING_LIKE: &str =
    "plan.source_kind_string_like";
pub(crate) const STORE_ARRAY_STR_PLAN_SOURCE_KIND_OTHER_OBJECT: &str =
    "plan.source_kind_other_object";
pub(crate) const STORE_ARRAY_STR_PLAN_SOURCE_KIND_MISSING: &str = "plan.source_kind_missing";
pub(crate) const STORE_ARRAY_STR_PLAN_SLOT_KIND_BORROWED_ALIAS: &str =
    "plan.slot_kind_borrowed_alias";
pub(crate) const STORE_ARRAY_STR_PLAN_SLOT_KIND_OTHER: &str = "plan.slot_kind_other";
pub(crate) const STORE_ARRAY_STR_PLAN_ACTION_RETARGET_ALIAS: &str = "plan.action_retarget_alias";
pub(crate) const STORE_ARRAY_STR_PLAN_ACTION_STORE_FROM_SOURCE: &str =
    "plan.action_store_from_source";
pub(crate) const STORE_ARRAY_STR_PLAN_ACTION_NEED_STABLE_OBJECT: &str =
    "plan.action_need_stable_object";
pub(crate) const STORE_ARRAY_STR_REASON_SOURCE_KIND_VIA_OBJECT: &str =
    "reason.source_kind_via_object";
pub(crate) const STORE_ARRAY_STR_REASON_RETARGET_KEEP_SOURCE_ARC: &str =
    "reason.retarget_keep_source_arc";
pub(crate) const STORE_ARRAY_STR_REASON_RETARGET_KEEP_SOURCE_ARC_PTR_EQ_HIT: &str =
    "reason.retarget_keep_source_arc_ptr_eq_hit";
pub(crate) const STORE_ARRAY_STR_REASON_RETARGET_KEEP_SOURCE_ARC_PTR_EQ_MISS: &str =
    "reason.retarget_keep_source_arc_ptr_eq_miss";
pub(crate) const STORE_ARRAY_STR_REASON_RETARGET_ALIAS_UPDATE: &str =
    "reason.retarget_alias_update";
pub(crate) const STORE_ARRAY_STR_LOOKUP_REGISTRY_SLOT_READ: &str = "lookup.registry_slot_read";
pub(crate) const STORE_ARRAY_STR_LOOKUP_CALLER_LATEST_FRESH_TAG: &str =
    "lookup.caller_latest_fresh_tag";

pub(crate) const CONST_SUFFIX_CACHED_HANDLE_HIT: &str = "cached_handle_hit";
pub(crate) const CONST_SUFFIX_TEXT_CACHE_RELOAD: &str = "text_cache_reload";
pub(crate) const CONST_SUFFIX_FREEZE_FALLBACK: &str = "freeze_fallback";
pub(crate) const CONST_SUFFIX_EMPTY_RETURN: &str = "empty_return";
pub(crate) const CONST_SUFFIX_CACHED_FAST_STR_HIT: &str = "cached_fast_str_hit";
pub(crate) const CONST_SUFFIX_CACHED_SPAN_HIT: &str = "cached_span_hit";

pub(crate) const BIRTH_PLACEMENT_RETURN_HANDLE: &str = "return_handle";
pub(crate) const BIRTH_PLACEMENT_BORROW_VIEW: &str = "borrow_view";
pub(crate) const BIRTH_PLACEMENT_FREEZE_OWNED: &str = "freeze_owned";
pub(crate) const BIRTH_PLACEMENT_FRESH_HANDLE: &str = "fresh_handle";
pub(crate) const BIRTH_PLACEMENT_MATERIALIZE_OWNED: &str = "materialize_owned";
pub(crate) const BIRTH_PLACEMENT_STORE_FROM_SOURCE: &str = "store_from_source";

pub(crate) const BIRTH_BACKEND_FREEZE_TEXT_PLAN_TOTAL: &str = "freeze_text_plan_total";
pub(crate) const BIRTH_BACKEND_FREEZE_TEXT_PLAN_VIEW1: &str = "freeze_text_plan_view1";
pub(crate) const BIRTH_BACKEND_FREEZE_TEXT_PLAN_PIECES2: &str = "freeze_text_plan_pieces2";
pub(crate) const BIRTH_BACKEND_FREEZE_TEXT_PLAN_PIECES3: &str = "freeze_text_plan_pieces3";
pub(crate) const BIRTH_BACKEND_FREEZE_TEXT_PLAN_PIECES4: &str = "freeze_text_plan_pieces4";
pub(crate) const BIRTH_BACKEND_FREEZE_TEXT_PLAN_OWNED_TMP: &str = "freeze_text_plan_owned_tmp";
pub(crate) const BIRTH_BACKEND_STRING_BOX_NEW_TOTAL: &str = "string_box_new_total";
pub(crate) const BIRTH_BACKEND_STRING_BOX_NEW_BYTES: &str = "string_box_new_bytes";
pub(crate) const BIRTH_BACKEND_STRING_BOX_CTOR_TOTAL: &str = "string_box_ctor_total";
pub(crate) const BIRTH_BACKEND_STRING_BOX_CTOR_BYTES: &str = "string_box_ctor_bytes";
pub(crate) const BIRTH_BACKEND_ARC_WRAP_TOTAL: &str = "arc_wrap_total";
pub(crate) const BIRTH_BACKEND_HANDLE_ISSUE_TOTAL: &str = "handle_issue_total";
pub(crate) const BIRTH_BACKEND_OBJECTIZE_STABLE_BOX_NOW_TOTAL: &str =
    "objectize_stable_box_now_total";
pub(crate) const BIRTH_BACKEND_OBJECTIZE_STABLE_BOX_NOW_BYTES: &str =
    "objectize_stable_box_now_bytes";
pub(crate) const BIRTH_BACKEND_ISSUE_FRESH_HANDLE_TOTAL: &str = "issue_fresh_handle_total";
pub(crate) const BIRTH_BACKEND_MATERIALIZE_OWNED_TOTAL: &str = "materialize_owned_total";
pub(crate) const BIRTH_BACKEND_MATERIALIZE_OWNED_BYTES: &str = "materialize_owned_bytes";
pub(crate) const BIRTH_BACKEND_GC_ALLOC_CALLED: &str = "gc_alloc_called";
pub(crate) const BIRTH_BACKEND_GC_ALLOC_BYTES: &str = "gc_alloc_bytes";
pub(crate) const BIRTH_BACKEND_GC_ALLOC_SKIPPED: &str = "gc_alloc_skipped";
pub(crate) const BIRTH_BACKEND_CARRIER_KIND_STABLE_BOX: &str = "carrier_kind.stable_box";
pub(crate) const BIRTH_BACKEND_CARRIER_KIND_SOURCE_KEEP: &str = "carrier_kind.source_keep";
pub(crate) const BIRTH_BACKEND_CARRIER_KIND_OWNED_BYTES: &str = "carrier_kind.owned_bytes";
pub(crate) const BIRTH_BACKEND_CARRIER_KIND_HANDLE: &str = "carrier_kind.handle";
pub(crate) const BIRTH_BACKEND_PUBLISH_REASON_EXTERNAL_BOUNDARY: &str =
    "publish_reason.external_boundary";
pub(crate) const BIRTH_BACKEND_PUBLISH_REASON_NEED_STABLE_OBJECT: &str =
    "publish_reason.need_stable_object";
pub(crate) const BIRTH_BACKEND_PUBLISH_REASON_GENERIC_FALLBACK: &str =
    "publish_reason.generic_fallback";
pub(crate) const BIRTH_BACKEND_PUBLISH_REASON_EXPLICIT_API: &str = "publish_reason.explicit_api";
pub(crate) const BIRTH_BACKEND_PUBLISH_BOUNDARY_SLOT_PUBLISH_HANDLE_TOTAL: &str =
    "publish_boundary.slot_publish_handle_total";
pub(crate) const BIRTH_BACKEND_PUBLISH_BOUNDARY_SLOT_OBJECTIZE_STABLE_BOX_TOTAL: &str =
    "publish_boundary.slot_objectize_stable_box_total";
pub(crate) const BIRTH_BACKEND_PUBLISH_BOUNDARY_SLOT_EMPTY: &str = "publish_boundary.slot_empty";
pub(crate) const BIRTH_BACKEND_PUBLISH_BOUNDARY_SLOT_ALREADY_PUBLISHED: &str =
    "publish_boundary.slot_already_published";
pub(crate) const BIRTH_BACKEND_SITE_STRING_CONCAT_HH_MATERIALIZE_OWNED_TOTAL: &str =
    "site.string_concat_hh.materialize_owned_total";
pub(crate) const BIRTH_BACKEND_SITE_STRING_CONCAT_HH_MATERIALIZE_OWNED_BYTES: &str =
    "site.string_concat_hh.materialize_owned_bytes";
pub(crate) const BIRTH_BACKEND_SITE_STRING_CONCAT_HH_OBJECTIZE_BOX_TOTAL: &str =
    "site.string_concat_hh.objectize_box_total";
pub(crate) const BIRTH_BACKEND_SITE_STRING_CONCAT_HH_PUBLISH_HANDLE_TOTAL: &str =
    "site.string_concat_hh.publish_handle_total";
pub(crate) const BIRTH_BACKEND_SITE_STRING_SUBSTRING_CONCAT_HHII_MATERIALIZE_OWNED_TOTAL: &str =
    "site.string_substring_concat_hhii.materialize_owned_total";
pub(crate) const BIRTH_BACKEND_SITE_STRING_SUBSTRING_CONCAT_HHII_MATERIALIZE_OWNED_BYTES: &str =
    "site.string_substring_concat_hhii.materialize_owned_bytes";
pub(crate) const BIRTH_BACKEND_SITE_STRING_SUBSTRING_CONCAT_HHII_OBJECTIZE_BOX_TOTAL: &str =
    "site.string_substring_concat_hhii.objectize_box_total";
pub(crate) const BIRTH_BACKEND_SITE_STRING_SUBSTRING_CONCAT_HHII_PUBLISH_HANDLE_TOTAL: &str =
    "site.string_substring_concat_hhii.publish_handle_total";
pub(crate) const BIRTH_BACKEND_SITE_CONST_SUFFIX_MATERIALIZE_OWNED_TOTAL: &str =
    "site.const_suffix.materialize_owned_total";
pub(crate) const BIRTH_BACKEND_SITE_CONST_SUFFIX_MATERIALIZE_OWNED_BYTES: &str =
    "site.const_suffix.materialize_owned_bytes";
pub(crate) const BIRTH_BACKEND_SITE_CONST_SUFFIX_OBJECTIZE_BOX_TOTAL: &str =
    "site.const_suffix.objectize_box_total";
pub(crate) const BIRTH_BACKEND_SITE_CONST_SUFFIX_PUBLISH_HANDLE_TOTAL: &str =
    "site.const_suffix.publish_handle_total";
pub(crate) const BIRTH_BACKEND_SITE_FREEZE_TEXT_PLAN_PIECES3_MATERIALIZE_OWNED_TOTAL: &str =
    "site.freeze_text_plan_pieces3.materialize_owned_total";
pub(crate) const BIRTH_BACKEND_SITE_FREEZE_TEXT_PLAN_PIECES3_MATERIALIZE_OWNED_BYTES: &str =
    "site.freeze_text_plan_pieces3.materialize_owned_bytes";
pub(crate) const BIRTH_BACKEND_SITE_FREEZE_TEXT_PLAN_PIECES3_OBJECTIZE_BOX_TOTAL: &str =
    "site.freeze_text_plan_pieces3.objectize_box_total";
pub(crate) const BIRTH_BACKEND_SITE_FREEZE_TEXT_PLAN_PIECES3_PUBLISH_HANDLE_TOTAL: &str =
    "site.freeze_text_plan_pieces3.publish_handle_total";

pub(crate) const STR_CONCAT2_ROUTE_TOTAL: &str = "total";
pub(crate) const STR_CONCAT2_ROUTE_DISPATCH_HIT: &str = "dispatch_hit";
pub(crate) const STR_CONCAT2_ROUTE_FAST_STR_OWNED: &str = "fast_str_owned";
pub(crate) const STR_CONCAT2_ROUTE_FAST_STR_RETURN_HANDLE: &str = "fast_str_return_handle";
pub(crate) const STR_CONCAT2_ROUTE_SPAN_FREEZE: &str = "span_freeze";
pub(crate) const STR_CONCAT2_ROUTE_SPAN_RETURN_HANDLE: &str = "span_return_handle";
pub(crate) const STR_CONCAT2_ROUTE_MATERIALIZE_FALLBACK: &str = "materialize_fallback";
pub(crate) const STR_CONCAT2_ROUTE_UNCLASSIFIED: &str = "unclassified";

pub(crate) const STR_LEN_ROUTE_TOTAL: &str = "total";
pub(crate) const STR_LEN_ROUTE_DISPATCH_HIT: &str = "dispatch_hit";
pub(crate) const STR_LEN_ROUTE_FAST_STR_HIT: &str = "fast_str_hit";
pub(crate) const STR_LEN_ROUTE_FALLBACK_HIT: &str = "fallback_hit";
pub(crate) const STR_LEN_ROUTE_MISS: &str = "miss";
pub(crate) const STR_LEN_ROUTE_LATEST_FRESH_HANDLE_FAST_STR_HIT: &str =
    "latest_fresh_handle_fast_str_hit";
pub(crate) const STR_LEN_ROUTE_LATEST_FRESH_HANDLE_FALLBACK_HIT: &str =
    "latest_fresh_handle_fallback_hit";
pub(crate) const STR_LEN_ROUTE_UNCLASSIFIED: &str = "unclassified";

pub(crate) const STR_SUBSTRING_ROUTE_TOTAL: &str = "total";
pub(crate) const STR_SUBSTRING_ROUTE_VIEW_ARC_CACHE_HANDLE_HIT: &str = "view_arc_cache_handle_hit";
pub(crate) const STR_SUBSTRING_ROUTE_VIEW_ARC_CACHE_REISSUE_HIT: &str =
    "view_arc_cache_reissue_hit";
pub(crate) const STR_SUBSTRING_ROUTE_VIEW_ARC_CACHE_MISS: &str = "view_arc_cache_miss";
pub(crate) const STR_SUBSTRING_ROUTE_FAST_CACHE_HIT: &str = "fast_cache_hit";
pub(crate) const STR_SUBSTRING_ROUTE_DISPATCH_HIT: &str = "dispatch_hit";
pub(crate) const STR_SUBSTRING_ROUTE_SLOW_PLAN: &str = "slow_plan";
pub(crate) const STR_SUBSTRING_ROUTE_SLOW_PLAN_RETURN_HANDLE: &str = "slow_plan_return_handle";
pub(crate) const STR_SUBSTRING_ROUTE_SLOW_PLAN_RETURN_EMPTY: &str = "slow_plan_return_empty";
pub(crate) const STR_SUBSTRING_ROUTE_SLOW_PLAN_FREEZE_SPAN: &str = "slow_plan_freeze_span";
pub(crate) const STR_SUBSTRING_ROUTE_SLOW_PLAN_VIEW_SPAN: &str = "slow_plan_view_span";
pub(crate) const STR_SUBSTRING_ROUTE_SLOW_PLAN_UNCLASSIFIED: &str = "slow_plan_unclassified";

pub(crate) const PIECEWISE_SUBRANGE_TOTAL: &str = "total";
pub(crate) const PIECEWISE_SUBRANGE_SINGLE_SESSION_HIT: &str = "single_session_hit";
pub(crate) const PIECEWISE_SUBRANGE_FALLBACK_INSERT: &str = "fallback_insert";
pub(crate) const PIECEWISE_SUBRANGE_EMPTY_RETURN: &str = "empty_return";
pub(crate) const PIECEWISE_SUBRANGE_PREFIX_ONLY: &str = "prefix_only";
pub(crate) const PIECEWISE_SUBRANGE_MIDDLE_ONLY: &str = "middle_only";
pub(crate) const PIECEWISE_SUBRANGE_SUFFIX_ONLY: &str = "suffix_only";
pub(crate) const PIECEWISE_SUBRANGE_PREFIX_MIDDLE: &str = "prefix_middle";
pub(crate) const PIECEWISE_SUBRANGE_MIDDLE_SUFFIX: &str = "middle_suffix";
pub(crate) const PIECEWISE_SUBRANGE_PREFIX_SUFFIX: &str = "prefix_suffix";
pub(crate) const PIECEWISE_SUBRANGE_ALL_THREE: &str = "all_three";
pub(crate) const PIECEWISE_SUBRANGE_UNCLASSIFIED: &str = "unclassified";

pub(crate) const STABLE_BOX_DEMAND_OBJECT_GET_LATEST_FRESH: &str = "object_get_latest_fresh";
pub(crate) const STABLE_BOX_DEMAND_OBJECT_WITH_HANDLE_LATEST_FRESH: &str =
    "object_with_handle_latest_fresh";
pub(crate) const STABLE_BOX_DEMAND_OBJECT_PAIR_LATEST_FRESH: &str = "object_pair_latest_fresh";
pub(crate) const STABLE_BOX_DEMAND_OBJECT_TRIPLE_LATEST_FRESH: &str = "object_triple_latest_fresh";
pub(crate) const STABLE_BOX_DEMAND_TEXT_READ_HANDLE_LATEST_FRESH: &str =
    "text_read_handle_latest_fresh";
pub(crate) const STABLE_BOX_DEMAND_TEXT_READ_PAIR_LATEST_FRESH: &str =
    "text_read_pair_latest_fresh";
pub(crate) const STABLE_BOX_DEMAND_TEXT_READ_TRIPLE_LATEST_FRESH: &str =
    "text_read_triple_latest_fresh";
pub(crate) const STABLE_BOX_DEMAND_OBJECT_WITH_HANDLE_ARRAY_STORE_STR_SOURCE_LATEST_FRESH: &str =
    "object_with_handle_array_store_str_source_latest_fresh";
pub(crate) const STABLE_BOX_DEMAND_OBJECT_WITH_HANDLE_SUBSTRING_PLAN_LATEST_FRESH: &str =
    "object_with_handle_substring_plan_latest_fresh";
pub(crate) const STABLE_BOX_DEMAND_OBJECT_WITH_HANDLE_DECODE_ARRAY_FAST_LATEST_FRESH: &str =
    "object_with_handle_decode_array_fast_latest_fresh";
pub(crate) const STABLE_BOX_DEMAND_OBJECT_WITH_HANDLE_DECODE_ANY_ARG_LATEST_FRESH: &str =
    "object_with_handle_decode_any_arg_latest_fresh";
pub(crate) const STABLE_BOX_DEMAND_OBJECT_WITH_HANDLE_DECODE_ANY_INDEX_LATEST_FRESH: &str =
    "object_with_handle_decode_any_index_latest_fresh";

pub(crate) const BORROWED_ALIAS_TO_STRING_BOX: &str = "to_string_box";
pub(crate) const BORROWED_ALIAS_EQUALS: &str = "equals";
pub(crate) const BORROWED_ALIAS_CLONE_BOX: &str = "clone_box";
pub(crate) const BORROWED_ALIAS_TO_STRING_BOX_LATEST_FRESH: &str = "to_string_box_latest_fresh";
pub(crate) const BORROWED_ALIAS_EQUALS_LATEST_FRESH: &str = "equals_latest_fresh";
pub(crate) const BORROWED_ALIAS_CLONE_BOX_LATEST_FRESH: &str = "clone_box_latest_fresh";
pub(crate) const BORROWED_ALIAS_BORROWED_SOURCE_FAST: &str = "borrowed_source_fast";
pub(crate) const BORROWED_ALIAS_AS_STR_FAST: &str = "as_str_fast";
pub(crate) const BORROWED_ALIAS_AS_STR_FAST_LIVE_SOURCE: &str = "as_str_fast_live_source";
pub(crate) const BORROWED_ALIAS_AS_STR_FAST_STALE_SOURCE: &str = "as_str_fast_stale_source";
pub(crate) const BORROWED_ALIAS_ARRAY_LEN_BY_INDEX_LATEST_FRESH: &str =
    "array_len_by_index_latest_fresh";
pub(crate) const BORROWED_ALIAS_ARRAY_INDEXOF_BY_INDEX_LATEST_FRESH: &str =
    "array_indexof_by_index_latest_fresh";
pub(crate) const BORROWED_ALIAS_ENCODE_EPOCH_HIT: &str = "encode_epoch_hit";
pub(crate) const BORROWED_ALIAS_ENCODE_CACHED_HANDLE_HIT: &str = "encode_cached_handle_hit";
pub(crate) const BORROWED_ALIAS_ENCODE_CACHED_HANDLE_HIT_ARRAY_GET_INDEX: &str =
    "encode_cached_handle_hit_array_get_index";
pub(crate) const BORROWED_ALIAS_ENCODE_CACHED_HANDLE_HIT_MAP_RUNTIME_DATA_GET_ANY: &str =
    "encode_cached_handle_hit_map_runtime_data_get_any";
pub(crate) const BORROWED_ALIAS_ENCODE_PTR_EQ_HIT: &str = "encode_ptr_eq_hit";
pub(crate) const BORROWED_ALIAS_ENCODE_TO_HANDLE_ARC: &str = "encode_to_handle_arc";
pub(crate) const BORROWED_ALIAS_ENCODE_TO_HANDLE_ARC_ARRAY_GET_INDEX: &str =
    "encode_to_handle_arc_array_get_index";
pub(crate) const BORROWED_ALIAS_ENCODE_TO_HANDLE_ARC_MAP_RUNTIME_DATA_GET_ANY: &str =
    "encode_to_handle_arc_map_runtime_data_get_any";
