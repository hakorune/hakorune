//! Return Handler
//!
//! Phase 260 P0.2: Extracted from joinir_block_converter.rs
//! Handles JoinIR Ret instruction conversion.

use crate::mir::join_ir_vm_bridge::JoinIrVmBridgeError;
use crate::mir::{BasicBlockId, MirFunction, MirInstruction, ValueId};

/// Handle JoinIR Ret instruction
///
/// Creates a Return terminator and finalizes the current block.
///
/// # Arguments
///
/// * `mir_func` - Target MIR function to modify
/// * `current_block_id` - Current basic block ID
/// * `current_instructions` - Pending instructions to flush
/// * `value` - Optional return value
/// * `finalize_fn` - Block finalization function
pub fn handle_ret<F>(
    mir_func: &mut MirFunction,
    current_block_id: BasicBlockId,
    current_instructions: Vec<MirInstruction>,
    value: &Option<ValueId>,
    finalize_fn: F,
) -> Result<(), JoinIrVmBridgeError>
where
    F: FnOnce(&mut MirFunction, BasicBlockId, Vec<MirInstruction>, MirInstruction),
{
    let return_terminator = MirInstruction::Return { value: *value };
    finalize_fn(mir_func, current_block_id, current_instructions, return_terminator);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{FunctionSignature, MirType, EffectMask};

    #[test]
    fn test_handle_ret_with_value() {
        let signature = FunctionSignature {
            name: "test".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let mut mir_func = MirFunction::new(signature, BasicBlockId::new(0));
        let current_block_id = BasicBlockId::new(0);
        let current_instructions = vec![];
        let value = Some(ValueId(42));

        let mut finalized = false;
        let result = handle_ret(
            &mut mir_func,
            current_block_id,
            current_instructions,
            &value,
            |_func, _block_id, _insts, terminator| {
                finalized = true;
                if let MirInstruction::Return { value } = terminator {
                    assert_eq!(value, Some(ValueId(42)));
                } else {
                    panic!("Expected Return terminator");
                }
            },
        );

        assert!(result.is_ok());
        assert!(finalized);
    }

    #[test]
    fn test_handle_ret_without_value() {
        let signature = FunctionSignature {
            name: "test".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let mut mir_func = MirFunction::new(signature, BasicBlockId::new(0));
        let current_block_id = BasicBlockId::new(0);
        let current_instructions = vec![];
        let value = None;

        let mut finalized = false;
        let result = handle_ret(
            &mut mir_func,
            current_block_id,
            current_instructions,
            &value,
            |_func, _block_id, _insts, terminator| {
                finalized = true;
                if let MirInstruction::Return { value } = terminator {
                    assert_eq!(value, None);
                } else {
                    panic!("Expected Return terminator");
                }
            },
        );

        assert!(result.is_ok());
        assert!(finalized);
    }
}
