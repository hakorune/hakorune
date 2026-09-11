//! Common private row dispatcher for prepared Loop operation families.
//!
//! This module only joins the three existing leaf service boundaries:
//! pure operations, canonical BindingSSA reads, and canonical assignments.
//! It owns no Recipe, full schedule, CFG, SSA, PHI, Completion, or
//! publication state. Segment-order preparation is Builder-free; physical
//! target validation and `emit_all` remain the only execution boundary here.

use super::carrier_emitter::{
    emit_prepared_carrier_seed_at_target_v1, DerivedCarrierSeedEmissionReceiptV1,
    LoopDerivedCarrierSeedEmissionRejectV1, PreparedLoopDerivedCarrierSeedEmissionV1,
};
use super::operation_emitter::{
    emit_prepared_pure_operation_at_target_v1, emit_prepared_read_binding_at_target_v1,
    emit_prepared_write_binding_at_target_v1, CanonicalBindingReadServicesV1,
    LoopOperationEmissionReceiptV1, LoopOperationEmissionRejectV1, LoopOperationServicesV1,
    LoopReadBindingEmissionRejectV1, LoopWriteBindingEmissionRejectV1,
    PreparedLoopOperationEmissionV1, PreparedLoopReadBindingEmissionV1,
    PreparedLoopWriteBindingEmissionV1, ReadBindingEmissionReceiptV1,
    WriteBindingEmissionReceiptV1,
};
use super::operation_ledger::{LoopOperationValueLedgerV1, LoopOperationValueReceiptV1};
use super::operation_target::{LoopOperationTargetRejectV1, VerifiedLoopOperationTargetBlockV1};
use crate::mir::builder::emission::phi_lifecycle::PhiTxn;
use crate::mir::builder::resolved_lowering::canonical_ssa::ResolvedSsaIdentityStateV2;
use crate::mir::builder::MirBuilder;
use crate::mir::loop_recipe_contract::LoopValueKeyV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum PreparedLoopOperationDispatchV1 {
    Pure(PreparedLoopOperationEmissionV1),
    Read(PreparedLoopReadBindingEmissionV1),
    CarrierSeed(PreparedLoopDerivedCarrierSeedEmissionV1),
    Write(PreparedLoopWriteBindingEmissionV1),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum LoopOperationDispatchReceiptV1 {
    Pure(LoopOperationEmissionReceiptV1),
    Read(ReadBindingEmissionReceiptV1),
    CarrierSeed(DerivedCarrierSeedEmissionReceiptV1),
    Write(WriteBindingEmissionReceiptV1),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum LoopOperationDispatchRejectV1 {
    Target(LoopOperationTargetRejectV1),
    Pure(LoopOperationEmissionRejectV1),
    Read(LoopReadBindingEmissionRejectV1),
    CarrierSeed(LoopDerivedCarrierSeedEmissionRejectV1),
    Write(LoopWriteBindingEmissionRejectV1),
    ValueAlreadyPublished(LoopValueKeyV1),
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum LoopOperationDispatchPreflightRejectV1 {
    Target(LoopOperationTargetRejectV1),
    EntryOwnerMismatch,
    ReceiptOwnerMismatch,
    PreheaderMismatch,
    ReadProjectionMissing {
        item: crate::mir::loop_recipe_contract::LoopItemKeyV1,
    },
    WriteProjectionMissing {
        item: crate::mir::loop_recipe_contract::LoopItemKeyV1,
    },
    SegmentPlacementMissing {
        item: crate::mir::loop_recipe_contract::LoopItemKeyV1,
    },
    DuplicateProducedValue(LoopValueKeyV1),
    MissingOperand {
        item: crate::mir::loop_recipe_contract::LoopItemKeyV1,
        value: LoopValueKeyV1,
    },
    ScheduleCountMismatch {
        expected: usize,
        found: usize,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum LoopOperationDispatchPhysicalFailureV1 {
    Target(LoopOperationTargetRejectV1),
    Pure(LoopOperationEmissionRejectV1),
    Read(LoopReadBindingEmissionRejectV1),
    CarrierSeed(LoopDerivedCarrierSeedEmissionRejectV1),
    Write(LoopWriteBindingEmissionRejectV1),
    ValueAlreadyPublished(LoopValueKeyV1),
    ReceiptCountMismatch { expected: usize, found: usize },
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct CompletedLoopOperationDispatchV1 {
    pub(super) operation_count: usize,
    pub(super) receipts: Box<[LoopOperationDispatchReceiptV1]>,
}

impl CompletedLoopOperationDispatchV1 {
    pub(super) const fn operation_count(&self) -> usize {
        self.operation_count
    }

    pub(super) fn receipts(&self) -> &[LoopOperationDispatchReceiptV1] {
        &self.receipts
    }

    pub(super) fn contains_result(&self, key: LoopValueKeyV1) -> bool {
        self.receipts.iter().any(|receipt| match receipt {
            LoopOperationDispatchReceiptV1::Pure(receipt) => receipt.result() == key,
            LoopOperationDispatchReceiptV1::Read(receipt) => receipt.result() == key,
            LoopOperationDispatchReceiptV1::CarrierSeed(receipt) => receipt.result() == key,
            LoopOperationDispatchReceiptV1::Write(_) => false,
        })
    }
}

/// Borrowed canonical services for one complete operation schedule.
///
/// The dispatcher sequences borrows of the existing pure/identity service
/// bundles. It is not a new physical or SSA owner.
pub(in crate::mir::builder::resolved_lowering) struct LoopOperationDispatchServicesV1<'a, 'source> {
    pub(super) builder: &'a mut MirBuilder,
    pub(super) identity: &'a mut ResolvedSsaIdentityStateV2<'source>,
    pub(super) phis: &'a mut PhiTxn,
}

impl<'a, 'source> LoopOperationDispatchServicesV1<'a, 'source> {
    pub(in crate::mir::builder::resolved_lowering) fn new(
        builder: &'a mut MirBuilder,
        identity: &'a mut ResolvedSsaIdentityStateV2<'source>,
        phis: &'a mut PhiTxn,
    ) -> Self {
        Self {
            builder,
            identity,
            phis,
        }
    }
}

pub(super) fn emit_prepared_operation_family_at_target_v1<'source>(
    prepared: PreparedLoopOperationDispatchV1,
    target: VerifiedLoopOperationTargetBlockV1,
    state: &mut LoopOperationValueLedgerV1,
    services: &mut LoopOperationDispatchServicesV1<'_, 'source>,
) -> Result<LoopOperationDispatchReceiptV1, LoopOperationDispatchRejectV1> {
    match prepared {
        PreparedLoopOperationDispatchV1::Pure(prepared) => {
            let mut pure = LoopOperationServicesV1::new(services.builder);
            emit_prepared_pure_operation_at_target_v1(prepared, target, state, &mut pure)
                .map(LoopOperationDispatchReceiptV1::Pure)
                .map_err(LoopOperationDispatchRejectV1::Pure)
        }
        PreparedLoopOperationDispatchV1::Read(prepared) => {
            if state.contains(prepared.result()) {
                return Err(LoopOperationDispatchRejectV1::ValueAlreadyPublished(
                    prepared.result(),
                ));
            }
            let mut identity = CanonicalBindingReadServicesV1 {
                builder: services.builder,
                identity: services.identity,
                phis: services.phis,
            };
            let receipt = emit_prepared_read_binding_at_target_v1(&prepared, target, &mut identity)
                .map_err(LoopOperationDispatchRejectV1::Read)?;
            state
                .publish(LoopOperationValueReceiptV1::new(
                    receipt.owner(),
                    receipt.result(),
                    prepared.class(),
                    receipt.item(),
                    receipt.physical_block(),
                    receipt.physical_value(),
                ))
                .map_err(|_| {
                    LoopOperationDispatchRejectV1::ValueAlreadyPublished(receipt.result())
                })?;
            Ok(LoopOperationDispatchReceiptV1::Read(receipt))
        }
        PreparedLoopOperationDispatchV1::CarrierSeed(prepared) => {
            if state.contains(prepared.result()) {
                return Err(LoopOperationDispatchRejectV1::ValueAlreadyPublished(
                    prepared.result(),
                ));
            }
            let class = prepared.class();
            let mut identity = CanonicalBindingReadServicesV1 {
                builder: services.builder,
                identity: services.identity,
                phis: services.phis,
            };
            let receipt = emit_prepared_carrier_seed_at_target_v1(prepared, target, &mut identity)
                .map_err(LoopOperationDispatchRejectV1::CarrierSeed)?;
            state
                .publish(LoopOperationValueReceiptV1::new(
                    receipt.owner(),
                    receipt.result(),
                    class,
                    receipt.item(),
                    receipt.physical_block(),
                    receipt.physical_value(),
                ))
                .map_err(|_| {
                    LoopOperationDispatchRejectV1::ValueAlreadyPublished(receipt.result())
                })?;
            Ok(LoopOperationDispatchReceiptV1::CarrierSeed(receipt))
        }
        PreparedLoopOperationDispatchV1::Write(prepared) => {
            let mut identity = CanonicalBindingReadServicesV1 {
                builder: services.builder,
                identity: services.identity,
                phis: services.phis,
            };
            emit_prepared_write_binding_at_target_v1(&prepared, target, state, &mut identity)
                .map(LoopOperationDispatchReceiptV1::Write)
                .map_err(LoopOperationDispatchRejectV1::Write)
        }
    }
}
