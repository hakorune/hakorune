//! Private leaf operation emission for the Loop physicalizer canary.
//!
//! This module consumes only a prepared operation and an exact physical block
//! receipt. It delegates Const emission and type publication to the existing
//! Builder owner; it does not own Recipe, CFG, SSA, PHI, or function lifecycle.

use super::operation_ledger::LoopOperationValueLedgerV1;
use super::operation_target::{LoopOperationTargetRejectV1, VerifiedLoopOperationTargetBlockV1};
use super::operation_type::{ensure_provisional_value_class, expected_mir_type};
pub(super) use super::pure_operation_emitter::{
    emit_prepared_operation_v1, emit_prepared_pure_operation_at_target_v1,
    emit_prepared_pure_operation_v1, LoopOperationEmissionReceiptV1, LoopOperationEmissionRejectV1,
    LoopOperationServicesV1, PreparedLoopOperationEmissionV1,
};
use super::topology::{LoopPhysicalBlockReceiptV1, LoopPhysicalBlockRoleV1, ReadyLoopEntryV1};
use crate::mir::builder::emission::phi_lifecycle::PhiTxn;
use crate::mir::builder::resolved_lowering::canonical_ssa::{
    CanonicalBindingReadReceiptV1, ResolvedSsaIdentityStateV2,
};
use crate::mir::builder::MirBuilder;
use crate::mir::loop_recipe_contract::{
    LoopBlockKeyV1, LoopItemKeyV1, LoopNodeKeyV1, LoopValueClassV1, LoopValueKeyV1,
    PreparedLoopReadBindingRowV1, PreparedLoopWriteBindingRowV1,
};
use crate::mir::resolved_semantics::{BindingRefV1, FunctionOwnerIdV1, SourceExprSiteV1};
use crate::mir::{BasicBlockId, ValueId};

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

    pub(super) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(super) const fn item(&self) -> LoopItemKeyV1 {
        self.item
    }

    pub(super) const fn logical_block(&self) -> LoopBlockKeyV1 {
        self.logical_block
    }

    pub(super) const fn expected_loop(&self) -> LoopNodeKeyV1 {
        self.expected_loop
    }

    pub(super) const fn expected_role(&self) -> LoopPhysicalBlockRoleV1 {
        self.expected_role
    }
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

pub(super) fn map_target_reject(
    error: LoopOperationTargetRejectV1,
) -> LoopOperationEmissionRejectV1 {
    match error {
        LoopOperationTargetRejectV1::EntryOwnerMismatch => {
            LoopOperationEmissionRejectV1::EntryOwnerMismatch
        }
        LoopOperationTargetRejectV1::ReceiptOwnerMismatch => {
            LoopOperationEmissionRejectV1::ReceiptOwnerMismatch
        }
        LoopOperationTargetRejectV1::PreheaderMismatch => {
            LoopOperationEmissionRejectV1::PreheaderMismatch
        }
        LoopOperationTargetRejectV1::PlacementMissing { loop_key, role } => {
            LoopOperationEmissionRejectV1::PlacementMissing { loop_key, role }
        }
        LoopOperationTargetRejectV1::LogicalPlacementMissing { loop_key, block } => {
            LoopOperationEmissionRejectV1::LogicalPlacementMissing { loop_key, block }
        }
        LoopOperationTargetRejectV1::PlacementMismatch {
            by_role,
            by_logical_block,
        } => LoopOperationEmissionRejectV1::PlacementMismatch {
            by_role,
            by_logical_block,
        },
        LoopOperationTargetRejectV1::SegmentPlacementMissing(segment) => {
            LoopOperationEmissionRejectV1::SegmentPlacementMissing(segment)
        }
        LoopOperationTargetRejectV1::TargetFunctionMissing => {
            LoopOperationEmissionRejectV1::TargetFunctionMissing
        }
        LoopOperationTargetRejectV1::PreheaderMissing(block) => {
            LoopOperationEmissionRejectV1::PreheaderMissing(block)
        }
        LoopOperationTargetRejectV1::TargetBlockMissing(block) => {
            LoopOperationEmissionRejectV1::TargetBlockMissing(block)
        }
        LoopOperationTargetRejectV1::TargetBlockTerminated(block) => {
            LoopOperationEmissionRejectV1::TargetBlockTerminated(block)
        }
    }
}

fn issue_target_for_read(
    prepared: &PreparedLoopReadBindingEmissionV1,
    entry: &ReadyLoopEntryV1,
    block_receipt: &LoopPhysicalBlockReceiptV1,
    builder: &MirBuilder,
) -> Result<VerifiedLoopOperationTargetBlockV1, LoopReadBindingEmissionRejectV1> {
    let target = VerifiedLoopOperationTargetBlockV1::issue(
        prepared.owner(),
        prepared.item(),
        prepared.expected_loop(),
        prepared.logical_block(),
        prepared.expected_role(),
        entry,
        block_receipt,
    )
    .map_err(|error| LoopReadBindingEmissionRejectV1::PreClaim(map_target_reject(error)))?;
    target
        .validate_function(builder)
        .map_err(|error| LoopReadBindingEmissionRejectV1::PreClaim(map_target_reject(error)))?;
    Ok(target)
}

fn issue_target_for_write(
    prepared: &PreparedLoopWriteBindingEmissionV1,
    entry: &ReadyLoopEntryV1,
    block_receipt: &LoopPhysicalBlockReceiptV1,
    builder: &MirBuilder,
) -> Result<VerifiedLoopOperationTargetBlockV1, LoopWriteBindingEmissionRejectV1> {
    let target = VerifiedLoopOperationTargetBlockV1::issue(
        prepared.owner(),
        prepared.item(),
        prepared.expected_loop(),
        prepared.logical_block(),
        prepared.expected_role(),
        entry,
        block_receipt,
    )
    .map_err(|error| LoopWriteBindingEmissionRejectV1::PreClaim(map_target_reject(error)))?;
    target
        .validate_function(builder)
        .map_err(|error| LoopWriteBindingEmissionRejectV1::PreClaim(map_target_reject(error)))?;
    Ok(target)
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
    let target = issue_target_for_read(prepared, entry, block_receipt, services.builder)?;
    emit_prepared_read_binding_at_target_v1(prepared, target, entry, services)
}

pub(super) fn emit_prepared_read_binding_at_target_v1(
    prepared: &PreparedLoopReadBindingEmissionV1,
    target: VerifiedLoopOperationTargetBlockV1,
    entry: &ReadyLoopEntryV1,
    services: &mut CanonicalBindingReadServicesV1<'_, '_>,
) -> Result<ReadBindingEmissionReceiptV1, LoopReadBindingEmissionRejectV1> {
    let by_role = target.physical_block();
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
    ensure_provisional_value_class(
        services.builder,
        canonical.physical_value(),
        prepared.class(),
    )
    .map_err(|_| LoopReadBindingEmissionRejectV1::ResultTypeMismatch)?;
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

    pub(super) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(super) const fn item(&self) -> LoopItemKeyV1 {
        self.item
    }

    pub(super) const fn logical_block(&self) -> LoopBlockKeyV1 {
        self.logical_block
    }

    pub(super) const fn expected_loop(&self) -> LoopNodeKeyV1 {
        self.expected_loop
    }

    pub(super) const fn expected_role(&self) -> LoopPhysicalBlockRoleV1 {
        self.expected_role
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
    let target = issue_target_for_write(prepared, entry, block_receipt, services.builder)?;
    emit_prepared_write_binding_at_target_v1(prepared, target, state, services)
}

pub(super) fn emit_prepared_write_binding_at_target_v1(
    prepared: &PreparedLoopWriteBindingEmissionV1,
    target: VerifiedLoopOperationTargetBlockV1,
    state: &LoopOperationValueLedgerV1,
    services: &mut CanonicalBindingReadServicesV1<'_, '_>,
) -> Result<WriteBindingEmissionReceiptV1, LoopWriteBindingEmissionRejectV1> {
    if prepared.source_binding.owner() != prepared.owner {
        return Err(LoopWriteBindingEmissionRejectV1::SourceBindingMismatch);
    }
    let by_role = target.physical_block();
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
