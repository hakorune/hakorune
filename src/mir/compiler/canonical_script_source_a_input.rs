//! Compiler-owned transport for the parser-backed canonical Script input.
//!
//! This module owns no Script meaning. It only preserves the parser-issued
//! AST-free rows and their front-door witness until a future Source-only A
//! issuer consumes them.

use crate::mir::compiler::canonical_source_identity::CanonicalSourceBytesDigestV1;
use crate::parser::callable_parameter_source::CanonicalScriptSourceRowsV1;
use hakorune_frontend_parser::parser::GrammarProfile;
use super::canonical_script_source_plan_envelope::CanonicalScriptSourcePlanEnvelopeV1;

#[derive(Debug)]
pub(crate) enum CanonicalScriptSourceAInputTransportV1 {
    NotApplicable,
    CompatibilitySource,
    Deferred,
    AdmissionMissing,
    SourceAuthorityUnavailable,
    CohortUnresolved,
    ObservationIncomplete,
    IntegrityInvalid,
    NonCandidate,
    HandoffReady(CanonicalScriptSourceAInputHandoffV1),
    SourceEnvelopeReady(CanonicalScriptSourcePlanEnvelopeV1),
    DiscardedBeforeA,
    MovedToParallelHandoff,
    DispositionTransported,
}

#[derive(Debug)]
pub(crate) struct CanonicalScriptSourceAInputHandoffV1 {
    rows: CanonicalScriptSourceRowsV1,
    source_identity: Box<str>,
    source_digest: CanonicalSourceBytesDigestV1,
    utf8_len: usize,
    read_count: u8,
    parse_count: u8,
    profile: GrammarProfile,
    _seal: CanonicalScriptSourceAInputHandoffSealV1,
}

#[derive(Debug)]
struct CanonicalScriptSourceAInputHandoffSealV1;

impl CanonicalScriptSourceAInputTransportV1 {
    pub(crate) fn from_frontdoor_parts(
        rows: CanonicalScriptSourceRowsV1,
        profile: GrammarProfile,
        source_identity: Box<str>,
        source_digest: CanonicalSourceBytesDigestV1,
        utf8_len: usize,
        read_count: u8,
        parse_count: u8,
    ) -> Self {
        if profile != GrammarProfile::Canonical
            || source_identity.is_empty()
            || read_count != 1
            || parse_count != 1
            || !rows.import_config().is_explicit()
            || !rows.import_config().is_complete()
        {
            return Self::SourceAuthorityUnavailable;
        }
        Self::HandoffReady(CanonicalScriptSourceAInputHandoffV1 {
            rows,
            source_identity,
            source_digest,
            utf8_len,
            read_count,
            parse_count,
            profile,
            _seal: CanonicalScriptSourceAInputHandoffSealV1,
        })
    }

    /// Explicitly close the current no-A boundary. `HandoffConsumed` remains
    /// reserved for a named A consumer; this method never issues that state.
    pub(crate) fn discard_before_a_consumer(self) {
        match self {
            Self::HandoffReady(_) => drop(Self::DiscardedBeforeA),
            Self::NotApplicable
            | Self::CompatibilitySource
            | Self::Deferred
            | Self::AdmissionMissing
            | Self::SourceAuthorityUnavailable
            | Self::CohortUnresolved
            | Self::ObservationIncomplete
            | Self::IntegrityInvalid
            | Self::NonCandidate
            | Self::DiscardedBeforeA
            | Self::MovedToParallelHandoff
            | Self::DispositionTransported => {}
            Self::SourceEnvelopeReady(envelope) => envelope.discard_before_a_consumer(),
        }
    }

    #[cfg(test)]
    pub(crate) fn state_name(&self) -> &'static str {
        match self {
            Self::NotApplicable => "NotApplicable",
            Self::CompatibilitySource => "CompatibilitySource",
            Self::Deferred => "Deferred",
            Self::AdmissionMissing => "AdmissionMissing",
            Self::SourceAuthorityUnavailable => "SourceAuthorityUnavailable",
            Self::CohortUnresolved => "CohortUnresolved",
            Self::ObservationIncomplete => "ObservationIncomplete",
            Self::IntegrityInvalid => "IntegrityInvalid",
            Self::NonCandidate => "NonCandidate",
            Self::HandoffReady(_) => "HandoffReady",
            Self::DiscardedBeforeA => "DiscardedBeforeA",
            Self::SourceEnvelopeReady(_) => "SourceEnvelopeReady",
            Self::MovedToParallelHandoff => "MovedToParallelHandoff",
            Self::DispositionTransported => "DispositionTransported",
        }
    }
}

impl CanonicalScriptSourceAInputHandoffV1 {
    pub(crate) fn rows(&self) -> &CanonicalScriptSourceRowsV1 {
        &self.rows
    }

    pub(crate) fn source_identity(&self) -> &str {
        &self.source_identity
    }

    pub(crate) const fn source_digest(&self) -> CanonicalSourceBytesDigestV1 {
        self.source_digest
    }

    pub(crate) const fn utf8_len(&self) -> usize {
        self.utf8_len
    }

    pub(crate) const fn receipt_counts(&self) -> (u8, u8) {
        (self.read_count, self.parse_count)
    }

    pub(crate) const fn profile(&self) -> GrammarProfile {
        self.profile
    }

    #[cfg(test)]
    pub(crate) fn witness(&self) -> (&str, CanonicalSourceBytesDigestV1, usize, (u8, u8)) {
        (
            &self.source_identity,
            self.source_digest,
            self.utf8_len,
            (self.read_count, self.parse_count),
        )
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_terminal_state_has_a_named_transport_disposition() {
        let states = [
            CanonicalScriptSourceAInputTransportV1::NotApplicable,
            CanonicalScriptSourceAInputTransportV1::CompatibilitySource,
            CanonicalScriptSourceAInputTransportV1::Deferred,
            CanonicalScriptSourceAInputTransportV1::AdmissionMissing,
            CanonicalScriptSourceAInputTransportV1::SourceAuthorityUnavailable,
            CanonicalScriptSourceAInputTransportV1::CohortUnresolved,
            CanonicalScriptSourceAInputTransportV1::ObservationIncomplete,
            CanonicalScriptSourceAInputTransportV1::IntegrityInvalid,
            CanonicalScriptSourceAInputTransportV1::NonCandidate,
            CanonicalScriptSourceAInputTransportV1::DiscardedBeforeA,
            CanonicalScriptSourceAInputTransportV1::MovedToParallelHandoff,
            CanonicalScriptSourceAInputTransportV1::DispositionTransported,
        ];
        assert!(states.iter().all(|state| !state.state_name().is_empty()));
    }

    #[test]
    fn no_a_discard_never_reports_handoff_consumed() {
        let transport = CanonicalScriptSourceAInputTransportV1::NonCandidate;
        assert_eq!(transport.state_name(), "NonCandidate");
        transport.discard_before_a_consumer();
    }
}
