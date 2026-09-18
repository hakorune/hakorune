//! Physical structure tests only; canonical source admission stays upstream.
use super::tests::allocation_invoke_function;
use crate::mir::instruction::{
    InvokeCallResultKind as ResultKind, InvokeNormalResultKind, InvokeOperation,
};
use crate::mir::{BasicBlockId, Callee, MirInstruction, MirVerifier, ValueId};

fn call_function() -> crate::mir::MirFunction {
    let mut function = allocation_invoke_function();
    let target = hakorune_mir_defs::CanonicalGlobalTargetV1::new_static_box_method(
        "Worker".into(),
        "run".into(),
        1,
    )
    .unwrap();
    let MirInstruction::Invoke { operation, .. } = function
        .blocks
        .get_mut(&BasicBlockId::new(1))
        .unwrap()
        .terminator
        .as_mut()
        .unwrap()
    else {
        unreachable!()
    };
    *operation = InvokeOperation::Call {
        call: crate::mir::definitions::MirCall::new(
            None,
            Callee::Global(target),
            vec![ValueId::new(1)],
        ),
        result: ResultKind::I64,
    };
    function
}

#[test]
fn i64_call_has_one_normal_projection_and_shared_operand_rewrite() {
    let function = call_function();
    MirVerifier::new().verify_function(&function).unwrap();
    let MirInstruction::Invoke { operation, .. } = function.blocks[&BasicBlockId::new(1)]
        .terminator
        .as_ref()
        .unwrap()
    else {
        unreachable!()
    };
    assert_eq!(
        operation.normal_result_kind(),
        Some(InvokeNormalResultKind::I64)
    );
    let mut operation = operation.clone();
    operation.rewrite_values(|value| value.0 += 1);
    assert_eq!(operation.used_values(), vec![ValueId::new(2)]);
}

#[test]
fn same_module_free_function_i64_call_uses_source_arity_contract() {
    let mut function = call_function();
    let MirInstruction::Invoke {
        operation:
            InvokeOperation::Call {
                call,
                result: ResultKind::I64,
            },
        ..
    } = function
        .blocks
        .get_mut(&BasicBlockId::new(1))
        .unwrap()
        .terminator
        .as_mut()
        .unwrap()
    else {
        unreachable!()
    };
    call.callee = Callee::Global(
        hakorune_mir_defs::CanonicalGlobalTargetV1::new_free_function("helper".into(), 1).unwrap(),
    );
    MirVerifier::new().verify_function(&function).unwrap();
}

#[test]
fn same_module_instance_i64_call_uses_receiver_and_source_arity_contract() {
    let mut function = call_function();
    let MirInstruction::Invoke {
        operation:
            InvokeOperation::Call {
                call,
                result: ResultKind::I64,
            },
        ..
    } = function
        .blocks
        .get_mut(&BasicBlockId::new(1))
        .unwrap()
        .terminator
        .as_mut()
        .unwrap()
    else {
        unreachable!()
    };
    call.callee = Callee::SameModuleInstance {
        key: hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::instance_box_method(
            "Worker", "run", 1,
        ),
        receiver: ValueId::new(1),
    };
    MirVerifier::new().verify_function(&function).unwrap();
}

#[test]
fn i64_call_rejects_role_arity_destination_and_projection_drift() {
    for mutation in 0..7 {
        let mut function = call_function();
        let MirInstruction::Invoke {
            operation: InvokeOperation::Call { call, result },
            ..
        } = function
            .blocks
            .get_mut(&BasicBlockId::new(1))
            .unwrap()
            .terminator
            .as_mut()
            .unwrap()
        else {
            unreachable!()
        };
        let expected = match mutation {
            0 => {
                *result = ResultKind::Unit;
                "call-result-contract-not-connected"
            }
            1 => {
                call.args.clear();
                "call-result-contract-not-connected"
            }
            2 => {
                call.dst = Some(ValueId::new(8));
                "embedded-call-destination"
            }
            3 => {
                call.callee = Callee::BirthConstructor {
                    key: hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::birth_constructor(
                        "Worker", 1,
                    ),
                    receiver: ValueId::new(1),
                };
                "call-result-contract-not-connected"
            }
            4 => {
                call.callee = Callee::Extern("foreign".into());
                "call-result-contract-not-connected"
            }
            5 => {
                let block = function.blocks.get_mut(&BasicBlockId::new(2)).unwrap();
                block.instructions.clear();
                block.instruction_spans.clear();
                "normal-result-count"
            }
            _ => {
                function
                    .blocks
                    .get_mut(&BasicBlockId::new(2))
                    .unwrap()
                    .add_instruction(MirInstruction::InvokeNormalResult {
                        invoke_block: BasicBlockId::new(1),
                        dst: ValueId::new(9),
                    });
                "normal-result-count"
            }
        };
        let error = MirVerifier::new().verify_function(&function).unwrap_err();
        assert!(
            format!("{error:?}").contains(expected),
            "{mutation}: {error:?}"
        );
    }
}

#[test]
fn call_fault_cannot_read_normal_result() {
    let mut function = call_function();
    function
        .blocks
        .get_mut(&BasicBlockId::new(3))
        .unwrap()
        .add_instruction(MirInstruction::Copy {
            dst: ValueId::new(9),
            src: ValueId::new(2),
        });
    assert!(MirVerifier::new().verify_function(&function).is_err());
}

fn cataloged_callee(
    param: crate::mir::MirType,
) -> (
    hakorune_mir_defs::CanonicalSameModuleCallableKeyV1,
    crate::mir::MirFunction,
) {
    use crate::mir::{FunctionSignature, MirType};
    let key =
        hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::static_box_method("Worker", "run", 1);
    let mut callee = crate::mir::MirFunction::new(
        FunctionSignature {
            name: key.mir_symbol_projection(),
            params: vec![param],
            return_type: MirType::Integer,
            effects: crate::mir::EffectMask::CONTROL,
        },
        BasicBlockId::new(0),
    );
    callee
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .unwrap()
        .set_terminator(MirInstruction::Return { value: None });
    (key, callee)
}

fn call_module(
    callee: (
        hakorune_mir_defs::CanonicalSameModuleCallableKeyV1,
        crate::mir::MirFunction,
    ),
) -> crate::mir::MirModule {
    let mut module = crate::mir::MirModule::new("call_argument_conformance".into());
    module.add_function(call_function());
    module.add_cataloged_box_method(callee.0, callee.1).unwrap();
    module
}

#[test]
fn cataloged_call_rejects_non_i64_parameter_on_the_scalar_edge() {
    // The published ordinary Call/Invoke edge spells every argument as a
    // scalar; a cataloged callee whose formal is a borrowed map (MapBox)
    // cannot be reached by it. Typed and untyped arguments both fail
    // closed: the map lane is T1-beta caller handoff, not this verifier's
    // concern to invent.
    for typed in [true, false] {
        let mut module = call_module(cataloged_callee(crate::mir::MirType::Box("MapBox".into())));
        if typed {
            module
                .functions
                .get_mut("invoke_control_test")
                .unwrap()
                .metadata
                .value_types
                .insert(ValueId::new(1), crate::mir::MirType::Integer);
        }
        let errors = MirVerifier::new().verify_module(&module).unwrap_err();
        assert!(
            format!("{errors:?}").contains("call-argument-type-drift"),
            "typed={typed}: {errors:?}"
        );
    }
}

#[test]
fn cataloged_call_rejects_proven_non_scalar_argument() {
    let mut module = call_module(cataloged_callee(crate::mir::MirType::Integer));
    module
        .functions
        .get_mut("invoke_control_test")
        .unwrap()
        .metadata
        .value_types
        .insert(ValueId::new(1), crate::mir::MirType::Float);
    let errors = MirVerifier::new().verify_module(&module).unwrap_err();
    assert!(
        format!("{errors:?}").contains("call-argument-type-drift"),
        "{errors:?}"
    );
}

#[test]
fn cataloged_call_accepts_i64_argument_contract() {
    for typed in [true, false] {
        let mut module = call_module(cataloged_callee(crate::mir::MirType::Integer));
        if typed {
            module
                .functions
                .get_mut("invoke_control_test")
                .unwrap()
                .metadata
                .value_types
                .insert(ValueId::new(1), crate::mir::MirType::Integer);
        }
        MirVerifier::new().verify_module(&module).unwrap();
    }
}
