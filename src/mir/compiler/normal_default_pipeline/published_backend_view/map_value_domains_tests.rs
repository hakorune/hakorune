//! Physical closure witnesses; no source selection, frame admission or C claims.
use super::super::PublishedMirBackendView;
use super::*;
use crate::mir::{
    BasicBlockId, Callee, CompareOp, ConstValue, EffectMask, FunctionSignature, MirFunction, MirModule,
    MirType, ValueId,
};
use hakorune_mir_defs::CanonicalSameModuleCallableKeyV1;

fn function(name: &str, params: usize) -> MirFunction {
    MirFunction::new(
        FunctionSignature {
            name: name.into(),
            params: vec![MirType::Unknown; params],
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
fn module(body: MirFunction) -> MirModule {
    let mut module = MirModule::new("map-domains".into());
    module.add_function(body);
    module
}

#[test]
fn map_literal_domain_selection_distinguishes_equal_result_representations() {
    for (literal, expected) in [
        (ConstValue::Integer(7), PhysicalOperation::I64Compare),
        (ConstValue::Bool(true), PhysicalOperation::BoolCompare),
        (
            ConstValue::String("s".into()),
            PhysicalOperation::StringCompare,
        ),
    ] {
        let mut body = function("main", 0);
        constant(&mut body, 7, literal.clone());
        constant(&mut body, 8, literal);
        append(
            &mut body,
            MirInstruction::Compare {
                dst: ValueId::new(9),
                op: CompareOp::Eq,
                lhs: ValueId::new(7),
                rhs: ValueId::new(8),
            },
        );
        write(&mut body, ValueId::new(9));
        let module = module(body);
        let view = PublishedMirBackendView::try_new(&module).unwrap();
        let index = MapBodyIndex::from_view(&view).unwrap();
        assert_eq!(
            index.map_value_domains().unwrap()[&("main", ValueId::new(9))],
            BTreeSet::from([ValueDomain::Bool])
        );
        assert_eq!(
            index.map_physical_operations().unwrap()[&("main", ValueId::new(9))],
            expected
        );
    }
}

#[test]
fn map_literal_domain_provisional_bool_does_not_admit_bad_inputs() {
    for literal in [ConstValue::Float(-0.0), ConstValue::Null, ConstValue::Void] {
        let mut body = function("main", 0);
        constant(&mut body, 7, literal);
        append(
            &mut body,
            MirInstruction::UnaryOp {
                dst: ValueId::new(8),
                op: UnaryOp::Not,
                operand: ValueId::new(7),
            },
        );
        write(&mut body, ValueId::new(8));
        let module = module(body);
        let view = PublishedMirBackendView::try_new(&module).unwrap();
        let index = MapBodyIndex::from_view(&view).unwrap();
        assert_eq!(
            index.map_value_domains().unwrap()[&("main", ValueId::new(8))],
            BTreeSet::from([ValueDomain::Bool])
        );
        assert!(index
            .map_physical_operations()
            .unwrap_err()
            .contains("operation-input-domain"));
    }
}

#[test]
fn map_literal_domain_missing_named_observation_rejects_select_union() {
    let mut body = function("main", 0);
    constant(&mut body, 6, ConstValue::Bool(true));
    constant(&mut body, 7, ConstValue::Integer(1));
    append(
        &mut body,
        MirInstruction::NewBox {
            dst: ValueId::new(8),
            target: ConstructionTarget::Named("MapBox".into()),
            args: vec![],
        },
    );
    append(
        &mut body,
        MirInstruction::Select {
            dst: ValueId::new(9),
            cond: ValueId::new(6),
            then_val: ValueId::new(7),
            else_val: ValueId::new(8),
        },
    );
    append(
        &mut body,
        MirInstruction::Compare {
            dst: ValueId::new(10),
            op: CompareOp::Eq,
            lhs: ValueId::new(9),
            rhs: ValueId::new(7),
        },
    );
    write(&mut body, ValueId::new(10));
    let module = module(body);
    let view = PublishedMirBackendView::try_new(&module).unwrap();
    let index = MapBodyIndex::from_view(&view).unwrap();
    assert!(index
        .map_value_domains()
        .unwrap_err()
        .contains("named-not-observed"));
    assert!(index.map_physical_operations().is_err());
}

#[test]
fn map_literal_domain_closes_recursive_formal_before_selecting_operation() {
    for seeds in [
        vec![],
        vec![ConstValue::Integer(1)],
        vec![ConstValue::Integer(1), ConstValue::String("s".into())],
    ] {
        let key = CanonicalSameModuleCallableKeyV1::test_static_box_method("Helpers", "stash", 1);
        let name = key.mir_symbol_projection();
        let mut body = function(&name, 1);
        let formal = body.params[0];
        // Recursive actual/formal SCC. An incoming root seed must propagate
        // through the entire cycle; no seed is not an Integer default.
        append(
            &mut body,
            MirInstruction::call(
                None,
                Callee::Global(key.canonical_global_target_v1().unwrap()),
                vec![formal],
                EffectMask::PURE,
            ),
        );
        append(
            &mut body,
            MirInstruction::Compare {
                dst: ValueId::new(7),
                op: CompareOp::Eq,
                lhs: formal,
                rhs: formal,
            },
        );
        write(&mut body, ValueId::new(7));
        let mut module = MirModule::new("recursive-domains".into());
        module.add_cataloged_box_method(key.clone(), body).unwrap();
        for (i, seed) in seeds.iter().enumerate() {
            let mut caller = function(&format!("caller{i}"), 0);
            constant(&mut caller, 7, seed.clone());
            append(
                &mut caller,
                MirInstruction::call(
                    None,
                    Callee::Global(key.canonical_global_target_v1().unwrap()),
                    vec![ValueId::new(7)],
                    EffectMask::PURE,
                ),
            );
            module.add_function(caller);
        }
        let view = PublishedMirBackendView::try_new(&module).unwrap();
        let index = MapBodyIndex::from_view(&view).unwrap();
        let domain = index.map_value_domains().unwrap()[&(name.as_str(), formal)].clone();
        let expected = match seeds.len() {
            0 => BTreeSet::from([ValueDomain::Unresolved]),
            1 => BTreeSet::from([ValueDomain::I64]),
            _ => BTreeSet::from([ValueDomain::I64, ValueDomain::String]),
        };
        assert_eq!(domain, expected);
        let operations = index.map_physical_operations();
        if seeds.len() == 1 {
            assert_eq!(
                operations.unwrap()[&(name.as_str(), ValueId::new(7))],
                PhysicalOperation::I64Compare
            );
        } else {
            assert!(operations.is_err());
        }
    }
}

#[test]
fn map_literal_domain_seeded_phi_copy_cycle_retains_f64_domain() {
    let mut body = function("main", 0);
    // Seed sorts after the PHI and Copy, exercising an initially empty cycle.
    constant(&mut body, 19, ConstValue::Float(-0.0));
    constant(&mut body, 18, ConstValue::Bool(false));
    write(&mut body, ValueId::new(19));
    body.blocks
        .get_mut(&BasicBlockId::new(0))
        .unwrap()
        .set_terminator(MirInstruction::Jump {
            target: BasicBlockId::new(1),
            edge_args: None,
        });
    let mut block = crate::mir::BasicBlock::new(BasicBlockId::new(1));
    block.add_instruction(MirInstruction::Phi {
        dst: ValueId::new(7),
        inputs: vec![
            (BasicBlockId::new(0), ValueId::new(19)),
            (BasicBlockId::new(1), ValueId::new(8)),
        ],
        type_hint: None,
    });
    block.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(8),
        src: ValueId::new(7),
    });
    block.add_instruction(MirInstruction::MapLiteralEntryWrite {
        receiver: ValueId::new(20),
        key: ValueId::new(21),
        value: ValueId::new(8),
    });
    block.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(18),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });
    body.add_block(block);
    let mut exit = crate::mir::BasicBlock::new(BasicBlockId::new(2));
    exit.set_terminator(MirInstruction::Return { value: None });
    body.add_block(exit);
    let module = module(body);
    let view = PublishedMirBackendView::try_new(&module).unwrap();
    let index = MapBodyIndex::from_view(&view).unwrap();
    let domains = index.map_value_domains().unwrap();
    for value in [7, 8, 19] {
        assert_eq!(
            domains[&("main", ValueId::new(value))],
            BTreeSet::from([ValueDomain::F64])
        );
    }
    assert!(index.map_physical_operations().unwrap().is_empty());
}

#[test]
fn map_literal_domain_string_concat_does_not_accept_generic_handle() {
    for use_handle in [false, true] {
        let mut body = function("main", 0);
        constant(&mut body, 7, ConstValue::String("a".into()));
        if use_handle {
            append(
                &mut body,
                MirInstruction::NewBox {
                    dst: ValueId::new(8),
                    target: ConstructionTarget::IntrinsicMap,
                    args: vec![],
                },
            );
        } else {
            constant(&mut body, 8, ConstValue::String("b".into()));
        }
        append(
            &mut body,
            MirInstruction::BinOp {
                dst: ValueId::new(9),
                op: BinaryOp::Add,
                lhs: ValueId::new(7),
                rhs: ValueId::new(8),
            },
        );
        write(&mut body, ValueId::new(9));
        let module = module(body);
        let view = PublishedMirBackendView::try_new(&module).unwrap();
        let index = MapBodyIndex::from_view(&view).unwrap();
        let operations = index.map_physical_operations();
        if use_handle {
            assert!(operations.is_err());
        } else {
            assert_eq!(
                operations.unwrap()[&("main", ValueId::new(9))],
                PhysicalOperation::StringConcat
            );
        }
    }
}
