use super::*;
use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
use crate::mir::{
    BasicBlock, BasicBlockId, CompareOp, EffectMask, FunctionSignature, MirType, UnaryOp, ValueId,
};

use super::string_return_profile::generic_string_void_sentinel_return_global_blocker;

fn make_function_with_global_call_args(
    name: &str,
    dst: Option<ValueId>,
    args: Vec<ValueId>,
) -> MirFunction {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let block = function
        .blocks
        .entry(BasicBlockId::new(0))
        .or_insert_with(|| BasicBlock::new(BasicBlockId::new(0)));
    block.instructions.push(MirInstruction::Call {
        dst,
        func: ValueId::INVALID,
        callee: Some(Callee::Global(name.to_string())),
        args,
        effects: EffectMask::PURE,
    });
    function
}

fn make_function_with_global_call(name: &str, dst: Option<ValueId>) -> MirFunction {
    make_function_with_global_call_args(name, dst, vec![ValueId::new(1), ValueId::new(2)])
}

mod blockers;
mod builder_registry_dispatch;
mod core;
mod generic_i64;
mod hostbridge;
mod jsonfrag_normalizer;
mod runtime_methods;
mod shape_reasons;
mod static_string_array;
mod void_logging;
mod void_sentinel;
