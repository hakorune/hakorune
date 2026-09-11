//! Private leaf operation emission for the Loop physicalizer.
//!
//! This module consumes only a prepared operation and an exact segment target.
//! It delegates Const emission and type publication to the existing Builder
//! owner; it does not own Recipe, CFG, SSA, PHI, or function lifecycle.

use super::operation_ledger::LoopOperationValueLedgerV1;
use super::operation_target::VerifiedLoopOperationTargetBlockV1;
use super::operation_type::{ensure_provisional_value_class, expected_mir_type};
pub(super) use super::pure_operation_emitter::{
    emit_prepared_pure_operation_at_target_v1, LoopOperationEmissionReceiptV1,
    LoopOperationEmissionRejectV1, LoopOperationServicesV1, PreparedLoopOperationEmissionV1,
};
use super::topology::LoopPhysicalBlockRoleV1;
use crate::mir::builder::emission::phi_lifecycle::PhiTxn;
use crate::mir::builder::resolved_lowering::canonical_ssa::{
    CanonicalBindingReadReceiptV1, ResolvedSsaIdentityStateV2,
};
use crate::mir::loop_recipe_contract::{
    LoopBlockKeyV1, LoopItemKeyV1, LoopNodeKeyV1, LoopValueClassV1, LoopValueKeyV1,
    PreparedLoopReadBindingRowV1, PreparedLoopWriteBindingRowV1,
};
use crate::mir::resolved_semantics::{BindingRefV1, FunctionOwnerIdV1, SourceExprSiteV1};
use crate::mir::{BasicBlockId, ValueId};

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
    class: LoopValueClassV1,
}

impl PreparedLoopReadBindingEmissionV1 {
    pub(super) fn from_row(
        owner: FunctionOwnerIdV1,
        row: &PreparedLoopReadBindingRowV1,
        expected_role: LoopPhysicalBlockRoleV1,
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
            class: row.class(),
        }
    }

    #[cfg(test)]
    pub(super) fn from_row_for_test(
        owner: FunctionOwnerIdV1,
        row: &PreparedLoopReadBindingRowV1,
        expected_role: LoopPhysicalBlockRoleV1,
    ) -> Self {
        Self::from_row(owner, row, expected_role)
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
    SourceBindingMismatch,
    CanonicalRead(String),
    CanonicalReceiptMismatch,
    ResultTypeMismatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct ReadBindingEmissionReceiptV1 {
    item: LoopItemKeyV1,
    result: LoopValueKeyV1,
    logical_block: LoopBlockKeyV1,
    canonical: CanonicalBindingReadReceiptV1,
}

impl ReadBindingEmissionReceiptV1 {
    pub(super) const fn owner(self) -> FunctionOwnerIdV1 {
        self.canonical.owner()
    }
    pub(super) const fn item(self) -> LoopItemKeyV1 {
        self.item
    }
    pub(super) const fn binding(self) -> BindingRefV1 {
        self.canonical.binding()
    }
    pub(super) const fn result(self) -> LoopValueKeyV1 {
        self.result
    }
    pub(super) const fn logical_block(self) -> LoopBlockKeyV1 {
        self.logical_block
    }
    pub(super) const fn physical_block(self) -> BasicBlockId {
        self.canonical.physical_block()
    }
    pub(super) const fn physical_value(self) -> ValueId {
        self.canonical.physical_value()
    }

    pub(super) const fn canonical(self) -> CanonicalBindingReadReceiptV1 {
        self.canonical
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

pub(super) fn emit_prepared_read_binding_at_target_v1(
    prepared: &PreparedLoopReadBindingEmissionV1,
    target: VerifiedLoopOperationTargetBlockV1,
    services: &mut CanonicalBindingReadServicesV1<'_, '_>,
) -> Result<ReadBindingEmissionReceiptV1, LoopReadBindingEmissionRejectV1> {
    let by_role = target.physical_block();
    if prepared.source_binding.owner() != prepared.owner {
        return Err(LoopReadBindingEmissionRejectV1::SourceBindingMismatch);
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
        item: prepared.item,
        result: prepared.result,
        logical_block: prepared.logical_block,
        canonical,
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
