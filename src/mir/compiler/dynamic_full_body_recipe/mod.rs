//! Complete caller-zero V2 Recipe producer for one resolver-backed Dynamic Loop.
//!
//! The producer consumes the source inventory once.  Its non-`Clone` Loop
//! source token moves into the verified artifact; the remaining source facts
//! stay beside that artifact for the later atomic source/envelope co-seal.

mod claims;
mod coseal;
mod mapping;

#[allow(unused_imports)]
pub(in crate::mir) use coseal::{
    issue_dynamic_full_loop_source_recipe_envelope_v2, DynamicFullLoopSourceRecipeEnvelopeRejectV2,
    VerifiedDynamicFullLoopSourceRecipeEnvelopeV2,
};

#[cfg(test)]
mod tests;

use crate::mir::loop_recipe_contract::{
    LoopRecipeProducerIdV1, LoopRecipeProvenanceV1, LoopRecipeV2RejectReason, LoopRecipeVerifierV2,
    VerifiedLoopRecipeArtifactV2,
};
use crate::mir::loop_structural_facts::{
    bind_resolved_loop_root_v1, LoopRootSourceBindingRejectV1,
};
use crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1;
use crate::mir::resolved_semantics::{
    FunctionOwnerIdV1, LoopExecutionFrameKeyV1, ResolvedScopeRegionPairV1,
};

use super::dynamic_full_body_source::{
    DynamicFullBodyBindingRowV1, DynamicFullBodySourceRowV1,
    VerifiedDynamicLoopFullBodySourceInventoryV1,
};
use claims::DynamicFullLoopRecipeClaimsV2;

#[derive(Debug)]
pub(in crate::mir) struct DynamicFullLoopRetainedSourceV1 {
    owner: FunctionOwnerIdV1,
    frame: LoopExecutionFrameKeyV1,
    scope_region: ResolvedScopeRegionPairV1,
    bindings: Box<[DynamicFullBodyBindingRowV1]>,
    rows: Box<[DynamicFullBodySourceRowV1]>,
    completion: VerifiedFunctionCompletionV1,
}

impl DynamicFullLoopRetainedSourceV1 {
    #[cfg(test)]
    fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    #[cfg(test)]
    fn bindings(&self) -> &[DynamicFullBodyBindingRowV1] {
        &self.bindings
    }

    #[cfg(test)]
    fn rows(&self) -> &[DynamicFullBodySourceRowV1] {
        &self.rows
    }

    #[cfg(test)]
    fn completion(&self) -> &VerifiedFunctionCompletionV1 {
        &self.completion
    }

    #[cfg(test)]
    fn frame(&self) -> &LoopExecutionFrameKeyV1 {
        &self.frame
    }

    #[cfg(test)]
    fn scope_region(&self) -> ResolvedScopeRegionPairV1 {
        self.scope_region
    }
}

#[derive(Debug)]
pub(in crate::mir) struct DynamicFullLoopRecipeCandidateV2 {
    source: DynamicFullLoopRetainedSourceV1,
    artifact: VerifiedLoopRecipeArtifactV2,
    claims: DynamicFullLoopRecipeClaimsV2,
}

impl DynamicFullLoopRecipeCandidateV2 {
    #[cfg(test)]
    fn source(&self) -> &DynamicFullLoopRetainedSourceV1 {
        &self.source
    }

    #[cfg(test)]
    fn artifact(&self) -> &VerifiedLoopRecipeArtifactV2 {
        &self.artifact
    }

    fn into_parts(
        self,
    ) -> (
        DynamicFullLoopRetainedSourceV1,
        VerifiedLoopRecipeArtifactV2,
        DynamicFullLoopRecipeClaimsV2,
    ) {
        (self.source, self.artifact, self.claims)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir) enum DynamicFullLoopRecipeProducerRejectV2 {
    SourceRoot(LoopRootSourceBindingRejectV1),
    Recipe(LoopRecipeV2RejectReason),
}

pub(in crate::mir) fn produce_dynamic_full_loop_recipe_v2(
    source: VerifiedDynamicLoopFullBodySourceInventoryV1,
) -> Result<DynamicFullLoopRecipeCandidateV2, DynamicFullLoopRecipeProducerRejectV2> {
    let owner = source.owner();
    let (membership, bindings, rows, completion) = source.into_parts();
    let (resolved_loop_source, frame, scope_region) = membership.into_parts();
    let source_root = bind_resolved_loop_root_v1(resolved_loop_source)
        .map_err(DynamicFullLoopRecipeProducerRejectV2::SourceRoot)?;

    let verified_recipe = LoopRecipeVerifierV2::verify(mapping::complete_dynamic_loop_recipe_v2())
        .map_err(DynamicFullLoopRecipeProducerRejectV2::Recipe)?;
    let source_binding = source_root.into_root_claim_v2(&verified_recipe);
    let artifact = LoopRecipeVerifierV2::bind_verified_artifact(
        LoopRecipeProvenanceV1::new(LoopRecipeProducerIdV1::CallableSingleLoopV1),
        source_binding,
        verified_recipe,
    )
    .map_err(DynamicFullLoopRecipeProducerRejectV2::Recipe)?;

    Ok(DynamicFullLoopRecipeCandidateV2 {
        source: DynamicFullLoopRetainedSourceV1 {
            owner,
            frame,
            scope_region,
            bindings,
            rows,
            completion,
        },
        artifact,
        claims: DynamicFullLoopRecipeClaimsV2::exact(),
    })
}
