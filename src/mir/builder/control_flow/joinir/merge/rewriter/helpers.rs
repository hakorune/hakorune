//! Helper utilities for instruction rewriter
//!
//! Phase 260 P0.1 Step 3: Extracted from instruction_rewriter.rs
//! Small, pure functions with no external dependencies.

use crate::mir::MirFunction;

/// Phase 132-R0 Task 3: Structural check for skippable continuation functions
///
/// A continuation function is skippable if it is a pure exit stub:
/// - 1 block only
/// - No instructions
/// - Return terminator only
///
/// This is a structural check (no by-name/by-id inference).
///
/// # Example
///
/// ```ignore
/// // Skippable (pure exit stub)
/// fn k_exit(ret_val) {
///     return ret_val
/// }
///
/// // Not skippable (has instructions)
/// fn k_exit(ret_val) {
///     local computed = ret_val + 1
///     return computed
/// }
/// ```
pub(in crate::mir::builder::control_flow::joinir::merge) fn is_skippable_continuation(
    func: &MirFunction,
) -> bool {
    if func.blocks.len() != 1 {
        return false;
    }
    let Some(block) = func.blocks.get(&func.entry_block) else {
        return false;
    };
    if !block.instructions.is_empty() {
        return false;
    }
    matches!(
        block.terminator,
        Some(crate::mir::MirInstruction::Return { .. })
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{BasicBlock, BasicBlockId, EffectMask, FunctionSignature, MirInstruction, MirType};

    fn make_test_function(name: &str) -> MirFunction {
        let signature = FunctionSignature {
            name: name.to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        MirFunction::new(signature, BasicBlockId::new(0))
    }

    #[test]
    fn test_is_skippable_continuation_pure_stub() {
        // Pure exit stub: 1 block, no instructions, return only
        let mut func = make_test_function("k_exit");
        let entry_block_id = func.entry_block;

        // MirFunction::new already creates entry block, just set terminator
        if let Some(block) = func.blocks.get_mut(&entry_block_id) {
            block.set_terminator(MirInstruction::Return { value: None });
        }

        assert!(is_skippable_continuation(&func));
    }

    #[test]
    fn test_is_skippable_continuation_has_instructions() {
        // Has instructions: not skippable
        let mut func = make_test_function("k_exit");
        let entry_block_id = func.entry_block;

        if let Some(block) = func.blocks.get_mut(&entry_block_id) {
            block.instructions.push(MirInstruction::Const {
                dst: crate::mir::ValueId::new(1),
                value: crate::mir::types::ConstValue::Integer(42),
            });
            block.set_terminator(MirInstruction::Return { value: None });
        }

        assert!(!is_skippable_continuation(&func));
    }

    #[test]
    fn test_is_skippable_continuation_multiple_blocks() {
        // Multiple blocks: not skippable
        let mut func = make_test_function("k_exit");
        let entry_block_id = func.entry_block;

        if let Some(block) = func.blocks.get_mut(&entry_block_id) {
            block.set_terminator(MirInstruction::Return { value: None });
        }

        // Add second block
        let block2 = BasicBlock::new(BasicBlockId::new(1));
        func.add_block(block2);

        assert!(!is_skippable_continuation(&func));
    }
}
