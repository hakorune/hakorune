//! Neutral, consuming handoff from Loop policy to the Recipe producer.
//!
//! This module joins ownership only. It does not construct a Recipe, inspect
//! AST/facts, select a route, or touch physical MIR/PHI/SSA state.

use crate::mir::loop_route_policy::VerifiedLoopPolicyWinnerV1;
use crate::mir::resolved_semantics::{
    FunctionOriginV1, LoopExecutionFrameKeyV1, SemanticOwnerSourceKindV1, SourceStmtSiteV1,
    VerifiedResolvedLoopSourceV1,
};

use super::types::{DirectAccumStructuralShapeV1, LoopStructuralFactsPayloadV1};

#[derive(Debug, Clone, PartialEq, Eq)]
struct LoopStructuralFactsIdentityV1 {
    function_origin: FunctionOriginV1,
    owner_source_kind: SemanticOwnerSourceKindV1,
    site: SourceStmtSiteV1,
}

#[derive(Debug, PartialEq, Eq)]
struct VerifiedLoopStructuralFactsSealV1;

/// Minimal AST-free structural capability for the selected-demand slice.
///
/// The future structural-facts producer may add owned observations behind this
/// capability. S0 deliberately keeps only the identity witness and its seal.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedLoopStructuralFactsV1 {
    identity: LoopStructuralFactsIdentityV1,
    frame_key: LoopExecutionFrameKeyV1,
    payload: LoopStructuralFactsPayloadV1,
    _seal: VerifiedLoopStructuralFactsSealV1,
}

#[derive(Debug, PartialEq, Eq)]
struct VerifiedSelectedLoopRecipeDemandSealV1;

/// One-way handoff accepted by the caller-zero Recipe producer facade.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedSelectedLoopRecipeDemandV1 {
    winner: VerifiedLoopPolicyWinnerV1,
    facts: VerifiedLoopStructuralFactsV1,
    source: VerifiedResolvedLoopSourceV1,
    _seal: VerifiedSelectedLoopRecipeDemandSealV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SelectedLoopDemandRejectV1 {
    ExecutionFrameMismatch,
    FactsSourceIdentityMismatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DirectAccumFactsPayloadRejectV1 {
    NotDirectAccum,
}

impl VerifiedSelectedLoopRecipeDemandV1 {
    pub(crate) fn into_parts(
        self,
    ) -> (
        VerifiedLoopPolicyWinnerV1,
        VerifiedLoopStructuralFactsV1,
        VerifiedResolvedLoopSourceV1,
    ) {
        (self.winner, self.facts, self.source)
    }
}

pub(crate) fn issue_direct_accum_structural_facts_v1(
    observed: super::types::DirectAccumObservedShapeV1,
) -> VerifiedLoopStructuralFactsV1 {
    VerifiedLoopStructuralFactsV1 {
        identity: LoopStructuralFactsIdentityV1 {
            function_origin: observed.function_origin,
            owner_source_kind: observed.owner_source_kind,
            site: observed.loop_site,
        },
        frame_key: observed.frame_key,
        payload: LoopStructuralFactsPayloadV1::DirectAccum(observed.shape),
        _seal: VerifiedLoopStructuralFactsSealV1,
    }
}

impl VerifiedLoopStructuralFactsV1 {
    pub(crate) fn into_direct_accum_v1(
        self,
    ) -> Result<DirectAccumStructuralShapeV1, DirectAccumFactsPayloadRejectV1> {
        match self.payload {
            LoopStructuralFactsPayloadV1::DirectAccum(shape) => Ok(shape),
            LoopStructuralFactsPayloadV1::IdentityOnly => {
                Err(DirectAccumFactsPayloadRejectV1::NotDirectAccum)
            }
        }
    }

    pub(crate) fn direct_accum_shape(&self) -> Option<&DirectAccumStructuralShapeV1> {
        match &self.payload {
            LoopStructuralFactsPayloadV1::IdentityOnly => None,
            LoopStructuralFactsPayloadV1::DirectAccum(shape) => Some(shape),
        }
    }
}

/// Consumes one policy winner, one structural identity witness, and one exact
/// resolved-source capability. No route/family dispatch is possible here.
pub(crate) fn issue_selected_loop_recipe_demand_v1(
    winner: VerifiedLoopPolicyWinnerV1,
    facts: VerifiedLoopStructuralFactsV1,
    source: VerifiedResolvedLoopSourceV1,
) -> Result<VerifiedSelectedLoopRecipeDemandV1, SelectedLoopDemandRejectV1> {
    if !winner.frame_key().matches(&facts.frame_key)
        || !source.frame_key().matches(&facts.frame_key)
    {
        return Err(SelectedLoopDemandRejectV1::ExecutionFrameMismatch);
    }
    if !source.matches_identity(
        facts.identity.function_origin,
        facts.identity.owner_source_kind,
        &facts.identity.site,
    ) {
        return Err(SelectedLoopDemandRejectV1::FactsSourceIdentityMismatch);
    }

    Ok(VerifiedSelectedLoopRecipeDemandV1 {
        winner,
        facts,
        source,
        _seal: VerifiedSelectedLoopRecipeDemandSealV1,
    })
}

#[cfg(test)]
pub(crate) fn verified_loop_structural_facts_for_test(
    function_origin: FunctionOriginV1,
    owner_source_kind: SemanticOwnerSourceKindV1,
    site: SourceStmtSiteV1,
) -> VerifiedLoopStructuralFactsV1 {
    let frame_key = crate::mir::resolved_semantics::loop_execution_frame_key_for_test();
    verified_loop_structural_facts_for_test_with_frame(
        function_origin,
        owner_source_kind,
        site,
        frame_key,
    )
}

#[cfg(test)]
pub(crate) fn verified_loop_structural_facts_for_test_with_frame(
    function_origin: FunctionOriginV1,
    owner_source_kind: SemanticOwnerSourceKindV1,
    site: SourceStmtSiteV1,
    frame_key: LoopExecutionFrameKeyV1,
) -> VerifiedLoopStructuralFactsV1 {
    VerifiedLoopStructuralFactsV1 {
        identity: LoopStructuralFactsIdentityV1 {
            function_origin,
            owner_source_kind,
            site,
        },
        frame_key,
        payload: LoopStructuralFactsPayloadV1::IdentityOnly,
        _seal: VerifiedLoopStructuralFactsSealV1,
    }
}
