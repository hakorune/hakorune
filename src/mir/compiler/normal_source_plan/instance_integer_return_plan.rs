//! Integer-literal Return variant for plain instance methods.
//!
//! This module verifies one already-selected no-parameter
//! `return <Integer literal>` method. Module ownership, all-method iteration,
//! and cumulative plan coverage live in `instance_function_plan`.

use crate::ast::{ASTNode, LiteralValue};
use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::source_projection::VerifiedSourceProjectionV1;
use crate::mir::compiler::source_view::ExprChildRoleV1;
use crate::mir::resolved_control_flow::{
    verify_function_completion_v1, DeclaredFunctionResultContractV1, FunctionExitCoverageV1,
    ReturnExitRelationV1, VerifiedFunctionCompletionV1,
};
use crate::mir::resolved_semantics::{
    BindingKindV1, BindingOriginV1, BindingRefV1, SourceBindingSiteV1, SourceExprSiteV1,
    SourceStmtSiteV1, VerifiedSemanticOwnerForestV1,
};

use super::instance_function_plan::{
    GeneralFunctionPlanErrorV1, VerifiedNormalInstanceFunctionFactsV1,
};
use super::module_source::NormalInstanceMethodSourceViewV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NormalInstanceIntegerReturnRecipeV1 {
    receiver: BindingRefV1,
    return_site: SourceStmtSiteV1,
    value_site: SourceExprSiteV1,
    value: i64,
}

impl NormalInstanceIntegerReturnRecipeV1 {
    pub(crate) const fn receiver(&self) -> BindingRefV1 {
        self.receiver
    }

    pub(crate) const fn return_site(&self) -> &SourceStmtSiteV1 {
        &self.return_site
    }

    pub(crate) const fn value_site(&self) -> &SourceExprSiteV1 {
        &self.value_site
    }

    pub(crate) const fn value(&self) -> i64 {
        self.value
    }
}

#[derive(Debug)]
pub(crate) struct VerifiedNormalInstanceIntegerReturnPlanV1 {
    facts: VerifiedNormalInstanceFunctionFactsV1,
    recipe: NormalInstanceIntegerReturnRecipeV1,
    completion: VerifiedFunctionCompletionV1,
}

impl VerifiedNormalInstanceIntegerReturnPlanV1 {
    pub(crate) const fn facts(&self) -> &VerifiedNormalInstanceFunctionFactsV1 {
        &self.facts
    }

    pub(crate) const fn recipe(&self) -> &NormalInstanceIntegerReturnRecipeV1 {
        &self.recipe
    }

    pub(crate) const fn completion(&self) -> &VerifiedFunctionCompletionV1 {
        &self.completion
    }
}

pub(super) fn seal_integer_literal_return_one(
    view: NormalInstanceMethodSourceViewV1<'_>,
    selected_value: i64,
    forest: VerifiedSemanticOwnerForestV1,
    projection: VerifiedSourceProjectionV1,
) -> Result<VerifiedNormalInstanceIntegerReturnPlanV1, GeneralFunctionPlanErrorV1> {
    let input = ResolvedFunctionLoweringInputV1::from_exact_parts_without_callable(
        view.function(),
        &forest,
        &projection,
    )
    .map_err(|cause| GeneralFunctionPlanErrorV1::Input {
        key: view.key().clone(),
        cause,
    })?;
    let receiver = verify_facts(view.key(), input)?;
    let recipe = compose_recipe(view.key(), input, receiver, selected_value)?;
    let completion = verify_function_completion_v1(input).map_err(|cause| {
        GeneralFunctionPlanErrorV1::Completion {
            key: view.key().clone(),
            cause,
        }
    })?;
    verify_pairing(view.key(), &recipe, &completion)?;
    Ok(VerifiedNormalInstanceIntegerReturnPlanV1 {
        facts: VerifiedNormalInstanceFunctionFactsV1::new(forest, projection, receiver),
        recipe,
        completion,
    })
}

fn verify_facts(
    key: &CanonicalSameModuleCallableKeyV1,
    input: ResolvedFunctionLoweringInputV1<'_>,
) -> Result<BindingRefV1, GeneralFunctionPlanErrorV1> {
    let function = input.function();
    let receiver = function
        .declaration_binding(&SourceBindingSiteV1::Receiver)
        .ok_or_else(|| fact_error(key, "missing_receiver"))?;
    let record = function
        .binding(receiver)
        .ok_or_else(|| fact_error(key, "missing_receiver_record"))?;
    if input.forest().owner_count() != 1
        || !input.forest().upvars().is_empty()
        || function.binding_count() != 1
        || function.declaration_sites().count() != 1
        || record.kind() != BindingKindV1::Receiver
        || record.diagnostic_name() != "me"
        || record.origin() != &BindingOriginV1::Source(SourceBindingSiteV1::Receiver)
        || function.variable_refs().next().is_some()
        || function.assignment_targets().next().is_some()
        || function.direct_call_targets().next().is_some()
    {
        return Err(fact_error(key, "instance_receiver_fact_closure"));
    }
    Ok(receiver)
}

fn compose_recipe(
    key: &CanonicalSameModuleCallableKeyV1,
    input: ResolvedFunctionLoweringInputV1<'_>,
    receiver: BindingRefV1,
    selected_value: i64,
) -> Result<NormalInstanceIntegerReturnRecipeV1, GeneralFunctionPlanErrorV1> {
    let source = input.source();
    let body = source
        .root_body()
        .map_err(|_| body_error(key, "body_navigation"))?;
    if body.statements().len() != 1 {
        return Err(body_error(key, "body_must_have_one_statement"));
    }
    let statement = source
        .body_stmt(&body, 0)
        .map_err(|_| body_error(key, "return_navigation"))?;
    if !matches!(statement.node(), ASTNode::Return { value: Some(_), .. }) {
        return Err(body_error(key, "exact_value_return_required"));
    }
    let value = source
        .child_expr_from_stmt(&statement, ExprChildRoleV1::ReturnValue)
        .map_err(|_| body_error(key, "return_value_navigation"))?;
    let ASTNode::Literal {
        value: LiteralValue::Integer(integer),
        ..
    } = value.node()
    else {
        return Err(body_error(key, "integer_literal_required"));
    };
    if *integer != selected_value {
        return Err(body_error(key, "selected_integer_literal_drift"));
    }
    Ok(NormalInstanceIntegerReturnRecipeV1 {
        receiver,
        return_site: statement.site().clone(),
        value_site: value.site().clone(),
        value: *integer,
    })
}

fn verify_pairing(
    key: &CanonicalSameModuleCallableKeyV1,
    recipe: &NormalInstanceIntegerReturnRecipeV1,
    completion: &VerifiedFunctionCompletionV1,
) -> Result<(), GeneralFunctionPlanErrorV1> {
    let exit = completion.function_exit_contract();
    if completion.explicit_site() != Some(recipe.return_site())
        || !completion.returns_value()
        || completion.unreachable_suffix_count() != 0
        || exit.declared_result() != &DeclaredFunctionResultContractV1::Unannotated
        || exit.coverage() != FunctionExitCoverageV1::ExactOneTerminalRootReturn
        || exit.return_contract_relation() != ReturnExitRelationV1::NotRequired
    {
        return Err(GeneralFunctionPlanErrorV1::Pairing {
            key: key.clone(),
            reason: "recipe_completion_mismatch",
        });
    }
    Ok(())
}

fn fact_error(
    key: &CanonicalSameModuleCallableKeyV1,
    reason: &'static str,
) -> GeneralFunctionPlanErrorV1 {
    GeneralFunctionPlanErrorV1::FactCoverage {
        key: key.clone(),
        reason,
    }
}

fn body_error(
    key: &CanonicalSameModuleCallableKeyV1,
    reason: &'static str,
) -> GeneralFunctionPlanErrorV1 {
    GeneralFunctionPlanErrorV1::UnsupportedBody {
        key: key.clone(),
        reason,
    }
}
