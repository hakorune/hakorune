//! Source-backed readable-Map lane: the checked scalar `get` terminal.
//!
//! `main` owns one `%{...}` literal and returns `m.get("k")`. The probe wraps
//! `nyash.map.checked_get_i64_v1` and `checked_end_v1` to prove the read
//! passes the exact UTF-8 key, writes the i64 out-slot only on Normal, and
//! leaves the owned storage live until `checked_end` — on the Fault path too.
//! The trailing bookkeeping counts show the read itself allocates no key or
//! outcome storage: every count comes from the literal install alone.
use super::*;
use crate::mir::{emit_lifecycle_physical_abi_json, MirCompiler, NormalCompileRequestV1};
use std::path::Path;
use std::process::Command;

#[test]
#[ignore = "requires selected C FFI, LLVM18 and lifecycle runtime"]
fn issued_map_read_source_exe_probe_observes_get_contract_and_end() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        use crate::runner::modes::common_util::normal_callable::{
            materialize_normal_callable_program_v1, NormalCallableMaterializationOutcomeV1,
        };
        // (source, exit, expected READ tail). The out-slot stays -1 when the
        // kernel records Fault — it writes only on Normal.
        let cases: [(&str, i32, &str); 3] = [
            (
                "local m = %{\"k\" => 7} return m.get(\"k\")",
                7,
                "7 1 1 1 1 1 1 READ k 1 7 0 1 2 1\n",
            ),
            (
                "local m = %{} return m.get(\"k\")",
                0,
                "0 1 1 0 0 0 0 READ k 1 0 0 1 2 1\n",
            ),
            (
                "local m = %{\"k\" => true} return m.get(\"k\")",
                70,
                "70 1 1 1 1 1 1 READ k 1 -1 1 1 2 1\n",
            ),
        ];
        for (case, (body, expected_exit, expected_tail)) in cases.iter().enumerate() {
            let source = format!("static box Main {{ main() {{ {body} }} }}");
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
            let dir = std::env::temp_dir().join(format!(
                "hako-map-read-source-{}-{case}",
                std::process::id()
            ));
            std::fs::create_dir_all(&dir).unwrap();
            MirCompiler::with_options(true)
                .compile_normal_with_published(
                    request,
                    |view, verification| -> Result<(), String> {
                        assert!(verification.is_ok(), "{verification:?}");
                        let input = view.issue_lifecycle_physical_abi_input()?;
                        let json = emit_lifecycle_physical_abi_json(&input)?;
                        assert_eq!(
                            json.matches("\"kind\":\"map_checked_get\"").count(),
                            1,
                            "{json}"
                        );
                        assert!(json.contains("\"utf8\":\"k\""), "{json}");
                        let runtime = Path::new("target/lifecycle-kernel/release");
                        let session = LifecycleRuntimeSessionV1::select(
                            runtime.join("libnyash_lifecycle_kernel.a"),
                        )?;
                        let object = dir.join("map-read.o");
                        compile_published_view_object(
                            view,
                            object.to_str().unwrap(),
                            Some(&session),
                        )
                        .map_err(|error| format!("object: {error}"))?;
                        assert_map_read_probe(
                            &object,
                            session.runtime_archive(),
                            &dir,
                            *expected_exit,
                            expected_tail,
                        )
                    },
                )
                .unwrap_or_else(|e| panic!("case {case}: {e}"));
            std::fs::remove_dir_all(dir).unwrap();
        }
    });
}

/// Caller-side `%{...}` argument handoff: `main` builds the literal, hands
/// the storage pointer across the sealed edge, and keeps the End — the
/// borrowed callee only reads. The READ/END sequence must place the
/// callee's get before the caller's End on Normal and Fault alike.
#[test]
#[ignore = "requires selected C FFI, LLVM18 and lifecycle runtime"]
fn issued_map_argument_source_exe_probe_observes_borrowed_read_and_caller_end() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        use crate::runner::modes::common_util::normal_callable::{
            materialize_normal_callable_program_v1, NormalCallableMaterializationOutcomeV1,
        };
        const CALLEE: &str =
            "static box Helpers { read_k(m: MapBox): i64 { return m.get(\"k\") } }";
        // (body, mode arg, exit, expected tail). The caller's construction
        // bookkeeping matches the root-owned lane: one literal install,
        // the read itself allocates nothing. `prepare-fault` stops the
        // literal mid-construction; the End still runs on the caller's
        // Fault chain and the callee never sees the storage.
        let cases: [(&str, Option<&str>, i32, &str); 3] = [
            (
                "return read_k(%{\"k\" => 7})",
                None,
                7,
                "7 1 1 1 1 1 1 READ k 1 7 0 1 2 1\n",
            ),
            (
                "return read_k(%{\"k\" => true})",
                None,
                70,
                "70 1 1 1 1 1 1 READ k 1 -1 1 1 2 1\n",
            ),
            (
                "return read_k(%{\"k\" => 7})",
                Some("prepare-fault"),
                70,
                "70 1 1 1 1 0 0 READ  0 0 0 0 1 1\n",
            ),
        ];
        for (case, (body, mode, expected_exit, expected_tail)) in cases.iter().enumerate() {
            let source = format!("{CALLEE} static box Main {{ main() {{ {body} }} }}");
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
            let dir = std::env::temp_dir()
                .join(format!("hako-map-arg-source-{}-{case}", std::process::id()));
            std::fs::create_dir_all(&dir).unwrap();
            MirCompiler::with_options(true)
                .compile_normal_with_published(
                    request,
                    |view, verification| -> Result<(), String> {
                        assert!(verification.is_ok(), "{verification:?}");
                        let input = view.issue_lifecycle_physical_abi_input()?;
                        let json = emit_lifecycle_physical_abi_json(&input)?;
                        assert_eq!(
                            json.matches("\"kind\":\"map_checked_get\"").count(),
                            1,
                            "{json}"
                        );
                        assert!(json.contains("\"kind\":\"map\""), "{json}");
                        let runtime = Path::new("target/lifecycle-kernel/release");
                        let session = LifecycleRuntimeSessionV1::select(
                            runtime.join("libnyash_lifecycle_kernel.a"),
                        )?;
                        let object = dir.join("map-arg.o");
                        compile_published_view_object(
                            view,
                            object.to_str().unwrap(),
                            Some(&session),
                        )
                        .map_err(|error| format!("object: {error}"))?;
                        assert_map_read_probe_with_mode(
                            &object,
                            session.runtime_archive(),
                            &dir,
                            *mode,
                            *expected_exit,
                            expected_tail,
                        )
                    },
                )
                .unwrap_or_else(|e| panic!("case {case}: {e}"));
            std::fs::remove_dir_all(dir).unwrap();
        }
    });
}

fn assert_map_read_probe(
    object: &Path,
    archive: &Path,
    dir: &Path,
    expected_exit: i32,
    expected_tail: &str,
) -> Result<(), String> {
    assert_map_read_probe_with_mode(object, archive, dir, None, expected_exit, expected_tail)
}

fn assert_map_read_probe_with_mode(
    object: &Path,
    archive: &Path,
    dir: &Path,
    mode: Option<&str>,
    expected_exit: i32,
    expected_tail: &str,
) -> Result<(), String> {
    let exe = dir.join("fault-probe-map-read");
    let mut command = Command::new("cc");
    command
        .arg("-Wl,--wrap=main")
        .arg("-DHAKO_MAP_SOURCE_PROBE")
        .arg("-DHAKO_MAP_READ_PROBE")
        .arg("lang/c-abi/tests/published_map_fault_probe.c")
        .arg(object)
        .arg(archive)
        .arg("-Wl,--wrap=nyash.map.checked_get_i64_v1");
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
    let mut run = Command::new(&exe);
    if let Some(mode) = mode {
        run.arg(mode);
    }
    let result = run
        .env("NYASH_NYRT_SILENT_RESULT", "1")
        .env("HAKO_NYRT_PLUGIN_HOST", "off")
        .output()
        .map_err(|e| e.to_string())?;
    assert_eq!(result.status.code(), Some(expected_exit), "{result:?}");
    let stdout = String::from_utf8_lossy(&result.stdout);
    if expected_exit == 70 {
        let reason = if mode.is_some() {
            "REPORT 100 "
        } else {
            "REPORT 104 "
        };
        assert!(stdout.contains(reason), "{stdout}");
    }
    assert!(stdout.ends_with(expected_tail), "{stdout}");
    Ok(())
}
