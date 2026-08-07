//! Common private row dispatcher for prepared Loop operation families.
//!
//! This module only joins the three existing leaf service boundaries:
//! pure operations, canonical BindingSSA reads, and canonical assignments.
//! It owns no Recipe, full schedule, CFG, SSA, PHI, Completion, or
//! publication state. Recipe-order prepare is Builder-free; physical target
//! validation and `emit_all` remain the only execution boundary here.

use super::operation_emitter::{
    emit_prepared_pure_operation_at_target_v1, emit_prepared_pure_operation_v1,
    emit_prepared_read_binding_at_target_v1, emit_prepared_read_binding_v1,
    emit_prepared_write_binding_at_target_v1, emit_prepared_write_binding_v1,
    CanonicalBindingReadServicesV1, LoopOperationEmissionReceiptV1, LoopOperationEmissionRejectV1,
    LoopOperationServicesV1, LoopReadBindingEmissionRejectV1, LoopReadEntryRequirementV1,
    LoopWriteBindingEmissionRejectV1, PreparedLoopOperationEmissionV1,
    PreparedLoopReadBindingEmissionV1, PreparedLoopWriteBindingEmissionV1,
    ReadBindingEmissionReceiptV1, WriteBindingEmissionReceiptV1,
};
use super::operation_ledger::{LoopOperationValueLedgerV1, LoopOperationValueReceiptV1};
use super::operation_target::{LoopOperationTargetRejectV1, VerifiedLoopOperationTargetBlockV1};
use super::topology::{LoopPhysicalBlockReceiptV1, ReadyLoopEntryV1};
use crate::mir::builder::emission::phi_lifecycle::PhiTxn;
use crate::mir::builder::resolved_lowering::canonical_ssa::ResolvedSsaIdentityStateV2;
use crate::mir::builder::MirBuilder;
use crate::mir::loop_recipe_contract::{
    LoopOperationPhysicalDemandRejectV1, LoopOperationV1, LoopValueKeyV1,
    PreparedLoopOperationProgramV1,
};
use std::collections::{BTreeMap, BTreeSet};

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

#[derive(Debug, PartialEq, Eq)]
pub(super) enum LoopOperationDispatchPreflightRejectV1 {
    Target(LoopOperationTargetRejectV1),
    EntryOwnerMismatch,
    ReceiptOwnerMismatch,
    PreheaderMismatch,
    LogicalPlacementMissing {
        item: crate::mir::loop_recipe_contract::LoopItemKeyV1,
    },
    ReadProjectionMissing {
        item: crate::mir::loop_recipe_contract::LoopItemKeyV1,
    },
    WriteProjectionMissing {
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
    Demand(LoopOperationPhysicalDemandRejectV1),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum LoopOperationDispatchPhysicalFailureV1 {
    Target(LoopOperationTargetRejectV1),
    Pure(LoopOperationEmissionRejectV1),
    Read(LoopReadBindingEmissionRejectV1),
    Write(LoopWriteBindingEmissionRejectV1),
    ValueAlreadyPublished(LoopValueKeyV1),
    ReceiptCountMismatch { expected: usize, found: usize },
}

#[derive(Debug)]
pub(super) struct PreparedLoopOperationDispatchPlanV1 {
    program: PreparedLoopOperationProgramV1,
    entry: ReadyLoopEntryV1,
    block_receipt: LoopPhysicalBlockReceiptV1,
    rows: Box<[PreparedLoopOperationDispatchV1]>,
    targets: Box<[VerifiedLoopOperationTargetBlockV1]>,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct CompletedLoopOperationDispatchV1 {
    operation_count: usize,
    receipts: Box<[LoopOperationDispatchReceiptV1]>,
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
            LoopOperationDispatchReceiptV1::Write(_) => false,
        })
    }
}

impl PreparedLoopOperationDispatchPlanV1 {
    pub(super) fn operation_count(&self) -> usize {
        self.program.coverage().operation_count()
    }

    pub(super) fn rows(&self) -> &[PreparedLoopOperationDispatchV1] {
        &self.rows
    }

    pub(super) fn validate_targets(
        &self,
        builder: &MirBuilder,
    ) -> Result<(), LoopOperationDispatchPhysicalFailureV1> {
        self.targets
            .iter()
            .copied()
            .try_for_each(|target| {
                target
                    .validate_function(builder)
                    .map_err(LoopOperationDispatchPhysicalFailureV1::Target)
            })
    }

    pub(super) fn emit_all<'source>(
        self,
        state: &mut LoopOperationValueLedgerV1,
        services: &mut LoopOperationDispatchServicesV1<'_, 'source>,
    ) -> Result<CompletedLoopOperationDispatchV1, LoopOperationDispatchPhysicalFailureV1> {
        self.validate_targets(services.builder)?;
        let Self {
            program,
            entry,
            block_receipt,
            rows,
            targets,
        } = self;
        let operation_count = program.coverage().operation_count();
        // Validate every physical target before the first leaf can mutate MIR.
        // Later target failures must not become a partial-emission surprise;
        // the outer unpublished function session remains the sole discard
        // boundary after this read-only phase.
        let mut receipts = Vec::with_capacity(rows.len());
        for (row, target) in rows.iter().zip(targets.iter()) {
            let row = row.clone();
            let target = *target;
            let receipt =
                emit_prepared_operation_family_at_target_v1(row, target, state, &entry, services)
                    .map_err(|reject| match reject {
                    LoopOperationDispatchRejectV1::Pure(error) => {
                        LoopOperationDispatchPhysicalFailureV1::Pure(error)
                    }
                    LoopOperationDispatchRejectV1::Read(error) => {
                        LoopOperationDispatchPhysicalFailureV1::Read(error)
                    }
                    LoopOperationDispatchRejectV1::Write(error) => {
                        LoopOperationDispatchPhysicalFailureV1::Write(error)
                    }
                    LoopOperationDispatchRejectV1::ValueAlreadyPublished(key) => {
                        LoopOperationDispatchPhysicalFailureV1::ValueAlreadyPublished(key)
                    }
                })?;
            receipts.push(receipt);
        }
        if receipts.len() != operation_count {
            return Err(
                LoopOperationDispatchPhysicalFailureV1::ReceiptCountMismatch {
                    expected: operation_count,
                    found: receipts.len(),
                },
            );
        }
        Ok(CompletedLoopOperationDispatchV1 {
            operation_count,
            receipts: receipts.into_boxed_slice(),
        })
    }
}

/// Prepare the complete Recipe-order operation schedule without Builder
/// effects. All producers and operands are checked before `emit_all` can
/// borrow canonical physical services.
pub(super) fn prepare_loop_operation_dispatch_v1(
    program: PreparedLoopOperationProgramV1,
    entry: ReadyLoopEntryV1,
    block_receipt: LoopPhysicalBlockReceiptV1,
) -> Result<PreparedLoopOperationDispatchPlanV1, LoopOperationDispatchPreflightRejectV1> {
    let owner = program.demand().context().owner();
    if entry.owner() != owner {
        return Err(LoopOperationDispatchPreflightRejectV1::EntryOwnerMismatch);
    }
    if block_receipt.owner() != owner {
        return Err(LoopOperationDispatchPreflightRejectV1::ReceiptOwnerMismatch);
    }
    if block_receipt.preheader() != entry.preheader() {
        return Err(LoopOperationDispatchPreflightRejectV1::PreheaderMismatch);
    }

    let read_rows_source = program
        .read_binding_rows()
        .map_err(LoopOperationDispatchPreflightRejectV1::Demand)?;
    let read_rows = IntoIterator::into_iter(read_rows_source)
        .map(|row| (row.item(), row))
        .collect::<BTreeMap<_, _>>();
    let write_rows_source = program
        .write_binding_rows()
        .map_err(LoopOperationDispatchPreflightRejectV1::Demand)?;
    let write_rows = IntoIterator::into_iter(write_rows_source)
        .map(|row| (row.item(), row))
        .collect::<BTreeMap<_, _>>();
    let operation_rows = program.operation_rows();
    let mut produced = BTreeSet::new();
    let mut available = BTreeSet::new();
    let mut rows = Vec::with_capacity(operation_rows.len());

    for row in operation_rows.iter().copied() {
        let role = block_receipt
            .role_for_logical(row.owner_loop(), row.block())
            .ok_or(
                LoopOperationDispatchPreflightRejectV1::LogicalPlacementMissing {
                    item: row.item(),
                },
            )?;
        let prepared = match row.operation() {
            LoopOperationV1::ReadBinding { result, .. } => {
                if !produced.insert(result) {
                    return Err(
                        LoopOperationDispatchPreflightRejectV1::DuplicateProducedValue(result),
                    );
                }
                let source = read_rows.get(&row.item()).ok_or(
                    LoopOperationDispatchPreflightRejectV1::ReadProjectionMissing {
                        item: row.item(),
                    },
                )?;
                available.insert(result);
                PreparedLoopOperationDispatchV1::Read(PreparedLoopReadBindingEmissionV1::from_row(
                    owner,
                    source,
                    role,
                    LoopReadEntryRequirementV1::CanonicalLive,
                ))
            }
            LoopOperationV1::ConstI64 { result, .. } => {
                if !produced.insert(result) {
                    return Err(
                        LoopOperationDispatchPreflightRejectV1::DuplicateProducedValue(result),
                    );
                }
                available.insert(result);
                PreparedLoopOperationDispatchV1::Pure(
                    PreparedLoopOperationEmissionV1::from_operation(
                        owner,
                        row.item(),
                        row.operation(),
                        row.owner_loop(),
                        row.block(),
                        role,
                    ),
                )
            }
            LoopOperationV1::BinaryI64 {
                left,
                right,
                result,
                ..
            }
            | LoopOperationV1::CompareI64 {
                left,
                right,
                result,
                ..
            } => {
                for value in [left, right] {
                    if !available.contains(&value) {
                        return Err(LoopOperationDispatchPreflightRejectV1::MissingOperand {
                            item: row.item(),
                            value,
                        });
                    }
                }
                if !produced.insert(result) {
                    return Err(
                        LoopOperationDispatchPreflightRejectV1::DuplicateProducedValue(result),
                    );
                }
                available.insert(result);
                PreparedLoopOperationDispatchV1::Pure(
                    PreparedLoopOperationEmissionV1::from_operation(
                        owner,
                        row.item(),
                        row.operation(),
                        row.owner_loop(),
                        row.block(),
                        role,
                    ),
                )
            }
            LoopOperationV1::WriteBinding { value, .. } => {
                if !available.contains(&value) {
                    return Err(LoopOperationDispatchPreflightRejectV1::MissingOperand {
                        item: row.item(),
                        value,
                    });
                }
                let source = write_rows.get(&row.item()).ok_or(
                    LoopOperationDispatchPreflightRejectV1::WriteProjectionMissing {
                        item: row.item(),
                    },
                )?;
                PreparedLoopOperationDispatchV1::Write(
                    PreparedLoopWriteBindingEmissionV1::from_row(owner, source, role),
                )
            }
        };
        rows.push(prepared);
    }

    if rows.len() != program.coverage().operation_count() {
        return Err(
            LoopOperationDispatchPreflightRejectV1::ScheduleCountMismatch {
                expected: program.coverage().operation_count(),
                found: rows.len(),
            },
        );
    }
    let targets = rows
        .iter()
        .map(|row| issue_target_for_row(row, &entry, &block_receipt))
        .collect::<Result<Box<[_]>, _>>()
        .map_err(LoopOperationDispatchPreflightRejectV1::Target)?;
    Ok(PreparedLoopOperationDispatchPlanV1 {
        program,
        entry,
        block_receipt,
        rows: rows.into_boxed_slice(),
        targets,
    })
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

fn issue_target_for_row(
    row: &PreparedLoopOperationDispatchV1,
    entry: &ReadyLoopEntryV1,
    block_receipt: &LoopPhysicalBlockReceiptV1,
) -> Result<VerifiedLoopOperationTargetBlockV1, LoopOperationTargetRejectV1> {
    match row {
        PreparedLoopOperationDispatchV1::Pure(prepared) => {
            VerifiedLoopOperationTargetBlockV1::issue(
                prepared.owner(),
                prepared.item(),
                prepared.expected_loop(),
                prepared.expected_block(),
                prepared.expected_role(),
                entry,
                block_receipt,
            )
        }
        PreparedLoopOperationDispatchV1::Read(prepared) => {
            VerifiedLoopOperationTargetBlockV1::issue(
                prepared.owner(),
                prepared.item(),
                prepared.expected_loop(),
                prepared.logical_block(),
                prepared.expected_role(),
                entry,
                block_receipt,
            )
        }
        PreparedLoopOperationDispatchV1::Write(prepared) => {
            VerifiedLoopOperationTargetBlockV1::issue(
                prepared.owner(),
                prepared.item(),
                prepared.expected_loop(),
                prepared.logical_block(),
                prepared.expected_role(),
                entry,
                block_receipt,
            )
        }
    }
}

fn emit_prepared_operation_family_at_target_v1<'source>(
    prepared: PreparedLoopOperationDispatchV1,
    target: VerifiedLoopOperationTargetBlockV1,
    state: &mut LoopOperationValueLedgerV1,
    entry: &ReadyLoopEntryV1,
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
            let receipt =
                emit_prepared_read_binding_at_target_v1(&prepared, target, entry, &mut identity)
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

pub(super) fn emit_prepared_operation_family_v1<'source>(
    prepared: PreparedLoopOperationDispatchV1,
    state: &mut LoopOperationValueLedgerV1,
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
