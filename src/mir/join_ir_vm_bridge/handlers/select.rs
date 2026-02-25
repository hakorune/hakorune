//! Select Handler
//!
//! Phase 260 P0.2: Extracted from joinir_block_converter.rs
//! Handles JoinIR Select instruction conversion.

use crate::mir::join_ir_vm_bridge::JoinIrVmBridgeError;
use crate::mir::{MirInstruction, MirType, ValueId};
use crate::runtime::get_global_ring0;

/// Handle JoinIR Select instruction
///
/// Converts Select to MirInstruction::Select (direct instruction, not control flow expansion).
/// Phase 256 P1.5: Select does not expand to branches/PHI.
///
/// # Arguments
///
/// * `instructions` - Target instruction vector to append to
/// * `dst` - Destination ValueId for selected value
/// * `cond` - Condition ValueId (boolean)
/// * `then_val` - Value if condition is true
/// * `else_val` - Value if condition is false
/// * `type_hint` - Optional type hint (currently unused)
/// * `debug` - Optional debug flag for logging
pub fn handle_select(
    instructions: &mut Vec<MirInstruction>,
    dst: &ValueId,
    cond: &ValueId,
    then_val: &ValueId,
    else_val: &ValueId,
    type_hint: &Option<MirType>,
    debug: bool,
) -> Result<(), JoinIrVmBridgeError> {
    // Phase 256 P1.5: Select → MirInstruction::Select (direct instruction, not control flow expansion)
    if debug {
        get_global_ring0().log.debug(&format!(
            "[joinir_block] Converting Select: dst={:?}, cond={:?}, then_val={:?}, else_val={:?}",
            dst, cond, then_val, else_val
        ));
    }

    // Emit Select instruction directly (no branch/phi expansion)
    let select_inst = MirInstruction::Select {
        dst: *dst,
        cond: *cond,
        then_val: *then_val,
        else_val: *else_val,
    };

    instructions.push(select_inst);

    let _ = type_hint;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_select() {
        let mut instructions = vec![];
        let dst = ValueId(100);
        let cond = ValueId(200);
        let then_val = ValueId(300);
        let else_val = ValueId(400);

        let result = handle_select(
            &mut instructions,
            &dst,
            &cond,
            &then_val,
            &else_val,
            &None,
            false,
        );

        assert!(result.is_ok());
        assert_eq!(instructions.len(), 1);

        if let MirInstruction::Select {
            dst: inst_dst,
            cond: inst_cond,
            then_val: inst_then,
            else_val: inst_else,
        } = &instructions[0]
        {
            assert_eq!(*inst_dst, ValueId(100));
            assert_eq!(*inst_cond, ValueId(200));
            assert_eq!(*inst_then, ValueId(300));
            assert_eq!(*inst_else, ValueId(400));
        } else {
            panic!("Expected Select instruction");
        }
    }

    #[test]
    fn test_handle_select_with_debug() {
        let mut instructions = vec![];
        let dst = ValueId(500);
        let cond = ValueId(600);
        let then_val = ValueId(700);
        let else_val = ValueId(800);

        // Test with debug enabled (should not panic, just print)
        let result = handle_select(
            &mut instructions,
            &dst,
            &cond,
            &then_val,
            &else_val,
            &Some(MirType::Integer),
            true,
        );

        assert!(result.is_ok());
        assert_eq!(instructions.len(), 1);
    }
}
