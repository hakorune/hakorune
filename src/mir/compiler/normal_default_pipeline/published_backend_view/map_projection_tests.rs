//! Complete private projection witnesses; not C/source capability evidence.
use super::*;
use crate::mir::boxed_sum_abi_plan::{refresh_module_boxed_sum_abi_plans, BoxedSumPayloadStorage};
use crate::mir::{MirEnumDecl, MirEnumVariantDecl};

#[test]
fn map_literal_complete_actions_keep_exact_constants_and_float_original_stop() {
    for (value, action) in [
        (ConstValue::Integer(-30), ProjectionAction::ExactI64(-30)),
        (ConstValue::Bool(true), ProjectionAction::ExactBool(true)),
        (ConstValue::Float(-0.0), ProjectionAction::ExactF64(1 << 63)),
        (
            ConstValue::Float(f64::from_bits(0x7ff8_1234_5678_9abc)),
            ProjectionAction::ExactF64(0x7ff8_1234_5678_9abc),
        ),
        (ConstValue::Null, ProjectionAction::ExactVoid),
        (ConstValue::Void, ProjectionAction::ExactVoid),
        (
            ConstValue::String("a\0b".into()),
            ProjectionAction::OriginalHandle,
        ),
    ] {
        let mut root = function("main", 0);
        constant(&mut root, 7, value);
        write(&mut root, 7);
        let module = module(root);
        let view = PublishedMirBackendView::try_new(&module).unwrap();
        let index = MapBodyIndex::from_view(&view).unwrap();
        assert_eq!(
            index.map_projection_actions().unwrap(),
            BTreeMap::from([(("main", ValueId::new(7)), action)])
        );
    }
    let mut root = function("main", 0);
    constant(&mut root, 7, ConstValue::Float(1.25));
    alias(&mut root, 8, 7, "StringBox");
    write(&mut root, 8);
    add(
        &mut root,
        MirInstruction::Return {
            value: Some(ValueId::new(8)),
        },
    );
    let module = module(root);
    let view = PublishedMirBackendView::try_new(&module).unwrap();
    let index = MapBodyIndex::from_view(&view)
        .unwrap()
        .with_named_allocations([(("main", 0, 1), NamedAllocationConsumer::AliasOperandZero)])
        .unwrap();
    assert!(index
        .map_projection_actions()
        .unwrap_err()
        .contains("original-float-unavailable"));
}

fn boxed(payload_type: MirType, demanded: u32, unit: bool) -> MirModule {
    let mut root = function("main", 0);
    constant(&mut root, 7, ConstValue::Integer(1));
    add(
        &mut root,
        MirInstruction::VariantMake {
            dst: ValueId::new(8),
            enum_name: "Envelope".into(),
            variant: "Value".into(),
            tag: 0,
            payload: (!unit).then_some(ValueId::new(7)),
            payload_type: Some(payload_type.clone()),
        },
    );
    add(
        &mut root,
        MirInstruction::VariantTag {
            dst: ValueId::new(9),
            value: ValueId::new(8),
            enum_name: "Envelope".into(),
        },
    );
    add(
        &mut root,
        MirInstruction::VariantProject {
            variant: "Value".into(),
            dst: ValueId::new(10),
            value: ValueId::new(8),
            enum_name: "Envelope".into(),
            tag: 0,
            payload_type: Some(payload_type),
        },
    );
    write(&mut root, demanded);
    let mut module = module(root);
    module.metadata.enum_decls.insert(
        "Envelope".into(),
        MirEnumDecl {
            type_parameters: vec!["T".into()],
            variants: vec![MirEnumVariantDecl {
                name: "Value".into(),
                payload_type_name: (!unit).then_some("T".into()),
            }],
        },
    );
    refresh_module_boxed_sum_abi_plans(&mut module);
    module
}

#[test]
fn map_literal_boxed_actions_and_domains_share_exact_site_storage() {
    for (ty, domain, expected) in [
        (
            MirType::Integer,
            ValueDomain::I64,
            ProjectionAction::OriginalI64,
        ),
        (
            MirType::Bool,
            ValueDomain::Bool,
            ProjectionAction::OriginalBoolI64,
        ),
        (
            MirType::String,
            ValueDomain::String,
            ProjectionAction::OriginalHandle,
        ),
        (
            MirType::Box("Payload".into()),
            ValueDomain::Handle,
            ProjectionAction::OriginalHandle,
        ),
    ] {
        for (value, domain, action) in [
            (8, ValueDomain::Handle, ProjectionAction::OriginalHandle),
            (9, ValueDomain::I64, ProjectionAction::OriginalI64),
            (10, domain, expected),
        ] {
            let module = boxed(ty.clone(), value, false);
            let view = PublishedMirBackendView::try_new(&module).unwrap();
            let index = MapBodyIndex::from_view(&view).unwrap();
            let key = ("main", ValueId::new(value));
            assert_eq!(
                index.map_value_domains().unwrap()[&key],
                BTreeSet::from([domain])
            );
            assert_eq!(index.map_projection_actions().unwrap()[&key], action);
            let row = action.row(std::ptr::null(), value, false);
            assert_eq!(row.encoding, 1); // Bool project is i64, not comparison i1.
        }
    }
    let module = boxed(MirType::Void, 8, true);
    let view = PublishedMirBackendView::try_new(&module).unwrap();
    let index = MapBodyIndex::from_view(&view).unwrap();
    assert_eq!(
        index.map_projection_actions().unwrap()[&("main", ValueId::new(8))],
        ProjectionAction::OriginalHandle
    ); // Unit construction still allocates.
}

#[test]
fn map_literal_boxed_missing_ambiguous_or_wrong_storage_never_guesses() {
    for mutation in 0..3 {
        let mut module = boxed(MirType::Bool, 10, false);
        match mutation {
            0 => module.metadata.boxed_sum_abi_plans.clear(),
            1 => module
                .metadata
                .boxed_sum_abi_plans
                .extend(module.metadata.boxed_sum_abi_plans.clone()),
            _ => {
                for plan in &mut module.metadata.boxed_sum_abi_plans {
                    for variant in &mut plan.variants {
                        variant.payload_storage = BoxedSumPayloadStorage::Handle;
                    }
                }
            }
        }
        let view = PublishedMirBackendView::try_new(&module).unwrap();
        let index = MapBodyIndex::from_view(&view).unwrap();
        assert!(index
            .map_projection_actions()
            .unwrap_err()
            .contains("producer-projection-unavailable"));
    }
    for ty in [MirType::Float, MirType::Void, MirType::Unknown] {
        let module = boxed(ty, 10, false);
        let view = PublishedMirBackendView::try_new(&module).unwrap();
        assert!(MapBodyIndex::from_view(&view)
            .unwrap()
            .map_projection_actions()
            .is_err());
    }
    // Unsupported undemanded boxed sites are not whole-module early failures.
    let module = boxed(MirType::Float, 7, false);
    let view = PublishedMirBackendView::try_new(&module).unwrap();
    assert!(MapBodyIndex::from_view(&view)
        .unwrap()
        .map_projection_actions()
        .is_ok());
}

#[test]
fn map_literal_complete_actions_use_checked_canonical_call_and_formal() {
    let key = CanonicalSameModuleCallableKeyV1::test_static_box_method("Helpers", "store", 1);
    let name = key.mir_symbol_projection();
    let mut callee = function(&name, 1);
    let formal = callee.params[0];
    write(&mut callee, formal.as_u32());
    let mut root = function("main", 0);
    constant(&mut root, 7, ConstValue::Bool(true));
    add(
        &mut root,
        MirInstruction::call(
            Some(ValueId::new(8)),
            Callee::Global(key.canonical_global_target_v1().unwrap()),
            vec![ValueId::new(7)],
            EffectMask::PURE,
        ),
    );
    write(&mut root, 8);
    let mut module = module(root);
    module.add_cataloged_box_method(key, callee).unwrap();
    let view = PublishedMirBackendView::try_new(&module).unwrap();
    let index = MapBodyIndex::from_view(&view).unwrap();
    let actions = index.map_projection_actions().unwrap();
    assert_eq!(
        actions[&("main", ValueId::new(8))],
        ProjectionAction::OriginalI64
    );
    assert_eq!(
        actions[&("main", ValueId::new(7))],
        ProjectionAction::ExactBool(true)
    );
    assert_eq!(
        actions[&(name.as_str(), formal)],
        ProjectionAction::Formal(0)
    );
    module
        .get_function_mut(&name)
        .unwrap()
        .signature
        .return_type = MirType::String;
    assert!(PublishedMirBackendView::try_new(&module).is_err());
}

#[test]
fn map_literal_complete_actions_keep_cycles_and_owned_copy_boundaries() {
    for seeded in [false, true] {
        let mut root = function("main", 0);
        constant(&mut root, 7, ConstValue::Bool(true));
        add(
            &mut root,
            MirInstruction::Phi {
                dst: ValueId::new(8),
                inputs: vec![(
                    BasicBlockId::new(0),
                    ValueId::new(if seeded { 7 } else { 8 }),
                )],
                type_hint: None,
            },
        );
        write(&mut root, 8);
        let module = module(root);
        let view = PublishedMirBackendView::try_new(&module).unwrap();
        let index = MapBodyIndex::from_view(&view).unwrap();
        if seeded {
            assert_eq!(
                index.map_projection_actions().unwrap()[&("main", ValueId::new(8))],
                ProjectionAction::Phi
            );
        } else {
            assert!(index
                .map_projection_actions()
                .unwrap_err()
                .contains("value-domain-unresolved"));
        }
    }
    let mut root = function("main", 0);
    constant(&mut root, 7, ConstValue::Integer(1));
    add(
        &mut root,
        MirInstruction::CopyOwned {
            dst: ValueId::new(8),
            src: ValueId::new(7),
        },
    );
    write(&mut root, 8);
    let module = module(root);
    let view = PublishedMirBackendView::try_new(&module).unwrap();
    assert!(MapBodyIndex::from_view(&view)
        .unwrap()
        .map_projection_actions()
        .unwrap_err()
        .contains("producer-projection-unavailable"));
}
