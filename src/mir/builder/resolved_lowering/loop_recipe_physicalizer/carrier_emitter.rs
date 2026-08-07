//! Profile-neutral emission of a derived Loop carrier entry.
//!
//! A derived carrier is a source-statement anchor, not an expression read.
//! This leaf therefore performs only the canonical non-claiming identity read
//! and publishes an immutable physical receipt; it owns no SSA or PHI state.

use super::operation_emitter::CanonicalBindingReadServicesV1;
use super::operation_target::VerifiedLoopOperationTargetBlockV1;
use crate::mir::loop_recipe_contract::PreparedLoopDerivedCarrierSeedRowV1;
use crate::mir::resolved_semantics::{BindingRefV1, FunctionOwnerIdV1};
use crate::mir::{BasicBlockId, MirType, ValueId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct PreparedLoopDerivedCarrierSeedEmissionV1 {
    owner: FunctionOwnerIdV1,
    item: crate::mir::loop_recipe_contract::LoopItemKeyV1,
    binding: BindingRefV1,
    result: crate::mir::loop_recipe_contract::LoopValueKeyV1,
    logical_block: crate::mir::loop_recipe_contract::LoopBlockKeyV1,
    expected_loop: crate::mir::loop_recipe_contract::LoopNodeKeyV1,
    expected_role: super::topology::LoopPhysicalBlockRoleV1,
    class: crate::mir::loop_recipe_contract::LoopValueClassV1,
}

impl PreparedLoopDerivedCarrierSeedEmissionV1 {
    pub(super) fn from_row(
        owner: FunctionOwnerIdV1,
        row: &PreparedLoopDerivedCarrierSeedRowV1,
        expected_role: super::topology::LoopPhysicalBlockRoleV1,
    ) -> Self {
        Self {
            owner,
            item: row.item(),
            binding: row.source_binding(),
            result: row.result(),
            logical_block: row.block(),
            expected_loop: row.owner_loop(),
            expected_role,
            class: row.class(),
        }
    }

    pub(super) const fn owner(self) -> FunctionOwnerIdV1 {
        self.owner
    }
    pub(super) const fn item(self) -> crate::mir::loop_recipe_contract::LoopItemKeyV1 {
        self.item
    }
    pub(super) const fn binding(self) -> BindingRefV1 {
        self.binding
    }
    pub(super) const fn result(self) -> crate::mir::loop_recipe_contract::LoopValueKeyV1 {
        self.result
    }
    pub(super) const fn logical_block(self) -> crate::mir::loop_recipe_contract::LoopBlockKeyV1 {
        self.logical_block
    }
    pub(super) const fn expected_loop(self) -> crate::mir::loop_recipe_contract::LoopNodeKeyV1 {
        self.expected_loop
    }
    pub(super) const fn expected_role(self) -> super::topology::LoopPhysicalBlockRoleV1 {
        self.expected_role
    }
    pub(super) const fn class(self) -> crate::mir::loop_recipe_contract::LoopValueClassV1 {
        self.class
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum LoopDerivedCarrierSeedEmissionRejectV1 {
    SourceBindingMismatch,
    CanonicalRead(String),
    CanonicalReceiptMismatch,
    ResultTypeMismatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct DerivedCarrierSeedEmissionReceiptV1 {
    owner: FunctionOwnerIdV1,
    item: crate::mir::loop_recipe_contract::LoopItemKeyV1,
    binding: BindingRefV1,
    result: crate::mir::loop_recipe_contract::LoopValueKeyV1,
    physical_block: BasicBlockId,
    physical_value: ValueId,
}

impl DerivedCarrierSeedEmissionReceiptV1 {
    pub(super) const fn owner(self) -> FunctionOwnerIdV1 {
        self.owner
    }
    pub(super) const fn item(self) -> crate::mir::loop_recipe_contract::LoopItemKeyV1 {
        self.item
    }
    pub(super) const fn binding(self) -> BindingRefV1 {
        self.binding
    }
    pub(super) const fn result(self) -> crate::mir::loop_recipe_contract::LoopValueKeyV1 {
        self.result
    }
    pub(super) const fn physical_block(self) -> BasicBlockId {
        self.physical_block
    }
    pub(super) const fn physical_value(self) -> ValueId {
        self.physical_value
    }
}

pub(super) fn emit_prepared_carrier_seed_at_target_v1(
    prepared: PreparedLoopDerivedCarrierSeedEmissionV1,
    target: VerifiedLoopOperationTargetBlockV1,
    services: &mut CanonicalBindingReadServicesV1<'_, '_>,
) -> Result<DerivedCarrierSeedEmissionReceiptV1, LoopDerivedCarrierSeedEmissionRejectV1> {
    let block = target.physical_block();
    if prepared.binding.owner() != prepared.owner {
        return Err(LoopDerivedCarrierSeedEmissionRejectV1::SourceBindingMismatch);
    }
    let canonical = services
        .identity
        .read_entry_receipt(services.builder, services.phis, block, prepared.binding)
        .map_err(LoopDerivedCarrierSeedEmissionRejectV1::CanonicalRead)?;
    if canonical.owner() != prepared.owner
        || canonical.binding() != prepared.binding
        || canonical.physical_block() != block
    {
        return Err(LoopDerivedCarrierSeedEmissionRejectV1::CanonicalReceiptMismatch);
    }
    let expected = match prepared.class {
        crate::mir::loop_recipe_contract::LoopValueClassV1::I64 => MirType::Integer,
        crate::mir::loop_recipe_contract::LoopValueClassV1::Bool => MirType::Bool,
        _ => return Err(LoopDerivedCarrierSeedEmissionRejectV1::ResultTypeMismatch),
    };
    if services
        .builder
        .function_state
        .type_ctx
        .get_type(canonical.physical_value())
        != Some(&expected)
    {
        return Err(LoopDerivedCarrierSeedEmissionRejectV1::ResultTypeMismatch);
    }
    Ok(DerivedCarrierSeedEmissionReceiptV1 {
        owner: prepared.owner,
        item: prepared.item,
        binding: prepared.binding,
        result: prepared.result,
        physical_block: block,
        physical_value: canonical.physical_value(),
    })
}
