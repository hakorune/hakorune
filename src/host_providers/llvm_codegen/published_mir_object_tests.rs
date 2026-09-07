use super::*;
use crate::mir::{MirCompiler, NormalCompileRequestV1};
use std::process::Command;

#[test]
#[ignore = "requires C FFI, LLVM18 and release lifecycle kernel"]
fn issued_pair_v4_direct_exe_and_linked_object_exit_30() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
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
            let parsed =
                crate::parser::NyashParser::parse_normal_callable_program_with_build_config(
                    &source,
                    crate::parser::ParserBuildConfig::default(),
                )
                .unwrap();
            let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) =
                crate::r#macro::transform_normal_callable_program_v1(parsed).unwrap()
            else {
                panic!("source-backed input required")
            };
            let request = NormalCompileRequestV1::for_mir_mode_callable_source(
                source,
                None,
                std::collections::HashMap::new(),
            );
            let dir = std::env::temp_dir().join(format!("hako-v4-{case}-{}", std::process::id()));
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
                    compile_published_view_object(view, object.to_str().unwrap(), Some(&session))?;
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
                    Ok(())
                })
                .unwrap();
            // Keep issued input only on failure, for diagnosing this exact production path.
            std::fs::remove_dir_all(dir).unwrap();
        }
    });
}
