//! Private leaf operation emission for the Loop physicalizer canary.
//!
//! This module consumes only a prepared operation and an exact physical block
//! receipt. It delegates Const emission and type publication to the existing
//! Builder owner; it does not own Recipe, CFG, SSA, PHI, or function lifecycle.

use super::operation_ledger::{LoopOperationValueLedgerV1, LoopOperationValueReceiptV1};
use super::topology::{
    LoopPhysicalBlockReceiptV1, LoopPhysicalBlockRoleV1, LoopPhysicalServicesV1, ReadyLoopEntryV1,
};
use crate::mir::builder::emission::constant;
use crate::mir::builder::emission::loop_operation;
use crate::mir::builder::emission::phi_lifecycle::PhiTxn;
use crate::mir::builder::resolved_lowering::canonical_ssa::{
    CanonicalBindingReadReceiptV1, ResolvedSsaIdentityStateV2,
};
use crate::mir::builder::MirBuilder;
use crate::mir::loop_recipe_contract::{
    LoopBinaryI64OpV1, LoopBlockKeyV1, LoopCompareI64OpV1, LoopItemKeyV1, LoopNodeKeyV1,
    LoopOperationV1, LoopValueClassV1, LoopValueKeyV1, PreparedLoopReadBindingRowV1,
    PreparedLoopWriteBindingRowV1,
};
use crate::mir::resolved_semantics::{BindingRefV1, FunctionOwnerIdV1, SourceExprSiteV1};
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum LoopReadEntryRequirementV1 {
    PreheaderSeed,
    CanonicalLive,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct PreparedLoopReadBindingEmissionV1 {
    owner: FunctionOwnerIdV1,
    item: LoopItemKeyV1,
    binding: crate::mir::loop_recipe_contract::LoopBindingKeyV1,
    result: LoopValueKeyV1,
    source_binding: BindingRefV1,
    source_site: SourceExprSiteV1,
    logical_block: LoopBlockKeyV1,
    expected_loop: LoopNodeKeyV1,
    expected_role: LoopPhysicalBlockRoleV1,
    entry_requirement: LoopReadEntryRequirementV1,
    class: LoopValueClassV1,
}

impl PreparedLoopReadBindingEmissionV1 {
    pub(super) fn from_row(
        owner: FunctionOwnerIdV1,
        row: &PreparedLoopReadBindingRowV1,
        expected_role: LoopPhysicalBlockRoleV1,
        entry_requirement: LoopReadEntryRequirementV1,
    ) -> Self {
        Self {
            owner,
            item: row.item(),
            binding: row.binding(),
            result: row.result(),
            source_binding: row.source_binding(),
            source_site: row.source_site().clone(),
            logical_block: row.block(),
            expected_loop: row.owner_loop(),
            expected_role,
            entry_requirement,
            class: row.class(),
        }
    }

    #[cfg(test)]
    pub(super) fn from_row_for_test(
        owner: FunctionOwnerIdV1,
        row: &PreparedLoopReadBindingRowV1,
        expected_role: LoopPhysicalBlockRoleV1,
        entry_requirement: LoopReadEntryRequirementV1,
    ) -> Self {
        Self::from_row(owner, row, expected_role, entry_requirement)
    }

    pub(super) const fn result(&self) -> LoopValueKeyV1 {
        self.result
    }

    pub(super) const fn class(&self) -> LoopValueClassV1 {
        self.class
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
    TargetBlockTerminated(BasicBlockId),
    ValueMissing(LoopValueKeyV1),
    ValueAlreadyPublished(LoopValueKeyV1),
    UnsupportedOperation,
    Emission(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum LoopReadBindingEmissionRejectV1 {
    PreClaim(LoopOperationEmissionRejectV1),
    EntryBindingMissing(BindingRefV1),
    SourceBindingMismatch,
    CanonicalRead(String),
    CanonicalReceiptMismatch,
    ResultTypeMismatch,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct ReadBindingEmissionReceiptV1 {
    owner: FunctionOwnerIdV1,
    item: LoopItemKeyV1,
    binding: BindingRefV1,
    result: LoopValueKeyV1,
    logical_block: LoopBlockKeyV1,
    physical_block: BasicBlockId,
    physical_value: ValueId,
}

impl ReadBindingEmissionReceiptV1 {
    pub(super) const fn owner(self) -> FunctionOwnerIdV1 {
        self.owner
    }
    pub(super) const fn item(self) -> LoopItemKeyV1 {
        self.item
    }
    pub(super) const fn binding(self) -> BindingRefV1 {
        self.binding
    }
    pub(super) const fn result(self) -> LoopValueKeyV1 {
        self.result
    }
    pub(super) const fn logical_block(self) -> LoopBlockKeyV1 {
        self.logical_block
    }
    pub(super) const fn physical_block(self) -> BasicBlockId {
        self.physical_block
    }
    pub(super) const fn physical_value(self) -> ValueId {
        self.physical_value
    }
}

pub(super) struct CanonicalBindingReadServicesV1<'a, 'source> {
    pub(super) builder: &'a mut crate::mir::builder::MirBuilder,
    pub(super) identity: &'a mut ResolvedSsaIdentityStateV2<'source>,
    pub(super) phis: &'a mut PhiTxn,
}

/// Borrowed Builder/type emission services for one pure operation.
///
/// CFG allocation belongs to `LoopPhysicalServicesV1`; operation emission
/// receives an already-issued physical block receipt and needs no CFG owner.
pub(super) struct LoopOperationServicesV1<'a> {
    pub(super) builder: &'a mut MirBuilder,
}

impl<'a> LoopOperationServicesV1<'a> {
    pub(super) fn new(builder: &'a mut MirBuilder) -> Self {
        Self { builder }
    }
}

impl<'a, 'source> CanonicalBindingReadServicesV1<'a, 'source> {
    fn claim_and_read(
        &mut self,
        site: &SourceExprSiteV1,
        binding: BindingRefV1,
        block: BasicBlockId,
    ) -> Result<CanonicalBindingReadReceiptV1, String> {
        self.identity.claim_variable_use_binding(site, binding)?;
        self.identity
            .read_entry_receipt(self.builder, self.phis, block, binding)
    }

    pub(super) fn define_assignment(
        &mut self,
        site: &SourceExprSiteV1,
        binding: BindingRefV1,
        block: BasicBlockId,
        value: ValueId,
    ) -> Result<(), String> {
        self.identity
            .define_assignment_exact(site, binding, block, value)
    }
}

fn expected_mir_type(class: LoopValueClassV1) -> MirType {
    match class {
        LoopValueClassV1::I64 => MirType::Integer,
        LoopValueClassV1::Bool => MirType::Bool,
        LoopValueClassV1::Unit => MirType::Void,
    }
}

pub(super) fn emit_prepared_operation_v1(
    prepared: PreparedLoopOperationEmissionV1,
    entry: &ReadyLoopEntryV1,
    block_receipt: &LoopPhysicalBlockReceiptV1,
    services: &mut LoopPhysicalServicesV1<'_>,
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

/// Emit one prepared pure operation.  Read/Write operations use their own
/// canonical identity adapters; this function only handles Const/Binary/
/// Compare and publishes the resulting value into the caller-owned schedule
/// state.
pub(super) fn emit_prepared_pure_operation_v1(
    prepared: PreparedLoopOperationEmissionV1,
    state: &mut LoopOperationValueLedgerV1,
    entry: &ReadyLoopEntryV1,
    block_receipt: &LoopPhysicalBlockReceiptV1,
    services: &mut LoopOperationServicesV1<'_>,
) -> Result<LoopOperationEmissionReceiptV1, LoopOperationEmissionRejectV1> {
    if entry.owner() != prepared.owner {
        return Err(LoopOperationEmissionRejectV1::EntryOwnerMismatch);
    }
    if block_receipt.owner() != prepared.owner {
        return Err(LoopOperationEmissionRejectV1::ReceiptOwnerMismatch);
    }
    if block_receipt.preheader() != entry.preheader() {
        return Err(LoopOperationEmissionRejectV1::PreheaderMismatch);
    }

    let function = services
        .builder
        .function_state
        .current_function
        .as_ref()
        .ok_or(LoopOperationEmissionRejectV1::TargetFunctionMissing)?;
    if function.get_block(entry.preheader()).is_none() {
        return Err(LoopOperationEmissionRejectV1::PreheaderMissing(
            entry.preheader(),
        ));
    }

    let by_role = block_receipt
        .lookup(prepared.expected_loop, prepared.expected_role)
        .ok_or(LoopOperationEmissionRejectV1::PlacementMissing {
            loop_key: prepared.expected_loop,
            role: prepared.expected_role,
        })?;
    let by_logical = block_receipt
        .lookup_logical(prepared.expected_loop, prepared.expected_block)
        .ok_or(LoopOperationEmissionRejectV1::LogicalPlacementMissing {
            loop_key: prepared.expected_loop,
            block: prepared.expected_block,
        })?;
    if by_role != by_logical {
        return Err(LoopOperationEmissionRejectV1::PlacementMismatch {
            by_role,
            by_logical_block: by_logical,
        });
    }
    let target = function
        .get_block(by_role)
        .ok_or(LoopOperationEmissionRejectV1::TargetBlockMissing(by_role))?;
    if target.terminator.is_some() {
        return Err(LoopOperationEmissionRejectV1::TargetBlockTerminated(
            by_role,
        ));
    }

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

/// Emit one Expr/SourceRead leaf after all source/effect/placement checks.
/// After the canonical claim starts, every error is terminal to the caller's
/// unpublished function session; this leaf never owns that discard.
pub(super) fn emit_prepared_read_binding_v1(
    prepared: &PreparedLoopReadBindingEmissionV1,
    entry: &ReadyLoopEntryV1,
    block_receipt: &LoopPhysicalBlockReceiptV1,
    services: &mut CanonicalBindingReadServicesV1<'_, '_>,
) -> Result<ReadBindingEmissionReceiptV1, LoopReadBindingEmissionRejectV1> {
    let preclaim = |error| LoopReadBindingEmissionRejectV1::PreClaim(error);
    if entry.owner() != prepared.owner {
        return Err(preclaim(LoopOperationEmissionRejectV1::EntryOwnerMismatch));
    }
    if block_receipt.owner() != prepared.owner {
        return Err(preclaim(
            LoopOperationEmissionRejectV1::ReceiptOwnerMismatch,
        ));
    }
    if block_receipt.preheader() != entry.preheader() {
        return Err(preclaim(LoopOperationEmissionRejectV1::PreheaderMismatch));
    }
    let function = services
        .builder
        .function_state
        .current_function
        .as_ref()
        .ok_or_else(|| preclaim(LoopOperationEmissionRejectV1::TargetFunctionMissing))?;
    if function.get_block(entry.preheader()).is_none() {
        return Err(preclaim(LoopOperationEmissionRejectV1::PreheaderMissing(
            entry.preheader(),
        )));
    }
    let by_role = block_receipt
        .lookup(prepared.expected_loop, prepared.expected_role)
        .ok_or_else(|| {
            preclaim(LoopOperationEmissionRejectV1::PlacementMissing {
                loop_key: prepared.expected_loop,
                role: prepared.expected_role,
            })
        })?;
    let by_logical = block_receipt
        .lookup_logical(prepared.expected_loop, prepared.logical_block)
        .ok_or_else(|| {
            preclaim(LoopOperationEmissionRejectV1::LogicalPlacementMissing {
                loop_key: prepared.expected_loop,
                block: prepared.logical_block,
            })
        })?;
    if by_role != by_logical {
        return Err(preclaim(LoopOperationEmissionRejectV1::PlacementMismatch {
            by_role,
            by_logical_block: by_logical,
        }));
    }
    let target = function
        .get_block(by_role)
        .ok_or_else(|| preclaim(LoopOperationEmissionRejectV1::TargetBlockMissing(by_role)))?;
    if target.terminator.is_some() {
        return Err(preclaim(
            LoopOperationEmissionRejectV1::TargetBlockTerminated(by_role),
        ));
    }
    if prepared.source_binding.owner() != prepared.owner {
        return Err(LoopReadBindingEmissionRejectV1::SourceBindingMismatch);
    }
    if matches!(
        prepared.entry_requirement,
        LoopReadEntryRequirementV1::PreheaderSeed
    ) && !entry.contains_binding(prepared.source_binding)
    {
        return Err(LoopReadBindingEmissionRejectV1::EntryBindingMissing(
            prepared.source_binding,
        ));
    }

    let canonical = services
        .claim_and_read(&prepared.source_site, prepared.source_binding, by_role)
        .map_err(LoopReadBindingEmissionRejectV1::CanonicalRead)?;
    if canonical.owner() != prepared.owner
        || canonical.binding() != prepared.source_binding
        || canonical.physical_block() != by_role
    {
        return Err(LoopReadBindingEmissionRejectV1::CanonicalReceiptMismatch);
    }
    if services
        .builder
        .function_state
        .type_ctx
        .get_type(canonical.physical_value())
        != Some(&expected_mir_type(prepared.class))
    {
        return Err(LoopReadBindingEmissionRejectV1::ResultTypeMismatch);
    }
    Ok(ReadBindingEmissionReceiptV1 {
        owner: prepared.owner,
        item: prepared.item,
        binding: prepared.source_binding,
        result: prepared.result,
        logical_block: prepared.logical_block,
        physical_block: by_role,
        physical_value: canonical.physical_value(),
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct PreparedLoopWriteBindingEmissionV1 {
    owner: FunctionOwnerIdV1,
    item: LoopItemKeyV1,
    binding: crate::mir::loop_recipe_contract::LoopBindingKeyV1,
    value: LoopValueKeyV1,
    source_binding: BindingRefV1,
    source_site: SourceExprSiteV1,
    logical_block: LoopBlockKeyV1,
    expected_loop: LoopNodeKeyV1,
    expected_role: LoopPhysicalBlockRoleV1,
    class: LoopValueClassV1,
}

impl PreparedLoopWriteBindingEmissionV1 {
    pub(super) fn from_row(
        owner: FunctionOwnerIdV1,
        row: &PreparedLoopWriteBindingRowV1,
        expected_role: LoopPhysicalBlockRoleV1,
    ) -> Self {
        Self {
            owner,
            item: row.item(),
            binding: row.binding(),
            value: row.value(),
            source_binding: row.source_binding(),
            source_site: row.source_site().clone(),
            logical_block: row.block(),
            expected_loop: row.owner_loop(),
            expected_role,
            class: row.class(),
        }
    }

    #[cfg(test)]
    pub(super) fn from_row_for_test(
        owner: FunctionOwnerIdV1,
        row: &PreparedLoopWriteBindingRowV1,
        expected_role: LoopPhysicalBlockRoleV1,
    ) -> Self {
        Self::from_row(owner, row, expected_role)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum LoopWriteBindingEmissionRejectV1 {
    PreClaim(LoopOperationEmissionRejectV1),
    SourceBindingMismatch,
    ValueMissing(LoopValueKeyV1),
    ResultTypeMismatch,
    CanonicalWrite(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct WriteBindingEmissionReceiptV1 {
    owner: FunctionOwnerIdV1,
    item: LoopItemKeyV1,
    binding: BindingRefV1,
    value: LoopValueKeyV1,
    physical_block: BasicBlockId,
    physical_value: ValueId,
}

impl WriteBindingEmissionReceiptV1 {
    pub(super) const fn owner(self) -> FunctionOwnerIdV1 {
        self.owner
    }
    pub(super) const fn item(self) -> LoopItemKeyV1 {
        self.item
    }
    pub(super) const fn binding(self) -> BindingRefV1 {
        self.binding
    }
    pub(super) const fn value(self) -> LoopValueKeyV1 {
        self.value
    }
    pub(super) const fn physical_block(self) -> BasicBlockId {
        self.physical_block
    }
    pub(super) const fn physical_value(self) -> ValueId {
        self.physical_value
    }
}

/// Emit one source-bound assignment through the canonical identity owner.
/// The value map is only an operation-schedule transport; BindingSSA remains
/// the sole assignment authority.
pub(super) fn emit_prepared_write_binding_v1(
    prepared: &PreparedLoopWriteBindingEmissionV1,
    state: &LoopOperationValueLedgerV1,
    entry: &ReadyLoopEntryV1,
    block_receipt: &LoopPhysicalBlockReceiptV1,
    services: &mut CanonicalBindingReadServicesV1<'_, '_>,
) -> Result<WriteBindingEmissionReceiptV1, LoopWriteBindingEmissionRejectV1> {
    let preclaim = |error| LoopWriteBindingEmissionRejectV1::PreClaim(error);
    if entry.owner() != prepared.owner {
        return Err(preclaim(LoopOperationEmissionRejectV1::EntryOwnerMismatch));
    }
    if block_receipt.owner() != prepared.owner {
        return Err(preclaim(
            LoopOperationEmissionRejectV1::ReceiptOwnerMismatch,
        ));
    }
    if block_receipt.preheader() != entry.preheader() {
        return Err(preclaim(LoopOperationEmissionRejectV1::PreheaderMismatch));
    }
    if prepared.source_binding.owner() != prepared.owner {
        return Err(LoopWriteBindingEmissionRejectV1::SourceBindingMismatch);
    }
    let function = services
        .builder
        .function_state
        .current_function
        .as_ref()
        .ok_or_else(|| preclaim(LoopOperationEmissionRejectV1::TargetFunctionMissing))?;
    if function.get_block(entry.preheader()).is_none() {
        return Err(preclaim(LoopOperationEmissionRejectV1::PreheaderMissing(
            entry.preheader(),
        )));
    }
    let by_role = block_receipt
        .lookup(prepared.expected_loop, prepared.expected_role)
        .ok_or_else(|| {
            preclaim(LoopOperationEmissionRejectV1::PlacementMissing {
                loop_key: prepared.expected_loop,
                role: prepared.expected_role,
            })
        })?;
    let by_logical = block_receipt
        .lookup_logical(prepared.expected_loop, prepared.logical_block)
        .ok_or_else(|| {
            preclaim(LoopOperationEmissionRejectV1::LogicalPlacementMissing {
                loop_key: prepared.expected_loop,
                block: prepared.logical_block,
            })
        })?;
    if by_role != by_logical {
        return Err(preclaim(LoopOperationEmissionRejectV1::PlacementMismatch {
            by_role,
            by_logical_block: by_logical,
        }));
    }
    let target = function
        .get_block(by_role)
        .ok_or_else(|| preclaim(LoopOperationEmissionRejectV1::TargetBlockMissing(by_role)))?;
    if target.terminator.is_some() {
        return Err(preclaim(
            LoopOperationEmissionRejectV1::TargetBlockTerminated(by_role),
        ));
    }
    let physical_value =
        state
            .get(prepared.value)
            .ok_or(LoopWriteBindingEmissionRejectV1::ValueMissing(
                prepared.value,
            ))?;
    if services
        .builder
        .function_state
        .type_ctx
        .get_type(physical_value)
        != Some(&expected_mir_type(prepared.class))
    {
        return Err(LoopWriteBindingEmissionRejectV1::ResultTypeMismatch);
    }
    services
        .define_assignment(
            &prepared.source_site,
            prepared.source_binding,
            by_role,
            physical_value,
        )
        .map_err(LoopWriteBindingEmissionRejectV1::CanonicalWrite)?;
    Ok(WriteBindingEmissionReceiptV1 {
        owner: prepared.owner,
        item: prepared.item,
        binding: prepared.source_binding,
        value: prepared.value,
        physical_block: by_role,
        physical_value,
    })
}
