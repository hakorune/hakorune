use crate::mir::compiler::{
    dynamic_full_body_recipe::claims::DynamicFullLoopClaimTargetV2,
    dynamic_full_body_source::{
        DynamicFullBodyBindingRoleV1, DynamicFullBodySourceRoleV1, DynamicFullBodySourceSiteV1,
    },
};
use crate::mir::dynamic_carrier_contract::DynamicCarrierLifecycleObligationV1;
use crate::mir::dynamic_invocation_contract::{
    DynamicInvocationInputHomeV1, DynamicInvocationOutcomeV1,
};
use crate::mir::dynamic_operator_contract::{
    issue_dynamic_operator_execution_envelope_v1, DynamicOperatorDomainV1, DynamicOperatorFamilyV1,
    DynamicOperatorFaultV1, DynamicOperatorNormalResultV1, DynamicOperatorValueClassV1,
};
use crate::mir::loop_recipe_contract::{
    LoopBindingKeyV1, LoopItemKeyV1, LoopJoinEdgeRoleV1, LoopOperationV2, LoopRecipeItemV2,
    LoopValueClassV2, LoopValueKeyV1, VerifiedLoopJoinSigV2,
};

use super::super::{
    DynamicFullLoopFaultFamilyV2, VerifiedDynamicInvocationCarrierLifecycleProgramV1,
};
use super::model::{
    DynamicOperatorCarrierDestinationV1, DynamicOperatorCarrierLifecycleProgramRejectV1,
    DynamicOperatorCarrierLifecycleRowV1, DynamicOperatorCarrierPublicationV1,
    VerifiedDynamicOperatorCarrierLifecycleCatalogV1,
};

pub(in crate::mir::compiler::dynamic_full_body_recipe::coseal::semantic_program) fn issue_operator_carrier_lifecycle_v1(
    program: &VerifiedDynamicInvocationCarrierLifecycleProgramV1<'_, '_>,
) -> Result<
    VerifiedDynamicOperatorCarrierLifecycleCatalogV1,
    DynamicOperatorCarrierLifecycleProgramRejectV1,
> {
    let semantic = &program.program;
    let recipe = semantic.envelope.artifact.recipe().as_recipe();

    let v9_item = source_item(semantic, DynamicFullBodySourceRoleV1::SubstringEndAdd)?;
    let v9_source = source_expr(semantic, DynamicFullBodySourceRoleV1::SubstringEndAdd)?.clone();
    let (v9_operands, v9_result, v9_contract) = require_dynamic_add(recipe, v9_item)?;
    require_fault_cut_point(
        semantic,
        v9_item,
        v9_result,
        DynamicFullLoopFaultFamilyV2::DynamicAdd,
    )?;

    let invocation = source_item(semantic, DynamicFullBodySourceRoleV1::SubstringCall)?;
    if semantic
        .envelope
        .calls
        .item_for(DynamicFullBodySourceRoleV1::SubstringCall)
        != Some(invocation)
    {
        return Err(DynamicOperatorCarrierLifecycleProgramRejectV1::InvocationRelation);
    }
    let call_site = source_expr(semantic, DynamicFullBodySourceRoleV1::SubstringCall)?;
    let invocation_envelope = semantic
        .envelope
        .catalog
        .envelope_for_exact_source(semantic.envelope.source.owner, call_site)
        .map_err(|_| DynamicOperatorCarrierLifecycleProgramRejectV1::InvocationRelation)?;
    if invocation_envelope.envelope().input_home()
        != DynamicInvocationInputHomeV1::BorrowedNoEscapeForInvocation
        || invocation_envelope.envelope().outcome()
            != DynamicInvocationOutcomeV1::NormalSelfContainedDynamicCarrierOrFault
    {
        return Err(DynamicOperatorCarrierLifecycleProgramRejectV1::InvocationRelation);
    }
    require_invocation_argument(recipe, invocation, 1, v9_result)?;
    require_fault_cut_point(
        semantic,
        invocation,
        operation_result(recipe, invocation)?,
        DynamicFullLoopFaultFamilyV2::DynamicInvocation,
    )?;

    let v17_item = source_item(semantic, DynamicFullBodySourceRoleV1::StepAdd)?;
    let v17_source = source_expr(semantic, DynamicFullBodySourceRoleV1::StepAdd)?.clone();
    let (v17_operands, v17_result, v17_contract) = require_dynamic_add(recipe, v17_item)?;
    require_fault_cut_point(
        semantic,
        v17_item,
        v17_result,
        DynamicFullLoopFaultFamilyV2::DynamicAdd,
    )?;

    let write = source_item(semantic, DynamicFullBodySourceRoleV1::StepAssignment)?;
    if source_item(semantic, DynamicFullBodySourceRoleV1::StepTargetI)? != write {
        return Err(DynamicOperatorCarrierLifecycleProgramRejectV1::SourceRelation);
    }
    let assignment_source =
        source_stmt(semantic, DynamicFullBodySourceRoleV1::StepAssignment)?.clone();
    let target_source = source_expr(semantic, DynamicFullBodySourceRoleV1::StepTargetI)?.clone();
    let expected_binding = binding_target(semantic, DynamicFullBodyBindingRoleV1::Induction)?;
    let source_binding = semantic
        .envelope
        .source
        .bindings
        .iter()
        .find(|row| row.role() == DynamicFullBodyBindingRoleV1::Induction)
        .map(|row| row.binding())
        .ok_or(DynamicOperatorCarrierLifecycleProgramRejectV1::SourceRelation)?;
    require_write_binding(recipe, write, expected_binding, v17_result)?;

    let root = semantic.control.after_loop_key();
    require_backedge(
        semantic.control.join_sig(),
        root,
        expected_binding,
        v17_result,
    )?;

    if v9_item == v17_item || v9_result == v17_result {
        return Err(DynamicOperatorCarrierLifecycleProgramRejectV1::Coverage);
    }

    Ok(VerifiedDynamicOperatorCarrierLifecycleCatalogV1 {
        rows: [
            DynamicOperatorCarrierLifecycleRowV1 {
                producer: v9_item,
                producer_source: v9_source,
                operands: v9_operands,
                result: v9_result,
                publication: DynamicOperatorCarrierPublicationV1::OnNormalResultPublication,
                contract: v9_contract,
                destination:
                    DynamicOperatorCarrierDestinationV1::EndAfterInvocationNormalOrFaultOutcome {
                        invocation,
                        argument_ordinal: 1,
                        input_contract: invocation_envelope.envelope().input_home(),
                    },
            },
            DynamicOperatorCarrierLifecycleRowV1 {
                producer: v17_item,
                producer_source: v17_source,
                operands: v17_operands,
                result: v17_result,
                publication: DynamicOperatorCarrierPublicationV1::OnNormalResultPublication,
                contract: v17_contract,
                destination: DynamicOperatorCarrierDestinationV1::ForwardToBindingAtRebindCommit {
                    write,
                    binding: expected_binding,
                    source_binding,
                    assignment_source,
                    target_source,
                    backedge_loop: root,
                },
            },
        ],
    })
}

pub(super) fn require_invocation_argument(
    recipe: &crate::mir::loop_recipe_contract::LoopRecipeV2,
    invocation: LoopItemKeyV1,
    argument_ordinal: usize,
    expected_value: LoopValueKeyV1,
) -> Result<(), DynamicOperatorCarrierLifecycleProgramRejectV1> {
    let Some(LoopRecipeItemV2::Operation {
        operation: LoopOperationV2::CallSlot { args, .. },
    }) = recipe
        .items
        .iter()
        .find(|row| row.key == invocation)
        .map(|row| &row.item)
    else {
        return Err(DynamicOperatorCarrierLifecycleProgramRejectV1::InvocationRelation);
    };
    if args.get(argument_ordinal) != Some(&expected_value)
        || args
            .iter()
            .filter(|value| **value == expected_value)
            .count()
            != 1
    {
        return Err(DynamicOperatorCarrierLifecycleProgramRejectV1::InvocationRelation);
    }
    Ok(())
}

pub(super) fn require_write_binding(
    recipe: &crate::mir::loop_recipe_contract::LoopRecipeV2,
    write: LoopItemKeyV1,
    expected_binding: LoopBindingKeyV1,
    expected_value: LoopValueKeyV1,
) -> Result<(), DynamicOperatorCarrierLifecycleProgramRejectV1> {
    let Some(LoopRecipeItemV2::Operation {
        operation: LoopOperationV2::WriteBinding { binding, value },
    }) = recipe
        .items
        .iter()
        .find(|row| row.key == write)
        .map(|row| &row.item)
    else {
        return Err(DynamicOperatorCarrierLifecycleProgramRejectV1::RecipeRelation);
    };
    if *binding != expected_binding || *value != expected_value {
        return Err(DynamicOperatorCarrierLifecycleProgramRejectV1::RecipeRelation);
    }
    Ok(())
}

pub(super) fn require_backedge(
    join_sig: &VerifiedLoopJoinSigV2,
    root: crate::mir::loop_recipe_contract::LoopNodeKeyV1,
    expected_binding: LoopBindingKeyV1,
    expected_value: LoopValueKeyV1,
) -> Result<(), DynamicOperatorCarrierLifecycleProgramRejectV1> {
    let backedges = join_sig
        .as_sig()
        .loops
        .iter()
        .filter(|row| row.key == root)
        .flat_map(|row| row.edges.iter())
        .filter(|edge| edge.role == LoopJoinEdgeRoleV1::Backedge)
        .collect::<Vec<_>>();
    let [backedge] = backedges.as_slice() else {
        return Err(DynamicOperatorCarrierLifecycleProgramRejectV1::BackedgeRelation);
    };
    if backedge.payload.as_slice()
        != [crate::mir::loop_recipe_contract::LoopJoinPayloadV2 {
            binding: expected_binding,
            value: expected_value,
            class: LoopValueClassV2::Dynamic,
        }]
    {
        return Err(DynamicOperatorCarrierLifecycleProgramRejectV1::BackedgeRelation);
    }
    Ok(())
}

fn require_dynamic_add(
    recipe: &crate::mir::loop_recipe_contract::LoopRecipeV2,
    item: LoopItemKeyV1,
) -> Result<
    (
        [LoopValueKeyV1; 2],
        LoopValueKeyV1,
        &'static crate::mir::dynamic_operator_contract::VerifiedDynamicOperatorExecutionEnvelopeV1,
    ),
    DynamicOperatorCarrierLifecycleProgramRejectV1,
> {
    let Some(LoopRecipeItemV2::Operation {
        operation:
            LoopOperationV2::DynamicAdd {
                left,
                right,
                result,
            },
    }) = recipe
        .items
        .iter()
        .find(|row| row.key == item)
        .map(|row| &row.item)
    else {
        return Err(DynamicOperatorCarrierLifecycleProgramRejectV1::RecipeRelation);
    };
    let domain = DynamicOperatorDomainV1::new(
        DynamicOperatorFamilyV1::Add,
        operator_class(recipe, *left)?,
        operator_class(recipe, *right)?,
    );
    let contract = issue_dynamic_operator_execution_envelope_v1(domain)
        .map_err(|_| DynamicOperatorCarrierLifecycleProgramRejectV1::OperatorContract)?;
    if contract.normal_result()
        != DynamicOperatorNormalResultV1::SelfContainedNonAliasingDynamicCarrier
        || contract.fault()
            != DynamicOperatorFaultV1::TypeErrorBeforeResultNoOperandMutationNoRebind
        || contract.lifecycle()
            != Some(DynamicCarrierLifecycleObligationV1::EndExactlyOnceUnlessForwarded)
        || value_class(recipe, *result)? != LoopValueClassV2::Dynamic
    {
        return Err(DynamicOperatorCarrierLifecycleProgramRejectV1::OperatorContract);
    }
    Ok(([*left, *right], *result, contract))
}

fn operator_class(
    recipe: &crate::mir::loop_recipe_contract::LoopRecipeV2,
    value: LoopValueKeyV1,
) -> Result<DynamicOperatorValueClassV1, DynamicOperatorCarrierLifecycleProgramRejectV1> {
    match value_class(recipe, value)? {
        LoopValueClassV2::Dynamic => Ok(DynamicOperatorValueClassV1::Dynamic),
        LoopValueClassV2::I64 => Ok(DynamicOperatorValueClassV1::I64),
        _ => Err(DynamicOperatorCarrierLifecycleProgramRejectV1::OperatorContract),
    }
}

fn value_class(
    recipe: &crate::mir::loop_recipe_contract::LoopRecipeV2,
    value: LoopValueKeyV1,
) -> Result<LoopValueClassV2, DynamicOperatorCarrierLifecycleProgramRejectV1> {
    recipe
        .values
        .iter()
        .find(|row| row.key == value)
        .map(|row| row.class)
        .ok_or(DynamicOperatorCarrierLifecycleProgramRejectV1::RecipeRelation)
}

fn operation_result(
    recipe: &crate::mir::loop_recipe_contract::LoopRecipeV2,
    item: LoopItemKeyV1,
) -> Result<LoopValueKeyV1, DynamicOperatorCarrierLifecycleProgramRejectV1> {
    match recipe
        .items
        .iter()
        .find(|row| row.key == item)
        .map(|row| &row.item)
    {
        Some(LoopRecipeItemV2::Operation {
            operation:
                LoopOperationV2::CallSlot {
                    result: Some(result),
                    ..
                },
        }) => Ok(*result),
        _ => Err(DynamicOperatorCarrierLifecycleProgramRejectV1::RecipeRelation),
    }
}

fn require_fault_cut_point(
    semantic: &super::super::VerifiedDynamicFullLoopSemanticProgramV2<'_, '_>,
    item: LoopItemKeyV1,
    result: LoopValueKeyV1,
    family: DynamicFullLoopFaultFamilyV2,
) -> Result<(), DynamicOperatorCarrierLifecycleProgramRejectV1> {
    let rows = semantic.fault_cut_points.borrow();
    let matches = rows
        .rows()
        .iter()
        .filter(|row| row.item() == item && row.normal_result() == result && row.family() == family)
        .count();
    (matches == 1)
        .then_some(())
        .ok_or(DynamicOperatorCarrierLifecycleProgramRejectV1::FaultRelation)
}

fn source_item(
    semantic: &super::super::VerifiedDynamicFullLoopSemanticProgramV2<'_, '_>,
    role: DynamicFullBodySourceRoleV1,
) -> Result<LoopItemKeyV1, DynamicOperatorCarrierLifecycleProgramRejectV1> {
    match semantic.envelope.coverage.source_target(role) {
        Some(DynamicFullLoopClaimTargetV2::Item(item)) => Ok(item),
        _ => Err(DynamicOperatorCarrierLifecycleProgramRejectV1::SourceRelation),
    }
}

fn binding_target(
    semantic: &super::super::VerifiedDynamicFullLoopSemanticProgramV2<'_, '_>,
    role: DynamicFullBodyBindingRoleV1,
) -> Result<
    crate::mir::loop_recipe_contract::LoopBindingKeyV1,
    DynamicOperatorCarrierLifecycleProgramRejectV1,
> {
    match semantic.envelope.coverage.binding_target(role) {
        Some(DynamicFullLoopClaimTargetV2::Binding(binding)) => Ok(binding),
        _ => Err(DynamicOperatorCarrierLifecycleProgramRejectV1::SourceRelation),
    }
}

fn source_expr<'a>(
    semantic: &'a super::super::VerifiedDynamicFullLoopSemanticProgramV2<'_, '_>,
    role: DynamicFullBodySourceRoleV1,
) -> Result<
    &'a crate::mir::resolved_semantics::SourceExprSiteV1,
    DynamicOperatorCarrierLifecycleProgramRejectV1,
> {
    semantic
        .envelope
        .source
        .rows
        .iter()
        .find_map(|row| {
            (row.role() == role).then(|| match row.site() {
                DynamicFullBodySourceSiteV1::Expression(site) => Some(site),
                DynamicFullBodySourceSiteV1::Statement(_) => None,
            })?
        })
        .ok_or(DynamicOperatorCarrierLifecycleProgramRejectV1::SourceRelation)
}

fn source_stmt<'a>(
    semantic: &'a super::super::VerifiedDynamicFullLoopSemanticProgramV2<'_, '_>,
    role: DynamicFullBodySourceRoleV1,
) -> Result<
    &'a crate::mir::resolved_semantics::SourceStmtSiteV1,
    DynamicOperatorCarrierLifecycleProgramRejectV1,
> {
    semantic
        .envelope
        .source
        .rows
        .iter()
        .find_map(|row| {
            (row.role() == role).then(|| match row.site() {
                DynamicFullBodySourceSiteV1::Statement(site) => Some(site),
                DynamicFullBodySourceSiteV1::Expression(_) => None,
            })?
        })
        .ok_or(DynamicOperatorCarrierLifecycleProgramRejectV1::SourceRelation)
}
