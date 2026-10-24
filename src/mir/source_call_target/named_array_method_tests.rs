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
                    row.body_shape(),
                    batch.ordinary_box_coverage(),
                    &brands,
                    None,
                    "Main",
                    &[],
                )
            })
            .unwrap()
        })
        .unwrap()
        .map(|issued| issued.rows)
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
    assert_ne!(obligation.construction(), Some(push.call_site()));
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
fn condition_position_push_stays_unarmed() {
    // `ArrayPush/1` remains `Body`-only: a condition-position `push` is
    // skipped before any named-array requirement is consulted.
    let text = "static box Main { run(s) { local arr = new ArrayBox() local i = 0 loop(arr.push(\"text\") == 0) { i = i + 1 } return 0 } }";
    assert!(rows(text).unwrap().is_empty());
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

#[test]
fn append_only_object_source_retains_all_stores_without_class_or_home_permission() {
    use crate::mir::resolved_semantics::{
        NamedArrayAppendOnlySourceIssueV1 as E, NamedArrayAppendOnlySourceV1,
        NamedArrayRequirementIssueV1 as C,
    };
    let app = include_str!("../../../apps/mimalloc-lite/main.hako");
    let app_batch = super::core_method_tests::batch(app, 804);
    let app_root = NyashParser::parse_from_string(app).unwrap();
    let app_brands = crate::analysis::brand_program_declaration_catalog::issue_brand_program_declaration_catalog_v1(&app_root).unwrap();
    app_batch
        .with_declaration_semantics(|view| {
            let row = view
                .declarations()
                .iter()
                .find(|row| {
                    row.function()
                        .method_calls()
                        .any(|(_, call)| call.selector() == "get")
                })
                .unwrap();
            row.with_source_ledger(|ledger| {
                let (_, read) = ledger
                    .method_calls()
                    .find(|(_, call)| call.selector() == "get")
                    .unwrap();
                let source = NamedArrayAppendOnlySourceV1::issue(
                    &ledger,
                    read,
                    app_batch.ordinary_box_coverage(),
                    &app_brands,
                )
                .unwrap();
                assert_eq!(source.appends().len(), 13, "all original stores retained");
                assert_eq!(source.reads().len(), 4);
                assert_eq!(
                    source
                        .appends()
                        .iter()
                        .filter(|row| row.call() < read.site())
                        .count(),
                    9
                );
            })
            .unwrap()
        })
        .unwrap();
    let original = "static box Main { run(factory) { local arr = new ArrayBox() loop(false) { arr.push(factory.make(1)) } factory.use(arr.get(0)) arr.push(factory.make(2)) return 0 } }";
    let cases = [
        (original.to_string(), None),
        (
            original.replace("arr.push(factory.make(2))", "arr.set(0, factory.make(2))"),
            Some(E::Argument),
        ),
        (
            original.replace("return 0", "local alias = arr return 0"),
            Some(E::HolderEscape),
        ),
        (
            original.replace("return 0", "arr = new ArrayBox() return 0"),
            Some(E::Construction(C::ReassignedReceiver)),
        ),
        (
            original.replace("arr.get(0)", "arr.get(true)"),
            Some(E::IndexSource),
        ),
        (
            original.replace(
                "arr.push(factory.make(2))",
                "local result = arr.push(factory.make(2))",
            ),
            Some(E::SourceOrder),
        ),
        (
            original.replace("arr.push(factory.make(2))", "arr.push(factory)"),
            Some(E::CandidateSource),
        ),
        (
            original.replace("arr.push(factory.make(2))", "arr.push(arr.get(0))"),
            Some(E::CandidateSource),
        ),
        (
            original.replace(
                "arr.push(factory.make(2))",
                "if true { arr.push(factory.make(2)) }",
            ),
            Some(E::SourceOrder),
        ),
        (
            original.replace("new ArrayBox()", "new ArrayBox(1)"),
            Some(E::Construction(C::ConstructorShape)),
        ),
    ];
    for (text, expected) in cases {
        let batch = super::core_method_tests::batch(&text, 803);
        let root = NyashParser::parse_from_string(&text).unwrap();
        let brands = crate::analysis::brand_program_declaration_catalog::issue_brand_program_declaration_catalog_v1(&root).unwrap();
        batch
            .with_declaration_semantics(|view| {
                view.declarations()[0]
                    .with_source_ledger(|ledger| {
                        let (_, read) = ledger
                            .method_calls()
                            .find(|(_, call)| call.selector() == "get")
                            .unwrap();
                        let result = NamedArrayAppendOnlySourceV1::issue(
                            &ledger,
                            read,
                            batch.ordinary_box_coverage(),
                            &brands,
                        );
                        if let Some(expected) = expected {
                            assert_eq!(result.err(), Some(expected), "{text}");
                        } else {
                            let source = result.unwrap();
                            assert_eq!(source.construction().owner(), ledger.owner());
                            assert_eq!(
                                source.appends().len(),
                                2,
                                "future store is retained, never current-read support"
                            );
                            assert_eq!(source.reads().len(), 1);
                            assert_eq!(source.reads()[0].call(), read.site());
                            assert_eq!(source.reads()[0].index(), read.arguments()[0].site());
                            assert_eq!(source.reads()[0].integer(), 0);
                            // A zero-trip loop retains its append source but
                            // proves neither a stored element nor read liveness.
                            assert!(ledger.loop_sites().any(|site| ledger.resolved_loop_placement(
                                site, source.appends()[0].call()).is_ok_and(|placement|
                                    placement == Some(crate::mir::resolved_semantics::ResolvedLoopPlacementV1::Body))));
                            assert!(source
                                .appends()
                                .iter()
                                .all(|row| ledger.method_call(row.candidate()).is_some()));
                            assert!(source.appends()[0].call() < source.reads()[0].call());
                            assert!(source.reads()[0].call() < source.appends()[1].call());
                        }
                    })
                    .unwrap()
            })
            .unwrap();
    }
}
