/*!
 * ArrayBox packed-record storage descriptors.
 *
 * This metadata owner maps concrete record layouts to future columnar ArrayBox
 * residence. It does not mutate ArrayBox runtime storage or install public
 * get/set behavior.
 */

use crate::mir::function::{ArrayRecordStorageColumnPlan, ArrayRecordStoragePlan};
use crate::mir::MirModule;

pub const ARRAY_RECORD_STORAGE_KIND_INLINE_RECORD_COLUMNS_V0: &str = "inline_record_columns_v0";

pub fn refresh_module_array_record_storage_plans(module: &mut MirModule) {
    module.metadata.array_record_storage_plans = build_array_record_storage_plans(module);
}

pub fn build_array_record_storage_plans(module: &MirModule) -> Vec<ArrayRecordStoragePlan> {
    module
        .metadata
        .record_layout_plans
        .iter()
        .map(|layout| ArrayRecordStoragePlan {
            record_name: layout.record_name.clone(),
            layout_id: layout.layout_id,
            storage_kind: ARRAY_RECORD_STORAGE_KIND_INLINE_RECORD_COLUMNS_V0.to_string(),
            field_count: layout.field_count,
            columns: layout
                .fields
                .iter()
                .map(|field| ArrayRecordStorageColumnPlan {
                    name: field.name.clone(),
                    column: field.slot,
                    storage: field.storage,
                })
                .collect(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{MirModule, RecordLayoutFieldPlan, RecordLayoutPlan, TypedObjectFieldStorage};

    #[test]
    fn build_array_record_storage_plans_maps_record_layout_to_columns() {
        let mut module = MirModule::new("array-record-storage-test".to_string());
        module.metadata.record_layout_plans.push(RecordLayoutPlan {
            record_name: "Meta".to_string(),
            layout_id: 3,
            layout_kind: "record_value_aggregate_v0".to_string(),
            field_count: 2,
            fields: vec![
                RecordLayoutFieldPlan {
                    name: "ptr".to_string(),
                    slot: 0,
                    declared_type_name: Some("i64".to_string()),
                    storage: TypedObjectFieldStorage::I64,
                },
                RecordLayoutFieldPlan {
                    name: "size".to_string(),
                    slot: 1,
                    declared_type_name: Some("usize".to_string()),
                    storage: TypedObjectFieldStorage::USize,
                },
            ],
        });

        let plans = build_array_record_storage_plans(&module);

        assert_eq!(plans.len(), 1);
        assert_eq!(plans[0].record_name, "Meta");
        assert_eq!(plans[0].layout_id, 3);
        assert_eq!(
            plans[0].storage_kind,
            ARRAY_RECORD_STORAGE_KIND_INLINE_RECORD_COLUMNS_V0
        );
        assert_eq!(plans[0].field_count, 2);
        assert_eq!(plans[0].columns[0].name, "ptr");
        assert_eq!(plans[0].columns[0].column, 0);
        assert_eq!(plans[0].columns[0].storage.as_str(), "i64");
        assert_eq!(plans[0].columns[1].storage.as_str(), "usize");
    }
}
