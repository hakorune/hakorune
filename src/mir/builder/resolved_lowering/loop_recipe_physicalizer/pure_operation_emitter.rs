//! Pure operation leaves for the caller-zero Loop physicalizer.
//!
//! This module owns the existing ConstI64/BinaryI64/CompareI64 emission
//! behavior. It deliberately keeps the pre-S0 result-type and ledger checks;
//! the later canonical Compare row will replace those checks in its own
//! prepared path.

use super::operation_emitter::map_target_reject;
use super::operation_ledger::{LoopOperationValueLedgerV1, LoopOperationValueReceiptV1};
use super::operation_target::{LoopOperationTargetRejectV1, VerifiedLoopOperationTargetBlockV1};
use super::operation_type::{ensure_provisional_value_class, expected_mir_type};
use super::topology::{LoopPhysicalBlockReceiptV1, LoopPhysicalBlockRoleV1, ReadyLoopEntryV1};
use crate::mir::builder::emission::constant;
use crate::mir::builder::emission::loop_operation;
use crate::mir::builder::MirBuilder;
use crate::mir::loop_recipe_contract::{
    LoopBinaryI64OpV1, LoopBlockKeyV1, LoopCompareI64OpV1, LoopItemKeyV1, LoopNodeKeyV1,
    LoopOperationV1, LoopValueClassV1, LoopValueKeyV1,
};
use crate::mir::resolved_semantics::FunctionOwnerIdV1;
use crate::mir::{BasicBlockId, MirType, ValueId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct PreparedLoopOperationEmissionV1 {
    owner: FunctionOwnerIdV1,
    item: LoopItemKeyV1,
    operation: LoopOperationV1,
    expected_loop: LoopNodeKeyV1,
    expected_block: LoopBlockKeyV1,
    expected_role: LoopPhysicalBlockRoleV1,
}

impl PreparedLoopOperationEmissionV1 {
    #[cfg(test)]
    pub(super) const fn const_i64_for_canary(
        owner: FunctionOwnerIdV1,
        item: LoopItemKeyV1,
        expected_loop: LoopNodeKeyV1,
        expected_block: LoopBlockKeyV1,
        expected_role: LoopPhysicalBlockRoleV1,
        result: LoopValueKeyV1,
        value: i64,
    ) -> Self {
        Self {
            owner,
            item,
            operation: LoopOperationV1::ConstI64 { result, value },
            expected_loop,
            expected_block,
            expected_role,
        }
    }

    pub(super) const fn from_operation(
        owner: FunctionOwnerIdV1,
        item: LoopItemKeyV1,
        operation: LoopOperationV1,
        expected_loop: LoopNodeKeyV1,
        expected_block: LoopBlockKeyV1,
        expected_role: LoopPhysicalBlockRoleV1,
    ) -> Self {
        Self {
            owner,
            item,
            operation,
            expected_loop,
            expected_block,
            expected_role,
        }
    }

    #[cfg(test)]
    pub(super) const fn from_operation_for_canary(
        owner: FunctionOwnerIdV1,
        item: LoopItemKeyV1,
        operation: LoopOperationV1,
        expected_loop: LoopNodeKeyV1,
        expected_block: LoopBlockKeyV1,
        expected_role: LoopPhysicalBlockRoleV1,
    ) -> Self {
        Self::from_operation(
            owner,
            item,
            operation,
            expected_loop,
            expected_block,
            expected_role,
        )
    }

    pub(super) const fn owner(self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(super) const fn item(self) -> LoopItemKeyV1 {
        self.item
    }

    pub(super) const fn expected_loop(self) -> LoopNodeKeyV1 {
        self.expected_loop
    }

    pub(super) const fn expected_block(self) -> LoopBlockKeyV1 {
        self.expected_block
    }

    pub(super) const fn expected_role(self) -> LoopPhysicalBlockRoleV1 {
        self.expected_role
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum LoopOperationEmissionRejectV1 {
    EntryOwnerMismatch,
    ReceiptOwnerMismatch,
    PreheaderMismatch,
    TargetFunctionMissing,
    PreheaderMissing(BasicBlockId),
    TargetBlockMissing(BasicBlockId),
    PlacementMissing {
        loop_key: LoopNodeKeyV1,
        role: LoopPhysicalBlockRoleV1,
    },
    LogicalPlacementMissing {
        loop_key: LoopNodeKeyV1,
        block: LoopBlockKeyV1,
    },
    PlacementMismatch {
        by_role: BasicBlockId,
        by_logical_block: BasicBlockId,
    },
    SegmentPlacementMissing(crate::mir::loop_recipe_contract::LoopPhysicalSegmentKeyV1),
    TargetBlockTerminated(BasicBlockId),
    ValueMissing(LoopValueKeyV1),
    ValueAlreadyPublished(LoopValueKeyV1),
    UnsupportedOperation,
    Emission(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct LoopOperationEmissionReceiptV1 {
    owner: FunctionOwnerIdV1,
    item: LoopItemKeyV1,
    result: LoopValueKeyV1,
    physical_block: BasicBlockId,
    physical_value: ValueId,
}

impl LoopOperationEmissionReceiptV1 {
    pub(super) const fn owner(self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(super) const fn item(self) -> LoopItemKeyV1 {
        self.item
    }

    pub(super) const fn result(self) -> LoopValueKeyV1 {
        self.result
    }

    pub(super) const fn physical_block(self) -> BasicBlockId {
        self.physical_block
    }

    pub(super) const fn physical_value(self) -> ValueId {
        self.physical_value
    }
}

pub(super) struct LoopOperationServicesV1<'a> {
    pub(super) builder: &'a mut MirBuilder,
}

impl<'a> LoopOperationServicesV1<'a> {
    pub(super) fn new(builder: &'a mut MirBuilder) -> Self {
        Self { builder }
    }
}

pub(super) fn emit_prepared_operation_v1(
    prepared: PreparedLoopOperationEmissionV1,
    entry: &ReadyLoopEntryV1,
    block_receipt: &LoopPhysicalBlockReceiptV1,
    services: &mut super::topology::LoopPhysicalServicesV1<'_>,
) -> Result<LoopOperationEmissionReceiptV1, LoopOperationEmissionRejectV1> {
    let mut state = LoopOperationValueLedgerV1::default();
    let mut operation_services = LoopOperationServicesV1::new(services.builder);
    emit_prepared_pure_operation_v1(
        prepared,
        &mut state,
        entry,
        block_receipt,
        &mut operation_services,
    )
}

fn issue_target_for_pure(
    prepared: PreparedLoopOperationEmissionV1,
    entry: &ReadyLoopEntryV1,
    block_receipt: &LoopPhysicalBlockReceiptV1,
    builder: &MirBuilder,
) -> Result<VerifiedLoopOperationTargetBlockV1, LoopOperationEmissionRejectV1> {
    let target = VerifiedLoopOperationTargetBlockV1::issue(
        prepared.owner(),
        prepared.item(),
        prepared.expected_loop(),
        prepared.expected_block(),
        prepared.expected_role(),
        entry,
        block_receipt,
    )
    .map_err(map_target_reject)?;
    target
        .validate_function(builder)
        .map_err(map_target_reject)?;
    Ok(target)
}

pub(super) fn emit_prepared_pure_operation_v1(
    prepared: PreparedLoopOperationEmissionV1,
    state: &mut LoopOperationValueLedgerV1,
    entry: &ReadyLoopEntryV1,
    block_receipt: &LoopPhysicalBlockReceiptV1,
    services: &mut LoopOperationServicesV1<'_>,
) -> Result<LoopOperationEmissionReceiptV1, LoopOperationEmissionRejectV1> {
    let target = issue_target_for_pure(prepared, entry, block_receipt, services.builder)?;
    emit_prepared_pure_operation_at_target_v1(prepared, target, state, services)
}

pub(super) fn emit_prepared_pure_operation_at_target_v1(
    prepared: PreparedLoopOperationEmissionV1,
    target: VerifiedLoopOperationTargetBlockV1,
    state: &mut LoopOperationValueLedgerV1,
    services: &mut LoopOperationServicesV1<'_>,
) -> Result<LoopOperationEmissionReceiptV1, LoopOperationEmissionRejectV1> {
    let by_role = target.physical_block();

    let (result, physical_value) = match prepared.operation {
        LoopOperationV1::ConstI64 { result, value } => {
            let value = constant::emit_integer_at(services.builder, by_role, value)
                .map_err(LoopOperationEmissionRejectV1::Emission)?;
            (result, value)
        }
        LoopOperationV1::BinaryI64 {
            op,
            left,
            right,
            result,
        } => {
            let lhs = state
                .get(left)
                .ok_or(LoopOperationEmissionRejectV1::ValueMissing(left))?;
            let rhs = state
                .get(right)
                .ok_or(LoopOperationEmissionRejectV1::ValueMissing(right))?;
            let value = match op {
                LoopBinaryI64OpV1::Add => {
                    loop_operation::emit_add_i64_at(services.builder, by_role, lhs, rhs)
                }
                LoopBinaryI64OpV1::Sub => {
                    loop_operation::emit_sub_i64_at(services.builder, by_role, lhs, rhs)
                }
            }
            .map_err(LoopOperationEmissionRejectV1::Emission)?;
            (result, value)
        }
        LoopOperationV1::CompareI64 {
            op,
            left,
            right,
            result,
        } => {
            let lhs = state
                .get(left)
                .ok_or(LoopOperationEmissionRejectV1::ValueMissing(left))?;
            let rhs = state
                .get(right)
                .ok_or(LoopOperationEmissionRejectV1::ValueMissing(right))?;
            let compare = match op {
                LoopCompareI64OpV1::Less => crate::mir::CompareOp::Lt,
                LoopCompareI64OpV1::LessEqual => crate::mir::CompareOp::Le,
                LoopCompareI64OpV1::Equal => crate::mir::CompareOp::Eq,
            };
            let value =
                loop_operation::emit_compare_i64_at(services.builder, by_role, compare, lhs, rhs)
                    .map_err(LoopOperationEmissionRejectV1::Emission)?;
            (result, value)
        }
        LoopOperationV1::ReadBinding { .. } | LoopOperationV1::WriteBinding { .. } => {
            return Err(LoopOperationEmissionRejectV1::UnsupportedOperation)
        }
    };
    let (expected, class) = match prepared.operation {
        LoopOperationV1::CompareI64 { .. } => (MirType::Bool, LoopValueClassV1::Bool),
        _ => (MirType::Integer, LoopValueClassV1::I64),
    };
    if services
        .builder
        .function_state
        .type_ctx
        .get_type(physical_value)
        != Some(&expected)
    {
        return Err(LoopOperationEmissionRejectV1::Emission(
            "[freeze:contract][loop_operation/result_type]".to_string(),
        ));
    }
    state
        .publish(LoopOperationValueReceiptV1::new(
            prepared.owner,
            result,
            class,
            prepared.item,
            by_role,
            physical_value,
        ))
        .map_err(|error| LoopOperationEmissionRejectV1::ValueAlreadyPublished(error.0))?;
    Ok(LoopOperationEmissionReceiptV1 {
        owner: prepared.owner,
        item: prepared.item,
        result,
        physical_block: by_role,
        physical_value,
    })
}
