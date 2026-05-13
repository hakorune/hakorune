/*!
 * Packed ArrayBox auto-use pilot metadata.
 *
 * This owner is the narrow C209 bridge from C207/C208 metadata to a future
 * compiler-selected packed ArrayBox path. It opens only the non-escaping
 * integer-lane direct-field-read pilot shape. It does not migrate hako_alloc,
 * materialize record objects, add public ArrayBox APIs, or add backend lowering.
 */

use crate::mir::array_record_materialization_boundary::ARRAY_RECORD_MATERIALIZATION_BOUNDARY_KIND_NON_ESCAPING_DIRECT_FIELD_READS_V0;
use crate::mir::function::{
    ArrayRecordAutoUseEligibilityPlan, ArrayRecordMaterializationBoundaryPlan,
    ArrayRecordPackedAutoUsePilotPlan,
};
use crate::mir::MirModule;

pub const ARRAY_RECORD_PACKED_AUTOUSE_PILOT_KIND_INTEGER_LANE_DIRECT_READS_V0: &str =
    "integer_lane_direct_reads_v0";

pub fn refresh_module_array_record_packed_autouse_pilot_plans(module: &mut MirModule) {
    module.metadata.array_record_packed_autouse_pilot_plans =
        build_array_record_packed_autouse_pilot_plans(module);
}

pub fn build_array_record_packed_autouse_pilot_plans(
    module: &MirModule,
) -> Vec<ArrayRecordPackedAutoUsePilotPlan> {
    module
        .metadata
        .array_record_materialization_boundary_plans
        .iter()
        .filter_map(|boundary| {
            let eligibility = module
                .metadata
                .array_record_autouse_eligibility_plans
                .iter()
                .find(|plan| {
                    plan.layout_id == boundary.layout_id && plan.record_name == boundary.record_name
                })?;
            classify_array_record_packed_autouse_pilot(eligibility, boundary)
        })
        .collect()
}

pub fn classify_array_record_packed_autouse_pilot(
    eligibility: &ArrayRecordAutoUseEligibilityPlan,
    boundary: &ArrayRecordMaterializationBoundaryPlan,
) -> Option<ArrayRecordPackedAutoUsePilotPlan> {
    if !boundary.direct_indexed_field_reads_allowed
        || boundary.visible_record_materialization_enabled
        || boundary.runtime_auto_use_enabled
    {
        return None;
    }

    if eligibility.field_count == 0 || eligibility.integer_lane_columns != eligibility.field_count {
        return None;
    }

    Some(ArrayRecordPackedAutoUsePilotPlan {
        record_name: boundary.record_name.clone(),
        layout_id: boundary.layout_id,
        pilot_kind: ARRAY_RECORD_PACKED_AUTOUSE_PILOT_KIND_INTEGER_LANE_DIRECT_READS_V0.to_string(),
        source_boundary_kind:
            ARRAY_RECORD_MATERIALIZATION_BOUNDARY_KIND_NON_ESCAPING_DIRECT_FIELD_READS_V0
                .to_string(),
        integer_lane_columns: eligibility.integer_lane_columns,
        direct_indexed_field_reads_enabled: true,
        private_runtime_storage_enabled: true,
        public_array_get_materialization_enabled: false,
        hako_alloc_migration_enabled: false,
        backend_lowering_enabled: false,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::array_record_autouse_eligibility::{
        ARRAY_RECORD_AUTOUSE_BACKEND_CAPABILITY, ARRAY_RECORD_AUTOUSE_DECISION_ELIGIBLE,
        ARRAY_RECORD_AUTOUSE_REASON_INTEGER_LANE_NON_ESCAPING_CANDIDATE,
    };
    use crate::mir::array_record_materialization_boundary::{
        ARRAY_RECORD_MATERIALIZATION_ACTION_FAIL_FAST_UNMATERIALIZED_RECORD,
        ARRAY_RECORD_MATERIALIZATION_DIAGNOSTIC_UNMATERIALIZED_RECORD,
    };

    fn eligibility() -> ArrayRecordAutoUseEligibilityPlan {
        ArrayRecordAutoUseEligibilityPlan {
            record_name: "Meta".to_string(),
            layout_id: 11,
            storage_kind: "inline_record_columns_v0".to_string(),
            decision: ARRAY_RECORD_AUTOUSE_DECISION_ELIGIBLE.to_string(),
            reason: ARRAY_RECORD_AUTOUSE_REASON_INTEGER_LANE_NON_ESCAPING_CANDIDATE.to_string(),
            field_count: 2,
            integer_lane_columns: 2,
            required_backend_capability: Some(ARRAY_RECORD_AUTOUSE_BACKEND_CAPABILITY.to_string()),
            production_auto_use_enabled: false,
        }
    }

    fn boundary() -> ArrayRecordMaterializationBoundaryPlan {
        ArrayRecordMaterializationBoundaryPlan {
            record_name: "Meta".to_string(),
            layout_id: 11,
            boundary_kind:
                ARRAY_RECORD_MATERIALIZATION_BOUNDARY_KIND_NON_ESCAPING_DIRECT_FIELD_READS_V0
                    .to_string(),
            source_decision: ARRAY_RECORD_AUTOUSE_DECISION_ELIGIBLE.to_string(),
            direct_indexed_field_reads_allowed: true,
            visible_record_materialization_enabled: false,
            public_array_get_action:
                ARRAY_RECORD_MATERIALIZATION_ACTION_FAIL_FAST_UNMATERIALIZED_RECORD.to_string(),
            returned_element_action:
                ARRAY_RECORD_MATERIALIZATION_ACTION_FAIL_FAST_UNMATERIALIZED_RECORD.to_string(),
            host_backend_escape_action:
                ARRAY_RECORD_MATERIALIZATION_ACTION_FAIL_FAST_UNMATERIALIZED_RECORD.to_string(),
            diagnostic: ARRAY_RECORD_MATERIALIZATION_DIAGNOSTIC_UNMATERIALIZED_RECORD.to_string(),
            runtime_auto_use_enabled: false,
        }
    }

    #[test]
    fn packed_autouse_pilot_consumes_eligible_boundary_rows() {
        let mut module = MirModule::new("packed-autouse-pilot-test".to_string());
        module
            .metadata
            .array_record_autouse_eligibility_plans
            .push(eligibility());
        module
            .metadata
            .array_record_materialization_boundary_plans
            .push(boundary());

        let plans = build_array_record_packed_autouse_pilot_plans(&module);

        assert_eq!(plans.len(), 1);
        assert_eq!(plans[0].record_name, "Meta");
        assert_eq!(plans[0].layout_id, 11);
        assert_eq!(
            plans[0].pilot_kind,
            ARRAY_RECORD_PACKED_AUTOUSE_PILOT_KIND_INTEGER_LANE_DIRECT_READS_V0
        );
        assert_eq!(plans[0].integer_lane_columns, 2);
        assert!(plans[0].direct_indexed_field_reads_enabled);
        assert!(plans[0].private_runtime_storage_enabled);
        assert!(!plans[0].public_array_get_materialization_enabled);
        assert!(!plans[0].hako_alloc_migration_enabled);
        assert!(!plans[0].backend_lowering_enabled);
    }

    #[test]
    fn packed_autouse_pilot_rejects_materializing_boundaries() {
        let mut eligibility = eligibility();
        eligibility.integer_lane_columns = 2;
        let mut boundary = boundary();
        boundary.visible_record_materialization_enabled = true;

        assert!(classify_array_record_packed_autouse_pilot(&eligibility, &boundary).is_none());
    }
}
