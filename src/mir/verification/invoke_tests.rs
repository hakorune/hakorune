use crate::mir::instruction::InvokeCallResultKind;
use crate::mir::instruction::{FaultFrameMode, InvokeOperation};
use crate::mir::types::ConstValue;
use crate::mir::{EffectMask, MirInstruction, ValueId};
pub(super) fn allocation_invoke_function() -> crate::mir::MirFunction {
    use crate::mir::{BasicBlock, BasicBlockId, FunctionSignature, MirFunction, MirType};
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "invoke_control_test".into(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::CONTROL,
        },
        BasicBlockId::new(0),
    );
    let mut entry = BasicBlock::new(BasicBlockId::new(0));
    entry.add_instruction(MirInstruction::FaultFrameEnter {
        dst: ValueId::new(0),
        mode: FaultFrameMode::RootOwned,
    });
    entry.add_instruction(MirInstruction::Const {
        dst: ValueId::new(1),
        value: ConstValue::Integer(7),
    });
    entry.set_terminator(MirInstruction::Jump {
        target: BasicBlockId::new(1),
        edge_args: None,
    });
    let mut origin = BasicBlock::new(BasicBlockId::new(1));
    origin.set_terminator(MirInstruction::Invoke {
        operation: InvokeOperation::NewBox {
            object: hakorune_mir_defs::CanonicalObjectIdV1::from_declaration_index(0).unwrap(),
        },
        fault_frame: ValueId::new(0),
        normal_landing: BasicBlockId::new(2),
        fault_landing: BasicBlockId::new(3),
    });
    let mut normal = BasicBlock::new(BasicBlockId::new(2));
    normal.add_instruction(MirInstruction::InvokeNormalResult {
        invoke_block: BasicBlockId::new(1),
        dst: ValueId::new(2),
    });
    normal.set_terminator(MirInstruction::Return { value: None });
    let mut fault = BasicBlock::new(BasicBlockId::new(3));
    fault.set_terminator(MirInstruction::ReturnFault {
        fault_frame: ValueId::new(0),
    });
    for block in [entry, origin, normal, fault] {
        function.add_block(block);
    }
    function.update_cfg();
    function
}

#[test]
fn allocation_identity_requires_definition_and_is_not_a_value_operand() {
    use crate::mir::function::{CanonicalObjectDefinitionV1, ObjectLayoutUnavailableV1};
    use crate::mir::{BasicBlockId, MirModule, MirVerifier};
    use hakorune_mir_defs::CanonicalObjectIdV1;

    let mut function = allocation_invoke_function();
    let invoke = function
        .blocks
        .get_mut(&BasicBlockId::new(1))
        .unwrap()
        .terminator
        .as_mut()
        .unwrap();
    let MirInstruction::Invoke { operation, .. } = invoke else {
        unreachable!()
    };
    let original = operation.clone();
    assert!(operation.used_values().is_empty());
    operation.rewrite_values(|_| panic!("object identity is not a ValueId"));
    assert_eq!(*operation, original);

    // Physical membership test only: source provenance belongs to the
    // branded declaration/claim handoff, not a numerically equal local ID.
    let mut module = MirModule::new("allocation_identity".into());
    module.add_function(function);
    let errors = MirVerifier::new().verify_module(&module).unwrap_err();
    assert!(format!("{errors:?}").contains("object-definition-missing"));
    module.install_object_definitions_preflighted(
        vec![CanonicalObjectDefinitionV1::from_source_declaration(
            "RenamedForDiagnostics".into(),
            Box::new([]),
            Err(ObjectLayoutUnavailableV1::Inheritance),
            crate::mir::function::ObjectDestructionDispositionV1::Unavailable(
                crate::mir::function::ObjectDestructionUnavailableV1::Declaration(
                    ObjectLayoutUnavailableV1::Inheritance,
                ),
            ),
        )]
        .into_boxed_slice(),
    );
    // A present but unsupported definition is not a missing identity.
    MirVerifier::new().verify_module(&module).unwrap();
    let function = module.functions.get_mut("invoke_control_test").unwrap();
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
    *operation = InvokeOperation::NewBox {
        object: CanonicalObjectIdV1::from_declaration_index(1).unwrap(),
    };
    let errors = MirVerifier::new().verify_module(&module).unwrap_err();
    assert!(format!("{errors:?}").contains("object-definition-missing"));
}

#[test]
fn exact_field_store_is_unit_and_requires_published_definition() {
    use crate::mir::{BasicBlockId, MirModule, MirVerifier};
    use hakorune_mir_defs::{CanonicalFieldRefV1, CanonicalObjectIdV1};
    let field = CanonicalFieldRefV1::from_declaration_ordinal(
        CanonicalObjectIdV1::from_declaration_index(0).unwrap(),
        0,
    )
    .unwrap();
    let mut function = allocation_invoke_function();
    let invoke = function
        .blocks
        .get_mut(&BasicBlockId::new(1))
        .unwrap()
        .terminator
        .as_mut()
        .unwrap();
    let MirInstruction::Invoke { operation, .. } = invoke else {
        unreachable!()
    };
    *operation = InvokeOperation::FieldSet {
        field,
        base: ValueId::new(1),
        value: ValueId::new(1),
    };
    assert_eq!(
        operation.used_values(),
        vec![ValueId::new(1), ValueId::new(1)]
    );
    let mut rewritten = operation.clone();
    rewritten.rewrite_values(|value| *value = ValueId::new(9));
    assert_eq!(
        rewritten.used_values(),
        vec![ValueId::new(9), ValueId::new(9)]
    );
    let errors = super::check_function(&function).unwrap_err();
    assert!(
        format!("{errors:?}").contains("normal-result-count"),
        "Unit store cannot expose an internal status as a source value"
    );
    function
        .blocks
        .get_mut(&BasicBlockId::new(2))
        .unwrap()
        .instructions
        .clear();
    super::check_function(&function).unwrap();
    let mut module = MirModule::new("missing_field_definition".into());
    module.add_function(function);
    let errors = MirVerifier::new().verify_module(&module).unwrap_err();
    assert!(format!("{errors:?}").contains("field-definition-missing"));
}

#[test]
fn invoke_normal_definition_and_backend_fence_survive_optimization() {
    use crate::mir::{BasicBlockId, MirModule, MirVerifier};
    let mut function = allocation_invoke_function();
    MirVerifier::new().verify_function(&function).unwrap();
    let origin = &function.blocks[&BasicBlockId::new(1)];
    assert_eq!(origin.out_edges().len(), 2);
    let invoke = origin.terminator.as_ref().unwrap();
    assert_eq!(invoke.dst_value(), None);
    assert_eq!(invoke.used_values(), vec![ValueId::new(0)]);
    assert!(!invoke.effects().is_pure());
    crate::mir::passes::dce::eliminate_dead_code_in_function(&mut function);
    assert!(function.blocks[&BasicBlockId::new(2)]
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::InvokeNormalResult { .. })));
    let mut module = MirModule::new("invoke_control_test".into());
    module.install_object_definitions_preflighted(
        vec![
            crate::mir::function::CanonicalObjectDefinitionV1::from_source_declaration(
                "Object".into(),
                Box::new([]),
                Ok(()),
                crate::mir::function::ObjectDestructionDispositionV1::PlainI64NoHook,
            ),
        ]
        .into_boxed_slice(),
    );
    module.add_function(function);
    crate::mir::passes::simplify_cfg::simplify(&mut module);
    let function = &module.functions["invoke_control_test"];
    assert!(!function.blocks.contains_key(&BasicBlockId::new(1)));
    assert!(
        matches!(&function.blocks[&BasicBlockId::new(2)].instructions[0],
            MirInstruction::InvokeNormalResult { invoke_block, .. } if *invoke_block == BasicBlockId::new(0))
    );
    MirVerifier::new().verify_module(&module).unwrap();
    let view = crate::mir::function::PublishedMirBackendView::try_new(&module).unwrap();
    assert_eq!(
        view.route(),
        crate::mir::function::PublishedStaticMethodRouteV1::UnsupportedBeforeObject
    );
}

#[test]
fn invoke_rejects_fault_result_use_and_invalid_origin_shapes() {
    use crate::mir::{BasicBlockId, MirVerifier};
    for mutation in 0..7 {
        let mut function = allocation_invoke_function();
        match mutation {
            0 => function
                .blocks
                .get_mut(&BasicBlockId::new(3))
                .unwrap()
                .set_terminator(MirInstruction::Return {
                    value: Some(ValueId::new(2)),
                }),
            1 => {
                let normal = function.blocks.get_mut(&BasicBlockId::new(2)).unwrap();
                normal.instructions[0] = MirInstruction::InvokeNormalResult {
                    invoke_block: BasicBlockId::new(99),
                    dst: ValueId::new(2),
                };
            }
            2 => {
                let normal = function.blocks.get_mut(&BasicBlockId::new(2)).unwrap();
                normal.instructions.clear();
                normal.instruction_spans.clear();
            }
            3 => {
                let normal = function.blocks.get_mut(&BasicBlockId::new(2)).unwrap();
                normal.add_instruction(MirInstruction::InvokeNormalResult {
                    invoke_block: BasicBlockId::new(1),
                    dst: ValueId::new(3),
                });
            }
            4 => function
                .blocks
                .get_mut(&BasicBlockId::new(3))
                .unwrap()
                .set_terminator(MirInstruction::Jump {
                    target: BasicBlockId::new(2),
                    edge_args: None,
                }),
            5 => function
                .blocks
                .get_mut(&BasicBlockId::new(3))
                .unwrap()
                .add_instruction(MirInstruction::Phi {
                    dst: ValueId::new(4),
                    inputs: vec![(BasicBlockId::new(1), ValueId::new(2))],
                    type_hint: None,
                }),
            6 => {
                function.entry_block = BasicBlockId::new(2);
                function
                    .blocks
                    .get_mut(&BasicBlockId::new(2))
                    .unwrap()
                    .set_terminator(MirInstruction::Jump {
                        target: BasicBlockId::new(1),
                        edge_args: None,
                    });
            }
            _ => unreachable!(),
        }
        function.update_cfg();
        assert!(
            MirVerifier::new().verify_function(&function).is_err(),
            "mutation={mutation}"
        );
    }
}

#[test]
fn cleanup_is_unit_and_requires_definition_owned_home_disposition() {
    use crate::mir::function::{
        CanonicalObjectDefinitionV1, ObjectDestructionDispositionV1 as D,
        ObjectDestructionUnavailableV1 as U,
    };
    use crate::mir::{BasicBlockId, MirModule, MirVerifier};
    let object = hakorune_mir_defs::CanonicalObjectIdV1::from_declaration_index(0).unwrap();
    for home in [false, true] {
        let mut operation = if home {
            InvokeOperation::HomeRelease {
                object,
                value: ValueId(1),
            }
        } else {
            InvokeOperation::ReclaimUnpublished {
                object,
                value: ValueId(1),
            }
        };
        let original = operation.clone();
        assert_eq!(operation.used_values(), vec![ValueId(1)]);
        operation.rewrite_values(|value| value.0 += 4);
        assert_eq!(operation.used_values(), vec![ValueId(5)]);
        operation.rewrite_values(|value| value.0 -= 4);
        assert_eq!(
            operation, original,
            "object identity is not a ValueId operand"
        );
        for disposition in [D::PlainI64NoHook, D::Unavailable(U::MemberRole)] {
            // Physical verifier witness, not source or runtime admission.
            let mut function = allocation_invoke_function();
            let origin = function.blocks.get_mut(&BasicBlockId(1)).unwrap();
            let Some(MirInstruction::Invoke {
                operation: target, ..
            }) = &mut origin.terminator
            else {
                unreachable!()
            };
            *target = operation.clone();
            let normal = function.blocks.get_mut(&BasicBlockId(2)).unwrap();
            normal.instructions.clear();
            normal.instruction_spans.clear();
            let mut module = MirModule::new("cleanup".into());
            module.add_function(function);
            module.install_object_definitions_preflighted(
                vec![CanonicalObjectDefinitionV1::from_source_declaration(
                    "Object".into(),
                    Box::new([]),
                    Ok(()),
                    disposition,
                )]
                .into_boxed_slice(),
            );
            let result = MirVerifier::new().verify_module(&module);
            if home && disposition != D::PlainI64NoHook {
                assert!(
                    format!("{:?}", result.unwrap_err()).contains("home-destruction-unavailable")
                );
            } else {
                result.unwrap();
            }
        }
    }
}

#[test]
fn invoke_birth_is_unit_and_rewrites_receiver_arguments_and_frame_uses() {
    use crate::mir::{BasicBlockId, Callee, MirVerifier};
    let key = hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::birth_constructor("Object", 1);
    let mut operation = InvokeOperation::Call { call: crate::mir::definitions::MirCall::new(
        None,
        Callee::BirthConstructor {
            key: key.clone(),
            receiver: ValueId::new(1),
        },
        vec![ValueId::new(1)],
    ), result: InvokeCallResultKind::Unit };
    assert_eq!(
        operation.used_values(),
        vec![ValueId::new(1), ValueId::new(1)]
    );
    operation.rewrite_values(|value| value.0 += 4);
    assert_eq!(
        operation.used_values(),
        vec![ValueId::new(5), ValueId::new(5)]
    );
    let InvokeOperation::Call { call, result: InvokeCallResultKind::Unit } = &operation else {
        unreachable!()
    };
    assert!(matches!(&call.callee, Callee::BirthConstructor { key: after, .. } if after == &key));
    let mut function = allocation_invoke_function();
    let origin = function.blocks.get_mut(&BasicBlockId::new(1)).unwrap();
    let MirInstruction::Invoke {
        operation: target, ..
    } = origin.terminator.as_mut().unwrap()
    else {
        unreachable!()
    };
    operation.rewrite_values(|value| value.0 -= 4);
    *target = operation;
    let normal = function.blocks.get_mut(&BasicBlockId::new(2)).unwrap();
    normal.instructions.clear();
    normal.instruction_spans.clear();
    MirVerifier::new().verify_function(&function).unwrap();
    let origin = function.blocks.get_mut(&BasicBlockId::new(1)).unwrap();
    let MirInstruction::Invoke {
        operation: InvokeOperation::Call { call, result: InvokeCallResultKind::Unit },
        ..
    } = origin.terminator.as_mut().unwrap()
    else {
        unreachable!()
    };
    call.dst = Some(ValueId::new(2));
    let errors = MirVerifier::new().verify_function(&function).unwrap_err();
    assert!(format!("{errors:?}").contains("embedded-call-destination"));
}

#[test]
fn fault_frame_is_internal_and_rejects_substitution_or_escape() {
    use crate::mir::{BasicBlockId, MirType, MirVerifier};
    let entry_id = BasicBlockId::new(0);
    let origin_id = BasicBlockId::new(1);
    let normal_id = BasicBlockId::new(2);
    let mut borrowed = allocation_invoke_function();
    borrowed.blocks.get_mut(&entry_id).unwrap().instructions[0] = MirInstruction::FaultFrameEnter {
        dst: ValueId::new(0),
        mode: FaultFrameMode::Borrowed,
    };
    MirVerifier::new().verify_function(&borrowed).unwrap();
    assert!(borrowed.params.is_empty() && borrowed.signature.params.is_empty());
    for mutation in 0..10 {
        let mut function = allocation_invoke_function();
        let expected = match mutation {
            0 => {
                function.blocks.get_mut(&entry_id).unwrap().instructions[0] =
                    MirInstruction::Const {
                        dst: ValueId::new(0),
                        value: ConstValue::Integer(0),
                    };
                "frame-entry-count"
            }
            1 => {
                function.blocks.get_mut(&entry_id).unwrap().add_instruction(
                    MirInstruction::FaultFrameEnter {
                        dst: ValueId::new(7),
                        mode: FaultFrameMode::Borrowed,
                    },
                );
                "frame-entry-count"
            }
            2 => {
                function
                    .blocks
                    .get_mut(&entry_id)
                    .unwrap()
                    .instructions
                    .swap(0, 1);
                "frame-entry-position"
            }
            3 => {
                function.params.push(ValueId::new(0));
                function.signature.params.push(MirType::Integer);
                "frame-source-value"
            }
            4 => {
                function
                    .metadata
                    .value_types
                    .insert(ValueId::new(0), MirType::Integer);
                "frame-source-value"
            }
            5 => {
                if let Some(MirInstruction::Invoke { fault_frame, .. }) =
                    &mut function.blocks.get_mut(&origin_id).unwrap().terminator
                {
                    *fault_frame = ValueId::new(1);
                }
                "foreign-frame-operand"
            }
            6 => {
                function.blocks.get_mut(&normal_id).unwrap().set_terminator(
                    MirInstruction::Return {
                        value: Some(ValueId::new(0)),
                    },
                );
                "frame-escaped-as-source-value"
            }
            7 => {
                function
                    .blocks
                    .get_mut(&normal_id)
                    .unwrap()
                    .add_instruction(MirInstruction::Copy {
                        dst: ValueId::new(8),
                        src: ValueId::new(0),
                    });
                "frame-escaped-as-source-value"
            }
            8 => {
                if let Some(MirInstruction::Invoke { operation, .. }) =
                    &mut function.blocks.get_mut(&origin_id).unwrap().terminator
                {
                    *operation = InvokeOperation::FieldSet {
                        field: hakorune_mir_defs::CanonicalFieldRefV1::from_declaration_ordinal(
                            hakorune_mir_defs::CanonicalObjectIdV1::from_declaration_index(0)
                                .unwrap(),
                            0,
                        )
                        .unwrap(),
                        base: ValueId::new(1),
                        value: ValueId::new(0),
                    };
                }
                "frame-escaped-as-source-value"
            }
            9 => {
                function
                    .blocks
                    .get_mut(&normal_id)
                    .unwrap()
                    .set_terminator(MirInstruction::Jump {
                        target: entry_id,
                        edge_args: None,
                    });
                "frame-entry-reentered"
            }
            _ => unreachable!(),
        };
        function.update_cfg();
        let errors = MirVerifier::new().verify_function(&function).unwrap_err();
        assert!(
            format!("{errors:?}").contains(expected),
            "mutation={mutation}: {errors:?}"
        );
    }
}
