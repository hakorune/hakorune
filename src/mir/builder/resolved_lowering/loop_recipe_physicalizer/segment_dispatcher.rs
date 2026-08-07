//! Segment-aware Callable operation dispatcher for the R2 cutover.
//!
//! This is a thin orchestration layer over the existing leaf dispatcher.  It
//! consumes the complete R1 physical layout, builds one item-to-segment index
//! from that layout, and issues every target through the exact segment receipt.
//! It owns no CFG, SSA, PHI, or retry state.

use super::carrier_emitter::PreparedLoopDerivedCarrierSeedEmissionV1;
use super::operation_dispatcher::{
    emit_prepared_operation_family_at_target_v1, CompletedLoopOperationDispatchV1,
    LoopOperationDispatchPhysicalFailureV1, LoopOperationDispatchPreflightRejectV1,
    LoopOperationDispatchRejectV1, LoopOperationDispatchServicesV1,
    PreparedLoopOperationDispatchV1,
};
use super::operation_emitter::{
    LoopReadEntryRequirementV1, PreparedLoopOperationEmissionV1, PreparedLoopReadBindingEmissionV1,
    PreparedLoopWriteBindingEmissionV1,
};
use super::operation_ledger::LoopOperationValueLedgerV1;
use super::operation_target::{LoopOperationTargetRejectV1, VerifiedLoopOperationTargetBlockV1};
use super::segment_topology::LoopPhysicalSegmentBlockReceiptV1;
use super::topology::ReadyLoopEntryV1;
use crate::mir::loop_recipe_contract::{
    LoopItemKeyV1, LoopOperationV1, LoopPhysicalSegmentKeyV1, PreparedLoopPhysicalLayoutV1,
};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug)]
pub(super) struct PreparedLoopSegmentOperationDispatchPlanV1 {
    layout: PreparedLoopPhysicalLayoutV1,
    entry: ReadyLoopEntryV1,
    segment_receipt: LoopPhysicalSegmentBlockReceiptV1,
    rows: Box<[PreparedLoopOperationDispatchV1]>,
    targets: Box<[VerifiedLoopOperationTargetBlockV1]>,
}

#[derive(Debug)]
pub(super) struct CompletedLoopSegmentProgramV1 {
    pub(super) layout: PreparedLoopPhysicalLayoutV1,
    pub(super) entry: ReadyLoopEntryV1,
    pub(super) segment_receipt: LoopPhysicalSegmentBlockReceiptV1,
    pub(super) dispatch: CompletedLoopOperationDispatchV1,
    pub(super) values: LoopOperationValueLedgerV1,
}

impl PreparedLoopSegmentOperationDispatchPlanV1 {
    pub(super) fn emit_all<'source>(
        self,
        mut state: LoopOperationValueLedgerV1,
        services: &mut LoopOperationDispatchServicesV1<'_, 'source>,
    ) -> Result<CompletedLoopSegmentProgramV1, LoopOperationDispatchPhysicalFailureV1> {
        self.targets.iter().copied().try_for_each(|target| {
            target
                .validate_function(services.builder)
                .map_err(LoopOperationDispatchPhysicalFailureV1::Target)
        })?;
        let Self {
            layout,
            entry,
            segment_receipt,
            rows,
            targets,
        } = self;
        let operation_count = layout.program().coverage().operation_count();
        let mut receipts = Vec::with_capacity(rows.len());
        for (row, target) in rows.iter().zip(targets.iter()) {
            let receipt = emit_prepared_operation_family_at_target_v1(
                row.clone(),
                *target,
                &mut state,
                &entry,
                services,
            )
            .map_err(map_dispatch_reject)?;
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
        Ok(CompletedLoopSegmentProgramV1 {
            layout,
            entry,
            segment_receipt,
            dispatch: CompletedLoopOperationDispatchV1 {
                operation_count,
                receipts: receipts.into_boxed_slice(),
            },
            values: state,
        })
    }
}

pub(super) fn prepare_loop_segment_operation_dispatch_v1(
    layout: PreparedLoopPhysicalLayoutV1,
    entry: ReadyLoopEntryV1,
    segment_receipt: LoopPhysicalSegmentBlockReceiptV1,
) -> Result<PreparedLoopSegmentOperationDispatchPlanV1, LoopOperationDispatchPreflightRejectV1> {
    let program = layout.program();
    let owner = program.demand().context().owner();
    if entry.owner() != owner {
        return Err(LoopOperationDispatchPreflightRejectV1::EntryOwnerMismatch);
    }
    if segment_receipt.owner() != owner {
        return Err(LoopOperationDispatchPreflightRejectV1::ReceiptOwnerMismatch);
    }
    if segment_receipt.preheader() != entry.preheader() {
        return Err(LoopOperationDispatchPreflightRejectV1::PreheaderMismatch);
    }

    let segments_by_item = segment_index(&layout)?;
    let read_rows_source = program
        .read_binding_rows()
        .map_err(LoopOperationDispatchPreflightRejectV1::Demand)?;
    let read_rows = read_rows_source
        .iter()
        .map(|row| (row.item(), row))
        .collect::<BTreeMap<_, _>>();
    let carrier_rows_source = program
        .derived_carrier_seed_rows()
        .map_err(LoopOperationDispatchPreflightRejectV1::Demand)?;
    let carrier_rows = carrier_rows_source
        .iter()
        .map(|row| (row.item(), row))
        .collect::<BTreeMap<_, _>>();
    let write_rows_source = program
        .write_binding_rows()
        .map_err(LoopOperationDispatchPreflightRejectV1::Demand)?;
    let write_rows = write_rows_source
        .iter()
        .map(|row| (row.item(), row))
        .collect::<BTreeMap<_, _>>();
    let operation_rows = program.operation_rows();
    let mut produced = BTreeSet::new();
    let mut available = BTreeSet::new();
    let mut rows = Vec::with_capacity(operation_rows.len());
    let mut segments = Vec::with_capacity(operation_rows.len());

    for row in operation_rows.iter().copied() {
        let segment = *segments_by_item.get(&row.item()).ok_or(
            LoopOperationDispatchPreflightRejectV1::SegmentPlacementMissing { item: row.item() },
        )?;
        let role =
            segment_receipt
                .role(segment)
                .ok_or(LoopOperationDispatchPreflightRejectV1::Target(
                    LoopOperationTargetRejectV1::SegmentPlacementMissing(segment),
                ))?;
        let prepared = match row.operation() {
            LoopOperationV1::ReadBinding { result, .. } => {
                if !produced.insert(result) {
                    return Err(
                        LoopOperationDispatchPreflightRejectV1::DuplicateProducedValue(result),
                    );
                }
                available.insert(result);
                if let Some(source) = read_rows.get(&row.item()) {
                    PreparedLoopOperationDispatchV1::Read(
                        PreparedLoopReadBindingEmissionV1::from_row(
                            owner,
                            source,
                            role,
                            LoopReadEntryRequirementV1::CanonicalLive,
                        ),
                    )
                } else if let Some(source) = carrier_rows.get(&row.item()) {
                    PreparedLoopOperationDispatchV1::CarrierSeed(
                        PreparedLoopDerivedCarrierSeedEmissionV1::from_row(owner, source, role),
                    )
                } else {
                    return Err(
                        LoopOperationDispatchPreflightRejectV1::ReadProjectionMissing {
                            item: row.item(),
                        },
                    );
                }
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
        segments.push(segment);
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
        .zip(segments.iter().copied())
        .map(|(row, segment)| issue_target_for_segment_row(row, segment, &entry, &segment_receipt))
        .collect::<Result<Box<[_]>, _>>()
        .map_err(LoopOperationDispatchPreflightRejectV1::Target)?;
    Ok(PreparedLoopSegmentOperationDispatchPlanV1 {
        layout,
        entry,
        segment_receipt,
        rows: rows.into_boxed_slice(),
        targets,
    })
}

fn segment_index(
    layout: &PreparedLoopPhysicalLayoutV1,
) -> Result<BTreeMap<LoopItemKeyV1, LoopPhysicalSegmentKeyV1>, LoopOperationDispatchPreflightRejectV1>
{
    let mut index = BTreeMap::new();
    for segment in layout.segments() {
        for &item in segment.operations() {
            if index.insert(item, segment.key()).is_some() {
                return Err(
                    LoopOperationDispatchPreflightRejectV1::SegmentPlacementMissing { item },
                );
            }
        }
    }
    Ok(index)
}

fn issue_target_for_segment_row(
    row: &PreparedLoopOperationDispatchV1,
    segment: LoopPhysicalSegmentKeyV1,
    entry: &ReadyLoopEntryV1,
    receipt: &LoopPhysicalSegmentBlockReceiptV1,
) -> Result<VerifiedLoopOperationTargetBlockV1, LoopOperationTargetRejectV1> {
    let (owner, item) = match row {
        PreparedLoopOperationDispatchV1::Pure(row) => (row.owner(), row.item()),
        PreparedLoopOperationDispatchV1::Read(row) => (row.owner(), row.item()),
        PreparedLoopOperationDispatchV1::CarrierSeed(row) => (row.owner(), row.item()),
        PreparedLoopOperationDispatchV1::Write(row) => (row.owner(), row.item()),
    };
    VerifiedLoopOperationTargetBlockV1::issue_for_segment(owner, item, segment, entry, receipt)
}

fn map_dispatch_reject(
    error: LoopOperationDispatchRejectV1,
) -> LoopOperationDispatchPhysicalFailureV1 {
    match error {
        LoopOperationDispatchRejectV1::Target(error) => {
            LoopOperationDispatchPhysicalFailureV1::Target(error)
        }
        LoopOperationDispatchRejectV1::Pure(error) => {
            LoopOperationDispatchPhysicalFailureV1::Pure(error)
        }
        LoopOperationDispatchRejectV1::Read(error) => {
            LoopOperationDispatchPhysicalFailureV1::Read(error)
        }
        LoopOperationDispatchRejectV1::CarrierSeed(error) => {
            LoopOperationDispatchPhysicalFailureV1::CarrierSeed(error)
        }
        LoopOperationDispatchRejectV1::Write(error) => {
            LoopOperationDispatchPhysicalFailureV1::Write(error)
        }
        LoopOperationDispatchRejectV1::ValueAlreadyPublished(key) => {
            LoopOperationDispatchPhysicalFailureV1::ValueAlreadyPublished(key)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::resolved_lowering::loop_recipe_physicalizer::operation_target::VerifiedLoopOperationTargetBlockV1;
    use crate::mir::builder::resolved_lowering::loop_recipe_physicalizer::segment_topology::LoopPhysicalSegmentBlockRowV1;
    use crate::mir::builder::resolved_lowering::loop_recipe_physicalizer::topology::{
        LoopPhysicalBlockRoleV1, ReadyLoopEntryV1,
    };
    use crate::mir::compiler::callable_single_loop_operation_effect::callable_operation_demand_parts_for_test;
    use crate::mir::loop_recipe_contract::VerifiedLoopOperationPhysicalDemandV1;
    use crate::mir::resolved_semantics::FunctionOwnerIssuerV1;
    use crate::mir::BasicBlockId;

    fn callable_layout() -> PreparedLoopPhysicalLayoutV1 {
        let (effect, context, continuation) = callable_operation_demand_parts_for_test();
        VerifiedLoopOperationPhysicalDemandV1::issue(context, effect, continuation)
            .expect("callable demand")
            .prepare_all()
            .expect("callable program")
            .prepare_physical_layout()
            .expect("callable layout")
    }

    fn receipt(
        layout: &PreparedLoopPhysicalLayoutV1,
        count: usize,
    ) -> LoopPhysicalSegmentBlockReceiptV1 {
        let rows = layout
            .segments()
            .iter()
            .take(count)
            .enumerate()
            .map(|(index, segment)| {
                LoopPhysicalSegmentBlockRowV1::new(
                    segment.key(),
                    if index == 0 {
                        LoopPhysicalBlockRoleV1::Header
                    } else {
                        LoopPhysicalBlockRoleV1::Body
                    },
                    BasicBlockId::new(index as u32 + 1),
                )
            })
            .collect();
        LoopPhysicalSegmentBlockReceiptV1::issue(
            layout.program().demand().context().owner(),
            BasicBlockId::new(0),
            &layout
                .segments()
                .iter()
                .take(count)
                .map(|segment| segment.key())
                .collect::<Vec<_>>(),
            rows,
        )
        .expect("test segment receipt")
    }

    #[test]
    fn segment_target_rejects_missing_exact_segment() {
        let layout = callable_layout();
        let segments = layout.segments();
        let receipt = receipt(&layout, 1);
        let entry = ReadyLoopEntryV1::new_for_test(
            layout.program().demand().context().owner(),
            BasicBlockId::new(0),
            Vec::new(),
        );
        let error = VerifiedLoopOperationTargetBlockV1::issue_for_segment(
            layout.program().demand().context().owner(),
            crate::mir::loop_recipe_contract::LoopItemKeyV1::new(1),
            segments[1].key(),
            &entry,
            &receipt,
        )
        .expect_err("missing exact segment must reject");
        assert_eq!(
            error,
            LoopOperationTargetRejectV1::SegmentPlacementMissing(segments[1].key())
        );
    }

    #[test]
    fn segment_target_rejects_foreign_entry_owner() {
        let layout = callable_layout();
        let receipt = receipt(&layout, 2);
        let mut issuer = FunctionOwnerIssuerV1::new_for_compilation().expect("issuer");
        let foreign = issuer.issue().expect("foreign owner");
        let entry = ReadyLoopEntryV1::new_for_test(foreign, BasicBlockId::new(0), Vec::new());
        let error = VerifiedLoopOperationTargetBlockV1::issue_for_segment(
            layout.program().demand().context().owner(),
            crate::mir::loop_recipe_contract::LoopItemKeyV1::new(1),
            layout.segments()[0].key(),
            &entry,
            &receipt,
        )
        .expect_err("foreign entry owner must reject");
        assert_eq!(error, LoopOperationTargetRejectV1::EntryOwnerMismatch);
    }
}
