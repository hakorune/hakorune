//! FastMemory `mem.*` call lowering.
//!
//! This module owns the source-call vocabulary for the memory-profile dialect.
//! It maps `mem.*` helper calls to MemOps or metadata facts, but it does not
//! choose backend lowering routes.

use super::lower_fastmem_expr;
use crate::ast::{ASTNode, LiteralValue};
use crate::mir::builder::MirBuilder;
use crate::mir::function::FastMemBlockNextProofKind;
use crate::mir::instruction::{FastMemRegionId, MemOpKind};
use crate::mir::{MirInstruction, ValueId};

pub(super) fn lower_fastmem_function_call(
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

pub(super) fn lower_fastmem_method_call(
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
