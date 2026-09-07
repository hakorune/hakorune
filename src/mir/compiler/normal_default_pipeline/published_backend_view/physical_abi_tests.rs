use super::*;
use crate::mir::{MirCompiler, NormalCompileRequestV1};

#[test]
fn numeric_capability_rejects_drift_in_referenced_physical_layout() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let parsed = crate::parser::NyashParser::parse_normal_callable_program_with_build_config(
            include_str!("../../../../../apps/typed-object-birth-min/main.hako"),
            crate::parser::ParserBuildConfig::default(),
        )
        .unwrap();
        let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) =
            crate::r#macro::transform_normal_callable_program_v1(parsed).unwrap()
        else {
            panic!("source")
        };
        MirCompiler::with_options(true).compile_normal_with_published(
            NormalCompileRequestV1::for_mir_mode_callable_source(source, None, Default::default()),
            |view, _| -> Result<(), String> {
                let input = view.issue_lifecycle_physical_abi_input()?;
                for index in 0..6 {
                    let mut bad = input.clone();
                    let object = &mut bad.layouts[0];
                    match index {
                        0 => object.runtime_type_id += 1,
                        1 => object.field_count += 1,
                        2 => object.fields[0].object_id += 1,
                        3 => object.fields[0].declaration_ordinal = 999,
                        4 => object.fields[0].runtime_slot += 1,
                        5 => object.fields[0].storage_kind += 1,
                        _ => unreachable!(),
                    }
                    assert!(crate::mir::backend_capability::enforce_published_lifecycle_backend_supported(view, &bad)
                        .unwrap_err().contains("lifecycle-numeric/"), "layout mutation {index}");
                }
                Ok(())
            }).unwrap();
    });
}
