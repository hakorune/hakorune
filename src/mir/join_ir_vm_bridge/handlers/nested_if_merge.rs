//! NestedIfMerge Handler - JoinIR NestedIfMerge instruction to MIR conversion
//!
//! Phase 260 P0.3: Extracted from joinir_block_converter.rs (lines 705-819)
//!
//! ## Responsibility
//!
//! Converts JoinIR NestedIfMerge instructions to MIR multi-level control flow.
//! This is the **most complex handler** in the system, handling N-level nested
//! conditional branching with merge operations.
//!
//! ## Key Design Points
//!
//! ### 1. NestedIfMerge → Multi-Level Branch Transformation (Phase 41-4)
//!
//! JoinIR NestedIfMerge instructions expand to multi-level conditional control flow
//! where each condition creates a nested branch level:
//!
//! ```text
//! JoinIR:
//!   NestedIfMerge {
//!     conds: [v100, v101],  // 2 levels of conditions
//!     merges: [
//!       MergePair { dst: v10, then_val: v20, else_val: v30 },
//!     ],
//!     k_next: None
//!   }
//!
//! MIR (2-level nesting):
//!   Level 0 Block (bb0):
//!     Branch { condition: v100, then_bb: bb1 (level 1), else_bb: bb3 (final_else) }
//!
//!   Level 1 Block (bb1):
//!     Branch { condition: v101, then_bb: bb2 (then), else_bb: bb3 (final_else) }
//!
//!   Then Block (bb2):
//!     v10 = Copy v20
//!     Jump bb4 (merge)
//!
//!   Final Else Block (bb3):
//!     v10 = Copy v30
//!     Jump bb4 (merge)
//!
//!   Merge Block (bb4):
//!     (ready for next instruction)
//! ```
//!
//! ### 2. Dynamic Block Allocation (N+3 blocks)
//!
//! The handler allocates exactly N+3 blocks for N conditions:
//! - N level blocks (including current block for level 0)
//! - 1 then block (reached when all conditions are true)
//! - 1 final_else block (reached when any condition is false)
//! - 1 merge block (continuation point)
//!
//! ### 3. Cascading Branch Logic
//!
//! Each level block branches to:
//! - **Then target**: Next level block (or then_block if last level)
//! - **Else target**: Always final_else_block (short-circuit on first false)
//!
//! This creates a cascading AND pattern: all conditions must be true to reach then_block.
//!
//! ### 4. Block Structure (3-level example)
//!
//! ```text
//! level_0_block (current)
//!   ↓ (Branch cond[0])
//!   ├─→ level_1_block
//!   │   ↓ (Branch cond[1])
//!   │   ├─→ level_2_block
//!   │   │   ↓ (Branch cond[2])
//!   │   │   ├─→ then_block → merge_block
//!   │   │   └─→ final_else_block → merge_block
//!   │   └─→ final_else_block → merge_block
//!   └─→ final_else_block → merge_block
//!
//! Continuation: merge_block becomes new current_block
//! ```
//!
//! ### 5. k_next Limitation (Phase 41-4)
//!
//! Currently, k_next (continuation after merge) is not supported.
//! All NestedIfMerge instructions must have k_next = None.
//!
//! ## Example Conversions
//!
//! ### Two-Level Nesting
//!
//! ```text
//! JoinIR:
//!   NestedIfMerge {
//!     conds: [v1, v2],
//!     merges: [{ dst: v10, then_val: v20, else_val: v30 }]
//!   }
//!
//! MIR:
//!   Level 0 Block (bb0):
//!     Branch { condition: v1, then_bb: bb1, else_bb: bb3 }
//!
//!   Level 1 Block (bb1):
//!     Branch { condition: v2, then_bb: bb2, else_bb: bb3 }
//!
//!   Then Block (bb2):
//!     v10 = Copy v20
//!     Jump bb4
//!
//!   Final Else Block (bb3):
//!     v10 = Copy v30
//!     Jump bb4
//!
//!   Merge Block (bb4):
//!     (current_block_id = bb4)
//! ```
//!
//! ### Three-Level Nesting with Multiple Variables
//!
//! ```text
//! JoinIR:
//!   NestedIfMerge {
//!     conds: [v50, v51, v52],
//!     merges: [
//!       { dst: v100, then_val: v101, else_val: v102 },
//!       { dst: v200, then_val: v201, else_val: v202 },
//!     ]
//!   }
//!
//! MIR:
//!   Level 0 Block (bb0):
//!     Branch { condition: v50, then_bb: bb1, else_bb: bb4 }
//!
//!   Level 1 Block (bb1):
//!     Branch { condition: v51, then_bb: bb2, else_bb: bb4 }
//!
//!   Level 2 Block (bb2):
//!     Branch { condition: v52, then_bb: bb3, else_bb: bb4 }
//!
//!   Then Block (bb3):
//!     v100 = Copy v101
//!     v200 = Copy v201
//!     Jump bb5
//!
//!   Final Else Block (bb4):
//!     v100 = Copy v102
//!     v200 = Copy v202
//!     Jump bb5
//!
//!   Merge Block (bb5):
//!     (current_block_id = bb5)
//! ```

use crate::mir::builder::copy_emitter::{self, CopyEmitReason};
use crate::mir::join_ir::{JoinContId, MergePair};
use crate::mir::{BasicBlock, BasicBlockId, MirFunction, MirInstruction, ValueId};

use super::super::JoinIrVmBridgeError;

/// Handle JoinIR NestedIfMerge instruction
///
/// Converts NestedIfMerge to MIR multi-level control flow with cascading branches.
/// Phase 41-4: Supports N-level nesting with multiple variable merges.
///
/// # Arguments
///
/// * `mir_func` - Target MIR function to modify
/// * `current_block_id` - Current block ID (will become level 0 block)
/// * `current_instructions` - Instructions to finalize in level 0 block
/// * `conds` - Array of condition ValueIds (one per nesting level, must not be empty)
/// * `merges` - Array of MergePair (dst, then_val, else_val) for each variable
/// * `k_next` - Optional continuation (currently not supported, must be None)
/// * `next_block_id` - Reference to next available block ID (will be incremented by N+2)
/// * `finalize_fn` - Function to finalize blocks (signature matches finalize_block)
///
/// # Returns
///
/// Returns the new current_block_id (merge_block) on success.
///
/// # Errors
///
/// Returns error if:
/// - k_next is Some (not yet supported)
/// - conds is empty (invalid configuration)
pub fn handle_nested_if_merge<F>(
    mir_func: &mut MirFunction,
    current_block_id: BasicBlockId,
    current_instructions: Vec<MirInstruction>,
    conds: &[ValueId],
    merges: &[MergePair],
    k_next: &Option<JoinContId>,
    next_block_id: &mut u32,
    finalize_fn: F,
) -> Result<BasicBlockId, JoinIrVmBridgeError>
where
    F: FnOnce(&mut MirFunction, BasicBlockId, Vec<MirInstruction>, MirInstruction),
{
    // Phase 41-4: NestedIfMerge → multi-level Branch + PHI
    if k_next.is_some() {
        return Err(JoinIrVmBridgeError::new(
            "NestedIfMerge: k_next not yet supported".to_string(),
        ));
    }

    if conds.is_empty() {
        return Err(JoinIrVmBridgeError::new(
            "NestedIfMerge: conds must not be empty".to_string(),
        ));
    }

    if crate::config::env::joinir_test_debug_enabled() {
        crate::runtime::get_global_ring0().log.debug(&format!(
            "[joinir_block] Converting NestedIfMerge: conds.len()={}",
            conds.len()
        ));
    }

    let num_conds = conds.len();

    // Allocate N+3 blocks: N level blocks (including current), then, final_else, merge
    let mut level_blocks: Vec<BasicBlockId> = Vec::with_capacity(num_conds);
    level_blocks.push(current_block_id); // Level 0 is current block

    // Allocate level 1..N blocks
    for _ in 1..num_conds {
        level_blocks.push(BasicBlockId(*next_block_id));
        *next_block_id += 1;
    }

    // Allocate then, final_else, and merge blocks
    let then_block = BasicBlockId(*next_block_id);
    *next_block_id += 1;
    let final_else_block = BasicBlockId(*next_block_id);
    *next_block_id += 1;
    let merge_block = BasicBlockId(*next_block_id);
    *next_block_id += 1;

    // Pre-create level 1+ blocks
    for level in 1..num_conds {
        mir_func.blocks.insert(
            level_blocks[level],
            BasicBlock::new(level_blocks[level]),
        );
    }

    // Finalize level 0 (current block) with finalize_fn (FnOnce constraint)
    // This must be done first and separately due to FnOnce
    let level_0_cond = conds[0];
    let level_0_next = if num_conds > 1 {
        level_blocks[1]
    } else {
        then_block
    };
    let level_0_branch = MirInstruction::Branch {
        condition: level_0_cond,
        then_bb: level_0_next,
        else_bb: final_else_block,
        then_edge_args: None,
        else_edge_args: None,
    };
    finalize_fn(mir_func, current_block_id, current_instructions, level_0_branch);

    // Now set terminators for level 1+ blocks
    for level in 1..num_conds {
        let this_block = level_blocks[level];
        let cond_var = conds[level];

        let next_true_block = if level + 1 < num_conds {
            level_blocks[level + 1]
        } else {
            then_block
        };

        let branch_terminator = MirInstruction::Branch {
            condition: cond_var,
            then_bb: next_true_block,
            else_bb: final_else_block,
            then_edge_args: None,
            else_edge_args: None,
        };

        if let Some(block) = mir_func.blocks.get_mut(&this_block) {
            block.set_terminator(branch_terminator);
        }
    }

    let mut then_block_obj = BasicBlock::new(then_block);
    then_block_obj.set_terminator(MirInstruction::Jump {
        target: merge_block,
        edge_args: None,
    });
    mir_func.blocks.insert(then_block, then_block_obj);

    let mut else_block_obj = BasicBlock::new(final_else_block);
    else_block_obj.set_terminator(MirInstruction::Jump {
        target: merge_block,
        edge_args: None,
    });
    mir_func.blocks.insert(final_else_block, else_block_obj);

    let merge_block_obj = BasicBlock::new(merge_block);
    mir_func.blocks.insert(merge_block, merge_block_obj);

    // Then block: copy then_val for each merge
    for merge in merges {
        copy_emitter::emit_copy_in_block(
            mir_func,
            then_block,
            merge.dst,
            merge.then_val,
            CopyEmitReason::JoinIrBridgeNestedIfMergeThen,
        )
        .map_err(JoinIrVmBridgeError::new)?;
    }

    // Final else block: copy else_val for each merge
    for merge in merges {
        copy_emitter::emit_copy_in_block(
            mir_func,
            final_else_block,
            merge.dst,
            merge.else_val,
            CopyEmitReason::JoinIrBridgeNestedIfMergeElse,
        )
        .map_err(JoinIrVmBridgeError::new)?;
    }

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
    fn test_handle_nested_if_merge_two_levels() {
        let mut mir_func = create_test_function();

        let current_block = BasicBlockId(0);

        let conds = vec![ValueId(100), ValueId(101)];
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
        let result = handle_nested_if_merge(
            &mut mir_func,
            current_block,
            vec![],
            &conds,
            &merges,
            &None,
            &mut next_block_id,
            mock_finalize,
        );

        assert!(result.is_ok());
        let merge_block = result.unwrap();

        // Verify block allocation: 2 levels + then + else + merge = 5 blocks total
        // Level 0: bb0 (current), Level 1: bb1, Then: bb2, Else: bb3, Merge: bb4
        assert_eq!(merge_block, BasicBlockId(4));
        assert_eq!(next_block_id, 5); // 4 blocks allocated (level 1 + then + else + merge)

        // Verify level 0 block branches to level 1 or final_else
        let level_0_block = mir_func.blocks.get(&BasicBlockId(0)).unwrap();
        assert!(matches!(
            level_0_block.terminator,
            Some(MirInstruction::Branch {
                condition: ValueId(100),
                then_bb: BasicBlockId(1), // level 1
                else_bb: BasicBlockId(3), // final_else
                ..
            })
        ));

        // Verify level 1 block branches to then or final_else
        let level_1_block = mir_func.blocks.get(&BasicBlockId(1)).unwrap();
        assert!(matches!(
            level_1_block.terminator,
            Some(MirInstruction::Branch {
                condition: ValueId(101),
                then_bb: BasicBlockId(2), // then
                else_bb: BasicBlockId(3), // final_else
                ..
            })
        ));

        // Verify then block has Copy and Jump to merge
        let then_block_obj = mir_func.blocks.get(&BasicBlockId(2)).unwrap();
        assert_eq!(then_block_obj.instructions.len(), 1);
        assert!(matches!(
            then_block_obj.instructions[0],
            MirInstruction::Copy { dst: d, src: s } if d == dst && s == then_val
        ));
        assert!(matches!(
            then_block_obj.terminator,
            Some(MirInstruction::Jump { target: t, .. }) if t == merge_block
        ));

        // Verify final_else block has Copy and Jump to merge
        let else_block_obj = mir_func.blocks.get(&BasicBlockId(3)).unwrap();
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
    fn test_handle_nested_if_merge_three_levels() {
        let mut mir_func = create_test_function();

        let current_block = BasicBlockId(0);

        let conds = vec![ValueId(100), ValueId(101), ValueId(102)];
        let merges = vec![
            MergePair {
                dst: ValueId(10),
                then_val: ValueId(20),
                else_val: ValueId(30),
                type_hint: None,
            },
            MergePair {
                dst: ValueId(11),
                then_val: ValueId(21),
                else_val: ValueId(31),
                type_hint: None,
            },
        ];

        let mut next_block_id = 1;
        let result = handle_nested_if_merge(
            &mut mir_func,
            current_block,
            vec![],
            &conds,
            &merges,
            &None,
            &mut next_block_id,
            mock_finalize,
        );

        assert!(result.is_ok());
        let merge_block = result.unwrap();

        // 3 levels + then + else + merge = 6 blocks total
        // Level 0: bb0, Level 1: bb1, Level 2: bb2, Then: bb3, Else: bb4, Merge: bb5
        assert_eq!(merge_block, BasicBlockId(5));
        assert_eq!(next_block_id, 6);

        // Verify level 0 branches
        let level_0 = mir_func.blocks.get(&BasicBlockId(0)).unwrap();
        assert!(matches!(
            level_0.terminator,
            Some(MirInstruction::Branch {
                condition: ValueId(100),
                then_bb: BasicBlockId(1),
                else_bb: BasicBlockId(4), // final_else
                ..
            })
        ));

        // Verify level 1 branches
        let level_1 = mir_func.blocks.get(&BasicBlockId(1)).unwrap();
        assert!(matches!(
            level_1.terminator,
            Some(MirInstruction::Branch {
                condition: ValueId(101),
                then_bb: BasicBlockId(2),
                else_bb: BasicBlockId(4), // final_else
                ..
            })
        ));

        // Verify level 2 branches to then block
        let level_2 = mir_func.blocks.get(&BasicBlockId(2)).unwrap();
        assert!(matches!(
            level_2.terminator,
            Some(MirInstruction::Branch {
                condition: ValueId(102),
                then_bb: BasicBlockId(3), // then
                else_bb: BasicBlockId(4), // final_else
                ..
            })
        ));

        // Verify then block has 2 Copy instructions
        let then_block_obj = mir_func.blocks.get(&BasicBlockId(3)).unwrap();
        assert_eq!(then_block_obj.instructions.len(), 2);
        assert!(matches!(
            then_block_obj.instructions[0],
            MirInstruction::Copy { dst: ValueId(10), src: ValueId(20) }
        ));
        assert!(matches!(
            then_block_obj.instructions[1],
            MirInstruction::Copy { dst: ValueId(11), src: ValueId(21) }
        ));

        // Verify final_else block has 2 Copy instructions
        let else_block_obj = mir_func.blocks.get(&BasicBlockId(4)).unwrap();
        assert_eq!(else_block_obj.instructions.len(), 2);
        assert!(matches!(
            else_block_obj.instructions[0],
            MirInstruction::Copy { dst: ValueId(10), src: ValueId(30) }
        ));
        assert!(matches!(
            else_block_obj.instructions[1],
            MirInstruction::Copy { dst: ValueId(11), src: ValueId(31) }
        ));
    }

    #[test]
    fn test_handle_nested_if_merge_rejects_k_next() {
        let mut mir_func = create_test_function();

        let current_block = BasicBlockId(0);

        let conds = vec![ValueId(100), ValueId(101)];
        let merges = vec![MergePair {
            dst: ValueId(10),
            then_val: ValueId(20),
            else_val: ValueId(30),
            type_hint: None,
        }];

        let k_next = Some(JoinContId(999));
        let mut next_block_id = 1;

        let result = handle_nested_if_merge(
            &mut mir_func,
            current_block,
            vec![],
            &conds,
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
    fn test_handle_nested_if_merge_rejects_empty_conds() {
        let mut mir_func = create_test_function();

        let current_block = BasicBlockId(0);

        let conds: Vec<ValueId> = vec![];
        let merges = vec![MergePair {
            dst: ValueId(10),
            then_val: ValueId(20),
            else_val: ValueId(30),
            type_hint: None,
        }];

        let mut next_block_id = 1;

        let result = handle_nested_if_merge(
            &mut mir_func,
            current_block,
            vec![],
            &conds,
            &merges,
            &None,
            &mut next_block_id,
            mock_finalize,
        );

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("conds must not be empty"));
    }
}
