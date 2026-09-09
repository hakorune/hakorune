//! Source-backed four-owner Map artifact and Fault-order evidence.
use super::*;
use crate::mir::{emit_lifecycle_physical_abi_json, MirCompiler, NormalCompileRequestV1};
use std::path::Path;
use std::process::Command;

#[test]
#[ignore = "requires selected C FFI, LLVM18 and lifecycle runtime"]
fn issued_four_distinct_map_call_owners_direct_exe_and_linked_object_exit_30() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        use crate::runner::modes::common_util::normal_callable::{
            materialize_normal_callable_program_v1, NormalCallableMaterializationOutcomeV1,
        };
        let source = r#"static box Main {
            main() {
                local first = helper(10)
                local second = middle(20)
                local third = later(1)
                local root_map = %{"root" => 2}
                return other(30)
            }
            helper(value: i64): i64 { local m = %{"first" => value} return 30 }
            middle(value: i64): i64 { local m = %{"middle" => value} return 30 }
            later(value: i64): i64 { local m = %{"later" => value} return 30 }
            other(value: i64): i64 { local m = %{"other" => value} return 30 }
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
            "hako-four-map-owners-source-{}",
            std::process::id()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let result = MirCompiler::with_options(true).compile_normal_with_published(
            request,
            |view, verification| -> Result<(), String> {
                assert!(verification.is_ok(), "{verification:?}");
                let input = view.issue_lifecycle_physical_abi_input()?;
                let json = emit_lifecycle_physical_abi_json(&input)?;
                assert!(json.matches("ordinary_i64").count() >= 4);
                assert!(json.matches("ordinary_call").count() >= 4);
                let runtime = Path::new("target/lifecycle-kernel/release");
                let direct = dir.join("direct");
                emit_published_view_exe(view, direct.to_str().unwrap(), runtime.to_str(), None)
                    .map_err(|error| format!("direct: {error}"))?;
                let session = LifecycleRuntimeSessionV1::select(
                    runtime.join("libnyash_lifecycle_kernel.a"),
                )?;
                let object = dir.join("four-map-owners.o");
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
                assert_four_owner_fault_cleanup(&object, session.runtime_archive(), &dir)
            },
        );
        std::fs::remove_dir_all(dir).unwrap();
        result.unwrap();
    });
}

fn assert_four_owner_fault_cleanup(object: &Path, archive: &Path, dir: &Path) -> Result<(), String> {
    let exe = dir.join("fault-probe-four-owners");
    let mut command = Command::new("cc");
    command
        .arg("-Wl,--wrap=main")
        .arg("-DHAKO_MAP_SOURCE_PROBE")
        .arg("-DHAKO_MAP_FOUR_OWNER_PROBE")
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
        ("normal", 30, 5, 5, 5, 0, 0, "10 20 1 2 30"),
        ("value-install-fault-1", 70, 1, 1, 1, 0, 101, "10"),
        ("value-install-fault-2", 70, 2, 2, 2, 0, 101, "10 20"),
        ("value-install-fault-3", 70, 3, 3, 3, 0, 101, "10 20 1"),
        ("value-install-fault-4", 70, 4, 4, 4, 0, 101, "10 20 1 2"),
        ("value-install-fault-5", 70, 5, 5, 5, 0, 101, "10 20 1 2 30"),
        ("value-outcome-fault-2", 70, 2, 2, 2, 0, 100, "10 20"),
        ("value-outcome-fault-5", 70, 5, 5, 5, 0, 100, "10 20 1 2 30"),
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
