//! One complete ordered source/effect ledger for the V1 physical consumers.
//!
//! The ledger is issued once while the Builder-free demand is sealed. Physical
//! dispatchers borrow its projections; they never pair Recipe items with source
//! evidence or effect rows again.

use super::super::ids::{LoopItemKeyV1, LoopValueKeyV1};
use super::super::operation_carrier_demand::PreparedLoopDerivedCarrierSeedRowV1;
use super::super::operation_effect::{
    VerifiedLoopOperationEffectProductV1, VerifiedLoopOperationSourceEvidenceV1,
};
use super::super::schema::{LoopOperationV1, LoopRecipeItemV1, LoopRecipeV1, LoopValueClassV1};
use super::super::source_bound_core::{LoopBindingEffectAnchorV1, LoopBindingEffectRoleV1};
use super::operation_physical_demand_rows::{
    PreparedLoopOperationRowV1, PreparedLoopOperationScheduleRowV1, PreparedLoopReadBindingRowV1,
    PreparedLoopWriteBindingRowV1,
};
use super::{LoopOperationPhysicalDemandRejectV1, LoopOperationPhysicalIndexV1};
use crate::mir::resolved_semantics::{SourceExprSiteV1, SourceStmtSiteV1};

#[derive(Debug)]
pub(crate) struct PreparedLoopOperationLedgerV1 {
    operation_rows: Box<[PreparedLoopOperationRowV1]>,
    read_binding_rows: Box<[PreparedLoopReadBindingRowV1]>,
    derived_carrier_seed_rows: Box<[PreparedLoopDerivedCarrierSeedRowV1]>,
    write_binding_rows: Box<[PreparedLoopWriteBindingRowV1]>,
}

impl PreparedLoopOperationLedgerV1 {
    pub(crate) fn operation_rows(&self) -> &[PreparedLoopOperationRowV1] {
        &self.operation_rows
    }

    pub(crate) fn read_binding_rows(&self) -> &[PreparedLoopReadBindingRowV1] {
        &self.read_binding_rows
    }

    pub(crate) fn derived_carrier_seed_rows(&self) -> &[PreparedLoopDerivedCarrierSeedRowV1] {
        &self.derived_carrier_seed_rows
    }

    pub(crate) fn write_binding_rows(&self) -> &[PreparedLoopWriteBindingRowV1] {
        &self.write_binding_rows
    }
}

pub(super) fn issue(
    recipe: &LoopRecipeV1,
    schedule: &[PreparedLoopOperationScheduleRowV1],
    operation_effect: &VerifiedLoopOperationEffectProductV1,
    index: &LoopOperationPhysicalIndexV1,
) -> Result<PreparedLoopOperationLedgerV1, LoopOperationPhysicalDemandRejectV1> {
    let operation_rows = schedule
        .iter()
        .copied()
        .map(|schedule| {
            let operation = operation_for(recipe, schedule.item()).ok_or(
                LoopOperationPhysicalDemandRejectV1::IncompleteSchedule {
                    expected: operation_effect.evidence().len(),
                    found: 0,
                },
            )?;
            Ok(PreparedLoopOperationRowV1::new(schedule, operation))
        })
        .collect::<Result<Vec<_>, LoopOperationPhysicalDemandRejectV1>>()?;

    let mut read_binding_rows = Vec::new();
    let mut derived_carrier_seed_rows = Vec::new();
    let mut write_binding_rows = Vec::new();
    for row in &operation_rows {
        match row.operation() {
            LoopOperationV1::ReadBinding { binding, result } => {
                let evidence = evidence_for(operation_effect, index, row.item())?;
                match evidence.anchor() {
                    LoopBindingEffectAnchorV1::Expr(site) => {
                        read_binding_rows.push(issue_read_row(
                            recipe,
                            row.schedule(),
                            binding,
                            result,
                            evidence,
                            site.site(),
                            operation_effect,
                        )?);
                    }
                    LoopBindingEffectAnchorV1::DerivedCarrierEntry {
                        owner,
                        source_loop,
                        carrier,
                    } => {
                        derived_carrier_seed_rows.push(issue_carrier_row(
                            recipe,
                            row.schedule(),
                            binding,
                            result,
                            evidence,
                            *owner,
                            source_loop,
                            *carrier,
                            operation_effect,
                        )?);
                    }
                }
            }
            LoopOperationV1::WriteBinding { binding, value } => {
                let evidence = evidence_for(operation_effect, index, row.item())?;
                let LoopBindingEffectAnchorV1::Expr(site) = evidence.anchor() else {
                    return Err(
                        LoopOperationPhysicalDemandRejectV1::CarrierSeedUnavailable {
                            item: row.item(),
                        },
                    );
                };
                write_binding_rows.push(issue_write_row(
                    recipe,
                    row.schedule(),
                    binding,
                    value,
                    evidence,
                    site.site(),
                    operation_effect,
                )?);
            }
            LoopOperationV1::ConstI64 { .. }
            | LoopOperationV1::BinaryI64 { .. }
            | LoopOperationV1::CompareI64 { .. } => {}
        }
    }

    Ok(PreparedLoopOperationLedgerV1 {
        operation_rows: operation_rows.into_boxed_slice(),
        read_binding_rows: read_binding_rows.into_boxed_slice(),
        derived_carrier_seed_rows: derived_carrier_seed_rows.into_boxed_slice(),
        write_binding_rows: write_binding_rows.into_boxed_slice(),
    })
}

fn operation_for(recipe: &LoopRecipeV1, item: LoopItemKeyV1) -> Option<LoopOperationV1> {
    recipe.items.iter().find_map(|row| match row.item {
        LoopRecipeItemV1::Operation { operation } if row.key == item => Some(operation),
        _ => None,
    })
}

fn evidence_for<'a>(
    operation_effect: &'a VerifiedLoopOperationEffectProductV1,
    index: &LoopOperationPhysicalIndexV1,
    item: LoopItemKeyV1,
) -> Result<&'a VerifiedLoopOperationSourceEvidenceV1, LoopOperationPhysicalDemandRejectV1> {
    let evidence_index = index
        .evidence_by_item
        .get(&item)
        .copied()
        .ok_or(LoopOperationPhysicalDemandRejectV1::ReadBindingEvidenceMissing { item })?;
    operation_effect
        .evidence()
        .get(evidence_index)
        .ok_or(LoopOperationPhysicalDemandRejectV1::ReadBindingEvidenceMissing { item })
}

fn issue_read_row(
    recipe: &LoopRecipeV1,
    schedule: PreparedLoopOperationScheduleRowV1,
    binding: super::super::ids::LoopBindingKeyV1,
    result: LoopValueKeyV1,
    evidence: &VerifiedLoopOperationSourceEvidenceV1,
    site: &SourceExprSiteV1,
    operation_effect: &VerifiedLoopOperationEffectProductV1,
) -> Result<PreparedLoopReadBindingRowV1, LoopOperationPhysicalDemandRejectV1> {
    let source_binding = evidence.source_binding().ok_or(
        LoopOperationPhysicalDemandRejectV1::ReadBindingSourceMissing {
            item: schedule.item(),
        },
    )?;
    let effect = operation_effect
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
                item: schedule.item(),
            },
        )?;
    let class = binding_class(recipe, binding);
    if effect.class() != class {
        return Err(
            LoopOperationPhysicalDemandRejectV1::ReadBindingSourceShape {
                item: schedule.item(),
            },
        );
    }
    if value_class(recipe, result) != Some(class) {
        return Err(
            LoopOperationPhysicalDemandRejectV1::ReadBindingSourceShape {
                item: schedule.item(),
            },
        );
    }
    Ok(PreparedLoopReadBindingRowV1::new(
        schedule,
        binding,
        result,
        source_binding,
        site.clone(),
        class,
    ))
}

fn issue_carrier_row(
    recipe: &LoopRecipeV1,
    schedule: PreparedLoopOperationScheduleRowV1,
    binding: super::super::ids::LoopBindingKeyV1,
    result: LoopValueKeyV1,
    evidence: &VerifiedLoopOperationSourceEvidenceV1,
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    source_loop: &SourceStmtSiteV1,
    carrier: super::super::ids::LoopCarrierKeyV1,
    operation_effect: &VerifiedLoopOperationEffectProductV1,
) -> Result<PreparedLoopDerivedCarrierSeedRowV1, LoopOperationPhysicalDemandRejectV1> {
    let source_binding = evidence.source_binding().ok_or(
        LoopOperationPhysicalDemandRejectV1::ReadBindingSourceMissing {
            item: schedule.item(),
        },
    )?;
    let effect = operation_effect
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
                item: schedule.item(),
            },
        )?;
    let class = binding_class(recipe, binding);
    let result_class = value_class(recipe, result).ok_or(
        LoopOperationPhysicalDemandRejectV1::ReadBindingSourceShape {
            item: schedule.item(),
        },
    )?;
    if owner != operation_effect.core().owner()
        || effect.class() != class
        || class != result_class
        || source_loop != evidence.source_loop()
    {
        return Err(
            LoopOperationPhysicalDemandRejectV1::ReadBindingSourceShape {
                item: schedule.item(),
            },
        );
    }
    Ok(PreparedLoopDerivedCarrierSeedRowV1::new(
        schedule,
        binding,
        result,
        source_binding,
        source_loop.clone(),
        carrier,
        class,
    ))
}

fn issue_write_row(
    recipe: &LoopRecipeV1,
    schedule: PreparedLoopOperationScheduleRowV1,
    binding: super::super::ids::LoopBindingKeyV1,
    value: LoopValueKeyV1,
    evidence: &VerifiedLoopOperationSourceEvidenceV1,
    site: &SourceExprSiteV1,
    operation_effect: &VerifiedLoopOperationEffectProductV1,
) -> Result<PreparedLoopWriteBindingRowV1, LoopOperationPhysicalDemandRejectV1> {
    let source_binding = evidence.source_binding().ok_or(
        LoopOperationPhysicalDemandRejectV1::WriteBindingSourceMissing {
            item: schedule.item(),
        },
    )?;
    let effect = operation_effect
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
                item: schedule.item(),
            },
        )?;
    let class = value_class(recipe, value).ok_or(
        LoopOperationPhysicalDemandRejectV1::WriteBindingSourceShape {
            item: schedule.item(),
        },
    )?;
    if effect.class() != class {
        return Err(
            LoopOperationPhysicalDemandRejectV1::WriteBindingSourceShape {
                item: schedule.item(),
            },
        );
    }
    Ok(PreparedLoopWriteBindingRowV1::new(
        schedule,
        binding,
        value,
        source_binding,
        site.clone(),
        class,
    ))
}

fn binding_class(
    recipe: &LoopRecipeV1,
    key: super::super::ids::LoopBindingKeyV1,
) -> LoopValueClassV1 {
    recipe
        .bindings
        .iter()
        .find(|row| row.key == key)
        .map(|row| row.class)
        .unwrap_or(LoopValueClassV1::Unit)
}

fn value_class(recipe: &LoopRecipeV1, key: LoopValueKeyV1) -> Option<LoopValueClassV1> {
    recipe
        .values
        .iter()
        .find(|row| row.key == key)
        .map(|row| row.class)
}
