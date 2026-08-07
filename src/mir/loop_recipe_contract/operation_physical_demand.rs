//! Builder-free full-program operation physical demand.
//!
//! This module consumes the already verified context, operation/effect
//! product, and logical continuation exactly once. It derives a complete
//! Recipe-order schedule; it never emits MIR or exposes a single-operation
//! extraction path.

use std::collections::{BTreeMap, BTreeSet};

use super::continuation::VerifiedLoopContinuationContractV1;
use super::ids::{LoopBlockKeyV1, LoopItemKeyV1, LoopNodeKeyV1};
use super::operation_effect::VerifiedLoopOperationEffectProductV1;
use super::schema::LoopRecipeItemV1;
use super::semantic_context::VerifiedLoopSemanticContextV1;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum LoopOperationPhysicalDemandRejectV1 {
    ContextOwnerMismatch,
    ScopeRegionOwnerMismatch,
    ContinuationOwnerMismatch,
    ContinuationLoopMismatch,
    ContinuationAfterMismatch,
    DuplicateEvidence { item: LoopItemKeyV1 },
    MissingEvidence { item: LoopItemKeyV1 },
    DuplicateSchedule { item: LoopItemKeyV1 },
    EvidencePlacementMismatch { item: LoopItemKeyV1 },
    IncompleteSchedule { expected: usize, found: usize },
}

/// Complete Builder-free Loop input. The index is only an item-to-evidence
/// lookup cache and has no authority over Recipe order or placement.
#[derive(Debug)]
pub(crate) struct VerifiedLoopOperationPhysicalDemandV1 {
    context: VerifiedLoopSemanticContextV1,
    operation_effect: VerifiedLoopOperationEffectProductV1,
    continuation: VerifiedLoopContinuationContractV1,
    index: LoopOperationPhysicalIndexV1,
}

#[derive(Debug, PartialEq, Eq)]
struct LoopOperationPhysicalIndexV1 {
    evidence_by_item: BTreeMap<LoopItemKeyV1, usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PreparedLoopOperationScheduleRowV1 {
    item: LoopItemKeyV1,
    block: LoopBlockKeyV1,
    owner_loop: LoopNodeKeyV1,
}

impl PreparedLoopOperationScheduleRowV1 {
    pub(crate) const fn item(self) -> LoopItemKeyV1 {
        self.item
    }

    pub(crate) const fn block(self) -> LoopBlockKeyV1 {
        self.block
    }

    pub(crate) const fn owner_loop(self) -> LoopNodeKeyV1 {
        self.owner_loop
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct LoopOperationCoverageReceiptV1 {
    operation_count: usize,
}

impl LoopOperationCoverageReceiptV1 {
    pub(crate) const fn operation_count(self) -> usize {
        self.operation_count
    }
}

/// Complete Recipe-order program prepared without any Builder effect.
#[derive(Debug)]
pub(crate) struct PreparedLoopOperationProgramV1 {
    demand: VerifiedLoopOperationPhysicalDemandV1,
    schedule: Box<[PreparedLoopOperationScheduleRowV1]>,
    coverage: LoopOperationCoverageReceiptV1,
}

impl VerifiedLoopOperationPhysicalDemandV1 {
    pub(crate) fn issue(
        context: VerifiedLoopSemanticContextV1,
        operation_effect: VerifiedLoopOperationEffectProductV1,
        continuation: VerifiedLoopContinuationContractV1,
    ) -> Result<Self, LoopOperationPhysicalDemandRejectV1> {
        let owner = operation_effect.core().owner();
        if context.owner() != owner {
            return Err(LoopOperationPhysicalDemandRejectV1::ContextOwnerMismatch);
        }
        let scope_region = context.scope_region();
        if scope_region.scope().owner() != owner || scope_region.region().owner() != owner {
            return Err(LoopOperationPhysicalDemandRejectV1::ScopeRegionOwnerMismatch);
        }
        if continuation.owner() != owner {
            return Err(LoopOperationPhysicalDemandRejectV1::ContinuationOwnerMismatch);
        }
        let root_loop = operation_effect.core().recipe().as_recipe().root_loop;
        if continuation.loop_key() != root_loop {
            return Err(LoopOperationPhysicalDemandRejectV1::ContinuationLoopMismatch);
        }
        if continuation.after().loop_key() != continuation.loop_key() {
            return Err(LoopOperationPhysicalDemandRejectV1::ContinuationAfterMismatch);
        }

        let mut evidence_by_item = BTreeMap::new();
        for (index, evidence) in operation_effect.evidence().iter().enumerate() {
            if evidence_by_item.insert(evidence.item(), index).is_some() {
                return Err(LoopOperationPhysicalDemandRejectV1::DuplicateEvidence {
                    item: evidence.item(),
                });
            }
        }
        Ok(Self {
            context,
            operation_effect,
            continuation,
            index: LoopOperationPhysicalIndexV1 { evidence_by_item },
        })
    }

    /// Consume the full demand and derive every operation row in Recipe order.
    pub(crate) fn prepare_all(
        self,
    ) -> Result<PreparedLoopOperationProgramV1, LoopOperationPhysicalDemandRejectV1> {
        let Self {
            context,
            operation_effect,
            continuation,
            index,
        } = self;
        let recipe = operation_effect.core().recipe().as_recipe();
        let item_rows = recipe
            .items
            .iter()
            .map(|row| (row.key, &row.item))
            .collect::<BTreeMap<_, _>>();
        let mut seen = BTreeSet::new();
        let mut schedule = Vec::new();
        for block in &recipe.blocks {
            for item in &block.items {
                let Some(LoopRecipeItemV1::Operation { .. }) = item_rows.get(item).copied() else {
                    continue;
                };
                if !seen.insert(*item) {
                    return Err(LoopOperationPhysicalDemandRejectV1::DuplicateSchedule {
                        item: *item,
                    });
                }
                let Some(evidence_index) = index.evidence_by_item.get(item).copied() else {
                    return Err(LoopOperationPhysicalDemandRejectV1::MissingEvidence {
                        item: *item,
                    });
                };
                let evidence = &operation_effect.evidence()[evidence_index];
                if evidence.block() != block.key || evidence.owner_loop() != block.owner_loop {
                    return Err(
                        LoopOperationPhysicalDemandRejectV1::EvidencePlacementMismatch {
                            item: *item,
                        },
                    );
                }
                schedule.push(PreparedLoopOperationScheduleRowV1 {
                    item: *item,
                    block: block.key,
                    owner_loop: block.owner_loop,
                });
            }
        }
        if schedule.len() != operation_effect.evidence().len() {
            return Err(LoopOperationPhysicalDemandRejectV1::IncompleteSchedule {
                expected: operation_effect.evidence().len(),
                found: schedule.len(),
            });
        }
        Ok(PreparedLoopOperationProgramV1 {
            demand: Self {
                context,
                operation_effect,
                continuation,
                index,
            },
            schedule: schedule.into_boxed_slice(),
            coverage: LoopOperationCoverageReceiptV1 {
                operation_count: seen.len(),
            },
        })
    }
}

impl PreparedLoopOperationProgramV1 {
    pub(crate) fn demand(&self) -> &VerifiedLoopOperationPhysicalDemandV1 {
        &self.demand
    }

    pub(crate) fn schedule(&self) -> &[PreparedLoopOperationScheduleRowV1] {
        &self.schedule
    }

    pub(crate) const fn coverage(&self) -> LoopOperationCoverageReceiptV1 {
        self.coverage
    }
}

impl VerifiedLoopOperationPhysicalDemandV1 {
    pub(crate) fn context(&self) -> &VerifiedLoopSemanticContextV1 {
        &self.context
    }

    pub(crate) fn operation_effect(&self) -> &VerifiedLoopOperationEffectProductV1 {
        &self.operation_effect
    }

    pub(crate) fn continuation(&self) -> &VerifiedLoopContinuationContractV1 {
        &self.continuation
    }
}
