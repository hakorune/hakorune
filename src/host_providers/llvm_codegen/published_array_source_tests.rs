//! Natural-source cutover witnesses; no manual MIR or dummy selection Call.
use super::*;
use crate::mir::{ConstructionTarget, MirCompiler, MirInstruction, NormalCompileRequestV1};

#[test]
#[ignore = "requires selected C FFI and release kernel"]
fn intrinsic_array_untyped_source_exe_and_linked_object() {
    std::thread::Builder::new()
        .stack_size(32 * 1024 * 1024)
        .spawn(|| {
            crate::runtime::ring0::ensure_global_ring0_initialized();
            for (case, source, allocations, writes) in [
                ("empty", "local a = []\nreturn 30", 1, 0),
                ("populated", "local a = [10, 20]\nreturn 30", 1, 2),
                ("nested", "local a = [[10], [20]]\nreturn 30", 3, 4),
                (
                    "shadow",
                    "box ArrayBox { birth() {} }\nlocal a = []\nreturn 30",
                    1,
                    0,
                ),
                (
                    "generic-shadow",
                    "box ArrayBox<T> { birth() {} }\nlocal a = []\nreturn 30",
                    1,
                    0,
                ),
            ] {
                use crate::runner::modes::common_util::normal_callable::{
                    materialize_normal_callable_program_v1, NormalCallableMaterializationOutcomeV1,
                };
                let NormalCallableMaterializationOutcomeV1::SourceBacked(source) =
                    materialize_normal_callable_program_v1(
                        source.to_owned(),
                        crate::parser::ParserBuildConfig::default(),
                    )
                    .unwrap()
                else {
                    panic!("source identity required")
                };
                let request = NormalCompileRequestV1::for_mir_mode_callable_source(
                    source,
                    None,
                    Default::default(),
                );
                let result = MirCompiler::with_options(false)
                    .compile_normal(request)
                    .unwrap_or_else(|e| panic!("{case}: {e}"));
                assert!(
                    result.verification_result.is_ok(),
                    "{case}: {:?}",
                    result.verification_result
                );
                let instructions: Vec<_> = result
                    .module
                    .functions
                    .values()
                    .flat_map(|f| f.blocks.values())
                    .flat_map(|b| &b.instructions)
                    .collect();
                assert_eq!(
                    instructions
                        .iter()
                        .filter(|i| matches!(
                            i,
                            MirInstruction::NewBox {
                                target: ConstructionTarget::IntrinsicArray,
                                ..
                            }
                        ))
                        .count(),
                    allocations,
                    "{case}"
                );
                assert_eq!(
                    instructions
                        .iter()
                        .filter(|i| matches!(i, MirInstruction::ArrayElementWrite { .. }))
                        .count(),
                    writes,
                    "{case}"
                );
                let view =
                    crate::mir::function::PublishedMirBackendView::try_new(&result.module).unwrap();
                let dir = std::env::temp_dir()
                    .join(format!("hako-array-source-{case}-{}", std::process::id()));
                std::fs::create_dir_all(&dir).unwrap();
                let direct = dir.join("direct");
                let object = dir.join("source.o");
                assert!(emit_published_view_exe(
                    &view,
                    direct.to_str().unwrap(),
                    Some("target/release"),
                    None
                )
                .unwrap());
                compile_published_view_object(&view, object.to_str().unwrap(), None).unwrap();
                let linked = dir.join("linked");
                super::super::link_object_capi_v2(
                    &object,
                    &linked,
                    Path::new("target/release/libnyash_kernel.a"),
                    None,
                )
                .unwrap();
                for exe in [direct, linked] {
                    let out = std::process::Command::new(exe)
                        .env("HAKO_NYRT_PLUGIN_HOST", "off")
                        .output()
                        .unwrap();
                    assert_eq!(
                        out.status.code(),
                        Some(30),
                        "{case}: {}",
                        String::from_utf8_lossy(&out.stderr)
                    );
                }
                std::fs::remove_dir_all(dir).unwrap();
            }
        })
        .unwrap()
        .join()
        .unwrap();
}

#[test]
fn intrinsic_and_named_source_construction_stay_distinct() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let source = "local literal = []\nlocal named = new ArrayBox()\nreturn 30";
    let ast = crate::parser::NyashParser::parse_from_string(source).unwrap();
    let request = NormalCompileRequestV1::for_mir_mode(ast, None, Default::default()).unwrap();
    let result = MirCompiler::with_options(false)
        .compile_normal(request)
        .unwrap();
    let targets: Vec<_> = result
        .module
        .functions
        .values()
        .flat_map(|f| f.blocks.values())
        .flat_map(|b| &b.instructions)
        .filter_map(|i| match i {
            MirInstruction::NewBox { target, .. } => Some(target),
            _ => None,
        })
        .collect();
    assert!(targets.contains(&&ConstructionTarget::IntrinsicArray));
    assert!(targets.contains(&&ConstructionTarget::Named("ArrayBox".into())));
}

// Dependency evidence only: this source cannot yet reach the typed-array helper
// or backend capability. The construction series retains that open obligation.
#[test]
fn unsupported_typed_array_and_loop_sources_keep_their_pre_effect_stop() {
    use crate::runner::modes::common_util::normal_callable::{
        materialize_normal_callable_program_v1, NormalCallableMaterializationOutcomeV1,
    };
    for (source, kind) in [
        ("local a: Array<String> = []\nreturn 30", "Local"),
        ("local a: PackedArray<i64> = []\nreturn 30", "Local"),
        ("local a: Array<i64>\nreturn 30", "Local"),
        ("local a = []\nlocal b: Array<i64> = a\nreturn 30", "Local"),
        (
            "local i = 0\nloop(i < 2) { local a = [10, 20] i = i + 1 }\nreturn 30",
            "Loop",
        ),
    ] {
        let NormalCallableMaterializationOutcomeV1::SourceBacked(source) =
            materialize_normal_callable_program_v1(
                source,
                crate::parser::ParserBuildConfig::default(),
            )
            .unwrap()
        else {
            panic!("source identity required")
        };
        let request =
            NormalCompileRequestV1::for_mir_mode_callable_source(source, None, Default::default());
        let error = MirCompiler::with_options(false)
            .compile_normal(request)
            .unwrap_err();
        assert!(
            error.contains("[mir/script-pre-effect/source]")
                && error.contains("UnsupportedStatement")
                && error.contains(kind),
            "{error}"
        );
    }
}

#[test]
fn instance_declaration_only_retains_unit_completion() {
    use crate::runner::modes::common_util::normal_callable::{
        materialize_normal_callable_program_v1, NormalCallableMaterializationOutcomeV1,
    };
    let NormalCallableMaterializationOutcomeV1::SourceBacked(source) =
        materialize_normal_callable_program_v1(
            "box Only { birth() {} }",
            crate::parser::ParserBuildConfig::default(),
        )
        .unwrap()
    else {
        panic!("source identity required")
    };
    let request =
        NormalCompileRequestV1::for_mir_mode_callable_source(source, None, Default::default());
    let result = MirCompiler::with_options(false)
        .compile_normal(request)
        .unwrap();
    assert!(result.verification_result.is_ok());
    let root = result.module.functions.get("main").expect("Script root");
    let unit_values: Vec<_> = root
        .blocks
        .values()
        .flat_map(|b| &b.instructions)
        .filter_map(|i| match i {
            MirInstruction::Const {
                dst,
                value: crate::mir::ConstValue::Void,
            } => Some(*dst),
            _ => None,
        })
        .collect();
    assert!(root.blocks.values().flat_map(|b| b.terminator.iter()).any(|i|
        matches!(i, MirInstruction::Return { value: Some(value) } if unit_values.contains(value))));
}

#[test]
fn numeric_typed_array_materialized_source_keeps_one_claim_and_backend_stop() {
    use crate::mir::function::{TypedArrayContractBoundary, TypedArrayContractSourceIdentity};
    use crate::runner::modes::common_util::normal_callable::{
        materialize_normal_callable_program_v1, NormalCallableMaterializationOutcomeV1,
    };
    crate::runtime::ring0::ensure_global_ring0_initialized();
    for element in ["i8", "i16", "i32", "i64", "u8", "u16", "u32"] {
        let source = format!("local a: Array<{element}> = [10, 20]\nreturn 30");
        let NormalCallableMaterializationOutcomeV1::SourceBacked(source) =
            materialize_normal_callable_program_v1(
                source,
                crate::parser::ParserBuildConfig::default(),
            )
            .unwrap()
        else {
            panic!("source identity required")
        };
        let result = MirCompiler::with_options(false)
            .compile_normal(NormalCompileRequestV1::for_mir_mode_callable_source(
                source,
                None,
                Default::default(),
            ))
            .unwrap_or_else(|error| panic!("{element}: {error}"));
        assert!(
            result.verification_result.is_ok(),
            "{element}: {:?}",
            result.verification_result
        );
        let root = result.module.functions.get("main").unwrap();
        use crate::mir::instruction::InvokeOperation;
        let instructions: Vec<_> = root
            .blocks
            .values()
            .flat_map(|b| b.all_instructions())
            .collect();
        let allocations: Vec<_> = instructions
            .iter()
            .filter_map(|i| match i {
                MirInstruction::InvokeNormalResult { dst, .. } => Some(*dst),
                _ => None,
            })
            .collect();
        assert_eq!(allocations.len(), 1);
        let claims: Vec<_> = instructions
            .iter()
            .filter_map(|i| match i {
                MirInstruction::Invoke {
                    operation: InvokeOperation::ArrayStateContractClaim { contract_id, array },
                    ..
                } => Some((contract_id, *array)),
                _ => None,
            })
            .collect();
        let [(claim_id, array)] = claims.as_slice() else {
            panic!("one preclaim")
        };
        assert_eq!(*array, allocations[0]);
        assert_eq!(
            instructions
                .iter()
                .filter(|i| matches!(
                    i,
                    MirInstruction::Invoke {
                        operation: InvokeOperation::ArrayElementWrite { .. },
                        ..
                    }
                ))
                .count(),
            2
        );
        assert!(!instructions.iter().any(|i| matches!(
            i,
            MirInstruction::ArrayElementWrite { .. }
                | MirInstruction::ArrayStateContractClaim { .. }
                | MirInstruction::NewBox { .. }
        )));
        let [source_row] = root.metadata.typed_array_contract_sources.as_slice() else {
            panic!("one source claim")
        };
        assert_eq!(&source_row.contract_id, *claim_id);
        assert_eq!(source_row.boundary, TypedArrayContractBoundary::LocalInit);
        assert!(matches!(
            source_row.source_identity,
            TypedArrayContractSourceIdentity::LocalSlot(_)
        ));
        assert_eq!(source_row.element_spec.element.source_name(), element);
        let [carrier] = root.metadata.typed_array_element_contracts.as_slice() else {
            panic!("one refreshed carrier")
        };
        assert_eq!(carrier.contract_id, source_row.contract_id);
        assert_eq!(carrier.source_identity, source_row.source_identity);
        assert_eq!(carrier.element_spec, source_row.element_spec);
        assert!(carrier.runtime_check_required);
        let view = crate::mir::function::PublishedMirBackendView::try_new(&result.module).unwrap();
        let dir = std::env::temp_dir().join(format!(
            "hako-typed-source-{element}-{}",
            std::process::id()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let error =
            compile_published_view_object(&view, dir.join("source.o").to_str().unwrap(), None)
                .unwrap_err();
        assert!(
            error.contains("typed_array_contract_backend_unsupported"),
            "{error}"
        );
        let error = emit_published_view_exe(
            &view,
            dir.join("source").to_str().unwrap(),
            Some("target/release"),
            None,
        )
        .unwrap_err();
        assert!(
            error.contains("typed_array_contract_backend_unsupported"),
            "{error}"
        );
        assert_eq!(std::fs::read_dir(&dir).unwrap().count(), 0);
        std::fs::remove_dir(dir).unwrap();
    }
}

#[test]
fn typed_array_source_lifecycle_consumes_real_prefix_and_stops_unknown_children() {
    use crate::runner::modes::common_util::normal_callable::{
        materialize_normal_callable_program_v1, NormalCallableMaterializationOutcomeV1,
    };
    crate::runtime::ring0::ensure_global_ring0_initialized();
    for (text, expected_error) in [
        ("local n = 30\nlocal a: Array<i64> = [10]\nlocal alias = a\nlocal b: Array<u8> = [20]\nreturn 30", None),
        ("local s = \"x\"\nlocal a: Array<i64> = []\nreturn 30", Some("caller-prefix-capability")),
        ("local a: Array<i64> = [[10]]\nreturn 30", Some("child-capability")),
        ("local a: Array<i64> = [\"x\"]\nreturn 30", Some("child-capability")),
    ] {
        let NormalCallableMaterializationOutcomeV1::SourceBacked(source) =
            materialize_normal_callable_program_v1(text, Default::default()).unwrap()
        else { panic!("source-backed required") };
        let result = MirCompiler::with_options(false).compile_normal(
            NormalCompileRequestV1::for_mir_mode_callable_source(source, None, Default::default()),
        );
        if let Some(reason) = expected_error {
            let error = match result { Err(error) => error, Ok(_) => panic!("expected stop: {text}") };
            assert!(error.contains("[script-array/source-lifecycle-unavailable]"), "{error}");
            assert!(error.contains(reason), "{error}");
        } else {
            let result = result.unwrap();
            assert!(result.verification_result.is_ok(), "{:?}", result.verification_result);
            let view = crate::mir::function::PublishedMirBackendView::try_new(&result.module).unwrap();
            let dir = std::env::temp_dir().join(format!("hako-array-prefix-stop-{}", std::process::id()));
            std::fs::create_dir_all(&dir).unwrap();
            let error = compile_published_view_object(&view, dir.join("source.o").to_str().unwrap(), None).unwrap_err();
            assert!(error.contains("typed_array_contract_backend_unsupported"), "{error}");
            assert_eq!(std::fs::read_dir(&dir).unwrap().count(), 0);
            std::fs::remove_dir(dir).unwrap();
        }
    }
}

#[test]
fn typed_array_real_source_requires_explicit_root_completion() {
    use crate::runner::modes::common_util::normal_callable::{
        materialize_normal_callable_program_v1, NormalCallableMaterializationOutcomeV1,
    };
    crate::runtime::ring0::ensure_global_ring0_initialized();
    for (text, reason) in [
        ("local a: Array<i64> = []", Some("root-terminal-unissued")),
        (
            "local a: Array<i64> = []\nlocal s = \"x\"\nreturn 30",
            Some("root-prefix-capability"),
        ),
        (
            "local a: Array<i64> = []\nreturn true",
            Some("root-result-capability"),
        ),
        ("local a: Array<i64> = []\nreturn", None),
    ] {
        let NormalCallableMaterializationOutcomeV1::SourceBacked(source) =
            materialize_normal_callable_program_v1(text, Default::default()).unwrap()
        else {
            panic!("source-backed")
        };
        let result = MirCompiler::with_options(false).compile_normal(
            NormalCompileRequestV1::for_mir_mode_callable_source(source, None, Default::default()),
        );
        if let Some(reason) = reason {
            let error = match result {
                Err(error) => error,
                Ok(_) => panic!("expected capability stop"),
            };
            assert!(error.contains(reason), "{error}");
        } else {
            let result = result.unwrap();
            assert!(
                result.verification_result.is_ok(),
                "{:?}",
                result.verification_result
            );
            let view =
                crate::mir::function::PublishedMirBackendView::try_new(&result.module).unwrap();
            let dir =
                std::env::temp_dir().join(format!("hako-array-bare-root-{}", std::process::id()));
            std::fs::create_dir_all(&dir).unwrap();
            let error =
                compile_published_view_object(&view, dir.join("root.o").to_str().unwrap(), None)
                    .unwrap_err();
            assert!(
                error.contains("typed_array_contract_backend_unsupported"),
                "{error}"
            );
            assert_eq!(std::fs::read_dir(&dir).unwrap().count(), 0);
            std::fs::remove_dir(dir).unwrap();
        }
    }
}

#[test]
fn selected_typed_array_published_stop_precedes_runtime_session_and_artifacts() {
    use crate::runner::modes::common_util::normal_callable::{
        materialize_normal_callable_program_v1, NormalCallableMaterializationOutcomeV1,
    };
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var(
        "HAKO_TYPED_OBJECT_STORE",
        "not-a-selected-script-profile",
        || {
            for optimize in [false, true] {
                for terminal in ["return 30", "return"] {
                    let text = format!("local a: Array<i64> = [10, 20]\nlocal alias = a\nlocal b: Array<u8> = []\n{terminal}");
                    let NormalCallableMaterializationOutcomeV1::SourceBacked(source) =
                        materialize_normal_callable_program_v1(text, Default::default()).unwrap()
                    else {
                        panic!("source-backed")
                    };
                    let directory = tempfile::tempdir().unwrap();
                    let mut calls = 0;
                    MirCompiler::with_options(optimize)
                        .compile_normal_with_published(
                            NormalCompileRequestV1::for_mir_mode_callable_source(
                                source,
                                None,
                                Default::default(),
                            ),
                            |view, verification| {
                                calls += 1;
                                assert!(verification.is_ok());
                                assert!(view.retained_script_array().is_some());
                                assert_eq!(
                                    view.route(),
                                    PublishedStaticMethodRouteV1::UnsupportedBeforeObject
                                );
                                assert!(view.lifecycle_storage_profile().is_none());
                                let object_error = try_compile_published_view_object(
                                    view,
                                    directory.path().join("source.o").to_str().unwrap(),
                                )
                                .unwrap_err();
                                let exe_error = emit_published_view_exe(
                                    view,
                                    directory.path().join("source").to_str().unwrap(),
                                    None,
                                    None,
                                )
                                .unwrap_err();
                                for error in [object_error, exe_error] {
                                    assert!(
                                        error.contains("typed_array_contract_backend_unsupported"),
                                        "{error}"
                                    );
                                }
                                assert_eq!(std::fs::read_dir(directory.path()).unwrap().count(), 0);
                                Ok(())
                            },
                        )
                        .unwrap();
                    assert_eq!(calls, 1);
                }
            }
        },
    );
}
