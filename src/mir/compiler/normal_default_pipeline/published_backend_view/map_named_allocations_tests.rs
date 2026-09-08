//! Physical observation binding witnesses; no production C-query/source claim.
use super::super::{map_value_domains::ValueDomain, PublishedMirBackendView};
use super::*;
use crate::mir::{
    BasicBlockId, Callee, ConstValue, EffectMask, FunctionSignature, MirFunction, MirModule,
    MirType,
};
use hakorune_mir_defs::CanonicalSameModuleCallableKeyV1;
use std::collections::{BTreeMap, BTreeSet};

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
fn add(body: &mut MirFunction, instruction: MirInstruction) {
    body.blocks
        .get_mut(&BasicBlockId::new(0))
        .unwrap()
        .add_instruction(instruction);
}
fn constant(body: &mut MirFunction, dst: u32, value: ConstValue) {
    add(
        body,
        MirInstruction::Const {
            dst: ValueId::new(dst),
            value,
        },
    );
}
fn alias(body: &mut MirFunction, dst: u32, src: u32, name: &str) {
    add(
        body,
        MirInstruction::NewBox {
            dst: ValueId::new(dst),
            target: ConstructionTarget::Named(name.into()),
            args: vec![ValueId::new(src)],
        },
    );
}
fn write(body: &mut MirFunction, value: u32) {
    add(
        body,
        MirInstruction::NewBox {
            dst: ValueId::new(20),
            target: ConstructionTarget::IntrinsicMap,
            args: vec![],
        },
    );
    constant(body, 21, ConstValue::String("v".into()));
    add(
        body,
        MirInstruction::MapLiteralEntryWrite {
            receiver: ValueId::new(20),
            key: ValueId::new(21),
            value: ValueId::new(value),
        },
    );
}
fn module(body: MirFunction) -> MirModule {
    let mut module = MirModule::new("named-projection".into());
    module.add_function(body);
    module
}

#[test]
fn map_literal_named_alias_keeps_exact_float_and_original_demand_separate() {
    for original_use in [false, true] {
        for name in ["StringBox", "RenamedPhysicalInput"] {
            let mut body = function("main", 0);
            let bits = 0x7ff8_1234_5678_9abcu64;
            constant(&mut body, 7, ConstValue::Float(f64::from_bits(bits)));
            alias(&mut body, 8, 7, name);
            alias(&mut body, 9, 8, name);
            write(&mut body, 9);
            if original_use {
                add(
                    &mut body,
                    MirInstruction::Return {
                        value: Some(ValueId::new(9)),
                    },
                );
            }
            let module = module(body);
            let view = PublishedMirBackendView::try_new(&module).unwrap();
            let index = MapBodyIndex::from_view(&view)
                .unwrap()
                .with_named_allocations([
                    (("main", 0, 1), NamedAllocationConsumer::AliasOperandZero),
                    (("main", 0, 2), NamedAllocationConsumer::AliasOperandZero),
                ])
                .unwrap();
            assert_eq!(
                index.map_value_demands().unwrap(),
                BTreeSet::from([
                    ("main", ValueId::new(7)),
                    ("main", ValueId::new(8)),
                    ("main", ValueId::new(9))
                ])
            );
            let domains = index.map_value_domains().unwrap();
            assert_eq!(
                domains[&("main", ValueId::new(9))],
                BTreeSet::from([ValueDomain::F64])
            );
            let Producer::Instruction {
                instruction:
                    MirInstruction::Const {
                        value: ConstValue::Float(original),
                        ..
                    },
                ..
            } = index.producer(("main", ValueId::new(7))).unwrap()
            else {
                panic!("original Float lost")
            };
            let actions = BTreeMap::from([
                (
                    ("main", ValueId::new(7)),
                    ProjectionAction::ExactF64(original.to_bits()),
                ),
                (
                    ("main", ValueId::new(8)),
                    index
                        .named_projection_action(("main", ValueId::new(8)))
                        .unwrap(),
                ),
                (
                    ("main", ValueId::new(9)),
                    index
                        .named_projection_action(("main", ValueId::new(9)))
                        .unwrap(),
                ),
            ]);
            let row = actions[&("main", ValueId::new(7))].row(std::ptr::null(), 7, original_use);
            assert_eq!(row.payload, bits);
            let alias_row =
                actions[&("main", ValueId::new(9))].row(std::ptr::null(), 9, original_use);
            let copy_row = ProjectionAction::Copy.row(std::ptr::null(), 9, original_use);
            assert_ne!(alias_row.action, copy_row.action);
            assert_eq!(alias_row.payload, 0);
            let original = index.original_value_demands(&actions).unwrap();
            for value in [7, 8, 9] {
                assert_eq!(
                    original.contains(&("main", ValueId::new(value))),
                    original_use
                );
            }
        }
    }
}

#[test]
fn map_literal_named_binding_rejects_missing_wrong_duplicate_and_bad_alias() {
    let mut body = function("main", 0);
    constant(&mut body, 7, ConstValue::Integer(5));
    alias(&mut body, 8, 7, "UnclassifiedName");
    write(&mut body, 8);
    let module = module(body);
    let view = PublishedMirBackendView::try_new(&module).unwrap();
    let make = || MapBodyIndex::from_view(&view).unwrap();
    assert!(make()
        .map_value_demands()
        .unwrap_err()
        .contains("not-observed"));
    for site in [
        ("main", 0, 0),
        ("main", 0, 2),
        ("other", 0, 1),
        ("main", 99, 1),
    ] {
        assert!(make()
            .with_named_allocations([(site, NamedAllocationConsumer::Map)])
            .err()
            .unwrap()
            .contains("site-mismatch"));
    }
    assert!(make()
        .with_named_allocations([
            (("main", 0, 1), NamedAllocationConsumer::Map),
            (("main", 0, 1), NamedAllocationConsumer::AliasOperandZero),
        ])
        .err()
        .unwrap()
        .contains("duplicate-binding"));
    for (consumer, reason) in [
        (NamedAllocationConsumer::InvalidPlan, "invalid-plan"),
        (NamedAllocationConsumer::Unsupported, "unsupported"),
    ] {
        assert!(make()
            .with_named_allocations([(("main", 0, 1), consumer)])
            .unwrap()
            .map_value_domains()
            .unwrap_err()
            .contains(reason));
    }
    for args in [vec![], vec![ValueId::new(999)]] {
        let mut body = function("main", 0);
        add(
            &mut body,
            MirInstruction::NewBox {
                dst: ValueId::new(8),
                target: ConstructionTarget::Named("Alias".into()),
                args,
            },
        );
        let mut module = MirModule::new("bad".into());
        module.add_function(body);
        let view = PublishedMirBackendView::try_new(&module).unwrap();
        assert!(MapBodyIndex::from_view(&view)
            .unwrap()
            .with_named_allocations([(("main", 0, 0), NamedAllocationConsumer::AliasOperandZero)])
            .is_err());
    }
}

#[test]
fn map_literal_named_allocating_outcomes_ignore_spelling_and_unused_errors() {
    for consumer in [
        NamedAllocationConsumer::Array,
        NamedAllocationConsumer::DirectArray,
        NamedAllocationConsumer::Map,
        NamedAllocationConsumer::File,
        NamedAllocationConsumer::TypedObject,
    ] {
        let mut body = function("main", 0);
        constant(&mut body, 7, ConstValue::Integer(5));
        alias(&mut body, 8, 7, "StringBox");
        alias(&mut body, 9, 7, "IgnoredBadPlan");
        write(&mut body, 8);
        let module = module(body);
        let view = PublishedMirBackendView::try_new(&module).unwrap();
        let index = MapBodyIndex::from_view(&view)
            .unwrap()
            .with_named_allocations([
                (("main", 0, 1), consumer),
                (("main", 0, 2), NamedAllocationConsumer::InvalidPlan),
            ])
            .unwrap();
        assert_eq!(
            index.map_value_demands().unwrap(),
            BTreeSet::from([("main", ValueId::new(8))])
        );
        assert_eq!(
            index.map_value_domains().unwrap()[&("main", ValueId::new(8))],
            BTreeSet::from([ValueDomain::Handle])
        );
        assert_eq!(
            index
                .named_projection_action(("main", ValueId::new(8)))
                .unwrap(),
            ProjectionAction::OriginalHandle
        );
    }
}

#[test]
fn map_literal_named_alias_closes_phi_select_and_canonical_formal_cycle() {
    let key = CanonicalSameModuleCallableKeyV1::test_static_box_method("Helpers", "stash", 1);
    let name = key.mir_symbol_projection();
    let mut callee = function(&name, 1);
    let formal = callee.params[0];
    alias(&mut callee, 7, formal.0, "OpaqueAlias");
    add(
        &mut callee,
        MirInstruction::call(
            None,
            Callee::Global(key.canonical_global_target_v1().unwrap()),
            vec![ValueId::new(7)],
            EffectMask::PURE,
        ),
    );
    write(&mut callee, 7);
    let mut root = function("main", 0);
    constant(&mut root, 7, ConstValue::Float(-0.0));
    constant(&mut root, 8, ConstValue::Bool(true));
    alias(&mut root, 9, 7, "FirstAlias");
    add(
        &mut root,
        MirInstruction::Phi {
            dst: ValueId::new(10),
            type_hint: None,
            inputs: vec![
                (BasicBlockId::new(0), ValueId::new(9)),
                (BasicBlockId::new(1), ValueId::new(11)),
            ],
        },
    );
    add(
        &mut root,
        MirInstruction::Select {
            dst: ValueId::new(11),
            cond: ValueId::new(8),
            then_val: ValueId::new(10),
            else_val: ValueId::new(9),
        },
    );
    add(
        &mut root,
        MirInstruction::call(
            None,
            Callee::Global(key.canonical_global_target_v1().unwrap()),
            vec![ValueId::new(11)],
            EffectMask::PURE,
        ),
    );
    let mut module = module(root);
    module.add_cataloged_box_method(key, callee).unwrap();
    let view = PublishedMirBackendView::try_new(&module).unwrap();
    let index = MapBodyIndex::from_view(&view)
        .unwrap()
        .with_named_allocations([
            (("main", 0, 2), NamedAllocationConsumer::AliasOperandZero),
            (
                (name.as_str(), 0, 0),
                NamedAllocationConsumer::AliasOperandZero,
            ),
        ])
        .unwrap();
    let domains = index.map_value_domains().unwrap();
    for key in [
        ("main", ValueId::new(7)),
        ("main", ValueId::new(9)),
        ("main", ValueId::new(10)),
        ("main", ValueId::new(11)),
        (name.as_str(), formal),
        (name.as_str(), ValueId::new(7)),
    ] {
        assert_eq!(domains[&key], BTreeSet::from([ValueDomain::F64]));
    }
}

#[path = "map_named_query_tests.rs"]
mod query;

#[path = "map_projection_tests.rs"]
mod projection;
