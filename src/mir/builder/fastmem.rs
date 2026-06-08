//! FastMemory source-region lowering.
//!
//! This module is the narrow MIRBuilder owner for `fastmem Contract { ... }`.
//! It records side-table region metadata and emits `MemOp` instructions for
//! the v0 memory dialect. It does not choose page-map strategy, backend route,
//! product activation, or provider/replacement-front policy.

mod branch;
mod calls;
mod ops;

use super::{MirBuilder, ValueId};
use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
use crate::mir::instruction::{FastMemRegionId, MemOpKind};

use branch::lower_fastmem_if;
use calls::{lower_fastmem_function_call, lower_fastmem_method_call};

pub(in crate::mir::builder) fn build_fastmem_region(
    builder: &mut MirBuilder,
    contract: String,
    body: Vec<ASTNode>,
    span: Span,
) -> Result<ValueId, String> {
    let region = builder.register_fastmem_region(contract, span, body.len())?;
    let mut last_value = None;
    for stmt in body {
        last_value = Some(lower_fastmem_stmt(builder, region, stmt)?);
    }
    match last_value {
        Some(value) => Ok(value),
        None => crate::mir::builder::emission::constant::emit_void(builder),
    }
}

fn lower_fastmem_stmt(
    builder: &mut MirBuilder,
    region: FastMemRegionId,
    stmt: ASTNode,
) -> Result<ValueId, String> {
    builder.metadata_ctx.set_current_span(stmt.span());
    match stmt {
        ASTNode::Local {
            variables,
            initial_values,
            ..
        } => lower_fastmem_local(builder, region, variables, initial_values),
        ASTNode::Assignment { target, value, .. } => {
            lower_fastmem_assignment(builder, region, *target, *value)
        }
        ASTNode::Print { expression, .. } => {
            let value = lower_fastmem_expr(builder, region, *expression)?;
            crate::mir::builder::stmts::print_stmt::build_print_from_value(builder, value)
        }
        ASTNode::Return {
            value: Some(expression),
            ..
        } => lower_fastmem_return(builder, region, *expression),
        ASTNode::If {
            condition,
            then_body,
            else_body,
            ..
        } => lower_fastmem_if(builder, region, *condition, then_body, else_body),
        ASTNode::Return { value: None, .. } => {
            let void_value = crate::mir::builder::emission::constant::emit_void(builder)?;
            crate::mir::builder::stmts::return_stmt::emit_return_from_value(builder, void_value)
        }
        other => lower_fastmem_expr(builder, region, other),
    }
}

fn lower_fastmem_local(
    builder: &mut MirBuilder,
    region: FastMemRegionId,
    variables: Vec<String>,
    initial_values: Vec<Option<Box<ASTNode>>>,
) -> Result<ValueId, String> {
    let mut values = Vec::with_capacity(variables.len());
    for (index, _name) in variables.iter().enumerate() {
        let value = if let Some(Some(init)) = initial_values.get(index) {
            lower_fastmem_expr(builder, region, *init.clone())?
        } else {
            crate::mir::builder::emission::constant::emit_null(builder)?
        };
        values.push(value);
    }
    crate::mir::builder::stmts::variable_stmt::build_local_statement_from_values(
        builder, variables, values,
    )
}

fn lower_fastmem_assignment(
    builder: &mut MirBuilder,
    region: FastMemRegionId,
    target: ASTNode,
    value: ASTNode,
) -> Result<ValueId, String> {
    match target {
        ASTNode::Variable { name, .. } => {
            let value_id = lower_fastmem_expr(builder, region, value)?;
            builder.build_assignment_from_value(name, value_id)
        }
        ASTNode::FieldAccess { object, field, .. } => {
            let base = lower_fastmem_expr(builder, region, *object)?;
            let base = builder.local_field_base(base);
            let value_id = lower_fastmem_expr(builder, region, value)?;
            builder.build_field_assignment_from_value_id(Some(region), base, field, value_id)?;
            Ok(value_id)
        }
        ASTNode::Index {
            target,
            index,
            span: _span,
        } => {
            let target_label = fastmem_index_table_label(&target);
            let base = lower_fastmem_expr(builder, region, *target)?;
            let idx = lower_fastmem_expr(builder, region, *index)?;
            let value_id = lower_fastmem_expr(builder, region, value)?;
            builder.build_index_access_from_values(
                Some(region),
                base,
                idx,
                target_label,
                "store",
                Some(value_id),
            )
        }
        other => Err(format!(
            "[freeze:contract][fastmem/unsupported_assignment_target] node={}",
            other.node_type()
        )),
    }
}

fn lower_fastmem_return(
    builder: &mut MirBuilder,
    region: FastMemRegionId,
    expression: ASTNode,
) -> Result<ValueId, String> {
    if let Some(return_value) =
        crate::mir::builder::stmts::return_stmt::try_apply_match_return_optimization(
            builder,
            Some(&expression),
            true,
        )?
    {
        return Ok(return_value);
    }

    let value = lower_fastmem_expr(builder, region, expression)?;
    crate::mir::builder::stmts::return_stmt::emit_return_from_value(builder, value)
}

fn lower_fastmem_expr(
    builder: &mut MirBuilder,
    region: FastMemRegionId,
    expr: ASTNode,
) -> Result<ValueId, String> {
    builder.metadata_ctx.set_current_span(expr.span());
    match expr {
        ASTNode::Literal { value, .. } => lower_fastmem_literal(builder, value),
        ASTNode::Variable { name, .. } => builder.build_variable_access(name),
        ASTNode::Me { .. } => super::stmts::variable_stmt::build_me_expression(builder),
        ASTNode::BinaryOp {
            operator,
            left,
            right,
            ..
        } => lower_fastmem_numeric_binary_op(builder, region, operator, *left, *right),
        ASTNode::FunctionCall {
            name, arguments, ..
        } => lower_fastmem_function_call(builder, region, name, arguments),
        ASTNode::MethodCall {
            object,
            method,
            arguments,
            ..
        } => lower_fastmem_method_call(builder, region, *object, method, arguments),
        ASTNode::Index { target, index, .. } => {
            let target_label = fastmem_index_table_label(&target);
            let base = lower_fastmem_expr(builder, region, *target)?;
            let idx = lower_fastmem_expr(builder, region, *index)?;
            builder.build_index_access_from_values(
                Some(region),
                base,
                idx,
                target_label,
                "load",
                None,
            )
        }
        ASTNode::FieldAccess { object, field, .. } => {
            let base = lower_fastmem_expr(builder, region, *object)?;
            let base = builder.local_field_base(base);
            builder.add_fastmem_field_access_site(
                region,
                base,
                field.clone(),
                None,
                "load",
                "verified_layout_field",
                "forbidden",
            )?;
            builder.build_field_access_from_value(base, field)
        }
        other => Err(format!(
            "[freeze:contract][fastmem/unsupported_expr] node={}",
            other.node_type()
        )),
    }
}

fn lower_fastmem_literal(builder: &mut MirBuilder, value: LiteralValue) -> Result<ValueId, String> {
    match value {
        LiteralValue::Integer(_)
        | LiteralValue::TypedInteger { .. }
        | LiteralValue::Bool(_)
        | LiteralValue::Null
        | LiteralValue::Void => builder.build_literal(value),
        _ => Err("[freeze:contract][fastmem/unsupported_literal]".to_string()),
    }
}

fn memop_kind_for_binary_operator(operator: BinaryOperator) -> Result<MemOpKind, String> {
    match operator {
        BinaryOperator::Shr => Ok(MemOpKind::LogicalShr),
        BinaryOperator::BitAnd => Ok(MemOpKind::BitAnd),
        BinaryOperator::Add => Ok(MemOpKind::Add),
        BinaryOperator::Subtract => Ok(MemOpKind::Sub),
        _ => Err(format!(
            "[freeze:contract][fastmem/unsupported_binary_op] op={}",
            operator
        )),
    }
}

fn lower_fastmem_numeric_binary_op(
    builder: &mut MirBuilder,
    region: FastMemRegionId,
    operator: BinaryOperator,
    left: ASTNode,
    right: ASTNode,
) -> Result<ValueId, String> {
    let lhs = lower_fastmem_expr(builder, region, left)?;
    let rhs = lower_fastmem_expr(builder, region, right)?;
    let kind = memop_kind_for_binary_operator(operator)?;
    builder.emit_fastmem_value_memop(region, kind, vec![lhs, rhs])
}

fn fastmem_index_table_label(target: &ASTNode) -> Option<String> {
    match target {
        ASTNode::Variable { name, .. } => Some(name.clone()),
        _ => None,
    }
}

#[cfg(test)]
mod tests;
