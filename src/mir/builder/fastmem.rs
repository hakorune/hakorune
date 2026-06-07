//! FastMemory source-region lowering.
//!
//! This module is the narrow MIRBuilder owner for `fastmem Contract { ... }`.
//! It records side-table region metadata and emits `MemOp` instructions for
//! the v0 memory dialect. It does not choose page-map strategy, backend route,
//! product activation, or provider/replacement-front policy.

use super::{MirBuilder, MirInstruction, ValueId};
use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
use crate::mir::builder::vars::assignment_resolver::AssignmentResolverBox;
use crate::mir::function::{
    FastMemBlockNextFact, FastMemBlockNextProofKind, FastMemFreeHeadNonEmptyFact,
    FastMemFreeHeadNonEmptyProofKind, FastMemLocalFreeNonEmptyFact,
    FastMemLocalFreeNonEmptyProofKind, FastMemRegionMetadata, FastMemRegionOrigin,
    FastMemRemoteOwnerFact, FastMemRemoteOwnerProofKind, FastMemSameOwnerFact,
    FastMemSameOwnerProofKind, FastMemTableLengthFact, FastMemTableLengthPolicyKind,
    RangeIndexFact, RangeIndexFactOriginKind,
};
use crate::mir::instruction::{FastMemRegionId, MemOpAccess, MemOpKind};
use crate::mir::loop_api::LoopBuilderApi;
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
        } => lower_fastmem_if(builder, region, *condition, then_body, else_body),
        ASTNode::Return { value: None, .. } => {
            crate::mir::builder::emission::constant::emit_void(builder)
        }
        other => lower_fastmem_expr(builder, region, other),
    }
}

fn lower_fastmem_if(
    builder: &mut MirBuilder,
    region: FastMemRegionId,
    condition: ASTNode,
    then_body: Vec<ASTNode>,
    else_body: Option<Vec<ASTNode>>,
) -> Result<ValueId, String> {
    let Some(else_body) = else_body else {
        return Err("[freeze:contract][fastmem/branch_cfg_requires_else]".to_string());
    };
    let mut condition_value = lower_fastmem_branch_condition(builder, region, condition)?;
    condition_value = builder.local_cond(condition_value);
    crate::mir::builder::ssa::local::finalize_branch_cond(builder, &mut condition_value)?;

    let pre_branch_bb = builder.current_block()?;
    let pre_branch_var_map = builder.variable_ctx.variable_map.clone();
    let then_block = builder.next_block_id();
    let else_block = builder.next_block_id();
    let merge_block = builder.next_block_id();

    builder.start_new_block(then_block)?;
    builder.hint_scope_enter(0);
    lower_fastmem_branch_body(builder, region, then_body)?;
    let then_exit_block = builder.current_block()?;
    let then_reaches_merge = !builder.is_current_block_terminated();
    if then_reaches_merge {
        builder.hint_scope_leave(0);
    }

    builder.variable_ctx.variable_map = pre_branch_var_map.clone();
    builder.start_new_block(else_block)?;
    builder.hint_scope_enter(0);
    lower_fastmem_branch_body(builder, region, else_body)?;
    let else_exit_block = builder.current_block()?;
    let else_reaches_merge = !builder.is_current_block_terminated();
    if else_reaches_merge {
        builder.hint_scope_leave(0);
    }

    crate::mir::builder::emission::branch::emit_conditional_edgecfg(
        builder,
        pre_branch_bb,
        condition_value,
        then_block,
        then_exit_block,
        then_reaches_merge,
        else_block,
        else_exit_block,
        else_reaches_merge,
        merge_block,
    )?;
    builder.suppress_next_entry_pin_copy();
    builder.start_new_block(merge_block)?;
    builder.variable_ctx.variable_map = pre_branch_var_map;
    crate::mir::builder::emission::constant::emit_void(builder)
}

fn lower_fastmem_branch_condition(
    builder: &mut MirBuilder,
    region: FastMemRegionId,
    condition: ASTNode,
) -> Result<ValueId, String> {
    let ASTNode::Variable { name, .. } = condition else {
        return Err("[freeze:contract][fastmem/branch_cfg_requires_owner_eq_condition]".to_string());
    };
    let condition_value = builder.build_variable_access(name)?;
    ensure_fastmem_owner_eq_condition(builder, region, condition_value)?;
    Ok(condition_value)
}

fn lower_fastmem_branch_body(
    builder: &mut MirBuilder,
    region: FastMemRegionId,
    body: Vec<ASTNode>,
) -> Result<Option<ValueId>, String> {
    let mut last_value = None;
    for stmt in body {
        last_value = Some(lower_fastmem_stmt(builder, region, stmt)?);
    }
    Ok(last_value)
}

fn ensure_fastmem_owner_eq_condition(
    builder: &MirBuilder,
    region: FastMemRegionId,
    condition_value: ValueId,
) -> Result<(), String> {
    let function = builder
        .scope_ctx
        .current_function
        .as_ref()
        .ok_or_else(|| "[freeze:contract][fastmem/outside_function]".to_string())?;
    let is_owner_eq = function.blocks.values().any(|block| {
        block.instructions.iter().any(|instruction| {
            matches!(
                instruction,
                MirInstruction::MemOp {
                    region: actual_region,
                    kind: MemOpKind::OwnerEq,
                    dst: Some(dst),
                    ..
                } if *actual_region == region && *dst == condition_value
            )
        })
    });
    if is_owner_eq {
        Ok(())
    } else {
        Err("[freeze:contract][fastmem/branch_cfg_requires_owner_eq_condition]".to_string())
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
        "mem.localFreePush" => {
            let args = lower_fastmem_args(builder, region, arguments)?;
            if args.len() != 2 {
                return Err(format!(
                    "[freeze:contract][fastmem/arity] call=mem.localFreePush expected=2 actual={}",
                    args.len()
                ));
            }
            builder.emit_fastmem_memop(region, MemOpKind::LocalFreePush, None, args, None)?;
            crate::mir::builder::emission::constant::emit_void(builder)
        }
        "mem.freeHeadPush" => {
            let args = lower_fastmem_args(builder, region, arguments)?;
            if args.len() != 2 {
                return Err(format!(
                    "[freeze:contract][fastmem/arity] call=mem.freeHeadPush expected=2 actual={}",
                    args.len()
                ));
            }
            builder.emit_fastmem_memop(region, MemOpKind::FreeHeadPush, None, args, None)?;
            crate::mir::builder::emission::constant::emit_void(builder)
        }
        "mem.atomicRemoteHeadPush" => {
            let args = lower_fastmem_args(builder, region, arguments)?;
            if args.len() != 2 {
                return Err(format!(
                    "[freeze:contract][fastmem/arity] call=mem.atomicRemoteHeadPush expected=2 actual={}",
                    args.len()
                ));
            }
            builder.emit_fastmem_memop(
                region,
                MemOpKind::AtomicRemoteHeadPush,
                None,
                args,
                None,
            )?;
            crate::mir::builder::emission::constant::emit_void(builder)
        }
        "mem.atomicRemoteHeadDrain" => {
            let arg = single_fastmem_arg(builder, region, "mem.atomicRemoteHeadDrain", arguments)?;
            builder.emit_fastmem_value_memop(region, MemOpKind::AtomicRemoteHeadDrain, vec![arg])
        }
        "mem.drainRemoteListToLocal" => {
            let args = lower_fastmem_args(builder, region, arguments)?;
            if args.len() != 2 {
                return Err(format!(
                    "[freeze:contract][fastmem/arity] call=mem.drainRemoteListToLocal expected=2 actual={}",
                    args.len()
                ));
            }
            builder.emit_fastmem_memop(
                region,
                MemOpKind::DrainRemoteListToLocal,
                None,
                args,
                None,
            )?;
            crate::mir::builder::emission::constant::emit_void(builder)
        }
        "mem.localFreePop" => {
            let arg = single_fastmem_arg(builder, region, "mem.localFreePop", arguments)?;
            builder.emit_fastmem_value_memop(region, MemOpKind::LocalFreePop, vec![arg])
        }
        "mem.freeHeadPop" => {
            let arg = single_fastmem_arg(builder, region, "mem.freeHeadPop", arguments)?;
            builder.emit_fastmem_value_memop(region, MemOpKind::FreeHeadPop, vec![arg])
        }
        "mem.assumeSameOwner" => {
            let args = lower_fastmem_args(builder, region, arguments)?;
            if args.len() != 2 {
                return Err(format!(
                    "[freeze:contract][fastmem/arity] call=mem.assumeSameOwner expected=2 actual={}",
                    args.len()
                ));
            }
            builder.add_fastmem_same_owner_fact(region, args[0], args[1])?;
            crate::mir::builder::emission::constant::emit_void(builder)
        }
        "mem.assumeRemoteOwner" => {
            let arg = single_fastmem_arg(builder, region, "mem.assumeRemoteOwner", arguments)?;
            builder.add_fastmem_remote_owner_fact(region, arg)?;
            crate::mir::builder::emission::constant::emit_void(builder)
        }
        "mem.assumeLocalFreeBlockNext" => {
            let arg =
                single_fastmem_arg(builder, region, "mem.assumeLocalFreeBlockNext", arguments)?;
            builder.add_fastmem_block_next_fact(
                region,
                arg,
                FastMemBlockNextProofKind::SourceAssumeLocalFreeBlockNext,
            )?;
            crate::mir::builder::emission::constant::emit_void(builder)
        }
        "mem.assumeFreeHeadBlockNext" => {
            let arg =
                single_fastmem_arg(builder, region, "mem.assumeFreeHeadBlockNext", arguments)?;
            builder.add_fastmem_block_next_fact(
                region,
                arg,
                FastMemBlockNextProofKind::SourceAssumeFreeHeadBlockNext,
            )?;
            crate::mir::builder::emission::constant::emit_void(builder)
        }
        "mem.assumeRemoteFreeBlockNext" => {
            let arg =
                single_fastmem_arg(builder, region, "mem.assumeRemoteFreeBlockNext", arguments)?;
            builder.add_fastmem_block_next_fact(
                region,
                arg,
                FastMemBlockNextProofKind::SourceAssumeRemoteFreeBlockNext,
            )?;
            crate::mir::builder::emission::constant::emit_void(builder)
        }
        "mem.assumeLocalFreeNonEmpty" => {
            let arg =
                single_fastmem_arg(builder, region, "mem.assumeLocalFreeNonEmpty", arguments)?;
            builder.add_fastmem_local_free_non_empty_fact(region, arg)?;
            crate::mir::builder::emission::constant::emit_void(builder)
        }
        "mem.assumeFreeHeadNonEmpty" => {
            let arg = single_fastmem_arg(builder, region, "mem.assumeFreeHeadNonEmpty", arguments)?;
            builder.add_fastmem_free_head_non_empty_fact(region, arg)?;
            crate::mir::builder::emission::constant::emit_void(builder)
        }
        "mem.assumeTableLength" => {
            if arguments.len() != 2 {
                return Err(format!(
                    "[freeze:contract][fastmem/arity] call=mem.assumeTableLength expected=2 actual={}",
                    arguments.len()
                ));
            }
            let table_id = fastmem_table_length_table_id(&arguments[0])?;
            let resolved_length = fastmem_positive_usize_source_value(builder, &arguments[1])?;
            let args = lower_fastmem_args(builder, region, arguments)?;
            builder.add_fastmem_table_length_fact(
                region,
                table_id,
                args[0],
                args[1],
                resolved_length,
            )?;
            crate::mir::builder::emission::constant::emit_void(builder)
        }
        "mem.assumeIndexInRange" => {
            if arguments.len() != 2 {
                return Err(format!(
                    "[freeze:contract][fastmem/arity] call=mem.assumeIndexInRange expected=2 actual={}",
                    arguments.len()
                ));
            }
            let resolved_upper = fastmem_positive_usize_source_value(builder, &arguments[1])?;
            let args = lower_fastmem_args(builder, region, arguments)?;
            let upper_value =
                builder.canonical_fastmem_range_upper_value(region, resolved_upper, args[1])?;
            builder.add_fastmem_range_index_fact(args[0], upper_value)?;
            crate::mir::builder::emission::constant::emit_void(builder)
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

fn fastmem_table_length_table_id(arg: &ASTNode) -> Result<String, String> {
    match arg {
        ASTNode::Variable { name, .. } => Ok(name.clone()),
        other => Err(format!(
            "[freeze:contract][fastmem/table_length_requires_table_variable] node={}",
            other.node_type()
        )),
    }
}

fn fastmem_positive_usize_source_value(
    builder: &MirBuilder,
    arg: &ASTNode,
) -> Result<Option<u64>, String> {
    let raw = match arg {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            ..
        } => Some(*value),
        ASTNode::Variable { name, .. } => {
            let Some(value_id) = builder.variable_ctx.variable_map.get(name).copied() else {
                return Ok(None);
            };
            fastmem_const_integer_value(builder, value_id)
        }
        _ => None,
    };
    raw.map(|value| {
        u64::try_from(value)
            .ok()
            .filter(|actual| *actual > 0)
            .ok_or_else(|| {
                format!(
                    "[freeze:contract][fastmem/table_length_requires_positive_usize] value={}",
                    value
                )
            })
    })
    .transpose()
}

fn fastmem_const_integer_value(builder: &MirBuilder, value_id: ValueId) -> Option<i64> {
    let function = builder.scope_ctx.current_function.as_ref()?;
    function.blocks.values().find_map(|block| {
        block
            .instructions
            .iter()
            .find_map(|instruction| match instruction {
                MirInstruction::Const {
                    dst,
                    value: crate::mir::ConstValue::Integer(actual),
                } if *dst == value_id => Some(*actual),
                _ => None,
            })
    })
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

    fn add_fastmem_table_length_fact(
        &mut self,
        region: FastMemRegionId,
        table_id: String,
        table_value: ValueId,
        length_value: ValueId,
        resolved_length: Option<u64>,
    ) -> Result<(), String> {
        let function = self
            .scope_ctx
            .current_function
            .as_mut()
            .ok_or_else(|| "[freeze:contract][fastmem/outside_function]".to_string())?;
        let fact_id = function.metadata.fastmem_table_length_facts.len() as u32;
        function
            .metadata
            .fastmem_table_length_facts
            .push(FastMemTableLengthFact {
                fact_id,
                region,
                table_id,
                table_value,
                length_value,
                resolved_length,
                policy: FastMemTableLengthPolicyKind::ExplicitConstLen,
            });
        Ok(())
    }

    fn add_fastmem_same_owner_fact(
        &mut self,
        region: FastMemRegionId,
        page_value: ValueId,
        proof_value: ValueId,
    ) -> Result<(), String> {
        let function = self
            .scope_ctx
            .current_function
            .as_mut()
            .ok_or_else(|| "[freeze:contract][fastmem/outside_function]".to_string())?;
        let fact_id = function.metadata.fastmem_same_owner_facts.len() as u32;
        function
            .metadata
            .fastmem_same_owner_facts
            .push(FastMemSameOwnerFact {
                fact_id,
                region,
                page_value,
                proof_value,
                proof_kind: FastMemSameOwnerProofKind::SourceAssumeOwnerEq,
                remote_owner_rejected: true,
            });
        Ok(())
    }

    fn add_fastmem_remote_owner_fact(
        &mut self,
        region: FastMemRegionId,
        page_value: ValueId,
    ) -> Result<(), String> {
        let function = self
            .scope_ctx
            .current_function
            .as_mut()
            .ok_or_else(|| "[freeze:contract][fastmem/outside_function]".to_string())?;
        let fact_id = function.metadata.fastmem_remote_owner_facts.len() as u32;
        function
            .metadata
            .fastmem_remote_owner_facts
            .push(FastMemRemoteOwnerFact {
                fact_id,
                region,
                page_value,
                proof_kind: FastMemRemoteOwnerProofKind::SourceAssumeRemoteOwner,
                same_owner_rejected: true,
            });
        Ok(())
    }

    fn add_fastmem_block_next_fact(
        &mut self,
        region: FastMemRegionId,
        block_value: ValueId,
        proof_kind: FastMemBlockNextProofKind,
    ) -> Result<(), String> {
        let function = self
            .scope_ctx
            .current_function
            .as_mut()
            .ok_or_else(|| "[freeze:contract][fastmem/outside_function]".to_string())?;
        let fact_id = function.metadata.fastmem_block_next_facts.len() as u32;
        function
            .metadata
            .fastmem_block_next_facts
            .push(FastMemBlockNextFact {
                fact_id,
                region,
                block_value,
                next_field_id: "next".to_string(),
                proof_kind,
                writable: true,
                provenance_valid: true,
            });
        Ok(())
    }

    fn add_fastmem_local_free_non_empty_fact(
        &mut self,
        region: FastMemRegionId,
        page_value: ValueId,
    ) -> Result<(), String> {
        let function = self
            .scope_ctx
            .current_function
            .as_mut()
            .ok_or_else(|| "[freeze:contract][fastmem/outside_function]".to_string())?;
        let fact_id = function.metadata.fastmem_local_free_non_empty_facts.len() as u32;
        function
            .metadata
            .fastmem_local_free_non_empty_facts
            .push(FastMemLocalFreeNonEmptyFact {
                fact_id,
                region,
                page_value,
                proof_kind: FastMemLocalFreeNonEmptyProofKind::SourceAssumeLocalFreeNonEmpty,
                non_empty: true,
            });
        Ok(())
    }

    fn add_fastmem_free_head_non_empty_fact(
        &mut self,
        region: FastMemRegionId,
        page_value: ValueId,
    ) -> Result<(), String> {
        let function = self
            .scope_ctx
            .current_function
            .as_mut()
            .ok_or_else(|| "[freeze:contract][fastmem/outside_function]".to_string())?;
        let fact_id = function.metadata.fastmem_free_head_non_empty_facts.len() as u32;
        function
            .metadata
            .fastmem_free_head_non_empty_facts
            .push(FastMemFreeHeadNonEmptyFact {
                fact_id,
                region,
                page_value,
                proof_kind: FastMemFreeHeadNonEmptyProofKind::SourceAssumeFreeHeadNonEmpty,
                non_empty: true,
            });
        Ok(())
    }

    fn add_fastmem_range_index_fact(
        &mut self,
        index_value: ValueId,
        upper_exclusive_value: ValueId,
    ) -> Result<(), String> {
        let body_bb = self.current_block()?;
        let lower_value = self.build_literal(LiteralValue::Integer(0))?;
        let function = self
            .scope_ctx
            .current_function
            .as_mut()
            .ok_or_else(|| "[freeze:contract][fastmem/outside_function]".to_string())?;
        let fact_id = function.metadata.range_index_facts.len() as u32;
        function.metadata.range_index_facts.push(RangeIndexFact {
            fact_id,
            origin_kind: RangeIndexFactOriginKind::FastMemAssume,
            index_value,
            lower_value,
            upper_exclusive_value,
            body_bb,
            step: 1,
            end_exclusive: true,
            index_body_read_only: true,
            loop_carried_writes_supported: false,
        });
        Ok(())
    }

    fn canonical_fastmem_range_upper_value(
        &mut self,
        region: FastMemRegionId,
        resolved_upper: Option<u64>,
        fallback: ValueId,
    ) -> Result<ValueId, String> {
        let Some(resolved_upper) = resolved_upper else {
            return Ok(fallback);
        };
        let function = self
            .scope_ctx
            .current_function
            .as_ref()
            .ok_or_else(|| "[freeze:contract][fastmem/outside_function]".to_string())?;
        Ok(function
            .metadata
            .fastmem_table_length_facts
            .iter()
            .find(|fact| fact.region == region && fact.resolved_length == Some(resolved_upper))
            .map(|fact| fact.length_value)
            .unwrap_or(fallback))
    }
}

#[cfg(test)]
mod tests;
