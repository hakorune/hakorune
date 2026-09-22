use super::*;
use crate::mir::resolved_semantics::{
    CoreMethodHomeResultRelationV1, NamedArrayRequirementIssueV1,
};
use crate::parser::NyashParser;

fn source(body: &str) -> String {
    format!("static box Main {{ run(s) {{ local arr = new ArrayBox() local i = 0 loop(i < 1) {{ {body} i = i + 1 }} return 0 }} }}")
}

fn rows(
    text: &str,
) -> Result<
    Box<
        [(
            crate::mir::resolved_semantics::SourceExprSiteV1,
            VerifiedSourceBoundCoreMethodCallV1,
        )],
    >,
    SourceBoundCoreMethodTargetIssueV1,
> {
    let batch = super::core_method_tests::batch(text, 800);
    let root = NyashParser::parse_from_string(text).unwrap();
    let brands = crate::analysis::brand_program_declaration_catalog::issue_brand_program_declaration_catalog_v1(&root).unwrap();
    batch
        .with_declaration_semantics(|view| {
            let row = view
                .declarations()
                .iter()
                .find(|row| row.function().only_loop_site().is_ok())
                .expect("loop callable");
            row.with_source_ledger(|ledger| {
                issue_source_bound_core_method_calls_with_named_arrays_v1(
                    &ledger,
                    batch.ordinary_box_coverage(),
                    &brands,
                )
            })
            .unwrap()
        })
        .unwrap()
}

#[test]
fn nested_text_producer_and_array_obligation_share_source_sites() {
    let rows = rows(&source("arr.push(s.substring(0, 1))")).unwrap();
    assert_eq!(rows.len(), 2);
    let push = rows
        .iter()
        .find(|(_, row)| row.contract().named_array_requirement().is_some())
        .unwrap()
        .1
        .contract();
    assert_eq!(
        push.target().result(),
        CoreMethodHomeResultRelationV1::NoValue
    );
    let obligation = push.named_array_requirement().unwrap();
    assert_eq!(obligation.call(), push.call_site());
    assert_eq!(obligation.owner(), push.owner());
    assert_ne!(obligation.construction(), push.call_site());
    let text = rows
        .iter()
        .find(|(site, _)| site == obligation.argument())
        .unwrap()
        .1
        .contract();
    assert_eq!(text.owner(), push.owner());
    assert_eq!(
        text.target().result(),
        CoreMethodHomeResultRelationV1::TextToCaller
    );
}

#[test]
fn text_literal_is_a_source_witness_without_a_synthetic_call() {
    let rows = rows(&source("arr.push(\"text\")")).unwrap();
    assert_eq!(rows.len(), 1);
    assert!(rows[0].1.contract().named_array_requirement().is_some());
}

#[test]
fn selected_array_contract_rejects_reassignment_value_demand_and_non_text() {
    for (body, expected) in [
        (
            "arr = new ArrayBox() arr.push(\"text\")",
            NamedArrayRequirementIssueV1::ReassignedReceiver,
        ),
        (
            "local result = arr.push(\"text\")",
            NamedArrayRequirementIssueV1::ValueDemand,
        ),
        (
            "arr.push(7)",
            NamedArrayRequirementIssueV1::TextSourceMissing,
        ),
    ] {
        assert_eq!(
            rows(&source(body)).unwrap_err(),
            SourceBoundCoreMethodTargetIssueV1::NamedArray(expected)
        );
    }
}

#[test]
fn constructor_arguments_reject_before_array_contract_issue() {
    let text = source("arr.push(\"text\")").replace("new ArrayBox()", "new ArrayBox(1)");
    assert_eq!(
        rows(&text).unwrap_err(),
        SourceBoundCoreMethodTargetIssueV1::NamedArray(
            NamedArrayRequirementIssueV1::ConstructorShape
        )
    );
}

#[test]
fn ordinary_shadow_and_other_named_class_do_not_acquire_array_authority() {
    let ordinary = format!(
        "box ArrayBox {{ birth() {{ return void }} }}\n{}",
        source("arr.push(\"text\")")
    );
    assert!(rows(&ordinary).unwrap().is_empty());
    let other = source("arr.push(\"text\")").replace("new ArrayBox()", "new OtherBox()");
    assert!(rows(&other).unwrap().is_empty());
}

#[test]
fn array_target_without_constructor_obligation_cannot_issue_source_contract() {
    use crate::mir::core_method_result_kind::{
        issue_core_method_manifest_row_ref_v2, CORE_METHOD_MANIFEST_BRAND_V2,
    };
    use crate::mir::resolved_semantics::*;
    let text = source("arr.push(\"text\")");
    let batch = super::core_method_tests::batch(&text, 801);
    batch
        .with_declaration_semantics(|view| {
            let row = &view.declarations()[0];
            row.with_source_ledger(|ledger| {
                let (_, call) = ledger.method_calls().next().unwrap();
                let loop_site = ledger.loop_sites().next().unwrap();
                let membership = ledger.resolved_loop_source(loop_site).unwrap();
                let target = CoreMethodInstanceTargetIssuerV1::array_text_append(
                    CORE_METHOD_MANIFEST_BRAND_V2,
                )
                .unwrap()
                .issue(
                    issue_core_method_manifest_row_ref_v2(
                        crate::mir::core_method_op::CoreMethodOp::ArrayPush,
                        1,
                    )
                    .unwrap(),
                )
                .unwrap();
                assert!(matches!(
                    ResolverCoreMethodCallableContractIssuerV1::issue(
                        &ledger,
                        call,
                        &membership,
                        ResolvedLoopPlacementV1::Body,
                        target,
                    ),
                    Err(ResolverCoreMethodCallableContractRejectV1::NamedArrayRequirementMismatch)
                ));
            })
            .unwrap();
        })
        .unwrap();
}

#[test]
fn foreign_text_owner_cannot_satisfy_nested_argument_obligation() {
    use crate::mir::resolved_semantics::*;
    let text = source("arr.push(s.substring(0, 1))");
    let foreign = rows(&text).unwrap();
    let foreign_text = foreign
        .iter()
        .find(|(_, row)| {
            row.contract().target().result() == CoreMethodHomeResultRelationV1::TextToCaller
        })
        .unwrap()
        .1
        .contract();
    let batch = super::core_method_tests::batch(&text, 802);
    let root = NyashParser::parse_from_string(&text).unwrap();
    let brands = crate::analysis::brand_program_declaration_catalog::issue_brand_program_declaration_catalog_v1(&root).unwrap();
    batch
        .with_declaration_semantics(|view| {
            view.declarations()[0]
                .with_source_ledger(|ledger| {
                    let (_, call) = ledger
                        .method_calls()
                        .find(|(_, call)| call.selector() == "push")
                        .unwrap();
                    assert!(matches!(
                        NamedArrayConstructionRequirementV1::issue(
                            &ledger,
                            call,
                            batch.ordinary_box_coverage(),
                            &brands,
                            Some(foreign_text),
                        ),
                        Err(NamedArrayRequirementIssueV1::ForeignTextSource)
                    ));
                })
                .unwrap();
        })
        .unwrap();
}
