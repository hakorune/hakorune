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

}
