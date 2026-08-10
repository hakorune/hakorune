//! Sole atomic full-table source/Recipe/Dynamic-envelope co-seal.

mod calls;
mod coverage;
mod local;
mod semantic_program;

#[cfg(test)]
mod tests;

use crate::mir::loop_recipe_contract::VerifiedLoopRecipeArtifactV2;
use crate::mir::source_call_target::VerifiedSourceBoundDynamicMemberCallV1;

use super::{DynamicFullLoopRecipeCandidateV2, DynamicFullLoopRetainedSourceV1};
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
pub(in crate::mir) use semantic_program::{
    issue_dynamic_carrier_flow_program_v1, issue_dynamic_carrier_ingress_lifecycle_program_v1,
    issue_dynamic_carrier_rebind_transaction_program_v1,
    issue_dynamic_full_loop_semantic_program_v2,
    issue_dynamic_invocation_carrier_lifecycle_program_v1,
    issue_dynamic_operator_carrier_lifecycle_program_v1, DynamicCarrierCurrentDispositionV1,
    DynamicCarrierFlowProgramRejectV1, DynamicCarrierIngressLifecycleProgramRejectV1,
    DynamicCarrierRebindTransactionRejectV1, DynamicFullLoopAfterRefV2,
    DynamicFullLoopFaultCutPointCatalogRefV2, DynamicFullLoopFaultCutPointV2,
    DynamicFullLoopFaultFamilyV2, DynamicFullLoopSemanticProgramRejectV2,
    DynamicInvocationCarrierDestinationRefV1, DynamicInvocationCarrierLifecycleCatalogRefV1,
    DynamicInvocationCarrierLifecycleProgramRejectV1, DynamicInvocationCarrierLifecycleRowRefV1,
    DynamicInvocationCarrierPublicationV1, DynamicOperatorCarrierDestinationRefV1,
    DynamicOperatorCarrierLifecycleCatalogRefV1, DynamicOperatorCarrierLifecycleProgramRejectV1,
    DynamicOperatorCarrierLifecycleRowRefV1, DynamicOperatorCarrierPublicationV1,
    VerifiedDynamicCarrierFlowProgramV1, VerifiedDynamicCarrierIngressLifecycleProgramV1,
    VerifiedDynamicCarrierRebindTransactionProgramV1, VerifiedDynamicFullLoopSemanticProgramV2,
    VerifiedDynamicInvocationCarrierLifecycleProgramV1,
    VerifiedDynamicOperatorCarrierLifecycleProgramV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum DynamicFullLoopSourceRecipeEnvelopeRejectV2 {
    Coverage(DynamicFullLoopCoverageRejectV2),
    Calls(DynamicFullLoopCallRelationRejectV2),
    IterationLocal,
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
}

impl VerifiedDynamicFullLoopSourceRecipeEnvelopeV2 {
    pub(in crate::mir) fn iteration_local(&self) -> DynamicIterationLocalValueRefV2<'_> {
        self.iteration_local.borrow(&self.source)
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
    targets: &[VerifiedSourceBoundDynamicMemberCallV1],
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
    Ok(VerifiedDynamicFullLoopSourceRecipeEnvelopeV2 {
        source,
        artifact,
        coverage,
        calls,
        iteration_local,
    })
}
