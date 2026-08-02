//! Closed pre-effect evidence for the pure Loop route policy.
//!
//! This DTO is an observation boundary. It does not select a route, inspect
//! the legacy receipt, or expose a recipe/Builder operation.

use super::schema::LoopRouteSourceUnavailableV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopRoutePolicySourceDeclineReasonV1 {
    SuppressedByEarlierCandidate,
    Unavailable(LoopRouteSourceUnavailableV1),
    PreEffectDeclined,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopRouteCandidateFactsV1 {
    SourceAvailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopRoutePolicyBlockReasonV1 {
    GlobalEntryBlocked,
    ReleaseNestedLoopGate,
    PolicyAndTerminalityUnavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopGenericDebtKeyV1 {
    GenericPostEffectDebt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopRoutePolicyEvidenceV1 {
    SourceDeclined(LoopRoutePolicySourceDeclineReasonV1),
    Candidate(LoopRouteCandidateFactsV1),
    PolicyBlocked(LoopRoutePolicyBlockReasonV1),
    GenericDebt(LoopGenericDebtKeyV1),
}

#[cfg(test)]
mod tests {
    use super::{
        LoopGenericDebtKeyV1, LoopRouteCandidateFactsV1, LoopRoutePolicyBlockReasonV1,
        LoopRoutePolicyEvidenceV1, LoopRoutePolicySourceDeclineReasonV1,
    };
    use crate::mir::loop_route_policy::LoopRouteSourceUnavailableV1;

    #[test]
    fn evidence_vocabulary_is_closed_and_round_trips_each_disposition() {
        let evidence = [
            LoopRoutePolicyEvidenceV1::SourceDeclined(
                LoopRoutePolicySourceDeclineReasonV1::SuppressedByEarlierCandidate,
            ),
            LoopRoutePolicyEvidenceV1::SourceDeclined(
                LoopRoutePolicySourceDeclineReasonV1::Unavailable(
                    LoopRouteSourceUnavailableV1::FactsAbsent,
                ),
            ),
            LoopRoutePolicyEvidenceV1::Candidate(LoopRouteCandidateFactsV1::SourceAvailable),
            LoopRoutePolicyEvidenceV1::PolicyBlocked(
                LoopRoutePolicyBlockReasonV1::PolicyAndTerminalityUnavailable,
            ),
            LoopRoutePolicyEvidenceV1::GenericDebt(LoopGenericDebtKeyV1::GenericPostEffectDebt),
        ];

        assert_eq!(evidence.len(), 5);
        assert!(matches!(
            evidence[0],
            LoopRoutePolicyEvidenceV1::SourceDeclined(_)
        ));
        assert!(matches!(
            evidence[2],
            LoopRoutePolicyEvidenceV1::Candidate(_)
        ));
        assert!(matches!(
            evidence[3],
            LoopRoutePolicyEvidenceV1::PolicyBlocked(_)
        ));
        assert!(matches!(
            evidence[4],
            LoopRoutePolicyEvidenceV1::GenericDebt(_)
        ));
    }
}
