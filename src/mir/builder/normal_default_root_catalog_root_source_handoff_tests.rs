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
                if source.terminal().owner() == owner
        ));
        assert_eq!(
            handoff.birth_keys(),
            [hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::birth_constructor("Pair", 2)],
            "several New sites retain one canonical Birth definition"
        );
        assert_eq!(handoff.births().len(), 1);
        assert_eq!(handoff.births()[0].object().declaration_index(), 0);
        let _identity = source.app_main_identity();
    }
}
