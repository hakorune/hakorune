//! Real Script source through both completed-root finishing consumers.
use super::super::normal_default_root_catalog_lifecycle_tests::{callable_source, session};
use super::*;
use crate::mir::builder::{CallableMainMaterializationPolicyV1, NormalRuntimeInputSnapshotV1};
use crate::parser::ParserBuildConfig;

fn completed(source: &str) -> CompletedNormalDefaultRootCatalogLifecycleV1 {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    session()
        .complete_normal_default_program_root_catalog_lifecycle(
            callable_source(source, ParserBuildConfig::default()),
            CallableMainMaterializationPolicyV1::Omitted,
            NormalRuntimeInputSnapshotV1::empty(),
        )
        .expect("source-backed Script completion")
}

#[test]
fn script_source_survives_scope_and_reaches_both_finishing_consumers() {
    for terminal in ["return 30", "return"] {
        let source = format!(
            "local first: Array<i8> = [10, 20]\nlocal alias = first\n\
             local second: Array<u32> = []\n{terminal}"
        );
        let diagnostic = completed(&source);
        assert!(matches!(
            &diagnostic.root_validation,
            RootValidation::Script { .. }
        ));
        let (_, module, validate) = diagnostic.into_parts();
        validate(&module).expect("diagnostic source retention");
        let artifact = completed(&source);
        assert!(matches!(
            &artifact.root_validation,
            RootValidation::Script { .. }
        ));
        let (_, module, validate) = artifact.into_artifact_parts();
        assert!(
            validate(&module)
                .expect("artifact source retention")
                .is_none(),
            "Script retention must not synthesize an App Main/Birth handoff"
        );
    }
}

#[test]
fn both_finishing_consumers_reject_missing_or_foreign_retained_root() {
    for artifact in [false, true] {
        for mutation in 0..3 {
            let completed = completed("local a: Array<i64> = [10, 20]\nreturn 30");
            let key = completed.root_validation.key().unwrap().to_owned();
            let mutate = |module: &mut MirModule| match mutation {
                0 => {
                    module.functions.remove(&key);
                }
                1 => {
                    module.functions.get_mut(&key).unwrap().entry_block =
                        crate::mir::BasicBlockId::new(u32::MAX);
                }
                2 => {
                    module.functions.get_mut(&key).unwrap().signature.name = "foreign".into();
                }
                _ => unreachable!(),
            };
            let error = if artifact {
                let (_, mut module, validate) = completed.into_artifact_parts();
                mutate(&mut module);
                validate(&module).unwrap_err()
            } else {
                let (_, mut module, validate) = completed.into_parts();
                mutate(&mut module);
                validate(&module).unwrap_err()
            };
            assert!(
                error.contains(match mutation {
                    0 => "root-missing",
                    1 => "script-root-owner-drift",
                    2 => "root-key-drift",
                    _ => unreachable!(),
                }),
                "{error}"
            );
        }
    }
}
