//! Located-node recursive Lower for the first closed canonical family.

use crate::ast::{ASTNode, LiteralValue};
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::located::{LocatedBodyV1, LocatedExprV1, LocatedStmtV1};
use crate::mir::compiler::source_view::{BodyChildRoleV1, ExprChildRoleV1};
use crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1;
use crate::mir::resolved_region_flow::VerifiedResolvedFunctionFlowV1;
use crate::mir::resolved_semantics::{BindingKindV1, ResolvedExitSiteV1, SourceBindingSiteV1};
use crate::mir::{MirInstruction, ValueId};

use super::super::MirBuilder;
use super::branch_transaction::{ResolvedActiveEffectStackV1, ResolvedEffectBindingClassV1};
use super::completion_consumption::{
    emit_canonical_explicit_return, ReadyFunctionCompletionV1,
    ResolvedFunctionCompletionConsumptionV1,
};
use super::flow_consumption::ResolvedFlowConsumptionV1;
use super::identity::ResolvedIdentityStateV1;
use super::semantic_stack::{ResolvedSemanticExpectedCountsV1, ResolvedSemanticStackV1};

pub(super) struct CanonicalFunctionLowererV1<'builder, 'source> {
    pub(super) builder: &'builder mut MirBuilder,
    pub(super) input: ResolvedFunctionLoweringInputV1<'source>,
    pub(super) identity: ResolvedIdentityStateV1<'source>,
    pub(super) semantics: ResolvedSemanticStackV1,
    pub(super) flow: ResolvedFlowConsumptionV1,
    pub(super) effects: ResolvedActiveEffectStackV1,
    pub(super) completion: ResolvedFunctionCompletionConsumptionV1,
}

impl<'builder, 'source> CanonicalFunctionLowererV1<'builder, 'source> {
    pub(super) fn new(
        builder: &'builder mut MirBuilder,
        input: ResolvedFunctionLoweringInputV1<'source>,
        function_flow: VerifiedResolvedFunctionFlowV1,
        completion: VerifiedFunctionCompletionV1,
        block_expr_count: usize,
    ) -> Result<Self, String> {
        if !builder
            .function_state
            .resolved_binding_state
            .is_installed_for(input.owner())
        {
            return Err("[freeze:contract][canonical_lowerer/authority_not_installed]".to_string());
        }
        let flow = ResolvedFlowConsumptionV1::new(function_flow);
        if flow.owner() != input.owner() {
            return Err("[freeze:contract][canonical_lowerer/flow_owner_mismatch]".to_string());
        }
        let semantics = ResolvedSemanticStackV1::new_with_expectations(
            input.function(),
            input.function().lowering_roots(),
            ResolvedSemanticExpectedCountsV1::new(
                block_expr_count,
                flow.expected_if_control_regions(),
                flow.expected_if_branch_pairs(),
            ),
        )?;
        let completion = ResolvedFunctionCompletionConsumptionV1::new(input.owner(), completion)?;
        Ok(Self {
            builder,
            input,
            identity: ResolvedIdentityStateV1::new(input.function()),
            semantics,
            flow,
            effects: ResolvedActiveEffectStackV1::new(),
            completion,
        })
    }

    pub(super) fn lower(mut self) -> Result<ReadyFunctionCompletionV1, String> {
        self.publish_parameters()?;
        let body = self
            .input
            .source()
            .root_body()
            .map_err(|error| error.to_string())?;
        self.lower_body(&body)?;
        self.flow.finish()?;
        if !self.effects.is_empty() {
            return Err("[freeze:contract][canonical_effect/finish_not_empty]".to_string());
        }
        self.semantics.finish()?;
        self.identity.finish()?;
        self.builder
            .function_state
            .resolved_binding_state
            .finish(self.input.owner())?;
        let body_end = u32::try_from(body.statements().len()).map_err(|_| {
            "[freeze:contract][canonical_completion/body_length_overflow]".to_string()
        })?;
        self.completion
            .finish(body.site(), body_end, self.semantics.function_region())
    }

    fn publish_parameters(&mut self) -> Result<(), String> {
        let ASTNode::FunctionDeclaration { params, .. } = self.input.source().root() else {
            unreachable!("preflight seals one function root")
        };
        let entries = {
            let function = self
                .builder
                .function_state
                .current_function
                .as_ref()
                .ok_or_else(|| {
                    "[freeze:contract][canonical_lowerer/function_missing]".to_string()
                })?;
            params
                .iter()
                .enumerate()
                .map(|(index, name)| {
                    let value = function.params.get(index).copied().ok_or_else(|| {
                        format!(
                            "[freeze:contract][canonical_lowerer/parameter_value_missing] index={index}"
                        )
                    })?;
                    let ty = function.signature.params.get(index).cloned();
                    Ok((index, name.clone(), value, ty))
                })
                .collect::<Result<Vec<_>, String>>()?
        };
        for (index, name, value, ty) in entries {
            self.identity.publish_declaration(
                &SourceBindingSiteV1::Parameter {
                    index: index as u32,
                },
                BindingKindV1::Parameter {
                    index: index as u32,
                },
                &name,
                value,
            )?;
            self.builder.register_value_kind(
                value,
                hakorune_mir_core::MirValueKind::Parameter(index as u32),
            );
            if let Some(ty) = ty {
                self.builder
                    .function_state
                    .type_ctx
                    .value_types
                    .insert(value, ty.clone());
                if let Some(registry) = self.builder.comp_ctx.current_slot_registry.as_mut() {
                    registry.ensure_slot(&name, Some(ty));
                }
            }
        }
        Ok(())
    }

    pub(super) fn lower_body(&mut self, body: &LocatedBodyV1<'source>) -> Result<(), String> {
        for index in 0..body.statements().len() {
            let statement = self
                .input
                .source()
                .body_stmt(body, index)
                .map_err(|error| error.to_string())?;
            self.lower_stmt(&statement)?;
        }
        Ok(())
    }

    fn lower_stmt(&mut self, statement: &LocatedStmtV1<'source>) -> Result<(), String> {
        self.builder
            .metadata_ctx
            .set_current_span(statement.node().span());
        match statement.node() {
            ASTNode::Local {
                variables,
                initial_values,
                ..
            } => self.lower_local(statement, variables, initial_values),
            ASTNode::Outbox { variables, .. } => self.lower_outbox(statement, variables),
            ASTNode::Assignment { .. } => self.lower_assignment(statement),
            ASTNode::If { .. } => self.lower_statement_if(statement),
            ASTNode::Return { value, .. } => {
                let return_value = if value.is_some() {
                    let value = self
                        .input
                        .source()
                        .child_expr_from_stmt(statement, ExprChildRoleV1::ReturnValue)
                        .map_err(|error| error.to_string())?;
                    self.lower_expr(&value)?
                } else {
                    crate::mir::builder::emission::constant::emit_void(self.builder)?
                };
                self.completion
                    .claim_explicit_return(statement.site(), self.semantics.function_region())?;
                self.identity
                    .mark_return(ResolvedExitSiteV1::Statement(statement.site().clone()))?;
                emit_canonical_explicit_return(self.builder, return_value)?;
                Ok(())
            }
            _ => {
                let expression = self
                    .input
                    .source()
                    .statement_expression(statement)
                    .map_err(|error| error.to_string())?;
                self.lower_expr(&expression).map(|_| ())
            }
        }
    }

    fn lower_local(
        &mut self,
        statement: &LocatedStmtV1<'source>,
        variables: &[String],
        initial_values: &[Option<Box<ASTNode>>],
    ) -> Result<(), String> {
        let mut values = Vec::with_capacity(variables.len());
        for (index, initial) in initial_values.iter().enumerate() {
            let value = if initial.is_some() {
                let initial = self
                    .input
                    .source()
                    .child_expr_from_stmt(
                        statement,
                        ExprChildRoleV1::LocalInitializer(index as u32),
                    )
                    .map_err(|error| error.to_string())?;
                self.lower_expr(&initial)?
            } else {
                crate::mir::builder::emission::constant::emit_null(self.builder)?
            };
            values.push(value);
        }
        for (index, (name, source)) in variables.iter().zip(values).enumerate() {
            let value = self.builder.next_value_id();
            self.builder.emit_instruction(MirInstruction::Copy {
                dst: value,
                src: source,
            })?;
            crate::mir::builder::metadata::propagate::propagate(self.builder, source, value);
            self.identity.publish_declaration(
                &SourceBindingSiteV1::Local {
                    statement: statement.site().clone(),
                    ordinal: index as u32,
                },
                BindingKindV1::Local {
                    ordinal: index as u32,
                },
                name,
                value,
            )?;
            if let Some(registry) = self.builder.comp_ctx.current_slot_registry.as_mut() {
                let ty = self
                    .builder
                    .function_state
                    .type_ctx
                    .value_types
                    .get(&value)
                    .cloned();
                registry.ensure_slot(name, ty);
            }
        }
        Ok(())
    }

    fn lower_outbox(
        &mut self,
        statement: &LocatedStmtV1<'source>,
        variables: &[String],
    ) -> Result<(), String> {
        for (index, name) in variables.iter().enumerate() {
            let source = crate::mir::builder::emission::constant::emit_void(self.builder)?;
            let value = self.builder.next_value_id();
            self.builder.emit_instruction(MirInstruction::Copy {
                dst: value,
                src: source,
            })?;
            self.identity.publish_declaration(
                &SourceBindingSiteV1::Outbox {
                    statement: statement.site().clone(),
                    ordinal: index as u32,
                },
                BindingKindV1::Outbox {
                    ordinal: index as u32,
                },
                name,
                value,
            )?;
            if let Some(function) = self.builder.function_state.current_function.as_mut() {
                function.metadata.outbox_bindings.push(name.clone());
            }
        }
        Ok(())
    }

    pub(super) fn lower_expr(
        &mut self,
        expression: &LocatedExprV1<'source>,
    ) -> Result<ValueId, String> {
        self.builder
            .metadata_ctx
            .set_current_span(expression.node().span());
        match expression.node() {
            ASTNode::Literal { value, .. } => self.lower_literal(value),
            ASTNode::Variable { name, .. } => self.identity.variable_value(expression.site(), name),
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
                let lhs = self.lower_expr(&left)?;
                let rhs = self.lower_expr(&right)?;
                self.builder
                    .build_binary_op_from_values(operator.clone(), lhs, rhs)
            }
            ASTNode::BlockExpr { .. } => self.lower_block_expr(expression),
            _ => unreachable!("preflight seals the first-family expression grammar"),
        }
    }

    fn lower_assignment(&mut self, statement: &LocatedStmtV1<'source>) -> Result<(), String> {
        let ASTNode::Assignment { target, .. } = statement.node() else {
            unreachable!("lower_assignment is called only for Assignment")
        };
        let ASTNode::Variable { name, .. } = target.as_ref() else {
            unreachable!("preflight accepts binding assignments only")
        };
        let target = self
            .input
            .source()
            .child_expr_from_stmt(statement, ExprChildRoleV1::AssignmentTarget)
            .map_err(|error| error.to_string())?;
        let binding = self
            .identity
            .resolve_assignment_binding(target.site(), name)?;
        let class = if self.effects.is_empty() {
            ResolvedEffectBindingClassV1::Visible
        } else {
            self.effects
                .authorize_current(self.input.function(), binding)?
        };

        let value = self
            .input
            .source()
            .child_expr_from_stmt(statement, ExprChildRoleV1::AssignmentValue)
            .map_err(|error| error.to_string())?;
        let value = self.lower_expr(&value)?;
        self.flow.claim_assignment(target.site(), binding, class)?;
        self.identity
            .claim_assignment_binding(target.site(), binding)?;
        let previous = self.identity.current_value(binding)?;
        if !self.builder.is_current_block_terminated() {
            self.builder
                .emit_instruction(MirInstruction::ReleaseStrong {
                    values: vec![previous],
                })?;
        }
        if self.effects.is_empty() {
            self.identity.rebind(binding, value)?;
        } else {
            self.effects.rebind_current(
                &mut self.identity,
                self.input.function(),
                binding,
                value,
            )?;
        }
        Ok(())
    }

    fn lower_block_expr(&mut self, expression: &LocatedExprV1<'source>) -> Result<ValueId, String> {
        let pair = self
            .input
            .function()
            .block_expr_scope_region_pair(expression.owner(), expression.site())
            .map_err(|error| {
                format!("[freeze:contract][canonical_scope/exact_pair_lookup] error={error:?}")
            })?;
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
        let session = self
            .semantics
            .enter_block_expr(self.input.function(), pair)?;
        let primary = self
            .lower_body(&prelude)
            .and_then(|()| self.lower_expr(&tail));
        match primary {
            Ok(value) => {
                self.semantics
                    .close_scope_region_success(session, &mut self.identity)?;
                Ok(value)
            }
            Err(primary) => match self
                .semantics
                .close_scope_region_error(session, &mut self.identity)
            {
                Ok(()) => Err(primary),
                Err(cleanup) => Err(format!(
                    "[freeze:contract][canonical_scope/during_cleanup] primary={primary} cleanup={cleanup}"
                )),
            },
        }
    }

    fn lower_literal(&mut self, literal: &LiteralValue) -> Result<ValueId, String> {
        self.builder.build_literal(literal.clone())
    }
}
