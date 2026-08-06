//! Test-only neutral Generic carrier facts snapshot.
//!
//! This module consumes one sealed resolver provenance product and adds only a
//! mode-neutral structural disposition. It deliberately does not connect to
//! `LoopFacts`, Generic V0/V1 facts, policy, Recipe, Builder, or MIR.

use crate::mir::resolved_semantics::generic_resolved_carrier_provenance::VerifiedResolvedCarrierProvenanceV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ResolvedCarrierDispositionV1 {
    NestedWriteWithPostLoopRead,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedGenericResolvedCarrierFactsV1 {
    provenance: VerifiedResolvedCarrierProvenanceV1,
    disposition: ResolvedCarrierDispositionV1,
    _seal: GenericResolvedCarrierFactsSealV1,
}

#[derive(Debug, PartialEq, Eq)]
struct GenericResolvedCarrierFactsSealV1;

/// Consume exactly one already-verified P1 provenance product. P1 validation
/// proves the role/forest/frame shape; this layer does not re-validate or mint
/// a second source authority.
pub(crate) fn issue_generic_resolved_carrier_facts_v1(
    provenance: VerifiedResolvedCarrierProvenanceV1,
) -> VerifiedGenericResolvedCarrierFactsV1 {
    VerifiedGenericResolvedCarrierFactsV1 {
        provenance,
        disposition: ResolvedCarrierDispositionV1::NestedWriteWithPostLoopRead,
        _seal: GenericResolvedCarrierFactsSealV1,
    }
}

impl VerifiedGenericResolvedCarrierFactsV1 {
    pub(crate) const fn disposition(&self) -> ResolvedCarrierDispositionV1 {
        self.disposition
    }
}
