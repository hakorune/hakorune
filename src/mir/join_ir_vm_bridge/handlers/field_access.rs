//! Field Access Handler
//!
//! Phase 260 P0.2: Extracted from joinir_block_converter.rs
//! Handles JoinIR FieldAccess instruction conversion.

use crate::mir::join_ir_vm_bridge::JoinIrVmBridgeError;
use crate::mir::{MirInstruction, ValueId};

/// Handle JoinIR FieldAccess instruction
///
/// Converts field access to canonical FieldGet pattern.
///
/// # Arguments
///
/// * `instructions` - Target instruction vector to append to
/// * `dst` - Destination ValueId for field value
/// * `object` - Object box ValueId
/// * `field` - Field name (used as method name)
pub fn handle_field_access(
    instructions: &mut Vec<MirInstruction>,
    dst: &ValueId,
    object: &ValueId,
    field: &str,
) -> Result<(), JoinIrVmBridgeError> {
    let mir_inst = MirInstruction::FieldGet {
        dst: *dst,
        base: *object,
        field: field.to_string(),
        declared_type: None,
    };
    instructions.push(mir_inst);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_field_access() {
        let mut instructions = vec![];
        let dst = ValueId(100);
        let object = ValueId(200);

        let result = handle_field_access(&mut instructions, &dst, &object, "my_field");

        assert!(result.is_ok());
        assert_eq!(instructions.len(), 1);

        if let MirInstruction::FieldGet {
            dst: inst_dst,
            base: box_val,
            field: method,
            declared_type,
        } = &instructions[0]
        {
            assert_eq!(*inst_dst, ValueId(100));
            assert_eq!(*box_val, ValueId(200));
            assert_eq!(method, "my_field");
            assert!(declared_type.is_none());
        } else {
            panic!("Expected FieldGet instruction");
        }
    }
}
