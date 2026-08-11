use crate::mir::callable_semantic_batch::{
    issue_resolved_callable_semantic_batch_v1, VerifiedResolvedCallableSemanticBatchV1,
};
use crate::mir::resolved_semantics::{
    BindingKindV1, FunctionSemanticResolverSessionV1, SourceBindingSiteV1,
};
use crate::parser::{NyashParser, ParserBuildConfig};

use super::{
    issue_callable_parameter_contract_v1, CallableParameterContractKindV1,
    CallableParameterDeclarationModeV1,
};

fn batch(source: &str, compilation: u32) -> VerifiedResolvedCallableSemanticBatchV1 {
    let parsed = NyashParser::parse_normal_callable_program_with_build_config(
        source,
        ParserBuildConfig::default(),
    )
    .expect("normal callable source parses");
    let source = crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let transformed = crate::r#macro::transform_normal_callable_program_v1(parsed)
            .expect("exact callable transform");
        let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = transformed
        else {
            panic!("fixture must remain source-backed")
        };
        source
    });
    let mut resolver =
        FunctionSemanticResolverSessionV1::new(compilation).expect("resolver session opens");
    issue_resolved_callable_semantic_batch_v1(&mut resolver, source)
        .expect("complete resolved callable semantic batch")
}

#[test]
fn projects_exact_i64_and_opaque_parameters_from_one_batch() {
    let batch = batch(
        r#"
static box StaticApi {
    run(source, pos: i64, end: i64, tail) { return end }
    ping() { return 0 }
}
box InstanceApi {
    read(offset) { return offset }
}
"#,
        7,
    );
    let catalog = issue_callable_parameter_contract_v1(&batch)
        .expect("complete parameter contract projection");
    let declarations = catalog.declarations().collect::<Vec<_>>();

    assert_eq!(declarations.len(), 3);
    assert_eq!(
        declarations[0].mode(),
        CallableParameterDeclarationModeV1::StaticBoxMethod
    );
    assert_eq!(
        declarations[0]
            .parameters()
            .iter()
            .map(|row| row.kind())
            .collect::<Vec<_>>(),
        [
            CallableParameterContractKindV1::OpaqueHandle,
            CallableParameterContractKindV1::ExactTrivial(
                crate::mir::exact_trivial_parameter_abi::ExactTrivialParameterAbiV1::I64
            ),
            CallableParameterContractKindV1::ExactTrivial(
                crate::mir::exact_trivial_parameter_abi::ExactTrivialParameterAbiV1::I64
            ),
            CallableParameterContractKindV1::OpaqueHandle,
        ]
    );
    assert!(declarations[0]
        .parameters()
        .iter()
        .zip([false, true, true, false])
        .all(|(row, is_trivial)| (row.home_demand()
            == crate::mir::resolved_semantics::HomeDemandV1::Trivial)
            == is_trivial));
    assert_eq!(declarations[1].parameters().len(), 0);
    assert_eq!(
        declarations[2].mode(),
        CallableParameterDeclarationModeV1::InstanceBoxMethod
    );

    for declaration in declarations {
        batch
            .with_lowering_input(declaration.batch_slot(), |input| {
                assert_eq!(input.owner(), declaration.owner());
                assert_eq!(
                    input.function().function_origin(),
                    declaration.function_origin()
                );
                for parameter in declaration.parameters() {
                    assert_eq!(parameter.binding().owner(), declaration.owner());
                    assert_eq!(
                        input
                            .function()
                            .declaration_binding(&SourceBindingSiteV1::Parameter {
                                index: parameter.ordinal(),
                            })
                            .expect("exact parameter declaration binding"),
                        parameter.binding()
                    );
                    assert_eq!(
                        input
                            .function()
                            .binding(parameter.binding())
                            .expect("exact resolved parameter")
                            .kind(),
                        BindingKindV1::Parameter {
                            index: parameter.ordinal(),
                        }
                    );
                }
            })
            .expect("same-batch lowering loan");
    }
}

#[test]
fn absent_ordinary_type_is_opaque_handle_not_exact_trivial() {
    let batch = batch("static box Api { run(value) { return value } }", 8);
    let catalog = issue_callable_parameter_contract_v1(&batch).unwrap();
    let parameter = &catalog.declarations().next().unwrap().parameters()[0];
    assert_eq!(
        parameter.kind(),
        CallableParameterContractKindV1::OpaqueHandle
    );
}

#[test]
fn parser_scan_loop_box_preserves_exact_i64_parameter_contracts() {
    let batch = batch(
        include_str!("../../../lang/src/compiler/parser/scan/parser_scan_loop_box.hako"),
        8,
    );
    let catalog = issue_callable_parameter_contract_v1(&batch).unwrap();
    let declarations = catalog.declarations().collect::<Vec<_>>();
    assert_eq!(
        declarations
            .iter()
            .map(|row| row.parameters().len())
            .collect::<Vec<_>>(),
        [4, 3, 4, 4]
    );
    assert_eq!(
        declarations[0]
            .parameters()
            .iter()
            .map(|row| row.kind())
            .collect::<Vec<_>>(),
        vec![
            CallableParameterContractKindV1::OpaqueHandle,
            CallableParameterContractKindV1::ExactTrivial(
                crate::mir::exact_trivial_parameter_abi::ExactTrivialParameterAbiV1::I64
            ),
            CallableParameterContractKindV1::ExactTrivial(
                crate::mir::exact_trivial_parameter_abi::ExactTrivialParameterAbiV1::I64
            ),
            CallableParameterContractKindV1::OpaqueHandle,
        ]
    );
    assert!(declarations[1..].iter().all(|declaration| declaration
        .parameters()
        .iter()
        .all(|row| row.kind() == CallableParameterContractKindV1::OpaqueHandle)));
}

#[test]
fn unsupported_explicit_type_rejects_without_opaque_fallback() {
    let batch = batch("static box Api { run(value: f64) { return value } }", 9);
    assert!(matches!(
        issue_callable_parameter_contract_v1(&batch),
        Err(
            super::CallableParameterContractIssueV1::UnsupportedDeclaredType {
                declaration: 0,
                parameter: 0,
            }
        )
    ));
}

#[test]
fn top_level_rows_remain_outside_direct_method_contract_catalog() {
    let batch = batch(
        "function helper(value: i64) { return value }\n\
         static box StaticApi { run(value) { return value } }",
        10,
    );
    let catalog = issue_callable_parameter_contract_v1(&batch).unwrap();
    let declarations = catalog.declarations().collect::<Vec<_>>();
    assert_eq!(declarations.len(), 1);
    assert_eq!(declarations[0].batch_slot(), 1);
}

#[test]
fn foreign_batches_keep_distinct_parameter_binding_identity() {
    let source = "static box Source { run(value) { return value } }";
    let first = batch(source, 11);
    let second = batch(source, 11);
    let first_catalog = issue_callable_parameter_contract_v1(&first).unwrap();
    let first_row = first_catalog.declarations().next().unwrap();
    let second_catalog = issue_callable_parameter_contract_v1(&second).unwrap();
    let second_row = second_catalog.declarations().next().unwrap();
    assert_ne!(first_row.owner(), second_row.owner());
    assert_ne!(
        first_row.parameters()[0].binding(),
        second_row.parameters()[0].binding()
    );
}

#[test]
fn issuer_keeps_resolver_and_forest_outside_the_contract_owner() {
    let issuer = include_str!("issuer.rs");
    let model = include_str!("model.rs");
    for forbidden in [
        "resolve_selected_callable_forests",
        "FunctionSemanticResolverSessionV1",
        "VerifiedSemanticOwnerForestV1",
        "resolved_forest",
        "ValueId",
        "MirType",
        "Recipe",
    ] {
        assert!(!issuer.contains(forbidden), "issuer retained {forbidden}");
        assert!(!model.contains(forbidden), "model retained {forbidden}");
    }
}
