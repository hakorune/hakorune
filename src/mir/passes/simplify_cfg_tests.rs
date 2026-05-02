use super::*;
use crate::ast::Span;
use crate::mir::{
    join_ir::lowering::inline_boundary::JumpArgsLayout, BasicBlock, ConstValue, EdgeArgs,
    EffectMask, FunctionSignature, MirModule, MirType, ValueId,
};

fn test_signature(name: &str, return_type: MirType) -> FunctionSignature {
    FunctionSignature {
        name: name.to_string(),
        params: vec![],
        return_type,
        effects: EffectMask::PURE,
    }
}

#[path = "simplify_cfg_tests/branch_threading.rs"]
mod branch_threading;
#[path = "simplify_cfg_tests/constant_folds.rs"]
mod constant_folds;
#[path = "simplify_cfg_tests/jump_merge.rs"]
mod jump_merge;
