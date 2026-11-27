use crate::mir::normal_callable_semantic_package::seal_selected_core_method_rows;
use crate::mir::callable_semantic_batch::{
    issue_resolved_callable_semantic_batch_v1,
    VerifiedResolvedCallableSemanticBatchV1,
};
use crate::mir::core_method_op::CoreMethodOp;
use crate::mir::resolved_semantics::{
    FunctionSemanticResolverSessionV1,
};
use crate::parser::{NyashParser, ParserBuildConfig};

use super::{
    issue_source_bound_core_method_calls_v1,
    SourceBoundCoreMethodTargetIssueV1,
};

const FIXTURE: &str = include_str!("../../../apps/tests/scan_with_init_typed_ok_min.hako");

pub(super) fn batch(source: &str, ordinal: u32) -> VerifiedResolvedCallableSemanticBatchV1 {
    let parsed = NyashParser::parse_normal_callable_program_with_build_config(
        source,
        ParserBuildConfig::default(),
    )
    .expect("normal callable source");
    let source = crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let transformed = crate::r#macro::transform_normal_callable_program_v1(parsed)
            .expect("exact callable transform");
        let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = transformed
        else {
            panic!("fixture must remain source-backed")
        };
        source
    });
    let mut resolver = FunctionSemanticResolverSessionV1::new(ordinal).unwrap();
    issue_resolved_callable_semantic_batch_v1(&mut resolver, source).unwrap()
}

#[test]
fn source_bound_core_method_issuer_keeps_exact_length_and_substring_rows() {
    let source = FIXTURE
        .replacen("local i: i64 = 0", "local text = \"abc\" local i: i64 = 0", 1)
        .replacen("s.substring(i, i + 1)", "text.substring(i, i + 1)", 1);
    let batch = batch(&source, 106);
    let rows = batch
        .with_declaration_semantics(|view| {
            let row = &view.declarations()[0];
            row.with_source_ledger(|ledger| issue_source_bound_core_method_calls_v1(&ledger))
                .unwrap()
        })
        .unwrap()
        .unwrap();

    assert_eq!(rows.len(), 2);
    assert_eq!(
        rows[0].1.contract().target().row().row().op,
        CoreMethodOp::StringLen
    );
    assert_eq!(
        rows[0].1.contract().placement(),
        crate::mir::resolved_semantics::ResolvedLoopPlacementV1::Condition
    );
    assert_eq!(
        rows[1].1.contract().target().row().row().op,
        CoreMethodOp::StringSubstring
    );
    assert_eq!(
        rows[1].1.contract().placement(),
        crate::mir::resolved_semantics::ResolvedLoopPlacementV1::Body
    );
    for (_, row) in rows.iter() {
        assert_eq!(row.contract().call_site(), row.contract().result_site());
        assert_eq!(
            row.contract().receiver_site().node().segments().last(),
            Some(&crate::mir::resolved_semantics::SourcePathSegmentV1::Receiver)
        );
    }
    assert_eq!(rows[0].1.contract().arguments().len(), 0);
    assert_eq!(rows[1].1.contract().arguments().len(), 2);
}

#[test]
fn source_bound_core_method_catalog_rejects_foreign_caller() {
    let batch = batch(FIXTURE, 107);
    let rows = batch
        .with_declaration_semantics(|view| {
            let row = &view.declarations()[0];
            row.with_source_ledger(|ledger| issue_source_bound_core_method_calls_v1(&ledger))
                .unwrap()
        })
        .unwrap()
        .unwrap();
    let declarations =
        crate::mir::builder::VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(
            &NyashParser::parse_from_string(FIXTURE).unwrap(),
        )
        .unwrap();
    let foreign = crate::mir::builder::CanonicalSameModuleCallableKeyV1::test_static_box_method(
        "Foreign", "run", 0,
    );
    let owner = rows[0].1.contract().owner();
    let rejected = seal_selected_core_method_rows(&declarations, &foreign, owner, rows.into_vec())
        .unwrap_err();
    assert_eq!(
        rejected,
        format!("{:?}", SourceBoundCoreMethodTargetIssueV1::ForeignCaller(foreign))
    );
}

#[test]
fn source_bound_core_method_catalog_rejects_duplicate_exact_site() {
    let batch = batch(FIXTURE, 108);
    let (first, second) = batch
        .with_declaration_semantics(|view| {
            let row = &view.declarations()[0];
            row.with_source_ledger(|ledger| {
                (
                    issue_source_bound_core_method_calls_v1(&ledger).unwrap(),
                    issue_source_bound_core_method_calls_v1(&ledger).unwrap(),
                )
            })
            .unwrap()
        })
        .unwrap();
    let declarations =
        crate::mir::builder::VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(
            &NyashParser::parse_from_string(FIXTURE).unwrap(),
        )
        .unwrap();
    let caller = declarations
        .declaration_for(
            crate::mir::builder::SameModuleCallableNamespaceV1::StaticBoxMethod,
            "Main",
            "find_ok",
            2,
        )
        .unwrap()
        .key()
        .clone();
    let owner = first[0].1.contract().owner();
    let duplicate_site = second[0].0.clone();
    let rejected = seal_selected_core_method_rows(
        &declarations,
        &caller,
        owner,
        first.into_vec().into_iter().chain(second.into_vec()),
    )
    .unwrap_err();
    assert_eq!(
        rejected,
        format!("{:?}", SourceBoundCoreMethodTargetIssueV1::CatalogCollision(duplicate_site))
    );
}
