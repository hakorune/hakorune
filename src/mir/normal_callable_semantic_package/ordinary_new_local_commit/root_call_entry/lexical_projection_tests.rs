//! Corroborate physical arguments against real source-issued call rows.
use super::*;
use crate::mir::builder::lexical_call_projection_test_fixture;

#[test]
fn ordered_projection_uses_exact_scalar_and_original_nested_result() {
    let (prepared, row, source) = lexical_call_projection_test_fixture();
    let call = prepared
        .materialize(row.call_site().owner(), &row, &source)
        .unwrap();
    assert_eq!(call.args, vec![ValueId(79), ValueId(81)]);
    assert!(matches!(
        call.callee,
        crate::mir::definitions::Callee::SameModuleInstance {
            receiver: ValueId(77),
            ..
        }
    ));
    assert_eq!(
        prepared.arguments.len(),
        2,
        "arity follows ordinals, not recorded instruction count"
    );
    let invoke = (BasicBlockId(1), MirInstruction::Invoke {
        operation: InvokeOperation::Call { call, result: InvokeCallResultKind::Unit },
        fault_frame: ValueId(5), normal_landing: BasicBlockId(2), fault_landing: BasicBlockId(3),
    });
    assert!(EmittedLexicalCallProjectionV1::from_unit_source(
        CallPacketSourceV1::instance(row), prepared, invoke, BasicBlockId(2),
    ).unwrap_err().contains("unit-call/normal-source-drift"),
        "a physical Unit opcode cannot upgrade the original value result authority");
}

#[test]
fn ordered_projection_refuses_swapped_arguments_and_foreign_receiver_read() {
    let (mut prepared, row, source) = lexical_call_projection_test_fixture();
    let owner = row.call_site().owner();
    prepared.arguments.swap(0, 1);
    assert!(prepared
        .materialize(owner, &row, &source)
        .unwrap_err()
        .contains("argument-projection-drift"));
    prepared.arguments.swap(0, 1);
    let LexicalCallArgumentProjectionV1::Scalar(scalar) = prepared.arguments.remove(0) else {
        panic!("scalar");
    };
    let LexicalReceiverProjectionV1::Lexical(receiver) = &mut prepared.receiver else {
        panic!("lexical receiver");
    };
    prepared.arguments.insert(
        0,
        LexicalCallArgumentProjectionV1::Scalar(std::mem::replace(receiver, scalar)),
    );
    assert!(prepared
        .materialize(owner, &row, &source)
        .unwrap_err()
        .contains("receiver"));
}

#[test]
fn ordered_projection_refuses_nested_target_landing_and_literal_drift() {
    for mutation in 0..4 {
        let (mut prepared, row, source) = lexical_call_projection_test_fixture();
        let LexicalCallArgumentProjectionV1::CallResult(inner) = &mut prepared.arguments[1] else {
            panic!("nested");
        };
        match mutation {
            0 => {
                let MirInstruction::Invoke {
                    operation: InvokeOperation::Call { call, .. },
                    ..
                } = &mut inner.invoke.1
                else {
                    panic!("invoke");
                };
                call.args[0] = ValueId(999);
            }
            1 => inner.projection.value_mut().unwrap().0 = BasicBlockId(999),
            2 => {
                let value_projection = inner.projection.value().unwrap().clone();
                inner.projection = NormalCallProjectionV1::Unit { normal_landing: BasicBlockId(7) };
                assert!(inner.projection.value().unwrap_err().contains("unit-call-has-no-value"));
                assert!(inner.validate_normal_recorded(&[inner.invoke.clone()], None).is_ok());
                assert!(inner.validate_normal_recorded(&[inner.invoke.clone(), value_projection], None)
                    .unwrap_err().contains("unit-call/unexpected-value-projection"));
            }
            _ => {
                let LexicalCallArgumentProjectionV1::Integer((
                    _,
                    MirInstruction::Const { value, .. },
                )) = &mut inner.prepared.arguments[0]
                else {
                    panic!("constant");
                };
                *value = crate::mir::ConstValue::Integer(10);
            }
        }
        assert!(prepared
            .materialize(row.call_site().owner(), &row, &source)
            .is_err());
    }
}
