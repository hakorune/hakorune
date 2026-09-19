//! Source-to-OBJ acceptance for the bounded borrowed Map array lane.
//!
//! This keeps the first nested read witness finite: a caller builds one child
//! Map, repeats that exact MapLocal twice in `functions`, and the same source
//! body reads `functions[0].name`. The normal and injected construction-Fault
//! runs share one source-issued object and prove the caller cleanup path.
use super::*;
use crate::mir::{MirCompiler, NormalCompileRequestV1};
use std::path::Path;
use std::process::Command;

#[test]
#[ignore = "requires selected C FFI, LLVM18 and lifecycle runtime"]
fn issued_borrowed_array_source_reaches_obj_normal_and_prepare_fault() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        use crate::runner::modes::common_util::normal_callable::{
            materialize_normal_callable_program_v1, NormalCallableMaterializationOutcomeV1,
        };
        let body = r#"local child = %{"name" => "main"}
                   return read(%{"functions" => [child, child]})"#;
        for (case, expected_exit) in [("normal", 30), ("prepare-fault", 70)] {
            let source = format!(
                "static box Helpers {{ read(m: MapBox): i64 {{ local text = m.get(\"functions\").get(0).get(\"name\") return 30 }} }} static box Main {{ main() {{ {body} }} }}"
            );
            let NormalCallableMaterializationOutcomeV1::SourceBacked(source) =
                materialize_normal_callable_program_v1(
                    source,
                    crate::parser::ParserBuildConfig::default(),
                )
                .expect("source materialization")
            else {
                panic!("source-backed request required");
            };
            let request = NormalCompileRequestV1::for_mir_mode_callable_source(
                source,
                None,
                std::collections::HashMap::new(),
            );
            let dir = std::env::temp_dir().join(format!(
                "hako-map-array-source-{}-{case}",
                std::process::id()
            ));
            std::fs::create_dir_all(&dir).unwrap();
            let result = MirCompiler::with_options(true).compile_normal_with_published(
                request,
                |view, verification| -> Result<(), String> {
                    assert!(verification.is_ok(), "{verification:?}");
                    let input = view.issue_lifecycle_physical_abi_input()?;
                    let json = emit_lifecycle_physical_abi_json(&input)?;
                    assert_eq!(
                        json.matches("\"kind\":\"map_install_borrowed_array\"")
                            .count(),
                        1,
                        "{json}"
                    );
                    assert_eq!(json.matches("\"kind\":\"map_array_index_map\"").count(), 1);
                    assert_eq!(json.matches("\"kind\":\"map_get_text\"").count(), 1);
                    let runtime = Path::new("target/lifecycle-kernel/release");
                    let session = LifecycleRuntimeSessionV1::select(
                        runtime.join("libnyash_lifecycle_kernel.a"),
                    )?;
                    let object = dir.join("borrowed-array.o");
                    compile_published_view_object(view, object.to_str().unwrap(), Some(&session))
                        .map_err(|error| format!("object: {error}"))?;
                    let linked = dir.join("linked");
                    link_borrowed_array_probe(&object, &linked, session.runtime_archive())?;
                    let output = Command::new(&linked)
                        .arg(case)
                        .env("NYASH_NYRT_SILENT_RESULT", "1")
                        .env("HAKO_NYRT_PLUGIN_HOST", "off")
                        .output()
                        .map_err(|error| error.to_string())?;
                    assert_eq!(
                        output.status.code(),
                        Some(expected_exit),
                        "{case}: {}",
                        String::from_utf8_lossy(&output.stderr)
                    );
                    if case == "prepare-fault" {
                        let stdout = String::from_utf8_lossy(&output.stdout);
                        assert!(stdout.contains("REPORT 100"), "{stdout}");
                    }
                    Ok(())
                },
            );
            std::fs::remove_dir_all(&dir).unwrap();
            result.unwrap_or_else(|error| panic!("{case}: {error}"));
        }
    });
}

#[test]
#[ignore = "requires selected C FFI, LLVM18 and lifecycle runtime"]
fn issued_empty_params_length_source_reaches_obj_normal_and_prepare_fault() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        use crate::runner::modes::common_util::normal_callable::{
            materialize_normal_callable_program_v1, NormalCallableMaterializationOutcomeV1,
        };
        for (case, expected_exit) in [("normal", 0), ("prepare-fault", 70)] {
            let source = r#"
                static box Helpers {
                    read_params(m: MapBox): i64 {
                        local count = m.get("params").length()
                        return count
                    }
                }
                static box Main {
                    main() { return read_params(%{"params" => []}) }
                }
            "#;
            let NormalCallableMaterializationOutcomeV1::SourceBacked(source) =
                materialize_normal_callable_program_v1(
                    source.to_string(),
                    crate::parser::ParserBuildConfig::default(),
                )
                .expect("source materialization")
            else {
                panic!("source-backed request required");
            };
            let request = crate::mir::NormalCompileRequestV1::for_mir_mode_callable_source(
                source,
                None,
                std::collections::HashMap::new(),
            );
            let dir = std::env::temp_dir().join(format!(
                "hako-map-array-length-source-{}-{case}",
                std::process::id()
            ));
            std::fs::create_dir_all(&dir).unwrap();
            let result = crate::mir::MirCompiler::with_options(true).compile_normal_with_published(
                request,
                |view, verification| -> Result<(), String> {
                    assert!(verification.is_ok(), "{verification:?}");
                    let input = view.issue_lifecycle_physical_abi_input()?;
                    let json = crate::mir::emit_lifecycle_physical_abi_json(&input)?;
                    assert_eq!(
                        json.matches("\"kind\":\"map_install_empty_array\"").count(),
                        1,
                        "{json}"
                    );
                    assert_eq!(
                        json.matches("\"kind\":\"map_array_length\"").count(),
                        1,
                        "{json}"
                    );
                    let runtime = Path::new("target/lifecycle-kernel/release");
                    let session = LifecycleRuntimeSessionV1::select(
                        runtime.join("libnyash_lifecycle_kernel.a"),
                    )?;
                    let object = dir.join("empty-params-length.o");
                    compile_published_view_object(view, object.to_str().unwrap(), Some(&session))
                        .map_err(|error| format!("object: {error}"))?;
                    let linked = dir.join("linked");
                    link_borrowed_array_probe(&object, &linked, session.runtime_archive())?;
                    let output = Command::new(&linked)
                        .arg(case)
                        .env("NYASH_NYRT_SILENT_RESULT", "1")
                        .env("HAKO_NYRT_PLUGIN_HOST", "off")
                        .output()
                        .map_err(|error| error.to_string())?;
                    assert_eq!(
                        output.status.code(),
                        Some(expected_exit),
                        "{case}: {}",
                        String::from_utf8_lossy(&output.stderr)
                    );
                    if case == "prepare-fault" {
                        assert!(
                            String::from_utf8_lossy(&output.stdout).contains("REPORT 100"),
                            "{}",
                            String::from_utf8_lossy(&output.stdout)
                        );
                    }
                    Ok(())
                },
            );
            std::fs::remove_dir_all(dir).unwrap();
            result.unwrap_or_else(|error| panic!("{case}: {error}"));
        }
    });
}

fn link_borrowed_array_probe(object: &Path, exe: &Path, archive: &Path) -> Result<(), String> {
    let mut command = Command::new("cc");
    command
        .args(["-Wl,--wrap=main", "-DHAKO_MAP_SOURCE_PROBE"])
        .arg("lang/c-abi/tests/published_map_fault_probe.c")
        .arg(object)
        .arg(archive);
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
        .arg(exe)
        .output()
        .map_err(|error| error.to_string())?;
    if linked.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&linked.stderr).into_owned())
    }
}
