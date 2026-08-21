//! Front-door co-seal for parser-owned pure-Script input rows.
//!
//! This module adds file identity/profile/receipt evidence to the parser
//! rows. It does not inspect AST, resolve names, or issue A/Recipe/physical
//! meaning.

use super::{NormalFileSourceReceiptV1, SealedNormalEntryProfileV1};
use crate::mir::{
    CanonicalScriptSourceAInputTransportV1, CanonicalSourceBytesDigestV1,
};
use crate::parser::callable_parameter_source::{
    CanonicalScriptSourceRowsDispositionV1, CanonicalScriptSourceRowsV1,
};

#[derive(Debug)]
pub(crate) enum CanonicalScriptSourceInputDispositionV1 {
    NotApplicable,
    CompatibilitySource,
    Deferred,
    AdmissionMissing,
    SourceAuthorityUnavailable,
    CohortUnresolved,
    ObservationIncomplete,
    IntegrityInvalid,
    NonCandidate,
    HandoffReady(CanonicalScriptSourceInputHandoffV1),
    MovedToParallelHandoff,
    DispositionTransported,
}

impl CanonicalScriptSourceInputDispositionV1 {
    /// The legacy canonical-core request has no A consumer yet.  This
    /// explicit terminal consumes the transport without interpreting it;
    /// it is deliberately not compiler `HandoffConsumed` and must be replaced by the
    /// future A input owner before any row can affect compilation.
    pub(super) fn discard_before_a_consumer(self) {
        match self {
            Self::NotApplicable
            | Self::CompatibilitySource
            | Self::Deferred
            | Self::AdmissionMissing
            | Self::SourceAuthorityUnavailable
            | Self::CohortUnresolved
            | Self::ObservationIncomplete
            | Self::IntegrityInvalid
            | Self::NonCandidate
            | Self::HandoffReady(_)
            | Self::MovedToParallelHandoff
            | Self::DispositionTransported => {}
        }
    }

    pub(super) fn into_compiler_transport(
        self,
    ) -> CanonicalScriptSourceAInputTransportV1 {
        match self {
            Self::NotApplicable => CanonicalScriptSourceAInputTransportV1::NotApplicable,
            Self::CompatibilitySource => {
                CanonicalScriptSourceAInputTransportV1::CompatibilitySource
            }
            Self::Deferred => CanonicalScriptSourceAInputTransportV1::Deferred,
            Self::AdmissionMissing => CanonicalScriptSourceAInputTransportV1::AdmissionMissing,
            Self::SourceAuthorityUnavailable => {
                CanonicalScriptSourceAInputTransportV1::SourceAuthorityUnavailable
            }
            Self::CohortUnresolved => CanonicalScriptSourceAInputTransportV1::CohortUnresolved,
            Self::ObservationIncomplete => {
                CanonicalScriptSourceAInputTransportV1::ObservationIncomplete
            }
            Self::IntegrityInvalid => CanonicalScriptSourceAInputTransportV1::IntegrityInvalid,
            Self::NonCandidate => CanonicalScriptSourceAInputTransportV1::NonCandidate,
            Self::HandoffReady(handoff) => handoff.into_compiler_transport(),
            Self::MovedToParallelHandoff => {
                CanonicalScriptSourceAInputTransportV1::MovedToParallelHandoff
            }
            Self::DispositionTransported => {
                CanonicalScriptSourceAInputTransportV1::DispositionTransported
            }
        }
    }
}

/// One-shot parser/front-door source input. The rows are already AST-free;
/// this owner only co-seals the existing file identity and receipt.
#[derive(Debug)]
pub(crate) struct CanonicalScriptSourceInputHandoffV1 {
    rows: CanonicalScriptSourceRowsV1,
    source_identity: Box<str>,
    source_digest: CanonicalSourceBytesDigestV1,
    utf8_len: usize,
    read_count: u8,
    parse_count: u8,
    _seal: CanonicalScriptSourceInputHandoffSealV1,
}

#[derive(Debug)]
struct CanonicalScriptSourceInputHandoffSealV1;

impl CanonicalScriptSourceInputHandoffV1 {
    fn issue(
        rows: CanonicalScriptSourceRowsV1,
        profile: &SealedNormalEntryProfileV1,
        receipt: &NormalFileSourceReceiptV1,
    ) -> CanonicalScriptSourceInputDispositionV1 {
        if !profile.is_canonical_core()
            || receipt.read_count != 1
            || receipt.parse_count != 1
            || !rows.import_config().is_explicit()
            || !rows.import_config().is_complete()
        {
            return CanonicalScriptSourceInputDispositionV1::SourceAuthorityUnavailable;
        }
        CanonicalScriptSourceInputDispositionV1::HandoffReady(Self {
            rows,
            source_identity: receipt.source_identity.clone(),
            source_digest: receipt.source_digest,
            utf8_len: receipt.utf8_len,
            read_count: receipt.read_count,
            parse_count: receipt.parse_count,
            _seal: CanonicalScriptSourceInputHandoffSealV1,
        })
    }

    pub(crate) fn rows(&self) -> &CanonicalScriptSourceRowsV1 {
        &self.rows
    }

    pub(crate) fn source_identity(&self) -> &str {
        &self.source_identity
    }

    pub(crate) const fn source_digest(&self) -> CanonicalSourceBytesDigestV1 {
        self.source_digest
    }

    pub(crate) const fn receipt_counts(&self) -> (u8, u8) {
        (self.read_count, self.parse_count)
    }

    pub(crate) const fn utf8_len(&self) -> usize {
        self.utf8_len
    }

    fn into_compiler_transport(self) -> CanonicalScriptSourceAInputTransportV1 {
        let Self {
            rows,
            source_identity,
            source_digest,
            utf8_len,
            read_count,
            parse_count,
            _seal: _,
        } = self;
        CanonicalScriptSourceAInputTransportV1::from_frontdoor_parts(
            rows,
            hakorune_frontend_parser::parser::GrammarProfile::Canonical,
            source_identity,
            source_digest,
            utf8_len,
            read_count,
            parse_count,
        )
    }
}

pub(super) fn co_seal_script_source_input(
    rows: CanonicalScriptSourceRowsDispositionV1,
    profile: &SealedNormalEntryProfileV1,
    receipt: &NormalFileSourceReceiptV1,
) -> CanonicalScriptSourceInputDispositionV1 {
    match rows {
        CanonicalScriptSourceRowsDispositionV1::NotApplicable => {
            CanonicalScriptSourceInputDispositionV1::NotApplicable
        }
        CanonicalScriptSourceRowsDispositionV1::CompatibilitySource => {
            CanonicalScriptSourceInputDispositionV1::CompatibilitySource
        }
        CanonicalScriptSourceRowsDispositionV1::Deferred => {
            CanonicalScriptSourceInputDispositionV1::Deferred
        }
        CanonicalScriptSourceRowsDispositionV1::SourceAuthorityUnavailable => {
            CanonicalScriptSourceInputDispositionV1::SourceAuthorityUnavailable
        }
        CanonicalScriptSourceRowsDispositionV1::AdmissionMissing => {
            CanonicalScriptSourceInputDispositionV1::AdmissionMissing
        }
        CanonicalScriptSourceRowsDispositionV1::CohortUnresolved => {
            CanonicalScriptSourceInputDispositionV1::CohortUnresolved
        }
        CanonicalScriptSourceRowsDispositionV1::ObservationIncomplete => {
            CanonicalScriptSourceInputDispositionV1::ObservationIncomplete
        }
        CanonicalScriptSourceRowsDispositionV1::IntegrityInvalid => {
            CanonicalScriptSourceInputDispositionV1::IntegrityInvalid
        }
        CanonicalScriptSourceRowsDispositionV1::NonCandidate => {
            CanonicalScriptSourceInputDispositionV1::NonCandidate
        }
        CanonicalScriptSourceRowsDispositionV1::HandoffReady(rows) => {
            CanonicalScriptSourceInputHandoffV1::issue(rows, profile, receipt)
        }
        CanonicalScriptSourceRowsDispositionV1::MovedToParallelHandoff => {
            CanonicalScriptSourceInputDispositionV1::MovedToParallelHandoff
        }
        CanonicalScriptSourceRowsDispositionV1::DispositionTransported => {
            CanonicalScriptSourceInputDispositionV1::DispositionTransported
        }
    }
}
