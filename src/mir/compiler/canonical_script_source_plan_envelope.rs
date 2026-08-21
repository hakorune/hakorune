//! Compiler-side source-plan envelope for the canonical Script no-A boundary.
//!
//! This is a transport/integrity owner only. It co-seals the existing parser
//! handoff with the already-classified source-plan relation, then moves once
//! into the explicit no-A discard boundary. It issues no Script meaning.

use super::canonical_script_source_a_input::{
    CanonicalScriptSourceAInputHandoffV1, CanonicalScriptSourceAInputTransportV1,
};
use super::normal_source_plan::{SealedNormalScalarRootV1, SealedNormalSourcePlanV1};

#[derive(Debug)]
pub(crate) struct CanonicalScriptSourcePlanEnvelopeV1 {
    handoff: CanonicalScriptSourceAInputHandoffV1,
    _relation: CanonicalScriptSourcePlanRelationSealV1,
    _seal: CanonicalScriptSourcePlanEnvelopeSealV1,
}

#[derive(Debug)]
struct CanonicalScriptSourcePlanRelationSealV1;

#[derive(Debug)]
struct CanonicalScriptSourcePlanEnvelopeSealV1;

impl CanonicalScriptSourcePlanEnvelopeV1 {
    pub(crate) fn seal(
        plan: &SealedNormalSourcePlanV1,
        input: CanonicalScriptSourceAInputTransportV1,
    ) -> CanonicalScriptSourceAInputTransportV1 {
        if !matches!(
            plan,
            SealedNormalSourcePlanV1::ScalarRoot(SealedNormalScalarRootV1::Script(_))
        ) {
            return input;
        }
        let handoff = match input {
            CanonicalScriptSourceAInputTransportV1::HandoffReady(handoff) => handoff,
            other => return other,
        };

        match Self::try_seal(plan, handoff) {
            Ok(envelope) => {
                CanonicalScriptSourceAInputTransportV1::SourceEnvelopeReady(envelope)
            }
            Err(EnvelopeRejectV1::SourceAuthorityUnavailable) => {
                CanonicalScriptSourceAInputTransportV1::SourceAuthorityUnavailable
            }
            Err(EnvelopeRejectV1::IntegrityInvalid) => {
                CanonicalScriptSourceAInputTransportV1::IntegrityInvalid
            }
        }
    }

    fn try_seal(
        plan: &SealedNormalSourcePlanV1,
        handoff: CanonicalScriptSourceAInputHandoffV1,
    ) -> Result<Self, EnvelopeRejectV1> {
        let Some(lineage) = plan.parser_lineage() else {
            return Err(EnvelopeRejectV1::SourceAuthorityUnavailable);
        };
        let Some(plan_witness) = plan.parser_invocation_witness() else {
            return Err(EnvelopeRejectV1::SourceAuthorityUnavailable);
        };
        if !plan_witness.same_as(handoff.rows().parser_invocation_witness()) {
            return Err(EnvelopeRejectV1::IntegrityInvalid);
        }
        if lineage.source_identity() != handoff.source_identity()
            || lineage.source_digest() != handoff.source_digest()
            || lineage.grammar_profile() != handoff.profile()
            || lineage.utf8_len() != handoff.utf8_len()
            || lineage.receipt_counts() != handoff.receipt_counts()
        {
            return Err(EnvelopeRejectV1::IntegrityInvalid);
        }
        Ok(Self {
            handoff,
            _relation: CanonicalScriptSourcePlanRelationSealV1,
            _seal: CanonicalScriptSourcePlanEnvelopeSealV1,
        })
    }

    pub(crate) fn discard_before_a_consumer(self) {
        drop(self);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EnvelopeRejectV1 {
    SourceAuthorityUnavailable,
    IntegrityInvalid,
}
