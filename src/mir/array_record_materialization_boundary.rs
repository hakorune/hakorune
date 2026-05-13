/*!
 * ArrayBox inline-record materialization / escape boundary metadata.
 *
 * This owner consumes C207 eligibility rows and emits a conservative boundary
 * contract for future packed ArrayBox auto-use. It does not materialize record
 * objects, enable runtime storage auto-use, or add backend lowering.
 */

use crate::mir::array_record_autouse_eligibility::ARRAY_RECORD_AUTOUSE_DECISION_ELIGIBLE;
use crate::mir::function::{
    ArrayRecordAutoUseEligibilityPlan, ArrayRecordMaterializationBoundaryPlan,
};
use crate::mir::MirModule;

pub const ARRAY_RECORD_MATERIALIZATION_BOUNDARY_KIND_NON_ESCAPING_DIRECT_FIELD_READS_V0: &str =
    "non_escaping_direct_field_reads_v0";
pub const ARRAY_RECORD_MATERIALIZATION_ACTION_FAIL_FAST_UNMATERIALIZED_RECORD: &str =
    "fail_fast_unmaterialized_record_value";
pub const ARRAY_RECORD_MATERIALIZATION_DIAGNOSTIC_UNMATERIALIZED_RECORD: &str =
    "[array/inline-record/unmaterialized] record value materialization is not enabled";

pub fn refresh_module_array_record_materialization_boundary_plans(module: &mut MirModule) {
    module.metadata.array_record_materialization_boundary_plans =
        build_array_record_materialization_boundary_plans(module);
}

pub fn build_array_record_materialization_boundary_plans(
    module: &MirModule,
) -> Vec<ArrayRecordMaterializationBoundaryPlan> {
    module
        .metadata
        .array_record_autouse_eligibility_plans
        .iter()
        .filter_map(classify_array_record_materialization_boundary)
        .collect()
}

pub fn classify_array_record_materialization_boundary(
    plan: &ArrayRecordAutoUseEligibilityPlan,
) -> Option<ArrayRecordMaterializationBoundaryPlan> {
    if plan.decision != ARRAY_RECORD_AUTOUSE_DECISION_ELIGIBLE {
        return None;
    }

    Some(ArrayRecordMaterializationBoundaryPlan {
        record_name: plan.record_name.clone(),
        layout_id: plan.layout_id,
        boundary_kind:
            ARRAY_RECORD_MATERIALIZATION_BOUNDARY_KIND_NON_ESCAPING_DIRECT_FIELD_READS_V0
                .to_string(),
        source_decision: plan.decision.clone(),
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
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::array_record_autouse_eligibility::{
        ARRAY_RECORD_AUTOUSE_BACKEND_CAPABILITY, ARRAY_RECORD_AUTOUSE_DECISION_REJECTED,
        ARRAY_RECORD_AUTOUSE_REASON_INTEGER_LANE_NON_ESCAPING_CANDIDATE,
        ARRAY_RECORD_AUTOUSE_REASON_UNSUPPORTED_COLUMN_KIND,
    };

    fn eligibility(decision: &str) -> ArrayRecordAutoUseEligibilityPlan {
        ArrayRecordAutoUseEligibilityPlan {
            record_name: "Meta".to_string(),
            layout_id: 9,
            storage_kind: "inline_record_columns_v0".to_string(),
            decision: decision.to_string(),
            reason: if decision == ARRAY_RECORD_AUTOUSE_DECISION_ELIGIBLE {
                ARRAY_RECORD_AUTOUSE_REASON_INTEGER_LANE_NON_ESCAPING_CANDIDATE.to_string()
            } else {
                ARRAY_RECORD_AUTOUSE_REASON_UNSUPPORTED_COLUMN_KIND.to_string()
            },
            field_count: 2,
            integer_lane_columns: 2,
            required_backend_capability: Some(ARRAY_RECORD_AUTOUSE_BACKEND_CAPABILITY.to_string()),
            production_auto_use_enabled: false,
        }
    }

    #[test]
    fn materialization_boundary_keeps_visible_records_fail_fast() {
        let mut module = MirModule::new("materialization-boundary-test".to_string());
        module
            .metadata
            .array_record_autouse_eligibility_plans
            .push(eligibility(ARRAY_RECORD_AUTOUSE_DECISION_ELIGIBLE));

        let plans = build_array_record_materialization_boundary_plans(&module);

        assert_eq!(plans.len(), 1);
        assert_eq!(plans[0].record_name, "Meta");
        assert_eq!(plans[0].layout_id, 9);
        assert_eq!(
            plans[0].boundary_kind,
            ARRAY_RECORD_MATERIALIZATION_BOUNDARY_KIND_NON_ESCAPING_DIRECT_FIELD_READS_V0
        );
        assert!(plans[0].direct_indexed_field_reads_allowed);
        assert!(!plans[0].visible_record_materialization_enabled);
        assert_eq!(
            plans[0].public_array_get_action,
            ARRAY_RECORD_MATERIALIZATION_ACTION_FAIL_FAST_UNMATERIALIZED_RECORD
        );
        assert_eq!(
            plans[0].returned_element_action,
            ARRAY_RECORD_MATERIALIZATION_ACTION_FAIL_FAST_UNMATERIALIZED_RECORD
        );
        assert_eq!(
            plans[0].host_backend_escape_action,
            ARRAY_RECORD_MATERIALIZATION_ACTION_FAIL_FAST_UNMATERIALIZED_RECORD
        );
        assert_eq!(
            plans[0].diagnostic,
            ARRAY_RECORD_MATERIALIZATION_DIAGNOSTIC_UNMATERIALIZED_RECORD
        );
        assert!(!plans[0].runtime_auto_use_enabled);
    }

    #[test]
    fn materialization_boundary_skips_rejected_candidates() {
        let mut module = MirModule::new("materialization-boundary-rejected-test".to_string());
        module
            .metadata
            .array_record_autouse_eligibility_plans
            .push(eligibility(ARRAY_RECORD_AUTOUSE_DECISION_REJECTED));

        let plans = build_array_record_materialization_boundary_plans(&module);

        assert!(plans.is_empty());
    }
}
