pub(crate) const STORE_ARRAY_STR: &str = "store.array.str";
pub(crate) const CONST_SUFFIX: &str = "const_suffix";
pub(crate) const BIRTH_PLACEMENT: &str = "birth.placement";
pub(crate) const BIRTH_BACKEND: &str = "birth.backend";
pub(crate) const STR_CONCAT2_ROUTE: &str = "str.concat2.route";
pub(crate) const STR_LEN_ROUTE: &str = "str.len.route";

pub(crate) const STORE_ARRAY_STR_CACHE_HIT: &str = "cache_hit";
pub(crate) const STORE_ARRAY_STR_CACHE_MISS_HANDLE: &str = "cache_miss_handle";
pub(crate) const STORE_ARRAY_STR_CACHE_MISS_EPOCH: &str = "cache_miss_epoch";
pub(crate) const STORE_ARRAY_STR_RETARGET_HIT: &str = "retarget_hit";
pub(crate) const STORE_ARRAY_STR_SOURCE_STORE: &str = "source_store";
pub(crate) const STORE_ARRAY_STR_NON_STRING_SOURCE: &str = "non_string_source";
pub(crate) const STORE_ARRAY_STR_EXISTING_SLOT: &str = "existing_slot";
pub(crate) const STORE_ARRAY_STR_APPEND_SLOT: &str = "append_slot";
pub(crate) const STORE_ARRAY_STR_SOURCE_STRING_BOX: &str = "source_string_box";
pub(crate) const STORE_ARRAY_STR_SOURCE_STRING_VIEW: &str = "source_string_view";
pub(crate) const STORE_ARRAY_STR_SOURCE_MISSING: &str = "source_missing";

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

pub(crate) const STR_CONCAT2_ROUTE_TOTAL: &str = "total";
pub(crate) const STR_CONCAT2_ROUTE_DISPATCH_HIT: &str = "dispatch_hit";
pub(crate) const STR_CONCAT2_ROUTE_FAST_STR_OWNED: &str = "fast_str_owned";
pub(crate) const STR_CONCAT2_ROUTE_FAST_STR_RETURN_HANDLE: &str = "fast_str_return_handle";
pub(crate) const STR_CONCAT2_ROUTE_SPAN_FREEZE: &str = "span_freeze";
pub(crate) const STR_CONCAT2_ROUTE_SPAN_RETURN_HANDLE: &str = "span_return_handle";
pub(crate) const STR_CONCAT2_ROUTE_MATERIALIZE_FALLBACK: &str = "materialize_fallback";

pub(crate) const STR_LEN_ROUTE_TOTAL: &str = "total";
pub(crate) const STR_LEN_ROUTE_DISPATCH_HIT: &str = "dispatch_hit";
pub(crate) const STR_LEN_ROUTE_FAST_STR_HIT: &str = "fast_str_hit";
pub(crate) const STR_LEN_ROUTE_FALLBACK_HIT: &str = "fallback_hit";
pub(crate) const STR_LEN_ROUTE_MISS: &str = "miss";
