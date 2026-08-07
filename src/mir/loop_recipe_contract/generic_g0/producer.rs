use crate::mir::loop_structural_facts::generic_g0::GenericG0SourceBrandV1;
use crate::mir::loop_structural_facts::{
    bind_resolved_loop_source_forest_v1, LoopSourceForestBindingRejectV1,
};
use crate::mir::numeric_substrate::NumericTarget;
use crate::mir::resolved_semantics::SemanticOwnerSourceKindV1;

use super::super::super::loop_recipe_contract::error::LoopRecipeRejectReasonV1;
use super::super::super::loop_recipe_contract::generic_g0_demand::VerifiedGenericRecipeDemandG0;
use super::super::super::loop_recipe_contract::ids::{LoopBindingKeyV1, LoopNodeKeyV1};
use super::super::super::loop_recipe_contract::join_sig::{
    LoopJoinSigElaboratorV1, LoopJoinSigRejectReasonV1,
};
use super::super::super::loop_recipe_contract::physical_input::VerifiedLoopPhysicalBoundaryV1;
use super::super::super::loop_recipe_contract::producer_id::LoopRecipeProducerIdV1;
use super::super::super::loop_recipe_contract::schema::{
    LoopRecipeArtifactV1, LoopRecipeProvenanceV1, LoopValueClassV1,
};
use super::super::super::loop_recipe_contract::source_bound_core::{
    issue_source_bound_core_v1, VerifiedLoopCoreProductV1,
};
use super::super::super::loop_recipe_contract::verify::LoopRecipeVerifierV1;
use super::after::{issue_after, GenericG0AfterRejectV1, VerifiedGenericAfterEffectG0};
use super::recipe::{generic_g0_recipe, GenericG0RecipeShapeRejectV1};
use super::relations::{build_relations, GenericG0RelationRejectV1};

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum GenericG0RecipeProducerRejectV1 {
    WindowBrandMismatch,
    SourceKindMismatch,
    SourceOwnerMismatch,
    SourceOriginMismatch,
    SourceSiteMismatch,
    SourceFrameMismatch,
    SourceForest(LoopSourceForestBindingRejectV1),
    RecipeShape(GenericG0RecipeShapeRejectV1),
    Recipe(LoopRecipeRejectReasonV1),
    JoinSig(LoopJoinSigRejectReasonV1),
    Relations(GenericG0RelationRejectV1),
    Core(LoopRecipeRejectReasonV1),
    After(GenericG0AfterRejectV1),
}

#[derive(Debug)]
pub(crate) struct VerifiedGenericRecipeProductG0 {
    core: VerifiedLoopCoreProductV1,
    after: VerifiedGenericAfterEffectG0,
    target: NumericTarget,
}

impl VerifiedGenericRecipeProductG0 {
    pub(crate) fn core(&self) -> &VerifiedLoopCoreProductV1 {
        &self.core
    }

    pub(crate) fn after(&self) -> &VerifiedGenericAfterEffectG0 {
        &self.after
    }

    pub(crate) const fn target(&self) -> NumericTarget {
        self.target
    }

    pub(crate) fn into_physical_boundary(self) -> VerifiedLoopPhysicalBoundaryV1 {
        VerifiedLoopPhysicalBoundaryV1::from_parts(self.core, self.after.into_after_binding())
    }
}

pub(crate) fn produce_generic_g0_recipe_v1(
    demand: VerifiedGenericRecipeDemandG0,
) -> Result<VerifiedGenericRecipeProductG0, GenericG0RecipeProducerRejectV1> {
    let (
        window_lease,
        source_brand,
        typed_bundle,
        post_loop_read,
        _profile,
        _mode,
        _coverage,
        _role_lease,
    ) = demand.into_parts();
    verify_brand(&window_lease, &source_brand)?;

    let (source_bundle, numeric, return_abi) = typed_bundle.into_parts();
    let (structural, source_types) = source_bundle.into_parts();
    let expected_source_binding = source_types
        .parameters()
        .get(1)
        .map(|row| row.binding)
        .ok_or(GenericG0RecipeProducerRejectV1::SourceOwnerMismatch)?;
    let (
        owner,
        origin,
        source_kind,
        forest,
        _,
        root_body,
        child_body,
        root_loop,
        _child_loop,
        outer_condition,
        inner_condition,
        outer_update,
        inner_update,
        tail,
        _,
        root_frame,
    ) = structural.into_parts();
    if source_kind != SemanticOwnerSourceKindV1::DeclaredFunction {
        return Err(GenericG0RecipeProducerRejectV1::SourceKindMismatch);
    }
    if source_types.owner() != owner {
        return Err(GenericG0RecipeProducerRejectV1::SourceOwnerMismatch);
    }
    if source_types.origin() != origin {
        return Err(GenericG0RecipeProducerRejectV1::SourceOriginMismatch);
    }
    if root_loop != *window_lease.site() {
        return Err(GenericG0RecipeProducerRejectV1::SourceSiteMismatch);
    }
    if !root_frame.matches(&window_lease.frame()) {
        return Err(GenericG0RecipeProducerRejectV1::SourceFrameMismatch);
    }
    let root_anchor = root_body
        .first()
        .ok_or(GenericG0RecipeProducerRejectV1::SourceSiteMismatch)?;
    let child_anchor = child_body
        .first()
        .ok_or(GenericG0RecipeProducerRejectV1::SourceSiteMismatch)?;

    let forest_binding = bind_resolved_loop_source_forest_v1(forest)
        .map_err(GenericG0RecipeProducerRejectV1::SourceForest)?;
    let recipe =
        generic_g0_recipe(&numeric).map_err(GenericG0RecipeProducerRejectV1::RecipeShape)?;
    let verified_recipe = LoopRecipeVerifierV1::verify(recipe.clone())
        .map_err(GenericG0RecipeProducerRejectV1::Recipe)?;
    let source_binding = forest_binding
        .into_source_binding(&verified_recipe)
        .map_err(|reason| GenericG0RecipeProducerRejectV1::SourceForest(reason))?;
    let artifact = LoopRecipeArtifactV1::new(
        LoopRecipeProvenanceV1::new(LoopRecipeProducerIdV1::GenericG0),
        source_binding,
        recipe,
    );
    let verified_artifact = LoopRecipeVerifierV1::verify_artifact(artifact)
        .map_err(GenericG0RecipeProducerRejectV1::Recipe)?;
    let join_sig = LoopJoinSigElaboratorV1::elaborate(verified_artifact.recipe())
        .map_err(GenericG0RecipeProducerRejectV1::JoinSig)?;
    let after_binding = join_sig
        .require_after_binding(
            LoopNodeKeyV1::new(0),
            LoopBindingKeyV1::new(1),
            LoopValueClassV1::I64,
        )
        .map_err(GenericG0RecipeProducerRejectV1::JoinSig)?;
    let (bindings, effects) = build_relations(
        owner,
        source_types.parameters(),
        &outer_condition,
        &inner_condition,
        &outer_update,
        &inner_update,
        &tail,
        root_anchor,
        child_anchor,
    )
    .map_err(GenericG0RecipeProducerRejectV1::Relations)?;
    let core = issue_source_bound_core_v1(verified_artifact, join_sig, owner, bindings, effects)
        .map_err(GenericG0RecipeProducerRejectV1::Core)?;
    let after = issue_after(
        after_binding,
        post_loop_read,
        return_abi,
        owner,
        root_frame,
        expected_source_binding,
    )
    .map_err(GenericG0RecipeProducerRejectV1::After)?;
    Ok(VerifiedGenericRecipeProductG0 {
        core,
        after,
        target: numeric.target(),
    })
}

fn verify_brand(
    lease: &crate::mir::resolved_semantics::VerifiedLoopFamilyWindowLeaseV1,
    brand: &GenericG0SourceBrandV1,
) -> Result<(), GenericG0RecipeProducerRejectV1> {
    if !brand.matches_window(lease) {
        return Err(GenericG0RecipeProducerRejectV1::WindowBrandMismatch);
    }
    Ok(())
}
