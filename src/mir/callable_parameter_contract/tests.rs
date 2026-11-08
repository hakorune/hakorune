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
fn usize_parameter_projects_exact_trivial_not_unsupported_or_handle() {
    let batch = batch(
        "static box Api { run(count: usize, pos: i64) { return pos } }",
        7,
    );
    let catalog = issue_callable_parameter_contract_v1(&batch).unwrap();
    let declaration = catalog.declarations().next().unwrap();
    assert_eq!(
        declaration
            .parameters()
            .iter()
            .map(|row| row.kind())
            .collect::<Vec<_>>(),
        [
            CallableParameterContractKindV1::ExactTrivial(
                crate::mir::exact_trivial_parameter_abi::ExactTrivialParameterAbiV1::USIZE
            ),
            CallableParameterContractKindV1::ExactTrivial(
                crate::mir::exact_trivial_parameter_abi::ExactTrivialParameterAbiV1::I64
            ),
        ]
    );
    assert!(declaration.parameters().iter().all(|row| {
        row.home_demand() == crate::mir::resolved_semantics::HomeDemandV1::Trivial
    }));
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
    let take = self::batch("box Payload {} static box Api { run(take value: Payload) { return 0 } }", 9);
    let catalog = issue_callable_parameter_contract_v1(&take).unwrap();
    let declarations = catalog.declarations().collect::<Vec<_>>();
    let parameter = &declarations.last().unwrap().parameters()[0];
    assert!(matches!(parameter.kind(), CallableParameterContractKindV1::DeclaredObject(_)));
    assert_eq!(parameter.home_demand(), crate::mir::resolved_semantics::HomeDemandV1::Home);
    assert_eq!(parameter.kind().home_demand(), crate::mir::resolved_semantics::HomeDemandV1::Handle);
}

#[test]
fn projects_exact_text_without_reinterpreting_it_as_an_opaque_handle() {
    let batch = batch(
        "static box TextApi { run(source: StringBox, needle: StringBox) { return 0 } }",
        81,
    );
    let catalog = issue_callable_parameter_contract_v1(&batch).unwrap();
    let declaration = catalog.declarations().next().unwrap();
    let parameters = declaration.parameters();

    assert_eq!(parameters.len(), 2);
    assert!(parameters.iter().all(|parameter| matches!(
        parameter.kind(),
        CallableParameterContractKindV1::ExactText(
            crate::mir::exact_text_parameter_abi::ExactTextFormalAbiV1::STRING_BOX
        )
    )));
    assert!(parameters
        .iter()
        .all(|parameter| parameter.home_demand()
            == crate::mir::resolved_semantics::HomeDemandV1::Handle));
    assert_eq!(parameters[0].ordinal(), 0);
    assert_eq!(parameters[1].ordinal(), 1);
    assert_ne!(parameters[0].binding(), parameters[1].binding());

    // The ADR spelling `String` is the same exact Text contract.
    let spelled = self::batch(
        "static box TextApi { run(source: String, needle: StringBox) { return 0 } }",
        82,
    );
    let catalog = issue_callable_parameter_contract_v1(&spelled).unwrap();
    assert!(catalog.declarations().next().unwrap().parameters().iter().all(|parameter| {
        matches!(
            parameter.kind(),
            CallableParameterContractKindV1::ExactText(
                crate::mir::exact_text_parameter_abi::ExactTextFormalAbiV1::STRING_BOX
            )
        )
    }));
}

#[test]
fn user_declared_string_box_wins_over_the_text_spelling() {
    let batch = batch("box String { v: i64 } static box Api { run(value: String) { return 0 } }", 83);
    let catalog = issue_callable_parameter_contract_v1(&batch).unwrap();
    let declarations = catalog.declarations().collect::<Vec<_>>();
    let parameter = &declarations.last().unwrap().parameters()[0];
    assert!(
        !matches!(parameter.kind(), CallableParameterContractKindV1::ExactText(_)),
        "a declared `box String` is the class authority, got {:?}", parameter.kind()
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
fn declared_box_names_project_as_handle_not_opaque_or_exact() {
    let batch = batch(
        "static box BoxHelpers {\n\
         \x20   array_len(arr: ArrayBox) { return 0 }\n\
         \x20   map_get(obj: MapBox, key) { return key }\n\
         }",
        12,
    );
    let catalog = issue_callable_parameter_contract_v1(&batch).unwrap();
    let declarations = catalog.declarations().collect::<Vec<_>>();
    assert_eq!(
        declarations[0]
            .parameters()
            .iter()
            .map(|row| row.kind())
            .collect::<Vec<_>>(),
        [CallableParameterContractKindV1::DeclaredHandle]
    );
    assert_eq!(
        declarations[1]
            .parameters()
            .iter()
            .map(|row| row.kind())
            .collect::<Vec<_>>(),
        [
            CallableParameterContractKindV1::Map,
            CallableParameterContractKindV1::OpaqueHandle,
        ]
    );
    for declaration in &declarations {
        assert!(declaration
            .parameters()
            .iter()
            .all(|row| row.home_demand() == crate::mir::resolved_semantics::HomeDemandV1::Handle));
    }
}

#[test]
fn ordinary_box_declaration_projects_declared_object() {
    let batch = batch(
        "box Wrap { v: i64 }\n\
         box Door { give(t: Wrap) { return t } }",
        14,
    );
    let catalog = issue_callable_parameter_contract_v1(&batch).unwrap();
    let declaration = catalog
        .declarations()
        .find(|row| row.parameters().len() == 1)
        .expect("Door.give declaration");
    let parameter = &declaration.parameters()[0];
    assert_eq!(
        parameter.kind(),
        CallableParameterContractKindV1::DeclaredObject("Wrap".into())
    );
    assert_eq!(
        parameter.home_demand(),
        crate::mir::resolved_semantics::HomeDemandV1::Handle
    );
}

#[test]
fn unresolved_declared_box_name_still_rejects() {
    let batch = batch("box Door { give(t: Wrap) { return t } }", 15);
    assert!(matches!(
        issue_callable_parameter_contract_v1(&batch),
        Err(
            super::CallableParameterContractIssueV1::UnsupportedDeclaredType { .. }
        )
    ));
}

#[test]
fn unadmitted_declared_box_name_still_rejects() {
    let batch = batch("static box Api { run(ctx: ContextBox) { return ctx } }", 13);
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
fn unsupported_explicit_type_rejects_without_opaque_fallback() {
    for spelling in ["f64", "Leaf?", "Shared<Leaf>", "Shared<Leaf?>", "Array<Leaf[]>" ] {
        let batch = batch(&format!("box Leaf {{}} static box Api {{ run(value: {spelling}) {{ return value }} }}"), 9);
        batch.with_declaration_semantics(|semantic| {
            let parameters = semantic.declarations().last().unwrap().parameters().unwrap();
            assert_eq!(parameters[0].type_reference(), Some(&crate::ast::TypeReferenceV1::parse(spelling).unwrap()));
        }).unwrap();
        assert!(matches!(issue_callable_parameter_contract_v1(&batch),
            Err(super::CallableParameterContractIssueV1::UnsupportedDeclaredType { .. })));
    }
    let batch = batch("box Leaf {} static box Api { run(): Leaf? { return null } }", 10);
    batch.with_declaration_semantics(|semantic| {
        assert!(matches!(semantic.declarations().last().unwrap().declared_result_type(), Some(crate::ast::TypeReferenceV1::Nullable(_))));
    }).unwrap();
    assert!(issue_callable_parameter_contract_v1(&batch).is_ok(), "owned nullable result declarations are admitted independently of parameters");
    for spelling in ["String?", "i64?", "Missing?", "Leaf[]?", "Shared<Leaf>?"] {
        let unsupported = self::batch(&format!("box Leaf {{}} static box Api {{ run(): {spelling} {{ return null }} }}"), 10);
        assert!(matches!(issue_callable_parameter_contract_v1(&unsupported),
            Err(super::CallableParameterContractIssueV1::UnsupportedResultType { .. })), "{spelling}");
    }

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

#[test]
fn explicit_edit_and_receiver_modes_survive_resolution_and_reject_execution() {
    use crate::parser::callable_parameter_source::ParserTransferKindV1 as Receiver;
    for (source, receiver, parameter_edit) in [
        ("box Api { run(edit value: i64) { return value } }", Receiver::Ordinary, true),
        ("box Api { edit run() { return 0 } }", Receiver::Edit, false),
        ("box Api { take run() { return 0 } }", Receiver::Take, false),
    ] {
        let batch = batch(source, 91);
        batch.with_declaration_semantics(|semantic| {
            let row = &semantic.declarations()[0];
            assert_eq!(row.receiver_transfer(), receiver);
            if parameter_edit {
                let transfer = row.parameters().unwrap()[0].transfer();
                assert!(!transfer.is_ordinary() && !transfer.is_take());
            }
        }).unwrap();
        let rejection = issue_callable_parameter_contract_v1(&batch).unwrap_err();
        assert!(matches!((parameter_edit, rejection),
            (true, super::CallableParameterContractIssueV1::UnsupportedTransfer { .. }) |
            (false, super::CallableParameterContractIssueV1::UnsupportedReceiverTransfer { .. })));
        assert!(NyashParser::parse_from_string(source).is_err(), "AST-only projection must not erase ownership modes");
    }
    let ordinary = batch("box Api { edit(edit: i64) { return edit } take(take: i64) { return take } run() { local edit = 1 return me.edit(edit) + me.take(edit) } }", 92);
    assert!(issue_callable_parameter_contract_v1(&ordinary).is_ok());
    assert!(NyashParser::parse_from_string("box Api { edit(edit: i64) { return edit } take(take: i64) { return take } }").is_ok());
}
