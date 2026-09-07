use super::*;
use crate::mir::{MirCompiler, NormalCompileRequestV1};
use std::process::Command;

#[test]
#[ignore = "requires C FFI, LLVM18 and release lifecycle kernel"]
fn issued_pair_v4_direct_exe_and_linked_object_exit_30() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_vars(
        &[
            ("NYASH_MACRO_DISABLE", None),
            ("NYASH_MACRO_ENABLE", None),
            ("NYASH_MACRO_DERIVE", None),
            ("NYASH_MACRO_DERIVE_ALL", None),
            ("NYASH_MACRO_PATHS", None),
            ("NYASH_TEST_RUN", None),
        ],
        || {
            let original = include_str!("../../../apps/typed-object-birth-min/main.hako");
            for (case, source, expected) in [
                ("pair", original.to_owned(), 30),
                (
                    "bool-first",
                    original.replace("new Pair(10, 20)", "new Pair(true, 20)"),
                    70,
                ),
                (
                    "bool-second",
                    original.replace("new Pair(10, 20)", "new Pair(10, false)"),
                    70,
                ),
            ] {
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
                    panic!("source-backed input required")
                };
                let request = NormalCompileRequestV1::for_mir_mode_callable_source(
                    source,
                    None,
                    std::collections::HashMap::new(),
                );
                let dir =
                    std::env::temp_dir().join(format!("hako-v4-{case}-{}", std::process::id()));
                std::fs::create_dir_all(&dir).unwrap();
                let runtime_dir = Path::new("target/lifecycle-kernel/release");
                let mut compiler = MirCompiler::with_options(true);
                compiler
                    .compile_normal_with_published(request, |view, _| -> Result<(), String> {
                        let json = emit_lifecycle_physical_abi_json(
                            &view.issue_lifecycle_physical_abi_input()?,
                        )?;
                        std::fs::write(dir.join("issued.json"), json).map_err(|e| e.to_string())?;
                        let direct = dir.join("direct");
                        assert!(emit_published_view_exe(
                            view,
                            direct.to_str().unwrap(),
                            runtime_dir.to_str(),
                            None
                        )?);
                        let session = LifecycleRuntimeSessionV1::select(
                            runtime_dir.join("libnyash_lifecycle_kernel.a"),
                        )?;
                        let object = dir.join("pair.o");
                        let linked = dir.join("linked");
                        compile_published_view_object(
                            view,
                            object.to_str().unwrap(),
                            Some(&session),
                        )?;
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
                            assert_eq!(
                                output.status.code(),
                                Some(expected),
                                "{}: stdout={} stderr={}",
                                exe.display(),
                                String::from_utf8_lossy(&output.stdout),
                                String::from_utf8_lossy(&output.stderr)
                            );
                            if expected == 70 {
                                assert!(String::from_utf8_lossy(&output.stderr)
                                    .contains("[fault:primary] reason=103"));
                            }
                        }
                        assert_runtime_cleanup_probe(
                            &object,
                            session.runtime_archive(),
                            &dir,
                            case,
                            expected,
                        )?;
                        Ok(())
                    })
                    .unwrap();
                // Keep issued input only on failure, for diagnosing this exact production path.
                std::fs::remove_dir_all(dir).unwrap();
            }
        },
    );
}

// Observe the exact host-emitted object against the actual selected runtime.
// Wrappers report calls; they do not modify source, input or runtime status.
fn assert_runtime_cleanup_probe(
    object: &Path,
    archive: &Path,
    dir: &Path,
    case: &str,
    expected: i32,
) -> Result<(), String> {
    let exe = dir.join("cleanup-probe");
    let mut cc = Command::new("cc");
    cc.arg(object)
        .arg("lang/c-abi/tests/published_lifecycle_v4_runtime_probe.c")
        .arg(archive);
    for name in [
        "fault.frame_init",
        "fault.frame_dispose",
        "fault.report_final",
        "object.checked_field_set",
        "object.home_release_plain_i64",
        "object.reclaim_unpublished",
    ] {
        cc.arg(format!("-Wl,--wrap=nyash.{name}_v1"));
    }
    let linked = cc
        .args(["-lpthread", "-ldl", "-lm", "-o"])
        .arg(&exe)
        .output()
        .map_err(|e| e.to_string())?;
    assert!(
        linked.status.success(),
        "{}",
        String::from_utf8_lossy(&linked.stderr)
    );
    let result = Command::new(&exe)
        .env("V4_PROBE_MODE", "normal")
        .env("HAKO_NYRT_PLUGIN_HOST", "off")
        .output()
        .map_err(|e| e.to_string())?;
    let stdout = String::from_utf8_lossy(&result.stdout);
    assert_eq!(result.status.code(), Some(expected), "{stdout}");
    let (counts, disposal) = match case {
        "pair" => (
            "COUNTS 1 2 1 0 0 1\n",
            "DISPOSE HOME 1 RECLAIM 0 REPORT 0\n",
        ),
        "bool-first" => (
            "COUNTS 1 0 0 1 1 1\n",
            "DISPOSE HOME 0 RECLAIM 1 REPORT 1\n",
        ),
        "bool-second" => (
            "COUNTS 1 1 0 1 1 1\n",
            "DISPOSE HOME 0 RECLAIM 1 REPORT 1\n",
        ),
        _ => unreachable!("finite source cases"),
    };
    assert!(stdout.contains(counts), "{stdout}");
    assert!(stdout.contains(disposal), "{stdout}");
    if expected == 70 {
        let fault = stdout
            .lines()
            .find(|line| line.starts_with("FAULT 103 "))
            .expect("Fault103");
        assert!(fault.ends_with("HOME 0 RECLAIM 1"), "{stdout}");
        assert!(
            stdout.find(fault).unwrap() < stdout.find(disposal).unwrap(),
            "{stdout}"
        );
        assert!(String::from_utf8_lossy(&result.stderr).contains("[fault:primary] reason=103"));
    } else {
        assert!(!stdout.contains("FAULT "), "{stdout}");
    }
    Ok(())
}
