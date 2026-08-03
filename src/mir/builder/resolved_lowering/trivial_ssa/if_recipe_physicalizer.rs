//! Physical consumer for the selected fixed-shell If demand.
//!
//! The portable artifact and JoinSig are consumed here, exactly once.  They
//! prove the source correspondence and fixed logical topology; the existing
//! canonical function session remains the only CFG/SSA/PHI writer.  Leaf
//! expressions and branch bodies are still emitted through the lowerer's
//! immutable source view.

use crate::mir::compiler::located::LocatedStmtV1;
use crate::mir::if_recipe_contract::{
    IfJoinEdgeRoleV1, IfJoinPortV1, IfSourcePathStepV1, IfValueClassV1,
};
use crate::mir::resolved_semantics::SourcePathSegmentV1;
use crate::mir::resolved_value_profile::product::TrivialRepresentationV1;

use super::super::if_recipe_adapter::{
    CanonicalIfPhysicalCorrespondenceV1, CanonicalIfPhysicalDemandV1,
};
use super::lowerer::CanonicalTrivialSsaLowererV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct CanonicalIfPhysicalSuccessV1;

pub(super) fn physicalize_if_recipe_v1<'builder, 'source>(
    lowerer: &mut CanonicalTrivialSsaLowererV1<'builder, 'source>,
    statement: &LocatedStmtV1<'source>,
    demand: CanonicalIfPhysicalDemandV1,
) -> Result<CanonicalIfPhysicalSuccessV1, String> {
    let (physical_input, correspondence) = demand.into_parts();
    let (artifact, join_sig) = physical_input.into_parts();
    verify_demand(statement, &correspondence, &artifact, join_sig.as_sig())?;
    lowerer.lower_if_materialization(statement, Some(true))?;
    Ok(CanonicalIfPhysicalSuccessV1)
}

fn verify_demand(
    statement: &LocatedStmtV1<'_>,
    correspondence: &CanonicalIfPhysicalCorrespondenceV1,
    artifact: &crate::mir::if_recipe_contract::VerifiedIfRecipeArtifactV1,
    join_sig: &crate::mir::if_recipe_contract::IfJoinSigV1,
) -> Result<(), String> {
    if statement.site() != correspondence.if_site() {
        return Err("[freeze:contract][if_recipe/source_site_mismatch]".to_string());
    }
    if correspondence.entry_binding().owner() != statement.owner() {
        return Err("[freeze:contract][if_recipe/binding_owner_mismatch]".to_string());
    }
    let Some(SourcePathSegmentV1::Body(root_index)) = correspondence
        .if_site()
        .node()
        .segments()
        .first()
    else {
        return Err("[freeze:contract][if_recipe/if_site_not_root_body]".to_string());
    };
    if correspondence.if_site().node().segments().len() != 1 {
        return Err("[freeze:contract][if_recipe/nested_if_site]".to_string());
    }
    let Some(IfSourcePathStepV1::BodyItem { index }) = artifact
        .source_binding()
        .as_source_binding()
        .claims
        .first()
        .and_then(|claim| claim.path.steps.first())
    else {
        return Err("[freeze:contract][if_recipe/source_claim_missing]".to_string());
    };
    if index != root_index {
        return Err("[freeze:contract][if_recipe/source_claim_site_mismatch]".to_string());
    }
    if join_sig.ports
        != [
            IfJoinPortV1::Entry,
            IfJoinPortV1::Condition,
            IfJoinPortV1::Then,
            IfJoinPortV1::Else,
            IfJoinPortV1::Continuation,
        ]
    {
        return Err("[freeze:contract][if_recipe/logical_ports_mismatch]".to_string());
    }
    let edges = &join_sig.edges;
    if edges[0].role != IfJoinEdgeRoleV1::Enter
        || edges[1].role != IfJoinEdgeRoleV1::True
        || edges[2].role != IfJoinEdgeRoleV1::False
        || edges[3].role != IfJoinEdgeRoleV1::ThenTransfer
        || edges[4].role != IfJoinEdgeRoleV1::ElseTransfer
        || edges[3].to != IfJoinPortV1::Continuation
        || edges[4].to != IfJoinPortV1::Continuation
    {
        return Err("[freeze:contract][if_recipe/logical_edges_mismatch]".to_string());
    }
    if edges[1].value.value != artifact.recipe().as_recipe().condition
        || edges[2].value.value != artifact.recipe().as_recipe().condition
        || edges[1].value.class != IfValueClassV1::Bool
        || edges[2].value.class != IfValueClassV1::Bool
    {
        return Err("[freeze:contract][if_recipe/condition_mapping_mismatch]".to_string());
    }
    if class_for_representation(correspondence.representation())? != join_sig.join.class {
        return Err("[freeze:contract][if_recipe/join_class_mismatch]".to_string());
    }
    let recipe_join = artifact
        .recipe()
        .as_recipe()
        .joins
        .first()
        .ok_or_else(|| "[freeze:contract][if_recipe/join_row_missing]".to_string())?;
    if recipe_join.binding != join_sig.join.binding
        || recipe_join.entry_value != join_sig.join.entry_value
        || recipe_join.then_value != join_sig.join.then_value
        || recipe_join.else_value != join_sig.join.else_value
    {
        return Err("[freeze:contract][if_recipe/join_pair_mismatch]".to_string());
    }
    if correspondence.condition() == correspondence.then_value()
        || correspondence.condition() == correspondence.else_value()
        || correspondence.continuation_read() == correspondence.condition()
        || correspondence.then_assignment() == correspondence.else_assignment()
    {
        return Err("[freeze:contract][if_recipe/source_roles_overlap]".to_string());
    }
    Ok(())
}

fn class_for_representation(
    representation: TrivialRepresentationV1,
) -> Result<IfValueClassV1, String> {
    match representation {
        TrivialRepresentationV1::InlineI64 => Ok(IfValueClassV1::I64),
        TrivialRepresentationV1::InlineBool => Ok(IfValueClassV1::Bool),
        _ => Err("[freeze:contract][if_recipe/unsupported_join_representation]".to_string()),
    }
}
