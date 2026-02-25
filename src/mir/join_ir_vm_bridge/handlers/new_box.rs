//! New Box Handler
//!
//! Phase 260 P0.2: Extracted from joinir_block_converter.rs
//! Handles JoinIR NewBox instruction conversion.

use crate::mir::join_ir_vm_bridge::JoinIrVmBridgeError;
use crate::mir::{MirInstruction, MirType, ValueId};

/// Handle JoinIR NewBox instruction
///
/// Creates a NewBox MIR instruction for box instantiation.
///
/// # Arguments
///
/// * `instructions` - Target instruction vector to append to
/// * `dst` - Destination ValueId for new box instance
/// * `box_name` - Box type name
/// * `args` - Constructor arguments
/// * `type_hint` - Optional type hint (currently unused - Phase 65-2-B TODO)
pub fn handle_new_box(
    instructions: &mut Vec<MirInstruction>,
    dst: &ValueId,
    box_name: &str,
    args: &[ValueId],
    type_hint: &Option<MirType>,
) -> Result<(), JoinIrVmBridgeError> {
    let mir_inst = MirInstruction::NewBox {
        dst: *dst,
        box_type: box_name.to_string(),
        args: args.to_vec(),
    };
    instructions.push(mir_inst);

    // Phase 65-2-B: TODO: type_hint を value_types に記録
    let _ = type_hint;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_new_box() {
        let mut instructions = vec![];
        let dst = ValueId(100);
        let args = vec![ValueId(1), ValueId(2)];

        let result = handle_new_box(&mut instructions, &dst, "MyBox", &args, &None);

        assert!(result.is_ok());
        assert_eq!(instructions.len(), 1);

        if let MirInstruction::NewBox {
            dst: inst_dst,
            box_type,
            args: inst_args,
        } = &instructions[0]
        {
            assert_eq!(*inst_dst, ValueId(100));
            assert_eq!(box_type, "MyBox");
            assert_eq!(inst_args, &[ValueId(1), ValueId(2)]);
        } else {
            panic!("Expected NewBox instruction");
        }
    }

    #[test]
    fn test_handle_new_box_no_args() {
        let mut instructions = vec![];
        let dst = ValueId(200);

        let result = handle_new_box(
            &mut instructions,
            &dst,
            "EmptyBox",
            &[],
            &Some(MirType::String),
        );

        assert!(result.is_ok());
        assert_eq!(instructions.len(), 1);

        if let MirInstruction::NewBox { args, .. } = &instructions[0] {
            assert!(args.is_empty());
        } else {
            panic!("Expected NewBox instruction");
        }
    }
}
