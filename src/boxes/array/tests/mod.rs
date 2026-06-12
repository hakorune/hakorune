use super::*;
use crate::mir::array_record_storage_plan::ARRAY_RECORD_STORAGE_KIND_INLINE_RECORD_COLUMNS_V0;
use crate::mir::function::{
    ArrayRecordStorageColumnPlan, ArrayRecordStoragePlan, TypedObjectFieldStorage,
};

fn inline_record_probe_plan(
    layout_id: u32,
    storage: Vec<TypedObjectFieldStorage>,
) -> ArrayRecordStoragePlan {
    ArrayRecordStoragePlan {
        record_name: "ProbeMeta".to_string(),
        layout_id,
        storage_kind: ARRAY_RECORD_STORAGE_KIND_INLINE_RECORD_COLUMNS_V0.to_string(),
        field_count: storage.len() as u32,
        columns: storage
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

fn inline_record_test_array() -> ArrayBox {
    ArrayInlineRecordProbe::build(
        7,
        vec![
            ArrayInlineRecordColumn::i64(vec![10, 20]),
            ArrayInlineRecordColumn::bool_values(vec![true, false]),
            ArrayInlineRecordColumn::f64(vec![1.5, 2.5]),
        ],
    )
    .expect("record columns must have equal row counts")
}

mod clone_identity;
mod combined_region;
mod inline_record;
mod invoke_surface;
mod lane_store;
mod lane_update;
