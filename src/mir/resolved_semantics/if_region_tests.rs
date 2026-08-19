use std::collections::BTreeMap;

use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};

use super::expression_source::ResolvedExpressionSourceInventoryV1;
use super::ids::{FunctionOwnerIdV1, FunctionOwnerIssuerV1, RegionId, ScopeId};
use super::if_region::{ResolvedIfRegionLookupErrorV1, ResolvedIfRegionVerificationErrorV1};
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

fn if_stmt(then_body: Vec<ASTNode>, else_body: Option<Vec<ASTNode>>) -> ASTNode {
    ASTNode::If {
        condition: Box::new(int(1)),
        then_body,
        else_body,
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

fn if_origin() -> SourceNodeSiteV1 {
    node(vec![SourcePathSegmentV1::Body(0)])
}

fn then_origin() -> SourceNodeSiteV1 {
    node(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::IfThenBody,
    ])
}

fn else_origin() -> SourceNodeSiteV1 {
    node(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::IfElseBody,
    ])
}

fn if_data(owner: FunctionOwnerIdV1, with_else: bool) -> ResolvedFunctionDataV1 {
    let function_origin = FunctionOriginV1::new(0, 0);
    let function_scope = ScopeId::new(owner, 0);
    let body_scope = ScopeId::new(owner, 1);
    let then_scope = ScopeId::new(owner, 2);
    let else_scope = ScopeId::new(owner, 3);
    let function_region = RegionId::new(owner, 0);
    let body_region = RegionId::new(owner, 1);
    let control_region = RegionId::new(owner, 2);
    let then_region = RegionId::new(owner, 3);
    let else_region = RegionId::new(owner, 4);
    let body_origin = node(vec![SourcePathSegmentV1::FunctionBody]);

    let mut scopes = BTreeMap::from([
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
            then_scope,
            ResolvedScopeRecordV1::new(
                ScopeKindV1::IfThen,
                Some(body_scope),
                then_region,
                Vec::new(),
                ScopeOriginV1::Source(then_origin()),
            ),
        ),
    ]);
    let mut regions = BTreeMap::from([
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
            control_region,
            ResolvedRegionRecordV1::new(
                RegionKindV1::If,
                Some(body_region),
                None,
                RegionOriginV1::Source(if_origin()),
            ),
        ),
        (
            then_region,
            ResolvedRegionRecordV1::new(
                RegionKindV1::IfThen,
                Some(control_region),
                Some(then_scope),
                RegionOriginV1::Source(then_origin()),
            ),
        ),
    ]);
    if with_else {
        scopes.insert(
            else_scope,
            ResolvedScopeRecordV1::new(
                ScopeKindV1::IfElse,
                Some(body_scope),
                else_region,
                Vec::new(),
                ScopeOriginV1::Source(else_origin()),
            ),
        );
        regions.insert(
            else_region,
            ResolvedRegionRecordV1::new(
                RegionKindV1::IfElse,
                Some(control_region),
                Some(else_scope),
                RegionOriginV1::Source(else_origin()),
            ),
        );
    }

    ResolvedFunctionDataV1 {
        explicit_extern_calls: BTreeMap::new(),
        owner,
        function_origin,
        root_profile: super::SemanticOwnerRootProfileV1::DeclaredFunction {
            receiver_policy: super::ReceiverPolicyV1::Absent,
        },
        function_scope,
        function_region,
        bindings: BTreeMap::new(),
        scopes,
        regions,
        declarations: BTreeMap::new(),
        variable_uses: BTreeMap::new(),
        assignment_targets: BTreeMap::new(),
        direct_call_targets: BTreeMap::new(),
        method_calls: BTreeMap::new(),
        expression_source: ResolvedExpressionSourceInventoryV1::default(),
        resolved_exits: BTreeMap::new(),
    }
}

#[allow(clippy::too_many_arguments)]
fn insert_pair(
    data: &mut ResolvedFunctionDataV1,
    owner: FunctionOwnerIdV1,
    scope_raw: u32,
    region_raw: u32,
    region_kind: RegionKindV1,
    scope_kind: ScopeKindV1,
    parent_region: RegionId,
    parent_scope: ScopeId,
    origin: SourceNodeSiteV1,
) {
    let scope = ScopeId::new(owner, scope_raw);
    let region = RegionId::new(owner, region_raw);
    data.scopes.insert(
        scope,
        ResolvedScopeRecordV1::new(
            scope_kind,
            Some(parent_scope),
            region,
            Vec::new(),
            ScopeOriginV1::Source(origin.clone()),
        ),
    );
    data.regions.insert(
        region,
        ResolvedRegionRecordV1::new(
            region_kind,
            Some(parent_region),
            Some(scope),
            RegionOriginV1::Source(origin),
        ),
    );
}

fn assert_pair(
    product: &VerifiedResolvedFunctionV1,
    pair: super::ResolvedScopeRegionPairV1,
    control: RegionId,
    surrounding_scope: ScopeId,
    region_kind: RegionKindV1,
    scope_kind: ScopeKindV1,
    origin: SourceNodeSiteV1,
) {
    let region = product.region(pair.region()).unwrap();
    let scope = product.scope(pair.scope()).unwrap();
    assert_eq!(region.kind(), region_kind);
    assert_eq!(region.parent(), Some(control));
    assert_eq!(region.lexical_scope(), Some(pair.scope()));
    assert_eq!(region.origin(), &RegionOriginV1::Source(origin.clone()));
    assert_eq!(scope.kind(), scope_kind);
    assert_eq!(scope.parent(), Some(surrounding_scope));
    assert_eq!(scope.owner_region(), pair.region());
    assert_eq!(scope.origin(), &ScopeOriginV1::Source(origin));
}

#[test]
fn resolver_indexes_same_span_sites_and_preserves_optional_else_topology() {
    let product = resolve(&function(vec![
        if_stmt(Vec::new(), None),
        if_stmt(Vec::new(), Some(Vec::new())),
    ]));
    let first_site = stmt(vec![SourcePathSegmentV1::Body(0)]);
    let second_site = stmt(vec![SourcePathSegmentV1::Body(1)]);
    let first = *product.if_region_bundle(&first_site).unwrap();
    let second = *product.if_region_bundle(&second_site).unwrap();

    assert_ne!(first.control(), second.control());
    assert!(first.else_pair().is_none());
    let explicit_empty = second.else_pair().expect("explicit empty else pair");
    assert!(product
        .scope(explicit_empty.scope())
        .unwrap()
        .declarations()
        .is_empty());

    let first_control = product.region(first.control()).unwrap();
    let surrounding_region = first_control.parent().unwrap();
    let surrounding_scope = product
        .region(surrounding_region)
        .unwrap()
        .lexical_scope()
        .unwrap();
    assert_eq!(first_control.kind(), RegionKindV1::If);
    assert_eq!(first_control.lexical_scope(), None);
    assert_pair(
        &product,
        first.then_pair(),
        first.control(),
        surrounding_scope,
        RegionKindV1::IfThen,
        ScopeKindV1::IfThen,
        node(vec![
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::IfThenBody,
        ]),
    );
}

#[test]
fn nested_if_sites_have_independent_exact_bundles() {
    let product = resolve(&function(vec![if_stmt(
        vec![if_stmt(Vec::new(), Some(Vec::new()))],
        None,
    )]));
    let outer = product
        .if_region_bundle(&stmt(vec![SourcePathSegmentV1::Body(0)]))
        .unwrap();
    let inner = product
        .if_region_bundle(&stmt(vec![
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::IfThen(0),
        ]))
        .unwrap();

    assert_ne!(outer.control(), inner.control());
    assert!(outer.else_pair().is_none());
    assert!(inner.else_pair().is_some());
    assert_eq!(
        product.region(inner.control()).unwrap().parent(),
        Some(outer.then_pair().region())
    );
}

#[test]
fn query_reports_a_typed_missing_exact_bundle() {
    let product = resolve(&function(vec![if_stmt(Vec::new(), None)]));
    let missing = stmt(vec![SourcePathSegmentV1::Body(9)]);
    assert_eq!(
        product.if_region_bundle(&missing),
        Err(ResolvedIfRegionLookupErrorV1::MissingExactBundle(missing))
    );
}

#[test]
fn manual_exact_bundle_ids_resolve_to_authoritative_records() {
    let product = seal(if_data(owner(), true)).unwrap();
    let bundle = *product
        .if_region_bundle(&stmt(vec![SourcePathSegmentV1::Body(0)]))
        .unwrap();
    let body_scope = ScopeId::new(product.owner(), 1);

    assert_pair(
        &product,
        bundle.then_pair(),
        bundle.control(),
        body_scope,
        RegionKindV1::IfThen,
        ScopeKindV1::IfThen,
        then_origin(),
    );
    assert_pair(
        &product,
        bundle.else_pair().unwrap(),
        bundle.control(),
        body_scope,
        RegionKindV1::IfElse,
        ScopeKindV1::IfElse,
        else_origin(),
    );
}

#[test]
fn seal_rejects_missing_control_and_missing_required_then() {
    let first_owner = owner();
    let mut missing_control = if_data(first_owner, false);
    missing_control
        .regions
        .remove(&RegionId::new(first_owner, 2));
    missing_control.regions.insert(
        RegionId::new(first_owner, 3),
        ResolvedRegionRecordV1::new(
            RegionKindV1::IfThen,
            Some(RegionId::new(first_owner, 1)),
            Some(ScopeId::new(first_owner, 2)),
            RegionOriginV1::Source(then_origin()),
        ),
    );
    assert!(matches!(
        seal(missing_control),
        Err(ResolvedFunctionVerificationErrorV1::IfRegion(
            ResolvedIfRegionVerificationErrorV1::OrphanBranchRegion(_)
        ))
    ));

    let second_owner = owner();
    let mut missing_then = if_data(second_owner, false);
    missing_then.regions.remove(&RegionId::new(second_owner, 3));
    missing_then.scopes.remove(&ScopeId::new(second_owner, 2));
    assert!(matches!(
        seal(missing_then),
        Err(ResolvedFunctionVerificationErrorV1::IfRegion(
            ResolvedIfRegionVerificationErrorV1::MissingThenPair(_)
        ))
    ));
}

#[test]
fn seal_rejects_wrong_control_and_branch_kind_or_origin() {
    let first_owner = owner();
    let mut wrong_control_kind = if_data(first_owner, false);
    wrong_control_kind.regions.insert(
        RegionId::new(first_owner, 2),
        ResolvedRegionRecordV1::new(
            RegionKindV1::Loop,
            Some(RegionId::new(first_owner, 1)),
            None,
            RegionOriginV1::Source(if_origin()),
        ),
    );
    assert!(seal(wrong_control_kind).is_err());

    let second_owner = owner();
    let mut wrong_then_kind = if_data(second_owner, false);
    wrong_then_kind.regions.insert(
        RegionId::new(second_owner, 3),
        ResolvedRegionRecordV1::new(
            RegionKindV1::LexicalScope,
            Some(RegionId::new(second_owner, 2)),
            Some(ScopeId::new(second_owner, 2)),
            RegionOriginV1::Source(then_origin()),
        ),
    );
    assert!(seal(wrong_then_kind).is_err());

    let third_owner = owner();
    let mut wrong_then_origin = if_data(third_owner, false);
    let wrong = node(vec![
        SourcePathSegmentV1::Body(1),
        SourcePathSegmentV1::IfThenBody,
    ]);
    wrong_then_origin.regions.insert(
        RegionId::new(third_owner, 3),
        ResolvedRegionRecordV1::new(
            RegionKindV1::IfThen,
            Some(RegionId::new(third_owner, 2)),
            Some(ScopeId::new(third_owner, 2)),
            RegionOriginV1::Source(wrong.clone()),
        ),
    );
    wrong_then_origin.scopes.insert(
        ScopeId::new(third_owner, 2),
        ResolvedScopeRecordV1::new(
            ScopeKindV1::IfThen,
            Some(ScopeId::new(third_owner, 1)),
            RegionId::new(third_owner, 3),
            Vec::new(),
            ScopeOriginV1::Source(wrong),
        ),
    );
    assert!(seal(wrong_then_origin).is_err());
}

#[test]
fn seal_rejects_control_scope_parent_and_derived_origin_mismatch() {
    let first_owner = owner();
    let mut scoped_control = if_data(first_owner, false);
    let control_scope = ScopeId::new(first_owner, 4);
    scoped_control.scopes.insert(
        control_scope,
        ResolvedScopeRecordV1::new(
            ScopeKindV1::LexicalBlock,
            Some(ScopeId::new(first_owner, 1)),
            RegionId::new(first_owner, 2),
            Vec::new(),
            ScopeOriginV1::Source(if_origin()),
        ),
    );
    scoped_control.regions.insert(
        RegionId::new(first_owner, 2),
        ResolvedRegionRecordV1::new(
            RegionKindV1::If,
            Some(RegionId::new(first_owner, 1)),
            Some(control_scope),
            RegionOriginV1::Source(if_origin()),
        ),
    );
    assert!(matches!(
        seal(scoped_control),
        Err(ResolvedFunctionVerificationErrorV1::IfRegion(
            ResolvedIfRegionVerificationErrorV1::ControlContractMismatch(_)
        ))
    ));

    let second_owner = owner();
    let mut wrong_parent = if_data(second_owner, false);
    wrong_parent.regions.insert(
        RegionId::new(second_owner, 2),
        ResolvedRegionRecordV1::new(
            RegionKindV1::If,
            Some(RegionId::new(second_owner, 0)),
            None,
            RegionOriginV1::Source(if_origin()),
        ),
    );
    wrong_parent.scopes.insert(
        ScopeId::new(second_owner, 2),
        ResolvedScopeRecordV1::new(
            ScopeKindV1::IfThen,
            Some(ScopeId::new(second_owner, 0)),
            RegionId::new(second_owner, 3),
            Vec::new(),
            ScopeOriginV1::Source(then_origin()),
        ),
    );
    assert!(matches!(
        seal(wrong_parent),
        Err(ResolvedFunctionVerificationErrorV1::IfRegion(
            ResolvedIfRegionVerificationErrorV1::ControlContractMismatch(_)
        ))
    ));

    let third_owner = owner();
    let mut wrong_origin = if_data(third_owner, false);
    wrong_origin.regions.insert(
        RegionId::new(third_owner, 2),
        ResolvedRegionRecordV1::new(
            RegionKindV1::If,
            Some(RegionId::new(third_owner, 1)),
            None,
            RegionOriginV1::Source(node(vec![SourcePathSegmentV1::Body(1)])),
        ),
    );
    assert!(matches!(
        seal(wrong_origin),
        Err(ResolvedFunctionVerificationErrorV1::IfRegion(
            ResolvedIfRegionVerificationErrorV1::MissingThenPair(_)
        ))
    ));
}

#[test]
fn seal_rejects_wrong_region_parent_and_scope_parent() {
    let first_owner = owner();
    let mut wrong_region_parent = if_data(first_owner, false);
    wrong_region_parent.regions.insert(
        RegionId::new(first_owner, 3),
        ResolvedRegionRecordV1::new(
            RegionKindV1::IfThen,
            Some(RegionId::new(first_owner, 1)),
            Some(ScopeId::new(first_owner, 2)),
            RegionOriginV1::Source(then_origin()),
        ),
    );
    assert!(matches!(
        seal(wrong_region_parent),
        Err(ResolvedFunctionVerificationErrorV1::IfRegion(
            ResolvedIfRegionVerificationErrorV1::BranchContractMismatch(_)
        ))
    ));

    let second_owner = owner();
    let mut wrong_scope_parent = if_data(second_owner, false);
    wrong_scope_parent.scopes.insert(
        ScopeId::new(second_owner, 2),
        ResolvedScopeRecordV1::new(
            ScopeKindV1::IfThen,
            Some(ScopeId::new(second_owner, 0)),
            RegionId::new(second_owner, 3),
            Vec::new(),
            ScopeOriginV1::Source(then_origin()),
        ),
    );
    assert!(matches!(
        seal(wrong_scope_parent),
        Err(ResolvedFunctionVerificationErrorV1::IfRegion(
            ResolvedIfRegionVerificationErrorV1::BranchContractMismatch(_)
        ))
    ));
}

#[test]
fn seal_rejects_broken_reciprocal_and_orphan_branch_records() {
    let first_owner = owner();
    let mut broken_reciprocal = if_data(first_owner, false);
    broken_reciprocal.regions.insert(
        RegionId::new(first_owner, 3),
        ResolvedRegionRecordV1::new(
            RegionKindV1::IfThen,
            Some(RegionId::new(first_owner, 2)),
            Some(ScopeId::new(first_owner, 1)),
            RegionOriginV1::Source(then_origin()),
        ),
    );
    assert!(seal(broken_reciprocal).is_err());

    let second_owner = owner();
    let mut orphan = if_data(second_owner, false);
    let orphan_scope = ScopeId::new(second_owner, 4);
    let orphan_region = RegionId::new(second_owner, 5);
    let orphan_origin = node(vec![
        SourcePathSegmentV1::Body(1),
        SourcePathSegmentV1::IfElseBody,
    ]);
    orphan.scopes.insert(
        orphan_scope,
        ResolvedScopeRecordV1::new(
            ScopeKindV1::IfElse,
            Some(ScopeId::new(second_owner, 1)),
            orphan_region,
            Vec::new(),
            ScopeOriginV1::Source(orphan_origin.clone()),
        ),
    );
    orphan.regions.insert(
        orphan_region,
        ResolvedRegionRecordV1::new(
            RegionKindV1::IfElse,
            Some(RegionId::new(second_owner, 1)),
            Some(orphan_scope),
            RegionOriginV1::Source(orphan_origin),
        ),
    );
    assert!(matches!(
        seal(orphan),
        Err(ResolvedFunctionVerificationErrorV1::IfRegion(
            ResolvedIfRegionVerificationErrorV1::OrphanBranchRegion(_)
        ))
    ));
}

#[test]
fn seal_rejects_duplicate_branch_cardinality_and_orphan_branch_scope() {
    let first_owner = owner();
    let mut duplicate_then = if_data(first_owner, false);
    insert_pair(
        &mut duplicate_then,
        first_owner,
        4,
        5,
        RegionKindV1::IfThen,
        ScopeKindV1::IfThen,
        RegionId::new(first_owner, 2),
        ScopeId::new(first_owner, 1),
        then_origin(),
    );
    assert!(matches!(
        seal(duplicate_then),
        Err(ResolvedFunctionVerificationErrorV1::DuplicateScopeOrigin)
            | Err(ResolvedFunctionVerificationErrorV1::DuplicateRegionOrigin)
    ));

    let second_owner = owner();
    let mut duplicate_else = if_data(second_owner, true);
    insert_pair(
        &mut duplicate_else,
        second_owner,
        4,
        5,
        RegionKindV1::IfElse,
        ScopeKindV1::IfElse,
        RegionId::new(second_owner, 2),
        ScopeId::new(second_owner, 1),
        else_origin(),
    );
    assert!(matches!(
        seal(duplicate_else),
        Err(ResolvedFunctionVerificationErrorV1::DuplicateScopeOrigin)
            | Err(ResolvedFunctionVerificationErrorV1::DuplicateRegionOrigin)
    ));

    let third_owner = owner();
    let mut orphan_scope = if_data(third_owner, false);
    insert_pair(
        &mut orphan_scope,
        third_owner,
        4,
        5,
        RegionKindV1::LexicalScope,
        ScopeKindV1::IfElse,
        RegionId::new(third_owner, 1),
        ScopeId::new(third_owner, 1),
        node(vec![
            SourcePathSegmentV1::Body(1),
            SourcePathSegmentV1::IfElseBody,
        ]),
    );
    assert!(matches!(
        seal(orphan_scope),
        Err(ResolvedFunctionVerificationErrorV1::IfRegion(
            ResolvedIfRegionVerificationErrorV1::OrphanBranchScope(_)
        ))
    ));
}
