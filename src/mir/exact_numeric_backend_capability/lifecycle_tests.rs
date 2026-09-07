use super::*;
use crate::mir::{BasicBlockId, MirCompiler, NormalCompileRequestV1, ValueId};

#[test]
fn issued_pair_covers_exact_contracts_without_opening_generic_obj() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let parsed = crate::parser::NyashParser::parse_normal_callable_program_with_build_config(
            include_str!("../../../apps/typed-object-birth-min/main.hako"),
            crate::parser::ParserBuildConfig::default(),
        )
        .unwrap();
        let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) =
            crate::r#macro::transform_normal_callable_program_v1(parsed).unwrap()
        else {
            panic!("source")
        };
        MirCompiler::with_options(true)
            .compile_normal_with_published(
                NormalCompileRequestV1::for_mir_mode_callable_source(
                    source,
                    None,
                    Default::default(),
                ),
                |view, _| -> Result<(), String> {
                    let input = view.issue_lifecycle_physical_abi_input()?;
                    crate::mir::backend_capability::enforce_published_lifecycle_backend_supported(
                        view, &input,
                    )?;
                    assert!(
                        crate::mir::backend_capability::enforce_published_backend_supported(
                            view,
                            "ny-llvmc-obj"
                        )
                        .unwrap_err()
                        .contains("contracts=2")
                    );
                    assert!(enforce(&view.module().clone(), &input)
                        .unwrap_err()
                        .contains("foreign-input"));
                    let (name, function) = view
                        .module()
                        .functions
                        .iter()
                        .find(|(_, f)| !f.metadata.exact_numeric_runtime_check_contracts.is_empty())
                        .unwrap();
                    let contracts = &function.metadata.exact_numeric_runtime_check_contracts;
                    assert_eq!(contracts.len(), 2);
                    let changes: &[(
                        &str,
                        fn(&mut crate::mir::function::ExactNumericRuntimeCheckContract),
                    )] = &[
                        ("value", |c| c.value = ValueId(999)),
                        ("block", |c| c.block = BasicBlockId(999)),
                        ("index", |c| c.instruction_index = 999),
                        ("field", |c| c.field.push_str("_foreign")),
                        ("type", |c| c.declared_type_name = "usize".into()),
                    ];
                    for (label, change) in changes {
                        let mut changed = contracts.clone();
                        change(&mut changed[0]);
                        assert!(
                            enforce_contracts(view.module(), &input, name, &changed).is_err(),
                            "{label}"
                        );
                    }
                    let mut duplicate = contracts.clone();
                    duplicate.push(contracts[0].clone());
                    assert!(enforce_contracts(view.module(), &input, name, &duplicate)
                        .unwrap_err()
                        .contains("duplicate-contract"));
                    assert!(enforce_contracts(
                        view.module(),
                        &input,
                        "not-this-function",
                        contracts
                    )
                    .is_err());
                    // Missing metadata must fail the mandatory published preflight,
                    // before the selected numeric consumer can see an empty list.
                    let mut missing = view.module().clone();
                    missing
                        .functions
                        .get_mut(name)
                        .unwrap()
                        .metadata
                        .exact_numeric_runtime_check_contracts
                        .clear();
                    assert!(
                        crate::mir::semantic_refresh::validate_published_contracts(&missing)
                            .is_err()
                    );
                    Ok(())
                },
            )
            .unwrap();
    });
}
