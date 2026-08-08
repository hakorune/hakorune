//! Builder-free full-program operation physical demand.
//!
//! This module consumes the already verified context, operation/effect
//! product, and logical continuation exactly once. It derives a complete
//! Recipe-order schedule; it never emits MIR or exposes a single-operation
//! extraction path.

use std::collections::{BTreeMap, BTreeSet};

use super::continuation::VerifiedLoopContinuationContractV1;
use super::ids::LoopItemKeyV1;
use super::operation_carrier_demand::PreparedLoopDerivedCarrierSeedRowV1;
use super::operation_effect::VerifiedLoopOperationEffectProductV1;
use super::schema::{LoopOperationV1, LoopRecipeItemV1};
use super::semantic_context::VerifiedLoopSemanticContextV1;
use super::source_bound_core::{LoopBindingEffectAnchorV1, LoopBindingEffectRoleV1};

#[path = "operation_physical_demand_rows.rs"]
mod operation_physical_demand_rows;
#[path = "operation_physical_demand_schedule.rs"]
mod operation_physical_demand_schedule;

pub(crate) use operation_physical_demand_rows::{
    PreparedLoopOperationRowV1, PreparedLoopOperationScheduleRowV1, PreparedLoopReadBindingRowV1,
    PreparedLoopWriteBindingRowV1,
};

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
    ReadBindingEvidenceMissing { item: LoopItemKeyV1 },
    ReadBindingSourceMissing { item: LoopItemKeyV1 },
    ReadBindingSourceShape { item: LoopItemKeyV1 },
    ReadBindingEffectMissing { item: LoopItemKeyV1 },
    WriteBindingEvidenceMissing { item: LoopItemKeyV1 },
    WriteBindingSourceMissing { item: LoopItemKeyV1 },
    WriteBindingEffectMissing { item: LoopItemKeyV1 },
    WriteBindingSourceShape { item: LoopItemKeyV1 },
    CarrierSeedUnavailable { item: LoopItemKeyV1 },
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
pub(super) struct LoopOperationPhysicalIndexV1 {
    evidence_by_item: BTreeMap<LoopItemKeyV1, usize>,
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
            .map(|row| (row.key, row.item.clone()))
            .collect::<BTreeMap<_, _>>();
        let mut seen = BTreeSet::new();
        let mut schedule = Vec::new();
        operation_physical_demand_schedule::append_operation_schedule(
            recipe,
            recipe.root_loop,
            &item_rows,
            &operation_effect,
            &index,
            &mut seen,
            &mut schedule,
        )?;
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

    /// Consume the complete prepared program into the private physical layout
    /// derived from Recipe/JoinSig order. No single-item extraction is exposed.
    pub(crate) fn prepare_physical_layout(
        self,
    ) -> Result<
        super::physical_layout::PreparedLoopPhysicalLayoutV1,
        super::physical_layout::LoopPhysicalLayoutRejectV1,
    > {
        super::physical_layout::PreparedLoopPhysicalLayoutV1::from_program(self)
    }

    pub(crate) fn schedule(&self) -> &[PreparedLoopOperationScheduleRowV1] {
        &self.schedule
    }

    pub(crate) const fn coverage(&self) -> LoopOperationCoverageReceiptV1 {
        self.coverage
    }

    /// Project every operation in Recipe order. This is the only operation
    /// schedule view; no single-item selector is exposed.
    pub(crate) fn operation_rows(&self) -> Box<[PreparedLoopOperationRowV1]> {
        let recipe = self.demand.operation_effect.core().recipe().as_recipe();
        self.schedule
            .iter()
            .copied()
            .map(|schedule| {
                let operation = recipe
                    .items
                    .iter()
                    .find(|row| row.key == schedule.item)
                    .and_then(|row| match row.item {
                        LoopRecipeItemV1::Operation { operation } => Some(operation),
                        _ => None,
                    })
                    .expect("prepared schedule contains only operation items");
                PreparedLoopOperationRowV1 {
                    schedule,
                    operation,
                }
            })
            .collect()
    }

    /// Project every ReadBinding row from the complete prepared program.
    /// There is deliberately no first/select/take operation API.
    pub(crate) fn read_binding_rows(
        &self,
    ) -> Result<Box<[PreparedLoopReadBindingRowV1]>, LoopOperationPhysicalDemandRejectV1> {
        let recipe = self.demand.operation_effect.core().recipe().as_recipe();
        let mut rows = Vec::new();
        for schedule in self.schedule.iter().copied() {
            let item = recipe
                .items
                .iter()
                .find(|row| row.key == schedule.item)
                .map(|row| &row.item);
            let Some(LoopRecipeItemV1::Operation { operation }) = item else {
                continue;
            };
            let LoopOperationV1::ReadBinding { binding, result } = *operation else {
                continue;
            };
            let evidence = self
                .demand
                .operation_effect
                .evidence()
                .iter()
                .find(|evidence| evidence.item() == schedule.item)
                .ok_or(
                    LoopOperationPhysicalDemandRejectV1::ReadBindingEvidenceMissing {
                        item: schedule.item,
                    },
                )?;
            let source_binding = evidence.source_binding().ok_or(
                LoopOperationPhysicalDemandRejectV1::ReadBindingSourceMissing {
                    item: schedule.item,
                },
            )?;
            let LoopBindingEffectAnchorV1::Expr(owned_site) = evidence.anchor() else {
                continue;
            };
            let source_site = owned_site.site().clone();
            let effect = self
                .demand
                .operation_effect
                .core()
                .effect_relations()
                .iter()
                .find(|effect| {
                    effect.recipe_binding() == binding
                        && effect.source_binding() == source_binding
                        && effect.anchor() == evidence.anchor()
                        && matches!(effect.role(), LoopBindingEffectRoleV1::SourceRead { .. })
                })
                .ok_or(
                    LoopOperationPhysicalDemandRejectV1::ReadBindingEffectMissing {
                        item: schedule.item,
                    },
                )?;
            if effect.class()
                != recipe
                    .bindings
                    .iter()
                    .find(|row| row.key == binding)
                    .map(|row| row.class)
                    .unwrap_or(super::schema::LoopValueClassV1::Unit)
            {
                return Err(
                    LoopOperationPhysicalDemandRejectV1::ReadBindingSourceShape {
                        item: schedule.item,
                    },
                );
            }
            let class = recipe
                .values
                .iter()
                .find(|value| value.key == result)
                .map(|value| value.class)
                .ok_or(
                    LoopOperationPhysicalDemandRejectV1::ReadBindingSourceShape {
                        item: schedule.item,
                    },
                )?;
            rows.push(PreparedLoopReadBindingRowV1 {
                schedule,
                binding,
                result,
                source_binding,
                source_site,
                class,
            });
        }
        Ok(rows.into_boxed_slice())
    }

    /// Project every `DerivedCarrierEntry` ReadBinding row from the complete
    /// program. The source statement anchor is retained as provenance; no
    /// expression site is fabricated and no source claim is issued here.
    pub(crate) fn derived_carrier_seed_rows(
        &self,
    ) -> Result<Box<[PreparedLoopDerivedCarrierSeedRowV1]>, LoopOperationPhysicalDemandRejectV1>
    {
        let recipe = self.demand.operation_effect.core().recipe().as_recipe();
        let mut rows = Vec::new();
        for schedule in self.schedule.iter().copied() {
            let Some(LoopRecipeItemV1::Operation { operation }) = recipe
                .items
                .iter()
                .find(|row| row.key == schedule.item)
                .map(|row| &row.item)
            else {
                continue;
            };
            let LoopOperationV1::ReadBinding { binding, result } = *operation else {
                continue;
            };
            let evidence = self
                .demand
                .operation_effect
                .evidence()
                .iter()
                .find(|evidence| evidence.item() == schedule.item)
                .ok_or(
                    LoopOperationPhysicalDemandRejectV1::ReadBindingEvidenceMissing {
                        item: schedule.item,
                    },
                )?;
            let LoopBindingEffectAnchorV1::DerivedCarrierEntry {
                owner,
                source_loop,
                carrier,
            } = evidence.anchor()
            else {
                continue;
            };
            let source_binding = evidence.source_binding().ok_or(
                LoopOperationPhysicalDemandRejectV1::ReadBindingSourceMissing {
                    item: schedule.item,
                },
            )?;
            let effect = self
                .demand
                .operation_effect
                .core()
                .effect_relations()
                .iter()
                .find(|effect| {
                    effect.recipe_binding() == binding
                        && effect.source_binding() == source_binding
                        && effect.anchor() == evidence.anchor()
                        && matches!(effect.role(), LoopBindingEffectRoleV1::DerivedCarrierEntry)
                })
                .ok_or(
                    LoopOperationPhysicalDemandRejectV1::ReadBindingEffectMissing {
                        item: schedule.item,
                    },
                )?;
            let class = recipe
                .bindings
                .iter()
                .find(|row| row.key == binding)
                .map(|row| row.class)
                .ok_or(
                    LoopOperationPhysicalDemandRejectV1::ReadBindingSourceShape {
                        item: schedule.item,
                    },
                )?;
            let result_class = recipe
                .values
                .iter()
                .find(|row| row.key == result)
                .map(|row| row.class)
                .ok_or(
                    LoopOperationPhysicalDemandRejectV1::ReadBindingSourceShape {
                        item: schedule.item,
                    },
                )?;
            if *owner != self.demand.context.owner()
                || source_loop != evidence.source_loop()
                || effect.class() != class
                || class != result_class
            {
                return Err(
                    LoopOperationPhysicalDemandRejectV1::ReadBindingSourceShape {
                        item: schedule.item,
                    },
                );
            }
            rows.push(PreparedLoopDerivedCarrierSeedRowV1 {
                schedule,
                binding,
                result,
                source_binding,
                source_loop: source_loop.clone(),
                carrier: *carrier,
                class,
            });
        }
        Ok(rows.into_boxed_slice())
    }

    /// Project every WriteBinding row from the complete prepared program.
    pub(crate) fn write_binding_rows(
        &self,
    ) -> Result<Box<[PreparedLoopWriteBindingRowV1]>, LoopOperationPhysicalDemandRejectV1> {
        let recipe = self.demand.operation_effect.core().recipe().as_recipe();
        let mut rows = Vec::new();
        for schedule in self.schedule.iter().copied() {
            let Some(LoopRecipeItemV1::Operation { operation }) = recipe
                .items
                .iter()
                .find(|row| row.key == schedule.item)
                .map(|row| &row.item)
            else {
                continue;
            };
            let LoopOperationV1::WriteBinding { binding, value } = *operation else {
                continue;
            };
            let evidence = self
                .demand
                .operation_effect
                .evidence()
                .iter()
                .find(|evidence| evidence.item() == schedule.item)
                .ok_or(
                    LoopOperationPhysicalDemandRejectV1::WriteBindingEvidenceMissing {
                        item: schedule.item,
                    },
                )?;
            let source_binding = evidence.source_binding().ok_or(
                LoopOperationPhysicalDemandRejectV1::WriteBindingSourceMissing {
                    item: schedule.item,
                },
            )?;
            let LoopBindingEffectAnchorV1::Expr(owned_site) = evidence.anchor() else {
                return Err(
                    LoopOperationPhysicalDemandRejectV1::CarrierSeedUnavailable {
                        item: schedule.item,
                    },
                );
            };
            let source_site = owned_site.site().clone();
            let effect = self
                .demand
                .operation_effect
                .core()
                .effect_relations()
                .iter()
                .find(|effect| {
                    effect.recipe_binding() == binding
                        && effect.source_binding() == source_binding
                        && effect.anchor() == evidence.anchor()
                        && matches!(effect.role(), LoopBindingEffectRoleV1::SourceWrite { .. })
                })
                .ok_or(
                    LoopOperationPhysicalDemandRejectV1::WriteBindingEffectMissing {
                        item: schedule.item,
                    },
                )?;
            let class = recipe
                .values
                .iter()
                .find(|row| row.key == value)
                .map(|row| row.class)
                .ok_or(
                    LoopOperationPhysicalDemandRejectV1::WriteBindingSourceShape {
                        item: schedule.item,
                    },
                )?;
            if effect.class() != class {
                return Err(
                    LoopOperationPhysicalDemandRejectV1::WriteBindingSourceShape {
                        item: schedule.item,
                    },
                );
            }
            rows.push(PreparedLoopWriteBindingRowV1 {
                schedule,
                binding,
                value,
                source_binding,
                source_site,
                class,
            });
        }
        Ok(rows.into_boxed_slice())
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
