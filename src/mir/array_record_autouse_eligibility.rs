/*!
 * Packed ArrayBox auto-use eligibility metadata.
 *
 * This owner classifies ArrayRecordStoragePlan rows for a future packed
 * ArrayBox inline-record path. It does not construct runtime storage, rewrite
 * MIR, or make backend lowering decisions.
 */

use crate::mir::array_record_storage_plan::ARRAY_RECORD_STORAGE_KIND_INLINE_RECORD_COLUMNS_V0;
use crate::mir::function::{ArrayRecordAutoUseEligibilityPlan, ArrayRecordStoragePlan};
use crate::mir::MirModule;

pub const ARRAY_RECORD_AUTOUSE_DECISION_ELIGIBLE: &str = "eligible";
pub const ARRAY_RECORD_AUTOUSE_DECISION_REJECTED: &str = "rejected";
pub const ARRAY_RECORD_AUTOUSE_DECISION_FAIL_FAST_REQUIRED: &str = "fail_fast_required";

pub const ARRAY_RECORD_AUTOUSE_REASON_INTEGER_LANE_NON_ESCAPING_CANDIDATE: &str =
    "integer-lane-non-escaping-candidate";
pub const ARRAY_RECORD_AUTOUSE_REASON_UNSUPPORTED_COLUMN_KIND: &str = "unsupported-column-kind";
pub const ARRAY_RECORD_AUTOUSE_REASON_LAYOUT_MISMATCH: &str = "layout-mismatch";
pub const ARRAY_RECORD_AUTOUSE_REASON_UNSUPPORTED_STORAGE_KIND: &str = "unsupported-storage-kind";

pub const ARRAY_RECORD_AUTOUSE_BACKEND_CAPABILITY: &str = "arraybox.inline_record_columns_v0";

pub fn refresh_module_array_record_autouse_eligibility_plans(module: &mut MirModule) {
    module.metadata.array_record_autouse_eligibility_plans =
        build_array_record_autouse_eligibility_plans(module);
}

pub fn build_array_record_autouse_eligibility_plans(
    module: &MirModule,
) -> Vec<ArrayRecordAutoUseEligibilityPlan> {
    module
        .metadata
        .array_record_storage_plans
        .iter()
        .map(classify_array_record_storage_plan)
        .collect()
}

pub fn classify_array_record_storage_plan(
    plan: &ArrayRecordStoragePlan,
) -> ArrayRecordAutoUseEligibilityPlan {
    let integer_lane_columns = plan
        .columns
        .iter()
        .filter(|column| column.storage.uses_integer_lane())
        .count() as u32;

    let (decision, reason, required_backend_capability) =
        if plan.storage_kind != ARRAY_RECORD_STORAGE_KIND_INLINE_RECORD_COLUMNS_V0 {
            (
                ARRAY_RECORD_AUTOUSE_DECISION_REJECTED,
                ARRAY_RECORD_AUTOUSE_REASON_UNSUPPORTED_STORAGE_KIND,
                None,
            )
        } else if plan.field_count as usize != plan.columns.len()
            || !plan
                .columns
                .iter()
                .enumerate()
                .all(|(index, column)| column.column as usize == index)
        {
            (
                ARRAY_RECORD_AUTOUSE_DECISION_REJECTED,
                ARRAY_RECORD_AUTOUSE_REASON_LAYOUT_MISMATCH,
                None,
            )
        } else if integer_lane_columns != plan.field_count {
            (
                ARRAY_RECORD_AUTOUSE_DECISION_REJECTED,
                ARRAY_RECORD_AUTOUSE_REASON_UNSUPPORTED_COLUMN_KIND,
                None,
            )
        } else {
            (
                ARRAY_RECORD_AUTOUSE_DECISION_ELIGIBLE,
                ARRAY_RECORD_AUTOUSE_REASON_INTEGER_LANE_NON_ESCAPING_CANDIDATE,
                Some(ARRAY_RECORD_AUTOUSE_BACKEND_CAPABILITY.to_string()),
            )
        };

    ArrayRecordAutoUseEligibilityPlan {
        record_name: plan.record_name.clone(),
        layout_id: plan.layout_id,
        storage_kind: plan.storage_kind.clone(),
        decision: decision.to_string(),
        reason: reason.to_string(),
        field_count: plan.field_count,
        integer_lane_columns,
        required_backend_capability,
        production_auto_use_enabled: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::function::{
        ArrayRecordStorageColumnPlan, ArrayRecordStoragePlan, TypedObjectFieldStorage,
    };
    use crate::mir::MirModule;

    fn storage_plan(
        record_name: &str,
        columns: Vec<TypedObjectFieldStorage>,
    ) -> ArrayRecordStoragePlan {
        ArrayRecordStoragePlan {
            record_name: record_name.to_string(),
            layout_id: 7,
            storage_kind: ARRAY_RECORD_STORAGE_KIND_INLINE_RECORD_COLUMNS_V0.to_string(),
            field_count: columns.len() as u32,
            columns: columns
                .into_iter()
                .enumerate()
                .map(|(column, storage)| ArrayRecordStorageColumnPlan {
                    name: format!("field_{column}"),
                    column: column as u32,
                    storage,
                })
                .collect(),
        }
    }

    #[test]
    fn build_array_record_autouse_eligibility_accepts_integer_lane_columns() {
        let mut module = MirModule::new("autouse-eligibility-test".to_string());
        module
            .metadata
            .array_record_storage_plans
            .push(storage_plan(
                "Meta",
                vec![TypedObjectFieldStorage::I64, TypedObjectFieldStorage::USize],
            ));

        let plans = build_array_record_autouse_eligibility_plans(&module);

        assert_eq!(plans.len(), 1);
        assert_eq!(plans[0].record_name, "Meta");
        assert_eq!(plans[0].decision, ARRAY_RECORD_AUTOUSE_DECISION_ELIGIBLE);
        assert_eq!(
            plans[0].reason,
            ARRAY_RECORD_AUTOUSE_REASON_INTEGER_LANE_NON_ESCAPING_CANDIDATE
        );
        assert_eq!(plans[0].integer_lane_columns, 2);
        assert_eq!(
            plans[0].required_backend_capability.as_deref(),
            Some(ARRAY_RECORD_AUTOUSE_BACKEND_CAPABILITY)
        );
        assert!(!plans[0].production_auto_use_enabled);
    }

    #[test]
    fn build_array_record_autouse_eligibility_rejects_handle_columns() {
        let plan = storage_plan(
            "Meta",
            vec![
                TypedObjectFieldStorage::I64,
                TypedObjectFieldStorage::Handle,
            ],
        );

        let result = classify_array_record_storage_plan(&plan);

        assert_eq!(result.decision, ARRAY_RECORD_AUTOUSE_DECISION_REJECTED);
        assert_eq!(
            result.reason,
            ARRAY_RECORD_AUTOUSE_REASON_UNSUPPORTED_COLUMN_KIND
        );
        assert_eq!(result.integer_lane_columns, 1);
        assert!(result.required_backend_capability.is_none());
        assert!(!result.production_auto_use_enabled);
    }

    #[test]
    fn build_array_record_autouse_eligibility_rejects_layout_mismatch() {
        let mut plan = storage_plan("Meta", vec![TypedObjectFieldStorage::I64]);
        plan.field_count = 2;

        let result = classify_array_record_storage_plan(&plan);

        assert_eq!(result.decision, ARRAY_RECORD_AUTOUSE_DECISION_REJECTED);
        assert_eq!(result.reason, ARRAY_RECORD_AUTOUSE_REASON_LAYOUT_MISMATCH);
        assert!(!result.production_auto_use_enabled);
    }
}
