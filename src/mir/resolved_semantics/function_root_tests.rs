use std::collections::BTreeMap;

use super::expression_source::ResolvedExpressionSourceInventoryV1;
use super::function_root::{
    build_verified_function_lowering_roots_v1, ResolvedFunctionRootVerificationErrorV1,
};
use super::ids::{FunctionOwnerIssuerV1, RegionId, ScopeId};
use super::product::{ResolvedFunctionDataV1, ResolvedFunctionDraftV1};
use super::records::{
    RegionKindV1, RegionOriginV1, ResolvedRegionRecordV1, ResolvedScopeRecordV1, ScopeKindV1,
    ScopeOriginV1,
};
use super::source_site::{FunctionOriginV1, SourceNodeSiteV1, SourcePathSegmentV1};

fn owner() -> super::FunctionOwnerIdV1 {
    FunctionOwnerIssuerV1::new_for_compilation()
        .unwrap()
        .issue()
        .unwrap()
}

fn root_site(segment: SourcePathSegmentV1) -> SourceNodeSiteV1 {
    SourceNodeSiteV1::from_segments(vec![segment])
}

fn data(body_segment: SourcePathSegmentV1) -> ResolvedFunctionDataV1 {
    let owner = owner();
    let function_origin = FunctionOriginV1::new(0, 0);
    let function_scope = ScopeId::new(owner, 0);
    let body_scope = ScopeId::new(owner, 1);
    let function_region = RegionId::new(owner, 0);
    let body_region = RegionId::new(owner, 1);
    let body_origin = root_site(body_segment.clone());
    ResolvedFunctionDataV1 {
        owner,
        function_origin,
        root_profile: match body_segment {
            SourcePathSegmentV1::FunctionBody => {
                super::SemanticOwnerRootProfileV1::DeclaredFunction {
                    receiver_policy: super::ReceiverPolicyV1::Absent,
                }
            }
            SourcePathSegmentV1::LambdaBodyRoot => super::SemanticOwnerRootProfileV1::Lambda,
            _ => super::SemanticOwnerRootProfileV1::DeclaredFunction {
                receiver_policy: super::ReceiverPolicyV1::Absent,
            },
        },
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
        ]),
        declarations: BTreeMap::new(),
        variable_uses: BTreeMap::new(),
        assignment_targets: BTreeMap::new(),
        direct_call_targets: BTreeMap::new(),
        method_calls: BTreeMap::new(),
        expression_source: ResolvedExpressionSourceInventoryV1::default(),
        resolved_exits: BTreeMap::new(),
    }
}

#[test]
fn seal_publishes_exact_function_and_function_body_roots() {
    let product = ResolvedFunctionDraftV1 {
        data: data(SourcePathSegmentV1::FunctionBody),
    }
    .seal()
    .unwrap();
    let roots = product.lowering_roots();

    assert_eq!(roots.function_pair().scope(), product.function_scope());
    assert_eq!(roots.function_pair().region(), product.function_region());
    let body_scope = product.scope(roots.body_pair().scope()).unwrap();
    let body_region = product.region(roots.body_pair().region()).unwrap();
    assert_eq!(body_scope.kind(), ScopeKindV1::LexicalBlock);
    assert_eq!(body_scope.parent(), Some(product.function_scope()));
    assert_eq!(body_region.kind(), RegionKindV1::Sequence);
    assert_eq!(body_region.parent(), Some(product.function_region()));
}

#[test]
fn lambda_body_root_uses_the_same_typed_carrier() {
    let product = ResolvedFunctionDraftV1 {
        data: data(SourcePathSegmentV1::LambdaBodyRoot),
    }
    .seal()
    .unwrap();

    assert_eq!(
        product
            .region(product.lowering_roots().body_pair().region())
            .unwrap()
            .kind(),
        RegionKindV1::Sequence
    );
}

#[test]
fn root_builder_rejects_missing_and_duplicate_body_roots() {
    let mut missing = data(SourcePathSegmentV1::FunctionBody);
    let body_region = RegionId::new(missing.owner, 1);
    missing
        .regions
        .get_mut(&body_region)
        .unwrap()
        .clone_from(&ResolvedRegionRecordV1::new(
            RegionKindV1::Sequence,
            Some(missing.function_region),
            Some(ScopeId::new(missing.owner, 1)),
            RegionOriginV1::Source(root_site(SourcePathSegmentV1::ScopeBodyRoot)),
        ));
    assert!(matches!(
        build_verified_function_lowering_roots_v1(&missing),
        Err(ResolvedFunctionRootVerificationErrorV1::BodyRegionCardinality { actual: 0 })
    ));

    let mut duplicate = data(SourcePathSegmentV1::FunctionBody);
    let second_scope = ScopeId::new(duplicate.owner, 2);
    let second_region = RegionId::new(duplicate.owner, 2);
    let second_origin = root_site(SourcePathSegmentV1::FunctionBody);
    duplicate.scopes.insert(
        second_scope,
        ResolvedScopeRecordV1::new(
            ScopeKindV1::LexicalBlock,
            Some(duplicate.function_scope),
            second_region,
            Vec::new(),
            ScopeOriginV1::Source(second_origin.clone()),
        ),
    );
    duplicate.regions.insert(
        second_region,
        ResolvedRegionRecordV1::new(
            RegionKindV1::Sequence,
            Some(duplicate.function_region),
            Some(second_scope),
            RegionOriginV1::Source(second_origin),
        ),
    );
    assert!(matches!(
        build_verified_function_lowering_roots_v1(&duplicate),
        Err(ResolvedFunctionRootVerificationErrorV1::BodyRegionCardinality { actual: 2 })
    ));
}

#[test]
fn root_builder_rejects_profile_and_body_root_mismatch() {
    let mut mismatched = data(SourcePathSegmentV1::FunctionBody);
    mismatched.root_profile = super::SemanticOwnerRootProfileV1::Script;
    assert!(matches!(
        build_verified_function_lowering_roots_v1(&mismatched),
        Err(ResolvedFunctionRootVerificationErrorV1::BodyRegionCardinality { actual: 0 })
    ));
}

#[test]
fn root_builder_rejects_wrong_body_parent_and_reciprocity() {
    let mut wrong_parent = data(SourcePathSegmentV1::FunctionBody);
    let body_region = RegionId::new(wrong_parent.owner, 1);
    let body_scope = ScopeId::new(wrong_parent.owner, 1);
    let origin = root_site(SourcePathSegmentV1::FunctionBody);
    wrong_parent.regions.insert(
        body_region,
        ResolvedRegionRecordV1::new(
            RegionKindV1::Sequence,
            None,
            Some(body_scope),
            RegionOriginV1::Source(origin.clone()),
        ),
    );
    assert_eq!(
        build_verified_function_lowering_roots_v1(&wrong_parent),
        Err(ResolvedFunctionRootVerificationErrorV1::BodyPairContractMismatch)
    );

    let mut wrong_pair = data(SourcePathSegmentV1::FunctionBody);
    let body_scope = ScopeId::new(wrong_pair.owner, 1);
    wrong_pair.scopes.insert(
        body_scope,
        ResolvedScopeRecordV1::new(
            ScopeKindV1::LexicalBlock,
            Some(wrong_pair.function_scope),
            wrong_pair.function_region,
            Vec::new(),
            ScopeOriginV1::Source(origin),
        ),
    );
    assert_eq!(
        build_verified_function_lowering_roots_v1(&wrong_pair),
        Err(ResolvedFunctionRootVerificationErrorV1::BodyPairContractMismatch)
    );
}

#[test]
fn root_builder_rejects_a_broken_function_pair() {
    let mut broken = data(SourcePathSegmentV1::FunctionBody);
    let function_scope = broken.function_scope;
    let function_region = broken.function_region;
    broken.scopes.insert(
        function_scope,
        ResolvedScopeRecordV1::new(
            ScopeKindV1::Function,
            None,
            RegionId::new(broken.owner, 1),
            Vec::new(),
            ScopeOriginV1::Function(broken.function_origin),
        ),
    );
    assert_eq!(
        build_verified_function_lowering_roots_v1(&broken),
        Err(ResolvedFunctionRootVerificationErrorV1::FunctionPairContractMismatch)
    );
    assert_ne!(RegionId::new(broken.owner, 1), function_region);
}
