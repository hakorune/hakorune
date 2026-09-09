//! Actual source publication and host artifacts, separate from synthetic C inputs.
use super::*;
use crate::mir::{MirCompiler, NormalCompileRequestV1};
use std::process::Command;

#[test]
#[ignore = "requires selected C FFI, LLVM18 and lifecycle runtime"]
fn issued_map_source_direct_exe_and_linked_object_exit_30() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let mut sources = vec!["static box Main { main() { local m = %{} return 30 } }".to_owned()];
        for definition in ["box Page {}", "box Page { birth() {} }"] {
            for body in [
                "local m = %{} return 30",
                "local m = %{} local alias = m local again = alias return 30",
                "local a = new Page() local m = %{\"a\" => a} return 30",
                "local m = %{} local a = new Page() return 30",
                "local m = %{} local n = %{} return 30",
                "local a = new Page() local b = new Page() local m = %{\"a\" => a, \"a\" => b} return 30",
            ] {
                sources.push(format!("{definition} static box Main {{ main() {{ {body} }} }}"));
            }
        }
        let first_value_case = sources.len();
        for body in [
            "local m = %{\"a\" => 30, \"b\" => true, \"c\" => false} return 30",
            "local x = true local alias = x local n = 30 local m = %{\"a\" => alias, \"b\" => alias, \"n\" => n} return 30",
            "local a = new Page() local b = new Page() local m = %{\"a\" => a, \"a\" => true, \"a\" => b, \"a\" => 30} return 30",
        ] {
            sources.push(format!("box Page {{}} static box Main {{ main() {{ {body} }} }}"));
        }
        for (case, source) in sources.into_iter().enumerate() {
            use crate::runner::modes::common_util::normal_callable::{
                materialize_normal_callable_program_v1, NormalCallableMaterializationOutcomeV1,
            };
            let NormalCallableMaterializationOutcomeV1::SourceBacked(source) =
                materialize_normal_callable_program_v1(
                    source,
                    crate::parser::ParserBuildConfig::default(),
                )
                .unwrap()
            else {
                panic!("source-backed request");
            };
            let request = NormalCompileRequestV1::for_mir_mode_callable_source(
                source,
                None,
                std::collections::HashMap::new(),
            );
            let dir =
                std::env::temp_dir().join(format!("hako-map-source-{}-{case}", std::process::id()));
            std::fs::create_dir_all(&dir).unwrap();
            MirCompiler::with_options(true)
                .compile_normal_with_published(
                    request,
                    |view, verification| -> Result<(), String> {
                        assert!(verification.is_ok(), "{verification:?}");
                        let input = view.issue_lifecycle_physical_abi_input()?;
                        if case == 0 {
                            assert!(input.layouts().is_empty());
                        }
                        let json = emit_lifecycle_physical_abi_json(&input)?;
                        assert!(json.contains("map_new"));
                        assert!(json.contains("map_end"));
                        assert!(!json.contains("MapBox"), "no selected legacy Map name");
                        std::fs::write(dir.join("issued.json"), json).map_err(|e| e.to_string())?;
                        let runtime = Path::new("target/lifecycle-kernel/release");
                        let direct = dir.join("direct");
                        assert!(emit_published_view_exe(
                            view,
                            direct.to_str().unwrap(),
                            runtime.to_str(),
                            None
                        )?);
                        let session = LifecycleRuntimeSessionV1::select(
                            runtime.join("libnyash_lifecycle_kernel.a"),
                        )?;
                        let object = dir.join("map.o");
                        compile_published_view_object(
                            view,
                            object.to_str().unwrap(),
                            Some(&session),
                        )?;
                        let linked = dir.join("linked");
                        super::super::link_object_capi_v2(
                            &object,
                            &linked,
                            session.runtime_archive(),
                            None,
                        )?;
                        for exe in [direct, linked] {
                            let result = Command::new(&exe)
                                .env("NYASH_NYRT_SILENT_RESULT", "1")
                                .env("HAKO_NYRT_PLUGIN_HOST", "off")
                                .output()
                                .map_err(|e| e.to_string())?;
                            assert_eq!(
                                result.status.code(),
                                Some(30),
                                "case {case}: {}",
                                String::from_utf8_lossy(&result.stderr)
                            );
                        }
                        // Duplicate-key source under both NoBirth and empty Birth.
                        if case == 6 || case == 12 {
                            assert_source_fault_cleanup(&object, session.runtime_archive(), &dir, false)?;
                        }
                        if case == first_value_case + 2 {
                            assert_source_fault_cleanup(&object, session.runtime_archive(), &dir, true)?;
                        }
                        Ok(())
                    },
                )
                .unwrap_or_else(|e| panic!("case {case}: {e}"));
            std::fs::remove_dir_all(dir).unwrap();
        }
    });
}

#[test]
#[ignore = "requires selected C FFI, LLVM18 and lifecycle runtime"]
fn issued_ordinary_child_map_value_direct_exe_and_linked_object_exit_30() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        use crate::runner::modes::common_util::normal_callable::{
            materialize_normal_callable_program_v1, NormalCallableMaterializationOutcomeV1,
        };
        let source = r#"static box Main {
            main() { return helper(30) }
            helper(value: i64): i64 { local m = %{"x" => value} return 30 }
        }"#;
        let NormalCallableMaterializationOutcomeV1::SourceBacked(source) =
            materialize_normal_callable_program_v1(
                source.to_owned(),
                crate::parser::ParserBuildConfig::default(),
            )
            .unwrap()
        else {
            panic!("source-backed input required");
        };
        let request = NormalCompileRequestV1::for_mir_mode_callable_source(
            source,
            None,
            std::collections::HashMap::new(),
        );
        let dir = std::env::temp_dir().join(format!(
            "hako-ordinary-child-map-source-{}",
            std::process::id()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let result = MirCompiler::with_options(true).compile_normal_with_published(
            request,
            |view, verification| -> Result<(), String> {
                assert!(verification.is_ok(), "{verification:?}");
                assert!(view.has_lifecycle_instructions());
                assert_eq!(view.route(), PublishedStaticMethodRouteV1::CanonicalTyped);
                let input = view.issue_lifecycle_physical_abi_input()?;
                assert_eq!(input.entry().ordinary_calls().len(), 1);
                let json = emit_lifecycle_physical_abi_json(&input)?;
                assert!(json.contains("ordinary_i64"));
                assert!(json.contains("map_install_value"));
                let runtime = Path::new("target/lifecycle-kernel/release");
                let direct = dir.join("direct");
                assert!(emit_published_view_exe(
                    view,
                    direct.to_str().unwrap(),
                    runtime.to_str(),
                    None,
                )
                .map_err(|error| format!("direct: {error}"))?);
                let session = LifecycleRuntimeSessionV1::select(
                    runtime.join("libnyash_lifecycle_kernel.a"),
                )?;
                let object = dir.join("ordinary-child-map.o");
                compile_published_view_object(
                    view,
                    object.to_str().unwrap(),
                    Some(&session),
                )
                .map_err(|error| format!("object: {error}"))?;
                let linked = dir.join("linked");
                super::super::link_object_capi_v2(
                    &object,
                    &linked,
                    session.runtime_archive(),
                    None,
                )?;
                for exe in [direct, linked] {
                    let output = Command::new(&exe)
                        .env("NYASH_NYRT_SILENT_RESULT", "1")
                        .env("HAKO_NYRT_PLUGIN_HOST", "off")
                        .output()
                        .map_err(|e| e.to_string())?;
                    assert_eq!(output.status.code(), Some(30), "{exe:?}: {output:?}");
                }
                Ok(())
            },
        );
        std::fs::remove_dir_all(&dir).unwrap();
        result.unwrap();
    });
}

#[test]
#[ignore = "requires selected C FFI, LLVM18 and lifecycle runtime"]
fn issued_root_and_ordinary_child_map_values_direct_exe_and_linked_object_exit_30() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        use crate::runner::modes::common_util::normal_callable::{
            materialize_normal_callable_program_v1, NormalCallableMaterializationOutcomeV1,
        };
        let source = r#"static box Main {
            main() { local root_map = %{"root" => 1} return helper(30) }
            helper(value: i64): i64 { local child_map = %{"child" => value} return 30 }
        }"#;
        let NormalCallableMaterializationOutcomeV1::SourceBacked(source) =
            materialize_normal_callable_program_v1(
                source.to_owned(),
                crate::parser::ParserBuildConfig::default(),
            )
            .unwrap()
        else {
            panic!("source-backed input required");
        };
        let request = NormalCompileRequestV1::for_mir_mode_callable_source(
            source,
            None,
            std::collections::HashMap::new(),
        );
        let dir = std::env::temp_dir().join(format!(
            "hako-root-and-ordinary-map-source-{}",
            std::process::id()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let result = MirCompiler::with_options(true).compile_normal_with_published(
            request,
            |view, verification| -> Result<(), String> {
                assert!(verification.is_ok(), "{verification:?}");
                assert!(view.has_lifecycle_instructions());
                assert_eq!(view.route(), PublishedStaticMethodRouteV1::CanonicalTyped);
                let input = view.issue_lifecycle_physical_abi_input()?;
                let json = emit_lifecycle_physical_abi_json(&input)?;
                assert!(json.contains("ordinary_i64"));
                assert!(json.contains("map_install_value"));
                let runtime = Path::new("target/lifecycle-kernel/release");
                let direct = dir.join("direct");
                assert!(emit_published_view_exe(
                    view,
                    direct.to_str().unwrap(),
                    runtime.to_str(),
                    None,
                )
                .map_err(|error| format!("direct: {error}"))?);
                let session = LifecycleRuntimeSessionV1::select(
                    runtime.join("libnyash_lifecycle_kernel.a"),
                )?;
                let object = dir.join("root-and-ordinary-map.o");
                compile_published_view_object(
                    view,
                    object.to_str().unwrap(),
                    Some(&session),
                )
                .map_err(|error| format!("object: {error}"))?;
                let linked = dir.join("linked");
                super::super::link_object_capi_v2(
                    &object,
                    &linked,
                    session.runtime_archive(),
                    None,
                )?;
                for exe in [direct, linked] {
                    let output = Command::new(&exe)
                        .env("NYASH_NYRT_SILENT_RESULT", "1")
                        .env("HAKO_NYRT_PLUGIN_HOST", "off")
                        .output()
                        .map_err(|e| e.to_string())?;
                    assert_eq!(output.status.code(), Some(30), "{exe:?}: {output:?}");
                }
                Ok(())
            },
        );
        std::fs::remove_dir_all(&dir).unwrap();
        result.unwrap();
    });
}

#[test]
#[ignore = "requires selected C FFI, LLVM18 and lifecycle runtime"]
fn issued_local_and_terminal_map_calls_direct_exe_and_linked_object_exit_30() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        use crate::runner::modes::common_util::normal_callable::{
            materialize_normal_callable_program_v1, NormalCallableMaterializationOutcomeV1,
        };
        let source = r#"static box Main {
            main() { local first = helper(10) return helper(10) }
            helper(value: i64): i64 { local m = %{"first" => value} return 30 }
        }"#;
        let NormalCallableMaterializationOutcomeV1::SourceBacked(source) =
            materialize_normal_callable_program_v1(
                source.to_owned(),
                crate::parser::ParserBuildConfig::default(),
            )
            .unwrap()
        else {
            panic!("source-backed input required");
        };
        let request = NormalCompileRequestV1::for_mir_mode_callable_source(
            source,
            None,
            std::collections::HashMap::new(),
        );
        let dir = std::env::temp_dir().join(format!(
            "hako-local-and-terminal-map-source-{}",
            std::process::id()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let result = MirCompiler::with_options(true).compile_normal_with_published(
            request,
            |view, verification| -> Result<(), String> {
                assert!(verification.is_ok(), "{verification:?}");
                assert!(view.has_lifecycle_instructions());
                assert_eq!(view.route(), PublishedStaticMethodRouteV1::CanonicalTyped);
                let input = view.issue_lifecycle_physical_abi_input()?;
                let json = emit_lifecycle_physical_abi_json(&input)?;
                assert!(json.contains("ordinary_i64"));
                assert!(json.matches("ordinary_i64").count() >= 1);
                assert!(json.matches("ordinary_call").count() >= 2);
                let runtime = Path::new("target/lifecycle-kernel/release");
                let direct = dir.join("direct");
                emit_published_view_exe(view, direct.to_str().unwrap(), runtime.to_str(), None)
                    .map_err(|error| format!("direct: {error}"))?;
                let session = LifecycleRuntimeSessionV1::select(
                    runtime.join("libnyash_lifecycle_kernel.a"),
                )?;
                let object = dir.join("local-and-terminal-map.o");
                compile_published_view_object(view, object.to_str().unwrap(), Some(&session))
                    .map_err(|error| format!("object: {error}"))?;
                let linked = dir.join("linked");
                super::super::link_object_capi_v2(
                    &object,
                    &linked,
                    session.runtime_archive(),
                    None,
                )?;
                for exe in [direct, linked] {
                    let output = Command::new(&exe)
                        .env("NYASH_NYRT_SILENT_RESULT", "1")
                        .env("HAKO_NYRT_PLUGIN_HOST", "off")
                        .output()
                        .map_err(|e| e.to_string())?;
                    assert_eq!(output.status.code(), Some(30), "{exe:?}: {output:?}");
                }
                Ok(())
            },
        );
        std::fs::remove_dir_all(&dir).unwrap();
        result.unwrap();
    });
}

#[test]
#[ignore = "requires selected C FFI, LLVM18 and lifecycle runtime"]
fn issued_distinct_map_call_owners_direct_exe_and_linked_object_exit_30() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        use crate::runner::modes::common_util::normal_callable::{
            materialize_normal_callable_program_v1, NormalCallableMaterializationOutcomeV1,
        };
        let source = r#"static box Main {
            main() {
                local first = helper(10)
                local root_map = %{"root" => 1}
                return other(20)
            }
            helper(value: i64): i64 { local m = %{"first" => value} return 30 }
            other(value: i64): i64 { local m = %{"second" => value} return 30 }
        }"#;
        let NormalCallableMaterializationOutcomeV1::SourceBacked(source) =
            materialize_normal_callable_program_v1(
                source.to_owned(),
                crate::parser::ParserBuildConfig::default(),
            )
            .unwrap()
        else {
            panic!("source-backed input required");
        };
        let request = NormalCompileRequestV1::for_mir_mode_callable_source(
            source,
            None,
            std::collections::HashMap::new(),
        );
        let dir = std::env::temp_dir().join(format!(
            "hako-distinct-map-owners-source-{}",
            std::process::id()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let result = MirCompiler::with_options(true).compile_normal_with_published(
            request,
            |view, verification| -> Result<(), String> {
                assert!(verification.is_ok(), "{verification:?}");
                let input = view.issue_lifecycle_physical_abi_input()?;
                let json = emit_lifecycle_physical_abi_json(&input)?;
                assert!(json.matches("ordinary_i64").count() >= 2);
                assert!(json.matches("ordinary_call").count() >= 2);
                let runtime = Path::new("target/lifecycle-kernel/release");
                let direct = dir.join("direct");
                emit_published_view_exe(view, direct.to_str().unwrap(), runtime.to_str(), None)
                    .map_err(|error| format!("direct: {error}"))?;
                let session = LifecycleRuntimeSessionV1::select(
                    runtime.join("libnyash_lifecycle_kernel.a"),
                )?;
                let object = dir.join("distinct-map-owners.o");
                compile_published_view_object(view, object.to_str().unwrap(), Some(&session))
                    .map_err(|error| format!("object: {error}"))?;
                let linked = dir.join("linked");
                super::super::link_object_capi_v2(
                    &object,
                    &linked,
                    session.runtime_archive(),
                    None,
                )?;
                for exe in [direct, linked] {
                    let output = Command::new(&exe)
                        .env("NYASH_NYRT_SILENT_RESULT", "1")
                        .env("HAKO_NYRT_PLUGIN_HOST", "off")
                        .output()
                        .map_err(|e| e.to_string())?;
                    assert_eq!(output.status.code(), Some(30), "{exe:?}: {output:?}");
                }
                Ok(())
            },
        );
        std::fs::remove_dir_all(&dir).unwrap();
        result.unwrap();
    });
}

#[test]
#[ignore = "requires selected C FFI, LLVM18 and lifecycle runtime"]
fn issued_three_distinct_map_call_owners_direct_exe_and_linked_object_exit_30() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        use crate::runner::modes::common_util::normal_callable::{
            materialize_normal_callable_program_v1, NormalCallableMaterializationOutcomeV1,
        };
        let source = r#"static box Main {
            main() {
                local first = helper(10)
                local second = middle(20)
                local root_map = %{"root" => 1}
                return other(30)
            }
            helper(value: i64): i64 { local m = %{"first" => value} return 30 }
            middle(value: i64): i64 { local m = %{"middle" => value} return 30 }
            other(value: i64): i64 { local m = %{"second" => value} return 30 }
        }"#;
        let NormalCallableMaterializationOutcomeV1::SourceBacked(source) =
            materialize_normal_callable_program_v1(
                source.to_owned(),
                crate::parser::ParserBuildConfig::default(),
            )
            .unwrap()
        else {
            panic!("source-backed input required");
        };
        let request = NormalCompileRequestV1::for_mir_mode_callable_source(
            source,
            None,
            std::collections::HashMap::new(),
        );
        let dir = std::env::temp_dir().join(format!(
            "hako-three-map-owners-source-{}",
            std::process::id()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let result = MirCompiler::with_options(true).compile_normal_with_published(
            request,
            |view, verification| -> Result<(), String> {
                assert!(verification.is_ok(), "{verification:?}");
                let input = view.issue_lifecycle_physical_abi_input()?;
                let json = emit_lifecycle_physical_abi_json(&input)?;
                assert!(json.matches("ordinary_i64").count() >= 3);
                assert!(json.matches("ordinary_call").count() >= 3);
                let runtime = Path::new("target/lifecycle-kernel/release");
                let direct = dir.join("direct");
                emit_published_view_exe(view, direct.to_str().unwrap(), runtime.to_str(), None)
                    .map_err(|error| format!("direct: {error}"))?;
                let session = LifecycleRuntimeSessionV1::select(
                    runtime.join("libnyash_lifecycle_kernel.a"),
                )?;
                let object = dir.join("three-map-owners.o");
                compile_published_view_object(view, object.to_str().unwrap(), Some(&session))
                    .map_err(|error| format!("object: {error}"))?;
                let linked = dir.join("linked");
                super::super::link_object_capi_v2(
                    &object,
                    &linked,
                    session.runtime_archive(),
                    None,
                )?;
                for exe in [direct, linked] {
                    let output = Command::new(&exe)
                        .env("NYASH_NYRT_SILENT_RESULT", "1")
                        .env("HAKO_NYRT_PLUGIN_HOST", "off")
                        .output()
                        .map_err(|e| e.to_string())?;
                    assert_eq!(output.status.code(), Some(30), "{exe:?}: {output:?}");
                }
                assert_three_owner_fault_cleanup(
                    &object,
                    session.runtime_archive(),
                    &dir,
                )?;
                Ok(())
            },
        );
        std::fs::remove_dir_all(&dir).unwrap();
        result.unwrap();
    });
}

fn assert_source_fault_cleanup(object: &Path, archive: &Path, dir: &Path, value_mixed: bool) -> Result<(), String> {
    let exe = dir.join("fault-probe");
    let mut command = Command::new("cc");
    command
        .arg("-Wl,--wrap=main")
        .arg("-DHAKO_MAP_SOURCE_PROBE")
        .arg("lang/c-abi/tests/published_map_fault_probe.c")
        .arg(object)
        .arg(archive);
    if value_mixed {
        command.arg("-DHAKO_MAP_VALUE_PROBE")
            .arg("-Wl,--wrap=nyash.map.checked_install_value_v1");
    }
    for name in [
        "storage_init",
        "storage_dispose",
        "key_init",
        "key_dispose",
        "outcome_init",
        "outcome_dispose",
        "checked_new",
        "key_prepare_utf8",
        "checked_install_indexed",
        "outcome_end",
        "checked_end",
    ] {
        command.arg(format!("-Wl,--wrap=nyash.map.{name}_v1"));
    }
    for name in [
        "object.home_release_plain_i64",
        "fault.report_final",
        "fault.frame_dispose",
    ] {
        command.arg(format!("-Wl,--wrap=nyash.{name}_v1"));
    }
    let linked = command
        .args(["-lpthread", "-ldl", "-lm", "-o"])
        .arg(&exe)
        .output()
        .map_err(|e| e.to_string())?;
    assert!(
        linked.status.success(),
        "{}",
        String::from_utf8_lossy(&linked.stderr)
    );
    let mut modes = vec![
        ("normal", if value_mixed { 4 } else { 2 }, if value_mixed { 4 } else { 2 }, 0),
        ("new-fault", 0, 0, 2), ("prepare-fault", 1, 0, 2),
        ("install-fault", 1, 1, 2), ("outcome-fault", 1, 1, 1),
        ("end-fault", if value_mixed { 4 } else { 2 }, if value_mixed { 4 } else { 2 }, 0),
    ];
    if value_mixed { modes.extend([("value-install-fault", 2, 2, 1), ("value-outcome-fault", 2, 2, 1)]); }
    for (mode, keys, outcomes, outer) in modes {
        let result = Command::new(&exe)
            .arg(mode)
            .env("NYASH_NYRT_SILENT_RESULT", "1")
            .env("HAKO_NYRT_PLUGIN_HOST", "off")
            .output()
            .map_err(|e| e.to_string())?;
        let expected = if mode == "normal" { 30 } else { 70 };
        let reports = u32::from(mode != "normal");
        assert_eq!(
            result.status.code(),
            Some(expected),
            "{mode}: {}",
            String::from_utf8_lossy(&result.stderr)
        );
        let stdout = String::from_utf8_lossy(&result.stdout);
        let frame =
            format!("FRAME OUTER {outer} REPORTS {reports} MAP 1 KEY {keys} OUTCOME {outcomes}\n");
        assert!(stdout.contains(&frame), "{mode}: {stdout}");
        assert!(
            stdout.ends_with(&format!(
                "{expected} 1 1 {keys} {keys} {outcomes} {outcomes}\n"
            )),
            "{mode}: {stdout}"
        );
        if reports != 0 {
            let reason = if mode == "install-fault" || mode == "value-install-fault" { 101 } else { 100 };
            let report = format!("REPORT {reason} OUTER {outer} MAP 1 KEY {keys} OUTCOME {outcomes}\n");
            assert!(stdout.contains(&report), "{mode}: {stdout}");
            assert!(stdout.find(&report).unwrap() < stdout.find(&frame).unwrap());
        }
    }
    Ok(())
}

fn assert_three_owner_fault_cleanup(object: &Path, archive: &Path, dir: &Path) -> Result<(), String> {
    let exe = dir.join("fault-probe-three-owners");
    let mut command = Command::new("cc");
    command
        .arg("-Wl,--wrap=main")
        .arg("-DHAKO_MAP_SOURCE_PROBE")
        .arg("-DHAKO_MAP_THREE_OWNER_PROBE")
        .arg("lang/c-abi/tests/published_map_fault_probe.c")
        .arg(object)
        .arg(archive)
        .arg("-Wl,--wrap=nyash.map.checked_install_value_v1");
    for name in [
        "storage_init",
        "storage_dispose",
        "key_init",
        "key_dispose",
        "outcome_init",
        "outcome_dispose",
        "checked_new",
        "key_prepare_utf8",
        "checked_install_indexed",
        "outcome_end",
        "checked_end",
    ] {
        command.arg(format!("-Wl,--wrap=nyash.map.{name}_v1"));
    }
    for name in [
        "object.home_release_plain_i64",
        "fault.report_final",
        "fault.frame_dispose",
    ] {
        command.arg(format!("-Wl,--wrap=nyash.{name}_v1"));
    }
    let linked = command
        .args(["-lpthread", "-ldl", "-lm", "-o"])
        .arg(&exe)
        .output()
        .map_err(|e| e.to_string())?;
    assert!(
        linked.status.success(),
        "{}",
        String::from_utf8_lossy(&linked.stderr)
    );
    let cases = [
        ("normal", 30, 4, 4, 4, 0, 0, "10 20 1 30"),
        ("value-install-fault-1", 70, 1, 1, 1, 0, 101, "10"),
        ("value-install-fault-2", 70, 2, 2, 2, 0, 101, "10 20"),
        ("value-install-fault-4", 70, 4, 4, 4, 0, 101, "10 20 1 30"),
        ("value-outcome-fault-2", 70, 2, 2, 2, 0, 100, "10 20"),
        ("value-outcome-fault-4", 70, 4, 4, 4, 0, 100, "10 20 1 30"),
    ];
    for (mode, expected, maps, keys, outcomes, outer, reason, values) in cases {
        let result = Command::new(&exe)
            .arg(mode)
            .env("NYASH_NYRT_SILENT_RESULT", "1")
            .env("HAKO_NYRT_PLUGIN_HOST", "off")
            .output()
            .map_err(|e| e.to_string())?;
        assert_eq!(result.status.code(), Some(expected), "{mode}: {:?}", result);
        let stdout = String::from_utf8_lossy(&result.stdout);
        let reports = u32::from(expected != 30);
        let frame = format!(
            "FRAME OUTER {outer} REPORTS {reports} MAP {maps} KEY {keys} OUTCOME {outcomes}\n"
        );
        assert!(stdout.contains(&frame), "{mode}: {stdout}");
        assert!(stdout.contains(&format!("VALUES {values}\n")), "{mode}: {stdout}");
        assert!(
            stdout.ends_with(&format!(
                "{expected} {maps} {maps} {keys} {keys} {outcomes} {outcomes}\n"
            )),
            "{mode}: {stdout}"
        );
        if reports != 0 {
            let report = format!(
                "REPORT {reason} OUTER {outer} MAP {maps} KEY {keys} OUTCOME {outcomes}\n"
            );
            assert!(stdout.contains(&report), "{mode}: {stdout}");
            assert!(stdout.find(&report).unwrap() < stdout.find(&frame).unwrap());
        }
    }
    Ok(())
}
