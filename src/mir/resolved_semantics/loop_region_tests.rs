use std::collections::BTreeMap;

use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};

use super::ids::{FunctionOwnerIdV1, FunctionOwnerIssuerV1, RegionId, ScopeId};
use super::loop_region::{
    build_verified_loop_region_index_v1, ResolvedLoopRegionLookupErrorV1,
    ResolvedLoopRegionVerificationErrorV1,
};
use super::product::{ResolvedFunctionDataV1, ResolvedFunctionDraftV1};
use super::records::{
    RegionKindV1, RegionOriginV1, ResolvedRegionRecordV1, ResolvedScopeRecordV1, ScopeKindV1,
    ScopeOriginV1,
};
use super::resolver::FunctionSemanticResolverSessionV1;
use super::source_site::{
    FunctionOriginV1, SourceNodeSiteV1, SourcePathSegmentV1, SourceStmtSiteV1,
};
use super::{
    FunctionSyntaxViewV1, ResolvedFunctionVerificationErrorV1, VerifiedResolvedFunctionV1,
};

fn owner() -> FunctionOwnerIdV1 {
    FunctionOwnerIssuerV1::new_for_compilation()
        .unwrap()
        .issue()
        .unwrap()
}

fn node(segments: Vec<SourcePathSegmentV1>) -> SourceNodeSiteV1 {
    SourceNodeSiteV1::from_segments(segments)
}

fn stmt(segments: Vec<SourcePathSegmentV1>) -> SourceStmtSiteV1 {
    SourceStmtSiteV1::from_node(node(segments))
}

fn function(body: Vec<ASTNode>) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: "fixture".into(),
        params: Vec::new(),
        param_decls: Vec::new(),
        return_type_name: None,
        body,
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn int(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn loop_stmt(body: Vec<ASTNode>) -> ASTNode {
    ASTNode::Loop {
        condition: Box::new(int(1)),
        body,
        span: Span::unknown(),
    }
}

fn resolve(tree: &ASTNode) -> std::sync::Arc<VerifiedResolvedFunctionV1> {
    FunctionSemanticResolverSessionV1::new(0)
        .unwrap()
        .resolve(FunctionSyntaxViewV1::from_ast(tree).unwrap())
        .unwrap()
}

fn seal(
    data: ResolvedFunctionDataV1,
) -> Result<VerifiedResolvedFunctionV1, ResolvedFunctionVerificationErrorV1> {
    ResolvedFunctionDraftV1 { data }.seal()
}

fn loop_origin(index: u32) -> SourceNodeSiteV1 {
    node(vec![SourcePathSegmentV1::Body(index)])
}

fn loop_body_origin(index: u32) -> SourceNodeSiteV1 {
    node(vec![
        SourcePathSegmentV1::Body(index),
        SourcePathSegmentV1::LoopBodyRoot,
    ])
}

fn loop_data(owner: FunctionOwnerIdV1) -> ResolvedFunctionDataV1 {
    let function_origin = FunctionOriginV1::new(0, 0);
    let function_scope = ScopeId::new(owner, 0);
    let body_scope = ScopeId::new(owner, 1);
    let loop_scope = ScopeId::new(owner, 2);
    let function_region = RegionId::new(owner, 0);
    let body_region = RegionId::new(owner, 1);
    let loop_region = RegionId::new(owner, 2);
    let body_origin = node(vec![SourcePathSegmentV1::FunctionBody]);

    ResolvedFunctionDataV1 {
        owner,
        function_origin,
        function_scope,
        function_region,
        bindings: BTreeMap::new(),
        scopes: BTreeMap::from([
            (
                function_scope,
                ResolvedScopeRecordV1::new(
                    ScopeKindV1::Function,
                    None,
                    function_region,
                    Vec::new(),
                    ScopeOriginV1::Function(function_origin),
                ),
            ),
            (
                body_scope,
                ResolvedScopeRecordV1::new(
                    ScopeKindV1::LexicalBlock,
                    Some(function_scope),
                    body_region,
                    Vec::new(),
                    ScopeOriginV1::Source(body_origin.clone()),
                ),
            ),
            (
                loop_scope,
                ResolvedScopeRecordV1::new(
                    ScopeKindV1::LoopBody,
                    Some(body_scope),
                    loop_region,
                    Vec::new(),
                    ScopeOriginV1::Source(loop_body_origin(0)),
                ),
            ),
        ]),
        regions: BTreeMap::from([
            (
                function_region,
                ResolvedRegionRecordV1::new(
                    RegionKindV1::Function,
                    None,
                    Some(function_scope),
                    RegionOriginV1::Function(function_origin),
                ),
            ),
            (
                body_region,
                ResolvedRegionRecordV1::new(
                    RegionKindV1::Sequence,
                    Some(function_region),
                    Some(body_scope),
                    RegionOriginV1::Source(body_origin),
                ),
            ),
            (
                loop_region,
                ResolvedRegionRecordV1::new(
                    RegionKindV1::Loop,
                    Some(body_region),
                    Some(loop_scope),
                    RegionOriginV1::Source(loop_origin(0)),
                ),
            ),
        ]),
        declarations: BTreeMap::new(),
        variable_uses: BTreeMap::new(),
        assignment_targets: BTreeMap::new(),
        direct_call_targets: BTreeMap::new(),
        resolved_exits: BTreeMap::new(),
    }
}

#[test]
fn resolver_indexes_same_span_sibling_loop_sites() {
    let product = resolve(&function(vec![
        loop_stmt(Vec::new()),
        loop_stmt(Vec::new()),
    ]));
    let first = *product
        .loop_region_bundle(&stmt(vec![SourcePathSegmentV1::Body(0)]))
        .unwrap();
    let second = *product
        .loop_region_bundle(&stmt(vec![SourcePathSegmentV1::Body(1)]))
        .unwrap();

    assert_eq!(product.loop_region_bundle_count(), 2);
    assert_ne!(first.loop_pair(), second.loop_pair());
}

#[test]
fn nested_loop_bundle_uses_the_outer_loop_as_its_exact_parent() {
    let product = resolve(&function(vec![loop_stmt(vec![loop_stmt(Vec::new())])]));
    let outer = *product
        .loop_region_bundle(&stmt(vec![SourcePathSegmentV1::Body(0)]))
        .unwrap();
    let inner = *product
        .loop_region_bundle(&stmt(vec![
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::LoopBody(0),
        ]))
        .unwrap();

    assert_eq!(product.loop_region_bundle_count(), 2);
    assert_eq!(
        product.region(inner.loop_pair().region()).unwrap().parent(),
        Some(outer.loop_pair().region())
    );
    assert_eq!(
        product.scope(inner.loop_pair().scope()).unwrap().parent(),
        Some(outer.loop_pair().scope())
    );
}

#[test]
fn query_reports_a_typed_missing_exact_bundle() {
    let product = resolve(&function(vec![loop_stmt(Vec::new())]));
    let missing = stmt(vec![SourcePathSegmentV1::Body(9)]);
    assert_eq!(
        product.loop_region_bundle(&missing),
        Err(ResolvedLoopRegionLookupErrorV1::MissingExactBundle(missing))
    );
}

#[test]
fn sealed_bundle_ids_point_to_exact_authoritative_records() {
    let product = seal(loop_data(owner())).unwrap();
    let pair = product
        .loop_region_bundle(&stmt(vec![SourcePathSegmentV1::Body(0)]))
        .unwrap()
        .loop_pair();
    let region = product.region(pair.region()).unwrap();
    let scope = product.scope(pair.scope()).unwrap();

    assert_eq!(region.kind(), RegionKindV1::Loop);
    assert_eq!(region.parent(), Some(RegionId::new(product.owner(), 1)));
    assert_eq!(region.lexical_scope(), Some(pair.scope()));
    assert_eq!(region.origin(), &RegionOriginV1::Source(loop_origin(0)));
    assert_eq!(scope.kind(), ScopeKindV1::LoopBody);
    assert_eq!(scope.parent(), Some(ScopeId::new(product.owner(), 1)));
    assert_eq!(scope.owner_region(), pair.region());
    assert_eq!(scope.origin(), &ScopeOriginV1::Source(loop_body_origin(0)));
}

#[test]
fn index_rejects_missing_loop_body_scope_and_broken_reciprocal_link() {
    let first_owner = owner();
    let mut missing = loop_data(first_owner);
    missing.scopes.remove(&ScopeId::new(first_owner, 2));
    assert_eq!(
        build_verified_loop_region_index_v1(&missing),
        Err(ResolvedLoopRegionVerificationErrorV1::MissingLoopBodyScope(
            RegionId::new(first_owner, 2)
        ))
    );

    let second_owner = owner();
    let mut broken = loop_data(second_owner);
    broken.regions.insert(
        RegionId::new(second_owner, 2),
        ResolvedRegionRecordV1::new(
            RegionKindV1::Loop,
            Some(RegionId::new(second_owner, 1)),
            Some(ScopeId::new(second_owner, 1)),
            RegionOriginV1::Source(loop_origin(0)),
        ),
    );
    assert!(matches!(
        build_verified_loop_region_index_v1(&broken),
        Err(ResolvedLoopRegionVerificationErrorV1::LoopBodyContractMismatch(_))
    ));
}

#[test]
fn index_rejects_wrong_region_and_scope_parents() {
    let first_owner = owner();
    let mut wrong_region_parent = loop_data(first_owner);
    wrong_region_parent.regions.insert(
        RegionId::new(first_owner, 2),
        ResolvedRegionRecordV1::new(
            RegionKindV1::Loop,
            Some(RegionId::new(first_owner, 0)),
            Some(ScopeId::new(first_owner, 2)),
            RegionOriginV1::Source(loop_origin(0)),
        ),
    );
    assert_eq!(
        build_verified_loop_region_index_v1(&wrong_region_parent),
        Err(ResolvedLoopRegionVerificationErrorV1::LoopContractMismatch(
            RegionId::new(first_owner, 2)
        ))
    );

    let second_owner = owner();
    let mut wrong_scope_parent = loop_data(second_owner);
    wrong_scope_parent.scopes.insert(
        ScopeId::new(second_owner, 2),
        ResolvedScopeRecordV1::new(
            ScopeKindV1::LoopBody,
            Some(ScopeId::new(second_owner, 0)),
            RegionId::new(second_owner, 2),
            Vec::new(),
            ScopeOriginV1::Source(loop_body_origin(0)),
        ),
    );
    assert!(matches!(
        build_verified_loop_region_index_v1(&wrong_scope_parent),
        Err(ResolvedLoopRegionVerificationErrorV1::LoopBodyContractMismatch(_))
    ));
}

#[test]
fn index_rejects_wrong_loop_and_body_origins() {
    let first_owner = owner();
    let mut wrong_loop_origin = loop_data(first_owner);
    wrong_loop_origin.regions.insert(
        RegionId::new(first_owner, 2),
        ResolvedRegionRecordV1::new(
            RegionKindV1::Loop,
            Some(RegionId::new(first_owner, 1)),
            Some(ScopeId::new(first_owner, 2)),
            RegionOriginV1::Source(loop_origin(1)),
        ),
    );
    assert!(matches!(
        build_verified_loop_region_index_v1(&wrong_loop_origin),
        Err(ResolvedLoopRegionVerificationErrorV1::LoopBodyContractMismatch(_))
    ));

    let second_owner = owner();
    let mut wrong_body_origin = loop_data(second_owner);
    wrong_body_origin.scopes.insert(
        ScopeId::new(second_owner, 2),
        ResolvedScopeRecordV1::new(
            ScopeKindV1::LoopBody,
            Some(ScopeId::new(second_owner, 1)),
            RegionId::new(second_owner, 2),
            Vec::new(),
            ScopeOriginV1::Source(loop_body_origin(1)),
        ),
    );
    assert!(matches!(
        build_verified_loop_region_index_v1(&wrong_body_origin),
        Err(ResolvedLoopRegionVerificationErrorV1::LoopBodyContractMismatch(_))
    ));
}

#[test]
fn index_rejects_duplicate_loop_site_cardinality() {
    let owner = owner();
    let mut data = loop_data(owner);
    let duplicate_scope = ScopeId::new(owner, 3);
    let duplicate_region = RegionId::new(owner, 3);
    data.scopes.insert(
        duplicate_scope,
        ResolvedScopeRecordV1::new(
            ScopeKindV1::LoopBody,
            Some(ScopeId::new(owner, 1)),
            duplicate_region,
            Vec::new(),
            ScopeOriginV1::Source(loop_body_origin(0)),
        ),
    );
    data.regions.insert(
        duplicate_region,
        ResolvedRegionRecordV1::new(
            RegionKindV1::Loop,
            Some(RegionId::new(owner, 1)),
            Some(duplicate_scope),
            RegionOriginV1::Source(loop_origin(0)),
        ),
    );

    assert_eq!(
        build_verified_loop_region_index_v1(&data),
        Err(ResolvedLoopRegionVerificationErrorV1::DuplicateLoopSite(
            stmt(vec![SourcePathSegmentV1::Body(0)])
        ))
    );
}

#[test]
fn index_rejects_orphan_loop_body_scope_and_non_loop_owner_region() {
    let first_owner = owner();
    let mut missing_region = loop_data(first_owner);
    missing_region
        .regions
        .remove(&RegionId::new(first_owner, 2));
    assert_eq!(
        build_verified_loop_region_index_v1(&missing_region),
        Err(ResolvedLoopRegionVerificationErrorV1::OrphanLoopBodyScope(
            ScopeId::new(first_owner, 2)
        ))
    );

    let second_owner = owner();
    let mut wrong_kind = loop_data(second_owner);
    let orphan_scope = ScopeId::new(second_owner, 3);
    let orphan_region = RegionId::new(second_owner, 3);
    wrong_kind.scopes.insert(
        orphan_scope,
        ResolvedScopeRecordV1::new(
            ScopeKindV1::LoopBody,
            Some(ScopeId::new(second_owner, 1)),
            orphan_region,
            Vec::new(),
            ScopeOriginV1::Source(loop_body_origin(1)),
        ),
    );
    wrong_kind.regions.insert(
        orphan_region,
        ResolvedRegionRecordV1::new(
            RegionKindV1::LexicalScope,
            Some(RegionId::new(second_owner, 1)),
            Some(orphan_scope),
            RegionOriginV1::Source(loop_body_origin(1)),
        ),
    );

    assert_eq!(
        build_verified_loop_region_index_v1(&wrong_kind),
        Err(ResolvedLoopRegionVerificationErrorV1::OrphanLoopBodyScope(
            orphan_scope
        ))
    );
}
