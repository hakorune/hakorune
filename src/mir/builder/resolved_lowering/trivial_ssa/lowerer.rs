//! Located materializer for the whole-owner trivial Binding SSA route.

use crate::ast::{ASTNode, LiteralValue};
use crate::mir::builder::emission::phi_lifecycle::PhiTxn;
use crate::mir::builder::resolved_lowering::canonical_cfg::CanonicalCfgSessionV1;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::located::{LocatedBodyV1, LocatedExprV1, LocatedStmtV1};
use crate::mir::compiler::source_view::{BodyChildRoleV1, ExprChildRoleV1};
use crate::mir::resolved_control_flow::if_control::{
    FunctionIfControlUseLedgerV1, ResolvedIfControlMaterializationV1, ResolvedIfElsePortV1,
    VerifiedResolvedFunctionIfControlV1,
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

use super::super::completion_consumption::{
    emit_canonical_explicit_return, ReadyFunctionCompletionV1,
    ResolvedFunctionCompletionConsumptionV1,
};
use super::super::semantic_stack::{ResolvedSemanticExpectedCountsV1, ResolvedSemanticStackV1};
use super::super::MirBuilder;
use super::identity::ResolvedSsaIdentityStateV2;
use super::operation::{emit_binary, mir_type};

pub(in crate::mir::builder::resolved_lowering) struct CanonicalTrivialSsaLowererV1<
    'builder,
    'source,
> {
    builder: &'builder mut MirBuilder,
    input: ResolvedFunctionLoweringInputV1<'source>,
    identity: ResolvedSsaIdentityStateV2<'source>,
    semantics: ResolvedSemanticStackV1,
    if_control: FunctionIfControlUseLedgerV1,
    profile: TrivialProfileConsumptionV1,
    completion: ResolvedFunctionCompletionConsumptionV1,
    cfg: CanonicalCfgSessionV1,
    phis: PhiTxn,
    implicit_completion: bool,
}

impl<'builder, 'source> CanonicalTrivialSsaLowererV1<'builder, 'source> {
    pub(in crate::mir::builder::resolved_lowering) fn new(
        builder: &'builder mut MirBuilder,
        input: ResolvedFunctionLoweringInputV1<'source>,
        if_control: VerifiedResolvedFunctionIfControlV1,
        completion: VerifiedFunctionCompletionV1,
        profile: VerifiedTrivialCanonicalOwnerV1,
        block_expr_count: usize,
    ) -> Result<Self, String> {
        if !builder
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
        let if_controls = if_control.row_count();
        let if_branches = if_controls + if_control.explicit_else_count();
        let semantics = ResolvedSemanticStackV1::new_with_expectations(
            input.function(),
            input.function().lowering_roots(),
            ResolvedSemanticExpectedCountsV1::new(block_expr_count, if_controls, if_branches),
        )?;
        let implicit_completion = completion.is_implicit_void();
        let completion = ResolvedFunctionCompletionConsumptionV1::new(input.owner(), completion)?;
        Ok(Self {
            builder,
            input,
            identity: ResolvedSsaIdentityStateV2::new(input.function()),
            semantics,
            if_control: if_control.into_use_ledger(),
            profile: TrivialProfileConsumptionV1::new(profile),
            completion,
            cfg: CanonicalCfgSessionV1::new(),
            phis: PhiTxn::begin("canonical_trivial_binding_ssa"),
            implicit_completion,
        })
    }

    pub(in crate::mir::builder::resolved_lowering) fn lower(
        mut self,
    ) -> Result<ReadyFunctionCompletionV1, String> {
        self.require_no_parameters()?;
        let body = self
            .input
            .source()
            .root_body()
            .map_err(|error| error.to_string())?;
        self.lower_body(&body, None)?;
        let body_end = u32::try_from(body.statements().len()).map_err(|_| {
            "[freeze:contract][canonical_binding_ssa/body_length_overflow]".to_string()
        })?;
        if self.completion_is_implicit() {
            self.profile
                .claim_terminal_implicit_no_value(body.site(), body_end)?;
            let value = crate::mir::builder::emission::constant::emit_void(self.builder)?;
            emit_canonical_explicit_return(self.builder, value)?;
        }
        self.seal_current_if_needed()?;

        self.semantics.finish()?;
        self.finish_cfg()?;
        self.profile.finish()?;
        self.if_control
            .finish()
            .map_err(|error| format!("[freeze:contract][if_control/finish] {error:?}"))?;
        self.identity.finish()?;
        self.phis
            .commit(self.builder)
            .map_err(|error| error.to_string())?;
        self.builder
            .resolved_binding_state
            .finish(self.input.owner())?;
        self.completion
            .finish(body.site(), body_end, self.semantics.function_region())
    }

    fn require_no_parameters(&self) -> Result<(), String> {
        let ASTNode::FunctionDeclaration { params, .. } = self.input.source().root() else {
            unreachable!("preflight seals one function root")
        };
        if params.is_empty() {
            Ok(())
        } else {
            Err("[freeze:contract][canonical_binding_ssa/parameter_profile_missing]".to_string())
        }
    }

    fn completion_is_implicit(&self) -> bool {
        self.implicit_completion
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
            ASTNode::Return { value, .. } => {
                let return_value = if value.is_some() {
                    let value = self
                        .input
                        .source()
                        .child_expr_from_stmt(statement, ExprChildRoleV1::ReturnValue)
                        .map_err(|error| error.to_string())?;
                    let (return_value, _) = self.lower_expr(&value, coverage.as_deref_mut())?;
                    self.profile
                        .claim_terminal_explicit_value(statement.site(), value.site())?;
                    return_value
                } else {
                    self.profile
                        .claim_terminal_explicit_no_value(statement.site())?;
                    crate::mir::builder::emission::constant::emit_void(self.builder)?
                };
                self.completion
                    .claim_explicit_return(statement.site(), self.semantics.function_region())?;
                self.identity
                    .mark_return(ResolvedExitSiteV1::Statement(statement.site().clone()))?;
                emit_canonical_explicit_return(self.builder, return_value)
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
            let binding = self.identity.publish_declaration(
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
            .identity
            .resolve_assignment_binding(target.site(), name)?;
        let value = self
            .input
            .source()
            .child_expr_from_stmt(statement, ExprChildRoleV1::AssignmentValue)
            .map_err(|error| error.to_string())?;
        let (value, actual) = self.lower_expr(&value, coverage)?;
        let block = self.current_block()?;
        self.identity
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
                let (_, value) = self.identity.variable_value(
                    self.builder,
                    &mut self.phis,
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
            .semantics
            .enter_block_expr(self.input.function(), pair)?;
        self.lower_body(&prelude, coverage.as_deref_mut())?;
        let result = self.lower_expr(&tail, coverage)?;
        self.semantics
            .close_scope_region_success(session, &mut self.identity)?;
        Ok(result)
    }

    fn lower_if(&mut self, statement: &LocatedStmtV1<'source>) -> Result<(), String> {
        let ASTNode::If { else_body, .. } = statement.node() else {
            unreachable!("If helper requires If")
        };
        let mut row = self
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
        let control = self.semantics.enter_region(
            self.input.function(),
            regions.control(),
            RegionKindV1::If,
        )?;
        let header = self.current_block()?;
        let then_block = self.builder.next_block_id();
        let explicit_else = matches!(row.else_port(), ResolvedIfElsePortV1::Explicit(_));
        let else_block = explicit_else.then(|| self.builder.next_block_id());
        let merge = self.builder.next_block_id();
        self.builder.ensure_block_exists(then_block)?;
        if let Some(block) = else_block {
            self.builder.ensure_block_exists(block)?;
        }
        self.builder.ensure_block_exists(merge)?;
        let false_target = else_block.unwrap_or(merge);
        {
            let cfg = &self.cfg;
            let function = self
                .builder
                .scope_ctx
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
        let then_scope = self.semantics.enter_scope_region(
            self.input.function(),
            regions.then_pair(),
            ScopeKindV1::IfThen,
            RegionKindV1::IfThen,
        )?;
        self.lower_body(&then_body, Some(&mut row))?;
        self.semantics
            .close_scope_region_success(then_scope, &mut self.identity)?;
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
            let else_scope = self.semantics.enter_scope_region(
                self.input.function(),
                pair,
                ScopeKindV1::IfElse,
                RegionKindV1::IfElse,
            )?;
            self.lower_body(&else_body, Some(&mut row))?;
            self.semantics
                .close_scope_region_success(else_scope, &mut self.identity)?;
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
        self.semantics.close_region(control)
    }

    fn emit_jump(&mut self, source: BasicBlockId, target: BasicBlockId) -> Result<(), String> {
        let cfg = &self.cfg;
        let function = self
            .builder
            .scope_ctx
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
            .scope_ctx
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
            let cfg = &mut self.cfg;
            let function = self
                .builder
                .scope_ctx
                .current_function
                .as_mut()
                .ok_or_else(|| {
                    "[freeze:contract][canonical_binding_ssa/function_missing]".to_string()
                })?;
            cfg.seal_block(function, block)
                .map_err(|error| error.to_string())?
        };
        self.identity
            .seal_block(self.builder, &mut self.phis, block, &witness)
    }

    fn finish_cfg(&mut self) -> Result<(), String> {
        let cfg = std::mem::take(&mut self.cfg);
        let function = self
            .builder
            .scope_ctx
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
        self.builder.current_block.ok_or_else(|| {
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
    if let Some(actual) = builder.type_ctx.value_types.get(&value) {
        if actual != &MirType::Unknown && actual != &expected {
            return Err(format!(
                "[freeze:contract][canonical_binding_ssa/value_type_mismatch] value={value:?} actual={actual:?} expected={expected:?}"
            ));
        }
    }
    builder.type_ctx.value_types.insert(value, expected);
    Ok(())
}
