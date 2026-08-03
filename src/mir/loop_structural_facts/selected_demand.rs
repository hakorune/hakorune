//! Neutral, consuming handoff from Loop policy to the Recipe producer.
//!
//! This module joins ownership only. It does not construct a Recipe, inspect
//! AST/facts, select a route, or touch physical MIR/PHI/SSA state.

use crate::mir::loop_route_policy::VerifiedLoopPolicyWinnerV1;
use crate::mir::resolved_semantics::{
    FunctionOriginV1, SemanticOwnerSourceKindV1, SourceStmtSiteV1, VerifiedResolvedLoopSourceV1,
};

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
    FactsSourceIdentityMismatch,
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

/// Consumes one policy winner, one structural identity witness, and one exact
/// resolved-source capability. No route/family dispatch is possible here.
pub(crate) fn issue_selected_loop_recipe_demand_v1(
    winner: VerifiedLoopPolicyWinnerV1,
    facts: VerifiedLoopStructuralFactsV1,
    source: VerifiedResolvedLoopSourceV1,
) -> Result<VerifiedSelectedLoopRecipeDemandV1, SelectedLoopDemandRejectV1> {
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
    VerifiedLoopStructuralFactsV1 {
        identity: LoopStructuralFactsIdentityV1 {
            function_origin,
            owner_source_kind,
            site,
        },
        _seal: VerifiedLoopStructuralFactsSealV1,
    }
}
