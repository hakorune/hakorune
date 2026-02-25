//! Block Finalizer - PHI-preserving block finalization
//!
//! Phase 260 P0.3: Extracted from joinir_block_converter.rs
//! Critical PHI preservation logic (Phase 189 FIX).
//!
//! ## Responsibilities
//!
//! 1. **finalize_block()**: Add instructions + terminator to block, preserving existing PHI
//! 2. **finalize_remaining_instructions()**: Flush pending instructions to current block
//!
//! ## Phase 189 FIX: PHI Preservation
//!
//! PHI instructions must remain at the beginning of the block.
//! When adding new instructions, existing PHIs are preserved and placed first.

use crate::ast::Span;
use crate::mir::{BasicBlockId, MirFunction, MirInstruction};

/// Finalize block with instructions and terminator
///
/// **Phase 189 FIX**: Preserves existing PHI instructions at block start.
/// PHI instructions must remain at the beginning of the block for SSA correctness.
///
/// # Arguments
///
/// * `mir_func` - Target MIR function to modify
/// * `block_id` - Block ID to finalize
/// * `instructions` - Instructions to add to block
/// * `terminator` - Block terminator (Branch/Jump/Return)
///
/// # PHI Preservation Logic
///
/// If block already has PHI instructions:
/// 1. Extract existing PHIs from block
/// 2. Merge: [PHIs] + [new instructions]
/// 3. Set merged instructions back to block
/// 4. Generate spans for all instructions
/// 5. Set terminator
///
/// If no existing PHIs:
/// 1. Set instructions directly
/// 2. Generate spans
/// 3. Set terminator
pub fn finalize_block(
    mir_func: &mut MirFunction,
    block_id: BasicBlockId,
    instructions: Vec<MirInstruction>,
    terminator: MirInstruction,
) {
    debug_log!(
        "[joinir_block/finalize_block] block_id={:?}, instructions.len()={}",
        block_id,
        instructions.len()
    );

    if let Some(block) = mir_func.blocks.get_mut(&block_id) {
        // Phase 189 FIX: Preserve existing PHI instructions at block start
        // PHI instructions must remain at the beginning of the block
        let existing_phis: Vec<_> = block
            .instructions
            .iter()
            .filter(|i| matches!(i, MirInstruction::Phi { .. }))
            .cloned()
            .collect();
        let phi_count = existing_phis.len();

        if phi_count > 0 {
            debug_log!(
                "[joinir_block/finalize_block] Preserving {} PHI instructions in block {:?}",
                phi_count,
                block_id
            );
            // PHI first, then new instructions
            let mut merged = existing_phis;
            merged.extend(instructions);
            let total_count = merged.len();
            block.instructions = merged;
            block.instruction_spans = vec![Span::unknown(); total_count];
        } else {
            let inst_count = instructions.len();
            block.instructions = instructions;
            block.instruction_spans = vec![Span::unknown(); inst_count];
        }
        block.set_terminator(terminator);
    }
}

/// Finalize remaining instructions without terminator
///
/// Flushes pending instructions to the current block.
/// **Phase 189 FIX**: Uses extend() to preserve existing instructions (e.g., PHI from handle_select()).
///
/// # Arguments
///
/// * `mir_func` - Target MIR function to modify
/// * `block_id` - Block ID to add instructions to
/// * `instructions` - Instructions to add
///
/// # Usage
///
/// Called when there are pending instructions but no terminator yet.
/// Common when building blocks incrementally.
pub fn finalize_remaining_instructions(
    mir_func: &mut MirFunction,
    block_id: BasicBlockId,
    mut instructions: Vec<MirInstruction>,
) {
    if !instructions.is_empty() {
        debug_log!(
            "[joinir_block] Final block {:?} has {} remaining instructions",
            block_id,
            instructions.len()
        );
        if let Some(block) = mir_func.blocks.get_mut(&block_id) {
            // Phase 189 FIX: Use extend() instead of assignment to preserve
            // existing instructions (e.g., PHI from handle_select())
            let new_count = instructions.len();
            block.instructions.append(&mut instructions);
            block
                .instruction_spans
                .extend(vec![Span::unknown(); new_count]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{BasicBlock, EffectMask, FunctionSignature, MirType, ValueId};

    #[test]
    fn test_finalize_block_no_phi() {
        let signature = FunctionSignature {
            name: "test".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let mut mir_func = MirFunction::new(signature, BasicBlockId::new(0));
        mir_func.blocks.insert(BasicBlockId::new(0), BasicBlock::new(BasicBlockId::new(0)));

        let instructions = vec![
            MirInstruction::Const {
                dst: ValueId(1),
                value: crate::mir::ConstValue::Integer(42),
            },
        ];
        let terminator = MirInstruction::Return { value: Some(ValueId(1)) };

        finalize_block(&mut mir_func, BasicBlockId::new(0), instructions, terminator);

        let block = mir_func.blocks.get(&BasicBlockId::new(0)).unwrap();
        assert_eq!(block.instructions.len(), 1);
        assert_eq!(block.instruction_spans.len(), 1);
        assert!(matches!(
            block.terminator,
            Some(MirInstruction::Return { .. })
        ));
    }

    #[test]
    fn test_finalize_block_with_phi() {
        let signature = FunctionSignature {
            name: "test".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let mut mir_func = MirFunction::new(signature, BasicBlockId::new(0));
        let mut block = BasicBlock::new(BasicBlockId::new(0));

        // Add existing PHI
        block.instructions.push(MirInstruction::Phi {
            dst: ValueId(10),
            inputs: vec![
                (BasicBlockId::new(1), ValueId(20)),
                (BasicBlockId::new(2), ValueId(30)),
            ],
            type_hint: None,
        });
        mir_func.blocks.insert(BasicBlockId::new(0), block);

        let instructions = vec![
            MirInstruction::Const {
                dst: ValueId(1),
                value: crate::mir::ConstValue::Integer(42),
            },
        ];
        let terminator = MirInstruction::Return { value: Some(ValueId(1)) };

        finalize_block(&mut mir_func, BasicBlockId::new(0), instructions, terminator);

        let block = mir_func.blocks.get(&BasicBlockId::new(0)).unwrap();
        assert_eq!(block.instructions.len(), 2); // PHI + Const

        // Verify PHI is first
        assert!(matches!(block.instructions[0], MirInstruction::Phi { .. }));
        assert!(matches!(block.instructions[1], MirInstruction::Const { .. }));
    }

    #[test]
    fn test_finalize_remaining_instructions() {
        let signature = FunctionSignature {
            name: "test".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let mut mir_func = MirFunction::new(signature, BasicBlockId::new(0));
        let mut block = BasicBlock::new(BasicBlockId::new(0));
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(5),
            value: crate::mir::ConstValue::Integer(10),
        });
        block.instruction_spans.push(Span::unknown());
        mir_func.blocks.insert(BasicBlockId::new(0), block);

        let instructions = vec![
            MirInstruction::Const {
                dst: ValueId(6),
                value: crate::mir::ConstValue::Integer(20),
            },
        ];

        finalize_remaining_instructions(&mut mir_func, BasicBlockId::new(0), instructions);

        let block = mir_func.blocks.get(&BasicBlockId::new(0)).unwrap();
        assert_eq!(block.instructions.len(), 2); // Original + new
        assert_eq!(block.instruction_spans.len(), 2);
    }

    #[test]
    fn test_finalize_remaining_instructions_empty() {
        let signature = FunctionSignature {
            name: "test".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let mut mir_func = MirFunction::new(signature, BasicBlockId::new(0));
        mir_func.blocks.insert(BasicBlockId::new(0), BasicBlock::new(BasicBlockId::new(0)));

        let instructions = vec![];

        finalize_remaining_instructions(&mut mir_func, BasicBlockId::new(0), instructions);

        let block = mir_func.blocks.get(&BasicBlockId::new(0)).unwrap();
        assert_eq!(block.instructions.len(), 0);
    }
}
