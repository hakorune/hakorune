//! FastMemory `mem.*` call lowering.
//!
//! This module owns the source-call vocabulary for the memory-profile dialect.
//! It maps `mem.*` helper calls to MemOps or metadata facts, but it does not
//! choose backend lowering routes.
//! Function-call and MethodCall facades share one intrinsic/preflight core;
//! each facade supplies only its existing indexed expression descent.

use crate::ast::{ASTNode, LiteralValue};
use crate::mir::builder::recursive_child_lowering::{
    drive_legacy_expression_v1, RawAstChildLoweringPortV1,
};
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

pub(in crate::mir::builder) struct PreparedFastMemIntrinsicV1 {
    route: PreparedFastMemIntrinsicRouteV1,
}

enum PreparedFastMemIntrinsicRouteV1 {
    Selected(FastMemIntrinsicSpec),
    Forbidden(Box<str>),
    ArityMismatch {
        call: &'static str,
        expected: usize,
        actual: usize,
    },
}

impl PreparedFastMemIntrinsicV1 {
    pub(in crate::mir::builder) fn prepare(name: &str, actual: usize) -> Self {
        let route = match lookup_fastmem_intrinsic(name) {
            None => PreparedFastMemIntrinsicRouteV1::Forbidden(name.into()),
            Some(spec) if actual != spec.arity.expected() => {
                PreparedFastMemIntrinsicRouteV1::ArityMismatch {
                    call: spec.call_name,
                    expected: spec.arity.expected(),
                    actual,
                }
            }
            Some(spec) => PreparedFastMemIntrinsicRouteV1::Selected(spec),
        };
        Self { route }
    }

    fn into_spec(self) -> Result<FastMemIntrinsicSpec, String> {
        match self.route {
            PreparedFastMemIntrinsicRouteV1::Selected(spec) => Ok(spec),
            PreparedFastMemIntrinsicRouteV1::Forbidden(call) => Err(format!(
                "[freeze:contract][fastmem/forbidden_call] call={call}"
            )),
            PreparedFastMemIntrinsicRouteV1::ArityMismatch {
                call,
                expected,
                actual,
            } => Err(format!(
                "[freeze:contract][fastmem/arity] call={call} expected={expected} actual={actual}"
            )),
        }
    }
}

pub(in crate::mir::builder) fn lower_prepared_fastmem_function_call_with_port_v1<Port>(
    builder: &mut MirBuilder,
    region: FastMemRegionId,
    intrinsic: PreparedFastMemIntrinsicV1,
    arguments: Vec<ASTNode>,
    port: &mut Port,
) -> Result<ValueId, String>
where
    Port: RawAstChildLoweringPortV1,
{
    let spec = intrinsic.into_spec()?;
    lower_fastmem_intrinsic_with(builder, region, spec, &arguments, |builder, index| {
        drive_legacy_expression_v1(builder, port, arguments[index].clone())
    })
}

fn lower_fastmem_intrinsic_with<Lower>(
    builder: &mut MirBuilder,
    region: FastMemRegionId,
    spec: FastMemIntrinsicSpec,
    arguments: &[ASTNode],
    mut lower_argument: Lower,
) -> Result<ValueId, String>
where
    Lower: FnMut(&mut MirBuilder, usize) -> Result<ValueId, String>,
{
    match spec.intrinsic {
        FastMemIntrinsic::Addr => {
            let arg = lower_argument(builder, 0)?;
            builder.emit_fastmem_value_memop(region, MemOpKind::AddrOf, vec![arg])
        }
        FastMemIntrinsic::CurrentAllocOwnerId => {
            builder.emit_fastmem_value_memop(region, MemOpKind::CurrentAllocOwnerId, Vec::new())
        }
        FastMemIntrinsic::OwnerEq => {
            let args = lower_fastmem_args_with(builder, arguments.len(), &mut lower_argument)?;
            builder.emit_fastmem_value_memop(region, MemOpKind::OwnerEq, args)
        }
        FastMemIntrinsic::LocalFreePush => {
            let args = lower_fastmem_args_with(builder, arguments.len(), &mut lower_argument)?;
            builder.emit_fastmem_memop(region, MemOpKind::LocalFreePush, None, args, None)?;
            crate::mir::builder::emission::constant::emit_void(builder)
        }
        FastMemIntrinsic::FreeHeadPush => {
            let args = lower_fastmem_args_with(builder, arguments.len(), &mut lower_argument)?;
            builder.emit_fastmem_memop(region, MemOpKind::FreeHeadPush, None, args, None)?;
            crate::mir::builder::emission::constant::emit_void(builder)
        }
        FastMemIntrinsic::AtomicRemoteHeadPush => {
            let args = lower_fastmem_args_with(builder, arguments.len(), &mut lower_argument)?;
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
            let arg = lower_argument(builder, 0)?;
            builder.emit_fastmem_value_memop(region, MemOpKind::AtomicRemoteHeadDrain, vec![arg])
        }
        FastMemIntrinsic::DrainRemoteListToLocal => {
            let args = lower_fastmem_args_with(builder, arguments.len(), &mut lower_argument)?;
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
            let arg = lower_argument(builder, 0)?;
            builder.emit_fastmem_value_memop(region, MemOpKind::LocalFreePop, vec![arg])
        }
        FastMemIntrinsic::FreeHeadPop => {
            let arg = lower_argument(builder, 0)?;
            builder.emit_fastmem_value_memop(region, MemOpKind::FreeHeadPop, vec![arg])
        }
        FastMemIntrinsic::AssumeSameOwner => {
            let args = lower_fastmem_args_with(builder, arguments.len(), &mut lower_argument)?;
            builder.add_fastmem_same_owner_fact(region, args[0], args[1])?;
            crate::mir::builder::emission::constant::emit_void(builder)
        }
        FastMemIntrinsic::AssumeRemoteOwner => {
            let arg = lower_argument(builder, 0)?;
            builder.add_fastmem_remote_owner_fact(region, arg)?;
            crate::mir::builder::emission::constant::emit_void(builder)
        }
        FastMemIntrinsic::AssumeLocalFreeBlockNext => {
            let arg = lower_argument(builder, 0)?;
            builder.add_fastmem_block_next_fact(
                region,
                arg,
                FastMemBlockNextProofKind::SourceAssumeLocalFreeBlockNext,
            )?;
            crate::mir::builder::emission::constant::emit_void(builder)
        }
        FastMemIntrinsic::AssumeFreeHeadBlockNext => {
            let arg = lower_argument(builder, 0)?;
            builder.add_fastmem_block_next_fact(
                region,
                arg,
                FastMemBlockNextProofKind::SourceAssumeFreeHeadBlockNext,
            )?;
            crate::mir::builder::emission::constant::emit_void(builder)
        }
        FastMemIntrinsic::AssumeRemoteFreeBlockNext => {
            let arg = lower_argument(builder, 0)?;
            builder.add_fastmem_block_next_fact(
                region,
                arg,
                FastMemBlockNextProofKind::SourceAssumeRemoteFreeBlockNext,
            )?;
            crate::mir::builder::emission::constant::emit_void(builder)
        }
        FastMemIntrinsic::AssumeLocalFreeNonEmpty => {
            let arg = lower_argument(builder, 0)?;
            builder.add_fastmem_local_free_non_empty_fact(region, arg)?;
            crate::mir::builder::emission::constant::emit_void(builder)
        }
        FastMemIntrinsic::AssumeFreeHeadNonEmpty => {
            let arg = lower_argument(builder, 0)?;
            builder.add_fastmem_free_head_non_empty_fact(region, arg)?;
            crate::mir::builder::emission::constant::emit_void(builder)
        }
        FastMemIntrinsic::AssumeTableLength => {
            let table_id = fastmem_table_length_table_id(&arguments[0])?;
            let resolved_length = fastmem_positive_usize_source_value(builder, &arguments[1])?;
            let args = lower_fastmem_args_with(builder, arguments.len(), &mut lower_argument)?;
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
            let args = lower_fastmem_args_with(builder, arguments.len(), &mut lower_argument)?;
            let upper_value =
                builder.canonical_fastmem_range_upper_value(region, resolved_upper, args[1])?;
            builder.add_fastmem_range_index_fact(args[0], upper_value)?;
            crate::mir::builder::emission::constant::emit_void(builder)
        }
    }
}

pub(in crate::mir::builder) fn lower_prepared_fastmem_method_call_with_port_v1<Port>(
    builder: &mut MirBuilder,
    region: FastMemRegionId,
    intrinsic: PreparedFastMemIntrinsicV1,
    arguments: &[ASTNode],
    port: &mut Port,
    input: &Port::MethodCallInput,
) -> Result<ValueId, String>
where
    Port: crate::mir::builder::calls::MethodCallDescentPortV1,
{
    let spec = intrinsic.into_spec()?;
    lower_fastmem_intrinsic_with(builder, region, spec, arguments, |builder, index| {
        crate::mir::builder::calls::lower_method_call_argument_v1(builder, port, input, index)
    })
}

fn lower_fastmem_args_with<Lower>(
    builder: &mut MirBuilder,
    count: usize,
    lower_argument: &mut Lower,
) -> Result<Vec<ValueId>, String>
where
    Lower: FnMut(&mut MirBuilder, usize) -> Result<ValueId, String>,
{
    let mut values = Vec::with_capacity(count);
    for index in 0..count {
        values.push(lower_argument(builder, index)?);
    }
    Ok(values)
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
            let Some(value_id) = builder
                .function_state
                .variable_ctx
                .variable_map
                .get(name)
                .copied()
            else {
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
    let function = builder.function_state.current_function.as_ref()?;
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

#[cfg(test)]
mod tests {
    use super::{
        FastMemIntrinsicArity, PreparedFastMemIntrinsicRouteV1, PreparedFastMemIntrinsicV1,
    };

    #[test]
    fn intrinsic_preflight_seals_vocabulary_arity_and_rejections_once() {
        for (name, expected) in [
            ("mem.addr", 1),
            ("mem.currentAllocOwnerId", 0),
            ("mem.ownerEq", 2),
            ("mem.localFreePush", 2),
            ("mem.freeHeadPush", 2),
            ("mem.atomicRemoteHeadPush", 2),
            ("mem.atomicRemoteHeadDrain", 1),
            ("mem.drainRemoteListToLocal", 2),
            ("mem.localFreePop", 1),
            ("mem.freeHeadPop", 1),
            ("mem.assumeSameOwner", 2),
            ("mem.assumeRemoteOwner", 1),
            ("mem.assumeLocalFreeBlockNext", 1),
            ("mem.assumeFreeHeadBlockNext", 1),
            ("mem.assumeRemoteFreeBlockNext", 1),
            ("mem.assumeLocalFreeNonEmpty", 1),
            ("mem.assumeFreeHeadNonEmpty", 1),
            ("mem.assumeTableLength", 2),
            ("mem.assumeIndexInRange", 2),
        ] {
            let prepared = PreparedFastMemIntrinsicV1::prepare(name, expected);
            let PreparedFastMemIntrinsicRouteV1::Selected(spec) = prepared.route else {
                panic!("{name}/{expected} must select");
            };
            assert_eq!(spec.arity.expected(), expected);
        }

        assert!(matches!(
            PreparedFastMemIntrinsicV1::prepare("mem.unknown", 0).route,
            PreparedFastMemIntrinsicRouteV1::Forbidden(_)
        ));
        assert!(matches!(
            PreparedFastMemIntrinsicV1::prepare("mem.addr", 2).route,
            PreparedFastMemIntrinsicRouteV1::ArityMismatch {
                call: "mem.addr",
                expected: 1,
                actual: 2,
            }
        ));
        assert_eq!(FastMemIntrinsicArity::Zero.expected(), 0);
    }
}
