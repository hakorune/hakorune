//! Complete opaque lifecycle catalog for verified Dynamic invocation results.
//!
//! The language-wide Dynamic envelope owns the lifecycle obligation. This
//! profile co-seal only relates that obligation to exact source, Recipe, and
//! destination evidence. It issues no Home or physical cleanup operation.

use crate::mir::compiler::{
    dynamic_full_body_recipe::claims::DynamicFullLoopClaimTargetV2,
    dynamic_full_body_source::{DynamicFullBodySourceRoleV1, DynamicFullBodySourceSiteV1},
};
use crate::mir::dynamic_carrier_contract::DynamicCarrierLifecycleObligationV1;
use crate::mir::dynamic_invocation_contract::{
    dynamic_invocation_execution_envelope_v1, DynamicInvocationInputHomeV1,
    DynamicInvocationOutcomeV1,
};
use crate::mir::loop_recipe_contract::{
    LoopItemKeyV1, LoopOperationV2, LoopRecipeItemV2, LoopValueKeyV1,
};
use crate::mir::resolved_semantics::{
    BindingRefV1, ResolvedScopeRegionPairV1, SourceBindingSiteV1, SourceExprSiteV1,
    SourceStmtSiteV1,
};

use super::{DynamicFullLoopFaultFamilyV2, VerifiedDynamicFullLoopSemanticProgramV2};

const INVOCATION_LIFECYCLE_COUNT_V1: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum DynamicInvocationCarrierPublicationV1 {
    OnNormalResultPublication,
}

#[derive(Debug)]
enum DynamicInvocationCarrierDestinationV1 {
    LoopBodyLocal {
        declaration: SourceBindingSiteV1,
        declaration_statement: SourceStmtSiteV1,
        binding: BindingRefV1,
        scope_region: ResolvedScopeRegionPairV1,
        read: SourceExprSiteV1,
        borrowed_by: LoopItemKeyV1,
        input_contract: DynamicInvocationInputHomeV1,
    },
    FullExpressionTemporary {
        boundary_source: SourceExprSiteV1,
        boundary_item: LoopItemKeyV1,
    },
}

#[derive(Debug)]
struct DynamicInvocationCarrierLifecycleRowV1 {
    producer: LoopItemKeyV1,
    producer_source: SourceExprSiteV1,
    result: LoopValueKeyV1,
    publication: DynamicInvocationCarrierPublicationV1,
    destination: DynamicInvocationCarrierDestinationV1,
    lifecycle: DynamicCarrierLifecycleObligationV1,
}

#[derive(Debug)]
pub(in crate::mir) struct DynamicInvocationCarrierLifecycleRowRefV1<'program> {
    row: &'program DynamicInvocationCarrierLifecycleRowV1,
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir) enum DynamicInvocationCarrierDestinationRefV1<'program> {
    LoopBodyLocal {
        declaration: &'program SourceBindingSiteV1,
        declaration_statement: &'program SourceStmtSiteV1,
        binding: BindingRefV1,
        scope_region: ResolvedScopeRegionPairV1,
        read: &'program SourceExprSiteV1,
        borrowed_by: LoopItemKeyV1,
        input_contract: DynamicInvocationInputHomeV1,
    },
    FullExpressionTemporary {
        boundary_source: &'program SourceExprSiteV1,
        boundary_item: LoopItemKeyV1,
    },
}

impl DynamicInvocationCarrierLifecycleRowRefV1<'_> {
    pub(in crate::mir) const fn producer(&self) -> LoopItemKeyV1 {
        self.row.producer
    }

    pub(in crate::mir) const fn producer_source(&self) -> &SourceExprSiteV1 {
        &self.row.producer_source
    }

    pub(in crate::mir) const fn result(&self) -> LoopValueKeyV1 {
        self.row.result
    }

    pub(in crate::mir) const fn publication(&self) -> DynamicInvocationCarrierPublicationV1 {
        self.row.publication
    }

    pub(in crate::mir) const fn destination(&self) -> DynamicInvocationCarrierDestinationRefV1<'_> {
        match &self.row.destination {
            DynamicInvocationCarrierDestinationV1::LoopBodyLocal {
                declaration,
                declaration_statement,
                binding,
                scope_region,
                read,
                borrowed_by,
                input_contract,
            } => DynamicInvocationCarrierDestinationRefV1::LoopBodyLocal {
                declaration,
                declaration_statement,
                binding: *binding,
                scope_region: *scope_region,
                read,
                borrowed_by: *borrowed_by,
                input_contract: *input_contract,
            },
            DynamicInvocationCarrierDestinationV1::FullExpressionTemporary {
                boundary_source,
                boundary_item,
            } => DynamicInvocationCarrierDestinationRefV1::FullExpressionTemporary {
                boundary_source,
                boundary_item: *boundary_item,
            },
        }
    }

    pub(in crate::mir) const fn lifecycle(&self) -> DynamicCarrierLifecycleObligationV1 {
        self.row.lifecycle
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum DynamicInvocationCarrierLifecycleRejectV1 {
    InvocationCoverage,
    EnvelopeContract,
    LocalRelation,
    RecipeRelation,
    TemporaryBoundary,
}

#[derive(Debug)]
pub(super) struct VerifiedDynamicInvocationCarrierLifecycleCatalogV1 {
    rows: [DynamicInvocationCarrierLifecycleRowV1; INVOCATION_LIFECYCLE_COUNT_V1],
}

#[derive(Debug)]
pub(in crate::mir) struct DynamicInvocationCarrierLifecycleCatalogRefV1<'program> {
    rows: &'program [DynamicInvocationCarrierLifecycleRowV1; INVOCATION_LIFECYCLE_COUNT_V1],
}

impl<'program> DynamicInvocationCarrierLifecycleCatalogRefV1<'program> {
    pub(in crate::mir) fn rows(
        &self,
    ) -> impl ExactSizeIterator<Item = DynamicInvocationCarrierLifecycleRowRefV1<'program>> + '_
    {
        self.rows
            .iter()
            .map(|row| DynamicInvocationCarrierLifecycleRowRefV1 { row })
    }
}

impl VerifiedDynamicInvocationCarrierLifecycleCatalogV1 {
    pub(super) const fn borrow(&self) -> DynamicInvocationCarrierLifecycleCatalogRefV1<'_> {
        DynamicInvocationCarrierLifecycleCatalogRefV1 { rows: &self.rows }
    }
}

pub(super) fn issue_invocation_carrier_lifecycle_v1(
    program: &VerifiedDynamicFullLoopSemanticProgramV2,
) -> Result<
    VerifiedDynamicInvocationCarrierLifecycleCatalogV1,
    DynamicInvocationCarrierLifecycleRejectV1,
> {
    let invocation_rows = program
        .fault_cut_points
        .borrow()
        .rows()
        .iter()
        .filter(|row| row.family() == DynamicFullLoopFaultFamilyV2::DynamicInvocation)
        .map(|row| (row.item(), row.normal_result()))
        .collect::<Vec<_>>();
    let [(local_producer, local_result), (temporary_producer, temporary_result)] =
        invocation_rows.as_slice()
    else {
        return Err(DynamicInvocationCarrierLifecycleRejectV1::InvocationCoverage);
    };

    let local_contract =
        require_envelope_contract(program, DynamicFullBodySourceRoleV1::SubstringCall)?;
    let temporary_contract =
        require_envelope_contract(program, DynamicFullBodySourceRoleV1::IndexOfCall)?;

    let local = program.envelope.iteration_local();
    if local.producer() != *local_producer
        || local.value() != *local_result
        || local.consumer() != *temporary_producer
    {
        return Err(DynamicInvocationCarrierLifecycleRejectV1::LocalRelation);
    }

    let boundary_item = match program
        .envelope
        .coverage
        .source_target(DynamicFullBodySourceRoleV1::InnerIfCondition)
    {
        Some(DynamicFullLoopClaimTargetV2::Item(item)) => item,
        _ => return Err(DynamicInvocationCarrierLifecycleRejectV1::TemporaryBoundary),
    };
    let boundary_source = expression_source(program, DynamicFullBodySourceRoleV1::InnerIfCondition)
        .ok_or(DynamicInvocationCarrierLifecycleRejectV1::TemporaryBoundary)?;
    let local_source = expression_source(program, DynamicFullBodySourceRoleV1::SubstringCall)
        .ok_or(DynamicInvocationCarrierLifecycleRejectV1::EnvelopeContract)?;
    let temporary_source = expression_source(program, DynamicFullBodySourceRoleV1::IndexOfCall)
        .ok_or(DynamicInvocationCarrierLifecycleRejectV1::EnvelopeContract)?;

    verify_recipe_relations(
        program.envelope.artifact.recipe().as_recipe(),
        (*local_producer, *local_result),
        (*temporary_producer, *temporary_result),
        boundary_item,
    )?;

    Ok(VerifiedDynamicInvocationCarrierLifecycleCatalogV1 {
        rows: [
            DynamicInvocationCarrierLifecycleRowV1 {
                producer: *local_producer,
                producer_source: local_source.clone(),
                result: *local_result,
                publication: DynamicInvocationCarrierPublicationV1::OnNormalResultPublication,
                destination: DynamicInvocationCarrierDestinationV1::LoopBodyLocal {
                    declaration: local.declaration().clone(),
                    declaration_statement: local.declaration_statement().clone(),
                    binding: local.binding(),
                    scope_region: local.scope_region(),
                    read: local.read().clone(),
                    borrowed_by: local.consumer(),
                    input_contract: temporary_contract.input,
                },
                lifecycle: local_contract.lifecycle,
            },
            DynamicInvocationCarrierLifecycleRowV1 {
                producer: *temporary_producer,
                producer_source: temporary_source.clone(),
                result: *temporary_result,
                publication: DynamicInvocationCarrierPublicationV1::OnNormalResultPublication,
                destination: DynamicInvocationCarrierDestinationV1::FullExpressionTemporary {
                    boundary_source: boundary_source.clone(),
                    boundary_item,
                },
                lifecycle: temporary_contract.lifecycle,
            },
        ],
    })
}

struct DynamicInvocationEnvelopeContractV1 {
    input: DynamicInvocationInputHomeV1,
    lifecycle: DynamicCarrierLifecycleObligationV1,
}

fn require_envelope_contract(
    program: &VerifiedDynamicFullLoopSemanticProgramV2,
    role: DynamicFullBodySourceRoleV1,
) -> Result<DynamicInvocationEnvelopeContractV1, DynamicInvocationCarrierLifecycleRejectV1> {
    let _site = expression_source(program, role)
        .ok_or(DynamicInvocationCarrierLifecycleRejectV1::EnvelopeContract)?;
    if program.envelope.calls.item_for(role).is_none() {
        return Err(DynamicInvocationCarrierLifecycleRejectV1::EnvelopeContract);
    }
    let envelope = dynamic_invocation_execution_envelope_v1();
    if envelope.outcome() != DynamicInvocationOutcomeV1::NormalSelfContainedDynamicCarrierOrFault
        || envelope.input_home() != DynamicInvocationInputHomeV1::BorrowedNoEscapeForInvocation
        || envelope.result_lifecycle()
            != DynamicCarrierLifecycleObligationV1::EndExactlyOnceUnlessForwarded
    {
        return Err(DynamicInvocationCarrierLifecycleRejectV1::EnvelopeContract);
    }
    Ok(DynamicInvocationEnvelopeContractV1 {
        input: envelope.input_home(),
        lifecycle: envelope.result_lifecycle(),
    })
}

fn expression_source<'program>(
    program: &'program VerifiedDynamicFullLoopSemanticProgramV2,
    role: DynamicFullBodySourceRoleV1,
) -> Option<&'program SourceExprSiteV1> {
    program.envelope.source.rows.iter().find_map(|row| {
        (row.role() == role).then(|| match row.site() {
            DynamicFullBodySourceSiteV1::Expression(site) => Some(site),
            DynamicFullBodySourceSiteV1::Statement(_) => None,
        })?
    })
}

fn verify_recipe_relations(
    recipe: &crate::mir::loop_recipe_contract::LoopRecipeV2,
    local: (LoopItemKeyV1, LoopValueKeyV1),
    temporary: (LoopItemKeyV1, LoopValueKeyV1),
    boundary_item: LoopItemKeyV1,
) -> Result<(), DynamicInvocationCarrierLifecycleRejectV1> {
    if local.0 == temporary.0 || local.1 == temporary.1 {
        return Err(DynamicInvocationCarrierLifecycleRejectV1::InvocationCoverage);
    }
    require_call_result(recipe, local.0, local.1)?;
    require_call_result(recipe, temporary.0, temporary.1)?;
    require_call_argument(recipe, temporary.0, local.1)?;
    let Some(boundary) = recipe.items.iter().find(|row| row.key == boundary_item) else {
        return Err(DynamicInvocationCarrierLifecycleRejectV1::TemporaryBoundary);
    };
    if !matches!(
        boundary.item,
        LoopRecipeItemV2::Operation {
            operation: LoopOperationV2::DynamicLess { left, .. }
        } if left == temporary.1
    ) {
        return Err(DynamicInvocationCarrierLifecycleRejectV1::TemporaryBoundary);
    }
    Ok(())
}

fn require_call_result(
    recipe: &crate::mir::loop_recipe_contract::LoopRecipeV2,
    item: LoopItemKeyV1,
    result: LoopValueKeyV1,
) -> Result<(), DynamicInvocationCarrierLifecycleRejectV1> {
    let Some(row) = recipe.items.iter().find(|row| row.key == item) else {
        return Err(DynamicInvocationCarrierLifecycleRejectV1::RecipeRelation);
    };
    if !matches!(
        row.item,
        LoopRecipeItemV2::Operation {
            operation: LoopOperationV2::CallSlot { result: Some(actual), .. }
        } if actual == result
    ) {
        return Err(DynamicInvocationCarrierLifecycleRejectV1::RecipeRelation);
    }
    Ok(())
}

fn require_call_argument(
    recipe: &crate::mir::loop_recipe_contract::LoopRecipeV2,
    item: LoopItemKeyV1,
    argument: LoopValueKeyV1,
) -> Result<(), DynamicInvocationCarrierLifecycleRejectV1> {
    let Some(row) = recipe.items.iter().find(|row| row.key == item) else {
        return Err(DynamicInvocationCarrierLifecycleRejectV1::RecipeRelation);
    };
    if !matches!(
        &row.item,
        LoopRecipeItemV2::Operation {
            operation: LoopOperationV2::CallSlot { args, .. }
        } if args.as_slice() == [argument]
    ) {
        return Err(DynamicInvocationCarrierLifecycleRejectV1::RecipeRelation);
    }
    Ok(())
}

#[cfg(test)]
pub(super) fn verify_recipe_invocation_lifecycle_for_test_v1(
    recipe: &crate::mir::loop_recipe_contract::LoopRecipeV2,
    local: (LoopItemKeyV1, LoopValueKeyV1),
    temporary: (LoopItemKeyV1, LoopValueKeyV1),
    borrowed_argument: LoopValueKeyV1,
    boundary_item: LoopItemKeyV1,
) -> Result<(), DynamicInvocationCarrierLifecycleRejectV1> {
    verify_recipe_relations(recipe, local, temporary, boundary_item)?;
    require_call_argument(recipe, temporary.0, borrowed_argument)
}
