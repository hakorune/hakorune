//! Sole atomic full-table source/Recipe/Dynamic-envelope co-seal.

mod a_prime_source;
mod calls;
mod coverage;
mod local;
mod physical_evidence;
mod semantic_program;

#[cfg(test)]
mod tests;

use crate::mir::loop_recipe_contract::VerifiedLoopRecipeArtifactV2;
use crate::mir::source_call_target::VerifiedSourceBoundDynamicMemberCallV1;

use super::{DynamicFullLoopRecipeCandidateV2, DynamicFullLoopRetainedSourceV1};
pub(in crate::mir) use a_prime_source::{
    DynamicAPrimeI64SourceRelationRejectV1, DynamicAPrimeI64SourceRelationViewV1,
};
use calls::{
    verify_dynamic_call_relations_v2, DynamicFullLoopCallRelationRejectV2,
    VerifiedDynamicFullLoopCallRelationsV2,
};
use coverage::{
    verify_complete_claim_coverage_v2, DynamicFullLoopCoverageRejectV2,
    VerifiedDynamicFullLoopClaimCoverageV2,
};
pub(in crate::mir) use local::DynamicIterationLocalValueRefV2;
use local::{verify_iteration_local_relation_v2, DynamicIterationLocalRelationV2};
use physical_evidence::issue_physical_evidence_v2;
use physical_evidence::issue_recipe_relations;
pub(in crate::mir) use physical_evidence::{
    DynamicFullLoopOperationEffectV2, DynamicFullLoopOperationPhysicalRefV2,
    DynamicFullLoopOperationSourceEffectV2, DynamicFullLoopPhysicalEvidenceRejectV2,
    DynamicFullLoopPhysicalItemKindV2, DynamicFullLoopPhysicalItemPlacementV2,
    DynamicFullLoopPhysicalRecipeRelationsViewV2, DynamicLoopPhysicalArmV2,
    DynamicLoopPhysicalBranchControlV2, DynamicLoopPhysicalControlRowV2,
    VerifiedDynamicFullLoopPhysicalEvidenceV2, DYNAMIC_FULL_LOOP_PHYSICAL_ITEM_COUNT_V2,
    DYNAMIC_FULL_LOOP_PHYSICAL_OPERATION_COUNT_V2,
};
pub(in crate::mir) use semantic_program::{
    issue_dynamic_exit_transaction_coseal_i0, issue_dynamic_full_loop_semantic_program_v2,
    issue_dynamic_invocation_carrier_lifecycle_program_v1,
    issue_dynamic_invocation_cleanup_projection_i0, DynamicExitTransactionCoSealRejectV1,
    DynamicFullLoopAfterRefV2, DynamicFullLoopFaultCutPointCatalogRefV2,
    DynamicFullLoopFaultCutPointV2, DynamicFullLoopFaultFamilyV2,
    DynamicCanonicalSessionAuthorityRefV1,
    DynamicFullLoopPhysicalInputRejectV2, DynamicFullLoopPhysicalInputViewV2,
    DynamicFullLoopSemanticProgramRejectV2, DynamicInvocationCarrierDestinationRefV1,
    DynamicInvocationCarrierLifecycleCatalogRefV1,
    DynamicInvocationCarrierLifecycleProgramRejectV1, DynamicInvocationCarrierLifecycleRowRefV1,
    DynamicInvocationCarrierPublicationV1, DynamicInvocationCleanupActionViewV1,
    DynamicInvocationCleanupCurrentDispositionV1, DynamicInvocationCleanupProjectionRejectV1,
    DynamicInvocationCleanupRowKindV1, DynamicInvocationCleanupRowViewV1,
    DynamicLoopPhysicalControlViewV2, VerifiedDynamicExitTransactionCoSealV1,
    VerifiedDynamicFullLoopSemanticProgramV2, VerifiedDynamicInvocationCarrierLifecycleProgramV1,
    VerifiedDynamicInvocationCleanupProjectionV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum DynamicFullLoopSourceRecipeEnvelopeRejectV2 {
    Coverage(DynamicFullLoopCoverageRejectV2),
    Calls(DynamicFullLoopCallRelationRejectV2),
    IterationLocal,
    PhysicalEvidence(DynamicFullLoopPhysicalEvidenceRejectV2),
}

/// Bounded source-bound Recipe product.
///
/// This is deliberately not a semantic program: JoinSig/Continuation, local
/// `ch` Home, callable Tail/Completion consumption, and Fault remain later
/// owners.
#[derive(Debug)]
pub(in crate::mir) struct VerifiedDynamicFullLoopSourceRecipeEnvelopeV2 {
    source: DynamicFullLoopRetainedSourceV1,
    artifact: VerifiedLoopRecipeArtifactV2,
    coverage: VerifiedDynamicFullLoopClaimCoverageV2,
    calls: VerifiedDynamicFullLoopCallRelationsV2,
    iteration_local: DynamicIterationLocalRelationV2,
    physical_evidence: VerifiedDynamicFullLoopPhysicalEvidenceV2,
}

impl VerifiedDynamicFullLoopSourceRecipeEnvelopeV2 {
    pub(in crate::mir) fn with_a_prime_source_relation<R>(
        &self,
        callback: impl for<'program> FnOnce(DynamicAPrimeI64SourceRelationViewV1<'program>) -> R,
    ) -> Result<R, DynamicAPrimeI64SourceRelationRejectV1> {
        a_prime_source::issue(self, callback)
    }

    pub(in crate::mir) fn iteration_local(&self) -> DynamicIterationLocalValueRefV2<'_> {
        self.iteration_local.borrow(&self.source)
    }

    pub(in crate::mir) fn physical_evidence(&self) -> &VerifiedDynamicFullLoopPhysicalEvidenceV2 {
        &self.physical_evidence
    }

    pub(in crate::mir) fn physical_recipe_relations(
        &self,
    ) -> Result<
        DynamicFullLoopPhysicalRecipeRelationsViewV2<'_>,
        DynamicFullLoopPhysicalEvidenceRejectV2,
    > {
        issue_recipe_relations(&self.physical_evidence, self.artifact.recipe(), &self.calls)
    }

    #[cfg(test)]
    fn artifact(&self) -> &VerifiedLoopRecipeArtifactV2 {
        &self.artifact
    }

    #[cfg(test)]
    fn source(&self) -> &DynamicFullLoopRetainedSourceV1 {
        &self.source
    }

    #[cfg(test)]
    fn coverage(&self) -> &VerifiedDynamicFullLoopClaimCoverageV2 {
        &self.coverage
    }

    #[cfg(test)]
    fn calls(&self) -> &VerifiedDynamicFullLoopCallRelationsV2 {
        &self.calls
    }
}

pub(in crate::mir) fn issue_dynamic_full_loop_source_recipe_envelope_v2(
    candidate: DynamicFullLoopRecipeCandidateV2,
    targets: Box<[VerifiedSourceBoundDynamicMemberCallV1]>,
) -> Result<
    VerifiedDynamicFullLoopSourceRecipeEnvelopeV2,
    DynamicFullLoopSourceRecipeEnvelopeRejectV2,
> {
    let (source, artifact, claims) = candidate.into_parts();
    let coverage = verify_complete_claim_coverage_v2(&source, artifact.recipe(), claims)
        .map_err(DynamicFullLoopSourceRecipeEnvelopeRejectV2::Coverage)?;
    let calls = verify_dynamic_call_relations_v2(&source, artifact.recipe(), targets)
        .map_err(DynamicFullLoopSourceRecipeEnvelopeRejectV2::Calls)?;
    let iteration_local = verify_iteration_local_relation_v2(&source, &coverage, &calls)
        .ok_or(DynamicFullLoopSourceRecipeEnvelopeRejectV2::IterationLocal)?;
    let physical_evidence = issue_physical_evidence_v2(&source, &artifact, &coverage, &calls)
        .map_err(DynamicFullLoopSourceRecipeEnvelopeRejectV2::PhysicalEvidence)?;
    Ok(VerifiedDynamicFullLoopSourceRecipeEnvelopeV2 {
        source,
        artifact,
        coverage,
        calls,
        iteration_local,
        physical_evidence,
    })
}
