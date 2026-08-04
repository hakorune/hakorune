//! Exact whole-owner trivial representation analyzer.

use std::collections::{BTreeMap, BTreeSet};

use crate::ast::ASTNode;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::located::{LocatedBodyV1, LocatedExprV1, LocatedStmtV1};
use crate::mir::compiler::normal_source_plan::VerifiedNormalMainRoleV1;
use crate::mir::compiler::source_view::{BodyChildRoleV1, ExprChildRoleV1};
use crate::mir::exact_trivial_return_abi::ExactTrivialReturnAbiV1;
use crate::mir::resolved_control_flow::if_control::VerifiedResolvedFunctionIfControlV1;
use crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1;
use crate::mir::resolved_semantics::{BindingRefV1, SourceBindingSiteV1, SourceStmtSiteV1};

use super::analyzer_policy::{DirectCallPolicyV1, ReturnPolicyV1, RootProfilePolicyV1};
use super::coverage::{
    verify_terminal_completion_co_seal_v1, ResolvedFactCoverageDraftV1, TrivialProfileDraftV1,
};
use super::direct_call::VerifiedTrivialDirectCallV1;
use super::error::{
    stop, stop_expression, stop_statement, AnalysisFailureV1, AnalysisResultV1,
    TrivialProfileContractErrorV1, TrivialProfileStopReasonV1, TrivialProfileStopSiteV1,
};
use super::function_return::seal_function_return_v1;
use super::operator::{
    derive_trivial_binary_profile_v1, derive_trivial_literal_profile_v1,
    TrivialBinaryProfileStopV1, TrivialLiteralProfileStopV1,
};
use super::parameter_entry::seal_parameter_entries_v1;
use super::product::{
    TrivialBindingDefinitionOriginV1, TrivialProfileCoverageSubjectV1, TrivialRepresentationV1,
    TrivialTerminalProfileV1, VerifiedTrivialCanonicalOwnerV1,
};
use super::recipe_facts::{RecipeBranchV1, TrivialIfRecipeFactsDraftV1};
use super::TrivialCanonicalOwnerAnalysisV1;

#[path = "analyzer_branch.rs"]
mod branch;
#[path = "analyzer_support.rs"]
mod support;

pub(super) fn analyze_trivial_canonical_owner_impl_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    completion: &VerifiedFunctionCompletionV1,
    if_control: &VerifiedResolvedFunctionIfControlV1,
) -> Result<TrivialCanonicalOwnerAnalysisV1, TrivialProfileContractErrorV1> {
    analyze_with_policy(
        input,
        completion,
        if_control,
        DirectCallPolicyV1::Forbidden,
        RootProfilePolicyV1::OrdinaryFirstFamily,
    )
}

pub(super) fn analyze_trivial_canonical_main_owner_impl_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    completion: &VerifiedFunctionCompletionV1,
    if_control: &VerifiedResolvedFunctionIfControlV1,
    _role: VerifiedNormalMainRoleV1,
) -> Result<TrivialCanonicalOwnerAnalysisV1, TrivialProfileContractErrorV1> {
    analyze_with_policy(
        input,
        completion,
        if_control,
        DirectCallPolicyV1::Forbidden,
        RootProfilePolicyV1::NormalMain0,
    )
}

pub(super) fn analyze_trivial_canonical_main_owner_with_finite_direct_calls_impl_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    completion: &VerifiedFunctionCompletionV1,
    if_control: &VerifiedResolvedFunctionIfControlV1,
    _role: VerifiedNormalMainRoleV1,
) -> Result<TrivialCanonicalOwnerAnalysisV1, TrivialProfileContractErrorV1> {
    analyze_with_policy(
        input,
        completion,
        if_control,
        DirectCallPolicyV1::FiniteOneOrMore,
        RootProfilePolicyV1::NormalMain0,
    )
}

pub(super) fn analyze_trivial_canonical_owner_with_finite_direct_calls_impl_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    completion: &VerifiedFunctionCompletionV1,
    if_control: &VerifiedResolvedFunctionIfControlV1,
) -> Result<TrivialCanonicalOwnerAnalysisV1, TrivialProfileContractErrorV1> {
    analyze_with_policy(
        input,
        completion,
        if_control,
        DirectCallPolicyV1::FiniteOneOrMore,
        RootProfilePolicyV1::OrdinaryFirstFamily,
    )
}

fn analyze_with_policy(
    input: ResolvedFunctionLoweringInputV1<'_>,
    completion: &VerifiedFunctionCompletionV1,
    if_control: &VerifiedResolvedFunctionIfControlV1,
    direct_call_policy: DirectCallPolicyV1,
    root_profile_policy: RootProfilePolicyV1,
) -> Result<TrivialCanonicalOwnerAnalysisV1, TrivialProfileContractErrorV1> {
    match AnalyzerV1::new(input, if_control, direct_call_policy, root_profile_policy)
        .and_then(|analyzer| analyzer.analyze(completion))
    {
        Ok(product) => Ok(TrivialCanonicalOwnerAnalysisV1::Admitted(product)),
        Err(AnalysisFailureV1::Stop(stop)) => {
            Ok(TrivialCanonicalOwnerAnalysisV1::NotAdmitted(stop))
        }
        Err(AnalysisFailureV1::Contract(error)) => Err(error),
    }
}

type ValueEnvironmentV1 = BTreeMap<BindingRefV1, TrivialRepresentationV1>;

struct AnalyzerV1<'a> {
    input: ResolvedFunctionLoweringInputV1<'a>,
    draft: TrivialProfileDraftV1,
    terminal: Option<TrivialTerminalProfileV1>,
    fact_coverage: ResolvedFactCoverageDraftV1,
    recipe_facts: TrivialIfRecipeFactsDraftV1,
    expected_if_sites: BTreeSet<SourceStmtSiteV1>,
    visited_if_sites: BTreeSet<SourceStmtSiteV1>,
    direct_call_policy: DirectCallPolicyV1,
    root_profile_policy: RootProfilePolicyV1,
    direct_call_count: u32,
}

impl<'a> AnalyzerV1<'a> {
    fn new(
        input: ResolvedFunctionLoweringInputV1<'a>,
        if_control: &VerifiedResolvedFunctionIfControlV1,
        direct_call_policy: DirectCallPolicyV1,
        root_profile_policy: RootProfilePolicyV1,
    ) -> AnalysisResultV1<Self> {
        if if_control.owner() != input.owner() {
            return Err(TrivialProfileContractErrorV1::IfControlOwnerMismatch.into());
        }
        Ok(Self {
            input,
            draft: TrivialProfileDraftV1::new(input.owner()),
            terminal: None,
            fact_coverage: ResolvedFactCoverageDraftV1::new(input.owner()),
            recipe_facts: TrivialIfRecipeFactsDraftV1::default(),
            expected_if_sites: if_control.exact_if_sites().cloned().collect(),
            visited_if_sites: BTreeSet::new(),
            direct_call_policy,
            root_profile_policy,
            direct_call_count: 0,
        })
    }

    fn analyze(
        mut self,
        completion: &VerifiedFunctionCompletionV1,
    ) -> AnalysisResultV1<VerifiedTrivialCanonicalOwnerV1> {
        self.verify_owner_transport()?;
        let requested_return = self.verify_root_profile()?;

        let body = self
            .input
            .source()
            .root_body()
            .map_err(|error| self.source_navigation(error))?;
        let mut environment = ValueEnvironmentV1::new();
        for (binding, representation) in
            seal_parameter_entries_v1(self.input, &mut self.draft, &mut self.fact_coverage)?
        {
            environment.insert(binding, representation);
        }
        let mut writes = BTreeSet::new();
        self.analyze_body(
            &body,
            &mut environment,
            &mut writes,
            ReturnPolicyV1::RootFinalOnly,
        )?;

        match self.direct_call_policy {
            DirectCallPolicyV1::FiniteOneOrMore if self.direct_call_count == 0 => {
                return Err(
                    TrivialProfileContractErrorV1::DirectCallCardinality { actual: 0 }.into(),
                )
            }
            _ => {}
        }

        if self.terminal.is_none() {
            let body_end = u32::try_from(body.statements().len())
                .map_err(|_| TrivialProfileContractErrorV1::TerminalCardinality)?;
            let subject = TrivialProfileCoverageSubjectV1::ImplicitNoValueTerminal {
                body: body.site().clone(),
                body_end,
            };
            self.draft.record_subject(subject)?;
            self.terminal = Some(TrivialTerminalProfileV1::ImplicitNoValue {
                body: body.site().clone(),
                body_end,
            });
        }

        let terminal = self
            .terminal
            .take()
            .ok_or(TrivialProfileContractErrorV1::TerminalCardinality)?;
        verify_terminal_completion_co_seal_v1(self.input.owner(), &terminal, completion)?;
        self.verify_if_control_coverage()?;
        self.fact_coverage.verify(self.input.function())?;
        let parts = self.draft.finish();
        let nested_recipe_facts = self.recipe_facts.nested_candidate();
        let recipe_facts = self.recipe_facts.finish();
        let function_return = seal_function_return_v1(
            self.input.owner(),
            requested_return,
            &terminal,
            &parts.coverage,
        )?;
        Ok(VerifiedTrivialCanonicalOwnerV1::from_verified_parts(
            self.input.owner(),
            parts.parameter_entries,
            parts.values,
            parts.direct_calls,
            parts.definitions,
            parts.merge_profiles,
            terminal,
            function_return,
            parts.coverage,
            recipe_facts,
            nested_recipe_facts,
        ))
    }

    fn verify_owner_transport(&self) -> AnalysisResultV1<()> {
        if self.input.owner() != self.input.source().owner()
            || self.input.owner() != self.input.function().owner()
            || self.input.forest().owner(self.input.owner()).is_none()
        {
            return Err(TrivialProfileContractErrorV1::OwnerTransportMismatch.into());
        }
        Ok(())
    }

    fn verify_root_profile(&self) -> AnalysisResultV1<Option<ExactTrivialReturnAbiV1>> {
        let ASTNode::FunctionDeclaration {
            name,
            return_type_name,
            uses,
            contracts,
            is_static,
            is_override,
            attrs,
            ..
        } = self.input.source().root()
        else {
            return Err(TrivialProfileContractErrorV1::InvalidFunctionRoot.into());
        };
        let owner_site = TrivialProfileStopSiteV1::Owner(self.input.owner());
        let role_mismatch = match self.root_profile_policy {
            RootProfilePolicyV1::OrdinaryFirstFamily => {
                !*is_static || *is_override || name == "main"
            }
            RootProfilePolicyV1::NormalMain0 => !*is_static || *is_override || name != "main",
        };
        if role_mismatch {
            return stop(
                owner_site,
                TrivialProfileStopReasonV1::OwnerFamilyOutsideProfile,
            );
        }
        if !uses.is_empty() || !contracts.is_empty() || !attrs.is_empty() {
            return stop(
                owner_site,
                TrivialProfileStopReasonV1::FunctionMetadataOutsideProfile,
            );
        }
        match (self.root_profile_policy, return_type_name.as_deref()) {
            (RootProfilePolicyV1::NormalMain0, Some("void")) => Ok(None),
            (_, None) => Ok(None),
            (_, Some(source_type_name)) => ExactTrivialReturnAbiV1::classify(source_type_name)
                .map(Some)
                .ok_or_else(|| {
                    AnalysisFailureV1::Stop(super::error::TrivialProfileStopV1::new(
                        owner_site,
                        TrivialProfileStopReasonV1::TypedSignatureOutsideProfile,
                    ))
                }),
        }
    }

    fn analyze_body(
        &mut self,
        body: &LocatedBodyV1<'a>,
        environment: &mut ValueEnvironmentV1,
        writes: &mut BTreeSet<BindingRefV1>,
        return_policy: ReturnPolicyV1,
    ) -> AnalysisResultV1<()> {
        for index in 0..body.statements().len() {
            let statement = self
                .input
                .source()
                .body_stmt(body, index)
                .map_err(|error| self.source_navigation(error))?;
            self.analyze_statement(
                &statement,
                environment,
                writes,
                return_policy,
                index + 1 == body.statements().len(),
            )?;
        }
        Ok(())
    }

    fn analyze_statement(
        &mut self,
        statement: &LocatedStmtV1<'a>,
        environment: &mut ValueEnvironmentV1,
        writes: &mut BTreeSet<BindingRefV1>,
        return_policy: ReturnPolicyV1,
        is_last: bool,
    ) -> AnalysisResultV1<()> {
        match statement.node() {
            ASTNode::Local {
                variables,
                initial_values,
                declared_type_names,
                ..
            } => {
                if self.recipe_facts.in_branch() {
                    self.recipe_facts.mark_unsupported();
                }
                if variables.is_empty()
                    || variables.len() != initial_values.len()
                    || variables.len() != declared_type_names.len()
                {
                    return stop_statement(
                        statement,
                        TrivialProfileStopReasonV1::StatementOutsideProfile,
                    );
                }
                if declared_type_names.iter().any(Option::is_some) {
                    return stop_statement(
                        statement,
                        TrivialProfileStopReasonV1::DeclaredLocalTypeOutsideProfile,
                    );
                }
                let mut pending = Vec::with_capacity(variables.len());
                for ordinal in 0..variables.len() {
                    let binding_site = SourceBindingSiteV1::Local {
                        statement: statement.site().clone(),
                        ordinal: ordinal as u32,
                    };
                    if initial_values[ordinal].is_none() {
                        return stop(
                            TrivialProfileStopSiteV1::Binding(binding_site),
                            TrivialProfileStopReasonV1::MissingLocalInitializer,
                        );
                    }
                    let initializer = self
                        .input
                        .source()
                        .child_expr_from_stmt(
                            statement,
                            ExprChildRoleV1::LocalInitializer(ordinal as u32),
                        )
                        .map_err(|error| self.source_navigation(error))?;
                    let representation = self.analyze_expr(&initializer, environment, writes)?;
                    let binding = self
                        .fact_coverage
                        .declaration_binding(self.input.function(), &binding_site)?;
                    pending.push((binding, binding_site, representation));
                }
                // Current canonical local semantics evaluates every initializer
                // before publishing any declaration from the statement.
                for (binding, binding_site, representation) in pending {
                    environment.insert(binding, representation);
                    self.draft.record_definition(
                        binding,
                        TrivialBindingDefinitionOriginV1::Declaration(binding_site),
                        representation,
                    )?;
                }
                Ok(())
            }
            ASTNode::Outbox { .. } => stop_statement(
                statement,
                TrivialProfileStopReasonV1::OutboxRepresentationUnavailable,
            ),
            ASTNode::Assignment { target, .. } => {
                if !matches!(target.as_ref(), ASTNode::Variable { .. }) {
                    return stop_statement(
                        statement,
                        TrivialProfileStopReasonV1::StatementOutsideProfile,
                    );
                }
                let target = self
                    .input
                    .source()
                    .child_expr_from_stmt(statement, ExprChildRoleV1::AssignmentTarget)
                    .map_err(|error| self.source_navigation(error))?;
                let value = self
                    .input
                    .source()
                    .child_expr_from_stmt(statement, ExprChildRoleV1::AssignmentValue)
                    .map_err(|error| self.source_navigation(error))?;
                let representation = self.analyze_expr(&value, environment, writes)?;
                let binding = self
                    .fact_coverage
                    .assignment_binding(self.input.function(), target.site())?;
                environment
                    .get(&binding)
                    .copied()
                    .ok_or(TrivialProfileContractErrorV1::MissingReachingProfile { binding })?;
                environment.insert(binding, representation);
                writes.insert(binding);
                self.draft.record_definition(
                    binding,
                    TrivialBindingDefinitionOriginV1::Assignment(target.site().clone()),
                    representation,
                )?;
                self.recipe_facts.record_assignment(
                    statement.site().clone(),
                    binding,
                    value.site().clone(),
                    representation,
                );
                Ok(())
            }
            ASTNode::If { else_body, .. } => {
                self.claim_if_control(statement)?;
                let condition = self
                    .input
                    .source()
                    .child_expr_from_stmt(statement, ExprChildRoleV1::IfCondition)
                    .map_err(|error| self.source_navigation(error))?;
                self.recipe_facts.begin_if(
                    statement.site().clone(),
                    condition.site().clone(),
                    else_body.is_some(),
                );
                let condition_representation =
                    self.analyze_expr(&condition, environment, writes)?;
                if condition_representation != TrivialRepresentationV1::InlineBool {
                    return stop_expression(
                        &condition,
                        TrivialProfileStopReasonV1::IfConditionNotBool,
                    );
                }
                let baseline = environment.clone();
                let then_body = self
                    .input
                    .source()
                    .child_body_from_stmt(statement, BodyChildRoleV1::IfThen)
                    .map_err(|error| self.source_navigation(error))?;
                let else_body = if else_body.is_some() {
                    Some(
                        self.input
                            .source()
                            .child_body_from_stmt(statement, BodyChildRoleV1::IfElse)
                            .map_err(|error| self.source_navigation(error))?,
                    )
                } else {
                    None
                };
                self.recipe_facts.set_bodies(
                    then_body.site().clone(),
                    else_body.as_ref().map(|body| body.site().clone()),
                );
                let (then_environment, then_writes) =
                    self.analyze_branch(&then_body, &baseline, RecipeBranchV1::Then)?;
                let (else_environment, else_writes) = if let Some(else_body) = else_body {
                    self.analyze_branch(&else_body, &baseline, RecipeBranchV1::Else)?
                } else {
                    (baseline.clone(), BTreeSet::new())
                };
                let merge_bindings = then_writes
                    .union(&else_writes)
                    .copied()
                    .collect::<BTreeSet<_>>();
                let merge_binding_list = merge_bindings.iter().copied().collect::<Vec<_>>();
                self.recipe_facts.finish_if(&merge_binding_list, &baseline);
                *environment = baseline;
                for binding in merge_bindings.iter().copied() {
                    let then_representation = then_environment
                        .get(&binding)
                        .copied()
                        .ok_or(TrivialProfileContractErrorV1::MissingReachingProfile { binding })?;
                    let else_representation = else_environment
                        .get(&binding)
                        .copied()
                        .ok_or(TrivialProfileContractErrorV1::MissingReachingProfile { binding })?;
                    if then_representation != else_representation {
                        return stop_statement(
                            statement,
                            TrivialProfileStopReasonV1::IfMergeProfileNotHomogeneous,
                        );
                    }
                    environment.insert(binding, then_representation);
                    writes.insert(binding);
                    self.draft.record_merge_profile(
                        statement.site().clone(),
                        binding,
                        then_representation,
                    )?;
                }
                Ok(())
            }
            ASTNode::Return { value, .. } => {
                if return_policy == ReturnPolicyV1::Forbidden {
                    return stop_statement(
                        statement,
                        TrivialProfileStopReasonV1::ReturnInsideFallthroughBranch,
                    );
                }
                if !is_last {
                    return stop_statement(statement, TrivialProfileStopReasonV1::ReturnNotFinal);
                }
                if self.terminal.is_some() {
                    return Err(TrivialProfileContractErrorV1::TerminalCardinality.into());
                }
                if value.is_some() {
                    let value = self
                        .input
                        .source()
                        .child_expr_from_stmt(statement, ExprChildRoleV1::ReturnValue)
                        .map_err(|error| self.source_navigation(error))?;
                    let representation = self.analyze_expr(&value, environment, writes)?;
                    let main_unit_value = self.root_profile_policy
                        == RootProfilePolicyV1::NormalMain0
                        && matches!(
                            representation,
                            TrivialRepresentationV1::ExplicitVoidValue
                                | TrivialRepresentationV1::NullSentinel
                        );
                    if representation == TrivialRepresentationV1::NullSentinel && !main_unit_value {
                        return stop_expression(
                            &value,
                            TrivialProfileStopReasonV1::NullRepresentationUnavailable,
                        );
                    }
                    if main_unit_value {
                        self.draft.record_subject(
                            TrivialProfileCoverageSubjectV1::ExplicitNoValueTerminal(
                                statement.site().clone(),
                            ),
                        )?;
                        self.terminal = Some(TrivialTerminalProfileV1::ExplicitNoValue {
                            statement: statement.site().clone(),
                        });
                    } else {
                        self.draft.record_subject(
                            TrivialProfileCoverageSubjectV1::ExplicitValueTerminal(
                                statement.site().clone(),
                            ),
                        )?;
                        self.terminal = Some(TrivialTerminalProfileV1::ExplicitValue {
                            statement: statement.site().clone(),
                            value: value.site().clone(),
                            representation,
                        });
                    }
                } else {
                    self.draft.record_subject(
                        TrivialProfileCoverageSubjectV1::ExplicitNoValueTerminal(
                            statement.site().clone(),
                        ),
                    )?;
                    self.terminal = Some(TrivialTerminalProfileV1::ExplicitNoValue {
                        statement: statement.site().clone(),
                    });
                }
                Ok(())
            }
            ASTNode::Literal { .. }
            | ASTNode::Variable { .. }
            | ASTNode::BinaryOp { .. }
            | ASTNode::BlockExpr { .. } => {
                let expression = self
                    .input
                    .source()
                    .statement_expression(statement)
                    .map_err(|error| self.source_navigation(error))?;
                self.analyze_expr(&expression, environment, writes)?;
                Ok(())
            }
            _ => stop_statement(
                statement,
                TrivialProfileStopReasonV1::StatementOutsideProfile,
            ),
        }
    }

    fn analyze_expr(
        &mut self,
        expression: &LocatedExprV1<'a>,
        environment: &mut ValueEnvironmentV1,
        writes: &mut BTreeSet<BindingRefV1>,
    ) -> AnalysisResultV1<TrivialRepresentationV1> {
        let representation = match expression.node() {
            ASTNode::Literal { value, .. } => match derive_trivial_literal_profile_v1(value) {
                Ok(profile) => {
                    self.recipe_facts
                        .record_literal(expression.site().clone(), value, profile);
                    profile
                }
                Err(TrivialLiteralProfileStopV1::String) => {
                    return stop_expression(
                        expression,
                        TrivialProfileStopReasonV1::StringRepresentationUnavailable,
                    )
                }
            },
            ASTNode::Variable { .. } => {
                let binding = self
                    .fact_coverage
                    .variable_binding(self.input.function(), expression.site())?;
                let representation = environment
                    .get(&binding)
                    .copied()
                    .ok_or(TrivialProfileContractErrorV1::MissingReachingProfile { binding })?;
                self.recipe_facts
                    .record_read(expression.site().clone(), binding, representation);
                representation
            }
            ASTNode::BinaryOp { operator, .. } => {
                let left_expr = self
                    .input
                    .source()
                    .child_expr_from_expr(expression, ExprChildRoleV1::BinaryLeft)
                    .map_err(|error| self.source_navigation(error))?;
                let right_expr = self
                    .input
                    .source()
                    .child_expr_from_expr(expression, ExprChildRoleV1::BinaryRight)
                    .map_err(|error| self.source_navigation(error))?;
                let left = self.analyze_expr(&left_expr, environment, writes)?;
                let right = self.analyze_expr(&right_expr, environment, writes)?;
                let representation = match derive_trivial_binary_profile_v1(operator, left, right) {
                    Ok(profile) => profile,
                    Err(TrivialBinaryProfileStopV1::OperatorOutsideProfile) => {
                        return stop_expression(
                            expression,
                            TrivialProfileStopReasonV1::BinaryOperatorOutsideProfile,
                        )
                    }
                    Err(TrivialBinaryProfileStopV1::OperandsNotExact) => {
                        return stop_expression(
                            expression,
                            TrivialProfileStopReasonV1::BinaryOperandsNotExact,
                        )
                    }
                };
                self.recipe_facts.record_binary(
                    expression.site().clone(),
                    operator,
                    left_expr.site().clone(),
                    right_expr.site().clone(),
                    representation,
                );
                representation
            }
            ASTNode::BlockExpr { .. } => {
                self.recipe_facts.mark_unsupported();
                self.input
                    .function()
                    .block_expr_scope_region_pair(expression.owner(), expression.site())
                    .map_err(|_| TrivialProfileContractErrorV1::BlockExprPairNotSealed {
                        site: expression.site().clone(),
                    })?;
                let baseline = environment.clone();
                let prelude = self
                    .input
                    .source()
                    .child_body_from_expr(expression, BodyChildRoleV1::BlockExprPrelude)
                    .map_err(|error| self.source_navigation(error))?;
                let mut inner_writes = BTreeSet::new();
                self.analyze_body(
                    &prelude,
                    environment,
                    &mut inner_writes,
                    ReturnPolicyV1::Forbidden,
                )?;
                let tail = self
                    .input
                    .source()
                    .child_expr_from_expr(expression, ExprChildRoleV1::BlockExprTail)
                    .map_err(|error| self.source_navigation(error))?;
                let result = self.analyze_expr(&tail, environment, &mut inner_writes)?;
                environment.retain(|binding, _| baseline.contains_key(binding));
                inner_writes.retain(|binding| baseline.contains_key(binding));
                writes.extend(inner_writes);
                result
            }
            ASTNode::FunctionCall {
                name, arguments, ..
            } => {
                match self.direct_call_policy {
                    DirectCallPolicyV1::Forbidden => {
                        return stop_expression(
                            expression,
                            TrivialProfileStopReasonV1::ExpressionOutsideProfile,
                        )
                    }
                    DirectCallPolicyV1::FiniteOneOrMore => {}
                }
                let mut argument_sites = Vec::with_capacity(arguments.len());
                for index in 0..arguments.len() {
                    let ordinal = u32::try_from(index).map_err(|_| {
                        TrivialProfileContractErrorV1::DirectCallHeaderMismatch {
                            site: expression.site().clone(),
                        }
                    })?;
                    let argument = self
                        .input
                        .source()
                        .child_expr_from_expr(expression, ExprChildRoleV1::CallArgument(ordinal))
                        .map_err(|error| self.source_navigation(error))?;
                    let representation = self.analyze_expr(&argument, environment, writes)?;
                    if representation != TrivialRepresentationV1::InlineI64 {
                        return stop_expression(
                            &argument,
                            TrivialProfileStopReasonV1::BinaryOperandsNotExact,
                        );
                    }
                    argument_sites.push(argument.site().clone());
                }
                if self.direct_call_policy == DirectCallPolicyV1::FiniteOneOrMore {
                    self.direct_call_count =
                        self.direct_call_count.checked_add(1).ok_or_else(|| {
                            TrivialProfileContractErrorV1::DirectCallCardinalityOverflow {
                                site: expression.site().clone(),
                            }
                        })?;
                }
                self.fact_coverage
                    .direct_call_target(self.input.function(), expression.site())?;
                let index = self
                    .input
                    .callable_index()
                    .ok_or(TrivialProfileContractErrorV1::MissingCallableIndex)?;
                let row = VerifiedTrivialDirectCallV1::seal(
                    self.input.owner(),
                    expression.site().clone(),
                    name,
                    argument_sites,
                    self.input.function(),
                    index,
                )?;
                self.draft.record_direct_call(row)?;
                self.recipe_facts.record_direct_call(expression.site().clone());
                return Ok(TrivialRepresentationV1::InlineI64);
            }
            _ => {
                return stop_expression(
                    expression,
                    TrivialProfileStopReasonV1::ExpressionOutsideProfile,
                )
            }
        };
        self.draft
            .record_value(expression.site().clone(), representation)?;
        Ok(representation)
    }
}
