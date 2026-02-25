//! Call Handler - JoinIR Call instruction to MIR conversion
//!
//! Phase 260 P0.3: Extracted from joinir_block_converter.rs (lines 367-454)
//!
//! ## Responsibility
//!
//! Converts JoinIR Call instructions to MIR Call + Return sequences.
//! Handles both tail calls and non-tail calls with proper block finalization.
//!
//! ## Key Design Points
//!
//! ### 1. Tail Call vs Non-Tail Call
//!
//! - **Non-tail call** (dst = Some): Emit Call instruction only
//! - **Tail call** (dst = None): Emit Call + Return sequence
//!
//! ### 2. Phase 131 P2: Stable Function Name ValueId (Module-Global SSOT)
//!
//! The merge pipeline relies on `Const(String("join_func_N"))` to detect tail calls.
//! The ValueId used for that const MUST be stable across *all* functions in the module.
//!
//! ```text
//! FUNC_NAME_ID_BASE = 90000
//! func_name_id = ValueId(90000 + func.0)
//! ```
//!
//! **Collision Avoidance**: Must not conflict with `call_result_id = ValueId(99991)`
//!
//! ### 3. Phase 131 P2: Legacy Jump-Args Metadata for Tail Calls
//!
//! The merge pipeline recovers carrier/env values from the legacy jump-args path
//! (via BasicBlock APIs) when converting Return → Jump to the exit block.
//! Without this, tail-call blocks look like "no args", forcing fallbacks that can
//! produce undefined ValueIds in DirectValue mode.
//!
//! ```rust
//! block.set_return_edge_args(EdgeArgs { layout: JumpArgsLayout::CarriersOnly, values: args.to_vec() });
//! ```
//!
//! ### 4. Phase 256 P1.8: Function Name Resolution
//!
//! When `func_name_map` is provided, use actual function names instead of "join_func_N".
//! This ensures proper function resolution in the merge pipeline.
//!
//! ## Example Conversions
//!
//! ### Non-Tail Call
//!
//! ```text
//! JoinIR:
//!   Call(func=JoinFuncId(5), args=[v1, v2], dst=Some(v10))
//!
//! MIR:
//!   %90005 = Const { value: "join_func_5" }
//!   %10 = Call { func: %90005, args: [v1, v2] }
//! ```
//!
//! ### Tail Call
//!
//! ```text
//! JoinIR:
//!   Call(func=JoinFuncId(5), args=[v1, v2], dst=None)
//!
//! MIR:
//!   %90005 = Const { value: "join_func_5" }
//!   %99991 = Call { func: %90005, args: [v1, v2] }
//!   Return %99991
//!
//! Block Metadata:
//!   legacy_jump_args = [v1, v2]
//!   jump_args_layout = CarriersOnly
//! ```

use crate::mir::join_ir::{JoinFuncId, JoinContId};
use crate::mir::join_ir::lowering::inline_boundary::JumpArgsLayout;
use crate::mir::{BasicBlockId, MirFunction, MirInstruction, ValueId};
use std::collections::BTreeMap;

use super::super::{join_func_name, JoinIrVmBridgeError};
use super::super::call_generator;

/// Handle JoinIR Call instruction conversion
///
/// Converts a JoinIR Call to MIR Call instruction(s), handling both tail calls
/// and non-tail calls. For tail calls, emits Call + Return and sets legacy jump-args metadata.
///
/// # Arguments
///
/// * `mir_func` - Target MIR function to modify
/// * `current_block_id` - ID of the block being built
/// * `current_instructions` - Instruction vector to append to (consumed for tail calls)
/// * `func_name_map` - Optional map from JoinFuncId to actual function names
/// * `func` - JoinIR function ID to call
/// * `args` - Call arguments
/// * `dst` - Optional destination for result (None = tail call)
/// * `k_next` - Continuation (must be None, error otherwise)
/// * `finalize_fn` - Function to finalize blocks (called for tail calls)
///
/// # Errors
///
/// - Returns error if k_next is Some (not yet supported)
/// - Returns error if func_name_id collides with call_result_id (99991)
///
/// # Example
///
/// ```ignore
/// let mut instructions = vec![];
/// handle_call(
///     &mut mir_func,
///     BasicBlockId(0),
///     instructions,
///     &func_name_map,
///     &JoinFuncId(5),
///     &[ValueId(1), ValueId(2)],
///     &Some(ValueId(10)),  // non-tail
///     &None,               // no k_next
///     finalize_block,
/// )?;
/// ```
pub fn handle_call<F>(
    mir_func: &mut MirFunction,
    current_block_id: BasicBlockId,
    mut current_instructions: Vec<MirInstruction>,
    func_name_map: &Option<BTreeMap<JoinFuncId, String>>,
    func: &JoinFuncId,
    args: &[ValueId],
    dst: &Option<ValueId>,
    k_next: &Option<JoinContId>,
    finalize_fn: F,
) -> Result<Vec<MirInstruction>, JoinIrVmBridgeError>
where
    F: FnOnce(&mut MirFunction, BasicBlockId, Vec<MirInstruction>, MirInstruction),
{
    // Phase 30.x: Call conversion
    if k_next.is_some() {
        return Err(JoinIrVmBridgeError::new(
            "Call with k_next is not yet supported".to_string(),
        ));
    }

    // Phase 256 P1.8: Use actual function name if available
    let func_name = if let Some(ref map) = func_name_map {
        map.get(func).cloned().unwrap_or_else(|| join_func_name(*func))
    } else {
        join_func_name(*func)
    };

    // Phase 131 P2: Stable function name ValueId (module-global SSOT)
    //
    // The merge pipeline relies on `Const(String("join_func_N"))` to detect tail calls.
    // The ValueId used for that const MUST be stable across *all* functions in the module.
    //
    // IMPORTANT: avoid collisions with `call_result_id = ValueId(99991)`.
    const FUNC_NAME_ID_BASE: u32 = 90000;
    let func_name_id = ValueId(FUNC_NAME_ID_BASE + func.0);
    if func_name_id == ValueId(99991) {
        return Err(JoinIrVmBridgeError::new(
            "[joinir_block] func_name_id collided with call_result_id (99991)".to_string(),
        ));
    }

    match dst {
        Some(result_dst) => {
            // Non-tail call: use call_generator to emit Const + Call
            call_generator::emit_call_pair(
                &mut current_instructions,
                func_name_id,
                *result_dst,
                &func_name,
                args,
            );
            Ok(current_instructions)
        }
        None => {
            // Tail call: emit Call + Return
            let call_result_id = ValueId(99991);
            call_generator::emit_call_pair(
                &mut current_instructions,
                func_name_id,
                call_result_id,
                &func_name,
                args,
            );

            // Phase 131 P2: Preserve tail-call args as legacy jump-args metadata (for exit wiring)
            //
            // The merge pipeline recovers carrier/env values from the legacy jump-args path
            // (via BasicBlock APIs) when converting Return → Jump to the exit block.
            // Without this, tail-call blocks look like "no args", forcing fallbacks that can
            // produce undefined ValueIds in DirectValue mode.
            let terminator = MirInstruction::Return {
                value: Some(call_result_id),
            };
            finalize_fn(
                mir_func,
                current_block_id,
                current_instructions,
                terminator,
            );
            if let Some(block) = mir_func.blocks.get_mut(&current_block_id) {
                if matches!(block.terminator, Some(MirInstruction::Return { .. })) && block.return_env().is_none() {
                    block.set_return_env(crate::mir::EdgeArgs {
                        layout: JumpArgsLayout::CarriersOnly,
                        values: args.to_vec(),
                    });
                }
            }
            Ok(vec![]) // Instructions consumed by finalize
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{ConstValue, EffectMask, MirType, FunctionSignature};
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
        // Entry block is already created by MirFunction::new
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
            block.instruction_spans = vec![crate::ast::Span::unknown(); block.instructions.len()];
            block.terminator = Some(terminator);
        }
    }

    #[test]
    fn test_handle_call_non_tail() {
        let mut mir_func = create_test_mir_func();
        let current_block_id = BasicBlockId(0);
        let current_instructions = vec![];
        let func_name_map = None;

        let result = handle_call(
            &mut mir_func,
            current_block_id,
            current_instructions,
            &func_name_map,
            &JoinFuncId(5),
            &[ValueId(1), ValueId(2)],
            &Some(ValueId(10)), // non-tail call
            &None,
            mock_finalize,
        );

        assert!(result.is_ok());
        let instructions = result.unwrap();
        assert_eq!(instructions.len(), 2);

        // Check Const instruction
        match &instructions[0] {
            MirInstruction::Const { dst, value } => {
                assert_eq!(*dst, ValueId(90005)); // FUNC_NAME_ID_BASE + 5
                if let ConstValue::String(s) = value {
                    assert_eq!(s, "join_func_5");
                } else {
                    panic!("Expected ConstValue::String");
                }
            }
            _ => panic!("Expected Const instruction"),
        }

        // Check Call instruction
        match &instructions[1] {
            MirInstruction::Call {
                dst,
                func,
                callee,
                args,
                effects,
            } => {
                assert_eq!(*dst, Some(ValueId(10)));
                assert_eq!(*func, ValueId(90005));
                assert_eq!(
                    *callee,
                    Some(crate::mir::definitions::Callee::Global(
                        "join_func_5".to_string()
                    ))
                );
                assert_eq!(args, &[ValueId(1), ValueId(2)]);
                assert_eq!(*effects, EffectMask::PURE);
            }
            _ => panic!("Expected Call instruction"),
        }

        // Block should not have terminator (non-tail call)
        let block = mir_func.blocks.get(&current_block_id).unwrap();
        assert!(block.terminator.is_none());
    }

    #[test]
    fn test_handle_call_tail() {
        let mut mir_func = create_test_mir_func();
        let current_block_id = BasicBlockId(0);
        let current_instructions = vec![];
        let func_name_map = None;

        let result = handle_call(
            &mut mir_func,
            current_block_id,
            current_instructions,
            &func_name_map,
            &JoinFuncId(3),
            &[ValueId(10), ValueId(20)],
            &None, // tail call
            &None,
            mock_finalize,
        );

        assert!(result.is_ok());
        let instructions = result.unwrap();
        assert_eq!(instructions.len(), 0); // Instructions consumed by finalize

        // Check block has been finalized
        let block = mir_func.blocks.get(&current_block_id).unwrap();
        assert_eq!(block.instructions.len(), 2);

        // Check Const instruction in block
        match &block.instructions[0] {
            MirInstruction::Const { dst, value } => {
                assert_eq!(*dst, ValueId(90003)); // FUNC_NAME_ID_BASE + 3
                if let ConstValue::String(s) = value {
                    assert_eq!(s, "join_func_3");
                } else {
                    panic!("Expected ConstValue::String");
                }
            }
            _ => panic!("Expected Const instruction"),
        }

        // Check Call instruction in block
        match &block.instructions[1] {
            MirInstruction::Call {
                dst,
                func,
                args,
                ..
            } => {
                assert_eq!(*dst, Some(ValueId(99991)));
                assert_eq!(*func, ValueId(90003));
                assert_eq!(args, &[ValueId(10), ValueId(20)]);
            }
            _ => panic!("Expected Call instruction"),
        }

        // Check Return terminator
        match &block.terminator {
            Some(MirInstruction::Return { value }) => {
                assert_eq!(*value, Some(ValueId(99991)));
            }
            _ => panic!("Expected Return terminator"),
        }

        // Check return environment metadata
        let env = block.return_env().expect("Block should have return_env");
        assert_eq!(env.values, vec![ValueId(10), ValueId(20)]);
        assert_eq!(env.layout, JumpArgsLayout::CarriersOnly);
    }

    #[test]
    fn test_handle_call_rejects_k_next() {
        let mut mir_func = create_test_mir_func();
        let current_block_id = BasicBlockId(0);
        let current_instructions = vec![];
        let func_name_map = None;

        let result = handle_call(
            &mut mir_func,
            current_block_id,
            current_instructions,
            &func_name_map,
            &JoinFuncId(1),
            &[],
            &Some(ValueId(5)),
            &Some(JoinContId(1)), // k_next provided
            mock_finalize,
        );

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("k_next is not yet supported"));
    }

    #[test]
    fn test_handle_call_with_func_name_map() {
        let mut mir_func = create_test_mir_func();
        let current_block_id = BasicBlockId(0);
        let current_instructions = vec![];

        // Create func_name_map
        let mut func_name_map = BTreeMap::new();
        func_name_map.insert(JoinFuncId(7), "my_custom_func".to_string());

        let result = handle_call(
            &mut mir_func,
            current_block_id,
            current_instructions,
            &Some(func_name_map),
            &JoinFuncId(7),
            &[ValueId(1)],
            &Some(ValueId(100)),
            &None,
            mock_finalize,
        );

        assert!(result.is_ok());
        let instructions = result.unwrap();

        // Check that custom function name is used
        match &instructions[0] {
            MirInstruction::Const { value, .. } => {
                if let ConstValue::String(s) = value {
                    assert_eq!(s, "my_custom_func");
                } else {
                    panic!("Expected ConstValue::String");
                }
            }
            _ => panic!("Expected Const instruction"),
        }
    }

    #[test]
    fn test_handle_call_collision_detection() {
        let mut mir_func = create_test_mir_func();
        let current_block_id = BasicBlockId(0);
        let current_instructions = vec![];
        let func_name_map = None;

        // JoinFuncId that would collide: 90000 + x = 99991 => x = 9991
        let result = handle_call(
            &mut mir_func,
            current_block_id,
            current_instructions,
            &func_name_map,
            &JoinFuncId(9991),
            &[],
            &Some(ValueId(50)),
            &None,
            mock_finalize,
        );

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("collided with call_result_id"));
    }
}
