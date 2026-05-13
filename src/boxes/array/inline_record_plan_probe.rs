use super::*;
use crate::mir::array_record_storage_plan::ARRAY_RECORD_STORAGE_KIND_INLINE_RECORD_COLUMNS_V0;
use crate::mir::function::ArrayRecordStoragePlan;

pub(super) struct ArrayInlineRecordPlanProbe;

impl ArrayInlineRecordPlanProbe {
    pub(super) fn build_integer_lane_array(
        plan: &ArrayRecordStoragePlan,
        values_by_column: Vec<Vec<i64>>,
    ) -> Option<ArrayBox> {
        if plan.storage_kind != ARRAY_RECORD_STORAGE_KIND_INLINE_RECORD_COLUMNS_V0 {
            return None;
        }
        if plan.field_count as usize != plan.columns.len() {
            return None;
        }
        if plan.columns.len() != values_by_column.len() {
            return None;
        }

        let mut columns = Vec::with_capacity(plan.columns.len());
        for (index, column_plan) in plan.columns.iter().enumerate() {
            if column_plan.column as usize != index {
                return None;
            }
            if !column_plan.storage.uses_integer_lane() {
                return None;
            }
            columns.push(ArrayInlineRecordColumn::i64(
                values_by_column[index].clone(),
            ));
        }

        ArrayInlineRecordProbe::build(plan.layout_id, columns)
    }
}
