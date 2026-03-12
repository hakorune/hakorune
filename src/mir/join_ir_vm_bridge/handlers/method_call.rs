//! Method Call Handler
//!
//! Phase 260 P0.2: Extracted from joinir_block_converter.rs
//! Handles JoinIR MethodCall instruction conversion.

use crate::mir::join_ir_vm_bridge::JoinIrVmBridgeError;
use crate::mir::{Callee, EffectMask, MirInstruction, MirType, ValueId};

/// Handle JoinIR MethodCall instruction
///
/// Creates a canonical Call(Method) MIR instruction for method invocation.
///
/// # Arguments
///
/// * `instructions` - Target instruction vector to append to
/// * `dst` - Destination ValueId for result
/// * `receiver` - Receiver box ValueId
/// * `method` - Method name
/// * `args` - Method arguments
/// * `type_hint` - Optional type hint (currently unused - Phase 65-2-A TODO)
pub fn handle_method_call(
    instructions: &mut Vec<MirInstruction>,
    dst: &ValueId,
    receiver: &ValueId,
    method: &str,
    args: &[ValueId],
    type_hint: &Option<MirType>,
) -> Result<(), JoinIrVmBridgeError> {
    let mir_inst = MirInstruction::Call {
        dst: Some(*dst),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "RuntimeDataBox".to_string(),
            method: method.to_string(),
            receiver: Some(*receiver),
            certainty: crate::mir::definitions::call_unified::TypeCertainty::Union,
            box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::RuntimeData,
        }),
        args: args.to_vec(),
        effects: EffectMask::PURE,
    };
    instructions.push(mir_inst);

    // Phase 65-2-A: TODO: type_hint を value_types に記録
    let _ = type_hint;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_method_call() {
        let mut instructions = vec![];
        let dst = ValueId(100);
        let receiver = ValueId(200);
        let args = vec![ValueId(1), ValueId(2)];

        let result = handle_method_call(
            &mut instructions,
            &dst,
            &receiver,
            "test_method",
            &args,
            &None,
        );

        assert!(result.is_ok());
        assert_eq!(instructions.len(), 1);

        if let MirInstruction::Call {
            dst: Some(inst_dst),
            func,
            callee:
                Some(Callee::Method {
                    method,
                    receiver: Some(box_val),
                    ..
                }),
            args: inst_args,
            effects,
        } = &instructions[0]
        {
            assert_eq!(*inst_dst, ValueId(100));
            assert_eq!(*box_val, ValueId(200));
            assert_eq!(*func, ValueId::INVALID);
            assert_eq!(method, "test_method");
            assert_eq!(inst_args, &[ValueId(1), ValueId(2)]);
            assert_eq!(*effects, EffectMask::PURE);
        } else {
            panic!("Expected Call(Method) instruction");
        }
    }

    #[test]
    fn test_handle_method_call_no_args() {
        let mut instructions = vec![];
        let dst = ValueId(300);
        let receiver = ValueId(400);

        let result = handle_method_call(
            &mut instructions,
            &dst,
            &receiver,
            "getter",
            &[],
            &Some(MirType::Integer),
        );

        assert!(result.is_ok());
        assert_eq!(instructions.len(), 1);

        if let MirInstruction::Call { args, .. } = &instructions[0] {
            assert!(args.is_empty());
        } else {
            panic!("Expected Call instruction");
        }
    }
}
