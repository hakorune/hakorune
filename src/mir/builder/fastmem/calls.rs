//! FastMemory `mem.*` call lowering.
//!
//! This module owns the source-call vocabulary for the memory-profile dialect.
//! It maps `mem.*` helper calls to MemOps or metadata facts, but it does not
//! choose backend lowering routes.

use crate::ast::{ASTNode, LiteralValue};
use crate::mir::builder::MirBuilder;
use crate::mir::function::FastMemBlockNextProofKind;
use crate::mir::instruction::{FastMemRegionId, MemOpKind};
use crate::mir::{MirInstruction, ValueId};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FastMemIntrinsic {
    Addr,
    CurrentAllocOwnerId,
    OwnerEq,
    LocalFreePush,
    FreeHeadPush,
    AtomicRemoteHeadPush,
    AtomicRemoteHeadDrain,
    DrainRemoteListToLocal,
    LocalFreePop,
    FreeHeadPop,
    AssumeSameOwner,
    AssumeRemoteOwner,
    AssumeLocalFreeBlockNext,
    AssumeFreeHeadBlockNext,
    AssumeRemoteFreeBlockNext,
    AssumeLocalFreeNonEmpty,
    AssumeFreeHeadNonEmpty,
    AssumeTableLength,
    AssumeIndexInRange,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FastMemIntrinsicArity {
    Zero,
    One,
    Two,
}

impl FastMemIntrinsicArity {
    fn expected(self) -> usize {
        match self {
            FastMemIntrinsicArity::Zero => 0,
            FastMemIntrinsicArity::One => 1,
            FastMemIntrinsicArity::Two => 2,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct FastMemIntrinsicSpec {
    intrinsic: FastMemIntrinsic,
    call_name: &'static str,
    arity: FastMemIntrinsicArity,
}

impl FastMemIntrinsicSpec {
    fn new(
        intrinsic: FastMemIntrinsic,
        call_name: &'static str,
        arity: FastMemIntrinsicArity,
    ) -> Self {
        Self {
            intrinsic,
            call_name,
            arity,
        }
    }
}

pub(crate) fn lower_fastmem_function_call(
    builder: &mut MirBuilder,
    region: FastMemRegionId,
    name: String,
    arguments: Vec<ASTNode>,
) -> Result<ValueId, String> {
    let Some(spec) = lookup_fastmem_intrinsic(&name) else {
        return Err(format!(
            "[freeze:contract][fastmem/forbidden_call] call={}",
            name
        ));
    };
    lower_fastmem_intrinsic(builder, region, spec, arguments)
}

fn lower_fastmem_intrinsic(
    builder: &mut MirBuilder,
    region: FastMemRegionId,
    spec: FastMemIntrinsicSpec,
    arguments: Vec<ASTNode>,
) -> Result<ValueId, String> {
    let call = spec.call_name;
    let expected = spec.arity.expected();
    if arguments.len() != expected {
        return Err(format!(
            "[freeze:contract][fastmem/arity] call={} expected={} actual={}",
            call,
            expected,
            arguments.len()
        ));
    }

    match spec.intrinsic {
        FastMemIntrinsic::Addr => {
            let arg = single_fastmem_arg(builder, region, call, arguments)?;
            builder.emit_fastmem_value_memop(region, MemOpKind::AddrOf, vec![arg])
        }
        FastMemIntrinsic::CurrentAllocOwnerId => {
            ensure_no_fastmem_args(call, &arguments)?;
            builder.emit_fastmem_value_memop(region, MemOpKind::CurrentAllocOwnerId, Vec::new())
        }
        FastMemIntrinsic::OwnerEq => {
            let args = lower_fastmem_args(builder, region, arguments)?;
            builder.emit_fastmem_value_memop(region, MemOpKind::OwnerEq, args)
        }
        FastMemIntrinsic::LocalFreePush => {
            let args = lower_fastmem_args(builder, region, arguments)?;
            builder.emit_fastmem_memop(region, MemOpKind::LocalFreePush, None, args, None)?;
            crate::mir::builder::emission::constant::emit_void(builder)
        }
        FastMemIntrinsic::FreeHeadPush => {
            let args = lower_fastmem_args(builder, region, arguments)?;
            builder.emit_fastmem_memop(region, MemOpKind::FreeHeadPush, None, args, None)?;
            crate::mir::builder::emission::constant::emit_void(builder)
        }
        FastMemIntrinsic::AtomicRemoteHeadPush => {
            let args = lower_fastmem_args(builder, region, arguments)?;
            builder.emit_fastmem_memop(
                region,
                MemOpKind::AtomicRemoteHeadPush,
                None,
                args,
                None,
            )?;
            crate::mir::builder::emission::constant::emit_void(builder)
        }
        FastMemIntrinsic::AtomicRemoteHeadDrain => {
            let arg = single_fastmem_arg(builder, region, call, arguments)?;
            builder.emit_fastmem_value_memop(region, MemOpKind::AtomicRemoteHeadDrain, vec![arg])
        }
        FastMemIntrinsic::DrainRemoteListToLocal => {
            let args = lower_fastmem_args(builder, region, arguments)?;
            builder.emit_fastmem_memop(
                region,
                MemOpKind::DrainRemoteListToLocal,
                None,
                args,
                None,
            )?;
            crate::mir::builder::emission::constant::emit_void(builder)
        }
        FastMemIntrinsic::LocalFreePop => {
            let arg = single_fastmem_arg(builder, region, call, arguments)?;
            builder.emit_fastmem_value_memop(region, MemOpKind::LocalFreePop, vec![arg])
        }
        FastMemIntrinsic::FreeHeadPop => {
            let arg = single_fastmem_arg(builder, region, call, arguments)?;
            builder.emit_fastmem_value_memop(region, MemOpKind::FreeHeadPop, vec![arg])
        }
        FastMemIntrinsic::AssumeSameOwner => {
            let args = lower_fastmem_args(builder, region, arguments)?;
            builder.add_fastmem_same_owner_fact(region, args[0], args[1])?;
            crate::mir::builder::emission::constant::emit_void(builder)
        }
        FastMemIntrinsic::AssumeRemoteOwner => {
            let arg = single_fastmem_arg(builder, region, call, arguments)?;
            builder.add_fastmem_remote_owner_fact(region, arg)?;
            crate::mir::builder::emission::constant::emit_void(builder)
        }
        FastMemIntrinsic::AssumeLocalFreeBlockNext => {
            let arg = single_fastmem_arg(builder, region, call, arguments)?;
            builder.add_fastmem_block_next_fact(
                region,
                arg,
                FastMemBlockNextProofKind::SourceAssumeLocalFreeBlockNext,
            )?;
            crate::mir::builder::emission::constant::emit_void(builder)
        }
        FastMemIntrinsic::AssumeFreeHeadBlockNext => {
            let arg = single_fastmem_arg(builder, region, call, arguments)?;
            builder.add_fastmem_block_next_fact(
                region,
                arg,
                FastMemBlockNextProofKind::SourceAssumeFreeHeadBlockNext,
            )?;
            crate::mir::builder::emission::constant::emit_void(builder)
        }
        FastMemIntrinsic::AssumeRemoteFreeBlockNext => {
            let arg = single_fastmem_arg(builder, region, call, arguments)?;
            builder.add_fastmem_block_next_fact(
                region,
                arg,
                FastMemBlockNextProofKind::SourceAssumeRemoteFreeBlockNext,
            )?;
            crate::mir::builder::emission::constant::emit_void(builder)
        }
        FastMemIntrinsic::AssumeLocalFreeNonEmpty => {
            let arg = single_fastmem_arg(builder, region, call, arguments)?;
            builder.add_fastmem_local_free_non_empty_fact(region, arg)?;
            crate::mir::builder::emission::constant::emit_void(builder)
        }
        FastMemIntrinsic::AssumeFreeHeadNonEmpty => {
            let arg = single_fastmem_arg(builder, region, call, arguments)?;
            builder.add_fastmem_free_head_non_empty_fact(region, arg)?;
            crate::mir::builder::emission::constant::emit_void(builder)
        }
        FastMemIntrinsic::AssumeTableLength => {
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
        FastMemIntrinsic::AssumeIndexInRange => {
            let resolved_upper = fastmem_positive_usize_source_value(builder, &arguments[1])?;
            let args = lower_fastmem_args(builder, region, arguments)?;
            let upper_value =
                builder.canonical_fastmem_range_upper_value(region, resolved_upper, args[1])?;
            builder.add_fastmem_range_index_fact(args[0], upper_value)?;
            crate::mir::builder::emission::constant::emit_void(builder)
        }
    }
}

pub(crate) fn lower_fastmem_method_call(
    builder: &mut MirBuilder,
    _region: FastMemRegionId,
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
    lower_fastmem_function_call(builder, _region, format!("mem.{}", method), arguments)
}

fn lower_fastmem_args(
    builder: &mut MirBuilder,
    _region: FastMemRegionId,
    arguments: Vec<ASTNode>,
) -> Result<Vec<ValueId>, String> {
    arguments
        .into_iter()
        .map(|arg| builder.build_expression(arg))
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

fn lookup_fastmem_intrinsic(name: &str) -> Option<FastMemIntrinsicSpec> {
    match name {
        "mem.addr" => Some(FastMemIntrinsicSpec::new(
            FastMemIntrinsic::Addr,
            "mem.addr",
            FastMemIntrinsicArity::One,
        )),
        "mem.currentAllocOwnerId" => Some(FastMemIntrinsicSpec::new(
            FastMemIntrinsic::CurrentAllocOwnerId,
            "mem.currentAllocOwnerId",
            FastMemIntrinsicArity::Zero,
        )),
        "mem.ownerEq" => Some(FastMemIntrinsicSpec::new(
            FastMemIntrinsic::OwnerEq,
            "mem.ownerEq",
            FastMemIntrinsicArity::Two,
        )),
        "mem.localFreePush" => Some(FastMemIntrinsicSpec::new(
            FastMemIntrinsic::LocalFreePush,
            "mem.localFreePush",
            FastMemIntrinsicArity::Two,
        )),
        "mem.freeHeadPush" => Some(FastMemIntrinsicSpec::new(
            FastMemIntrinsic::FreeHeadPush,
            "mem.freeHeadPush",
            FastMemIntrinsicArity::Two,
        )),
        "mem.atomicRemoteHeadPush" => Some(FastMemIntrinsicSpec::new(
            FastMemIntrinsic::AtomicRemoteHeadPush,
            "mem.atomicRemoteHeadPush",
            FastMemIntrinsicArity::Two,
        )),
        "mem.atomicRemoteHeadDrain" => Some(FastMemIntrinsicSpec::new(
            FastMemIntrinsic::AtomicRemoteHeadDrain,
            "mem.atomicRemoteHeadDrain",
            FastMemIntrinsicArity::One,
        )),
        "mem.drainRemoteListToLocal" => Some(FastMemIntrinsicSpec::new(
            FastMemIntrinsic::DrainRemoteListToLocal,
            "mem.drainRemoteListToLocal",
            FastMemIntrinsicArity::Two,
        )),
        "mem.localFreePop" => Some(FastMemIntrinsicSpec::new(
            FastMemIntrinsic::LocalFreePop,
            "mem.localFreePop",
            FastMemIntrinsicArity::One,
        )),
        "mem.freeHeadPop" => Some(FastMemIntrinsicSpec::new(
            FastMemIntrinsic::FreeHeadPop,
            "mem.freeHeadPop",
            FastMemIntrinsicArity::One,
        )),
        "mem.assumeSameOwner" => Some(FastMemIntrinsicSpec::new(
            FastMemIntrinsic::AssumeSameOwner,
            "mem.assumeSameOwner",
            FastMemIntrinsicArity::Two,
        )),
        "mem.assumeRemoteOwner" => Some(FastMemIntrinsicSpec::new(
            FastMemIntrinsic::AssumeRemoteOwner,
            "mem.assumeRemoteOwner",
            FastMemIntrinsicArity::One,
        )),
        "mem.assumeLocalFreeBlockNext" => Some(FastMemIntrinsicSpec::new(
            FastMemIntrinsic::AssumeLocalFreeBlockNext,
            "mem.assumeLocalFreeBlockNext",
            FastMemIntrinsicArity::One,
        )),
        "mem.assumeFreeHeadBlockNext" => Some(FastMemIntrinsicSpec::new(
            FastMemIntrinsic::AssumeFreeHeadBlockNext,
            "mem.assumeFreeHeadBlockNext",
            FastMemIntrinsicArity::One,
        )),
        "mem.assumeRemoteFreeBlockNext" => Some(FastMemIntrinsicSpec::new(
            FastMemIntrinsic::AssumeRemoteFreeBlockNext,
            "mem.assumeRemoteFreeBlockNext",
            FastMemIntrinsicArity::One,
        )),
        "mem.assumeLocalFreeNonEmpty" => Some(FastMemIntrinsicSpec::new(
            FastMemIntrinsic::AssumeLocalFreeNonEmpty,
            "mem.assumeLocalFreeNonEmpty",
            FastMemIntrinsicArity::One,
        )),
        "mem.assumeFreeHeadNonEmpty" => Some(FastMemIntrinsicSpec::new(
            FastMemIntrinsic::AssumeFreeHeadNonEmpty,
            "mem.assumeFreeHeadNonEmpty",
            FastMemIntrinsicArity::One,
        )),
        "mem.assumeTableLength" => Some(FastMemIntrinsicSpec::new(
            FastMemIntrinsic::AssumeTableLength,
            "mem.assumeTableLength",
            FastMemIntrinsicArity::Two,
        )),
        "mem.assumeIndexInRange" => Some(FastMemIntrinsicSpec::new(
            FastMemIntrinsic::AssumeIndexInRange,
            "mem.assumeIndexInRange",
            FastMemIntrinsicArity::Two,
        )),
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
