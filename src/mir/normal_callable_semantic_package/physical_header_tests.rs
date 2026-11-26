use crate::mir::builder::{
    CanonicalSameModuleCallableKeyV1, CompilationContext, SelectedNormalCallableKeyV1,
};
use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
use super::tests::final_source;

use super::{issue_normal_callable_semantic_package_v1, SelectedCallableSemanticRefV1};

fn issue(
    source: &str,
) -> Result<
    super::VerifiedNormalCallableSemanticPackageV1,
    super::NormalCallableSemanticPackageIssueV1,
> {
    let mut resolver = FunctionSemanticResolverSessionV1::new(194).unwrap();
    issue_normal_callable_semantic_package_v1(&mut resolver, final_source(source))
}

#[test]
fn explicit_result_annotation_lends_one_complete_header_view() {
    let package = issue("static box Api { run(value: i64): i64 { return value } }")
        .expect("explicit source/header cohort");
    let key = SelectedNormalCallableKeyV1::Cataloged(
        CanonicalSameModuleCallableKeyV1::test_static_box_method("Api", "run", 1),
    );
    let slot = package.selected.batch_slot(&key).unwrap();
    let header = package.physical_header.row(slot, &package.result_contracts).unwrap();
    let header_owner = header.owner();
    assert_eq!(header.result(), crate::mir::exact_trivial_scalar_abi::ExactTrivialScalarAbiV1::I64);
    assert_eq!(header.completion().owner(), header_owner);
    let mut context = CompilationContext::new();
    let installed = package
        .prepare_install_with_consumer(&mut context, None)
        .expect("vacant catalog slot")
        .commit();
    let mut port = installed.begin_lowering(&context).expect("same catalog");
    port.with_selected_lowering_input(&key, |input| {
        let result_contract = input
            .result_contract()
            .expect("generic result contract row");
        assert!(matches!(
            result_contract.declared_result(),
            crate::mir::resolved_control_flow::DeclaredFunctionResultContractV1::Annotated(name)
                if name.as_ref() == "i64"
        ));
        assert_eq!(header_owner, input.source().owner());
        let completion = result_contract.completion();
        assert_eq!(completion.owner(), header_owner);
        assert!(completion.returns_value());
        assert_eq!(completion.explicit_sites().len(), 1);
        assert!(completion.cleanup().crossed_scopes().is_empty());
        assert!(matches!(
            input.semantic(),
            SelectedCallableSemanticRefV1::Ordinary
        ));
    })
    .expect("header loan");
}

#[test]
fn absent_result_annotation_keeps_ordinary_package_without_physical_header() {
    let package = issue("static box Api { run(value: i64) { return value } }")
        .expect("ordinary package remains source-compatible");
    let key = SelectedNormalCallableKeyV1::Cataloged(
        CanonicalSameModuleCallableKeyV1::test_static_box_method("Api", "run", 1),
    );
    let slot = package.selected.batch_slot(&key).unwrap();
    assert!(package.physical_header.row(slot, &package.result_contracts).is_none());
    let mut context = CompilationContext::new();
    let installed = package
        .prepare_install_with_consumer(&mut context, None)
        .expect("vacant catalog slot")
        .commit();
    let mut port = installed.begin_lowering(&context).expect("same catalog");
    port.with_selected_lowering_input(&key, |input| {
        let result_contract = input
            .result_contract()
            .expect("unannotated result contract is retained");
        assert!(matches!(
            result_contract.declared_result(),
            crate::mir::resolved_control_flow::DeclaredFunctionResultContractV1::Unannotated
        ));
        assert!(result_contract.result().is_none());
    })
    .expect("ordinary loan");
}

#[test]
fn valid_void_siblings_do_not_poison_sparse_physical_header_cohort() {
    let package = issue(
        "static box Api {\
             observe(a: i64) { return }\
             explicit(a: i64): void { return }\
             run(value: i64): i64 { return value }\
         }",
    )
    .expect("ordinary Void siblings remain source-valid");
    let selected_keys = package.selected.keys().cloned().collect::<Vec<_>>();
    assert_eq!(selected_keys.len(), 3);
    for key in &selected_keys {
        let SelectedNormalCallableKeyV1::Cataloged(source_key) = key else {
            panic!("Void/header fixture rows must stay cataloged");
        };
        let slot = package.selected.batch_slot(key).unwrap();
        assert_eq!(
            package.physical_header.row(slot, &package.result_contracts).is_some(),
            source_key.name() == "run"
        );
    }
    let mut context = CompilationContext::new();
    let installed = package
        .prepare_install_with_consumer(&mut context, None)
        .expect("vacant catalog slot")
        .commit();
    let mut port = installed.begin_lowering(&context).expect("same catalog");
    for key in selected_keys {
        let SelectedNormalCallableKeyV1::Cataloged(source_key) = &key else {
            panic!("Void/header fixture rows must stay cataloged")
        };
        let expected_header = source_key.name() == "run";
        port.with_selected_lowering_input(&key, |input| {
            let result_contract = input
                .result_contract()
                .expect("result contract row remains for every generic callable");
            assert_eq!(
                result_contract.result().is_some(),
                expected_header,
                "{}",
                source_key.name()
            );
            if expected_header {
                assert!(matches!(
                    result_contract.declared_result(),
                    crate::mir::resolved_control_flow::DeclaredFunctionResultContractV1::Annotated(
                        name
                    ) if name.as_ref() == "i64"
                ));
            } else if source_key.name() == "explicit" {
                assert!(matches!(
                    result_contract.declared_result(),
                    crate::mir::resolved_control_flow::DeclaredFunctionResultContractV1::Void
                ));
            } else {
                assert!(matches!(
                    result_contract.declared_result(),
                    crate::mir::resolved_control_flow::DeclaredFunctionResultContractV1::Unannotated
                ));
            }
        })
        .expect("selected row loan");
    }
}

#[test]
fn mixed_package_lends_only_the_eligible_physical_header_row() {
    let package = issue(include_str!(
        "../../../lang/src/compiler/parser/scan/parser_scan_loop_box.hako"
    ))
    .expect("mixed parser scan package");
    let selected_keys = package.selected.keys().cloned().collect::<Vec<_>>();
    assert_eq!(selected_keys.len(), 4);
    let mut context = CompilationContext::new();
    let installed = package
        .prepare_install_with_consumer(&mut context, None)
        .expect("vacant catalog slot")
        .commit();
    let mut port = installed.begin_lowering(&context).expect("same catalog");
    for key in selected_keys {
        let SelectedNormalCallableKeyV1::Cataloged(source_key) = &key else {
            panic!("parser scan rows must stay cataloged")
        };
        let method = source_key.name();
        let expected_header = method == "skip_while";
        port.with_selected_lowering_input(&key, |input| {
            let contract = input.result_contract().unwrap();
            assert_eq!(contract.result().is_some(), expected_header, "{method}");
            if let Some(result) = contract.result() {
                assert_eq!(
                    result,
                    crate::mir::exact_trivial_scalar_abi::ExactTrivialScalarAbiV1::I64
                );
                assert!(contract.completion().returns_value());
                assert_eq!(contract.completion().explicit_sites().len(), 2);
            }
        })
        .expect("selected row loan");
    }
}

#[test]
fn non_i64_result_annotation_is_not_reclassified_as_a_physical_header() {
    let issue = issue("static box Api { run(value: i64): i32 { return value } }");
    assert!(matches!(
        issue,
        Err(super::NormalCallableSemanticPackageIssueV1::PhysicalHeader { _error: _ })
    ));
}
