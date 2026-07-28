//! Disconnected first function-plan slice for plain instance methods.
//!
//! Every instance method in one verified module must be a no-parameter
//! `return <Integer literal>` function. This owner performs no MIR lowering,
//! publication, route reselection, or physical receiver/ownership selection.

use std::collections::BTreeMap;

use crate::ast::{ASTNode, LiteralValue};
use crate::mir::builder::{CanonicalSameModuleCallableKeyV1, SameModuleCallableNamespaceV1};
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::lowering_input::CanonicalLoweringErrorV1;
use crate::mir::compiler::source_projection::{
    SourceNavigationErrorV1, VerifiedSourceProjectionV1,
};
use crate::mir::compiler::source_view::ExprChildRoleV1;
use crate::mir::resolved_control_flow::{
    verify_function_completion_v1, DeclaredFunctionResultContractV1,
    FunctionCompletionVerificationErrorV1, FunctionExitCoverageV1, ReturnExitRelationV1,
    VerifiedFunctionCompletionV1,
};
use crate::mir::resolved_semantics::{
    BindingKindV1, BindingOriginV1, BindingRefV1, FunctionSemanticResolverSessionV1,
    FunctionSyntaxViewV1, ResolveFunctionErrorV1, ResolveOwnerForestErrorV1, SourceBindingSiteV1,
    SourceExprSiteV1, SourceStmtSiteV1, VerifiedSemanticOwnerForestV1,
};
use crate::mir::source_call_target::SameModuleCallableSourceReceiverPolicyV1;

use super::main_source::{NormalMainFunctionSourceErrorV1, NormalMainFunctionSourceViewV1};
use super::module_source::{
    NormalInstanceMethodSourceLoanErrorV1, NormalInstanceMethodSourceViewV1,
    VerifiedNormalModuleSourceV1,
};

#[derive(Debug)]
pub(crate) struct VerifiedNormalInstanceFunctionFactsV1 {
    forest: VerifiedSemanticOwnerForestV1,
    projection: VerifiedSourceProjectionV1,
    receiver: BindingRefV1,
}

impl VerifiedNormalInstanceFunctionFactsV1 {
    pub(crate) const fn receiver(&self) -> BindingRefV1 {
        self.receiver
    }

    pub(crate) fn owner_count(&self) -> usize {
        self.forest.owner_count()
    }
}

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

#[derive(Debug)]
pub(crate) struct VerifiedNormalInstanceIntegerReturnPlanSetV1 {
    source: VerifiedNormalModuleSourceV1,
    plans: BTreeMap<CanonicalSameModuleCallableKeyV1, VerifiedNormalInstanceIntegerReturnPlanV1>,
}

impl VerifiedNormalInstanceIntegerReturnPlanSetV1 {
    pub(crate) fn source_identity(&self) -> &str {
        self.source.source_identity()
    }

    pub(crate) fn plans(
        &self,
    ) -> impl Iterator<
        Item = (
            &CanonicalSameModuleCallableKeyV1,
            &VerifiedNormalInstanceIntegerReturnPlanV1,
        ),
    > {
        self.plans.iter()
    }

    pub(crate) fn len(&self) -> usize {
        self.plans.len()
    }

    pub(super) fn borrow_exact_main_function(
        &self,
    ) -> Result<NormalMainFunctionSourceViewV1<'_>, NormalMainFunctionSourceErrorV1> {
        self.source.borrow_exact_main_function()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GeneralFunctionPlanStageV1 {
    Inventory,
    Source,
    Resolve,
    Recipe,
    Completion,
    Pairing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GeneralFunctionSignatureStopV1 {
    Parameters,
    ParameterDeclarations,
    ReturnAnnotation,
    Uses,
    Attributes,
}

#[derive(Debug)]
pub(crate) enum GeneralFunctionPlanErrorV1 {
    NoInstanceMethod,
    Source {
        key: CanonicalSameModuleCallableKeyV1,
        cause: NormalInstanceMethodSourceLoanErrorV1,
    },
    UnsupportedSignature {
        key: CanonicalSameModuleCallableKeyV1,
        reason: GeneralFunctionSignatureStopV1,
    },
    ResolverSession(ResolveFunctionErrorV1),
    Resolver {
        key: CanonicalSameModuleCallableKeyV1,
        cause: ResolveOwnerForestErrorV1,
    },
    Projection {
        key: CanonicalSameModuleCallableKeyV1,
        cause: SourceNavigationErrorV1,
    },
    Input {
        key: CanonicalSameModuleCallableKeyV1,
        cause: CanonicalLoweringErrorV1,
    },
    FactCoverage {
        key: CanonicalSameModuleCallableKeyV1,
        reason: &'static str,
    },
    UnsupportedBody {
        key: CanonicalSameModuleCallableKeyV1,
        reason: &'static str,
    },
    Completion {
        key: CanonicalSameModuleCallableKeyV1,
        cause: FunctionCompletionVerificationErrorV1,
    },
    Pairing {
        key: CanonicalSameModuleCallableKeyV1,
        reason: &'static str,
    },
    PlanKeyCoverageMismatch {
        expected: usize,
        actual: usize,
    },
}

impl GeneralFunctionPlanErrorV1 {
    pub(crate) const fn stage(&self) -> GeneralFunctionPlanStageV1 {
        match self {
            Self::NoInstanceMethod => GeneralFunctionPlanStageV1::Inventory,
            Self::Source { .. } | Self::UnsupportedSignature { .. } => {
                GeneralFunctionPlanStageV1::Source
            }
            Self::ResolverSession(_)
            | Self::Resolver { .. }
            | Self::Projection { .. }
            | Self::Input { .. }
            | Self::FactCoverage { .. } => GeneralFunctionPlanStageV1::Resolve,
            Self::UnsupportedBody { .. } => GeneralFunctionPlanStageV1::Recipe,
            Self::Completion { .. } => GeneralFunctionPlanStageV1::Completion,
            Self::Pairing { .. } | Self::PlanKeyCoverageMismatch { .. } => {
                GeneralFunctionPlanStageV1::Pairing
            }
        }
    }
}

#[derive(Debug)]
pub(crate) struct RejectedGeneralFunctionPlanSetV1 {
    owner: VerifiedNormalModuleSourceV1,
    error: GeneralFunctionPlanErrorV1,
}

impl RejectedGeneralFunctionPlanSetV1 {
    pub(crate) fn source_identity(&self) -> &str {
        self.owner.source_identity()
    }

    pub(crate) const fn stage(&self) -> GeneralFunctionPlanStageV1 {
        self.error.stage()
    }

    pub(crate) const fn error(&self) -> &GeneralFunctionPlanErrorV1 {
        &self.error
    }

    pub(crate) fn discard(self) {
        drop(self);
    }
}

impl VerifiedNormalModuleSourceV1 {
    pub(crate) fn seal_instance_integer_return_plans(
        self,
    ) -> Result<VerifiedNormalInstanceIntegerReturnPlanSetV1, RejectedGeneralFunctionPlanSetV1>
    {
        let keys = self
            .callable_catalog()
            .keys()
            .filter(|key| key.namespace() == SameModuleCallableNamespaceV1::InstanceBoxMethod)
            .cloned()
            .collect::<Vec<_>>();
        if keys.is_empty() {
            return Err(reject(self, GeneralFunctionPlanErrorV1::NoInstanceMethod));
        }
        let mut resolver = match FunctionSemanticResolverSessionV1::new(0) {
            Ok(resolver) => resolver,
            Err(error) => {
                return Err(reject(
                    self,
                    GeneralFunctionPlanErrorV1::ResolverSession(error),
                ))
            }
        };
        let mut plans = BTreeMap::new();
        for key in &keys {
            let view = match self.borrow_instance_method_source(key) {
                Ok(view) => view,
                Err(cause) => {
                    return Err(reject(
                        self,
                        GeneralFunctionPlanErrorV1::Source {
                            key: key.clone(),
                            cause,
                        },
                    ))
                }
            };
            let plan = match seal_one(&mut resolver, view) {
                Ok(plan) => plan,
                Err(error) => return Err(reject(self, error)),
            };
            if plans.insert(key.clone(), plan).is_some() {
                return Err(reject(
                    self,
                    GeneralFunctionPlanErrorV1::Pairing {
                        key: key.clone(),
                        reason: "duplicate_plan_key",
                    },
                ));
            }
        }
        if plans.len() != keys.len() {
            let actual = plans.len();
            return Err(reject(
                self,
                GeneralFunctionPlanErrorV1::PlanKeyCoverageMismatch {
                    expected: keys.len(),
                    actual,
                },
            ));
        }
        Ok(VerifiedNormalInstanceIntegerReturnPlanSetV1 {
            source: self,
            plans,
        })
    }
}

fn seal_one(
    resolver: &mut FunctionSemanticResolverSessionV1,
    view: NormalInstanceMethodSourceViewV1<'_>,
) -> Result<VerifiedNormalInstanceIntegerReturnPlanV1, GeneralFunctionPlanErrorV1> {
    verify_signature(view)?;
    let declaration = view.declaration();
    let receiver_policy =
        SameModuleCallableSourceReceiverPolicyV1::from_namespace(view.key().namespace())
            .into_shadow_policy();
    let syntax = FunctionSyntaxViewV1::from_borrowed_function_parts(
        declaration.params(),
        declaration.body(),
        receiver_policy,
    );
    let forest =
        resolver
            .resolve_forest(syntax)
            .map_err(|cause| GeneralFunctionPlanErrorV1::Resolver {
                key: view.key().clone(),
                cause,
            })?;
    let projection =
        VerifiedSourceProjectionV1::seal(view.function(), &forest).map_err(|cause| {
            GeneralFunctionPlanErrorV1::Projection {
                key: view.key().clone(),
                cause,
            }
        })?;
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
    let recipe = compose_recipe(view.key(), input, receiver)?;
    let completion = verify_function_completion_v1(input).map_err(|cause| {
        GeneralFunctionPlanErrorV1::Completion {
            key: view.key().clone(),
            cause,
        }
    })?;
    verify_pairing(view.key(), &recipe, &completion)?;
    Ok(VerifiedNormalInstanceIntegerReturnPlanV1 {
        facts: VerifiedNormalInstanceFunctionFactsV1 {
            forest,
            projection,
            receiver,
        },
        recipe,
        completion,
    })
}

fn verify_signature(
    view: NormalInstanceMethodSourceViewV1<'_>,
) -> Result<(), GeneralFunctionPlanErrorV1> {
    let declaration = view.declaration();
    let reason = if !declaration.params().is_empty() {
        Some(GeneralFunctionSignatureStopV1::Parameters)
    } else if !declaration.param_decls().is_empty() {
        Some(GeneralFunctionSignatureStopV1::ParameterDeclarations)
    } else if declaration.return_type_name().is_some() {
        Some(GeneralFunctionSignatureStopV1::ReturnAnnotation)
    } else if !declaration.uses().is_empty() {
        Some(GeneralFunctionSignatureStopV1::Uses)
    } else if !declaration.attrs().is_empty() {
        Some(GeneralFunctionSignatureStopV1::Attributes)
    } else {
        None
    };
    match reason {
        Some(reason) => Err(GeneralFunctionPlanErrorV1::UnsupportedSignature {
            key: view.key().clone(),
            reason,
        }),
        None => Ok(()),
    }
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

fn reject(
    owner: VerifiedNormalModuleSourceV1,
    error: GeneralFunctionPlanErrorV1,
) -> RejectedGeneralFunctionPlanSetV1 {
    RejectedGeneralFunctionPlanSetV1 { owner, error }
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
