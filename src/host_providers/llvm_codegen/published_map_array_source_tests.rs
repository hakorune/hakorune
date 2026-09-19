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

const AGGREGATE_MAP_READ_SOURCE: &str = r#"
static box Helpers {
    read_all(m: MapBox): i64 {
        local name = m.get("functions").get(0).get("name")
        local params_count = m.get("params").length()
        local kind = m.get("blocks").get(0).get("kind")
        local blocks_count = m.get("blocks").length()
        return params_count
    }
}
static box Main {
    main() {
        local function_row = %{ "name" => "main" }
        local block = %{ "kind" => "entry" }
        return read_all(%{
            "functions" => [function_row],
            "params" => [],
            "blocks" => [block]
        })
    }
}
"#;

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

#[test]
#[ignore = "requires selected C FFI, LLVM18 and lifecycle runtime"]
fn issued_borrowed_blocks_kind_source_reaches_obj_normal_and_prepare_fault() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        use crate::runner::modes::common_util::normal_callable::{
            materialize_normal_callable_program_v1, NormalCallableMaterializationOutcomeV1,
        };
        for (case, expected_exit) in [("normal", 30), ("prepare-fault", 70)] {
            let source = r#"
                static box Helpers {
                    read_kind(m: MapBox): i64 {
                        local kind = m.get("blocks").get(0).get("kind")
                        return 30
                    }
                }
                static box Main {
                    main() {
                        local block = %{ "kind" => "entry" }
                        return read_kind(%{ "blocks" => [block] })
                    }
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
                "hako-map-blocks-kind-source-{}-{case}",
                std::process::id()
            ));
            std::fs::create_dir_all(&dir).unwrap();
            let result = crate::mir::MirCompiler::with_options(true).compile_normal_with_published(
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
                    let object = dir.join("blocks-kind.o");
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

#[test]
#[ignore = "requires selected C FFI, LLVM18 and lifecycle runtime"]
fn issued_borrowed_blocks_length_source_reaches_obj_normal_and_prepare_fault() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        use crate::runner::modes::common_util::normal_callable::{
            materialize_normal_callable_program_v1, NormalCallableMaterializationOutcomeV1,
        };
        for (case, expected_exit) in [("normal", 1), ("prepare-fault", 70)] {
            let source = r#"
                static box Helpers {
                    read_length(m: MapBox): i64 {
                        local count = m.get("blocks").length()
                        return count
                    }
                }
                static box Main {
                    main() {
                        local block = %{ "kind" => "entry" }
                        return read_length(%{ "blocks" => [block] })
                    }
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
                "hako-map-blocks-length-source-{}-{case}",
                std::process::id()
            ));
            std::fs::create_dir_all(&dir).unwrap();
            let result = crate::mir::MirCompiler::with_options(true).compile_normal_with_published(
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
                    assert_eq!(json.matches("\"kind\":\"map_array_length\"").count(), 1);
                    assert_eq!(json.matches("\"kind\":\"map_array_index_map\"").count(), 0);
                    let runtime = Path::new("target/lifecycle-kernel/release");
                    let session = LifecycleRuntimeSessionV1::select(
                        runtime.join("libnyash_lifecycle_kernel.a"),
                    )?;
                    let object = dir.join("blocks-length.o");
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

#[test]
#[ignore = "requires selected C FFI, LLVM18 and lifecycle runtime"]
fn issued_aggregate_map_read_source_reaches_obj_fault_matrix() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        use crate::runner::modes::common_util::normal_callable::{
            materialize_normal_callable_program_v1, NormalCallableMaterializationOutcomeV1,
        };
        for (case, expected_exit, expected_report) in [
            ("normal", 0, None),
            ("prepare-fault", 70, Some(100)),
            ("install-fault", 70, Some(100)),
            ("empty-install-fault", 70, Some(100)),
            ("array-index-fault", 70, Some(106)),
            ("text-read-fault", 70, Some(109)),
            ("array-length-fault", 70, Some(112)),
            ("end-fault", 70, Some(100)),
        ] {
            let NormalCallableMaterializationOutcomeV1::SourceBacked(source) =
                materialize_normal_callable_program_v1(
                    AGGREGATE_MAP_READ_SOURCE.to_string(),
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
                "hako-map-aggregate-source-{}-{case}",
                std::process::id()
            ));
            std::fs::create_dir_all(&dir).unwrap();
            let result = crate::mir::MirCompiler::with_options(true).compile_normal_with_published(
                request,
                |view, verification| -> Result<(), String> {
                    assert!(verification.is_ok(), "{verification:?}");
                    let input = view.issue_lifecycle_physical_abi_input()?;
                    let json = emit_lifecycle_physical_abi_json(&input)?;
                    assert_eq!(
                        json.matches("\"kind\":\"map_install_borrowed_array\"")
                            .count(),
                        2,
                        "{json}"
                    );
                    assert_eq!(
                        json.matches("\"kind\":\"map_install_empty_array\"").count(),
                        1,
                        "{json}"
                    );
                    assert_eq!(json.matches("\"kind\":\"map_array_index_map\"").count(), 2);
                    assert_eq!(json.matches("\"kind\":\"map_array_length\"").count(), 2);
                    assert_eq!(json.matches("\"kind\":\"map_get_text\"").count(), 2);
                    let runtime = Path::new("target/lifecycle-kernel/release");
                    let session = LifecycleRuntimeSessionV1::select(
                        runtime.join("libnyash_lifecycle_kernel.a"),
                    )?;
                    let object = dir.join("aggregate-map-read.o");
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
                    if let Some(reason) = expected_report {
                        let report = format!("REPORT {reason} ");
                        assert!(
                            String::from_utf8_lossy(&output.stdout).contains(&report),
                            "expected {report} in {}",
                            String::from_utf8_lossy(&output.stdout)
                        );
                    }
                    assert_probe_end_precedes_frame(&output.stdout)?;
                    Ok(())
                },
            );
            std::fs::remove_dir_all(dir).unwrap();
            result.unwrap_or_else(|error| panic!("{case}: {error}"));
        }
    });
}

fn assert_probe_end_precedes_frame(stdout: &[u8]) -> Result<(), String> {
    let text = String::from_utf8_lossy(stdout);
    let mut end = None;
    let mut frame = None;
    let mut tokens = text.split_whitespace();
    while let Some(token) = tokens.next() {
        match token {
            "SEQ_END" => end = tokens.next().and_then(|value| value.parse::<u32>().ok()),
            "SEQ_FRAME" => frame = tokens.next().and_then(|value| value.parse::<u32>().ok()),
            _ => {}
        }
    }
    let Some(end) = end else {
        return Err(format!("missing SEQ_END in {text}"));
    };
    let Some(frame) = frame else {
        return Err(format!("missing SEQ_FRAME in {text}"));
    };
    if end == 0 || frame <= end {
        return Err(format!("cleanup order end={end} frame={frame}: {text}"));
    }
    Ok(())
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
        "checked_install_borrowed_array",
        "checked_install_empty_array",
        "outcome_end",
        "checked_end",
        "checked_array_length",
        "checked_array_index_map",
        "checked_get_text",
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
