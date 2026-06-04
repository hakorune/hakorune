macro_rules! tls_bump_unit_method {
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
    #[inline(always)]
    fn bump(cell: &Cell<u64>) {
        cell.set(cell.get() + 1);
    }

    tls_bump_unit_method! {
        store_array_str_enter => store_array_str_total,
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
        store_array_str_update_text_resident_hit => store_array_str_update_text_resident_hit,
        store_array_str_update_text_resident_miss => store_array_str_update_text_resident_miss,
        store_array_str_update_text_fallback_hit => store_array_str_update_text_fallback_hit,
        store_array_str_update_text_fallback_miss => store_array_str_update_text_fallback_miss,
    }

    #[inline(always)]
    fn store_array_str_cache_probe(&self, kind: CacheProbeKind) {
        match kind {
            CacheProbeKind::Hit => Self::bump(&self.store_array_str_cache_hit),
            CacheProbeKind::MissHandle => Self::bump(&self.store_array_str_cache_miss_handle),
            CacheProbeKind::MissDropEpoch => Self::bump(&self.store_array_str_cache_miss_epoch),
        }
    }
}
