//! Complete Home-free Fault authorization catalog for the bounded Dynamic Loop.
//!
//! This module classifies only verified Recipe operation sites.  It does not
//! create a runtime Fault, outcome, cleanup obligation, or control-flow edge.

use crate::mir::loop_recipe_contract::{
    LoopItemKeyV1, LoopOperationExecutionClassV2, LoopOperationFaultFamilyV2, LoopRecipeItemV2,
    LoopValueKeyV1,
};

use super::super::VerifiedDynamicFullLoopSourceRecipeEnvelopeV2;
use crate::mir::compiler::dynamic_full_body_source::DynamicFullBodySourceRoleV1;

const FAULT_CUT_POINT_COUNT_V2: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum DynamicFullLoopFaultFamilyV2 {
    DynamicInvocation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) struct DynamicFullLoopFaultCutPointV2 {
    item: LoopItemKeyV1,
    family: DynamicFullLoopFaultFamilyV2,
    normal_result: LoopValueKeyV1,
}

impl DynamicFullLoopFaultCutPointV2 {
    pub(in crate::mir) const fn item(&self) -> LoopItemKeyV1 {
        self.item
    }

    pub(in crate::mir) const fn family(&self) -> DynamicFullLoopFaultFamilyV2 {
        self.family
    }

    pub(in crate::mir) const fn normal_result(&self) -> LoopValueKeyV1 {
        self.normal_result
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum DynamicFullLoopFaultCutPointRejectV2 {
    MissingInvocationRelation,
    ReusedInvocationItem,
    UnexpectedDynamicInvocation,
    ResultlessDynamicInvocation,
    ExactCoverageMismatch,
}

#[derive(Debug)]
pub(super) struct VerifiedDynamicFullLoopFaultCutPointCatalogV2 {
    rows: [DynamicFullLoopFaultCutPointV2; FAULT_CUT_POINT_COUNT_V2],
}

#[derive(Debug, Clone, Copy)]
pub(in crate::mir) struct DynamicFullLoopFaultCutPointCatalogRefV2<'program> {
    rows: &'program [DynamicFullLoopFaultCutPointV2; FAULT_CUT_POINT_COUNT_V2],
}

impl<'program> DynamicFullLoopFaultCutPointCatalogRefV2<'program> {
    pub(in crate::mir) const fn rows(
        &self,
    ) -> &'program [DynamicFullLoopFaultCutPointV2; FAULT_CUT_POINT_COUNT_V2] {
        self.rows
    }
}

impl VerifiedDynamicFullLoopFaultCutPointCatalogV2 {
    pub(super) const fn borrow(&self) -> DynamicFullLoopFaultCutPointCatalogRefV2<'_> {
        DynamicFullLoopFaultCutPointCatalogRefV2 { rows: &self.rows }
    }
}

pub(super) fn issue_fault_cut_points_v2(
    envelope: &VerifiedDynamicFullLoopSourceRecipeEnvelopeV2,
) -> Result<VerifiedDynamicFullLoopFaultCutPointCatalogV2, DynamicFullLoopFaultCutPointRejectV2> {
    let substring = envelope
        .calls
        .item_for(DynamicFullBodySourceRoleV1::SubstringCall)
        .ok_or(DynamicFullLoopFaultCutPointRejectV2::MissingInvocationRelation)?;
    let index_of = envelope
        .calls
        .item_for(DynamicFullBodySourceRoleV1::IndexOfCall)
        .ok_or(DynamicFullLoopFaultCutPointRejectV2::MissingInvocationRelation)?;
    if substring == index_of {
        return Err(DynamicFullLoopFaultCutPointRejectV2::ReusedInvocationItem);
    }

    verify_recipe_fault_cut_points_v2(
        envelope.artifact.recipe().as_recipe(),
        [substring, index_of],
    )
}

fn verify_recipe_fault_cut_points_v2(
    recipe: &crate::mir::loop_recipe_contract::LoopRecipeV2,
    invocation_items: [LoopItemKeyV1; 2],
) -> Result<VerifiedDynamicFullLoopFaultCutPointCatalogV2, DynamicFullLoopFaultCutPointRejectV2> {
    let mut rows = Vec::with_capacity(FAULT_CUT_POINT_COUNT_V2);
    for row in &recipe.items {
        let LoopRecipeItemV2::Operation { operation } = &row.item else {
            continue;
        };
        let (family, normal_result) = match operation.execution_class_v2() {
            LoopOperationExecutionClassV2::FaultBeforeNormalResult {
                family: LoopOperationFaultFamilyV2::DynamicAdd,
                ..
            } => return Err(DynamicFullLoopFaultCutPointRejectV2::ExactCoverageMismatch),
            LoopOperationExecutionClassV2::FaultBeforeNormalResult { .. } => {
                return Err(DynamicFullLoopFaultCutPointRejectV2::ExactCoverageMismatch)
            }
            LoopOperationExecutionClassV2::ExternallyBoundOutcome { normal_result }
                if invocation_items.contains(&row.key) =>
            {
                let result = normal_result
                    .ok_or(DynamicFullLoopFaultCutPointRejectV2::ResultlessDynamicInvocation)?;
                (DynamicFullLoopFaultFamilyV2::DynamicInvocation, result)
            }
            LoopOperationExecutionClassV2::ExternallyBoundOutcome { .. } => {
                return Err(DynamicFullLoopFaultCutPointRejectV2::UnexpectedDynamicInvocation)
            }
            LoopOperationExecutionClassV2::NonFaulting => continue,
        };
        rows.push(DynamicFullLoopFaultCutPointV2 {
            item: row.key,
            family,
            normal_result,
        });
    }

    let family_count = |family| rows.iter().filter(|row| row.family == family).count();
    if rows.len() != FAULT_CUT_POINT_COUNT_V2
        || family_count(DynamicFullLoopFaultFamilyV2::DynamicInvocation) != 2
    {
        return Err(DynamicFullLoopFaultCutPointRejectV2::ExactCoverageMismatch);
    }
    let rows = rows
        .try_into()
        .map_err(|_| DynamicFullLoopFaultCutPointRejectV2::ExactCoverageMismatch)?;
    Ok(VerifiedDynamicFullLoopFaultCutPointCatalogV2 { rows })
}

#[cfg(test)]
pub(super) fn verify_recipe_fault_cut_points_for_test_v2(
    recipe: &crate::mir::loop_recipe_contract::LoopRecipeV2,
    invocation_items: [LoopItemKeyV1; 2],
) -> Result<(), DynamicFullLoopFaultCutPointRejectV2> {
    verify_recipe_fault_cut_points_v2(recipe, invocation_items).map(|_| ())
}
