impl ThreadCounters {
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
            .set(
                self.birth_backend_site_string_concat_hh_materialize_owned_bytes
                    .get()
                    + bytes,
            );
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
        Self::bump(&self.birth_backend_site_string_substring_concat_hhii_materialize_owned_total);
        self.birth_backend_site_string_substring_concat_hhii_materialize_owned_bytes
            .set(
                self.birth_backend_site_string_substring_concat_hhii_materialize_owned_bytes
                    .get()
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
            .set(
                self.birth_backend_site_const_suffix_materialize_owned_bytes
                    .get()
                    + bytes,
            );
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
        self.birth_backend_site_freeze_text_plan_pieces3_materialize_owned_bytes
            .set(
                self.birth_backend_site_freeze_text_plan_pieces3_materialize_owned_bytes
                    .get()
                    + bytes,
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

}
