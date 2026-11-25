pub(super) use crate::ast::Span;
pub(super) use crate::mir::builder::copy_emitter::{self, CopyEmitReason};
pub(super) use crate::mir::{
    BasicBlock, BasicBlockId, Callee, ConstValue, EffectMask, FunctionSignature, MirFunction,
    MirInstruction, MirModule, MirType, ValueId,
};

mod liveness;
mod local_fields;
mod memory;
mod observer;
mod overwrite;

fn eliminate_dead_code(module: &mut MirModule) -> usize {
    super::eliminate_dead_code_with_required_roots(module, &Default::default())
}
