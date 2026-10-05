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
    for mutation in 0..3 {
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
            1 => inner.projection.0 = BasicBlockId(999),
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
