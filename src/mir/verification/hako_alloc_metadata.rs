use crate::mir::array_record_packed_autouse_pilot::ARRAY_RECORD_PACKED_AUTOUSE_PILOT_KIND_INTEGER_LANE_DIRECT_READS_V0;
use crate::mir::function::{ArrayRecordPackedAutoUsePilotPlan, RecordLayoutPlan};
use crate::mir::hako_alloc_aligned_small_packed_store_pilot::{
    HAKO_ALLOC_ALIGNED_SMALL_META_RECORD, HAKO_ALLOC_ALIGNED_SMALL_META_STORE_OWNER,
    HAKO_ALLOC_ALIGNED_SMALL_PACKED_STORE_PILOT_KIND,
};
use crate::mir::hako_alloc_huge_page_packed_store_pilot::{
    HAKO_ALLOC_HUGE_PAGE_META_RECORD, HAKO_ALLOC_HUGE_PAGE_META_STORE_OWNER,
    HAKO_ALLOC_HUGE_PAGE_PACKED_STORE_PILOT_KIND, HAKO_ALLOC_HUGE_PAGE_RELEASED_PAGE_ID_SENTINEL,
    HAKO_ALLOC_HUGE_PAGE_RELEASED_SIZE_SENTINEL,
};
use crate::mir::verification_types::VerificationError;
use crate::mir::MirModule;
use std::collections::HashSet;

pub(super) fn check_hako_alloc_metadata_invariants(
    module: &MirModule,
) -> Result<(), Vec<VerificationError>> {
    let mut errors = Vec::new();
    check_aligned_small_rows(module, &mut errors);
    check_huge_page_rows(module, &mut errors);

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn check_aligned_small_rows(module: &MirModule, errors: &mut Vec<VerificationError>) {
    let mut seen = HashSet::new();
    for plan in &module
        .metadata
        .hako_alloc_aligned_small_packed_store_pilot_plans
    {
        if !seen.insert((plan.record_name.as_str(), plan.layout_id)) {
            push_error(
                errors,
                &plan.record_name,
                plan.layout_id,
                "duplicate aligned-small metadata packed-store plan",
            );
        }

        expect_eq(
            errors,
            &plan.record_name,
            plan.layout_id,
            "aligned-small record_name",
            &plan.record_name,
            HAKO_ALLOC_ALIGNED_SMALL_META_RECORD,
        );
        expect_eq(
            errors,
            &plan.record_name,
            plan.layout_id,
            "aligned-small store_owner",
            &plan.store_owner,
            HAKO_ALLOC_ALIGNED_SMALL_META_STORE_OWNER,
        );
        expect_eq(
            errors,
            &plan.record_name,
            plan.layout_id,
            "aligned-small pilot_kind",
            &plan.pilot_kind,
            HAKO_ALLOC_ALIGNED_SMALL_PACKED_STORE_PILOT_KIND,
        );
        check_common_plan_flags(
            errors,
            &plan.record_name,
            plan.layout_id,
            "aligned-small",
            plan.private_runtime_storage_enabled,
            plan.hako_alloc_source_mentions_compiler,
            plan.live_scalar_columns_retained,
            plan.public_array_get_materialization_enabled,
            plan.backend_lowering_enabled,
        );

        if (
            plan.ptr_column,
            plan.alignment_column,
            plan.padded_size_column,
        ) != (0, 1, 2)
        {
            push_error(
                errors,
                &plan.record_name,
                plan.layout_id,
                "aligned-small columns must be ptr=0, alignment=1, padded_size=2",
            );
        }

        check_source_packed_pilot(module, &plan.record_name, plan.layout_id, 3, errors);
        check_record_layout(
            module,
            &plan.record_name,
            plan.layout_id,
            &[
                ("ptr", plan.ptr_column),
                ("alignment", plan.alignment_column),
                ("padded_size", plan.padded_size_column),
            ],
            errors,
        );
    }
}

fn check_huge_page_rows(module: &MirModule, errors: &mut Vec<VerificationError>) {
    let mut seen = HashSet::new();
    for plan in &module
        .metadata
        .hako_alloc_huge_page_packed_store_pilot_plans
    {
        if !seen.insert((plan.record_name.as_str(), plan.layout_id)) {
            push_error(
                errors,
                &plan.record_name,
                plan.layout_id,
                "duplicate huge-page metadata packed-store plan",
            );
        }

        expect_eq(
            errors,
            &plan.record_name,
            plan.layout_id,
            "huge-page record_name",
            &plan.record_name,
            HAKO_ALLOC_HUGE_PAGE_META_RECORD,
        );
        expect_eq(
            errors,
            &plan.record_name,
            plan.layout_id,
            "huge-page store_owner",
            &plan.store_owner,
            HAKO_ALLOC_HUGE_PAGE_META_STORE_OWNER,
        );
        expect_eq(
            errors,
            &plan.record_name,
            plan.layout_id,
            "huge-page pilot_kind",
            &plan.pilot_kind,
            HAKO_ALLOC_HUGE_PAGE_PACKED_STORE_PILOT_KIND,
        );
        check_common_plan_flags(
            errors,
            &plan.record_name,
            plan.layout_id,
            "huge-page",
            plan.private_runtime_storage_enabled,
            plan.hako_alloc_source_mentions_compiler,
            plan.live_scalar_columns_retained,
            plan.public_array_get_materialization_enabled,
            plan.backend_lowering_enabled,
        );

        if (
            plan.page_id_column,
            plan.ptr_column,
            plan.requested_size_column,
            plan.committed_size_column,
            plan.live_column,
        ) != (0, 1, 2, 3, 4)
        {
            push_error(
                errors,
                &plan.record_name,
                plan.layout_id,
                "huge-page columns must be page_id=0, ptr=1, requested_size=2, committed_size=3, live=4",
            );
        }
        if plan.released_page_id_sentinel != HAKO_ALLOC_HUGE_PAGE_RELEASED_PAGE_ID_SENTINEL {
            push_error(
                errors,
                &plan.record_name,
                plan.layout_id,
                "huge-page released_page_id_sentinel must stay -1",
            );
        }
        if plan.released_size_sentinel != HAKO_ALLOC_HUGE_PAGE_RELEASED_SIZE_SENTINEL {
            push_error(
                errors,
                &plan.record_name,
                plan.layout_id,
                "huge-page released_size_sentinel must stay 0",
            );
        }

        check_source_packed_pilot(module, &plan.record_name, plan.layout_id, 5, errors);
        check_record_layout(
            module,
            &plan.record_name,
            plan.layout_id,
            &[
                ("page_id", plan.page_id_column),
                ("ptr", plan.ptr_column),
                ("requested_size", plan.requested_size_column),
                ("committed_size", plan.committed_size_column),
                ("live", plan.live_column),
            ],
            errors,
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn check_common_plan_flags(
    errors: &mut Vec<VerificationError>,
    record_name: &str,
    layout_id: u32,
    prefix: &str,
    private_runtime_storage_enabled: bool,
    hako_alloc_source_mentions_compiler: bool,
    live_scalar_columns_retained: bool,
    public_array_get_materialization_enabled: bool,
    backend_lowering_enabled: bool,
) {
    expect_bool(
        errors,
        record_name,
        layout_id,
        &format!("{} private_runtime_storage_enabled", prefix),
        private_runtime_storage_enabled,
        true,
    );
    expect_bool(
        errors,
        record_name,
        layout_id,
        &format!("{} hako_alloc_source_mentions_compiler", prefix),
        hako_alloc_source_mentions_compiler,
        false,
    );
    expect_bool(
        errors,
        record_name,
        layout_id,
        &format!("{} live_scalar_columns_retained", prefix),
        live_scalar_columns_retained,
        true,
    );
    expect_bool(
        errors,
        record_name,
        layout_id,
        &format!("{} public_array_get_materialization_enabled", prefix),
        public_array_get_materialization_enabled,
        false,
    );
    expect_bool(
        errors,
        record_name,
        layout_id,
        &format!("{} backend_lowering_enabled", prefix),
        backend_lowering_enabled,
        false,
    );
}

fn check_source_packed_pilot(
    module: &MirModule,
    record_name: &str,
    layout_id: u32,
    expected_integer_columns: u32,
    errors: &mut Vec<VerificationError>,
) {
    let Some(pilot) = module
        .metadata
        .array_record_packed_autouse_pilot_plans
        .iter()
        .find(|pilot| pilot.record_name == record_name && pilot.layout_id == layout_id)
    else {
        push_error(
            errors,
            record_name,
            layout_id,
            "missing source C209 packed ArrayBox pilot",
        );
        return;
    };

    check_packed_pilot_flags(pilot, expected_integer_columns, errors);
}

fn check_packed_pilot_flags(
    pilot: &ArrayRecordPackedAutoUsePilotPlan,
    expected_integer_columns: u32,
    errors: &mut Vec<VerificationError>,
) {
    expect_eq(
        errors,
        &pilot.record_name,
        pilot.layout_id,
        "source packed pilot_kind",
        &pilot.pilot_kind,
        ARRAY_RECORD_PACKED_AUTOUSE_PILOT_KIND_INTEGER_LANE_DIRECT_READS_V0,
    );
    if pilot.integer_lane_columns != expected_integer_columns {
        push_error(
            errors,
            &pilot.record_name,
            pilot.layout_id,
            &format!(
                "source packed pilot integer_lane_columns must be {}",
                expected_integer_columns
            ),
        );
    }
    expect_bool(
        errors,
        &pilot.record_name,
        pilot.layout_id,
        "source packed direct_indexed_field_reads_enabled",
        pilot.direct_indexed_field_reads_enabled,
        true,
    );
    expect_bool(
        errors,
        &pilot.record_name,
        pilot.layout_id,
        "source packed private_runtime_storage_enabled",
        pilot.private_runtime_storage_enabled,
        true,
    );
    expect_bool(
        errors,
        &pilot.record_name,
        pilot.layout_id,
        "source packed public_array_get_materialization_enabled",
        pilot.public_array_get_materialization_enabled,
        false,
    );
    expect_bool(
        errors,
        &pilot.record_name,
        pilot.layout_id,
        "source packed hako_alloc_migration_enabled",
        pilot.hako_alloc_migration_enabled,
        false,
    );
    expect_bool(
        errors,
        &pilot.record_name,
        pilot.layout_id,
        "source packed backend_lowering_enabled",
        pilot.backend_lowering_enabled,
        false,
    );
}

fn check_record_layout(
    module: &MirModule,
    record_name: &str,
    layout_id: u32,
    expected_fields: &[(&str, u32)],
    errors: &mut Vec<VerificationError>,
) {
    let Some(layout) = module
        .metadata
        .record_layout_plans
        .iter()
        .find(|layout| layout.record_name == record_name && layout.layout_id == layout_id)
    else {
        push_error(
            errors,
            record_name,
            layout_id,
            "missing backing record layout",
        );
        return;
    };

    check_layout_fields(layout, expected_fields, errors);
}

fn check_layout_fields(
    layout: &RecordLayoutPlan,
    expected_fields: &[(&str, u32)],
    errors: &mut Vec<VerificationError>,
) {
    if layout.field_count != expected_fields.len() as u32
        || layout.fields.len() != expected_fields.len()
    {
        push_error(
            errors,
            &layout.record_name,
            layout.layout_id,
            "backing record layout field count mismatch",
        );
        return;
    }

    for (expected_name, expected_slot) in expected_fields {
        let Some(field) = layout
            .fields
            .iter()
            .find(|field| field.name == *expected_name)
        else {
            push_error(
                errors,
                &layout.record_name,
                layout.layout_id,
                &format!("missing backing record field `{}`", expected_name),
            );
            continue;
        };

        if field.slot != *expected_slot {
            push_error(
                errors,
                &layout.record_name,
                layout.layout_id,
                &format!(
                    "backing record field `{}` must use slot {}",
                    expected_name, expected_slot
                ),
            );
        }
        if !field.storage.uses_integer_lane() {
            push_error(
                errors,
                &layout.record_name,
                layout.layout_id,
                &format!(
                    "backing record field `{}` must use integer lane",
                    expected_name
                ),
            );
        }
    }
}

fn expect_eq(
    errors: &mut Vec<VerificationError>,
    record_name: &str,
    layout_id: u32,
    label: &str,
    actual: &str,
    expected: &str,
) {
    if actual != expected {
        push_error(
            errors,
            record_name,
            layout_id,
            &format!("{} must be `{}`", label, expected),
        );
    }
}

fn expect_bool(
    errors: &mut Vec<VerificationError>,
    record_name: &str,
    layout_id: u32,
    label: &str,
    actual: bool,
    expected: bool,
) {
    if actual != expected {
        push_error(
            errors,
            record_name,
            layout_id,
            &format!("{} must be {}", label, expected),
        );
    }
}

fn push_error(
    errors: &mut Vec<VerificationError>,
    record_name: &str,
    layout_id: u32,
    reason: &str,
) {
    errors.push(VerificationError::HakoAllocMetadataInvariantViolation {
        record_name: record_name.to_string(),
        layout_id,
        reason: reason.to_string(),
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::function::{
        ArrayRecordPackedAutoUsePilotPlan, HakoAllocAlignedSmallPackedStorePilotPlan,
        HakoAllocHugePagePackedStorePilotPlan, RecordLayoutFieldPlan, RecordLayoutPlan,
        TypedObjectFieldStorage,
    };
    use crate::mir::MirModule;

    fn source_pilot(
        record_name: &str,
        layout_id: u32,
        integer_lane_columns: u32,
    ) -> ArrayRecordPackedAutoUsePilotPlan {
        ArrayRecordPackedAutoUsePilotPlan {
            record_name: record_name.to_string(),
            layout_id,
            pilot_kind: ARRAY_RECORD_PACKED_AUTOUSE_PILOT_KIND_INTEGER_LANE_DIRECT_READS_V0
                .to_string(),
            source_boundary_kind: "non_escaping_direct_field_reads_v0".to_string(),
            integer_lane_columns,
            direct_indexed_field_reads_enabled: true,
            private_runtime_storage_enabled: true,
            public_array_get_materialization_enabled: false,
            hako_alloc_migration_enabled: false,
            backend_lowering_enabled: false,
        }
    }

    fn record_layout(record_name: &str, layout_id: u32, fields: &[&str]) -> RecordLayoutPlan {
        RecordLayoutPlan {
            record_name: record_name.to_string(),
            layout_id,
            layout_kind: "record_value_aggregate_v0".to_string(),
            field_count: fields.len() as u32,
            fields: fields
                .iter()
                .enumerate()
                .map(|(slot, name)| RecordLayoutFieldPlan {
                    name: (*name).to_string(),
                    slot: slot as u32,
                    declared_type_name: Some("i64".to_string()),
                    storage: TypedObjectFieldStorage::I64,
                })
                .collect(),
        }
    }

    fn aligned_plan(layout_id: u32) -> HakoAllocAlignedSmallPackedStorePilotPlan {
        HakoAllocAlignedSmallPackedStorePilotPlan {
            record_name: HAKO_ALLOC_ALIGNED_SMALL_META_RECORD.to_string(),
            store_owner: HAKO_ALLOC_ALIGNED_SMALL_META_STORE_OWNER.to_string(),
            layout_id,
            pilot_kind: HAKO_ALLOC_ALIGNED_SMALL_PACKED_STORE_PILOT_KIND.to_string(),
            ptr_column: 0,
            alignment_column: 1,
            padded_size_column: 2,
            private_runtime_storage_enabled: true,
            hako_alloc_source_mentions_compiler: false,
            live_scalar_columns_retained: true,
            public_array_get_materialization_enabled: false,
            backend_lowering_enabled: false,
        }
    }

    fn huge_plan(layout_id: u32) -> HakoAllocHugePagePackedStorePilotPlan {
        HakoAllocHugePagePackedStorePilotPlan {
            record_name: HAKO_ALLOC_HUGE_PAGE_META_RECORD.to_string(),
            store_owner: HAKO_ALLOC_HUGE_PAGE_META_STORE_OWNER.to_string(),
            layout_id,
            pilot_kind: HAKO_ALLOC_HUGE_PAGE_PACKED_STORE_PILOT_KIND.to_string(),
            page_id_column: 0,
            ptr_column: 1,
            requested_size_column: 2,
            committed_size_column: 3,
            live_column: 4,
            released_page_id_sentinel: HAKO_ALLOC_HUGE_PAGE_RELEASED_PAGE_ID_SENTINEL,
            released_size_sentinel: HAKO_ALLOC_HUGE_PAGE_RELEASED_SIZE_SENTINEL,
            private_runtime_storage_enabled: true,
            hako_alloc_source_mentions_compiler: false,
            live_scalar_columns_retained: true,
            public_array_get_materialization_enabled: false,
            backend_lowering_enabled: false,
        }
    }

    fn valid_module() -> MirModule {
        let mut module = MirModule::new("hako-alloc-metadata-test".to_string());
        module
            .metadata
            .array_record_packed_autouse_pilot_plans
            .push(source_pilot(HAKO_ALLOC_ALIGNED_SMALL_META_RECORD, 17, 3));
        module
            .metadata
            .array_record_packed_autouse_pilot_plans
            .push(source_pilot(HAKO_ALLOC_HUGE_PAGE_META_RECORD, 29, 5));
        module.metadata.record_layout_plans.push(record_layout(
            HAKO_ALLOC_ALIGNED_SMALL_META_RECORD,
            17,
            &["ptr", "alignment", "padded_size"],
        ));
        module.metadata.record_layout_plans.push(record_layout(
            HAKO_ALLOC_HUGE_PAGE_META_RECORD,
            29,
            &["page_id", "ptr", "requested_size", "committed_size", "live"],
        ));
        module
            .metadata
            .hako_alloc_aligned_small_packed_store_pilot_plans
            .push(aligned_plan(17));
        module
            .metadata
            .hako_alloc_huge_page_packed_store_pilot_plans
            .push(huge_plan(29));
        module
    }

    fn first_reason(module: &MirModule) -> String {
        let errors = check_hako_alloc_metadata_invariants(module).unwrap_err();
        match &errors[0] {
            VerificationError::HakoAllocMetadataInvariantViolation { reason, .. } => reason.clone(),
            other => panic!("unexpected verifier error: {:?}", other),
        }
    }

    #[test]
    fn verifier_accepts_valid_hako_alloc_metadata_rows() {
        let module = valid_module();
        assert!(check_hako_alloc_metadata_invariants(&module).is_ok());
    }

    #[test]
    fn verifier_rejects_missing_source_pilot() {
        let mut module = valid_module();
        module
            .metadata
            .array_record_packed_autouse_pilot_plans
            .clear();

        let reason = first_reason(&module);
        assert!(reason.contains("missing source C209 packed ArrayBox pilot"));
    }

    #[test]
    fn verifier_rejects_malformed_aligned_columns() {
        let mut module = valid_module();
        module
            .metadata
            .hako_alloc_aligned_small_packed_store_pilot_plans[0]
            .alignment_column = 2;

        let reason = first_reason(&module);
        assert!(reason.contains("aligned-small columns must be"));
    }

    #[test]
    fn verifier_rejects_visible_materialization() {
        let mut module = valid_module();
        module
            .metadata
            .hako_alloc_huge_page_packed_store_pilot_plans[0]
            .public_array_get_materialization_enabled = true;

        let reason = first_reason(&module);
        assert!(reason.contains("public_array_get_materialization_enabled must be false"));
    }

    #[test]
    fn verifier_rejects_bad_huge_released_sentinel() {
        let mut module = valid_module();
        module
            .metadata
            .hako_alloc_huge_page_packed_store_pilot_plans[0]
            .released_page_id_sentinel = 0;

        let reason = first_reason(&module);
        assert!(reason.contains("released_page_id_sentinel must stay -1"));
    }
}
