//! Common private row dispatcher for prepared Loop operation families.
//!
//! This module only joins the three existing leaf service boundaries:
//! pure operations, canonical BindingSSA reads, and canonical assignments.
//! It owns no Recipe, full schedule, CFG, SSA, PHI, Completion, or
//! publication state. The complete Recipe-order `prepare/emit_all` wrapper
//! remains a later caller-zero seam.

use super::operation_emitter::{
    emit_prepared_pure_operation_v1, emit_prepared_read_binding_v1, emit_prepared_write_binding_v1,
    CanonicalBindingReadServicesV1, LoopOperationEmissionReceiptV1, LoopOperationEmissionRejectV1,
    LoopOperationServicesV1, LoopOperationValueStateV1, LoopReadBindingEmissionRejectV1,
    LoopWriteBindingEmissionRejectV1, PreparedLoopOperationEmissionV1,
    PreparedLoopReadBindingEmissionV1, PreparedLoopWriteBindingEmissionV1,
    ReadBindingEmissionReceiptV1, WriteBindingEmissionReceiptV1,
};
use super::topology::{LoopPhysicalBlockReceiptV1, ReadyLoopEntryV1};
use crate::mir::builder::emission::phi_lifecycle::PhiTxn;
use crate::mir::builder::resolved_lowering::canonical_ssa::ResolvedSsaIdentityStateV2;
use crate::mir::builder::MirBuilder;
use crate::mir::loop_recipe_contract::LoopValueKeyV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum PreparedLoopOperationDispatchV1 {
    Pure(PreparedLoopOperationEmissionV1),
    Read(PreparedLoopReadBindingEmissionV1),
    Write(PreparedLoopWriteBindingEmissionV1),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum LoopOperationDispatchReceiptV1 {
    Pure(LoopOperationEmissionReceiptV1),
    Read(ReadBindingEmissionReceiptV1),
    Write(WriteBindingEmissionReceiptV1),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum LoopOperationDispatchRejectV1 {
    Pure(LoopOperationEmissionRejectV1),
    Read(LoopReadBindingEmissionRejectV1),
    Write(LoopWriteBindingEmissionRejectV1),
    ValueAlreadyPublished(LoopValueKeyV1),
}

/// Borrowed canonical services for one complete operation schedule.
///
/// The dispatcher sequences borrows of the existing pure/identity service
/// bundles. It is not a new physical or SSA owner.
pub(super) struct LoopOperationDispatchServicesV1<'a, 'source> {
    pub(super) builder: &'a mut MirBuilder,
    pub(super) identity: &'a mut ResolvedSsaIdentityStateV2<'source>,
    pub(super) phis: &'a mut PhiTxn,
}

impl<'a, 'source> LoopOperationDispatchServicesV1<'a, 'source> {
    pub(super) fn new(
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

pub(super) fn emit_prepared_operation_family_v1<'source>(
    prepared: PreparedLoopOperationDispatchV1,
    state: &mut LoopOperationValueStateV1,
    entry: &ReadyLoopEntryV1,
    block_receipt: &LoopPhysicalBlockReceiptV1,
    services: &mut LoopOperationDispatchServicesV1<'_, 'source>,
) -> Result<LoopOperationDispatchReceiptV1, LoopOperationDispatchRejectV1> {
    match prepared {
        PreparedLoopOperationDispatchV1::Pure(prepared) => {
            let mut pure = LoopOperationServicesV1::new(services.builder);
            emit_prepared_pure_operation_v1(prepared, state, entry, block_receipt, &mut pure)
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
            let receipt =
                emit_prepared_read_binding_v1(&prepared, entry, block_receipt, &mut identity)
                    .map_err(LoopOperationDispatchRejectV1::Read)?;
            state
                .insert(receipt.result(), receipt.physical_value())
                .map_err(|_| {
                    LoopOperationDispatchRejectV1::ValueAlreadyPublished(receipt.result())
                })?;
            Ok(LoopOperationDispatchReceiptV1::Read(receipt))
        }
        PreparedLoopOperationDispatchV1::Write(prepared) => {
            let mut identity = CanonicalBindingReadServicesV1 {
                builder: services.builder,
                identity: services.identity,
                phis: services.phis,
            };
            emit_prepared_write_binding_v1(&prepared, state, entry, block_receipt, &mut identity)
                .map(LoopOperationDispatchReceiptV1::Write)
                .map_err(LoopOperationDispatchRejectV1::Write)
        }
    }
}
