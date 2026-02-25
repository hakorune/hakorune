//! Shared utility functions for shape detection

use crate::mir::join_ir::{CompareOp, JoinFuncId, JoinFunction, JoinInst, JoinModule};

/// Name-based guard: Check if module contains function with exact name
pub(super) fn name_guard_exact(module: &JoinModule, expected_name: &str) -> bool {
    module.functions.values().any(|f| f.name == expected_name)
}

/// Find the loop_step function in the module
pub(super) fn find_loop_step(module: &JoinModule) -> Option<&JoinFunction> {
    module
        .functions
        .values()
        .find(|f| f.name == "loop_step")
        .or_else(|| module.functions.get(&JoinFuncId::new(1)))
}

/// Count Compare operations with specific op
pub(super) fn count_compare_ops(module: &JoinModule, target_op: CompareOp) -> usize {
    module
        .functions
        .values()
        .flat_map(|f| &f.body)
        .filter(|inst| match inst {
            JoinInst::Compute(mir_inst) => match mir_inst {
                crate::mir::join_ir::MirLikeInst::Compare { op, .. } => *op == target_op,
                _ => false,
            },
            _ => false,
        })
        .count()
}
