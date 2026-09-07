use super::normal_default_root_catalog_lifecycle_tests::{callable_source, session};
use crate::mir::builder::{CallableMainMaterializationPolicyV1, NormalRuntimeInputSnapshotV1};
use crate::parser::ParserBuildConfig;

#[test]
fn final_handoff_retains_exact_source_for_alias_and_multiple_homes() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    for body in [
        "local pair = new Pair(10, 20) local alias = pair return alias.left + alias.right",
        "local first = new Pair(10, 20) local alias = first local second = new Pair(30, 40) return alias.left + alias.right",
    ] {
        let source = callable_source(
            &format!(
                "box Pair {{ left: i64 right: i64
                    birth(left, right) {{ me.left = left me.right = right }} }}
                 static box Main {{ main() {{ {body} }} }}"
            ),
            ParserBuildConfig::default(),
        );
        let completed = session()
            .complete_normal_default_program_root_catalog_lifecycle(
                source,
                CallableMainMaterializationPolicyV1::Omitted,
                NormalRuntimeInputSnapshotV1::empty(),
            )
            .expect("selected Pair source must lower");
        let (_, module, validate) = completed.into_artifact_parts();
        let handoff = validate(&module)
            .expect("final artifact validation")
            .expect("selected Pair root handoff");
        let source = handoff.root_source().expect("retained source relation");
        assert!(matches!(
            handoff.root_result(),
            Some(crate::mir::normal_callable_semantic_package::FinalizedRootResultAbiV1::I64AddReturn { owner })
                if source.terminal_i64_add().expect("I64 source relation").owner() == owner
        ));
        assert_eq!(
            handoff.birth_keys().expect("callable keys"),
            [hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::birth_constructor("Pair", 2)],
            "several New sites retain one canonical Birth definition"
        );
        assert_eq!(handoff.births().expect("callable births").len(), 1);
        assert_eq!(handoff.births().expect("callable births")[0].object().declaration_index(), 0);
        let _identity = source.app_main_identity();
    }
}

#[test]
fn artifact_validation_retains_issued_root_birth_handoff() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let source = callable_source(
        include_str!("../../../apps/typed-object-birth-min/main.hako"),
        ParserBuildConfig::default(),
    );
    let completed = session()
        .complete_normal_default_program_root_catalog_lifecycle(
            source,
            CallableMainMaterializationPolicyV1::Omitted,
            NormalRuntimeInputSnapshotV1::empty(),
        )
        .expect("source-backed Pair must lower");
    let (_, module, validate) = completed.into_artifact_parts();
    let root = module.functions.get("main").expect("main root");
    let fields = root
        .blocks
        .values()
        .flat_map(|block| block.all_instructions())
        .filter_map(|instruction| match instruction {
            crate::mir::MirInstruction::ObjectFieldGet { dst, .. } => Some(*dst),
            _ => None,
        })
        .collect::<Vec<_>>();
    let adds = root
        .blocks
        .values()
        .flat_map(|block| block.all_instructions())
        .filter_map(|instruction| match instruction {
            crate::mir::MirInstruction::BinOp {
                dst,
                op: crate::mir::BinaryOp::Add,
                lhs,
                rhs,
            } => Some((*dst, *lhs, *rhs)),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(fields.len(), 2, "terminal consumer owns two field reads");
    assert_eq!(adds.len(), 1, "terminal consumer owns one Add");
    assert_eq!([adds[0].1, adds[0].2], [fields[0], fields[1]]);
    assert!(root.blocks.values().flat_map(|block| block.all_instructions()).any(|instruction|
        matches!(instruction, crate::mir::MirInstruction::Return { value: Some(value) } if *value == adds[0].0)));
    let handoff = validate(&module)
        .expect("artifact handoff must validate")
        .expect("Pair root has a handoff");
    assert_eq!(handoff.root_key(), "main");
    let root_source = handoff
        .root_source()
        .expect("Pair root retains its issued source relation");
    assert!(matches!(
        handoff.root_result(),
        Some(crate::mir::normal_callable_semantic_package::FinalizedRootResultAbiV1::I64AddReturn { owner })
            if root_source.terminal_i64_add().expect("I64 source relation").owner() == owner
    ));
    let _identity = root_source.app_main_identity();
    assert_eq!(
        handoff.birth_keys().expect("callable keys"),
        [hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::birth_constructor("Pair", 2)],
    );
}

#[test]
fn selected_bare_return_emits_value_free_root_terminator() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let source = callable_source(
        "box Pair { left: i64 right: i64 birth(left, right) { me.left = left me.right = right } } static box Main { main() { local pair = new Pair(10, 20) return } }",
        ParserBuildConfig::default(),
    );
    let completed = session()
        .complete_normal_default_program_root_catalog_lifecycle(
            source,
            CallableMainMaterializationPolicyV1::Omitted,
            NormalRuntimeInputSnapshotV1::empty(),
        )
        .expect("selected bare return must lower");
    let (_, module, validate) = completed.into_artifact_parts();
    let root = module.functions.get("main").expect("root");
    assert_eq!(
        root.blocks
            .values()
            .flat_map(|block| block.all_instructions())
            .filter(|instruction| matches!(
                instruction,
                crate::mir::MirInstruction::Return { value: None }
            ))
            .count(),
        1
    );
    assert!(!root
        .blocks
        .values()
        .flat_map(|block| block.all_instructions())
        .any(|instruction| matches!(
            instruction,
            crate::mir::MirInstruction::Return { value: Some(_) }
        )));
    let handoff = validate(&module)
        .expect("artifact validation")
        .expect("Pair handoff");
    let source = handoff.root_source().expect("Unit source relation");
    assert!(matches!(handoff.root_result(),
        Some(crate::mir::normal_callable_semantic_package::FinalizedRootResultAbiV1::UnitReturn { owner })
            if source.terminal_unit_return().is_some_and(|terminal| terminal.owner() == owner)));
}

#[test]
fn selected_bare_return_rejects_value_bearing_terminal_drift() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let source = callable_source(
        "box Pair { left: i64 right: i64 birth(left, right) { me.left = left me.right = right } } static box Main { main() { local pair = new Pair(10, 20) return } }",
        ParserBuildConfig::default(),
    );
    let completed = session()
        .complete_normal_default_program_root_catalog_lifecycle(
            source,
            CallableMainMaterializationPolicyV1::Omitted,
            NormalRuntimeInputSnapshotV1::empty(),
        )
        .expect("selected bare return must lower");
    let (_, mut module, validate) = completed.into_artifact_parts();
    let root = module.functions.get_mut("main").expect("root");
    let terminator = root
        .blocks
        .values_mut()
        .find_map(|block| {
            matches!(
                block.terminator,
                Some(crate::mir::MirInstruction::Return { value: None })
            )
            .then_some(&mut block.terminator)
        })
        .expect("value-free return");
    *terminator = Some(crate::mir::MirInstruction::Return {
        value: Some(crate::mir::ValueId::new(0)),
    });
    let error = validate(&module).expect_err("value-bearing drift must reject");
    assert!(error.contains("unit-return-control-drift"), "{error}");
}
