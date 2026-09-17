//! Source-backed C8 map-consumer artifact and runtime Fault evidence.
//!
//! `main -> use_map -> make_map`: the non-AppMain `use_map` owner receives a
//! Map-result invoke lease, holds it live across its own literal, and ends
//! both. The probe follows the returned map's storage from callee install
//! through `storage_move` into the caller's `checked_end`, and injects Fault
//! into the callee install (ordinal 1) and the caller install (ordinal 2).
use super::*;
use crate::mir::{emit_lifecycle_physical_abi_json, MirCompiler, NormalCompileRequestV1};
use std::path::Path;
use std::process::Command;

#[test]
#[ignore = "requires selected C FFI, LLVM18 and lifecycle runtime"]
fn issued_map_consumer_exe_probe_observes_returned_map_and_fault_release() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        use crate::runner::modes::common_util::normal_callable::{
            materialize_normal_callable_program_v1, NormalCallableMaterializationOutcomeV1,
        };
        let source = r#"static box Main {
            main() { return use_map(10) }
            use_map(seed: i64): i64 { local m = make_map() local n = %{"b" => 2} return 42 }
            make_map() { return %{"a" => 1} }
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
        let dir =
            std::env::temp_dir().join(format!("hako-map-consumer-source-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let result = MirCompiler::with_options(true).compile_normal_with_published(
            request,
            |view, verification| -> Result<(), String> {
                assert!(verification.is_ok(), "{verification:?}");
                let input = view.issue_lifecycle_physical_abi_input()?;
                let json = emit_lifecycle_physical_abi_json(&input)?;
                // The lifecycle route reaches `make_map` only through the
                // map-result call edge; a scalar Call would leave the
                // lifecycle-bearing callee uncataloged.
                assert_eq!(json.matches("\"result\":\"map\"").count(), 1, "{json}");
                assert_eq!(json.matches("ordinary_map").count(), 1, "{json}");
                let runtime = Path::new("target/lifecycle-kernel/release");
                let session =
                    LifecycleRuntimeSessionV1::select(runtime.join("libnyash_lifecycle_kernel.a"))?;
                let object = dir.join("map-consumer.o");
                compile_published_view_object(view, object.to_str().unwrap(), Some(&session))
                    .map_err(|error| format!("object: {error}"))?;
                assert_map_consumer_probe(&object, session.runtime_archive(), &dir)
            },
        );
        std::fs::remove_dir_all(dir).unwrap();
        result.unwrap();
    });
}

fn assert_map_consumer_probe(object: &Path, archive: &Path, dir: &Path) -> Result<(), String> {
    let exe = dir.join("fault-probe-map-consumer");
    let mut command = Command::new("cc");
    command
        .arg("-Wl,--wrap=main")
        .arg("-DHAKO_MAP_SOURCE_PROBE")
        .arg("-DHAKO_MAP_CONSUMER_PROBE")
        .arg("lang/c-abi/tests/published_map_fault_probe.c")
        .arg(object)
        .arg(archive)
        .arg("-Wl,--wrap=nyash.map.checked_install_value_v1")
        .arg("-Wl,--wrap=nyash.map.storage_move_v1");
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
    // mode, exit, report reason, expected END lines (each resolves the ended
    // storage's installed entry through the storage_move chain), and the
    // trailing counts: result, map init/dispose, key init/dispose, outcome
    // init/dispose, storage_move.
    let cases: [(&str, i32, u32, &[&str], &str); 6] = [
        (
            "normal",
            42,
            0,
            &["END b=2 local", "END a=1 moved"],
            "42 2 3 2 2 2 2 1\n",
        ),
        // Callee fault: `a=>1` never reaches the map; make_map's own cleanup
        // ends the still-live lease and no storage_move ever runs.
        (
            "value-install-fault-1",
            70,
            101,
            &["END empty local"],
            "70 1 1 1 1 1 1 0\n",
        ),
        // Caller fault: `b=>2` fails while the received lease is live; the
        // caller's fault cleanup ends both, and `a=1` is still carried by the
        // moved callee storage.
        (
            "value-install-fault-2",
            70,
            101,
            &["END empty local", "END a=1 moved"],
            "70 2 3 2 2 2 2 1\n",
        ),
        (
            "value-outcome-fault-1",
            70,
            100,
            &["END a=1 local"],
            "70 1 1 1 1 1 1 0\n",
        ),
        (
            "value-outcome-fault-2",
            70,
            100,
            &["END b=2 local", "END a=1 moved"],
            "70 2 3 2 2 2 2 1\n",
        ),
        (
            "end-fault",
            70,
            100,
            &["END b=2 local", "END a=1 moved"],
            "70 2 3 2 2 2 2 1\n",
        ),
    ];
    for (mode, expected, reason, ends, tail) in cases {
        let result = Command::new(&exe)
            .arg(mode)
            .env("NYASH_NYRT_SILENT_RESULT", "1")
            .env("HAKO_NYRT_PLUGIN_HOST", "off")
            .output()
            .map_err(|e| e.to_string())?;
        assert_eq!(result.status.code(), Some(expected), "{mode}: {result:?}");
        let stdout = String::from_utf8_lossy(&result.stdout);
        if reason != 0 {
            let report = format!("REPORT {reason} ");
            assert!(stdout.contains(&report), "{mode}: {stdout}");
        }
        for end in ends {
            assert!(stdout.contains(end), "{mode}: {stdout}");
        }
        assert!(stdout.ends_with(tail), "{mode}: {stdout}");
    }
    Ok(())
}
