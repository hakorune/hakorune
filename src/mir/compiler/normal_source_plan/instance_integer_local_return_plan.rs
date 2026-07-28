//! Integer-initialized Local return variant for plain instance methods.
//!
//! Family selection and semantic resolution happen once in the cumulative
//! owner. This module verifies one lexical Local relation and dynamic Integer
//! payload without claiming an exact scalar ABI or physical local slot.

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
    BindingKindV1, BindingOriginV1, BindingRefV1, ResolvedLexicalRefV1, SourceBindingSiteV1,
    SourceExprSiteV1, SourceStmtSiteV1, VerifiedSemanticOwnerForestV1,
};

use super::instance_function_plan::{
    GeneralFunctionPlanErrorV1, VerifiedNormalInstanceFunctionFactsV1,
};
use super::module_source::NormalInstanceMethodSourceViewV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VerifiedNormalInstanceLocalV1 {
    site: SourceBindingSiteV1,
    binding: BindingRefV1,
    source_name: Box<str>,
}

impl VerifiedNormalInstanceLocalV1 {
    pub(crate) const fn site(&self) -> &SourceBindingSiteV1 {
        &self.site
    }

    pub(crate) const fn binding(&self) -> BindingRefV1 {
        self.binding
    }

    pub(crate) const fn source_name(&self) -> &str {
        &self.source_name
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NormalInstanceIntegerLocalReturnRecipeV1 {
    receiver: BindingRefV1,
    local: BindingRefV1,
    local_site: SourceBindingSiteV1,
    initializer_site: SourceExprSiteV1,
    initializer_value: i64,
    return_site: SourceStmtSiteV1,
    value_site: SourceExprSiteV1,
}

impl NormalInstanceIntegerLocalReturnRecipeV1 {
    pub(crate) const fn receiver(&self) -> BindingRefV1 {
        self.receiver
    }

    pub(crate) const fn local(&self) -> BindingRefV1 {
        self.local
    }

    pub(crate) const fn local_site(&self) -> &SourceBindingSiteV1 {
        &self.local_site
    }

    pub(crate) const fn initializer_site(&self) -> &SourceExprSiteV1 {
        &self.initializer_site
    }

    pub(crate) const fn initializer_value(&self) -> i64 {
        self.initializer_value
    }

    pub(crate) const fn return_site(&self) -> &SourceStmtSiteV1 {
        &self.return_site
    }

    pub(crate) const fn value_site(&self) -> &SourceExprSiteV1 {
        &self.value_site
    }
}

#[derive(Debug)]
pub(crate) struct VerifiedNormalInstanceIntegerLocalReturnPlanV1 {
    facts: VerifiedNormalInstanceFunctionFactsV1,
    local: VerifiedNormalInstanceLocalV1,
    recipe: NormalInstanceIntegerLocalReturnRecipeV1,
    completion: VerifiedFunctionCompletionV1,
}

impl VerifiedNormalInstanceIntegerLocalReturnPlanV1 {
    pub(crate) const fn facts(&self) -> &VerifiedNormalInstanceFunctionFactsV1 {
        &self.facts
    }

    pub(crate) const fn local(&self) -> &VerifiedNormalInstanceLocalV1 {
        &self.local
    }

    pub(crate) const fn recipe(&self) -> &NormalInstanceIntegerLocalReturnRecipeV1 {
        &self.recipe
    }

    pub(crate) const fn completion(&self) -> &VerifiedFunctionCompletionV1 {
        &self.completion
    }
}

pub(super) fn seal_integer_local_return_one(
    view: NormalInstanceMethodSourceViewV1<'_>,
    selected_name: &str,
    selected_value: i64,
    forest: VerifiedSemanticOwnerForestV1,
    projection: VerifiedSourceProjectionV1,
) -> Result<VerifiedNormalInstanceIntegerLocalReturnPlanV1, GeneralFunctionPlanErrorV1> {
    let input = ResolvedFunctionLoweringInputV1::from_exact_parts_without_callable(
        view.function(),
        &forest,
        &projection,
    )
    .map_err(|cause| GeneralFunctionPlanErrorV1::Input {
        key: view.key().clone(),
        cause,
    })?;
    let recipe = compose_recipe(view.key(), input, selected_name, selected_value)?;
    let receiver = recipe.receiver();
    let local = recipe.local();
    verify_facts(view.key(), input, receiver, local, selected_name, &recipe)?;
    let completion = verify_function_completion_v1(input).map_err(|cause| {
        GeneralFunctionPlanErrorV1::Completion {
            key: view.key().clone(),
            cause,
        }
    })?;
    verify_pairing(view.key(), input, &recipe, &completion)?;
    Ok(VerifiedNormalInstanceIntegerLocalReturnPlanV1 {
        facts: VerifiedNormalInstanceFunctionFactsV1::new(forest, projection, receiver),
        local: VerifiedNormalInstanceLocalV1 {
            site: recipe.local_site().clone(),
            binding: local,
            source_name: selected_name.into(),
        },
        recipe,
        completion,
    })
}

fn compose_recipe(
    key: &CanonicalSameModuleCallableKeyV1,
    input: ResolvedFunctionLoweringInputV1<'_>,
    selected_name: &str,
    selected_value: i64,
) -> Result<NormalInstanceIntegerLocalReturnRecipeV1, GeneralFunctionPlanErrorV1> {
    let source = input.source();
    let body = source
        .root_body()
        .map_err(|_| body_error(key, "body_navigation"))?;
    if body.statements().len() != 2 {
        return Err(body_error(key, "body_must_have_local_and_return"));
    }
    let local_statement = source
        .body_stmt(&body, 0)
        .map_err(|_| body_error(key, "local_navigation"))?;
    let ASTNode::Local {
        variables,
        initial_values,
        declared_type_names,
        ..
    } = local_statement.node()
    else {
        return Err(body_error(key, "exact_local_required"));
    };
    let ([source_name], [Some(_)], [None]) = (
        variables.as_slice(),
        initial_values.as_slice(),
        declared_type_names.as_slice(),
    ) else {
        return Err(body_error(key, "exact_untyped_single_local_required"));
    };
    if source_name != selected_name {
        return Err(body_error(key, "selected_local_name_drift"));
    }
    let initializer = source
        .child_expr_from_stmt(&local_statement, ExprChildRoleV1::LocalInitializer(0))
        .map_err(|_| body_error(key, "local_initializer_navigation"))?;
    let ASTNode::Literal {
        value: LiteralValue::Integer(value),
        ..
    } = initializer.node()
    else {
        return Err(body_error(key, "integer_initializer_required"));
    };
    if *value != selected_value {
        return Err(body_error(key, "selected_integer_initializer_drift"));
    }

    let return_statement = source
        .body_stmt(&body, 1)
        .map_err(|_| body_error(key, "return_navigation"))?;
    if !matches!(
        return_statement.node(),
        ASTNode::Return { value: Some(_), .. }
    ) {
        return Err(body_error(key, "exact_value_return_required"));
    }
    let return_value = source
        .child_expr_from_stmt(&return_statement, ExprChildRoleV1::ReturnValue)
        .map_err(|_| body_error(key, "return_value_navigation"))?;
    let ASTNode::Variable { name, .. } = return_value.node() else {
        return Err(body_error(key, "local_variable_return_required"));
    };
    if name != selected_name {
        return Err(body_error(key, "selected_return_local_drift"));
    }
    let local_site = SourceBindingSiteV1::Local {
        statement: local_statement.site().clone(),
        ordinal: 0,
    };
    let local = input
        .function()
        .declaration_binding(&local_site)
        .ok_or_else(|| fact_error(key, "missing_local"))?;
    let receiver = input
        .function()
        .declaration_binding(&SourceBindingSiteV1::Receiver)
        .ok_or_else(|| fact_error(key, "missing_receiver"))?;
    Ok(NormalInstanceIntegerLocalReturnRecipeV1 {
        receiver,
        local,
        local_site,
        initializer_site: initializer.site().clone(),
        initializer_value: *value,
        return_site: return_statement.site().clone(),
        value_site: return_value.site().clone(),
    })
}

fn verify_facts(
    key: &CanonicalSameModuleCallableKeyV1,
    input: ResolvedFunctionLoweringInputV1<'_>,
    receiver: BindingRefV1,
    local: BindingRefV1,
    source_name: &str,
    recipe: &NormalInstanceIntegerLocalReturnRecipeV1,
) -> Result<(), GeneralFunctionPlanErrorV1> {
    let function = input.function();
    let receiver_record = function
        .binding(receiver)
        .ok_or_else(|| fact_error(key, "missing_receiver_record"))?;
    let local_record = function
        .binding(local)
        .ok_or_else(|| fact_error(key, "missing_local_record"))?;
    let mut variable_refs = function.variable_refs();
    let Some((value_site, resolved)) = variable_refs.next() else {
        return Err(fact_error(key, "missing_local_use"));
    };
    if input.forest().owner_count() != 1
        || !input.forest().upvars().is_empty()
        || function.binding_count() != 2
        || function.declaration_sites().count() != 2
        || receiver_record.kind() != BindingKindV1::Receiver
        || receiver_record.diagnostic_name() != "me"
        || receiver_record.origin() != &BindingOriginV1::Source(SourceBindingSiteV1::Receiver)
        || local_record.kind() != (BindingKindV1::Local { ordinal: 0 })
        || local_record.diagnostic_name() != source_name
        || local_record.origin() != &BindingOriginV1::Source(recipe.local_site().clone())
        || value_site != recipe.value_site()
        || resolved != &ResolvedLexicalRefV1::Local(local)
        || variable_refs.next().is_some()
        || function.assignment_targets().next().is_some()
        || function.direct_call_targets().next().is_some()
    {
        return Err(fact_error(key, "instance_integer_local_fact_closure"));
    }
    Ok(())
}

fn verify_pairing(
    key: &CanonicalSameModuleCallableKeyV1,
    input: ResolvedFunctionLoweringInputV1<'_>,
    recipe: &NormalInstanceIntegerLocalReturnRecipeV1,
    completion: &VerifiedFunctionCompletionV1,
) -> Result<(), GeneralFunctionPlanErrorV1> {
    let exit = completion.function_exit_contract();
    if input.function().variable_ref(recipe.value_site())
        != Some(ResolvedLexicalRefV1::Local(recipe.local()))
        || completion.explicit_site() != Some(recipe.return_site())
        || !completion.returns_value()
        || completion.unreachable_suffix_count() != 0
        || exit.declared_result() != &DeclaredFunctionResultContractV1::Unannotated
        || exit.coverage() != FunctionExitCoverageV1::ExactOneTerminalRootReturn
        || exit.return_contract_relation() != ReturnExitRelationV1::NotRequired
    {
        return Err(GeneralFunctionPlanErrorV1::Pairing {
            key: key.clone(),
            reason: "integer_local_recipe_completion_mismatch",
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
