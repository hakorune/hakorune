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
