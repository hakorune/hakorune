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
    MirInstruction::Call {
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
    MirInstruction::Call {
        dst: Some(dst),
        func: ValueId::INVALID,
        callee: Some(Callee::Extern(name.to_string())),
        args,
        effects: EffectMask::PURE,
    }
}

fn ensure_ring0_initialized() {
    use crate::runtime::ring0::{default_ring0, init_global_ring0};
    let _ = std::panic::catch_unwind(|| {
        init_global_ring0(default_ring0());
    });
}
