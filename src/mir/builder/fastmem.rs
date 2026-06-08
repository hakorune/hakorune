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
use crate::mir::builder::vars::assignment_resolver::AssignmentResolverBox;
use crate::mir::instruction::{FastMemRegionId, MemOpAccess, MemOpKind};

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
        ASTNode::Print { expression, .. }
        | ASTNode::Return {
            value: Some(expression),
            ..
        } => lower_fastmem_expr(builder, region, *expression),
        ASTNode::If {
            condition,
            then_body,
            else_body,
            ..
        } => lower_fastmem_if(builder, region, *condition, then_body, else_body),
        ASTNode::Return { value: None, .. } => {
            crate::mir::builder::emission::constant::emit_void(builder)
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
    let mut last = None;
    for (index, name) in variables.iter().enumerate() {
        let Some(Some(init)) = initial_values.get(index) else {
            return Err(format!(
                "[freeze:contract][fastmem/local_missing_initializer] name={}",
                name
            ));
        };
        let value = lower_fastmem_expr(builder, region, *init.clone())?;
        builder.declare_local_in_current_scope(name, value)?;
        last = Some(value);
    }
    last.ok_or_else(|| "[freeze:contract][fastmem/local_empty]".to_string())
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
            AssignmentResolverBox::ensure_declared(builder, &name)?;
            builder.variable_ctx.variable_map.insert(name, value_id);
            Ok(value_id)
        }
        ASTNode::FieldAccess { object, field, .. } => {
            let base = lower_fastmem_expr(builder, region, *object)?;
            builder.add_fastmem_field_access_site(
                region,
                base,
                field.clone(),
                None,
                "store",
                "verified_layout_field",
                "forbidden",
            )?;
            let value_id = lower_fastmem_expr(builder, region, value)?;
            builder.emit_fastmem_memop(
                region,
                MemOpKind::FieldStore,
                None,
                vec![base, value_id],
                Some(MemOpAccess::field(field)),
            )?;
            Ok(value_id)
        }
        ASTNode::Index {
            target,
            index,
            span: _span,
        } => lower_fastmem_index_access(builder, region, *target, *index, "store", Some(value)),
        other => Err(format!(
            "[freeze:contract][fastmem/unsupported_assignment_target] node={}",
            other.node_type()
        )),
    }
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
        } => {
            let lhs = lower_fastmem_expr(builder, region, *left)?;
            let rhs = lower_fastmem_expr(builder, region, *right)?;
            let kind = memop_kind_for_binary_operator(operator)?;
            builder.emit_fastmem_value_memop(region, kind, vec![lhs, rhs])
        }
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
            lower_fastmem_index_access(builder, region, *target, *index, "load", None)
        }
        ASTNode::FieldAccess { object, field, .. } => {
            let base = lower_fastmem_expr(builder, region, *object)?;
            builder.add_fastmem_field_access_site(
                region,
                base,
                field.clone(),
                None,
                "load",
                "verified_layout_field",
                "forbidden",
            )?;
            builder.emit_fastmem_value_memop_with_access(
                region,
                MemOpKind::FieldLoad,
                vec![base],
                Some(MemOpAccess::field(field)),
            )
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

fn fastmem_table_access(target: &ASTNode) -> Option<MemOpAccess> {
    match target {
        ASTNode::Variable { name, .. } => Some(MemOpAccess::table(name.clone())),
        _ => None,
    }
}

fn lower_fastmem_index_access(
    builder: &mut MirBuilder,
    region: FastMemRegionId,
    target: ASTNode,
    index: ASTNode,
    access_kind: &'static str,
    store_value: Option<ASTNode>,
) -> Result<ValueId, String> {
    let base = lower_fastmem_expr(builder, region, target.clone())?;
    let idx = lower_fastmem_expr(builder, region, index)?;
    let access = fastmem_table_access(&target);
    builder.add_fastmem_index_access_site(
        region,
        base,
        idx,
        access.as_ref().and_then(|access| access.table_id.clone()),
        None,
        access_kind,
        "verified_table_index",
        "forbidden",
    )?;
    let slot = builder.emit_fastmem_value_memop_with_access(
        region,
        MemOpKind::TableIndex,
        vec![base, idx],
        access,
    )?;
    if let Some(value) = store_value {
        let value_id = lower_fastmem_expr(builder, region, value)?;
        builder.emit_fastmem_memop(
            region,
            MemOpKind::FieldStore,
            None,
            vec![slot, value_id],
            None,
        )?;
        Ok(value_id)
    } else {
        Ok(slot)
    }
}

#[cfg(test)]
mod tests;
