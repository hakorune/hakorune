//! Helper functions for plan_rewrites() stage
//!
//! Phase 286C-4 Step 1: Extracted from merge_and_rewrite() to support plan_rewrites()
//! These are pure helper functions that both the current monolithic code and the
//! new plan_rewrites() function can use.

use crate::mir::{BasicBlock, BasicBlockId, MirFunction, MirInstruction};
use crate::mir::builder::joinir_id_remapper::JoinIrIdRemapper;
use std::collections::BTreeMap;

/// Build a local block map for a single function
///
/// Maps old block IDs (from JoinIR function) to new block IDs (in host MIR).
/// This is needed for remapping Branch/Jump targets within the same function.
///
/// # Arguments
/// * `func_name` - Name of the function
/// * `func` - The function whose blocks to map
/// * `remapper` - The ID remapper containing block mappings
///
/// # Returns
/// * BTreeMap<old_block_id, new_block_id> for this function
pub fn build_local_block_map(
    func_name: &str,
    func: &MirFunction,
    remapper: &JoinIrIdRemapper,
) -> Result<BTreeMap<BasicBlockId, BasicBlockId>, String> {
    let mut local_block_map = BTreeMap::new();

    for old_block_id in func.blocks.keys() {
        let new_block_id = remapper
            .get_block(func_name, *old_block_id)
            .ok_or_else(|| format!("Block {:?} not found for {}", old_block_id, func_name))?;
        local_block_map.insert(*old_block_id, new_block_id);
    }

    Ok(local_block_map)
}

/// Synchronize instruction spans to match instruction count
///
/// After adding/removing instructions during rewriting, the instruction_spans
/// vector may not match the instructions vector. This function ensures they match.
///
/// # Arguments
/// * `instructions` - The current instructions
/// * `old_block` - The original block with spans
///
/// # Returns
/// * A spans vector that matches the instructions length
pub fn sync_spans(
    instructions: &[MirInstruction],
    old_block: &BasicBlock,
) -> Vec<crate::ast::Span> {
    let inst_count = instructions.len();
    let mut spans = old_block.instruction_spans.clone();

    if inst_count > spans.len() {
        // Use a default span for the extra instructions
        let default_span = spans
            .last()
            .copied()
            .unwrap_or_else(crate::ast::Span::unknown);
        spans.resize(inst_count, default_span);
    } else if inst_count < spans.len() {
        // Truncate spans to match instructions
        spans.truncate(inst_count);
    }

    spans
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Span;
    use crate::mir::ValueId;

    #[test]
    fn test_sync_spans_exact_match() {
        let instructions = vec![
            MirInstruction::Const { dst: ValueId(1), value: crate::mir::types::ConstValue::Integer(42) },
            MirInstruction::Const { dst: ValueId(2), value: crate::mir::types::ConstValue::Integer(43) },
        ];
        let mut block = BasicBlock::new(BasicBlockId(0));
        block.instruction_spans = vec![Span::unknown(), Span::unknown()];

        let result = sync_spans(&instructions, &block);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_sync_spans_more_instructions() {
        let instructions = vec![
            MirInstruction::Const { dst: ValueId(1), value: crate::mir::types::ConstValue::Integer(42) },
            MirInstruction::Const { dst: ValueId(2), value: crate::mir::types::ConstValue::Integer(43) },
            MirInstruction::Const { dst: ValueId(3), value: crate::mir::types::ConstValue::Integer(44) },
        ];
        let mut block = BasicBlock::new(BasicBlockId(0));
        block.instruction_spans = vec![Span::unknown()];

        let result = sync_spans(&instructions, &block);
        assert_eq!(result.len(), 3); // Should be padded
    }

    #[test]
    fn test_sync_spans_fewer_instructions() {
        let instructions = vec![
            MirInstruction::Const { dst: ValueId(1), value: crate::mir::types::ConstValue::Integer(42) },
        ];
        let mut block = BasicBlock::new(BasicBlockId(0));
        block.instruction_spans = vec![Span::unknown(), Span::unknown(), Span::unknown()];

        let result = sync_spans(&instructions, &block);
        assert_eq!(result.len(), 1); // Should be truncated
    }
}
