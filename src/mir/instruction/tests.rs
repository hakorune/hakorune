//! Tests for MIR Instructions
//!
//! Comprehensive test suite for all MIR instruction types and their methods.

use super::super::{EffectMask, ValueId};
use super::MirInstruction;
use crate::mir::types::{BinaryOp, ConstValue};

fn allocation_invoke_function() -> crate::mir::MirFunction {
    use crate::mir::{BasicBlock, BasicBlockId, FunctionSignature, MirFunction, MirType};
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "invoke_control_test".into(),
            params: vec![MirType::Integer],
            return_type: MirType::Void,
            effects: EffectMask::CONTROL,
        },
        BasicBlockId::new(0),
    );
    function.params = vec![ValueId::new(0)];
    let mut entry = BasicBlock::new(BasicBlockId::new(0));
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
        operation: super::InvokeOperation::NewBox {
            box_type: "Object".into(),
            args: vec![ValueId::new(1)],
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
fn invoke_normal_definition_and_backend_fence_survive_optimization() {
    use crate::mir::{BasicBlockId, MirModule, MirVerifier};
    let mut function = allocation_invoke_function();
    MirVerifier::new().verify_function(&function).unwrap();
    let origin = &function.blocks[&BasicBlockId::new(1)];
    assert_eq!(origin.out_edges().len(), 2);
    let invoke = origin.terminator.as_ref().unwrap();
    assert_eq!(invoke.dst_value(), None);
    assert_eq!(invoke.used_values(), vec![ValueId::new(1), ValueId::new(0)]);
    assert!(!invoke.effects().is_pure());
    crate::mir::passes::dce::eliminate_dead_code_in_function(&mut function);
    assert!(function.blocks[&BasicBlockId::new(2)]
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::InvokeNormalResult { .. })));
    let mut module = MirModule::new("invoke_control_test".into());
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
fn invoke_birth_is_unit_and_rewrites_receiver_arguments_and_frame_uses() {
    use crate::mir::{BasicBlockId, Callee, MirVerifier};
    let key = hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::birth_constructor("Object", 1);
    let mut operation = super::InvokeOperation::Call(crate::mir::definitions::MirCall::new(
        None,
        Callee::BirthConstructor {
            key: key.clone(),
            receiver: ValueId::new(1),
        },
        vec![ValueId::new(1)],
    ));
    assert_eq!(
        operation.used_values(),
        vec![ValueId::new(1), ValueId::new(1)]
    );
    operation.rewrite_values(|value| value.0 += 4);
    assert_eq!(
        operation.used_values(),
        vec![ValueId::new(5), ValueId::new(5)]
    );
    let super::InvokeOperation::Call(call) = &operation else {
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
        operation: super::InvokeOperation::Call(call),
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
fn test_const_instruction() {
    let dst = ValueId::new(0);
    let inst = MirInstruction::Const {
        dst,
        value: ConstValue::Integer(42),
    };

    assert_eq!(inst.dst_value(), Some(dst));
    assert!(inst.used_values().is_empty());
    assert!(inst.effects().is_pure());
}

#[test]
fn ownership_transport_instructions_are_conservative_and_exact() {
    let src = ValueId::new(7);
    let dst = ValueId::new(8);
    let copy = MirInstruction::CopyOwned { dst, src };
    let destroy = MirInstruction::DestroyOwned { value: dst };

    assert_eq!(copy.dst_value(), Some(dst));
    assert_eq!(copy.used_values(), vec![src]);
    assert_eq!(copy.effects(), EffectMask::WRITE);
    assert!(!copy.effects().is_pure());
    assert_eq!(copy.to_string(), "%8 = copy_owned %7");

    assert_eq!(destroy.dst_value(), None);
    assert_eq!(destroy.used_values(), vec![dst]);
    assert_eq!(destroy.effects(), EffectMask::WRITE);
    assert!(!destroy.effects().is_pure());
    assert_eq!(destroy.to_string(), "destroy_owned %8");
}

#[test]
fn test_binop_instruction() {
    let dst = ValueId::new(0);
    let lhs = ValueId::new(1);
    let rhs = ValueId::new(2);

    let inst = MirInstruction::BinOp {
        dst,
        op: BinaryOp::Add,
        lhs,
        rhs,
    };

    assert_eq!(inst.dst_value(), Some(dst));
    assert_eq!(inst.used_values(), vec![lhs, rhs]);
    assert!(inst.effects().is_pure());
}

#[test]
fn pinned_text_op_is_read_typed_and_transport_only() {
    let root = crate::mir::pinned_text_access_plan::PinnedTextRootIdV1::from_frame_row(2);
    let kind =
        crate::mir::pinned_text_access_plan::PinnedTextAccessKindV1::Utf8ScalarSliceEqWholeText {
            lhs_root: root,
            lhs_byte_offset: ValueId::new(11),
            lhs_width: ValueId::new(12),
            rhs_root: root,
        };
    let mut table = crate::mir::pinned_text_access_plan::PinnedTextAccessPlanTableV1::new(9);
    let plan = table.issue(kind);
    let inst = MirInstruction::PinnedTextOp {
        dst: ValueId::new(13),
        plan,
        kind,
    };

    assert_eq!(inst.effects(), EffectMask::READ);
    assert_eq!(inst.dst_value(), Some(ValueId::new(13)));
    assert_eq!(inst.used_values(), vec![ValueId::new(11), ValueId::new(12)]);
    assert_eq!(
        crate::mir::contracts::backend_core_ops::instruction_tag(&inst),
        "PinnedTextOp"
    );
    assert!(crate::mir::contracts::backend_core_ops::is_supported_mir_json_instruction(&inst));
    assert_eq!(
        crate::mir::contracts::backend_core_ops::llvm_json_ops_for_instruction(&inst),
        &[] as &[&str]
    );
    assert!(table.verify_census(&[(plan, kind)]).is_ok());
}

#[test]
fn test_call_instruction() {
    let dst = ValueId::new(0);
    let func = ValueId::new(1);
    let arg1 = ValueId::new(2);
    let arg2 = ValueId::new(3);

    let inst = MirInstruction::LegacyCallV0 {
        dst: Some(dst),
        func,
        callee: None, // Legacy mode for test
        args: vec![arg1, arg2],
        effects: EffectMask::IO,
    };

    assert_eq!(inst.dst_value(), Some(dst));
    assert_eq!(inst.used_values(), vec![func, arg1, arg2]);
    assert_eq!(inst.effects(), EffectMask::IO);
}

#[test]
fn typed_call_used_values_project_callee_operands_before_args() {
    use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
    use crate::mir::definitions::Callee;

    let typed_func_sentinel = ValueId::new(99);
    let args = vec![ValueId::new(40), ValueId::new(41)];
    let cases = vec![
        (
            Callee::Global(crate::mir::test_global_target("global".to_string())),
            vec![],
        ),
        (Callee::Extern("env.extern".to_string()), vec![]),
        (
            Callee::Constructor {
                box_type: "Box".to_string(),
            },
            vec![],
        ),
        (
            Callee::Method {
                box_name: "Box".to_string(),
                method: "method".to_string(),
                receiver: Some(ValueId::new(10)),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
            },
            vec![ValueId::new(10)],
        ),
        (Callee::Value(ValueId::new(20)), vec![ValueId::new(20)]),
        (
            Callee::Closure {
                params: vec!["x".to_string()],
                captures: vec![
                    ("a".to_string(), ValueId::new(30)),
                    ("b".to_string(), ValueId::new(30)),
                ],
                me_capture: Some(ValueId::new(31)),
            },
            vec![ValueId::new(30), ValueId::new(30), ValueId::new(31)],
        ),
    ];

    for (callee, target_values) in cases {
        let inst = MirInstruction::LegacyCallV0 {
            dst: Some(ValueId::new(1)),
            func: typed_func_sentinel,
            callee: Some(callee),
            args: args.clone(),
            effects: EffectMask::PURE,
        };
        let mut expected = target_values;
        expected.extend(args.iter().copied());
        assert_eq!(inst.used_values(), expected);
    }
}

#[test]
fn call_kind_metadata_delegates_to_canonical_call_methods() {
    use crate::mir::definitions::Callee;

    let inst = MirInstruction::LegacyCallV0 {
        dst: Some(ValueId::new(1)),
        func: ValueId::new(99),
        callee: Some(Callee::Value(ValueId::new(7))),
        args: vec![ValueId::new(8)],
        effects: EffectMask::PURE,
    };

    assert_eq!(
        crate::mir::instruction_kinds::dst_via_meta(&inst),
        Some(ValueId::new(1))
    );
    assert_eq!(
        crate::mir::instruction_kinds::used_via_meta(&inst),
        Some(vec![ValueId::new(7), ValueId::new(8)])
    );
}

#[test]
fn test_call_instruction_extern_name() {
    let inst = MirInstruction::LegacyCallV0 {
        dst: None,
        func: ValueId::INVALID,
        callee: Some(crate::mir::Callee::Extern(
            "env.runtime.checkpoint".to_string(),
        )),
        args: vec![],
        effects: EffectMask::IO,
    };

    assert_eq!(inst.extern_name(), Some("env.runtime.checkpoint"));

    let non_extern = MirInstruction::Const {
        dst: ValueId::new(0),
        value: ConstValue::Integer(1),
    };
    assert_eq!(non_extern.extern_name(), None);
}

/*
#[test]
fn test_const_value_conversion() {
    let const_val = ConstValue::Integer(42);
    let nyash_val = const_val.to_nyash_value();

    assert_eq!(nyash_val, NyashValue::new_integer(42));

    let back = ConstValue::from_nyash_value(&nyash_val).unwrap();
    assert_eq!(back, const_val);
}
*/

#[test]
fn test_ref_new_instruction() {
    let dst = ValueId::new(0);
    let box_val = ValueId::new(1);
    let inst = MirInstruction::RefNew { dst, box_val };

    assert_eq!(inst.dst_value(), Some(dst));
    assert_eq!(inst.used_values(), vec![box_val]);
    assert!(inst.effects().is_pure());
}

#[test]
fn test_field_get_instruction() {
    let dst = ValueId::new(0);
    let base = ValueId::new(1);
    let inst = MirInstruction::FieldGet {
        dst,
        base,
        field: "x".to_string(),
        declared_type: Some(crate::mir::MirType::Box("IntegerBox".to_string())),
    };

    assert_eq!(inst.dst_value(), Some(dst));
    assert_eq!(inst.used_values(), vec![base]);
    assert!(!inst.effects().is_pure());
}

#[test]
fn test_field_set_instruction() {
    let base = ValueId::new(0);
    let value = ValueId::new(1);
    let inst = MirInstruction::FieldSet {
        base,
        field: "x".to_string(),
        value,
        declared_type: Some(crate::mir::MirType::Box("IntegerBox".to_string())),
    };

    assert_eq!(inst.dst_value(), None);
    assert_eq!(inst.used_values(), vec![base, value]);
    assert!(!inst.effects().is_pure());
}

#[test]
fn test_method_call_instruction() {
    use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
    use crate::mir::definitions::Callee;

    let dst = ValueId::new(0);
    let receiver = ValueId::new(1);
    let arg = ValueId::new(2);
    let inst = crate::mir::ssot::method_call::runtime_method_call(
        Some(dst),
        receiver,
        "ArrayBox".to_string(),
        "push".to_string(),
        vec![arg],
        EffectMask::MUT,
        TypeCertainty::Known,
    );

    assert!(matches!(
        &inst,
        MirInstruction::Call(crate::mir::definitions::MirCall {
            callee: Callee::Method {
                box_name,
                method,
                receiver: Some(recv),
                certainty,
                box_kind,
            },
            ..
        }) if box_name == "ArrayBox"
            && method == "push"
            && *recv == receiver
            && *certainty == TypeCertainty::Known
            && *box_kind == CalleeBoxKind::RuntimeData
    ));

    assert_eq!(inst.dst_value(), Some(dst));
    assert_eq!(inst.used_values(), vec![receiver, arg]);
    assert_eq!(inst.effects(), EffectMask::MUT);
}

#[test]
fn test_method_call_getfield_instruction() {
    use crate::mir::definitions::call_unified::TypeCertainty;

    let dst = ValueId::new(0);
    let reference = ValueId::new(1);
    let field_name = ValueId::new(2);
    let inst = crate::mir::ssot::method_call::runtime_method_call(
        Some(dst),
        reference,
        "InstanceBox",
        "getField",
        vec![field_name],
        EffectMask::READ,
        TypeCertainty::Known,
    );

    assert_eq!(inst.dst_value(), Some(dst));
    assert_eq!(inst.used_values(), vec![reference, field_name]);
    assert!(!inst.effects().is_pure());
    assert!(inst
        .effects()
        .contains(super::super::effect::Effect::ReadHeap));
}

#[test]
fn test_method_call_setfield_instruction() {
    use crate::mir::definitions::call_unified::TypeCertainty;

    let reference = ValueId::new(0);
    let field_name = ValueId::new(1);
    let value = ValueId::new(2);
    let inst = crate::mir::ssot::method_call::runtime_method_call(
        None,
        reference,
        "InstanceBox",
        "setField",
        vec![field_name, value],
        EffectMask::WRITE,
        TypeCertainty::Known,
    );

    assert_eq!(inst.dst_value(), None);
    assert_eq!(inst.used_values(), vec![reference, field_name, value]);
    assert!(!inst.effects().is_pure());
    assert!(inst
        .effects()
        .contains(super::super::effect::Effect::WriteHeap));
}

#[test]
fn test_weakref_new_instruction() {
    let dst = ValueId::new(0);
    let box_val = ValueId::new(1);
    let inst = MirInstruction::WeakRef {
        dst,
        op: crate::mir::WeakRefOp::New,
        value: box_val,
    };

    assert_eq!(inst.dst_value(), Some(dst));
    assert_eq!(inst.used_values(), vec![box_val]);
    assert!(inst.effects().is_pure());
}

#[test]
fn test_weakref_load_instruction() {
    let dst = ValueId::new(0);
    let weak_ref = ValueId::new(1);
    let inst = MirInstruction::WeakRef {
        dst,
        op: crate::mir::WeakRefOp::Load,
        value: weak_ref,
    };

    assert_eq!(inst.dst_value(), Some(dst));
    assert_eq!(inst.used_values(), vec![weak_ref]);
    assert!(!inst.effects().is_pure());
    assert!(inst
        .effects()
        .contains(super::super::effect::Effect::ReadHeap));
}

#[test]
fn test_barrier_instructions() {
    let ptr = ValueId::new(0);

    let read_barrier = MirInstruction::Barrier {
        op: crate::mir::BarrierOp::Read,
        ptr,
    };
    assert_eq!(read_barrier.dst_value(), None);
    assert_eq!(read_barrier.used_values(), vec![ptr]);
    assert!(read_barrier
        .effects()
        .contains(super::super::effect::Effect::Barrier));
    assert!(read_barrier
        .effects()
        .contains(super::super::effect::Effect::ReadHeap));

    let write_barrier = MirInstruction::Barrier {
        op: crate::mir::BarrierOp::Write,
        ptr,
    };
    assert_eq!(write_barrier.dst_value(), None);
    assert_eq!(write_barrier.used_values(), vec![ptr]);
    assert!(write_barrier
        .effects()
        .contains(super::super::effect::Effect::Barrier));
    assert!(write_barrier
        .effects()
        .contains(super::super::effect::Effect::WriteHeap));
}

#[test]
fn test_extern_call_instruction() {
    use crate::mir::definitions::Callee;

    let dst = ValueId::new(0);
    let arg1 = ValueId::new(1);
    let arg2 = ValueId::new(2);
    let inst = crate::mir::ssot::extern_call::extern_call(
        Some(dst),
        "env.console".to_string(),
        "log".to_string(),
        vec![arg1, arg2],
        super::super::effect::EffectMask::IO,
    );

    assert!(matches!(
        &inst,
        MirInstruction::Call(crate::mir::definitions::MirCall {
            callee: Callee::Extern(name),
            ..
        }) if name == "env.console.log"
    ));

    assert_eq!(inst.dst_value(), Some(dst));
    assert_eq!(inst.used_values(), vec![arg1, arg2]);
    assert_eq!(inst.effects(), super::super::effect::EffectMask::IO);

    // Test void extern call
    let void_inst = crate::mir::ssot::extern_call::extern_call(
        None,
        "env.canvas".to_string(),
        "fillRect".to_string(),
        vec![arg1],
        super::super::effect::EffectMask::IO,
    );

    assert_eq!(void_inst.dst_value(), None);
    assert_eq!(void_inst.used_values(), vec![arg1]);
}
