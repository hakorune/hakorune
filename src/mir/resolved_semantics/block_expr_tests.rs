use std::collections::BTreeMap;

use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};

use super::ids::{FunctionOwnerIdV1, FunctionOwnerIssuerV1, RegionId, ScopeId};
use super::product::{ResolvedFunctionDataV1, ResolvedFunctionDraftV1};
use super::records::{
    RegionKindV1, RegionOriginV1, ResolvedRegionRecordV1, ResolvedScopeRecordV1, ScopeKindV1,
    ScopeOriginV1,
};
use super::resolver::{FunctionSemanticResolverSessionV1, ResolveFunctionErrorV1};
use super::shadow::ShadowResolveErrorV0;
use super::source_site::{FunctionOriginV1, SourceNodeSiteV1, SourcePathSegmentV1};
use super::verifier::{source_region_contains_site_v1, ResolvedFunctionVerificationErrorV1};
use super::FunctionSyntaxViewV1;

fn owner() -> FunctionOwnerIdV1 {
    FunctionOwnerIssuerV1::new_for_compilation()
        .unwrap()
        .issue()
        .unwrap()
}

fn node(segments: Vec<SourcePathSegmentV1>) -> SourceNodeSiteV1 {
    SourceNodeSiteV1::from_segments(segments)
}

fn blockexpr_root() -> SourceNodeSiteV1 {
    node(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::BlockExprPreludeRoot,
    ])
}

fn blockexpr_data(owner: FunctionOwnerIdV1) -> ResolvedFunctionDataV1 {
    let function_origin = FunctionOriginV1::new(0, 0);
    let function_scope = ScopeId::new(owner, 0);
    let blockexpr_scope = ScopeId::new(owner, 1);
    let function_region = RegionId::new(owner, 0);
    let blockexpr_region = RegionId::new(owner, 1);
    let blockexpr_origin = blockexpr_root();

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
                blockexpr_scope,
                ResolvedScopeRecordV1::new(
                    ScopeKindV1::BlockExpr,
                    Some(function_scope),
                    blockexpr_region,
                    Vec::new(),
                    ScopeOriginV1::Source(blockexpr_origin.clone()),
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
                blockexpr_region,
                ResolvedRegionRecordV1::new(
                    RegionKindV1::BlockExpr,
                    Some(function_region),
                    Some(blockexpr_scope),
                    RegionOriginV1::Source(blockexpr_origin),
                ),
            ),
        ]),
        declarations: BTreeMap::new(),
        variable_uses: BTreeMap::new(),
        assignment_targets: BTreeMap::new(),
        resolved_exits: BTreeMap::new(),
    }
}

fn seal(
    data: ResolvedFunctionDataV1,
) -> Result<super::VerifiedResolvedFunctionV1, ResolvedFunctionVerificationErrorV1> {
    ResolvedFunctionDraftV1 { data }.seal()
}

#[test]
fn sealed_blockexpr_pair_has_exact_kinds_origin_and_normalized_parity() {
    let first = seal(blockexpr_data(owner())).unwrap();
    let second = seal(blockexpr_data(owner())).unwrap();
    let scope = first.scope(ScopeId::new(first.owner(), 1)).unwrap();
    let region = first.region(RegionId::new(first.owner(), 1)).unwrap();

    assert_eq!(scope.kind(), ScopeKindV1::BlockExpr);
    assert_eq!(region.kind(), RegionKindV1::BlockExpr);
    assert_eq!(scope.owner_region(), RegionId::new(first.owner(), 1));
    assert_eq!(region.lexical_scope(), Some(ScopeId::new(first.owner(), 1)));
    assert_eq!(first.normalized_graph(), second.normalized_graph());
}

#[test]
fn seal_rejects_blockexpr_scope_with_non_blockexpr_region() {
    let owner = owner();
    let mut data = blockexpr_data(owner);
    data.regions.insert(
        RegionId::new(owner, 1),
        ResolvedRegionRecordV1::new(
            RegionKindV1::LexicalScope,
            Some(RegionId::new(owner, 0)),
            Some(ScopeId::new(owner, 1)),
            RegionOriginV1::Source(blockexpr_root()),
        ),
    );

    assert!(matches!(
        seal(data),
        Err(ResolvedFunctionVerificationErrorV1::BlockExprScopeContractMismatch(_))
    ));
}

#[test]
fn seal_rejects_blockexpr_region_with_non_blockexpr_scope() {
    let owner = owner();
    let mut data = blockexpr_data(owner);
    data.scopes.insert(
        ScopeId::new(owner, 1),
        ResolvedScopeRecordV1::new(
            ScopeKindV1::LexicalBlock,
            Some(ScopeId::new(owner, 0)),
            RegionId::new(owner, 1),
            Vec::new(),
            ScopeOriginV1::Source(blockexpr_root()),
        ),
    );

    assert!(matches!(
        seal(data),
        Err(ResolvedFunctionVerificationErrorV1::BlockExprRegionContractMismatch(_))
    ));
}

#[test]
fn seal_rejects_blockexpr_pair_with_different_exact_origins() {
    let owner = owner();
    let mut data = blockexpr_data(owner);
    data.regions.insert(
        RegionId::new(owner, 1),
        ResolvedRegionRecordV1::new(
            RegionKindV1::BlockExpr,
            Some(RegionId::new(owner, 0)),
            Some(ScopeId::new(owner, 1)),
            RegionOriginV1::Source(node(vec![
                SourcePathSegmentV1::Body(1),
                SourcePathSegmentV1::BlockExprPreludeRoot,
            ])),
        ),
    );

    assert!(matches!(
        seal(data),
        Err(ResolvedFunctionVerificationErrorV1::BlockExprScopeContractMismatch(_))
    ));
}

#[test]
fn blockexpr_region_contains_only_its_prelude_and_tail_descendants() {
    let origin = RegionOriginV1::Source(blockexpr_root());
    let contained = [
        node(vec![
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::BlockExprPrelude(0),
        ]),
        node(vec![
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::BlockExprPrelude(0),
            SourcePathSegmentV1::Value,
        ]),
        node(vec![
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::BlockExprTail,
        ]),
    ];
    for site in contained {
        assert!(source_region_contains_site_v1(
            RegionKindV1::BlockExpr,
            &origin,
            &site
        ));
    }
    for site in [
        node(vec![SourcePathSegmentV1::Body(1)]),
        node(vec![
            SourcePathSegmentV1::Body(1),
            SourcePathSegmentV1::BlockExprTail,
        ]),
        blockexpr_root(),
    ] {
        assert!(!source_region_contains_site_v1(
            RegionKindV1::BlockExpr,
            &origin,
            &site
        ));
    }
}

#[test]
fn canonical_resolver_keeps_blockexpr_explicitly_unsupported() {
    let tree = ASTNode::FunctionDeclaration {
        name: "fixture".into(),
        params: Vec::new(),
        param_decls: Vec::new(),
        return_type_name: None,
        body: vec![ASTNode::BlockExpr {
            prelude_stmts: Vec::new(),
            tail_expr: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(1),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }],
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    };
    let error = FunctionSemanticResolverSessionV1::new(0)
        .unwrap()
        .resolve(FunctionSyntaxViewV1::from_ast(&tree).unwrap())
        .unwrap_err();

    assert!(matches!(
        error,
        ResolveFunctionErrorV1::Syntax(
            ShadowResolveErrorV0::UnsupportedExpression {
                kind: "BlockExpr",
                ..
            } | ShadowResolveErrorV0::UnsupportedStatement {
                kind: "BlockExpr",
                ..
            }
        )
    ));
}
