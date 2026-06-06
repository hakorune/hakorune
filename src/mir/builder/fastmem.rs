//! FastMemory source-region lowering.
//!
//! This module is the narrow MIRBuilder owner for `fastmem Contract { ... }`.
//! It records side-table region metadata and emits `MemOp` instructions for
//! the v0 memory dialect. It does not choose page-map strategy, backend route,
//! product activation, or provider/replacement-front policy.

use super::{MirBuilder, MirInstruction, ValueId};
use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
use crate::mir::builder::vars::assignment_resolver::AssignmentResolverBox;
use crate::mir::function::{FastMemRegionMetadata, FastMemRegionOrigin};
use crate::mir::instruction::{FastMemRegionId, MemOpAccess, MemOpKind};
use crate::mir::MirType;

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
        } => {
            let mut last = lower_fastmem_expr(builder, region, *condition)?;
            for child in then_body {
                last = lower_fastmem_stmt(builder, region, child)?;
            }
            if let Some(children) = else_body {
                for child in children {
                    last = lower_fastmem_stmt(builder, region, child)?;
                }
            }
            Ok(last)
        }
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
            span,
        } => {
            let slot = lower_fastmem_expr(
                builder,
                region,
                ASTNode::Index {
                    target,
                    index,
                    span,
                },
            )?;
            let value_id = lower_fastmem_expr(builder, region, value)?;
            builder.emit_fastmem_memop(
                region,
                MemOpKind::FieldStore,
                None,
                vec![slot, value_id],
                None,
            )?;
            Ok(value_id)
        }
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
            let access = fastmem_table_access(&target);
            let base = lower_fastmem_expr(builder, region, *target)?;
            let idx = lower_fastmem_expr(builder, region, *index)?;
            builder.emit_fastmem_value_memop_with_access(
                region,
                MemOpKind::TableIndex,
                vec![base, idx],
                access,
            )
        }
        ASTNode::FieldAccess { object, field, .. } => {
            let base = lower_fastmem_expr(builder, region, *object)?;
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

fn lower_fastmem_function_call(
    builder: &mut MirBuilder,
    region: FastMemRegionId,
    name: String,
    arguments: Vec<ASTNode>,
) -> Result<ValueId, String> {
    match name.as_str() {
        "mem.addr" => {
            let arg = single_fastmem_arg(builder, region, "mem.addr", arguments)?;
            builder.emit_fastmem_value_memop(region, MemOpKind::AddrOf, vec![arg])
        }
        "mem.currentAllocOwnerId" => {
            ensure_no_fastmem_args("mem.currentAllocOwnerId", &arguments)?;
            builder.emit_fastmem_value_memop(region, MemOpKind::CurrentAllocOwnerId, Vec::new())
        }
        "mem.ownerEq" => {
            let args = lower_fastmem_args(builder, region, arguments)?;
            if args.len() != 2 {
                return Err(format!(
                    "[freeze:contract][fastmem/arity] call=mem.ownerEq expected=2 actual={}",
                    args.len()
                ));
            }
            builder.emit_fastmem_value_memop(region, MemOpKind::OwnerEq, args)
        }
        _ => Err(format!(
            "[freeze:contract][fastmem/forbidden_call] call={}",
            name
        )),
    }
}

fn lower_fastmem_method_call(
    builder: &mut MirBuilder,
    region: FastMemRegionId,
    object: ASTNode,
    method: String,
    arguments: Vec<ASTNode>,
) -> Result<ValueId, String> {
    let ASTNode::Variable { name, .. } = object else {
        return Err(format!(
            "[freeze:contract][fastmem/forbidden_method_receiver] method={}",
            method
        ));
    };
    if name != "mem" {
        return Err(format!(
            "[freeze:contract][fastmem/forbidden_method_receiver] receiver={} method={}",
            name, method
        ));
    }
    lower_fastmem_function_call(builder, region, format!("mem.{}", method), arguments)
}

fn lower_fastmem_args(
    builder: &mut MirBuilder,
    region: FastMemRegionId,
    arguments: Vec<ASTNode>,
) -> Result<Vec<ValueId>, String> {
    arguments
        .into_iter()
        .map(|arg| lower_fastmem_expr(builder, region, arg))
        .collect()
}

fn single_fastmem_arg(
    builder: &mut MirBuilder,
    region: FastMemRegionId,
    call: &str,
    arguments: Vec<ASTNode>,
) -> Result<ValueId, String> {
    let mut args = lower_fastmem_args(builder, region, arguments)?;
    if args.len() != 1 {
        return Err(format!(
            "[freeze:contract][fastmem/arity] call={} expected=1 actual={}",
            call,
            args.len()
        ));
    }
    Ok(args.remove(0))
}

fn ensure_no_fastmem_args(call: &str, arguments: &[ASTNode]) -> Result<(), String> {
    if arguments.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "[freeze:contract][fastmem/arity] call={} expected=0 actual={}",
            call,
            arguments.len()
        ))
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

impl MirBuilder {
    fn register_fastmem_region(
        &mut self,
        contract: String,
        source_span: Span,
        body_statement_count: usize,
    ) -> Result<FastMemRegionId, String> {
        let function = self
            .scope_ctx
            .current_function
            .as_mut()
            .ok_or_else(|| "[freeze:contract][fastmem/outside_function]".to_string())?;
        let id = FastMemRegionId::new(function.metadata.fastmem_regions.len() as u32);
        function
            .metadata
            .fastmem_regions
            .push(FastMemRegionMetadata {
                id,
                contract,
                source_span,
                origin: FastMemRegionOrigin::SourceFastMemBlock,
                body_statement_count,
                emitted_memop_count: 0,
            });
        Ok(id)
    }

    fn note_fastmem_memop(&mut self, region: FastMemRegionId) -> Result<(), String> {
        let function = self
            .scope_ctx
            .current_function
            .as_mut()
            .ok_or_else(|| "[freeze:contract][fastmem/outside_function]".to_string())?;
        let Some(metadata) = function
            .metadata
            .fastmem_regions
            .iter_mut()
            .find(|entry| entry.id == region)
        else {
            return Err(format!(
                "[freeze:contract][fastmem/unknown_region] region={}",
                region.0
            ));
        };
        metadata.emitted_memop_count += 1;
        Ok(())
    }

    fn emit_fastmem_value_memop(
        &mut self,
        region: FastMemRegionId,
        kind: MemOpKind,
        operands: Vec<ValueId>,
    ) -> Result<ValueId, String> {
        self.emit_fastmem_value_memop_with_access(region, kind, operands, None)
    }

    fn emit_fastmem_value_memop_with_access(
        &mut self,
        region: FastMemRegionId,
        kind: MemOpKind,
        operands: Vec<ValueId>,
        access: Option<MemOpAccess>,
    ) -> Result<ValueId, String> {
        let dst = self.next_value_id();
        self.emit_fastmem_memop(region, kind, Some(dst), operands, access)?;
        self.type_ctx.value_types.insert(dst, MirType::Integer);
        Ok(dst)
    }

    fn emit_fastmem_memop(
        &mut self,
        region: FastMemRegionId,
        kind: MemOpKind,
        dst: Option<ValueId>,
        operands: Vec<ValueId>,
        access: Option<MemOpAccess>,
    ) -> Result<(), String> {
        self.note_fastmem_memop(region)?;
        self.emit_instruction(MirInstruction::MemOp {
            region,
            kind,
            dst,
            operands,
            access,
            effects: kind.effect_mask(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{BinaryOperator, LiteralValue};

    fn span() -> Span {
        Span::unknown()
    }

    fn var(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: span(),
        }
    }

    fn int_lit(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: span(),
        }
    }

    fn mem_addr(arg: ASTNode) -> ASTNode {
        ASTNode::FunctionCall {
            name: "mem.addr".to_string(),
            arguments: vec![arg],
            span: span(),
        }
    }

    fn bin(operator: BinaryOperator, left: ASTNode, right: ASTNode) -> ASTNode {
        ASTNode::BinaryOp {
            operator,
            left: Box::new(left),
            right: Box::new(right),
            span: span(),
        }
    }

    fn index(target: ASTNode, idx: ASTNode) -> ASTNode {
        ASTNode::Index {
            target: Box::new(target),
            index: Box::new(idx),
            span: span(),
        }
    }

    fn field(object: ASTNode, name: &str) -> ASTNode {
        ASTNode::FieldAccess {
            object: Box::new(object),
            field: name.to_string(),
            span: span(),
        }
    }

    fn assign(target: ASTNode, value: ASTNode) -> ASTNode {
        ASTNode::Assignment {
            target: Box::new(target),
            value: Box::new(value),
            span: span(),
        }
    }

    fn local(name: &str, value: ASTNode) -> ASTNode {
        ASTNode::Local {
            variables: vec![name.to_string()],
            initial_values: vec![Some(Box::new(value))],
            declared_type_names: Vec::new(),
            span: span(),
        }
    }

    #[test]
    fn fastmem_source_lowers_to_region_metadata_and_memops() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("fastmem_test/0".to_string());
        let body = vec![
            local("ptr", int_lit(4096)),
            ASTNode::FastMemRegion {
                contract: "PageMapV0".to_string(),
                body: vec![
                    local("addr", mem_addr(var("ptr"))),
                    local(
                        "key",
                        bin(
                            BinaryOperator::BitAnd,
                            bin(BinaryOperator::Shr, var("addr"), int_lit(12)),
                            int_lit(255),
                        ),
                    ),
                ],
                span: span(),
            },
        ];

        super::super::stmts::block_stmt::build_block(&mut builder, body).unwrap();
        let function = builder.scope_ctx.current_function.as_ref().unwrap();
        assert_eq!(function.metadata.fastmem_regions.len(), 1);
        let region = &function.metadata.fastmem_regions[0];
        assert_eq!(region.contract, "PageMapV0");
        assert_eq!(region.body_statement_count, 2);
        assert_eq!(region.emitted_memop_count, 3);

        let kinds: Vec<MemOpKind> = function
            .blocks
            .values()
            .flat_map(|block| block.instructions.iter())
            .filter_map(|inst| match inst {
                MirInstruction::MemOp { kind, .. } => Some(*kind),
                _ => None,
            })
            .collect();
        assert_eq!(
            kinds,
            vec![MemOpKind::AddrOf, MemOpKind::LogicalShr, MemOpKind::BitAnd]
        );
    }

    #[test]
    fn fastmem_layout_table_source_preserves_symbolic_access_ids() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("fastmem_access/0".to_string());
        let body = vec![
            local("page_table", int_lit(8192)),
            local("key", int_lit(3)),
            local("ptr", int_lit(12288)),
            ASTNode::FastMemRegion {
                contract: "PageMapV0".to_string(),
                body: vec![
                    local("page", index(var("page_table"), var("key"))),
                    local("owner", field(var("page"), "owner_id")),
                    assign(field(var("page"), "local_free_head"), var("ptr")),
                ],
                span: span(),
            },
        ];

        super::super::stmts::block_stmt::build_block(&mut builder, body).unwrap();
        let function = builder.scope_ctx.current_function.as_ref().unwrap();
        let access_entries: Vec<(MemOpKind, Option<String>, Option<String>)> = function
            .blocks
            .values()
            .flat_map(|block| block.instructions.iter())
            .filter_map(|inst| match inst {
                MirInstruction::MemOp { kind, access, .. } => Some((
                    *kind,
                    access.as_ref().and_then(|access| access.table_id.clone()),
                    access.as_ref().and_then(|access| access.field_id.clone()),
                )),
                _ => None,
            })
            .collect();

        assert_eq!(
            access_entries,
            vec![
                (MemOpKind::TableIndex, Some("page_table".to_string()), None,),
                (MemOpKind::FieldLoad, None, Some("owner_id".to_string())),
                (
                    MemOpKind::FieldStore,
                    None,
                    Some("local_free_head".to_string()),
                ),
            ]
        );
    }
}
