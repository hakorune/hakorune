use hakorune_mir_core::BindingId;

use crate::mir::resolved_semantics::{
    BindingOriginV1, BindingRefV1, FunctionOwnerIssuerV1, OwnedExprSiteV1, SourceBindingSiteV1,
    SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1, SourceStmtSiteV1,
};

use super::ids::{LoopBindingKeyV1, LoopBlockKeyV1, LoopItemKeyV1, LoopNodeKeyV1};
use super::join_sig::LoopJoinSigElaboratorV1;
use super::operation_effect::{
    LoopOperationEffectRejectV1, LoopOperationSourceEvidenceV1,
    VerifiedLoopOperationEffectProductV1,
};
use super::schema::{LoopOperationV1, LoopRecipeArtifactV1, LoopRecipeItemV1, LoopValueClassV1};
use super::source_bound_core::{
    issue_source_bound_core_for_test, LoopBindingEffectAnchorV1, LoopBindingEffectRelationV1,
    LoopBindingEffectRoleV1, LoopRecipeBindingRelationV1,
};
use super::verify::LoopRecipeVerifierV1;

const GOLDEN: &str = include_str!("fixtures/nested_predicate_v1.json");

fn owner() -> crate::mir::resolved_semantics::FunctionOwnerIdV1 {
    let mut issuer = FunctionOwnerIssuerV1::new_for_compilation().expect("owner issuer");
    issuer.issue().expect("owner")
}

fn source_binding(
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    slot: u32,
) -> BindingRefV1 {
    BindingRefV1::new(owner, BindingId::new(slot))
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

fn artifact() -> LoopRecipeArtifactV1 {
    serde_json::from_str(GOLDEN).expect("nested recipe golden")
}

fn core_and_evidence(
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
) -> (
    super::source_bound_core::VerifiedLoopCoreProductV1,
    Vec<LoopOperationSourceEvidenceV1>,
) {
    let artifact = artifact();
    let recipe = LoopRecipeVerifierV1::verify(artifact.recipe.clone()).expect("recipe");
    let join_sig = LoopJoinSigElaboratorV1::elaborate(&recipe).expect("JoinSig");
    let bindings = (0..3)
        .map(|key| {
            LoopRecipeBindingRelationV1::new(
                LoopBindingKeyV1::new(key),
                source_binding(owner, key),
                LoopValueClassV1::I64,
                BindingOriginV1::Source(SourceBindingSiteV1::Parameter { index: key }),
            )
        })
        .collect();
    let mut reads = 0;
    let mut writes = 0;
    let mut effects = Vec::new();
    let mut evidence = Vec::new();
    for block in &recipe.as_recipe().blocks {
        for item_key in &block.items {
            let row = recipe
                .as_recipe()
                .items
                .iter()
                .find(|row| row.key == *item_key)
                .expect("item row");
            let LoopRecipeItemV1::Operation { operation } = row.item else {
                continue;
            };
            let anchor = LoopBindingEffectAnchorV1::Expr(expr(owner, item_key.raw()));
            let (source_binding, role) = match operation {
                LoopOperationV1::ReadBinding { binding, .. } => {
                    let role = LoopBindingEffectRoleV1::SourceRead { ordinal: reads };
                    reads += 1;
                    (Some(source_binding(owner, binding.raw())), Some(role))
                }
                LoopOperationV1::WriteBinding { binding, .. } => {
                    let role = LoopBindingEffectRoleV1::SourceWrite { ordinal: writes };
                    writes += 1;
                    (Some(source_binding(owner, binding.raw())), Some(role))
                }
                _ => (None, None),
            };
            if let (Some(source_binding), Some(role)) = (source_binding, role) {
                effects.push(LoopBindingEffectRelationV1::new(
                    role,
                    match operation {
                        LoopOperationV1::ReadBinding { binding, .. }
                        | LoopOperationV1::WriteBinding { binding, .. } => binding,
                        _ => unreachable!(),
                    },
                    source_binding,
                    LoopValueClassV1::I64,
                    anchor.clone(),
                ));
            }
            evidence.push(LoopOperationSourceEvidenceV1::new(
                *item_key,
                anchor,
                source_loop(block.owner_loop.raw()),
                block.owner_loop,
                block.key,
                source_binding,
            ));
        }
    }
    let core = issue_source_bound_core_for_test(artifact, join_sig, owner, bindings, effects)
        .expect("source-bound core");
    (core, evidence)
}

#[test]
fn operation_effect_product_covers_nested_recipe_operations_once() {
    let (core, evidence) = core_and_evidence(owner());
    let product = VerifiedLoopOperationEffectProductV1::issue(core, evidence).unwrap();
    assert_eq!(product.evidence().len(), 19);
    assert_eq!(product.core().recipe().as_recipe().loops.len(), 2);
}

#[test]
fn operation_effect_product_rejects_duplicate_and_missing_items() {
    let owner = owner();
    let (core, mut evidence) = core_and_evidence(owner);
    let duplicate = evidence[0].clone();
    evidence.push(duplicate);
    assert!(matches!(
        VerifiedLoopOperationEffectProductV1::issue(core, evidence),
        Err(LoopOperationEffectRejectV1::DuplicateEvidence { item })
            if item == LoopItemKeyV1::new(0)
    ));

    let (core, mut evidence) = core_and_evidence(owner);
    evidence.pop();
    assert!(matches!(
        VerifiedLoopOperationEffectProductV1::issue(core, evidence),
        Err(LoopOperationEffectRejectV1::MissingEvidence { .. })
    ));
}

#[test]
fn operation_effect_product_rejects_foreign_owner_and_wrong_placement() {
    let main_owner = owner();
    let foreign_owner = owner();
    let (core, mut evidence) = core_and_evidence(main_owner);
    evidence[0] = LoopOperationSourceEvidenceV1::new(
        evidence[0].item(),
        LoopBindingEffectAnchorV1::Expr(expr(foreign_owner, 0)),
        evidence[0].source_loop().clone(),
        evidence[0].owner_loop(),
        evidence[0].block(),
        evidence[0].source_binding(),
    );
    assert!(matches!(
        VerifiedLoopOperationEffectProductV1::issue(core, evidence),
        Err(LoopOperationEffectRejectV1::OwnerMismatch { item })
            if item == LoopItemKeyV1::new(0)
    ));

    let (core, mut evidence) = core_and_evidence(main_owner);
    evidence[0] = LoopOperationSourceEvidenceV1::new(
        evidence[0].item(),
        evidence[0].anchor().clone(),
        evidence[0].source_loop().clone(),
        LoopNodeKeyV1::new(1),
        LoopBlockKeyV1::new(2),
        evidence[0].source_binding(),
    );
    assert!(matches!(
        VerifiedLoopOperationEffectProductV1::issue(core, evidence),
        Err(LoopOperationEffectRejectV1::PlacementMismatch { item })
            if item == LoopItemKeyV1::new(0)
    ));
}

#[test]
fn operation_effect_product_rejects_binding_evidence_on_pure_operation() {
    let owner = owner();
    let (core, mut evidence) = core_and_evidence(owner);
    let pure = evidence
        .iter()
        .find(|row| row.item() == LoopItemKeyV1::new(1))
        .expect("const operation evidence")
        .clone();
    evidence[1] = LoopOperationSourceEvidenceV1::new(
        pure.item(),
        pure.anchor().clone(),
        pure.source_loop().clone(),
        pure.owner_loop(),
        pure.block(),
        Some(source_binding(owner, 0)),
    );
    assert!(matches!(
        VerifiedLoopOperationEffectProductV1::issue(core, evidence),
        Err(LoopOperationEffectRejectV1::UnexpectedBindingEvidence { item })
            if item == LoopItemKeyV1::new(1)
    ));
}
