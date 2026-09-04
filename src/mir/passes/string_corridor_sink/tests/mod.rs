use super::*;
use crate::ast::Span;
use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
use crate::mir::{
    BasicBlock, BasicBlockId, EffectMask, FunctionSignature, MirCompiler, MirModule, MirType,
};
use crate::parser::NyashParser;

mod benchmarks;
mod boundaries;
mod concat_and_return;
mod materialization;
mod method_set;
mod substring_len;

fn method_call(
    dst: ValueId,
    receiver: ValueId,
    box_name: &str,
    method: &str,
    args: Vec<ValueId>,
    ty: MirType,
) -> MirInstruction {
    let _ = ty;
    MirInstruction::LegacyCallV0 {
        dst: Some(dst),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: box_name.to_string(),
            method: method.to_string(),
            receiver: Some(receiver),
            certainty: TypeCertainty::Known,
            box_kind: CalleeBoxKind::RuntimeData,
        }),
        args,
        effects: EffectMask::PURE,
    }
}

fn extern_call(dst: ValueId, name: &str, args: Vec<ValueId>) -> MirInstruction {
    MirInstruction::LegacyCallV0 {
        dst: Some(dst),
        func: ValueId::INVALID,
        callee: Some(Callee::Extern(name.to_string())),
        args,
        effects: EffectMask::PURE,
    }
}

/// Test-only read projection for the canonical and compatibility call carriers.
/// Assertions should observe the published callee/operands, not force a
/// particular transport variant.
fn call_parts(inst: &MirInstruction) -> Option<(Option<ValueId>, &Callee, &[ValueId], EffectMask)> {
    match inst {
        MirInstruction::Call(call) => Some((call.dst, &call.callee, &call.args, call.effects)),
        MirInstruction::LegacyCallV0 {
            dst,
            callee: Some(callee),
            args,
            effects,
            ..
        } => Some((*dst, callee, args, *effects)),
        _ => None,
    }
}

fn is_extern_call(
    inst: &MirInstruction,
    dst: ValueId,
    name: &str,
    args: &[ValueId],
    effects: Option<EffectMask>,
) -> bool {
    let Some((actual_dst, callee, actual_args, actual_effects)) = call_parts(inst) else {
        return false;
    };
    matches!(callee, Callee::Extern(actual_name) if actual_name == name)
        && actual_dst == Some(dst)
        && actual_args == args
        && effects.is_none_or(|expected| actual_effects == expected)
}

fn is_method_call(
    inst: &MirInstruction,
    dst: ValueId,
    box_name: &str,
    method: &str,
    receiver: ValueId,
    args: &[ValueId],
) -> bool {
    let Some((actual_dst, callee, actual_args, _)) = call_parts(inst) else {
        return false;
    };
    matches!(
        callee,
        Callee::Method {
            box_name: actual_box,
            method: actual_method,
            receiver: Some(actual_receiver),
            ..
        } if actual_box == box_name
            && actual_method == method
            && *actual_receiver == receiver
    ) && actual_dst == Some(dst)
        && actual_args == args
}

fn ensure_ring0_initialized() {
    use crate::runtime::ring0::{default_ring0, init_global_ring0};
    let _ = std::panic::catch_unwind(|| {
        init_global_ring0(default_ring0());
    });
}
