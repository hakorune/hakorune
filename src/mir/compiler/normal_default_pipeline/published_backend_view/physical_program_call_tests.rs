//! The physical input gate must not export ordinary I64 as a Birth body.
use super::*;
use crate::mir::instruction::InvokeCallResultKind;

#[test]
fn non_unit_call_stays_before_physical_export() {
    for ordinary in [false, true] {
        let callee = if ordinary {
            Callee::Global(
                hakorune_mir_defs::CanonicalGlobalTargetV1::new_static_box_method(
                    "Worker".into(),
                    "run".into(),
                    0,
                )
                .unwrap(),
            )
        } else {
            Callee::BirthConstructor {
                key: hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::birth_constructor(
                    "Worker", 0,
                ),
                receiver: crate::mir::ValueId::new(1),
            }
        };
        let instruction = MirInstruction::Invoke {
            operation: InvokeOperation::Call {
                call: MirCall::new(None, callee, vec![]),
                result: InvokeCallResultKind::I64,
            },
            fault_frame: crate::mir::ValueId::new(0),
            normal_landing: crate::mir::BasicBlockId::new(2),
            fault_landing: crate::mir::BasicBlockId::new(3),
        };
        assert!(validate_instruction(&instruction, false)
            .unwrap_err()
            .contains("instruction-unsupported"));
    }
}
