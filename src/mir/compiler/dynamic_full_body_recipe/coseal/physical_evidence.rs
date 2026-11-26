//! Private source/Recipe placement and operation-effect co-seal.
//!
//! This module does not observe AST, infer faultability, or choose a physical
//! schedule. It relates already verified source claims, Recipe placement,
//! exact CallSlot rows, and the existing exhaustive operation-class projection
//! once inside the source/Recipe envelope transaction.

use crate::mir::compiler::dynamic_full_body_source::{
    DynamicFullBodySourceRoleV1, DynamicFullBodySourceSiteV1,
};
use crate::mir::loop_recipe_contract::{
    LoopBlockKeyV1, LoopItemKeyV1, LoopNodeKeyV1, LoopOperationExecutionClassV2, LoopOperationV2,
    LoopRecipeItemV2, VerifiedLoopRecipeV2,
};
use crate::mir::resolved_semantics::SourceExprSiteV1;

use super::super::claims::DynamicFullLoopClaimTargetV2;
use super::super::DynamicFullLoopRetainedSourceV1;
use super::calls::VerifiedDynamicFullLoopCallRelationsV2;
use super::coverage::VerifiedDynamicFullLoopClaimCoverageV2;
use crate::mir::loop_recipe_contract::VerifiedLoopRecipeArtifactV2;

pub(in crate::mir) const DYNAMIC_FULL_LOOP_PHYSICAL_ITEM_COUNT_V2: usize = 17;
pub(in crate::mir) const DYNAMIC_FULL_LOOP_PHYSICAL_OPERATION_COUNT_V2: usize = 15;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum DynamicFullLoopPhysicalItemKindV2 {
    Operation,
    If,
    Exit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) struct DynamicFullLoopPhysicalItemPlacementV2 {
    item: LoopItemKeyV1,
    owner_loop: LoopNodeKeyV1,
    block: LoopBlockKeyV1,
    kind: DynamicFullLoopPhysicalItemKindV2,
}

impl DynamicFullLoopPhysicalItemPlacementV2 {
    #[cfg(test)]
    pub(in crate::mir) const fn for_test(
        item: LoopItemKeyV1,
        owner_loop: LoopNodeKeyV1,
        block: LoopBlockKeyV1,
        kind: DynamicFullLoopPhysicalItemKindV2,
    ) -> Self {
        Self {
            item,
            owner_loop,
            block,
            kind,
        }
    }




}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum DynamicFullLoopOperationEffectV2 {
    BindingRead,
    BindingWrite,
    ExternalCall,
    ExpressionEvaluation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) struct DynamicFullLoopOperationSourceEffectV2 {
    item: LoopItemKeyV1,
    owner_loop: LoopNodeKeyV1,
    block: LoopBlockKeyV1,
    source_role: DynamicFullBodySourceRoleV1,
    source_site: SourceExprSiteV1,
    effect: DynamicFullLoopOperationEffectV2,
    execution: LoopOperationExecutionClassV2,
    call_role: Option<DynamicFullBodySourceRoleV1>,
}

impl DynamicFullLoopOperationSourceEffectV2 {





    pub(in crate::mir) const fn effect(&self) -> DynamicFullLoopOperationEffectV2 {
        self.effect
    }

    pub(in crate::mir) const fn execution(&self) -> LoopOperationExecutionClassV2 {
        self.execution
    }

    pub(in crate::mir) const fn call_role(&self) -> Option<DynamicFullBodySourceRoleV1> {
        self.call_role
    }
}

#[derive(Debug)]
pub(in crate::mir) struct VerifiedDynamicFullLoopPhysicalEvidenceV2 {
    placements:
        Box<[DynamicFullLoopPhysicalItemPlacementV2; DYNAMIC_FULL_LOOP_PHYSICAL_ITEM_COUNT_V2]>,
    operations: Box<
        [DynamicFullLoopOperationSourceEffectV2; DYNAMIC_FULL_LOOP_PHYSICAL_OPERATION_COUNT_V2],
    >,
}

impl VerifiedDynamicFullLoopPhysicalEvidenceV2 {
    pub(in crate::mir) fn placements(
        &self,
    ) -> &[DynamicFullLoopPhysicalItemPlacementV2; DYNAMIC_FULL_LOOP_PHYSICAL_ITEM_COUNT_V2] {
        &self.placements
    }

    pub(in crate::mir) fn operations(
        &self,
    ) -> &[DynamicFullLoopOperationSourceEffectV2; DYNAMIC_FULL_LOOP_PHYSICAL_OPERATION_COUNT_V2]
    {
        &self.operations
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum DynamicFullLoopPhysicalEvidenceRejectV2 {
    PlacementCardinality,
    DuplicateItem,
    MissingItem,
    UnexpectedNestedLoop,
    ControlCardinality,
    OperationSourceCardinality,
    MissingSourceAnchor,
    MultipleSourceAnchors,
    CallRelation,
    EffectCoverage,
    ExecutionCoverage,
}

pub(super) fn issue_physical_evidence_v2(
    source: &DynamicFullLoopRetainedSourceV1,
    artifact: &VerifiedLoopRecipeArtifactV2,
    coverage: &VerifiedDynamicFullLoopClaimCoverageV2,
    calls: &VerifiedDynamicFullLoopCallRelationsV2,
) -> Result<VerifiedDynamicFullLoopPhysicalEvidenceV2, DynamicFullLoopPhysicalEvidenceRejectV2> {
    let recipe = artifact.recipe();
    let placements = issue_placements(recipe)?;
    let operations = issue_operation_sources(source, coverage, calls, recipe, &placements)?;
    verify_operation_coverage(&operations)?;
    let placements = placements
        .try_into()
        .map_err(|_| DynamicFullLoopPhysicalEvidenceRejectV2::PlacementCardinality)?;
    let operations = operations
        .try_into()
        .map_err(|_| DynamicFullLoopPhysicalEvidenceRejectV2::OperationSourceCardinality)?;
    Ok(VerifiedDynamicFullLoopPhysicalEvidenceV2 {
        placements,
        operations,
    })
}

fn verify_operation_coverage(
    rows: &[DynamicFullLoopOperationSourceEffectV2],
) -> Result<(), DynamicFullLoopPhysicalEvidenceRejectV2> {
    let effects = [
        (DynamicFullLoopOperationEffectV2::BindingRead, 5),
        (DynamicFullLoopOperationEffectV2::BindingWrite, 1),
        (DynamicFullLoopOperationEffectV2::ExternalCall, 2),
        (DynamicFullLoopOperationEffectV2::ExpressionEvaluation, 7),
    ];
    if effects.iter().any(|(effect, expected)| {
        rows.iter().filter(|row| row.effect() == *effect).count() != *expected
    }) {
        return Err(DynamicFullLoopPhysicalEvidenceRejectV2::EffectCoverage);
    }
    let non_faulting = rows
        .iter()
        .filter(|row| matches!(row.execution(), LoopOperationExecutionClassV2::NonFaulting))
        .count();
    let faulting = rows
        .iter()
        .filter(|row| {
            matches!(
                row.execution(),
                LoopOperationExecutionClassV2::FaultBeforeNormalResult { .. }
            )
        })
        .count();
    let externally_bound = rows
        .iter()
        .filter(|row| {
            matches!(
                row.execution(),
                LoopOperationExecutionClassV2::ExternallyBoundOutcome { .. }
            )
        })
        .count();
    if (non_faulting, faulting, externally_bound) != (13, 0, 2) {
        return Err(DynamicFullLoopPhysicalEvidenceRejectV2::ExecutionCoverage);
    }
    Ok(())
}

fn issue_placements(
    recipe: &VerifiedLoopRecipeV2,
) -> Result<Vec<DynamicFullLoopPhysicalItemPlacementV2>, DynamicFullLoopPhysicalEvidenceRejectV2> {
    let mut rows = Vec::with_capacity(recipe.as_recipe().items.len());
    let mut seen = std::collections::BTreeSet::new();
    let recipe = recipe.as_recipe();
    for block in &recipe.blocks {
        for item in &block.items {
            if !seen.insert(*item) {
                return Err(DynamicFullLoopPhysicalEvidenceRejectV2::DuplicateItem);
            }
            let Some(row) = recipe.items.iter().find(|row| row.key == *item) else {
                return Err(DynamicFullLoopPhysicalEvidenceRejectV2::MissingItem);
            };
            let kind = match row.item {
                LoopRecipeItemV2::Operation { .. } => DynamicFullLoopPhysicalItemKindV2::Operation,
                LoopRecipeItemV2::If { .. } => DynamicFullLoopPhysicalItemKindV2::If,
                LoopRecipeItemV2::Exit { .. } => DynamicFullLoopPhysicalItemKindV2::Exit,
                LoopRecipeItemV2::Loop { .. } => {
                    return Err(DynamicFullLoopPhysicalEvidenceRejectV2::UnexpectedNestedLoop)
                }
            };
            rows.push(DynamicFullLoopPhysicalItemPlacementV2 {
                item: *item,
                owner_loop: block.owner_loop,
                block: block.key,
                kind,
            });
        }
    }
    if rows.len() != recipe.items.len()
        || rows.len() != DYNAMIC_FULL_LOOP_PHYSICAL_ITEM_COUNT_V2
        || seen.len() != rows.len()
    {
        return Err(DynamicFullLoopPhysicalEvidenceRejectV2::PlacementCardinality);
    }
    let controls = rows
        .iter()
        .filter(|row| row.kind != DynamicFullLoopPhysicalItemKindV2::Operation)
        .count();
    if controls != 2 {
        return Err(DynamicFullLoopPhysicalEvidenceRejectV2::ControlCardinality);
    }
    Ok(rows)
}

fn issue_operation_sources(
    source: &DynamicFullLoopRetainedSourceV1,
    coverage: &VerifiedDynamicFullLoopClaimCoverageV2,
    calls: &VerifiedDynamicFullLoopCallRelationsV2,
    recipe: &VerifiedLoopRecipeV2,
    placements: &[DynamicFullLoopPhysicalItemPlacementV2],
) -> Result<Vec<DynamicFullLoopOperationSourceEffectV2>, DynamicFullLoopPhysicalEvidenceRejectV2> {
    let recipe = recipe.as_recipe();
    let mut rows = Vec::with_capacity(DYNAMIC_FULL_LOOP_PHYSICAL_OPERATION_COUNT_V2);
    for placement in placements
        .iter()
        .filter(|row| row.kind == DynamicFullLoopPhysicalItemKindV2::Operation)
    {
        let Some(item) = recipe.items.iter().find(|row| row.key == placement.item) else {
            return Err(DynamicFullLoopPhysicalEvidenceRejectV2::MissingItem);
        };
        let LoopRecipeItemV2::Operation { operation } = &item.item else {
            return Err(DynamicFullLoopPhysicalEvidenceRejectV2::OperationSourceCardinality);
        };
        let (source_role, source_site) = source_anchor(source, coverage, placement.item)?;
        let (effect, call_role) = match operation {
            LoopOperationV2::BranchPhi { .. }
            | LoopOperationV2::ConstText { .. }
            | LoopOperationV2::ConstNull { .. }
            | LoopOperationV2::BoolExpression { .. }
            | LoopOperationV2::ChildLoopAfter { .. } => {
                return Err(DynamicFullLoopPhysicalEvidenceRejectV2::OperationSourceCardinality);
            }
            LoopOperationV2::ReadBinding { .. } => {
                (DynamicFullLoopOperationEffectV2::BindingRead, None)
            }
            LoopOperationV2::WriteBinding { .. } => {
                (DynamicFullLoopOperationEffectV2::BindingWrite, None)
            }
            LoopOperationV2::CallSlot { .. } => {
                let call_role = [
                    DynamicFullBodySourceRoleV1::SubstringCall,
                    DynamicFullBodySourceRoleV1::IndexOfCall,
                ]
                .into_iter()
                .find(|role| calls.item_for(*role) == Some(placement.item))
                .ok_or(DynamicFullLoopPhysicalEvidenceRejectV2::CallRelation)?;
                (
                    DynamicFullLoopOperationEffectV2::ExternalCall,
                    Some(call_role),
                )
            }
            LoopOperationV2::ConstI64 { .. }
            | LoopOperationV2::BinaryI64 { .. }
            | LoopOperationV2::TextAdd { .. }
            | LoopOperationV2::CompareI64 { .. }
            | LoopOperationV2::DynamicAdd { .. }
            | LoopOperationV2::DynamicSub { .. }
            | LoopOperationV2::DynamicLess { .. }
            | LoopOperationV2::TextEq { .. } => {
                (DynamicFullLoopOperationEffectV2::ExpressionEvaluation, None)
            }
        };
        rows.push(DynamicFullLoopOperationSourceEffectV2 {
            item: placement.item,
            owner_loop: placement.owner_loop,
            block: placement.block,
            source_role,
            source_site,
            effect,
            execution: operation.execution_class_v2(),
            call_role,
        });
    }
    if rows.len() != DYNAMIC_FULL_LOOP_PHYSICAL_OPERATION_COUNT_V2 {
        return Err(DynamicFullLoopPhysicalEvidenceRejectV2::OperationSourceCardinality);
    }
    Ok(rows)
}

fn source_anchor(
    source: &DynamicFullLoopRetainedSourceV1,
    coverage: &VerifiedDynamicFullLoopClaimCoverageV2,
    item: LoopItemKeyV1,
) -> Result<(DynamicFullBodySourceRoleV1, SourceExprSiteV1), DynamicFullLoopPhysicalEvidenceRejectV2>
{
    let mut matches = coverage
        .source_claims()
        .iter()
        .filter_map(|claim| match claim.target {
            DynamicFullLoopClaimTargetV2::Item(claimed) if claimed == item => source
                .rows
                .iter()
                .find(|row| row.role() == claim.role)
                .and_then(|row| match row.site() {
                    DynamicFullBodySourceSiteV1::Expression(site) => {
                        Some((claim.role, site.clone()))
                    }
                    DynamicFullBodySourceSiteV1::Statement(_) => None,
                }),
            _ => None,
        });
    let Some(anchor) = matches.next() else {
        return Err(DynamicFullLoopPhysicalEvidenceRejectV2::MissingSourceAnchor);
    };
    if matches.next().is_some() {
        return Err(DynamicFullLoopPhysicalEvidenceRejectV2::MultipleSourceAnchors);
    }
    Ok(anchor)
}
