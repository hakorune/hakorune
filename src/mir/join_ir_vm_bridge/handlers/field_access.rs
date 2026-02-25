//! Field Access Handler
//!
//! Phase 260 P0.2: Extracted from joinir_block_converter.rs
//! Handles JoinIR FieldAccess instruction conversion.

use crate::mir::join_ir_vm_bridge::JoinIrVmBridgeError;
use crate::mir::{Callee, EffectMask, MirInstruction, ValueId};

/// Handle JoinIR FieldAccess instruction
///
/// Converts field access to canonical Call(Method) getter pattern (Phase 51).
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
    // Phase 51: FieldAccess → Call(Method) getter pattern
    let mir_inst = MirInstruction::Call {
        dst: Some(*dst),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "RuntimeDataBox".to_string(),
            method: field.to_string(),
            receiver: Some(*object),
            certainty: crate::mir::definitions::call_unified::TypeCertainty::Union,
            box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::RuntimeData,
        }),
        args: vec![],
        effects: EffectMask::PURE,
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

        if let MirInstruction::Call {
            dst: Some(inst_dst),
            func,
            callee: Some(Callee::Method {
                method,
                receiver: Some(box_val),
                ..
            }),
            args,
            effects,
        } = &instructions[0]
        {
            assert_eq!(*inst_dst, ValueId(100));
            assert_eq!(*box_val, ValueId(200));
            assert_eq!(*func, ValueId::INVALID);
            assert_eq!(method, "my_field");
            assert!(args.is_empty());
            assert_eq!(*effects, EffectMask::PURE);
        } else {
            panic!("Expected Call(Method) instruction");
        }
    }
}
