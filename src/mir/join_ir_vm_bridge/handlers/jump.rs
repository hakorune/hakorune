//! Jump Handler - JoinIR Jump instruction to MIR conversion
//!
//! Phase 260 P0.3: Extracted from joinir_block_converter.rs (lines 456-579)
//!
//! ## Responsibility
//!
//! Converts JoinIR Jump instructions to MIR tail call sequences.
//! Handles both conditional and unconditional jumps to continuation functions.
//!
//! ## Key Design Points
//!
//! ### 1. Jump → Tail Call Transformation (Phase 256 P1.9)
//!
//! JoinIR Jump instructions are converted to tail calls to continuation functions:
//!
//! ```text
//! JoinIR:
//!   Jump(cont=JoinContId(5), args=[v1, v2], cond=None)
//!
//! MIR:
//!   %91005 = Const { value: "join_func_5" }
//!   %99992 = Call { func: %91005, args: [v1, v2] }
//!   Return %99992
//! ```
//!
//! ### 2. Conditional vs Unconditional Jumps
//!
//! - **Unconditional** (cond = None): Direct tail call in current block
//! - **Conditional** (cond = Some): Branch to exit block (tail call) vs continue block
//!
//! ### 3. Stable ValueId Allocation
//!
//! ```text
//! JUMP_FUNC_NAME_ID_BASE = 91000  (distinct from handle_call's 90000)
//! func_name_id = ValueId(91000 + cont.0)
//! call_result_id = ValueId(99992)  (distinct from handle_call's 99991)
//! ```
//!
//! This ensures no collision between Call and Jump instructions.
//!
//! ### 4. Legacy Jump-Args Metadata (Phase 246-EX)
//!
//! Jump args are preserved in block metadata for exit PHI construction:
//!
//! ```rust
//! block.set_return_edge_args(EdgeArgs { layout: JumpArgsLayout::CarriersOnly, values: args.to_vec() });
//! ```
//!
//! This metadata is used by the merge pipeline to wire up exit PHI nodes correctly.
//!
//! ## Example Conversions
//!
//! ### Unconditional Jump
//!
//! ```text
//! JoinIR:
//!   Jump(cont=JoinContId(3), args=[v10, v20], cond=None)
//!
//! MIR (appended to current block):
//!   %91003 = Const { value: "join_func_3" }
//!   %99992 = Call { func: %91003, args: [v10, v20] }
//!   Return %99992
//!
//! Block Metadata:
//!   legacy_jump_args = [v10, v20]
//!   jump_args_layout = CarriersOnly
//! ```
//!
//! ### Conditional Jump
//!
//! ```text
//! JoinIR:
//!   Jump(cont=JoinContId(7), args=[v1, v2], cond=Some(v100))
//!
//! MIR:
//!   Current Block:
//!     Branch { condition: v100, then_bb: exit_block, else_bb: continue_block }
//!
//!   Exit Block (tail call):
//!     %91007 = Const { value: "join_func_7" }
//!     %99992 = Call { func: %91007, args: [v1, v2] }
//!     Return %99992
//!
//!   Continue Block:
//!     (empty, ready for next instruction)
//!
//! Exit Block Metadata:
//!   legacy_jump_args = [v1, v2]
//!   jump_args_layout = CarriersOnly
//! ```

use crate::mir::join_ir::lowering::inline_boundary::JumpArgsLayout;
use crate::mir::join_ir::{JoinContId, JoinFuncId};
use crate::mir::{BasicBlock, BasicBlockId, MirFunction, MirInstruction, ValueId};
#[cfg(debug_assertions)]
use crate::runtime::get_global_ring0;
use std::collections::BTreeMap;

use super::super::call_generator;
use super::super::{join_func_name, JoinIrVmBridgeError};

/// Handle JoinIR Jump instruction conversion
///
/// Converts a JoinIR Jump to MIR tail call to continuation function.
/// Handles both conditional jumps (Branch + tail call) and unconditional jumps (direct tail call).
///
/// # Arguments
///
/// * `mir_func` - Target MIR function to modify
/// * `allocator` - Block allocator for creating new blocks (conditional case)
/// * `current_block_id` - ID of the block being built
/// * `current_instructions` - Instruction vector to append to (consumed for unconditional jumps)
/// * `func_name_map` - Optional map from JoinFuncId to actual function names
/// * `cont` - Continuation ID (target of jump)
/// * `args` - Jump arguments (passed to continuation)
/// * `cond` - Optional condition variable (Some = conditional jump, None = unconditional)
/// * `finalize_fn` - Function to finalize blocks (called for unconditional jumps)
///
/// # Returns
///
/// New current block ID. For conditional jumps, returns the continue block.
/// For unconditional jumps, returns the same block ID (since block is finalized).
///
/// # Errors
///
/// Currently does not return errors, but uses Result for consistency with other handlers.
///
/// # Example
///
/// ```ignore
/// // Unconditional jump
/// let new_block_id = handle_jump(
///     &mut mir_func,
///     &mut allocator,
///     BasicBlockId(0),
///     instructions,
///     &func_name_map,
///     &JoinContId(5),
///     &[ValueId(1), ValueId(2)],
///     &None,  // unconditional
///     finalize_block,
/// )?;
///
/// // Conditional jump
/// let new_block_id = handle_jump(
///     &mut mir_func,
///     &mut allocator,
///     BasicBlockId(0),
///     instructions,
///     &func_name_map,
///     &JoinContId(7),
///     &[ValueId(10)],
///     &Some(ValueId(100)),  // conditional on v100
///     finalize_block,
/// )?;
/// ```
pub fn handle_jump<F>(
    mir_func: &mut MirFunction,
    allocator: &mut super::super::block_allocator::BlockAllocator,
    current_block_id: BasicBlockId,
    mut current_instructions: Vec<MirInstruction>,
    func_name_map: &Option<BTreeMap<JoinFuncId, String>>,
    cont: &JoinContId,
    args: &[ValueId],
    cond: &Option<ValueId>,
    finalize_fn: F,
) -> Result<BasicBlockId, JoinIrVmBridgeError>
where
    F: FnOnce(&mut MirFunction, BasicBlockId, Vec<MirInstruction>, MirInstruction),
{
    // Phase 256 P1.9: Jump → tail call to continuation function
    // Previously was just `ret args[0]`, now generates `call cont(args...); ret result`
    #[cfg(debug_assertions)]
    if crate::config::env::joinir_test_debug_enabled() {
        get_global_ring0().log.debug(&format!(
            "[joinir_block] Converting Jump to tail call: cont={:?}, args={:?}, cond={:?}",
            cont, args, cond
        ));
    }

    // Get continuation function name
    let cont_name = get_continuation_name(func_name_map, cont);

    // Phase 256 P1.9: Use distinct ValueIds for Jump tail call
    // FUNC_NAME_ID_BASE for call targets, 99992 for Jump result (distinct from 99991 in handle_call)
    const JUMP_FUNC_NAME_ID_BASE: u32 = 91000; // Different from handle_call's 90000
    let func_name_id = ValueId(JUMP_FUNC_NAME_ID_BASE + cont.0);
    let call_result_id = ValueId(99992); // Distinct from handle_call's 99991

    match cond {
        Some(cond_var) => {
            // Conditional jump → Branch + tail call to continuation
            let (exit_block_id, continue_block_id) = allocator.allocate_two();

            let branch_terminator = MirInstruction::Branch {
                condition: *cond_var,
                then_bb: exit_block_id,
                else_bb: continue_block_id,
                then_edge_args: None,
                else_edge_args: None,
            };

            finalize_fn(
                mir_func,
                current_block_id,
                current_instructions,
                branch_terminator,
            );

            // Exit block: tail call to continuation function
            let mut exit_block = BasicBlock::new(exit_block_id);

            // Phase 256 P1.9: Generate tail call to continuation
            call_generator::emit_call_pair_with_spans(
                &mut exit_block.instructions,
                &mut exit_block.instruction_spans,
                func_name_id,
                call_result_id,
                &cont_name,
                args,
            );
            exit_block.set_terminator(MirInstruction::Return {
                value: Some(call_result_id),
            });
            exit_block.set_return_env(crate::mir::EdgeArgs {
                layout: JumpArgsLayout::CarriersOnly,
                values: args.to_vec(),
            });
            mir_func.blocks.insert(exit_block_id, exit_block);

            // Continue block
            let continue_block = BasicBlock::new(continue_block_id);
            mir_func.blocks.insert(continue_block_id, continue_block);

            Ok(continue_block_id)
        }
        None => {
            // Unconditional jump → tail call to continuation
            // Finalize current block with tail call
            call_generator::emit_call_pair(
                &mut current_instructions,
                func_name_id,
                call_result_id,
                &cont_name,
                args,
            );

            let return_terminator = MirInstruction::Return {
                value: Some(call_result_id),
            };

            finalize_fn(
                mir_func,
                current_block_id,
                current_instructions,
                return_terminator,
            );
            if let Some(block) = mir_func.blocks.get_mut(&current_block_id) {
                if matches!(block.terminator, Some(MirInstruction::Return { .. }))
                    && block.return_env().is_none()
                {
                    block.set_return_env(crate::mir::EdgeArgs {
                        layout: JumpArgsLayout::CarriersOnly,
                        values: args.to_vec(),
                    });
                }
            }

            Ok(current_block_id)
        }
    }
}

/// Phase 256 P1.9: Get continuation function name from func_name_map
///
/// Resolves a continuation ID to its function name, using the provided
/// func_name_map if available, otherwise falling back to join_func_name.
///
/// # Arguments
///
/// * `func_name_map` - Optional map from JoinFuncId to actual function names
/// * `cont` - Continuation ID to resolve
///
/// # Returns
///
/// Function name as a String
///
/// # Note
///
/// JoinContId.0 == JoinFuncId.0 (same underlying ID via as_cont())
fn get_continuation_name(
    func_name_map: &Option<BTreeMap<JoinFuncId, String>>,
    cont: &JoinContId,
) -> String {
    // JoinContId.0 == JoinFuncId.0 (same underlying ID via as_cont())
    if let Some(ref map) = func_name_map {
        if let Some(name) = map.get(&JoinFuncId(cont.0)) {
            return name.clone();
        }
    }
    // Fallback: use join_func_name()
    join_func_name(JoinFuncId(cont.0))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Span;
    use crate::mir::join_ir_vm_bridge::block_allocator::BlockAllocator;
    use crate::mir::{ConstValue, EffectMask, FunctionSignature, MirType};
    use std::collections::BTreeMap;

    /// Helper: Create empty MirFunction for testing
    fn create_test_mir_func() -> MirFunction {
        let signature = FunctionSignature {
            name: "test_func".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let func = MirFunction::new(signature, BasicBlockId(0));
        func
    }

    /// Mock finalize function that captures the block finalization
    fn mock_finalize(
        mir_func: &mut MirFunction,
        block_id: BasicBlockId,
        instructions: Vec<MirInstruction>,
        terminator: MirInstruction,
    ) {
        if let Some(block) = mir_func.blocks.get_mut(&block_id) {
            block.instructions = instructions;
            block.instruction_spans = vec![Span::unknown(); block.instructions.len()];
            block.terminator = Some(terminator);
        }
    }

    #[test]
    fn test_handle_jump_unconditional() {
        let mut mir_func = create_test_mir_func();
        let mut allocator = BlockAllocator::new(1);
        let current_block_id = BasicBlockId(0);
        let current_instructions = vec![];
        let func_name_map = None;

        let result = handle_jump(
            &mut mir_func,
            &mut allocator,
            current_block_id,
            current_instructions,
            &func_name_map,
            &JoinContId(3),
            &[ValueId(10), ValueId(20)],
            &None, // unconditional
            mock_finalize,
        );

        assert!(result.is_ok());
        let new_block_id = result.unwrap();
        assert_eq!(new_block_id, current_block_id); // Same block for unconditional

        // Check block has been finalized
        let block = mir_func.blocks.get(&current_block_id).unwrap();
        assert_eq!(block.instructions.len(), 2);

        // Check Const instruction
        match &block.instructions[0] {
            MirInstruction::Const { dst, value } => {
                assert_eq!(*dst, ValueId(91003)); // JUMP_FUNC_NAME_ID_BASE + 3
                if let ConstValue::String(s) = value {
                    assert_eq!(s, "join_func_3");
                } else {
                    panic!("Expected ConstValue::String");
                }
            }
            _ => panic!("Expected Const instruction"),
        }

        // Check Call instruction
        match &block.instructions[1] {
            MirInstruction::Call {
                dst, func, args, ..
            } => {
                assert_eq!(*dst, Some(ValueId(99992)));
                assert_eq!(*func, ValueId(91003));
                assert_eq!(args, &[ValueId(10), ValueId(20)]);
            }
            _ => panic!("Expected Call instruction"),
        }

        // Check Return terminator
        match &block.terminator {
            Some(MirInstruction::Return { value }) => {
                assert_eq!(*value, Some(ValueId(99992)));
            }
            _ => panic!("Expected Return terminator"),
        }

        // Check return environment metadata
        let env = block.return_env().expect("Block should have return_env");
        assert_eq!(env.values, vec![ValueId(10), ValueId(20)]);
        assert_eq!(env.layout, JumpArgsLayout::CarriersOnly);
    }

    #[test]
    fn test_handle_jump_conditional() {
        let mut mir_func = create_test_mir_func();
        let mut allocator = BlockAllocator::new(1);
        let current_block_id = BasicBlockId(0);
        let current_instructions = vec![];
        let func_name_map = None;

        let result = handle_jump(
            &mut mir_func,
            &mut allocator,
            current_block_id,
            current_instructions,
            &func_name_map,
            &JoinContId(7),
            &[ValueId(1), ValueId(2)],
            &Some(ValueId(100)), // conditional on v100
            mock_finalize,
        );

        assert!(result.is_ok());
        let new_block_id = result.unwrap();
        assert_eq!(new_block_id, BasicBlockId(2)); // Continue block

        // Check current block has Branch terminator
        let current_block = mir_func.blocks.get(&current_block_id).unwrap();
        match &current_block.terminator {
            Some(MirInstruction::Branch {
                condition,
                then_bb,
                else_bb,
                ..
            }) => {
                assert_eq!(*condition, ValueId(100));
                assert_eq!(*then_bb, BasicBlockId(1)); // exit block
                assert_eq!(*else_bb, BasicBlockId(2)); // continue block
            }
            _ => panic!("Expected Branch terminator"),
        }

        // Check exit block (should have tail call)
        let exit_block = mir_func.blocks.get(&BasicBlockId(1)).unwrap();
        assert_eq!(exit_block.instructions.len(), 2);

        // Check Const instruction in exit block
        match &exit_block.instructions[0] {
            MirInstruction::Const { dst, value } => {
                assert_eq!(*dst, ValueId(91007)); // JUMP_FUNC_NAME_ID_BASE + 7
                if let ConstValue::String(s) = value {
                    assert_eq!(s, "join_func_7");
                } else {
                    panic!("Expected ConstValue::String");
                }
            }
            _ => panic!("Expected Const instruction"),
        }

        // Check Call instruction in exit block
        match &exit_block.instructions[1] {
            MirInstruction::Call {
                dst, func, args, ..
            } => {
                assert_eq!(*dst, Some(ValueId(99992)));
                assert_eq!(*func, ValueId(91007));
                assert_eq!(args, &[ValueId(1), ValueId(2)]);
            }
            _ => panic!("Expected Call instruction"),
        }

        // Check Return terminator in exit block
        match &exit_block.terminator {
            Some(MirInstruction::Return { value }) => {
                assert_eq!(*value, Some(ValueId(99992)));
            }
            _ => panic!("Expected Return terminator"),
        }

        // Check return environment metadata in exit block
        let env = exit_block
            .return_env()
            .expect("Exit block should have return_env");
        assert_eq!(env.values, vec![ValueId(1), ValueId(2)]);
        assert_eq!(env.layout, JumpArgsLayout::CarriersOnly);

        // Check continue block exists and is empty
        let continue_block = mir_func.blocks.get(&BasicBlockId(2)).unwrap();
        assert_eq!(continue_block.instructions.len(), 0);
        assert!(continue_block.terminator.is_none());
    }

    #[test]
    fn test_get_continuation_name_with_map() {
        let mut func_name_map = BTreeMap::new();
        func_name_map.insert(JoinFuncId(5), "my_continuation".to_string());

        let name = get_continuation_name(&Some(func_name_map), &JoinContId(5));
        assert_eq!(name, "my_continuation");
    }

    #[test]
    fn test_get_continuation_name_without_map() {
        let name = get_continuation_name(&None, &JoinContId(3));
        assert_eq!(name, "join_func_3");
    }

    #[test]
    fn test_get_continuation_name_fallback() {
        let mut func_name_map = BTreeMap::new();
        func_name_map.insert(JoinFuncId(1), "other_func".to_string());

        // Request cont 5, but only cont 1 is in map
        let name = get_continuation_name(&Some(func_name_map), &JoinContId(5));
        assert_eq!(name, "join_func_5"); // Should fallback
    }
}
