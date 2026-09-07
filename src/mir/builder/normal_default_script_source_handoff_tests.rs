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
        let handoff = validate(&module)
            .expect("artifact source retention")
            .expect("selected Array has a final handoff");
        let array = handoff.script_array().expect("Script variant");
        assert_eq!(array.acquisition_count(), 2);
        array
            .validate_root_binding(&module.functions[handoff.root_key()])
            .unwrap();
        assert!(handoff.root_source().is_none());
        assert!(handoff.root_result().is_none());
        assert!(handoff.births().is_none());
        assert!(handoff.birth_keys().is_none());
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

#[test]
fn array_emission_mutations_reject_in_both_finishing_consumers() {
    use crate::mir::{ConstructionTarget, MirInstruction, ValueId};
    for artifact in [false, true] {
        for mutation in 0..11 {
            let completed = completed("local a: Array<i64> = [10, 20]\nlocal alias = a\nreturn 30");
            let key = completed.root_validation.key().unwrap().to_owned();
            let mutate = |module: &mut MirModule| {
                let root = module.functions.get_mut(&key).unwrap();
                let block = root.blocks.get_mut(&root.entry_block).unwrap();
                let writes: Vec<_> = block
                    .instructions
                    .iter()
                    .enumerate()
                    .filter_map(|(index, inst)| {
                        matches!(inst, MirInstruction::ArrayElementWrite { .. }).then_some(index)
                    })
                    .collect();
                match mutation {
                    0 => {
                        if let Some(MirInstruction::NewBox { target, .. }) = block
                            .instructions
                            .iter_mut()
                            .find(|inst| matches!(inst, MirInstruction::NewBox { .. }))
                        {
                            *target = ConstructionTarget::Named("ArrayBox".into());
                        }
                    }
                    1 => {
                        if let Some(MirInstruction::ArrayStateContractClaim {
                            contract_id, ..
                        }) = block.instructions.iter_mut().find(|inst| {
                            matches!(inst, MirInstruction::ArrayStateContractClaim { .. })
                        }) {
                            contract_id.push_str(":foreign");
                        }
                    }
                    2 => {
                        if let MirInstruction::ArrayElementWrite { value, .. } =
                            &mut block.instructions[writes[0]]
                        {
                            *value = ValueId::new(u32::MAX);
                        }
                    }
                    3 => {
                        block.instructions.remove(writes[0]);
                    }
                    4 => {
                        block
                            .instructions
                            .push(block.instructions[writes[0]].clone());
                    }
                    5 => {
                        block.instructions.swap(writes[0], writes[1]);
                    }
                    6 => {
                        let local = block
                            .instructions
                            .iter()
                            .position(|inst| matches!(inst, MirInstruction::Copy { .. }))
                            .unwrap();
                        block.instructions.remove(local);
                    }
                    7 => {
                        root.metadata.typed_array_contract_sources.clear();
                    }
                    8 => {
                        block.terminator = None;
                    }
                    9 => {
                        block.terminator = Some(MirInstruction::Return {
                            value: Some(ValueId::new(u32::MAX)),
                        });
                    }
                    10 => {
                        if let Some(MirInstruction::Const { value, .. }) =
                            block.instructions.last_mut()
                        {
                            *value = crate::mir::ConstValue::Integer(31);
                        }
                    }
                    _ => unreachable!(),
                }
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
                error.contains("[script-array/emission/"),
                "mutation {mutation}: {error}"
            );
        }
    }
}

#[test]
fn array_source_binding_survives_actual_compiler_finishing_with_optimization() {
    use crate::mir::{MirCompiler, NormalCompileRequestV1};
    use crate::runner::modes::common_util::normal_callable::{
        materialize_normal_callable_program_v1, NormalCallableMaterializationOutcomeV1,
    };
    for optimize in [false, true] {
        for (text, expected_stop) in [
            (
                "local seed = 10\nlocal a: Array<i64> = [10, 10]\nreturn 30",
                None,
            ),
            (
                "local a: Array<u8> = []\nlocal b: Array<i64> = [1.5, 1.5]\nreturn 30",
                None,
            ),
            ("local a: Array<i64> = []\nreturn", None),
            // The original f32 probe is outside the seven admitted annotations.
            // Keep its upstream Stop; it is not finishing evidence.
            (
                "local a: Array<u8> = []\nlocal b: Array<f32> = [1.5, 1.5]\nreturn 30",
                Some("ObservationDeferred"),
            ),
        ] {
            let NormalCallableMaterializationOutcomeV1::SourceBacked(source) =
                materialize_normal_callable_program_v1(
                    text.to_owned(),
                    ParserBuildConfig::default(),
                )
                .unwrap()
            else {
                panic!("source authority required")
            };
            let request = NormalCompileRequestV1::for_mir_mode_callable_source(
                source,
                None,
                Default::default(),
            );
            let result = MirCompiler::with_options(optimize).compile_normal(request);
            if let Some(stop) = expected_stop {
                let error = result
                    .err()
                    .expect("unsupported annotation must stop upstream");
                assert!(error.contains(stop), "{text}: {error}");
                continue;
            }
            let result =
                result.unwrap_or_else(|error| panic!("optimize={optimize}, {text}: {error}"));
            assert!(
                result.verification_result.is_ok(),
                "{:?}",
                result.verification_result
            );
            let copies = result
                .module
                .functions
                .values()
                .flat_map(|root| root.blocks.values())
                .flat_map(|block| &block.instructions)
                .filter(|instruction| {
                    matches!(instruction, crate::mir::MirInstruction::Copy { .. })
                })
                .count();
            if optimize {
                assert_eq!(copies, 0, "DCE must actually remove dead Local copies");
            } else {
                assert!(
                    copies > 0,
                    "unoptimized Local copies witness the transformation"
                );
            }
        }
    }
}

#[test]
fn script_array_artifact_requires_finishing_and_preserves_unissued_distinction() {
    let finished = completed("local a: Array<i64> = []\nreturn 30");
    let RootValidation::Script { source, entry, .. } = finished.root_validation else {
        panic!("Script source required")
    };
    let error = source.into_array_artifact(entry).unwrap_err();
    assert!(error.contains("artifact-before-finishing"), "{error}");
    let (_, module, validate) = completed("local scalar = 1\nreturn 30").into_artifact_parts();
    assert!(
        validate(&module).unwrap().is_none(),
        "unissued Array is not an empty Array product"
    );
}
