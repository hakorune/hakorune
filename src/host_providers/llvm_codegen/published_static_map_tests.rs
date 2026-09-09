//! Natural Script Map through the generic copy/boxed V2 host contract.
use super::*;
use crate::mir::{MirCompiler, NormalCompileRequestV1};

#[test]
#[ignore = "requires C FFI, LLVM18 and quick kernel archive"]
fn static_map_source_v2_direct_and_linked_objects() {
    std::thread::Builder::new().stack_size(32 * 1024 * 1024)
        .spawn(run_static_map_sources).unwrap().join().unwrap();
}

fn run_static_map_sources() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        for source in [
            "local m = %{} return 30",
            "local m = %{\"bool\" => true, \"float\" => 1.5, \"void\" => null, \"text\" => \"日本語\"} return 30",
            "box MapBox { birth() {} } local m = %{} return 30",
            "local m = %{\"a\" => 1, \"a\" => 2} return 30",
            "local m = %{\"nested\" => %{\"v\" => true}} return 30",
            "local x = 7 local m = %{\"a\" => x, \"nested\" => %{\"v\" => x}} return 30",
        ] {
            use crate::runner::modes::common_util::normal_callable::{
                materialize_normal_callable_program_v1, NormalCallableMaterializationOutcomeV1,
            };
            let NormalCallableMaterializationOutcomeV1::SourceBacked(input) =
                materialize_normal_callable_program_v1(source.to_owned(),
                    crate::parser::ParserBuildConfig::default()).unwrap()
            else { panic!("source-backed Script input") };
            let request = NormalCompileRequestV1::for_mir_mode_callable_source(
                input, None, std::collections::HashMap::new(),
            );
            let directory = tempfile::tempdir().unwrap();
            MirCompiler::with_options(true).compile_normal_with_published(request, |view, verified| {
                assert!(verified.is_ok(), "{verified:?}");
                let body = crate::runner::mir_json_emit::emit_published_view_body(view)?;
                assert!(body.contains("intrinsic_map"));
                assert!(!body.contains("\"method\":\"set\""));
                let executable = directory.path().join("direct");
                assert!(emit_published_view_exe(view, executable.to_str().unwrap(), Some("target/quick"), None)?);
                assert_eq!(std::process::Command::new(executable).status().unwrap().code(), Some(30));
                let object = directory.path().join("separate.o");
                assert!(try_compile_published_view_object(view, object.to_str().unwrap(), None)?);
                let linked = directory.path().join("linked");
                super::super::link_object_capi_v2(&object, &linked,
                    Path::new("target/quick/libnyash_kernel.a"), None)?;
                assert_eq!(std::process::Command::new(linked).status().unwrap().code(), Some(30));
                Ok(())
            }).unwrap_or_else(|error| panic!("{source}: {error}"));
        }
    });
}
