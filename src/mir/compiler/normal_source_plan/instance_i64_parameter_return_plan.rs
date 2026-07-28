//! Exact `i64` parameter-return variant for plain instance methods.
//!
//! Family selection and semantic resolution happen once in the cumulative
//! owner. This module verifies the selected parameter facts, Recipe, and
//! completion without issuing a physical receiver or parameter ABI.

use crate::ast::ASTNode;
use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::source_projection::VerifiedSourceProjectionV1;
use crate::mir::compiler::source_view::ExprChildRoleV1;
use crate::mir::exact_trivial_parameter_abi::ExactTrivialParameterAbiV1;
use crate::mir::resolved_control_flow::{
    verify_function_completion_v1, DeclaredFunctionResultContractV1, FunctionExitCoverageV1,
    ReturnExitRelationV1, VerifiedFunctionCompletionV1,
};
use crate::mir::resolved_semantics::{
    BindingKindV1, BindingOriginV1, BindingRefV1, ResolvedLexicalRefV1, SourceBindingSiteV1,
    SourceExprSiteV1, SourceStmtSiteV1, VerifiedSemanticOwnerForestV1,
};

use super::instance_function_plan::{
    GeneralFunctionPlanErrorV1, VerifiedNormalInstanceFunctionFactsV1,
};
use super::module_source::NormalInstanceMethodSourceViewV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VerifiedNormalInstanceI64ParameterV1 {
    site: SourceBindingSiteV1,
    binding: BindingRefV1,
    source_name: Box<str>,
    abi: ExactTrivialParameterAbiV1,
}

impl VerifiedNormalInstanceI64ParameterV1 {
    pub(crate) const fn site(&self) -> &SourceBindingSiteV1 {
        &self.site
    }

    pub(crate) const fn binding(&self) -> BindingRefV1 {
        self.binding
    }

    pub(crate) const fn source_name(&self) -> &str {
        &self.source_name
    }

    pub(crate) const fn abi(&self) -> ExactTrivialParameterAbiV1 {
        self.abi
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NormalInstanceI64ParameterReturnRecipeV1 {
    receiver: BindingRefV1,
    parameter: BindingRefV1,
    parameter_site: SourceBindingSiteV1,
    return_site: SourceStmtSiteV1,
    value_site: SourceExprSiteV1,
    abi: ExactTrivialParameterAbiV1,
}

impl NormalInstanceI64ParameterReturnRecipeV1 {
    pub(crate) const fn receiver(&self) -> BindingRefV1 {
        self.receiver
    }

    pub(crate) const fn parameter(&self) -> BindingRefV1 {
        self.parameter
    }

    pub(crate) const fn parameter_site(&self) -> &SourceBindingSiteV1 {
        &self.parameter_site
    }

    pub(crate) const fn return_site(&self) -> &SourceStmtSiteV1 {
        &self.return_site
    }

    pub(crate) const fn value_site(&self) -> &SourceExprSiteV1 {
        &self.value_site
    }

    pub(crate) const fn abi(&self) -> ExactTrivialParameterAbiV1 {
        self.abi
    }
}

#[derive(Debug)]
pub(crate) struct VerifiedNormalInstanceI64ParameterReturnPlanV1 {
    facts: VerifiedNormalInstanceFunctionFactsV1,
    parameter: VerifiedNormalInstanceI64ParameterV1,
    recipe: NormalInstanceI64ParameterReturnRecipeV1,
    completion: VerifiedFunctionCompletionV1,
}

impl VerifiedNormalInstanceI64ParameterReturnPlanV1 {
    pub(crate) const fn facts(&self) -> &VerifiedNormalInstanceFunctionFactsV1 {
        &self.facts
    }

    pub(crate) const fn parameter(&self) -> &VerifiedNormalInstanceI64ParameterV1 {
        &self.parameter
    }

    pub(crate) const fn recipe(&self) -> &NormalInstanceI64ParameterReturnRecipeV1 {
        &self.recipe
    }

    pub(crate) const fn completion(&self) -> &VerifiedFunctionCompletionV1 {
        &self.completion
    }
}

pub(super) fn seal_i64_parameter_return_one(
    view: NormalInstanceMethodSourceViewV1<'_>,
    source_name: &str,
    abi: ExactTrivialParameterAbiV1,
    forest: VerifiedSemanticOwnerForestV1,
    projection: VerifiedSourceProjectionV1,
) -> Result<VerifiedNormalInstanceI64ParameterReturnPlanV1, GeneralFunctionPlanErrorV1> {
    let input = ResolvedFunctionLoweringInputV1::from_exact_parts_without_callable(
        view.function(),
        &forest,
        &projection,
    )
    .map_err(|cause| GeneralFunctionPlanErrorV1::Input {
        key: view.key().clone(),
        cause,
    })?;
    let parameter_site = SourceBindingSiteV1::Parameter { index: 0 };
    let receiver = input
        .function()
        .declaration_binding(&SourceBindingSiteV1::Receiver)
        .ok_or_else(|| fact_error(view.key(), "missing_receiver"))?;
    let parameter = input
        .function()
        .declaration_binding(&parameter_site)
        .ok_or_else(|| fact_error(view.key(), "missing_parameter"))?;
    let recipe = compose_recipe(
        view.key(),
        input,
        receiver,
        parameter,
        parameter_site.clone(),
        source_name,
        abi,
    )?;
    verify_facts(
        view.key(),
        input,
        receiver,
        parameter,
        &parameter_site,
        source_name,
        &recipe,
    )?;
    let completion = verify_function_completion_v1(input).map_err(|cause| {
        GeneralFunctionPlanErrorV1::Completion {
            key: view.key().clone(),
            cause,
        }
    })?;
    verify_pairing(view.key(), input, &recipe, &completion)?;
    Ok(VerifiedNormalInstanceI64ParameterReturnPlanV1 {
        facts: VerifiedNormalInstanceFunctionFactsV1::new(forest, projection, receiver),
        parameter: VerifiedNormalInstanceI64ParameterV1 {
            site: parameter_site,
            binding: parameter,
            source_name: source_name.into(),
            abi,
        },
        recipe,
        completion,
    })
}

fn compose_recipe(
    key: &CanonicalSameModuleCallableKeyV1,
    input: ResolvedFunctionLoweringInputV1<'_>,
    receiver: BindingRefV1,
    parameter: BindingRefV1,
    parameter_site: SourceBindingSiteV1,
    source_name: &str,
    abi: ExactTrivialParameterAbiV1,
) -> Result<NormalInstanceI64ParameterReturnRecipeV1, GeneralFunctionPlanErrorV1> {
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
    let ASTNode::Variable { name, .. } = value.node() else {
        return Err(body_error(key, "parameter_variable_required"));
    };
    if name != source_name {
        return Err(body_error(key, "selected_parameter_name_drift"));
    }
    Ok(NormalInstanceI64ParameterReturnRecipeV1 {
        receiver,
        parameter,
        parameter_site,
        return_site: statement.site().clone(),
        value_site: value.site().clone(),
        abi,
    })
}

fn verify_facts(
    key: &CanonicalSameModuleCallableKeyV1,
    input: ResolvedFunctionLoweringInputV1<'_>,
    receiver: BindingRefV1,
    parameter: BindingRefV1,
    parameter_site: &SourceBindingSiteV1,
    source_name: &str,
    recipe: &NormalInstanceI64ParameterReturnRecipeV1,
) -> Result<(), GeneralFunctionPlanErrorV1> {
    let function = input.function();
    let receiver_record = function
        .binding(receiver)
        .ok_or_else(|| fact_error(key, "missing_receiver_record"))?;
    let parameter_record = function
        .binding(parameter)
        .ok_or_else(|| fact_error(key, "missing_parameter_record"))?;
    let mut variable_refs = function.variable_refs();
    let Some((value_site, resolved)) = variable_refs.next() else {
        return Err(fact_error(key, "missing_parameter_use"));
    };
    if input.forest().owner_count() != 1
        || !input.forest().upvars().is_empty()
        || function.binding_count() != 2
        || function.declaration_sites().count() != 2
        || receiver_record.kind() != BindingKindV1::Receiver
        || receiver_record.diagnostic_name() != "me"
        || receiver_record.origin() != &BindingOriginV1::Source(SourceBindingSiteV1::Receiver)
        || parameter_record.kind() != (BindingKindV1::Parameter { index: 0 })
        || parameter_record.diagnostic_name() != source_name
        || parameter_record.origin() != &BindingOriginV1::Source(parameter_site.clone())
        || value_site != recipe.value_site()
        || resolved != &ResolvedLexicalRefV1::Local(parameter)
        || variable_refs.next().is_some()
        || function.assignment_targets().next().is_some()
        || function.direct_call_targets().next().is_some()
    {
        return Err(fact_error(key, "instance_i64_parameter_fact_closure"));
    }
    Ok(())
}

fn verify_pairing(
    key: &CanonicalSameModuleCallableKeyV1,
    input: ResolvedFunctionLoweringInputV1<'_>,
    recipe: &NormalInstanceI64ParameterReturnRecipeV1,
    completion: &VerifiedFunctionCompletionV1,
) -> Result<(), GeneralFunctionPlanErrorV1> {
    let exit = completion.function_exit_contract();
    if input.function().variable_ref(recipe.value_site())
        != Some(ResolvedLexicalRefV1::Local(recipe.parameter()))
        || completion.explicit_site() != Some(recipe.return_site())
        || !completion.returns_value()
        || completion.unreachable_suffix_count() != 0
        || exit.declared_result() != &DeclaredFunctionResultContractV1::Unannotated
        || exit.coverage() != FunctionExitCoverageV1::ExactOneTerminalRootReturn
        || exit.return_contract_relation() != ReturnExitRelationV1::NotRequired
    {
        return Err(GeneralFunctionPlanErrorV1::Pairing {
            key: key.clone(),
            reason: "parameter_recipe_completion_mismatch",
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
