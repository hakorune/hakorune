//! Closed decline vocabulary for all-route observation provenance.
//!
//! This DTO is an observation boundary. It does not select a route, inspect
//! the legacy receipt, or expose a recipe/Builder operation.

use super::schema::LoopRouteSourceUnavailableV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopRoutePolicySourceDeclineReasonV1 {
    SuppressedByEarlierCandidate,
    Unavailable(LoopRouteSourceUnavailableV1),
    PreEffectDeclined,
    ExcludedByVerifiedSingletonObservation,
}

#[cfg(test)]
mod tests {
    use super::LoopRoutePolicySourceDeclineReasonV1;
    use super::super::schema::LoopRouteSourceUnavailableV1;

    #[test]
    fn decline_vocabulary_is_closed() {
        let reasons = [
            LoopRoutePolicySourceDeclineReasonV1::SuppressedByEarlierCandidate,
            LoopRoutePolicySourceDeclineReasonV1::Unavailable(
                LoopRouteSourceUnavailableV1::FactsAbsent,
            ),
            LoopRoutePolicySourceDeclineReasonV1::PreEffectDeclined,
            LoopRoutePolicySourceDeclineReasonV1::ExcludedByVerifiedSingletonObservation,
        ];

        assert_eq!(reasons.len(), 4);
        assert!(matches!(
            reasons[1],
            LoopRoutePolicySourceDeclineReasonV1::Unavailable(_)
        ));
    }
}
