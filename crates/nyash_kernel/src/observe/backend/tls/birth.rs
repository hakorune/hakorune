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

macro_rules! tls_bump_total_and_unit_method {
    ($($name:ident => $total_field:ident, $field:ident,)+) => {
        $(
            #[inline(always)]
            fn $name(&self) {
                Self::bump(&self.$total_field);
                Self::bump(&self.$field);
            }
        )+
    };
}

macro_rules! tls_bump_total_and_bytes_method {
    ($($name:ident => $count_field:ident, $bytes_field:ident,)+) => {
        $(
            #[inline(always)]
            fn $name(&self, bytes: u64) {
                Self::bump(&self.$count_field);
                self.$bytes_field.set(self.$bytes_field.get() + bytes);
            }
        )+
    };
}

impl ThreadCounters {
    tls_bump_unit_method! {
        birth_placement_return_handle => birth_placement_return_handle,
        birth_placement_borrow_view => birth_placement_borrow_view,
        birth_placement_freeze_owned => birth_placement_freeze_owned,
        birth_placement_fresh_handle => birth_placement_fresh_handle,
        birth_placement_materialize_owned => birth_placement_materialize_owned,
        birth_placement_store_from_source => birth_placement_store_from_source,
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
        birth_backend_publish_boundary_slot_publish_handle => birth_backend_publish_boundary_slot_publish_handle_total,
        birth_backend_publish_boundary_slot_objectize_stable_box => birth_backend_publish_boundary_slot_objectize_stable_box_total,
        birth_backend_publish_boundary_slot_empty => birth_backend_publish_boundary_slot_empty,
        birth_backend_publish_boundary_slot_already_published => birth_backend_publish_boundary_slot_already_published,
        birth_backend_site_string_concat_hh_objectize_box => birth_backend_site_string_concat_hh_objectize_box_total,
        birth_backend_site_string_concat_hh_publish_handle => birth_backend_site_string_concat_hh_publish_handle_total,
        birth_backend_site_string_substring_concat_hhii_objectize_box => birth_backend_site_string_substring_concat_hhii_objectize_box_total,
        birth_backend_site_string_substring_concat_hhii_publish_handle => birth_backend_site_string_substring_concat_hhii_publish_handle_total,
        birth_backend_site_const_suffix_objectize_box => birth_backend_site_const_suffix_objectize_box_total,
        birth_backend_site_const_suffix_publish_handle => birth_backend_site_const_suffix_publish_handle_total,
        birth_backend_site_freeze_text_plan_pieces3_objectize_box => birth_backend_site_freeze_text_plan_pieces3_objectize_box_total,
        birth_backend_site_freeze_text_plan_pieces3_publish_handle => birth_backend_site_freeze_text_plan_pieces3_publish_handle_total,
    }

    tls_bump_total_and_unit_method! {
        birth_backend_freeze_text_plan_view1 => birth_backend_freeze_text_plan_total, birth_backend_freeze_text_plan_view1,
        birth_backend_freeze_text_plan_pieces2 => birth_backend_freeze_text_plan_total, birth_backend_freeze_text_plan_pieces2,
        birth_backend_freeze_text_plan_pieces3 => birth_backend_freeze_text_plan_total, birth_backend_freeze_text_plan_pieces3,
        birth_backend_freeze_text_plan_pieces4 => birth_backend_freeze_text_plan_total, birth_backend_freeze_text_plan_pieces4,
        birth_backend_freeze_text_plan_owned_tmp => birth_backend_freeze_text_plan_total, birth_backend_freeze_text_plan_owned_tmp,
    }

    tls_bump_total_and_bytes_method! {
        birth_backend_string_box_new => birth_backend_string_box_new_total, birth_backend_string_box_new_bytes,
        birth_backend_string_box_ctor => birth_backend_string_box_ctor_total, birth_backend_string_box_ctor_bytes,
        birth_backend_objectize_stable_box_now => birth_backend_objectize_stable_box_now_total, birth_backend_objectize_stable_box_now_bytes,
        birth_backend_materialize_owned => birth_backend_materialize_owned_total, birth_backend_materialize_owned_bytes,
        birth_backend_gc_alloc => birth_backend_gc_alloc_called, birth_backend_gc_alloc_bytes,
        birth_backend_site_string_concat_hh_materialize_owned => birth_backend_site_string_concat_hh_materialize_owned_total, birth_backend_site_string_concat_hh_materialize_owned_bytes,
        birth_backend_site_string_substring_concat_hhii_materialize_owned => birth_backend_site_string_substring_concat_hhii_materialize_owned_total, birth_backend_site_string_substring_concat_hhii_materialize_owned_bytes,
        birth_backend_site_const_suffix_materialize_owned => birth_backend_site_const_suffix_materialize_owned_total, birth_backend_site_const_suffix_materialize_owned_bytes,
        birth_backend_site_freeze_text_plan_pieces3_materialize_owned => birth_backend_site_freeze_text_plan_pieces3_materialize_owned_total, birth_backend_site_freeze_text_plan_pieces3_materialize_owned_bytes,
    }
}
