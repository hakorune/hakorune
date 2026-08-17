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
use super::super::super::loop_recipe_contract::{
    VerifiedLoopContinuationContractV1, VerifiedLoopSemanticContextV1,
};
use super::super::operation_physical_demand::{
    LoopOperationPhysicalDemandRejectV1, PreparedLoopOperationProgramV1,
    VerifiedLoopOperationPhysicalDemandV1,
};
use super::after::{
    issue_after, GenericG0AfterRejectV1, VerifiedGenericAfterEffectG0,
    VerifiedGenericG0TailCapabilityV1,
};
use super::operation_effect::issue_generic_g0_operation_effect_v1;
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
    OperationEffect(super::super::operation_effect::LoopOperationEffectRejectV1),
}

#[derive(Debug)]
pub(crate) struct VerifiedGenericRecipeProductG0 {
    operation_effect: super::super::operation_effect::VerifiedLoopOperationEffectProductV1,
    after: VerifiedGenericAfterEffectG0,
    context: VerifiedLoopSemanticContextV1,
    target: NumericTarget,
}

impl VerifiedGenericRecipeProductG0 {
    pub(crate) fn core(&self) -> &VerifiedLoopCoreProductV1 {
        self.operation_effect.core()
    }

    pub(crate) fn operation_effect(
        &self,
    ) -> &super::super::operation_effect::VerifiedLoopOperationEffectProductV1 {
        &self.operation_effect
    }

    pub(crate) fn after(&self) -> &VerifiedGenericAfterEffectG0 {
        &self.after
    }

    pub(crate) fn context(&self) -> &VerifiedLoopSemanticContextV1 {
        &self.context
    }

    pub(crate) const fn target(&self) -> NumericTarget {
        self.target
    }

    /// Consume the complete Generic operation product into the neutral
    /// Builder-free program.  This is the production ownership transition;
    /// the old demand-part split remains test-only below.
    pub(crate) fn into_prepared_operation_program(
        self,
    ) -> Result<PreparedLoopOperationProgramV1, LoopOperationPhysicalDemandRejectV1> {
        let Self {
            operation_effect,
            after,
            context,
            target: _,
        } = self;
        let continuation = VerifiedLoopContinuationContractV1::from_after(
            operation_effect.core().owner(),
            after.into_after_binding(),
        );
        VerifiedLoopOperationPhysicalDemandV1::issue(context, operation_effect, continuation)?
            .prepare_all()
    }

    pub(crate) fn into_physical_boundary(self) -> VerifiedLoopPhysicalBoundaryV1 {
        let (core, _) = self.operation_effect.into_parts();
        VerifiedLoopPhysicalBoundaryV1::from_parts(core, self.after.into_after_binding())
    }

    #[cfg(test)]
    pub(crate) fn into_operation_effect(
        self,
    ) -> super::super::operation_effect::VerifiedLoopOperationEffectProductV1 {
        self.operation_effect
    }

    #[cfg(test)]
    pub(crate) fn into_operation_demand_parts(
        self,
    ) -> (
        super::super::operation_effect::VerifiedLoopOperationEffectProductV1,
        VerifiedLoopSemanticContextV1,
        VerifiedLoopContinuationContractV1,
    ) {
        let Self {
            operation_effect,
            after,
            context,
            target: _,
        } = self;
        let continuation = VerifiedLoopContinuationContractV1::from_after(
            operation_effect.core().owner(),
            after.into_after_binding(),
        );
        (operation_effect, context, continuation)
    }

    /// Test-only physical ingress split.  Unlike the legacy topology helper,
    /// this preserves the G0 tail while moving only the neutral After binding
    /// into the common continuation demand.
    #[cfg(test)]
    pub(crate) fn into_physical_parts_for_test(
        self,
    ) -> (
        super::super::operation_effect::VerifiedLoopOperationEffectProductV1,
        VerifiedLoopSemanticContextV1,
        VerifiedLoopContinuationContractV1,
        VerifiedGenericG0TailCapabilityV1,
        NumericTarget,
    ) {
        let Self {
            operation_effect,
            after,
            context,
            target,
        } = self;
        let (after_binding, tail) = after.into_physical_parts();
        let continuation = VerifiedLoopContinuationContractV1::from_after(
            operation_effect.core().owner(),
            after_binding,
        );
        (operation_effect, context, continuation, tail, target)
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
    let context = VerifiedLoopSemanticContextV1::from_parts(
        source_brand.owner(),
        source_brand.origin(),
        source_brand.source_kind(),
        source_brand.root_site().clone(),
        source_brand.frame(),
        window_lease.scope_region(),
    );

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
        child_loop,
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
    let operation_effect = issue_generic_g0_operation_effect_v1(
        core,
        &root_loop,
        &child_loop,
        child_anchor,
        &outer_condition,
        &inner_condition,
        &outer_update,
        &inner_update,
        &tail,
    )
    .map_err(GenericG0RecipeProducerRejectV1::OperationEffect)?;
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
        operation_effect,
        after,
        context,
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
