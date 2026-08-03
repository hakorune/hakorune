//! Located materializer for the whole-owner trivial Binding SSA route.

use crate::ast::{ASTNode, LiteralValue};
use crate::mir::canonical_direct_static_call_capability::CanonicalDirectStaticCallCapabilityV1;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::located::{LocatedBodyV1, LocatedExprV1, LocatedStmtV1};
use crate::mir::compiler::source_view::{BodyChildRoleV1, ExprChildRoleV1};
use crate::mir::resolved_control_flow::if_control::{
    ResolvedIfControlMaterializationV1, ResolvedIfElsePortV1, VerifiedResolvedFunctionIfControlV1,
};
use crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1;
use crate::mir::resolved_semantics::{
    BindingKindV1, RegionKindV1, ResolvedExitSiteV1, ScopeKindV1, SourceBindingSiteV1,
};
use crate::mir::resolved_value_profile::product::{
    TrivialRepresentationV1, VerifiedTrivialCanonicalOwnerV1,
};
use crate::mir::resolved_value_profile::TrivialProfileConsumptionV1;
use crate::mir::{BasicBlockId, MirType, ValueId};

use super::super::completion_consumption::ReadyFunctionCompletionV1;
use super::super::if_recipe_adapter::{
    CanonicalIfPhysicalDemandV1, CanonicalIfRecipeAdmissionDispositionV1,
};
use super::super::MirBuilder;
use super::operation::{emit_binary, mir_type};
use super::parameter_entry::publish_parameter_entries_v1;
use crate::mir::builder::resolved_lowering::canonical_ssa::CanonicalSsaFunctionSessionV2;

pub(in crate::mir::builder::resolved_lowering) struct CanonicalTrivialSsaLowererV1<
    'builder,
    'source,
> {
    builder: &'builder mut MirBuilder,
    input: ResolvedFunctionLoweringInputV1<'source>,
    session: CanonicalSsaFunctionSessionV2<'source>,
    profile: TrivialProfileConsumptionV1,
    if_recipe: CanonicalIfRecipeAdmissionDispositionV1,
}

impl<'builder, 'source> CanonicalTrivialSsaLowererV1<'builder, 'source> {
    pub(in crate::mir::builder::resolved_lowering) fn new(
        builder: &'builder mut MirBuilder,
        input: ResolvedFunctionLoweringInputV1<'source>,
        if_control: VerifiedResolvedFunctionIfControlV1,
        completion: VerifiedFunctionCompletionV1,
        profile: VerifiedTrivialCanonicalOwnerV1,
        block_expr_count: usize,
        if_recipe: CanonicalIfRecipeAdmissionDispositionV1,
    ) -> Result<Self, String> {
        if !builder
            .function_state
            .resolved_binding_state
            .is_installed_for(input.owner())
        {
            return Err(
                "[freeze:contract][canonical_binding_ssa/authority_not_installed]".to_string(),
            );
        }
        if if_control.owner() != input.owner() || profile.owner() != input.owner() {
            return Err("[freeze:contract][canonical_binding_ssa/owner_mismatch]".to_string());
        }
        let requires_direct_call_capability = !profile.direct_calls().is_empty();
        let function = builder
            .function_state
            .current_function
            .as_mut()
            .ok_or_else(|| {
                "[freeze:contract][canonical_direct_call/function_missing]".to_string()
            })?;
        CanonicalDirectStaticCallCapabilityV1::install_for_function(
            &mut function.metadata.canonical_direct_static_call_capabilities,
            requires_direct_call_capability,
        )
        .map_err(str::to_string)?;
        let session =
            CanonicalSsaFunctionSessionV2::new(input, if_control, completion, block_expr_count)?;
        Ok(Self {
            builder,
            input,
            session,
            profile: TrivialProfileConsumptionV1::new(profile),
            if_recipe,
        })
    }

    pub(in crate::mir::builder::resolved_lowering) fn lower(
        mut self,
    ) -> Result<ReadyFunctionCompletionV1, String> {
        publish_parameter_entries_v1(self.builder, &mut self.session.identity, &mut self.profile)?;
        let body = self
            .input
            .source()
            .root_body()
            .map_err(|error| error.to_string())?;
        self.lower_body(&body, None)?;
        let body_end = u32::try_from(body.statements().len()).map_err(|_| {
            "[freeze:contract][canonical_binding_ssa/body_length_overflow]".to_string()
        })?;
        if self.session.completion_is_implicit() {
            self.profile
                .claim_terminal_implicit_no_value(body.site(), body_end)?;
        }
        self.seal_current_if_needed()?;

        self.session.semantics.finish()?;
        self.finish_cfg()?;
        self.profile.finish()?;
        self.session
            .if_control
            .finish()
            .map_err(|error| format!("[freeze:contract][if_control/finish] {error:?}"))?;
        self.if_recipe
            .finish()
            .map_err(|error| format!("[freeze:contract][if_recipe/finish] {error:?}"))?;
        self.session.identity.finish()?;
        self.session
            .phis
            .commit(self.builder)
            .map_err(|error| error.to_string())?;
        self.builder
            .function_state
            .resolved_binding_state
            .finish(self.input.owner())?;
        self.session.completion.finish(
            body.site(),
            body_end,
            self.session.semantics.function_region(),
        )
    }

    fn completion_is_implicit(&self) -> bool {
        self.session.completion_is_implicit()
    }

    fn lower_body(
        &mut self,
        body: &LocatedBodyV1<'source>,
        mut coverage: Option<&mut ResolvedIfControlMaterializationV1>,
    ) -> Result<(), String> {
        for index in 0..body.statements().len() {
            let statement = self
                .input
                .source()
                .body_stmt(body, index)
                .map_err(|error| error.to_string())?;
            self.lower_stmt(&statement, coverage.as_deref_mut())?;
        }
        Ok(())
    }

    fn lower_stmt(
        &mut self,
        statement: &LocatedStmtV1<'source>,
        mut coverage: Option<&mut ResolvedIfControlMaterializationV1>,
    ) -> Result<(), String> {
        self.builder
            .metadata_ctx
            .set_current_span(statement.node().span());
        if matches!(statement.node(), ASTNode::If { .. }) {
            return self.lower_if(statement);
        }
        if let Some(row) = coverage.as_deref_mut() {
            row.claim_statement(statement)
                .map_err(|error| format!("[freeze:contract][if_control/statement] {error:?}"))?;
        }
        match statement.node() {
            ASTNode::Local {
                variables,
                initial_values,
                ..
            } => self.lower_local(statement, variables, initial_values, coverage),
            ASTNode::Assignment { .. } => self.lower_assignment(statement, coverage),
            ASTNode::Return { .. } => {
                if self.session.completion.returns_value() {
                    let value = self
                        .input
                        .source()
                        .child_expr_from_stmt(statement, ExprChildRoleV1::ReturnValue)
                        .map_err(|error| error.to_string())?;
                    let (return_value, _) = self.lower_expr(&value, coverage.as_deref_mut())?;
                    self.profile
                        .claim_terminal_explicit_value(statement.site(), value.site())?;
                    let block = self.builder.function_state.current_block.ok_or_else(|| {
                        "[freeze:contract][canonical_completion/current_block_missing]".to_string()
                    })?;
                    self.session.completion.claim_explicit_return(
                        statement.site(),
                        self.session.semantics.function_region(),
                        block,
                        return_value,
                    )?;
                } else {
                    if matches!(statement.node(), ASTNode::Return { value: Some(_), .. }) {
                        let value = self
                            .input
                            .source()
                            .child_expr_from_stmt(statement, ExprChildRoleV1::ReturnValue)
                            .map_err(|error| error.to_string())?;
                        self.lower_expr(&value, coverage.as_deref_mut())?;
                    }
                    self.profile
                        .claim_terminal_explicit_no_value(statement.site())?;
                    self.session.completion.claim_explicit_unit(
                        statement.site(),
                        self.session.semantics.function_region(),
                    )?;
                }
                self.session
                    .identity
                    .mark_return(ResolvedExitSiteV1::Statement(statement.site().clone()))?;
                Ok(())
            }
            ASTNode::Literal { .. }
            | ASTNode::Variable { .. }
            | ASTNode::BinaryOp { .. }
            | ASTNode::BlockExpr { .. } => self
                .input
                .source()
                .statement_expression(statement)
                .map_err(|error| error.to_string())
                .and_then(|expression| {
                    self.lower_expr(&expression, coverage.as_deref_mut())
                        .map(|_| ())
                }),
            _ => Err("[freeze:contract][canonical_binding_ssa/statement_outside_profile]".into()),
        }
    }

    fn lower_local(
        &mut self,
        statement: &LocatedStmtV1<'source>,
        variables: &[String],
        initial_values: &[Option<Box<ASTNode>>],
        mut coverage: Option<&mut ResolvedIfControlMaterializationV1>,
    ) -> Result<(), String> {
        let mut pending = Vec::with_capacity(variables.len());
        for (ordinal, initial) in initial_values.iter().enumerate() {
            if initial.is_none() {
                return Err(
                    "[freeze:contract][canonical_binding_ssa/uninitialized_local]".to_string(),
                );
            }
            let expression = self
                .input
                .source()
                .child_expr_from_stmt(statement, ExprChildRoleV1::LocalInitializer(ordinal as u32))
                .map_err(|error| error.to_string())?;
            let (value, representation) = self.lower_expr(&expression, coverage.as_deref_mut())?;
            pending.push((ordinal, value, representation));
        }
        let block = self.current_block()?;
        for (name, (ordinal, value, actual)) in variables.iter().zip(pending) {
            let site = SourceBindingSiteV1::Local {
                statement: statement.site().clone(),
                ordinal: ordinal as u32,
            };
            let binding = self.session.identity.publish_declaration(
                &site,
                BindingKindV1::Local {
                    ordinal: ordinal as u32,
                },
                name,
                block,
                value,
            )?;
            let expected = self.profile.claim_declaration(binding, &site)?;
            require_representation(actual, expected, "declaration")?;
        }
        Ok(())
    }

    fn lower_assignment(
        &mut self,
        statement: &LocatedStmtV1<'source>,
        mut coverage: Option<&mut ResolvedIfControlMaterializationV1>,
    ) -> Result<(), String> {
        let ASTNode::Assignment { target, .. } = statement.node() else {
            unreachable!("assignment helper requires assignment")
        };
        let ASTNode::Variable { name, .. } = target.as_ref() else {
            unreachable!("preflight accepts binding assignment only")
        };
        let target = self
            .input
            .source()
            .child_expr_from_stmt(statement, ExprChildRoleV1::AssignmentTarget)
            .map_err(|error| error.to_string())?;
        if let Some(row) = coverage.as_deref_mut() {
            row.claim_expression(&target)
                .map_err(|error| format!("[freeze:contract][if_control/target] {error:?}"))?;
        }
        let binding = self
            .session
            .identity
            .resolve_assignment_binding(target.site(), name)?;
        let value = self
            .input
            .source()
            .child_expr_from_stmt(statement, ExprChildRoleV1::AssignmentValue)
            .map_err(|error| error.to_string())?;
        let (value, actual) = self.lower_expr(&value, coverage)?;
        let block = self.current_block()?;
        self.session
            .identity
            .define_assignment(target.site(), binding, block, value)?;
        let expected = self.profile.claim_assignment(binding, target.site())?;
        require_representation(actual, expected, "assignment")
    }

    fn lower_expr(
        &mut self,
        expression: &LocatedExprV1<'source>,
        mut coverage: Option<&mut ResolvedIfControlMaterializationV1>,
    ) -> Result<(ValueId, TrivialRepresentationV1), String> {
        self.builder
            .metadata_ctx
            .set_current_span(expression.node().span());
        if let Some(row) = coverage.as_deref_mut() {
            row.claim_expression(expression)
                .map_err(|error| format!("[freeze:contract][if_control/expression] {error:?}"))?;
        }
        let (value, derived) = match expression.node() {
            ASTNode::Literal { value, .. } => (self.lower_literal(value)?, None),
            ASTNode::Variable { name, .. } => {
                let block = self.current_block()?;
                let (_, value) = self.session.identity.variable_value(
                    self.builder,
                    &mut self.session.phis,
                    block,
                    expression.site(),
                    name,
                )?;
                (value, None)
            }
            ASTNode::BinaryOp { operator, .. } => {
                let left = self
                    .input
                    .source()
                    .child_expr_from_expr(expression, ExprChildRoleV1::BinaryLeft)
                    .map_err(|error| error.to_string())?;
                let right = self
                    .input
                    .source()
                    .child_expr_from_expr(expression, ExprChildRoleV1::BinaryRight)
                    .map_err(|error| error.to_string())?;
                let (lhs, _) = self.lower_expr(&left, coverage.as_deref_mut())?;
                let (rhs, _) = self.lower_expr(&right, coverage.as_deref_mut())?;
                let expected = self.profile.claim_value(expression.site())?;
                let value = emit_binary(self.builder, operator, lhs, rhs, expected)?;
                return Ok((value, expected));
            }
            ASTNode::BlockExpr { .. } => {
                let result = self.lower_block_expr(expression, coverage.as_deref_mut())?;
                (result.0, Some(result.1))
            }
            ASTNode::FunctionCall { arguments, .. } => {
                let mut argument_values = Vec::with_capacity(arguments.len());
                let mut argument_sites = Vec::with_capacity(arguments.len());
                for index in 0..arguments.len() {
                    let index = u32::try_from(index).map_err(|_| {
                        "[freeze:contract][canonical_direct_call/argument_index_overflow]"
                            .to_string()
                    })?;
                    let argument = self
                        .input
                        .source()
                        .child_expr_from_expr(expression, ExprChildRoleV1::CallArgument(index))
                        .map_err(|error| error.to_string())?;
                    let (value, representation) =
                        self.lower_expr(&argument, coverage.as_deref_mut())?;
                    require_representation(
                        representation,
                        TrivialRepresentationV1::InlineI64,
                        "direct_call_argument",
                    )?;
                    argument_sites.push(argument.site().clone());
                    argument_values.push(value);
                }
                let row = self.profile.claim_direct_call(expression.site())?;
                if row.arguments() != argument_sites {
                    return Err(
                        "[freeze:contract][canonical_direct_call/argument_site_drift]".to_string(),
                    );
                }
                return super::direct_call::emit(self.builder, self.input, &row, argument_values);
            }
            _ => unreachable!("preflight seals trivial expression grammar"),
        };
        let expected = self.profile.claim_value(expression.site())?;
        if let Some(derived) = derived {
            require_representation(derived, expected, "expression")?;
        }
        ensure_value_representation(self.builder, value, expected)?;
        Ok((value, expected))
    }

    fn lower_block_expr(
        &mut self,
        expression: &LocatedExprV1<'source>,
        mut coverage: Option<&mut ResolvedIfControlMaterializationV1>,
    ) -> Result<(ValueId, TrivialRepresentationV1), String> {
        let pair = self
            .input
            .function()
            .block_expr_scope_region_pair(expression.owner(), expression.site())
            .map_err(|error| format!("[freeze:contract][canonical_scope/pair] {error:?}"))?;
        let prelude = self
            .input
            .source()
            .child_body_from_expr(expression, BodyChildRoleV1::BlockExprPrelude)
            .map_err(|error| error.to_string())?;
        let tail = self
            .input
            .source()
            .child_expr_from_expr(expression, ExprChildRoleV1::BlockExprTail)
            .map_err(|error| error.to_string())?;
        if let Some(row) = coverage.as_deref_mut() {
            row.claim_body(&prelude)
                .map_err(|error| format!("[freeze:contract][if_control/body] {error:?}"))?;
        }
        let session = self
            .session
            .semantics
            .enter_block_expr(self.input.function(), pair)?;
        self.lower_body(&prelude, coverage.as_deref_mut())?;
        let result = self.lower_expr(&tail, coverage)?;
        self.session
            .semantics
            .close_scope_region_success(session, &mut self.session.identity)?;
        Ok(result)
    }

    fn lower_if(&mut self, statement: &LocatedStmtV1<'source>) -> Result<(), String> {
        if self.if_recipe.is_not_selected() {
            return self.lower_if_materialization(statement, None);
        }
        let demand = self
            .if_recipe
            .take_if(statement)
            .map_err(|error| format!("[freeze:contract][if_recipe/take] {error:?}"))?;
        super::if_recipe_physicalizer::physicalize_if_recipe_v1(self, statement, demand)
            .map(|_| ())
    }

    pub(super) fn lower_if_materialization(
        &mut self,
        statement: &LocatedStmtV1<'source>,
        selected_explicit_else: Option<bool>,
    ) -> Result<(), String> {
        let ASTNode::If { else_body, .. } = statement.node() else {
            unreachable!("If helper requires If")
        };
        let mut row = self
            .session
            .if_control
            .claim(statement)
            .map_err(|error| format!("[freeze:contract][if_control/claim] {error:?}"))?;
        row.claim_statement(statement)
            .map_err(|error| format!("[freeze:contract][if_control/statement] {error:?}"))?;
        let condition = self
            .input
            .source()
            .child_expr_from_stmt(statement, ExprChildRoleV1::IfCondition)
            .map_err(|error| error.to_string())?;
        let (condition, representation) = self.lower_expr(&condition, Some(&mut row))?;
        require_representation(
            representation,
            TrivialRepresentationV1::InlineBool,
            "if_condition",
        )?;

        let regions = row.regions();
        let control = self.session.semantics.enter_region(
            self.input.function(),
            regions.control(),
            RegionKindV1::If,
        )?;
        let header = self.current_block()?;
        let then_block = self.builder.next_block_id();
        let explicit_else = selected_explicit_else
            .unwrap_or_else(|| matches!(row.else_port(), ResolvedIfElsePortV1::Explicit(_)));
        let else_block = explicit_else.then(|| self.builder.next_block_id());
        let merge = self.builder.next_block_id();
        self.builder.ensure_block_exists(then_block)?;
        if let Some(block) = else_block {
            self.builder.ensure_block_exists(block)?;
        }
        self.builder.ensure_block_exists(merge)?;
        let false_target = else_block.unwrap_or(merge);
        {
            let cfg = &self.session.cfg;
            let function = self
                .builder
                .function_state
                .current_function
                .as_mut()
                .ok_or_else(|| {
                    "[freeze:contract][canonical_binding_ssa/function_missing]".to_string()
                })?;
            cfg.emit_branch(function, header, condition, then_block, false_target)
                .map_err(|error| error.to_string())?;
        }
        self.seal_block_if_needed(header)?;

        self.seal_block_if_needed(then_block)?;
        self.builder.start_new_block(then_block)?;
        let then_body = self
            .input
            .source()
            .child_body_from_stmt(statement, BodyChildRoleV1::IfThen)
            .map_err(|error| error.to_string())?;
        row.claim_body(&then_body)
            .map_err(|error| format!("[freeze:contract][if_control/then_body] {error:?}"))?;
        let then_scope = self.session.semantics.enter_scope_region(
            self.input.function(),
            regions.then_pair(),
            ScopeKindV1::IfThen,
            RegionKindV1::IfThen,
        )?;
        self.lower_body(&then_body, Some(&mut row))?;
        self.session
            .semantics
            .close_scope_region_success(then_scope, &mut self.session.identity)?;
        let then_exit = self.current_block()?;
        self.emit_jump(then_exit, merge)?;

        if let Some(else_block) = else_block {
            self.seal_block_if_needed(else_block)?;
            self.builder.start_new_block(else_block)?;
            let else_body = self
                .input
                .source()
                .child_body_from_stmt(statement, BodyChildRoleV1::IfElse)
                .map_err(|error| error.to_string())?;
            row.claim_body(&else_body)
                .map_err(|error| format!("[freeze:contract][if_control/else_body] {error:?}"))?;
            let pair = regions.else_pair().ok_or_else(|| {
                "[freeze:contract][canonical_binding_ssa/else_pair_missing]".to_string()
            })?;
            let else_scope = self.session.semantics.enter_scope_region(
                self.input.function(),
                pair,
                ScopeKindV1::IfElse,
                RegionKindV1::IfElse,
            )?;
            self.lower_body(&else_body, Some(&mut row))?;
            self.session
                .semantics
                .close_scope_region_success(else_scope, &mut self.session.identity)?;
            let else_exit = self.current_block()?;
            self.emit_jump(else_exit, merge)?;
        } else if else_body.is_some() || regions.else_pair().is_some() {
            return Err("[freeze:contract][canonical_binding_ssa/else_topology]".to_string());
        }

        self.seal_block_if_needed(merge)?;
        self.builder.start_new_block(merge)?;
        row.finish_coverage()
            .map_err(|error| format!("[freeze:contract][if_control/coverage] {error:?}"))?;
        let _representation_only = self.profile.claim_if_merges(statement.site())?;
        self.session.semantics.close_region(control)
    }

    fn emit_jump(&mut self, source: BasicBlockId, target: BasicBlockId) -> Result<(), String> {
        let cfg = &self.session.cfg;
        let function = self
            .builder
            .function_state
            .current_function
            .as_mut()
            .ok_or_else(|| {
                "[freeze:contract][canonical_binding_ssa/function_missing]".to_string()
            })?;
        cfg.emit_jump(function, source, target)
            .map_err(|error| error.to_string())
    }

    fn seal_current_if_needed(&mut self) -> Result<(), String> {
        let block = self.current_block()?;
        self.seal_block_if_needed(block)
    }

    fn seal_block_if_needed(&mut self, block: BasicBlockId) -> Result<(), String> {
        let already_sealed = self
            .builder
            .function_state
            .current_function
            .as_ref()
            .and_then(|function| function.get_block(block))
            .ok_or_else(|| {
                format!("[freeze:contract][canonical_binding_ssa/block_missing] block={block:?}")
            })?
            .is_sealed();
        if already_sealed {
            return Ok(());
        }
        let witness = {
            let cfg = &mut self.session.cfg;
            let function = self
                .builder
                .function_state
                .current_function
                .as_mut()
                .ok_or_else(|| {
                    "[freeze:contract][canonical_binding_ssa/function_missing]".to_string()
                })?;
            cfg.seal_block(function, block)
                .map_err(|error| error.to_string())?
        };
        self.session
            .identity
            .seal_block(self.builder, &mut self.session.phis, block, &witness)
    }

    fn finish_cfg(&mut self) -> Result<(), String> {
        let cfg = std::mem::take(&mut self.session.cfg);
        let function = self
            .builder
            .function_state
            .current_function
            .as_ref()
            .ok_or_else(|| {
                "[freeze:contract][canonical_binding_ssa/function_missing]".to_string()
            })?;
        let verified_cfg = cfg.finish(function).map_err(|error| error.to_string())?;
        debug_assert_eq!(verified_cfg.blocks().len(), function.blocks.len());
        Ok(())
    }

    fn current_block(&self) -> Result<BasicBlockId, String> {
        self.builder.function_state.current_block.ok_or_else(|| {
            "[freeze:contract][canonical_binding_ssa/current_block_missing]".to_string()
        })
    }

    fn lower_literal(&mut self, literal: &LiteralValue) -> Result<ValueId, String> {
        self.builder.build_literal(literal.clone())
    }
}

fn require_representation(
    actual: TrivialRepresentationV1,
    expected: TrivialRepresentationV1,
    subject: &str,
) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "[freeze:contract][canonical_binding_ssa/representation_mismatch] subject={subject} actual={actual:?} expected={expected:?}"
        ))
    }
}

fn ensure_value_representation(
    builder: &mut MirBuilder,
    value: ValueId,
    representation: TrivialRepresentationV1,
) -> Result<(), String> {
    let expected = mir_type(representation);
    if let Some(actual) = builder.function_state.type_ctx.value_types.get(&value) {
        if actual != &MirType::Unknown && actual != &expected {
            return Err(format!(
                "[freeze:contract][canonical_binding_ssa/value_type_mismatch] value={value:?} actual={actual:?} expected={expected:?}"
            ));
        }
    }
    builder
        .function_state
        .type_ctx
        .value_types
        .insert(value, expected);
    Ok(())
}
