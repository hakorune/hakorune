//! Retained source -> production host OBJ/EXE plus physical rejection evidence.
use super::{emit_published_view_exe, try_compile_published_view_object};
use crate::mir::{MirCompiler, NormalCompileRequestV1};
use std::path::PathBuf;
use std::process::Command;

#[test]
#[ignore = "requires selected C FFI and llc-18"]
fn retained_script_inputs_reach_native_c_and_reject_physical_mutations() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let directory = std::env::temp_dir().join(format!("hako native C {}", std::process::id()));
        std::fs::create_dir_all(&directory).unwrap();
        struct Cleanup(PathBuf);
        impl Drop for Cleanup {
            fn drop(&mut self) {
                let _ = std::fs::remove_dir_all(&self.0);
            }
        }
        let _cleanup = Cleanup(directory.clone());
        let mut cases = Vec::new();
        for spec in ["i8", "i16", "i32", "i64", "u8", "u16", "u32"] {
            for optimize in [false, true] {
                for terminal in ["return 30", "return"] {
                    let source = format!("local before = 9\nlocal a: Array<{spec}> = [10, 20]\nlocal alias = a\nlocal b: Array<{spec}> = []\n{terminal}");
                    cases.push((source, optimize));
                }
            }
        }
        for values in ["true, 22", "1.25, 22", "128, 22"] {
            cases.push((
                format!("local prior: Array<i8> = [7]\nlocal a: Array<i8> = [{values}]\nreturn 30"),
                true,
            ));
        }
        for result in [0i64, 255, 256, i64::MAX] {
            for optimize in [false, true] {
                cases.push((
                    format!("local a: Array<i64> = [10, 20]\nlocal alias = a\nlocal b: Array<u8> = []\nreturn {result}"),
                    optimize,
                ));
            }
        }
        for (index, (source, optimize)) in cases.iter().enumerate() {
            let parsed =
                crate::parser::NyashParser::parse_normal_callable_program_with_build_config(
                    source,
                    crate::parser::ParserBuildConfig::default(),
                )
                .unwrap();
            let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) =
                crate::r#macro::transform_normal_callable_program_v1(parsed).unwrap()
            else {
                panic!("retained source required")
            };
            MirCompiler::with_options(*optimize)
                .compile_normal_with_published(
                    NormalCompileRequestV1::for_mir_mode_callable_source(
                        source,
                        None,
                        Default::default(),
                    ),
                    |view, verification| -> Result<(), String> {
                        assert!(verification.is_ok());
                        let runtime = "target/lifecycle-kernel/release";
                        let object = directory.join(format!("{index}.host.o"));
                        assert!(try_compile_published_view_object(
                            view,
                            object.to_str().unwrap(),
                            Some(runtime),
                        )?);
                        let direct = directory.join(format!("{index}.host-exe"));
                        assert!(emit_published_view_exe(
                            view,
                            direct.to_str().unwrap(),
                            Some(runtime),
                            None,
                        )?);
                        let expected = if index >= 31 {
                            [0, 255, 70, 70][(index - 31) / 2]
                        } else if index >= 28 {
                            70
                        } else if index % 2 == 0 {
                            30
                        } else {
                            0
                        };
                        let result = Command::new(direct)
                            .env("NYASH_NYRT_SILENT_RESULT", "1")
                            .env("HAKO_NYRT_PLUGIN_HOST", "off")
                            .output()
                            .unwrap();
                        assert_eq!(
                            result.status.code(),
                            Some(expected),
                            "case {index}: {result:?}"
                        );
                        let input = view.issue_lifecycle_physical_abi_input()?;
                        std::fs::write(
                            directory.join(format!("{index}.json")),
                            crate::mir::emit_lifecycle_physical_abi_json(&input)?,
                        )
                        .unwrap();
                        Ok(())
                    },
                )
                .unwrap();
        }
        let output = Command::new("python3")
            .arg(root.join("lang/c-abi/tests/published_native_array_physical_test.py"))
            .arg(&directory)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    });
}
