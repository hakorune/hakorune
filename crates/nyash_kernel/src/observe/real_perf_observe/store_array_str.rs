use super::CacheProbeKind;

pub(crate) fn record_store_array_str_enter() {
    super::super::backend::store_array_str_enter();
}

pub(crate) fn record_store_array_str_cache_probe(kind: CacheProbeKind) {
    super::super::backend::store_array_str_cache_probe(kind);
}

pub(crate) fn record_store_array_str_retarget_hit() {
    super::super::backend::store_array_str_retarget_hit();
}

pub(crate) fn record_store_array_str_latest_fresh_retarget_hit() {
    super::super::backend::store_array_str_latest_fresh_retarget_hit();
}

pub(crate) fn record_store_array_str_source_store() {
    super::super::backend::store_array_str_source_store();
}

pub(crate) fn record_store_array_str_latest_fresh_source_store() {
    super::super::backend::store_array_str_latest_fresh_source_store();
}

pub(crate) fn record_store_array_str_non_string_source() {
    super::super::backend::store_array_str_non_string_source();
}

pub(crate) fn record_store_array_str_existing_slot() {
    super::super::backend::store_array_str_existing_slot();
}

pub(crate) fn record_store_array_str_append_slot() {
    super::super::backend::store_array_str_append_slot();
}

pub(crate) fn record_store_array_str_source_string_box() {
    super::super::backend::store_array_str_source_string_box();
}

pub(crate) fn record_store_array_str_source_string_view() {
    super::super::backend::store_array_str_source_string_view();
}

pub(crate) fn record_store_array_str_source_missing() {
    super::super::backend::store_array_str_source_missing();
}

pub(crate) fn record_store_array_str_plan_source_kind_string_like() {
    super::super::backend::store_array_str_plan_source_kind_string_like();
}

pub(crate) fn record_store_array_str_plan_source_kind_other_object() {
    super::super::backend::store_array_str_plan_source_kind_other_object();
}

pub(crate) fn record_store_array_str_plan_source_kind_missing() {
    super::super::backend::store_array_str_plan_source_kind_missing();
}

pub(crate) fn record_store_array_str_plan_slot_kind_borrowed_alias() {
    super::super::backend::store_array_str_plan_slot_kind_borrowed_alias();
}

pub(crate) fn record_store_array_str_plan_slot_kind_other() {
    super::super::backend::store_array_str_plan_slot_kind_other();
}

pub(crate) fn record_store_array_str_plan_action_retarget_alias() {
    super::super::backend::store_array_str_plan_action_retarget_alias();
}

pub(crate) fn record_store_array_str_plan_action_store_from_source() {
    super::super::backend::store_array_str_plan_action_store_from_source();
}

pub(crate) fn record_store_array_str_plan_action_need_stable_object() {
    super::super::backend::store_array_str_plan_action_need_stable_object();
}

pub(crate) fn record_store_array_str_reason_source_kind_via_object() {
    super::super::backend::store_array_str_reason_source_kind_via_object();
}

pub(crate) fn record_store_array_str_reason_retarget_keep_source_arc() {
    super::super::backend::store_array_str_reason_retarget_keep_source_arc();
}

pub(crate) fn record_store_array_str_reason_retarget_keep_source_arc_ptr_eq_hit() {
    super::super::backend::store_array_str_reason_retarget_keep_source_arc_ptr_eq_hit();
}

pub(crate) fn record_store_array_str_reason_retarget_keep_source_arc_ptr_eq_miss() {
    super::super::backend::store_array_str_reason_retarget_keep_source_arc_ptr_eq_miss();
}

pub(crate) fn record_store_array_str_reason_retarget_alias_update() {
    super::super::backend::store_array_str_reason_retarget_alias_update();
}

pub(crate) fn record_store_array_str_lookup_registry_slot_read() {
    super::super::backend::store_array_str_lookup_registry_slot_read();
}

pub(crate) fn record_store_array_str_lookup_caller_latest_fresh_tag() {
    super::super::backend::store_array_str_lookup_caller_latest_fresh_tag();
}

pub(crate) fn record_store_array_str_update_text_resident_hit() {
    super::super::backend::store_array_str_update_text_resident_hit();
}

pub(crate) fn record_store_array_str_update_text_resident_miss() {
    super::super::backend::store_array_str_update_text_resident_miss();
}

pub(crate) fn record_store_array_str_update_text_fallback_hit() {
    super::super::backend::store_array_str_update_text_fallback_hit();
}

pub(crate) fn record_store_array_str_update_text_fallback_miss() {
    super::super::backend::store_array_str_update_text_fallback_miss();
}
