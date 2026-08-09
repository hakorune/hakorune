//! Sole atomic full-table source/Recipe/Dynamic-envelope co-seal.

mod calls;
mod coverage;

#[cfg(test)]
mod tests;

use crate::mir::dynamic_invocation_contract::VerifiedDynamicInvocationEnvelopeCatalogV1;
use crate::mir::loop_recipe_contract::VerifiedLoopRecipeArtifactV2;

use super::{DynamicFullLoopRecipeCandidateV2, DynamicFullLoopRetainedSourceV1};
use calls::{
    verify_dynamic_call_relations_v2, DynamicFullLoopCallRelationRejectV2,
    VerifiedDynamicFullLoopCallRelationsV2,
};
use coverage::{
    verify_complete_claim_coverage_v2, DynamicFullLoopCoverageRejectV2,
    VerifiedDynamicFullLoopClaimCoverageV2,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum DynamicFullLoopSourceRecipeEnvelopeRejectV2 {
    Coverage(DynamicFullLoopCoverageRejectV2),
    Calls(DynamicFullLoopCallRelationRejectV2),
}

/// Bounded source-bound Recipe product.
///
/// This is deliberately not a semantic program: JoinSig/Continuation, local
/// `ch` Home, callable Tail/Completion consumption, and Fault remain later
/// owners.
#[derive(Debug)]
pub(in crate::mir) struct VerifiedDynamicFullLoopSourceRecipeEnvelopeV2<'env, 'decl> {
    source: DynamicFullLoopRetainedSourceV1,
    artifact: VerifiedLoopRecipeArtifactV2,
    coverage: VerifiedDynamicFullLoopClaimCoverageV2,
    calls: VerifiedDynamicFullLoopCallRelationsV2,
    catalog: &'env VerifiedDynamicInvocationEnvelopeCatalogV1<'decl>,
}

impl VerifiedDynamicFullLoopSourceRecipeEnvelopeV2<'_, '_> {
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

    #[cfg(test)]
    fn catalog_len(&self) -> usize {
        self.catalog.len()
    }
}

pub(in crate::mir) fn issue_dynamic_full_loop_source_recipe_envelope_v2<'env, 'decl>(
    candidate: DynamicFullLoopRecipeCandidateV2,
    catalog: &'env VerifiedDynamicInvocationEnvelopeCatalogV1<'decl>,
) -> Result<
    VerifiedDynamicFullLoopSourceRecipeEnvelopeV2<'env, 'decl>,
    DynamicFullLoopSourceRecipeEnvelopeRejectV2,
> {
    let (source, artifact, claims) = candidate.into_parts();
    let coverage = verify_complete_claim_coverage_v2(&source, artifact.recipe(), claims)
        .map_err(DynamicFullLoopSourceRecipeEnvelopeRejectV2::Coverage)?;
    let calls = verify_dynamic_call_relations_v2(&source, artifact.recipe(), catalog)
        .map_err(DynamicFullLoopSourceRecipeEnvelopeRejectV2::Calls)?;
    Ok(VerifiedDynamicFullLoopSourceRecipeEnvelopeV2 {
        source,
        artifact,
        coverage,
        calls,
        catalog,
    })
}
