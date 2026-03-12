//! IfMerge Handler - JoinIR IfMerge instruction to MIR conversion
//!
//! Phase 260 P0.3: Extracted from joinir_block_converter.rs (lines 610-688)
//!
//! ## Responsibility
//!
//! Converts JoinIR IfMerge instructions to MIR control flow with if/phi pattern.
//! Handles multiple variable merges simultaneously (e.g., sum and count in fold operations).
//!
//! ## Key Design Points
//!
//! ### 1. IfMerge → If/Phi Transformation (Phase 33-6)
//!
//! JoinIR IfMerge instructions expand to conditional control flow with merge copies:
//!
//! ```text
//! JoinIR:
//!   IfMerge {
//!     cond: v100,
//!     merges: [
//!       MergePair { dst: v10, then_val: v20, else_val: v30 },
//!       MergePair { dst: v11, then_val: v21, else_val: v31 },
//!     ],
//!     k_next: None
//!   }
//!
//! MIR:
//!   Cond Block:
//!     Branch { condition: v100, then_bb: then_block, else_bb: else_block }
//!
//!   Then Block:
//!     v10 = Copy v20
//!     v11 = Copy v21
//!     Jump merge_block
//!
//!   Else Block:
//!     v10 = Copy v30
//!     v11 = Copy v31
//!     Jump merge_block
//!
//!   Merge Block:
//!     (ready for next instruction)
//! ```
//!
//! ### 2. Multiple Variable Support
//!
//! IfMerge can handle multiple merge pairs simultaneously, enabling efficient
//! parallel updates (e.g., accumulator and counter in fold operations).
//!
//! ### 3. Block Structure
//!
//! ```text
//! cond_block (current)
//!   ↓ (Branch)
//!   ├─→ then_block → merge_block
//!   └─→ else_block → merge_block
//!
//! Continuation: merge_block becomes new current_block
//! ```
//!
//! ### 4. k_next Limitation (Phase 33-6)
//!
//! Currently, k_next (continuation after merge) is not supported.
//! All IfMerge instructions must have k_next = None.
//!
//! ## Example Conversions
//!
//! ### Single Variable Merge
//!
//! ```text
//! JoinIR:
//!   IfMerge { cond: v1, merges: [{ dst: v10, then_val: v20, else_val: v30 }] }
//!
//! MIR:
//!   Cond Block (bb0):
//!     Branch { condition: v1, then_bb: bb1, else_bb: bb2 }
//!
//!   Then Block (bb1):
//!     v10 = Copy v20
//!     Jump bb3
//!
//!   Else Block (bb2):
//!     v10 = Copy v30
//!     Jump bb3
//!
//!   Merge Block (bb3):
//!     (current_block_id = bb3)
//! ```
//!
//! ### Multiple Variable Merge
//!
//! ```text
//! JoinIR:
//!   IfMerge {
//!     cond: v50,
//!     merges: [
//!       { dst: v100, then_val: v101, else_val: v102 },
//!       { dst: v200, then_val: v201, else_val: v202 },
//!     ]
//!   }
//!
//! MIR:
//!   Cond Block (bb0):
//!     Branch { condition: v50, then_bb: bb1, else_bb: bb2 }
//!
//!   Then Block (bb1):
//!     v100 = Copy v101
//!     v200 = Copy v201
//!     Jump bb3
//!
//!   Else Block (bb2):
//!     v100 = Copy v102
//!     v200 = Copy v202
//!     Jump bb3
//!
//!   Merge Block (bb3):
//!     (current_block_id = bb3)
//! ```

use crate::mir::builder::copy_emitter::{self, CopyEmitReason};
use crate::mir::join_ir::{JoinContId, MergePair};
use crate::mir::{BasicBlock, BasicBlockId, MirFunction, MirInstruction, ValueId};

use super::super::JoinIrVmBridgeError;

/// Handle JoinIR IfMerge instruction
///
/// Converts IfMerge to MIR control flow with conditional branches and merge copies.
/// Phase 33-6: Supports multiple variable merges (e.g., sum and count in fold).
///
/// # Arguments
///
/// * `mir_func` - Target MIR function to modify
/// * `current_block_id` - Current block ID (will become cond block)
/// * `current_instructions` - Instructions to finalize in cond block
/// * `cond` - Condition ValueId for branch
/// * `merges` - Array of MergePair (dst, then_val, else_val) for each variable
/// * `k_next` - Optional continuation (currently not supported, must be None)
/// * `next_block_id` - Reference to next available block ID (will be incremented by 3)
/// * `finalize_fn` - Function to finalize blocks (signature matches finalize_block)
///
/// # Returns
///
/// Returns the new current_block_id (merge_block) on success.
///
/// # Errors
///
/// Returns error if k_next is Some (not yet supported).
pub fn handle_if_merge<F>(
    mir_func: &mut MirFunction,
    current_block_id: BasicBlockId,
    current_instructions: Vec<MirInstruction>,
    cond: &ValueId,
    merges: &[MergePair],
    k_next: &Option<JoinContId>,
    next_block_id: &mut u32,
    finalize_fn: F,
) -> Result<BasicBlockId, JoinIrVmBridgeError>
where
    F: FnOnce(&mut MirFunction, BasicBlockId, Vec<MirInstruction>, MirInstruction),
{
    // Phase 33-6: IfMerge → if/phi (multiple variables)
    if k_next.is_some() {
        return Err(JoinIrVmBridgeError::new(
            "IfMerge: k_next not yet supported".to_string(),
        ));
    }

    if crate::config::env::joinir_test_debug_enabled() {
        crate::runtime::get_global_ring0().log.debug(&format!(
            "[joinir_block] Converting IfMerge: merges.len()={}",
            merges.len()
        ));
    }

    // Allocate 3 blocks: then, else, merge
    let cond_block = current_block_id;
    let then_block = BasicBlockId(*next_block_id);
    *next_block_id += 1;
    let else_block = BasicBlockId(*next_block_id);
    *next_block_id += 1;
    let merge_block = BasicBlockId(*next_block_id);
    *next_block_id += 1;

    // Cond block: branch terminator
    let branch_terminator = MirInstruction::Branch {
        condition: *cond,
        then_bb: then_block,
        else_bb: else_block,
        then_edge_args: None,
        else_edge_args: None,
    };
    finalize_fn(
        mir_func,
        cond_block,
        current_instructions,
        branch_terminator,
    );

    // Then block: copy then_val for each merge
    let then_block_obj = BasicBlock::new(then_block);
    mir_func.blocks.insert(then_block, then_block_obj);
    for merge in merges {
        copy_emitter::emit_copy_in_block(
            mir_func,
            then_block,
            merge.dst,
            merge.then_val,
            CopyEmitReason::JoinIrBridgeIfMergeThen,
        )
        .map_err(JoinIrVmBridgeError::new)?;
    }
    mir_func
        .get_block_mut(then_block)
        .ok_or_else(|| JoinIrVmBridgeError::new(format!("then block {:?} missing", then_block)))?
        .set_terminator(MirInstruction::Jump {
            target: merge_block,
            edge_args: None,
        });

    // Else block: copy else_val for each merge
    let else_block_obj = BasicBlock::new(else_block);
    mir_func.blocks.insert(else_block, else_block_obj);
    for merge in merges {
        copy_emitter::emit_copy_in_block(
            mir_func,
            else_block,
            merge.dst,
            merge.else_val,
            CopyEmitReason::JoinIrBridgeIfMergeElse,
        )
        .map_err(JoinIrVmBridgeError::new)?;
    }
    mir_func
        .get_block_mut(else_block)
        .ok_or_else(|| JoinIrVmBridgeError::new(format!("else block {:?} missing", else_block)))?
        .set_terminator(MirInstruction::Jump {
            target: merge_block,
            edge_args: None,
        });

    // Merge block (empty, ready for next instruction)
    let merge_block_obj = BasicBlock::new(merge_block);
    mir_func.blocks.insert(merge_block, merge_block_obj);

    Ok(merge_block)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::function::{FunctionSignature, MirFunction};
    use crate::mir::{BasicBlockId, MirInstruction, ValueId};

    /// Mock finalize function for testing
    fn mock_finalize(
        mir_func: &mut MirFunction,
        block_id: BasicBlockId,
        instructions: Vec<MirInstruction>,
        terminator: MirInstruction,
    ) {
        if let Some(block) = mir_func.blocks.get_mut(&block_id) {
            block.instructions.extend(instructions);
            block.set_terminator(terminator);
        }
    }

    /// Helper to create a minimal MirFunction for testing
    fn create_test_function() -> MirFunction {
        let signature = FunctionSignature {
            name: "test_func".to_string(),
            params: vec![],
            return_type: crate::mir::MirType::Void,
            effects: crate::mir::EffectMask::PURE,
        };
        let entry_block = BasicBlockId(0);
        MirFunction::new(signature, entry_block)
    }

    #[test]
    fn test_handle_if_merge_single_variable() {
        let mut mir_func = create_test_function();

        let current_block = BasicBlockId(0);

        let cond = ValueId(100);
        let dst = ValueId(10);
        let then_val = ValueId(20);
        let else_val = ValueId(30);

        let merges = vec![MergePair {
            dst,
            then_val,
            else_val,
            type_hint: None,
        }];

        let mut next_block_id = 1;
        let result = handle_if_merge(
            &mut mir_func,
            current_block,
            vec![],
            &cond,
            &merges,
            &None,
            &mut next_block_id,
            mock_finalize,
        );

        assert!(result.is_ok());
        let merge_block = result.unwrap();
        assert_eq!(merge_block, BasicBlockId(3));
        assert_eq!(next_block_id, 4); // 3 blocks allocated

        // Verify cond block has Branch terminator
        let cond_block_obj = mir_func.blocks.get(&current_block).unwrap();
        assert!(matches!(
            cond_block_obj.terminator,
            Some(MirInstruction::Branch { .. })
        ));

        // Verify then block has Copy and Jump
        let then_block_obj = mir_func.blocks.get(&BasicBlockId(1)).unwrap();
        assert_eq!(then_block_obj.instructions.len(), 1);
        assert!(matches!(
            then_block_obj.instructions[0],
            MirInstruction::Copy { dst: d, src: s } if d == dst && s == then_val
        ));
        assert!(matches!(
            then_block_obj.terminator,
            Some(MirInstruction::Jump { target: t, .. }) if t == merge_block
        ));

        // Verify else block has Copy and Jump
        let else_block_obj = mir_func.blocks.get(&BasicBlockId(2)).unwrap();
        assert_eq!(else_block_obj.instructions.len(), 1);
        assert!(matches!(
            else_block_obj.instructions[0],
            MirInstruction::Copy { dst: d, src: s } if d == dst && s == else_val
        ));
        assert!(matches!(
            else_block_obj.terminator,
            Some(MirInstruction::Jump { target: t, .. }) if t == merge_block
        ));

        // Verify merge block exists and is empty
        let merge_block_obj = mir_func.blocks.get(&merge_block).unwrap();
        assert_eq!(merge_block_obj.instructions.len(), 0);
    }

    #[test]
    fn test_handle_if_merge_multiple_variables() {
        let mut mir_func = create_test_function();

        let current_block = BasicBlockId(0);

        let cond = ValueId(50);
        let merges = vec![
            MergePair {
                dst: ValueId(100),
                then_val: ValueId(101),
                else_val: ValueId(102),
                type_hint: None,
            },
            MergePair {
                dst: ValueId(200),
                then_val: ValueId(201),
                else_val: ValueId(202),
                type_hint: None,
            },
        ];

        let mut next_block_id = 1;
        let result = handle_if_merge(
            &mut mir_func,
            current_block,
            vec![],
            &cond,
            &merges,
            &None,
            &mut next_block_id,
            mock_finalize,
        );

        assert!(result.is_ok());
        let _merge_block = result.unwrap();

        // Verify then block has 2 Copy instructions
        let then_block_obj = mir_func.blocks.get(&BasicBlockId(1)).unwrap();
        assert_eq!(then_block_obj.instructions.len(), 2);
        assert!(matches!(
            then_block_obj.instructions[0],
            MirInstruction::Copy {
                dst: ValueId(100),
                src: ValueId(101)
            }
        ));
        assert!(matches!(
            then_block_obj.instructions[1],
            MirInstruction::Copy {
                dst: ValueId(200),
                src: ValueId(201)
            }
        ));

        // Verify else block has 2 Copy instructions
        let else_block_obj = mir_func.blocks.get(&BasicBlockId(2)).unwrap();
        assert_eq!(else_block_obj.instructions.len(), 2);
        assert!(matches!(
            else_block_obj.instructions[0],
            MirInstruction::Copy {
                dst: ValueId(100),
                src: ValueId(102)
            }
        ));
        assert!(matches!(
            else_block_obj.instructions[1],
            MirInstruction::Copy {
                dst: ValueId(200),
                src: ValueId(202)
            }
        ));
    }

    #[test]
    fn test_handle_if_merge_rejects_k_next() {
        let mut mir_func = create_test_function();

        let current_block = BasicBlockId(0);

        let cond = ValueId(100);
        let merges = vec![MergePair {
            dst: ValueId(10),
            then_val: ValueId(20),
            else_val: ValueId(30),
            type_hint: None,
        }];

        let k_next = Some(JoinContId(999));
        let mut next_block_id = 1;

        let result = handle_if_merge(
            &mut mir_func,
            current_block,
            vec![],
            &cond,
            &merges,
            &k_next,
            &mut next_block_id,
            mock_finalize,
        );

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("k_next not yet supported"));
    }

    #[test]
    fn test_handle_if_merge_empty_merges() {
        // Edge case: no merge pairs (should still create blocks)
        let mut mir_func = create_test_function();

        let current_block = BasicBlockId(0);

        let cond = ValueId(100);
        let merges: Vec<MergePair> = vec![];

        let mut next_block_id = 1;
        let result = handle_if_merge(
            &mut mir_func,
            current_block,
            vec![],
            &cond,
            &merges,
            &None,
            &mut next_block_id,
            mock_finalize,
        );

        assert!(result.is_ok());

        // Verify blocks exist even with empty merges
        let then_block_obj = mir_func.blocks.get(&BasicBlockId(1)).unwrap();
        assert_eq!(then_block_obj.instructions.len(), 0);

        let else_block_obj = mir_func.blocks.get(&BasicBlockId(2)).unwrap();
        assert_eq!(else_block_obj.instructions.len(), 0);
    }
}
