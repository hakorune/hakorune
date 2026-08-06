//! Neutral, consuming handoff from Loop policy to the Recipe producer.
//!
//! This module joins ownership only. It does not construct a Recipe, inspect
//! AST/facts, select a route, or touch physical MIR/PHI/SSA state.

use crate::mir::loop_route_policy::VerifiedLoopPolicyWinnerV1;
use crate::mir::resolved_semantics::{
    FunctionOriginV1, LoopExecutionFrameKeyV1, SemanticOwnerSourceKindV1, SourceStmtSiteV1,
    VerifiedResolvedLoopSourceV1,
};

use super::direct_accum_exclusivity::{
    issue_direct_accum_disjointness_v1, DirectAccumDisjointnessRejectV1,
    VerifiedDirectAccumDisjointnessV1,
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
    direct_accum_disjointness: Option<VerifiedDirectAccumDisjointnessV1>,
    _seal: VerifiedLoopStructuralFactsSealV1,
}

#[derive(Debug, PartialEq, Eq)]
struct VerifiedSelectedLoopRecipeDemandSealV1;

#[derive(Debug, PartialEq, Eq)]
struct VerifiedDirectAccumSingletonObservationSealV1;

/// One source-owned DirectAccum candidate together with an explicit
/// pre-effect exclusivity proof. This is the only product allowed to mint the
/// later policy admission for the singleton pilot.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedDirectAccumSingletonObservationV1 {
    facts: VerifiedLoopStructuralFactsV1,
    source: VerifiedResolvedLoopSourceV1,
    _seal: VerifiedDirectAccumSingletonObservationSealV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DirectAccumSingletonObservationRejectV1 {
    ExecutionFrameMismatch,
    FactsSourceIdentityMismatch,
    NotDirectAccum,
    MissingDisjointness,
}

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
    MissingDisjointness,
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
) -> Result<VerifiedLoopStructuralFactsV1, DirectAccumDisjointnessRejectV1> {
    let disjointness = issue_direct_accum_disjointness_v1(&observed.shape)?;
    Ok(VerifiedLoopStructuralFactsV1 {
        identity: LoopStructuralFactsIdentityV1 {
            function_origin: observed.function_origin,
            owner_source_kind: observed.owner_source_kind,
            site: observed.loop_site,
        },
        frame_key: observed.frame_key,
        payload: LoopStructuralFactsPayloadV1::DirectAccum(observed.shape),
        direct_accum_disjointness: Some(disjointness),
        _seal: VerifiedLoopStructuralFactsSealV1,
    })
}

impl VerifiedLoopStructuralFactsV1 {
    pub(crate) fn into_direct_accum_v1(
        self,
    ) -> Result<DirectAccumStructuralShapeV1, DirectAccumFactsPayloadRejectV1> {
        if matches!(&self.payload, LoopStructuralFactsPayloadV1::DirectAccum(_))
            && self.direct_accum_disjointness.is_none()
        {
            return Err(DirectAccumFactsPayloadRejectV1::MissingDisjointness);
        }
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

    pub(crate) fn direct_accum_disjointness(&self) -> Option<&VerifiedDirectAccumDisjointnessV1> {
        self.direct_accum_disjointness.as_ref()
    }

    pub(crate) fn into_direct_accum_singleton_observation_v1(
        self,
        source: VerifiedResolvedLoopSourceV1,
    ) -> Result<VerifiedDirectAccumSingletonObservationV1, DirectAccumSingletonObservationRejectV1>
    {
        if !source.frame_key().matches(&self.frame_key) {
            return Err(DirectAccumSingletonObservationRejectV1::ExecutionFrameMismatch);
        }
        if !source.matches_identity(
            self.identity.function_origin,
            self.identity.owner_source_kind,
            &self.identity.site,
        ) {
            return Err(DirectAccumSingletonObservationRejectV1::FactsSourceIdentityMismatch);
        }
        if !matches!(&self.payload, LoopStructuralFactsPayloadV1::DirectAccum(_)) {
            return Err(DirectAccumSingletonObservationRejectV1::NotDirectAccum);
        }
        if self.direct_accum_disjointness.is_none() {
            return Err(DirectAccumSingletonObservationRejectV1::MissingDisjointness);
        }
        Ok(VerifiedDirectAccumSingletonObservationV1 {
            facts: self,
            source,
            _seal: VerifiedDirectAccumSingletonObservationSealV1,
        })
    }
}

impl VerifiedDirectAccumSingletonObservationV1 {
    pub(crate) fn owner(&self) -> crate::mir::resolved_semantics::FunctionOwnerIdV1 {
        self.facts
            .direct_accum_shape()
            .expect("DirectAccum singleton retains its structural shape")
            .condition_binding
            .owner()
    }

    pub(crate) fn frame_key(&self) -> LoopExecutionFrameKeyV1 {
        self.source.frame_key()
    }

    pub(crate) fn matches_source_identity(
        &self,
        function_origin: crate::mir::resolved_semantics::FunctionOriginV1,
        owner_source_kind: crate::mir::resolved_semantics::SemanticOwnerSourceKindV1,
        site: &crate::mir::resolved_semantics::SourceStmtSiteV1,
    ) -> bool {
        self.source
            .matches_identity(function_origin, owner_source_kind, site)
    }

    pub(crate) fn into_parts(
        self,
    ) -> (VerifiedLoopStructuralFactsV1, VerifiedResolvedLoopSourceV1) {
        (self.facts, self.source)
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
        direct_accum_disjointness: None,
        _seal: VerifiedLoopStructuralFactsSealV1,
    }
}
