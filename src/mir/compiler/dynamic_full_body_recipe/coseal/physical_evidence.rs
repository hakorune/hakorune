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
    LoopBlockKeyV1, LoopConditionV2, LoopExitKindV2, LoopItemKeyV1, LoopNodeKeyV1,
    LoopOperationExecutionClassV2, LoopOperationV2, LoopRecipeItemV2, VerifiedLoopRecipeV2,
};
use crate::mir::resolved_semantics::SourceExprSiteV1;

use super::super::claims::DynamicFullLoopClaimTargetV2;
use super::super::DynamicFullLoopRetainedSourceV1;
use super::calls::VerifiedDynamicFullLoopCallRelationsV2;
use super::coverage::VerifiedDynamicFullLoopClaimCoverageV2;
use crate::mir::loop_recipe_contract::VerifiedLoopRecipeArtifactV2;
use crate::mir::source_call_target::VerifiedSourceBoundDynamicMemberCallV1;

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

    pub(in crate::mir) const fn item(&self) -> LoopItemKeyV1 {
        self.item
    }

    pub(in crate::mir) const fn owner_loop(&self) -> LoopNodeKeyV1 {
        self.owner_loop
    }

    pub(in crate::mir) const fn block(&self) -> LoopBlockKeyV1 {
        self.block
    }

    pub(in crate::mir) const fn kind(&self) -> DynamicFullLoopPhysicalItemKindV2 {
        self.kind
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum DynamicLoopPhysicalArmV2 {
    Fallthrough,
    Exit {
        item: LoopItemKeyV1,
        kind: LoopExitKindV2,
    },
}

impl DynamicLoopPhysicalArmV2 {
    pub(in crate::mir) const fn exit_kind(&self) -> Option<LoopExitKindV2> {
        match self {
            Self::Fallthrough => None,
            Self::Exit { kind, .. } => Some(*kind),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) struct DynamicLoopPhysicalBranchControlV2 {
    owner_block: LoopBlockKeyV1,
    if_item: LoopItemKeyV1,
    condition: crate::mir::loop_recipe_contract::LoopValueKeyV1,
    then_block: LoopBlockKeyV1,
    else_block: Option<LoopBlockKeyV1>,
    then_arm: DynamicLoopPhysicalArmV2,
    else_arm: DynamicLoopPhysicalArmV2,
}

impl DynamicLoopPhysicalBranchControlV2 {
    pub(in crate::mir) const fn owner_block(&self) -> LoopBlockKeyV1 {
        self.owner_block
    }

    pub(in crate::mir) const fn if_item(&self) -> LoopItemKeyV1 {
        self.if_item
    }

    pub(in crate::mir) const fn condition(
        &self,
    ) -> crate::mir::loop_recipe_contract::LoopValueKeyV1 {
        self.condition
    }

    pub(in crate::mir) const fn then_block(&self) -> LoopBlockKeyV1 {
        self.then_block
    }

    pub(in crate::mir) const fn else_block(&self) -> Option<LoopBlockKeyV1> {
        self.else_block
    }

    pub(in crate::mir) const fn then_arm(&self) -> DynamicLoopPhysicalArmV2 {
        self.then_arm
    }

    pub(in crate::mir) const fn else_arm(&self) -> DynamicLoopPhysicalArmV2 {
        self.else_arm
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) struct DynamicLoopPhysicalControlRowV2 {
    loop_key: LoopNodeKeyV1,
    condition: LoopConditionV2,
    body_block: LoopBlockKeyV1,
    branches: Box<[DynamicLoopPhysicalBranchControlV2]>,
}

impl DynamicLoopPhysicalControlRowV2 {
    pub(in crate::mir) const fn loop_key(&self) -> LoopNodeKeyV1 {
        self.loop_key
    }

    pub(in crate::mir) const fn condition(&self) -> LoopConditionV2 {
        self.condition
    }

    pub(in crate::mir) const fn body_block(&self) -> LoopBlockKeyV1 {
        self.body_block
    }

    pub(in crate::mir) fn branches(&self) -> &[DynamicLoopPhysicalBranchControlV2] {
        &self.branches
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
    pub(in crate::mir) const fn item(&self) -> LoopItemKeyV1 {
        self.item
    }

    pub(in crate::mir) const fn owner_loop(&self) -> LoopNodeKeyV1 {
        self.owner_loop
    }

    pub(in crate::mir) const fn block(&self) -> LoopBlockKeyV1 {
        self.block
    }

    pub(in crate::mir) const fn source_role(&self) -> DynamicFullBodySourceRoleV1 {
        self.source_role
    }

    pub(in crate::mir) fn source_site(&self) -> &SourceExprSiteV1 {
        &self.source_site
    }

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
pub(in crate::mir) struct DynamicFullLoopOperationPhysicalRefV2<'program> {
    evidence: &'program DynamicFullLoopOperationSourceEffectV2,
    operation: &'program LoopOperationV2,
    call: Option<&'program VerifiedSourceBoundDynamicMemberCallV1>,
}

impl DynamicFullLoopOperationPhysicalRefV2<'_> {
    pub(in crate::mir) const fn item(&self) -> LoopItemKeyV1 {
        self.evidence.item()
    }

    pub(in crate::mir) const fn owner_loop(&self) -> LoopNodeKeyV1 {
        self.evidence.owner_loop()
    }

    pub(in crate::mir) const fn block(&self) -> LoopBlockKeyV1 {
        self.evidence.block()
    }

    pub(in crate::mir) const fn source_role(&self) -> DynamicFullBodySourceRoleV1 {
        self.evidence.source_role()
    }

    pub(in crate::mir) fn source_site(&self) -> &SourceExprSiteV1 {
        self.evidence.source_site()
    }

    pub(in crate::mir) const fn effect(&self) -> DynamicFullLoopOperationEffectV2 {
        self.evidence.effect()
    }

    pub(in crate::mir) const fn execution(&self) -> LoopOperationExecutionClassV2 {
        self.evidence.execution()
    }

    pub(in crate::mir) const fn call_role(&self) -> Option<DynamicFullBodySourceRoleV1> {
        self.evidence.call_role()
    }

    pub(in crate::mir) fn operation(&self) -> &LoopOperationV2 {
        self.operation
    }

    pub(in crate::mir) fn call(&self) -> Option<&VerifiedSourceBoundDynamicMemberCallV1> {
        self.call
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

#[derive(Debug)]
pub(in crate::mir) struct DynamicFullLoopPhysicalRecipeRelationsViewV2<'program> {
    placements: &'program [DynamicFullLoopPhysicalItemPlacementV2;
                  DYNAMIC_FULL_LOOP_PHYSICAL_ITEM_COUNT_V2],
    operations: Box<
        [DynamicFullLoopOperationPhysicalRefV2<'program>;
            DYNAMIC_FULL_LOOP_PHYSICAL_OPERATION_COUNT_V2],
    >,
    controls: Box<[DynamicLoopPhysicalControlRowV2]>,
}

impl<'program> DynamicFullLoopPhysicalRecipeRelationsViewV2<'program> {
    pub(in crate::mir) fn into_parts(
        self,
    ) -> (
        &'program [DynamicFullLoopPhysicalItemPlacementV2;
                      DYNAMIC_FULL_LOOP_PHYSICAL_ITEM_COUNT_V2],
        Box<
            [DynamicFullLoopOperationPhysicalRefV2<'program>;
                DYNAMIC_FULL_LOOP_PHYSICAL_OPERATION_COUNT_V2],
        >,
        Box<[DynamicLoopPhysicalControlRowV2]>,
    ) {
        (self.placements, self.operations, self.controls)
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
    ControlShape,
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

pub(super) fn issue_recipe_relations<'program>(
    evidence: &'program VerifiedDynamicFullLoopPhysicalEvidenceV2,
    recipe: &'program VerifiedLoopRecipeV2,
    calls: &'program VerifiedDynamicFullLoopCallRelationsV2,
) -> Result<
    DynamicFullLoopPhysicalRecipeRelationsViewV2<'program>,
    DynamicFullLoopPhysicalEvidenceRejectV2,
> {
    let recipe_wire = recipe.as_recipe();
    let placements = evidence.placements();
    let operations = issue_operation_refs(evidence, recipe_wire, calls)?;
    let controls = issue_control_rows(recipe_wire, placements)?;
    Ok(DynamicFullLoopPhysicalRecipeRelationsViewV2 {
        placements,
        operations,
        controls,
    })
}

fn issue_operation_refs<'program>(
    evidence: &'program VerifiedDynamicFullLoopPhysicalEvidenceV2,
    recipe: &'program crate::mir::loop_recipe_contract::LoopRecipeV2,
    calls: &'program VerifiedDynamicFullLoopCallRelationsV2,
) -> Result<
    Box<
        [DynamicFullLoopOperationPhysicalRefV2<'program>;
            DYNAMIC_FULL_LOOP_PHYSICAL_OPERATION_COUNT_V2],
    >,
    DynamicFullLoopPhysicalEvidenceRejectV2,
> {
    let mut rows = Vec::with_capacity(DYNAMIC_FULL_LOOP_PHYSICAL_OPERATION_COUNT_V2);
    for source in evidence.operations() {
        let Some(row) = recipe.items.iter().find(|row| row.key == source.item()) else {
            return Err(DynamicFullLoopPhysicalEvidenceRejectV2::MissingItem);
        };
        let LoopRecipeItemV2::Operation { operation } = &row.item else {
            return Err(DynamicFullLoopPhysicalEvidenceRejectV2::OperationSourceCardinality);
        };
        let call = match source.call_role() {
            Some(_) => Some(
                calls
                    .target_for_item(source.item())
                    .ok_or(DynamicFullLoopPhysicalEvidenceRejectV2::CallRelation)?,
            ),
            None => None,
        };
        rows.push(DynamicFullLoopOperationPhysicalRefV2 {
            evidence: source,
            operation,
            call,
        });
    }
    rows.try_into()
        .map_err(|_| DynamicFullLoopPhysicalEvidenceRejectV2::OperationSourceCardinality)
}

fn issue_control_rows(
    recipe: &crate::mir::loop_recipe_contract::LoopRecipeV2,
    placements: &[DynamicFullLoopPhysicalItemPlacementV2; DYNAMIC_FULL_LOOP_PHYSICAL_ITEM_COUNT_V2],
) -> Result<Box<[DynamicLoopPhysicalControlRowV2]>, DynamicFullLoopPhysicalEvidenceRejectV2> {
    let mut rows = Vec::with_capacity(recipe.loops.len());
    for loop_row in &recipe.loops {
        let mut branches = Vec::new();
        for placement in placements.iter().filter(|placement| {
            placement.owner_loop() == loop_row.key
                && placement.kind() == DynamicFullLoopPhysicalItemKindV2::If
        }) {
            let Some(item) = recipe
                .items
                .iter()
                .find(|item| item.key == placement.item())
            else {
                return Err(DynamicFullLoopPhysicalEvidenceRejectV2::ControlShape);
            };
            let LoopRecipeItemV2::If {
                condition,
                then_block,
                else_block,
            } = item.item
            else {
                return Err(DynamicFullLoopPhysicalEvidenceRejectV2::ControlShape);
            };
            branches.push(DynamicLoopPhysicalBranchControlV2 {
                owner_block: placement.block(),
                if_item: placement.item(),
                condition,
                then_block,
                else_block,
                then_arm: exit_arm(recipe, then_block)?,
                else_arm: else_block
                    .map(|block| exit_arm(recipe, block))
                    .transpose()?
                    .unwrap_or(DynamicLoopPhysicalArmV2::Fallthrough),
            });
        }
        rows.push(DynamicLoopPhysicalControlRowV2 {
            loop_key: loop_row.key,
            condition: loop_row.condition,
            body_block: loop_row.body,
            branches: branches.into_boxed_slice(),
        });
    }
    let branch_count = rows.iter().flat_map(|row| row.branches()).count();
    if rows.len() != recipe.loops.len() || branch_count != 1 || rows.len() + branch_count != 2 {
        return Err(DynamicFullLoopPhysicalEvidenceRejectV2::ControlCardinality);
    }
    Ok(rows.into_boxed_slice())
}

fn exit_arm(
    recipe: &crate::mir::loop_recipe_contract::LoopRecipeV2,
    block: LoopBlockKeyV1,
) -> Result<DynamicLoopPhysicalArmV2, DynamicFullLoopPhysicalEvidenceRejectV2> {
    let Some(block) = recipe.blocks.iter().find(|row| row.key == block) else {
        return Err(DynamicFullLoopPhysicalEvidenceRejectV2::ControlShape);
    };
    let mut exits = block.items.iter().filter_map(|item| {
        let row = recipe.items.iter().find(|row| row.key == *item)?;
        let LoopRecipeItemV2::Exit { exit } = row.item else {
            return None;
        };
        recipe
            .exits
            .iter()
            .find(|candidate| candidate.key == exit)
            .map(|candidate| (*item, candidate.kind))
    });
    let Some((item, kind)) = exits.next() else {
        return Ok(DynamicLoopPhysicalArmV2::Fallthrough);
    };
    if exits.next().is_some() {
        return Err(DynamicFullLoopPhysicalEvidenceRejectV2::ControlShape);
    }
    Ok(DynamicLoopPhysicalArmV2::Exit { item, kind })
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
            | LoopOperationV2::CompareI64 { .. }
            | LoopOperationV2::DynamicAdd { .. }
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
