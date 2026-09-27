//! Call-free coverage pins for `issue_with_source_relations`: the token
//! admits an empty call inventory only beside a bridge-issued
//! `VerifiedCallableLoopCallFreeCoverageV1`, and keeps the named reject
//! boundaries for foreign owners, foreign loop sites, and residual call
//! evidence.

use super::tests::{armed_loop_cond_parts, selected_relation, ARMED_LOOP_COND_SOURCE};
use super::{
    CallableLoopSourceRouteRejectV1, CallableLoopSourceRouteTokenV1,
    CallableLoopSourceTargetProbeV1, VerifiedCallableLoopCallFreeCoverageV1,
};
use crate::mir::resolved_semantics::{
    FunctionOwnerIdV1, SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1, SourceStmtSiteV1,
};

/// A bridge-shaped coverage proof for the armed fixture's exact loop site.
/// The token validates owner and loop-site identity; complete grammar
/// coverage is the bridge proof's contract, not the token's.
fn call_free_coverage(
    owner: FunctionOwnerIdV1,
    loop_site: SourceStmtSiteV1,
) -> VerifiedCallableLoopCallFreeCoverageV1 {
    let mut condition_segments = loop_site.node().segments().to_vec();
    condition_segments.push(SourcePathSegmentV1::LoopCondition);
    let mut statement_segments = loop_site.node().segments().to_vec();
    statement_segments.push(SourcePathSegmentV1::LoopBody(0));
    VerifiedCallableLoopCallFreeCoverageV1::issue(
        owner,
        loop_site,
        SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(condition_segments)),
        Box::new([SourceStmtSiteV1::from_node(
            SourceNodeSiteV1::from_segments(statement_segments),
        )]),
        Box::new([]),
    )
}

#[test]
fn issue_with_source_relations_admits_verified_call_free_coverage() {
    let parts = armed_loop_cond_parts(ARMED_LOOP_COND_SOURCE);
    let coverage = call_free_coverage(
        parts.owner,
        SourceStmtSiteV1::from_node(parts.parent_site.clone()),
    );
    let token = CallableLoopSourceRouteTokenV1::issue_with_source_relations(
        parts.owner,
        parts.parent_site,
        parts.function_origin,
        parts.source_kind,
        parts.outcome,
        parts.selection,
        Some(parts.projection),
        Box::new([]),
        CallableLoopSourceTargetProbeV1::empty(),
        Some(coverage),
    )
    .expect("verified call-free coverage admits the empty call inventory");
    assert!(token.source_target().is_none());
    assert!(
        token
            .source_coverage()
            .and_then(|coverage| coverage.as_call_free())
            .is_some()
    );
    let (_, _, _, _, _, items, coverage) = token
        .into_physical_parts()
        .expect("call-free coverage reaches the physical boundary");
    assert!(items.is_empty());
    assert!(coverage.as_call_free().is_some());
}

#[test]
fn issue_with_source_relations_keeps_unproven_empty_inventory_missing() {
    let parts = armed_loop_cond_parts(ARMED_LOOP_COND_SOURCE);
    let reject = CallableLoopSourceRouteTokenV1::issue_with_source_relations(
        parts.owner,
        parts.parent_site,
        parts.function_origin,
        parts.source_kind,
        parts.outcome,
        parts.selection,
        Some(parts.projection),
        Box::new([]),
        CallableLoopSourceTargetProbeV1::empty(),
        None,
    )
    .expect_err("an empty inventory without the coverage proof must not mint a route");
    assert_eq!(reject, CallableLoopSourceRouteRejectV1::SourceItemsMissing);
}

#[test]
fn issue_with_source_relations_rejects_a_foreign_call_free_owner() {
    let parts = armed_loop_cond_parts(ARMED_LOOP_COND_SOURCE);
    let foreign_owner = crate::mir::resolved_semantics::FunctionOwnerIssuerV1::new_for_compilation()
        .and_then(|mut issuer| issuer.issue())
        .expect("foreign owner");
    let coverage = call_free_coverage(
        foreign_owner,
        SourceStmtSiteV1::from_node(parts.parent_site.clone()),
    );
    let reject = CallableLoopSourceRouteTokenV1::issue_with_source_relations(
        parts.owner,
        parts.parent_site,
        parts.function_origin,
        parts.source_kind,
        parts.outcome,
        parts.selection,
        Some(parts.projection),
        Box::new([]),
        CallableLoopSourceTargetProbeV1::empty(),
        Some(coverage),
    )
    .expect_err("a coverage proof from another owner must not satisfy this route");
    assert_eq!(reject, CallableLoopSourceRouteRejectV1::SourceCoverageForeign);
}

#[test]
fn issue_with_source_relations_rejects_a_foreign_call_free_site() {
    let parts = armed_loop_cond_parts(ARMED_LOOP_COND_SOURCE);
    let coverage = call_free_coverage(
        parts.owner,
        SourceStmtSiteV1::from_node(
            crate::mir::resolved_semantics::SourcePathV1::root_body(9).node(),
        ),
    );
    let reject = CallableLoopSourceRouteTokenV1::issue_with_source_relations(
        parts.owner,
        parts.parent_site,
        parts.function_origin,
        parts.source_kind,
        parts.outcome,
        parts.selection,
        Some(parts.projection),
        Box::new([]),
        CallableLoopSourceTargetProbeV1::empty(),
        Some(coverage),
    )
    .expect_err("a coverage proof for another loop site must not satisfy this route");
    assert_eq!(
        reject,
        CallableLoopSourceRouteRejectV1::SourceCoverageSiteMismatch
    );
}

#[test]
fn issue_with_source_relations_rejects_call_free_with_residual_evidence() {
    let parts = armed_loop_cond_parts(ARMED_LOOP_COND_SOURCE);
    let coverage = call_free_coverage(
        parts.owner,
        SourceStmtSiteV1::from_node(parts.parent_site.clone()),
    );
    let reject = CallableLoopSourceRouteTokenV1::issue_with_source_relations(
        parts.owner,
        parts.parent_site,
        parts.function_origin,
        parts.source_kind,
        parts.outcome,
        parts.selection,
        Some(parts.projection),
        Box::new([]),
        CallableLoopSourceTargetProbeV1::from_parts(
            Box::new([]),
            vec![parts.call_sites[0].clone()].into_boxed_slice(),
            false,
        ),
        Some(coverage),
    )
    .expect_err("a call-free claim beside uncovered call rows must not co-seal");
    assert_eq!(
        reject,
        CallableLoopSourceRouteRejectV1::SourceCallResidualEvidence
    );
}

#[test]
fn issue_with_source_relations_rejects_call_free_beside_call_items() {
    let parts = armed_loop_cond_parts(ARMED_LOOP_COND_SOURCE);
    let coverage = call_free_coverage(
        parts.owner,
        SourceStmtSiteV1::from_node(parts.parent_site.clone()),
    );
    let call_site = parts.call_sites[0].clone();
    let reject = CallableLoopSourceRouteTokenV1::issue_with_source_relations(
        parts.owner,
        parts.parent_site,
        parts.function_origin,
        parts.source_kind,
        parts.outcome,
        parts.selection,
        Some(parts.projection),
        parts.items,
        CallableLoopSourceTargetProbeV1::from_parts(
            vec![selected_relation(&call_site)].into_boxed_slice(),
            Box::new([]),
            false,
        ),
        Some(coverage),
    )
    .expect_err("a call-free proof beside a call inventory must not co-seal");
    assert_eq!(
        reject,
        CallableLoopSourceRouteRejectV1::SourceCallResidualEvidence
    );
}
