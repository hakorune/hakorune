use crate::mir::fastmem_layout_contract::resolve_fastmem_field_contract;
use crate::mir::instruction::{FastMemRegionId, MemOpAccess};
use crate::mir::{BasicBlockId, ValueId};

use super::types::{
    FastMemAccessPlan, FastMemAccessPlanKind, FastMemAccessPlanPayload, FastMemAccessPlanStatus,
    FastMemFieldAccessMode, FastMemFieldAccessPlan,
};

pub(super) fn field_plan(
    block: BasicBlockId,
    instruction_index: usize,
    region: FastMemRegionId,
    dst: Option<ValueId>,
    operands: &[ValueId],
    access: Option<&MemOpAccess>,
    mode: FastMemFieldAccessMode,
    contract: Option<&str>,
) -> Option<FastMemAccessPlan> {
    let access = access?;
    let field_id = access.field_id.as_ref()?.clone();
    let base = operands.first().copied()?;
    let value = if mode == FastMemFieldAccessMode::Store {
        operands.get(1).copied()
    } else {
        None
    };
    let resolved = contract.map(|contract| {
        resolve_fastmem_field_contract(contract, &field_id, mode).map_err(|err| err.reason())
    });
    let (
        status,
        failure_reason,
        layout_id,
        canonical_field_id,
        byte_offset,
        field_size,
        field_type,
        alignment,
        mutability,
        field_class,
    ) = match resolved {
        Some(Ok(resolved)) => (
            FastMemAccessPlanStatus::Verified,
            None,
            Some(resolved.layout_id),
            resolved.field_id,
            Some(resolved.byte_offset),
            Some(resolved.field_size),
            Some(resolved.field_type),
            Some(resolved.alignment),
            Some(resolved.mutability),
            Some(resolved.field_class),
        ),
        Some(Err(reason)) => (
            FastMemAccessPlanStatus::Rejected,
            Some(reason),
            access.layout_id.clone(),
            field_id,
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        None => (
            FastMemAccessPlanStatus::SymbolicOnly,
            Some("layout-field-contract-unresolved".to_string()),
            access.layout_id.clone(),
            field_id,
            None,
            None,
            None,
            None,
            None,
            None,
        ),
    };
    Some(FastMemAccessPlan {
        block,
        instruction_index,
        region,
        kind: match mode {
            FastMemFieldAccessMode::Load => FastMemAccessPlanKind::FieldLoad,
            FastMemFieldAccessMode::Store => FastMemAccessPlanKind::FieldStore,
        },
        status,
        failure_reason,
        payload: FastMemAccessPlanPayload::Field(FastMemFieldAccessPlan {
            layout_id,
            field_id: canonical_field_id,
            base,
            value,
            result: dst,
            mode,
            byte_offset,
            field_size,
            field_type,
            alignment,
            mutability,
            field_class,
        }),
    })
}
