//! Negative mutation of a real published Map program: neither writer
//! branch may turn a reserved borrowed carrier into an i64 transport.
use super::*;
use crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1 as Carrier;
use crate::mir::compiler::normal_default_pipeline::{MirCompiler, NormalCompileRequestV1};

#[test]
fn borrowed_carrier_writer_refuses_both_parameter_and_actual_encoding() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let source = "static box Helpers { read_k(m: MapBox): i64 { return m.get(\"k\") } } static box Main { main() { return read_k(%{\"k\" => 7}) } }";
        let parsed = crate::parser::NyashParser::parse_normal_callable_program_with_build_config(
            source,
            crate::parser::ParserBuildConfig::default(),
        )
        .unwrap();
        let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) =
            crate::r#macro::transform_normal_callable_program_v1(parsed).unwrap()
        else {
            panic!("source-backed")
        };
        let request = NormalCompileRequestV1::for_mir_mode_callable_source(
            source,
            None,
            std::collections::HashMap::new(),
        );
        MirCompiler::with_options(false)
            .compile_normal_with_published(request, |view, verification| -> Result<(), String> {
                assert!(verification.is_ok(), "{verification:?}");
                let input = view.issue_lifecycle_physical_abi_input()?;
                let mut program = input.program().clone();
                let callee = program
                    .functions
                    .iter_mut()
                    .find(|f| f.name == "Helpers.read_k/1")
                    .unwrap();
                assert_eq!(
                    callee.param_carriers,
                    Some(&[Carrier::CheckedMapStorage][..])
                );
                // Spoof only the transport metadata; this is deliberately not
                // a valid source-authorized borrowed call.
                callee.param_carriers = Some(&[Carrier::BorrowedTaggedValue]);
                for parameter_first in [true, false] {
                    program.functions.sort_by_key(|f| {
                        if parameter_first {
                            f.name == "main"
                        } else {
                            f.name != "main"
                        }
                    });
                    let first = &program.functions[0];
                    assert_eq!(first.name == "main", !parameter_first);
                    let error =
                        super::super::physical_program_json::emit_lifecycle_physical_program_value(
                            &program, None,
                        )
                        .unwrap_err();
                    assert!(
                        error.contains("borrowed-carrier-activation-missing"),
                        "{error}"
                    );
                }
                Ok(())
            })
            .unwrap();
    });
}
