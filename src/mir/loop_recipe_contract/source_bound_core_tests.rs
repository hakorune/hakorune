use hakorune_mir_core::BindingId;

use crate::mir::resolved_semantics::{
    BindingOriginV1, BindingRefV1, FunctionOwnerIssuerV1, OwnedExprSiteV1, SourceBindingSiteV1,
    SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1, SourceStmtSiteV1,
};

use super::error::LoopRecipeRejectReasonV1 as Reject;
use super::ids::{LoopBindingKeyV1, LoopCarrierKeyV1};
use super::join_sig::LoopJoinSigElaboratorV1;
use super::schema::{LoopRecipeArtifactV1, LoopValueClassV1};
use super::source_bound_core::{
    issue_source_bound_core_for_test, LoopBindingEffectAnchorV1, LoopBindingEffectRelationV1,
    LoopBindingEffectRoleV1, LoopRecipeBindingRelationV1,
};
use super::verify::LoopRecipeVerifierV1;

const GOLDEN: &str = include_str!("fixtures/nested_predicate_v1.json");
const ALTERNATE_GOLDEN: &str = include_str!("fixtures/accum_nested_v1.json");

fn artifact() -> LoopRecipeArtifactV1 {
    serde_json::from_str(GOLDEN).expect("nested recipe golden")
}

fn owner() -> crate::mir::resolved_semantics::FunctionOwnerIdV1 {
    let mut issuer = FunctionOwnerIssuerV1::new_for_compilation().expect("owner issuer");
    issuer.issue().expect("owner")
}

fn binding(owner: crate::mir::resolved_semantics::FunctionOwnerIdV1, slot: u32) -> BindingRefV1 {
    BindingRefV1::new(owner, BindingId::new(slot))
}

fn declaration(index: u32) -> BindingOriginV1 {
    BindingOriginV1::Source(SourceBindingSiteV1::Parameter { index })
}

fn expr(owner: crate::mir::resolved_semantics::FunctionOwnerIdV1, index: u32) -> OwnedExprSiteV1 {
    OwnedExprSiteV1::new(
        owner,
        SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
            SourcePathSegmentV1::Body(index),
        ])),
    )
}

fn source_loop(index: u32) -> SourceStmtSiteV1 {
    SourceStmtSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
        SourcePathSegmentV1::LoopBody(index),
    ]))
}

fn rows(
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
) -> (
    Vec<LoopRecipeBindingRelationV1>,
    Vec<LoopBindingEffectRelationV1>,
) {
    let bindings = (0..3)
        .map(|key| {
            LoopRecipeBindingRelationV1::new(
                LoopBindingKeyV1::new(key),
                binding(owner, key),
                LoopValueClassV1::I64,
                declaration(key),
            )
        })
        .collect();
    let effects = vec![
        LoopBindingEffectRelationV1::new(
            LoopBindingEffectRoleV1::SourceRead { ordinal: 0 },
            LoopBindingKeyV1::new(0),
            binding(owner, 0),
            LoopValueClassV1::I64,
            LoopBindingEffectAnchorV1::Expr(expr(owner, 0)),
        ),
        LoopBindingEffectRelationV1::new(
            LoopBindingEffectRoleV1::SourceWrite { ordinal: 0 },
            LoopBindingKeyV1::new(1),
            binding(owner, 1),
            LoopValueClassV1::I64,
            LoopBindingEffectAnchorV1::Expr(expr(owner, 1)),
        ),
        LoopBindingEffectRelationV1::new(
            LoopBindingEffectRoleV1::DerivedCarrierEntry,
            LoopBindingKeyV1::new(2),
            binding(owner, 2),
            LoopValueClassV1::I64,
            LoopBindingEffectAnchorV1::DerivedCarrierEntry {
                owner,
                source_loop: source_loop(0),
                carrier: LoopCarrierKeyV1::new(2),
            },
        ),
    ];
    (bindings, effects)
}

fn issue(
    artifact: LoopRecipeArtifactV1,
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    bindings: Vec<LoopRecipeBindingRelationV1>,
    effects: Vec<LoopBindingEffectRelationV1>,
) -> Result<super::source_bound_core::VerifiedLoopCoreProductV1, Reject> {
    let recipe = LoopRecipeVerifierV1::verify(artifact.recipe.clone()).expect("recipe");
    let join_sig = LoopJoinSigElaboratorV1::elaborate(&recipe).expect("JoinSig");
    issue_source_bound_core_for_test(artifact, join_sig, owner, bindings, effects)
}

#[test]
fn source_bound_core_seals_recipe_joinsig_and_relations_once() {
    let owner = owner();
    let (bindings, effects) = rows(owner);
    let product = issue(artifact(), owner, bindings, effects).expect("core product");
    assert_eq!(product.owner(), owner);
    assert_eq!(product.binding_relations().len(), 3);
    assert_eq!(product.effect_relations().len(), 3);
    assert_eq!(product.recipe().as_recipe().bindings.len(), 3);
    assert_eq!(product.join_sig().as_sig().loops.len(), 2);
}

#[test]
fn source_bound_core_rejects_foreign_binding_and_uncovered_rows() {
    let main_owner = owner();
    let foreign = owner();
    let (mut bindings, effects) = rows(main_owner);
    bindings[1] = LoopRecipeBindingRelationV1::new(
        LoopBindingKeyV1::new(1),
        binding(foreign, 1),
        LoopValueClassV1::I64,
        declaration(1),
    );
    assert!(matches!(
        issue(artifact(), main_owner, bindings, effects),
        Err(Reject::SourceBoundForeignBinding { key }) if key == LoopBindingKeyV1::new(1)
    ));

    let (mut bindings, effects) = rows(main_owner);
    bindings.pop();
    assert!(matches!(
        issue(artifact(), main_owner, bindings, effects),
        Err(Reject::SourceBoundBindingCoverageMismatch { .. })
    ));
}

#[test]
fn source_bound_core_rejects_derived_carrier_and_duplicate_effect_mismatch() {
    let owner = owner();
    let (bindings, mut effects) = rows(owner);
    effects[2] = LoopBindingEffectRelationV1::new(
        LoopBindingEffectRoleV1::DerivedCarrierEntry,
        LoopBindingKeyV1::new(2),
        binding(owner, 2),
        LoopValueClassV1::I64,
        LoopBindingEffectAnchorV1::DerivedCarrierEntry {
            owner,
            source_loop: source_loop(0),
            carrier: LoopCarrierKeyV1::new(0),
        },
    );
    assert!(matches!(
        issue(artifact(), owner, bindings.clone(), effects),
        Err(Reject::SourceBoundDerivedCarrierMismatch { carrier })
            if carrier == LoopCarrierKeyV1::new(0)
    ));

    let (bindings, mut effects) = rows(owner);
    effects[2] = LoopBindingEffectRelationV1::new(
        LoopBindingEffectRoleV1::DerivedCarrierEntry,
        LoopBindingKeyV1::new(2),
        binding(owner, 2),
        LoopValueClassV1::I64,
        LoopBindingEffectAnchorV1::DerivedCarrierEntry {
            owner,
            source_loop: SourceStmtSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
                SourcePathSegmentV1::Body(0),
            ])),
            carrier: LoopCarrierKeyV1::new(2),
        },
    );
    assert!(matches!(
        issue(artifact(), owner, bindings.clone(), effects),
        Err(Reject::SourceBoundDerivedAnchorEmpty { carrier })
            if carrier == LoopCarrierKeyV1::new(2)
    ));

    let mut duplicate = rows(owner).1;
    duplicate.push(duplicate[0].clone());
    assert!(matches!(
        issue(artifact(), owner, bindings, duplicate),
        Err(Reject::SourceBoundDuplicateEffect { .. })
    ));
}

#[test]
fn source_bound_core_rejects_join_sig_from_different_recipe_shape() {
    let owner = owner();
    let (bindings, effects) = rows(owner);
    let alternate = artifact();
    let alternate_artifact: LoopRecipeArtifactV1 =
        serde_json::from_str(ALTERNATE_GOLDEN).expect("alternate recipe golden");
    let alternate_recipe =
        LoopRecipeVerifierV1::verify(alternate_artifact.recipe.clone()).expect("recipe");
    let alternate_sig = LoopJoinSigElaboratorV1::elaborate(&alternate_recipe).expect("JoinSig");
    let result =
        issue_source_bound_core_for_test(alternate, alternate_sig, owner, bindings, effects);
    assert!(
        matches!(result, Err(Reject::SourceBoundJoinSigMismatch)),
        "{result:?}"
    );
}
