//! Physical use propagation witnesses, not source or C consumer admission.
use super::super::PublishedMirBackendView;
use super::*;
use crate::mir::{
    BasicBlockId, Callee, ConstValue, ConstructionTarget, EffectMask, FunctionSignature,
    MirFunction, MirModule, MirType, ValueId,
};
use hakorune_mir_defs::{
    CanonicalBuiltinGlobalV1, CanonicalGlobalTargetV1, CanonicalSameModuleCallableKeyV1,
};

fn function(name: &str, params: usize) -> MirFunction {
    MirFunction::new(
        FunctionSignature {
            name: name.into(),
            params: vec![MirType::Integer; params],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    )
}

fn append(body: &mut MirFunction, instruction: MirInstruction) {
    body.blocks
        .get_mut(&BasicBlockId::new(0))
        .unwrap()
        .add_instruction(instruction);
}

fn constant(body: &mut MirFunction, dst: u32, value: ConstValue) {
    append(
        body,
        MirInstruction::Const {
            dst: ValueId::new(dst),
            value,
        },
    );
}

fn write(body: &mut MirFunction, value: ValueId) {
    append(
        body,
        MirInstruction::NewBox {
            dst: ValueId::new(20),
            target: ConstructionTarget::IntrinsicMap,
            args: vec![],
        },
    );
    constant(body, 21, ConstValue::String("v".into()));
    append(
        body,
        MirInstruction::MapLiteralEntryWrite {
            receiver: ValueId::new(20),
            key: ValueId::new(21),
            value,
        },
    );
}

fn print(body: &mut MirFunction, value: ValueId) {
    append(
        body,
        MirInstruction::call(
            None,
            Callee::Global(CanonicalGlobalTargetV1::Builtin(
                CanonicalBuiltinGlobalV1::Print,
            )),
            vec![value],
            EffectMask::IO,
        ),
    );
}

#[test]
fn map_literal_original_demand_keeps_map_only_float_separate_from_original_uses() {
    for original_use in [false, true] {
        let mut body = function("main", 0);
        constant(&mut body, 7, ConstValue::Float(-0.0));
        append(
            &mut body,
            MirInstruction::Copy {
                dst: ValueId::new(8),
                src: ValueId::new(7),
            },
        );
        write(&mut body, ValueId::new(8));
        if original_use {
            print(&mut body, ValueId::new(8));
        }
        let mut module = MirModule::new("copy-demand".into());
        module.add_function(body);
        let view = PublishedMirBackendView::try_new(&module).unwrap();
        let index = MapBodyIndex::from_view(&view).unwrap();
        let actions = BTreeMap::from([
            (
                ("main", ValueId::new(7)),
                ProjectionAction::ExactF64((-0.0_f64).to_bits()),
            ),
            (("main", ValueId::new(8)), ProjectionAction::Copy),
        ]);
        let original = index.original_value_demands(&actions).unwrap();
        assert_eq!(original.contains(&("main", ValueId::new(7))), original_use);
        assert_eq!(original.contains(&("main", ValueId::new(8))), original_use);
        assert!(original.contains(&("main", ValueId::new(20))));
        assert!(original.contains(&("main", ValueId::new(21))));
        let mut missing = actions.clone();
        missing.remove(&("main", ValueId::new(7)));
        assert!(index
            .original_value_demands(&missing)
            .unwrap_err()
            .contains("projection-coverage-mismatch"));
    }
}

#[test]
fn map_literal_original_demand_propagates_formal_uses_to_every_caller() {
    for original_use in [false, true] {
        let key = CanonicalSameModuleCallableKeyV1::test_static_box_method("Helpers", "stash", 1);
        let mut callee = function(&key.mir_symbol_projection(), 1);
        let formal = callee.params[0];
        write(&mut callee, formal);
        if original_use {
            print(&mut callee, formal);
        }
        let mut module = MirModule::new("formal-demand".into());
        module
            .add_cataloged_box_method(key.clone(), callee)
            .unwrap();
        for (name, literal) in [
            ("float", ConstValue::Float(1.5)),
            ("bool", ConstValue::Bool(true)),
        ] {
            let mut caller = function(name, 0);
            constant(&mut caller, 7, literal);
            append(
                &mut caller,
                MirInstruction::call(
                    Some(ValueId::new(8)),
                    Callee::Global(key.canonical_global_target_v1().unwrap()),
                    vec![ValueId::new(7)],
                    EffectMask::PURE,
                ),
            );
            // Consuming the integer call result must not force an original
            // lane for a Map-only formal in the callee.
            append(
                &mut caller,
                MirInstruction::Return {
                    value: Some(ValueId::new(8)),
                },
            );
            module.add_function(caller);
        }
        let target = module.canonical_callable_definition_symbol(&key).unwrap();
        let view = PublishedMirBackendView::try_new(&module).unwrap();
        let index = MapBodyIndex::from_view(&view).unwrap();
        let actions = BTreeMap::from([
            ((target, formal), ProjectionAction::Formal(0)),
            (
                ("float", ValueId::new(7)),
                ProjectionAction::ExactF64(1.5_f64.to_bits()),
            ),
            (("bool", ValueId::new(7)), ProjectionAction::ExactBool(true)),
        ]);
        let original = index.original_value_demands(&actions).unwrap();
        for key in [
            (target, formal),
            ("float", ValueId::new(7)),
            ("bool", ValueId::new(7)),
        ] {
            assert_eq!(original.contains(&key), original_use, "{key:?}");
        }
    }
}

#[test]
fn map_literal_original_value_action_demands_its_producer() {
    let mut body = function("main", 0);
    constant(&mut body, 7, ConstValue::String("nested".into()));
    write(&mut body, ValueId::new(7));
    let mut module = MirModule::new("original-producer".into());
    module.add_function(body);
    let view = PublishedMirBackendView::try_new(&module).unwrap();
    let index = MapBodyIndex::from_view(&view).unwrap();
    let actions = BTreeMap::from([(("main", ValueId::new(7)), ProjectionAction::OriginalHandle)]);
    assert!(index
        .original_value_demands(&actions)
        .unwrap()
        .contains(&("main", ValueId::new(7))));
}

#[test]
fn map_literal_original_demand_select_condition_does_not_leak_to_other_formal() {
    let key = CanonicalSameModuleCallableKeyV1::test_static_box_method("Helpers", "choose", 2);
    let mut callee = function(&key.mir_symbol_projection(), 2);
    let condition = callee.params[0];
    let float = callee.params[1];
    constant(&mut callee, 7, ConstValue::Integer(30));
    append(
        &mut callee,
        MirInstruction::Select {
            dst: ValueId::new(8),
            cond: condition,
            then_val: float,
            else_val: ValueId::new(7),
        },
    );
    write(&mut callee, ValueId::new(8));
    append(
        &mut callee,
        MirInstruction::MapLiteralEntryWrite {
            receiver: ValueId::new(20),
            key: ValueId::new(21),
            value: condition,
        },
    );
    let mut module = MirModule::new("select-formal-demand".into());
    module
        .add_cataloged_box_method(key.clone(), callee)
        .unwrap();
    let mut caller = function("main", 0);
    constant(&mut caller, 7, ConstValue::Bool(true));
    constant(&mut caller, 8, ConstValue::Float(1.5));
    append(
        &mut caller,
        MirInstruction::call(
            Some(ValueId::new(9)),
            Callee::Global(key.canonical_global_target_v1().unwrap()),
            vec![ValueId::new(7), ValueId::new(8)],
            EffectMask::PURE,
        ),
    );
    module.add_function(caller);
    let target = module.canonical_callable_definition_symbol(&key).unwrap();
    let view = PublishedMirBackendView::try_new(&module).unwrap();
    let index = MapBodyIndex::from_view(&view).unwrap();
    let actions = BTreeMap::from([
        ((target, condition), ProjectionAction::Formal(0)),
        ((target, float), ProjectionAction::Formal(1)),
        ((target, ValueId::new(7)), ProjectionAction::ExactI64(30)),
        ((target, ValueId::new(8)), ProjectionAction::Select),
        (("main", ValueId::new(7)), ProjectionAction::ExactBool(true)),
        (
            ("main", ValueId::new(8)),
            ProjectionAction::ExactF64(1.5_f64.to_bits()),
        ),
    ]);
    let original = index.original_value_demands(&actions).unwrap();
    assert!(original.contains(&(target, condition)));
    assert!(original.contains(&("main", ValueId::new(7))));
    for key in [
        (target, float),
        (target, ValueId::new(7)),
        (target, ValueId::new(8)),
        ("main", ValueId::new(8)),
    ] {
        assert!(
            !original.contains(&key),
            "unexpected original lane: {key:?}"
        );
    }
}

#[test]
fn map_literal_original_demand_preserves_ownership_copy_operation() {
    let mut body = function("main", 0);
    constant(&mut body, 7, ConstValue::String("owned".into()));
    append(
        &mut body,
        MirInstruction::CopyOwned {
            dst: ValueId::new(8),
            src: ValueId::new(7),
        },
    );
    write(&mut body, ValueId::new(8));
    let mut module = MirModule::new("owned-demand".into());
    module.add_function(body);
    let view = PublishedMirBackendView::try_new(&module).unwrap();
    let index = MapBodyIndex::from_view(&view).unwrap();
    let actions = BTreeMap::from([
        (("main", ValueId::new(7)), ProjectionAction::OriginalHandle),
        (("main", ValueId::new(8)), ProjectionAction::Copy),
    ]);
    let original = index.original_value_demands(&actions).unwrap();
    assert!(original.contains(&("main", ValueId::new(7))));
    assert!(original.contains(&("main", ValueId::new(8))));
}

#[test]
fn map_literal_original_demand_phi_backedge_reaches_a_fixed_point() {
    for original_use in [false, true] {
        let mut body = function("main", 0);
        constant(&mut body, 7, ConstValue::Integer(1));
        constant(&mut body, 9, ConstValue::Bool(false));
        constant(&mut body, 10, ConstValue::Integer(30));
        body.blocks
            .get_mut(&BasicBlockId::new(0))
            .unwrap()
            .set_terminator(MirInstruction::Jump {
                target: BasicBlockId::new(1),
                edge_args: None,
            });
        let mut loop_block = crate::mir::BasicBlock::new(BasicBlockId::new(1));
        loop_block.add_instruction(MirInstruction::Phi {
            dst: ValueId::new(8),
            inputs: vec![
                (BasicBlockId::new(0), ValueId::new(7)),
                (BasicBlockId::new(1), ValueId::new(8)),
            ],
            type_hint: None,
        });
        loop_block.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(20),
            target: ConstructionTarget::IntrinsicMap,
            args: vec![],
        });
        loop_block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(21),
            value: ConstValue::String("v".into()),
        });
        loop_block.add_instruction(MirInstruction::MapLiteralEntryWrite {
            receiver: ValueId::new(20),
            key: ValueId::new(21),
            value: ValueId::new(8),
        });
        if original_use {
            loop_block.add_instruction(MirInstruction::call(
                None,
                Callee::Global(CanonicalGlobalTargetV1::Builtin(
                    CanonicalBuiltinGlobalV1::Print,
                )),
                vec![ValueId::new(8)],
                EffectMask::IO,
            ));
        }
        loop_block.set_terminator(MirInstruction::Branch {
            condition: ValueId::new(9),
            then_bb: BasicBlockId::new(1),
            else_bb: BasicBlockId::new(2),
            then_edge_args: None,
            else_edge_args: None,
        });
        body.add_block(loop_block);
        let mut exit = crate::mir::BasicBlock::new(BasicBlockId::new(2));
        exit.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(10)),
        });
        body.add_block(exit);
        let mut module = MirModule::new("phi-demand".into());
        module.add_function(body);
        let view = PublishedMirBackendView::try_new(&module).unwrap();
        let index = MapBodyIndex::from_view(&view).unwrap();
        let actions = BTreeMap::from([
            (("main", ValueId::new(7)), ProjectionAction::ExactI64(1)),
            (("main", ValueId::new(8)), ProjectionAction::Phi),
        ]);
        let original = index.original_value_demands(&actions).unwrap();
        assert_eq!(original.contains(&("main", ValueId::new(7))), original_use);
        assert_eq!(original.contains(&("main", ValueId::new(8))), original_use);
        assert!(original.contains(&("main", ValueId::new(9))));
    }
}
