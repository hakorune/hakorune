use super::{CanonicalSameModuleCallableKeyV1, VerifiedSameModuleCallableDeclarationCatalogV1};

/// Pure, disconnected selection for bare static-call recovery.
///
/// Earlier call resolvers, argument evaluation, target emission, and retry
/// policy stay outside this product. Candidate cardinality comes only from the
/// complete catalog's static-only index.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum BareStaticRecoveryDecisionV1 {
    Unique(CanonicalSameModuleCallableKeyV1),
    NoRecovery(BareStaticRecoveryNoRecoveryReasonV1),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BareStaticRecoveryNoRecoveryReasonV1 {
    NoCandidate,
    Ambiguous { candidate_count: usize },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum BareStaticRecoveryDecisionErrorV1 {
    ArityOverflow { arity: usize },
}

impl BareStaticRecoveryDecisionV1 {
    pub(crate) fn decide(
        catalog: &VerifiedSameModuleCallableDeclarationCatalogV1,
        source_name: &str,
        arity: usize,
    ) -> Result<Self, BareStaticRecoveryDecisionErrorV1> {
        let checked_arity = u32::try_from(arity)
            .map_err(|_| BareStaticRecoveryDecisionErrorV1::ArityOverflow { arity })?;
        Ok(
            match catalog.static_candidates(source_name, checked_arity) {
                [] => Self::NoRecovery(BareStaticRecoveryNoRecoveryReasonV1::NoCandidate),
                [key] => Self::Unique(key.clone()),
                candidates => Self::NoRecovery(BareStaticRecoveryNoRecoveryReasonV1::Ambiguous {
                    candidate_count: candidates.len(),
                }),
            },
        )
    }
}
