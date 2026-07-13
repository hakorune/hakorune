//! Located-node recursive Lower for the first closed canonical family.

use crate::ast::{ASTNode, LiteralValue};
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::located::{LocatedBodyV1, LocatedExprV1, LocatedStmtV1};
use crate::mir::compiler::source_view::ExprChildRoleV1;
use crate::mir::resolved_semantics::{BindingKindV1, ResolvedExitSiteV1, SourceBindingSiteV1};
use crate::mir::{MirInstruction, MirType, ValueId};

use super::super::MirBuilder;
use super::identity::ResolvedIdentityStateV1;

pub(super) struct CanonicalFunctionLowererV1<'builder, 'source> {
    builder: &'builder mut MirBuilder,
    input: ResolvedFunctionLoweringInputV1<'source>,
    identity: ResolvedIdentityStateV1<'source>,
}

impl<'builder, 'source> CanonicalFunctionLowererV1<'builder, 'source> {
    pub(super) fn new(
        builder: &'builder mut MirBuilder,
        input: ResolvedFunctionLoweringInputV1<'source>,
    ) -> Result<Self, String> {
        if !builder
            .resolved_binding_state
            .is_installed_for(input.owner())
        {
            return Err("[freeze:contract][canonical_lowerer/authority_not_installed]".to_string());
        }
        Ok(Self {
            builder,
            input,
            identity: ResolvedIdentityStateV1::new(input.function()),
        })
    }

    pub(super) fn lower(mut self) -> Result<(), String> {
        self.publish_parameters()?;
        let body = self
            .input
            .source()
            .root_body()
            .map_err(|error| error.to_string())?;
        self.lower_body(&body)?;
        self.identity.finish()?;
        self.builder
            .resolved_binding_state
            .finish(self.input.owner())
    }

    fn publish_parameters(&mut self) -> Result<(), String> {
        let ASTNode::FunctionDeclaration { params, .. } = self.input.source().root() else {
            unreachable!("preflight seals one function root")
        };
        let entries = {
            let function = self
                .builder
                .scope_ctx
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
                self.builder.type_ctx.value_types.insert(value, ty.clone());
                if let Some(registry) = self.builder.comp_ctx.current_slot_registry.as_mut() {
                    registry.ensure_slot(&name, Some(ty));
                }
            }
        }
        Ok(())
    }

    fn lower_body(&mut self, body: &LocatedBodyV1<'source>) -> Result<(), String> {
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
            ASTNode::Assignment { target, .. } => {
                let ASTNode::Variable { name, .. } = target.as_ref() else {
                    unreachable!("preflight accepts binding assignments only")
                };
                let target = self
                    .input
                    .source()
                    .child_expr_from_stmt(statement, ExprChildRoleV1::AssignmentTarget)
                    .map_err(|error| error.to_string())?;
                let binding = self.identity.assignment_binding(target.site(), name)?;
                let value = self
                    .input
                    .source()
                    .child_expr_from_stmt(statement, ExprChildRoleV1::AssignmentValue)
                    .map_err(|error| error.to_string())?;
                let value = self.lower_expr(&value)?;
                let previous = self.identity.current_value(binding)?;
                if !self.builder.is_current_block_terminated() {
                    self.builder
                        .emit_instruction(MirInstruction::ReleaseStrong {
                            values: vec![previous],
                        })?;
                }
                self.identity.rebind(binding, value)?;
                Ok(())
            }
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
                self.identity
                    .mark_return(ResolvedExitSiteV1::Statement(statement.site().clone()))?;
                crate::mir::builder::stmts::return_stmt::emit_return_from_value(
                    self.builder,
                    return_value,
                )?;
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
                let ty = self.builder.type_ctx.value_types.get(&value).cloned();
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
            if let Some(function) = self.builder.scope_ctx.current_function.as_mut() {
                function.metadata.outbox_bindings.push(name.clone());
            }
        }
        Ok(())
    }

    fn lower_expr(&mut self, expression: &LocatedExprV1<'source>) -> Result<ValueId, String> {
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
            _ => unreachable!("preflight seals the first-family expression grammar"),
        }
    }

    fn lower_literal(&mut self, literal: &LiteralValue) -> Result<ValueId, String> {
        let value = self.builder.build_literal(literal.clone())?;
        if matches!(literal, LiteralValue::Void | LiteralValue::Null) {
            self.builder
                .type_ctx
                .value_types
                .insert(value, MirType::Void);
        }
        Ok(value)
    }
}
