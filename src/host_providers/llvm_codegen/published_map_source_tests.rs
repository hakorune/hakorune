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
                        Ok(())
                    },
                )
                .unwrap_or_else(|e| panic!("case {case}: {e}"));
            std::fs::remove_dir_all(dir).unwrap();
        }
    });
}
