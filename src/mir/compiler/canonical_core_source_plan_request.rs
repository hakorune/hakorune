//! Small ownership boundary for the canonical-core source-plan request.
//!
//! Kept separate from family dispatch so the dispatch owner stays below the
//! 760-line split trigger while the parser-backed carrier is transported.

use super::super::canonical_script_source_a_input::CanonicalScriptSourceAInputTransportV1;
use super::super::canonical_script_source_plan_envelope::CanonicalScriptSourcePlanEnvelopeV1;
use super::super::canonical_source_identity::CanonicalSourceBytesDigestV1;
use super::super::normal_source_plan::{
    SealedNormalCallableModuleSourceV1, SealedNormalMainSourceV1, SealedNormalScalarRootV1,
    SealedNormalScriptSourceV1, SealedNormalSourcePlanV1,
};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct NormalSourcePlanReceiptV1 {
    source_identity: Box<str>,
    source_digest: CanonicalSourceBytesDigestV1,
    utf8_len: usize,
    read_count: u8,
    parse_count: u8,
    _seal: NormalSourcePlanReceiptSealV1,
}

#[derive(Debug, PartialEq, Eq)]
struct NormalSourcePlanReceiptSealV1;

impl NormalSourcePlanReceiptV1 {
    pub(crate) fn one_read_one_parse(
        source_identity: Box<str>,
        source_digest: CanonicalSourceBytesDigestV1,
        utf8_len: usize,
        read_count: u8,
        parse_count: u8,
    ) -> Self {
        Self {
            source_identity,
            source_digest,
            utf8_len,
            read_count,
            parse_count,
            _seal: NormalSourcePlanReceiptSealV1,
        }
    }

    pub(crate) const fn source_digest(&self) -> CanonicalSourceBytesDigestV1 {
        self.source_digest
    }

    #[cfg(test)]
    pub(crate) fn counts(&self) -> (u8, u8) {
        (self.read_count, self.parse_count)
    }
}

#[derive(Debug)]
pub(crate) struct VerifiedCanonicalCoreSourcePlanAdmissionV1 {
    _seal: VerifiedCanonicalCoreSourcePlanAdmissionSealV1,
}

#[derive(Debug)]
struct VerifiedCanonicalCoreSourcePlanAdmissionSealV1;

impl VerifiedCanonicalCoreSourcePlanAdmissionV1 {
    pub(crate) fn seal_from_frontdoor_profile() -> Self {
        Self {
            _seal: VerifiedCanonicalCoreSourcePlanAdmissionSealV1,
        }
    }
}

#[derive(Debug)]
pub(crate) struct CanonicalCoreSourcePlanCompileRequestV1 {
    dispatch: CanonicalCoreSourcePlanDispatchV1,
    _seal: CanonicalCoreSourcePlanCompileRequestSealV1,
}

#[derive(Debug)]
struct CanonicalCoreSourcePlanCompileRequestSealV1;

impl CanonicalCoreSourcePlanCompileRequestV1 {
    pub(crate) fn new(
        plan: SealedNormalSourcePlanV1,
        admission: VerifiedCanonicalCoreSourcePlanAdmissionV1,
        receipt: NormalSourcePlanReceiptV1,
        transport: CanonicalScriptSourceAInputTransportV1,
    ) -> Self {
        let transport = CanonicalScriptSourcePlanEnvelopeV1::seal(&plan, transport);
        let dispatch = match plan {
            SealedNormalSourcePlanV1::ScalarRoot(SealedNormalScalarRootV1::Main0(source)) => {
                transport.discard_before_a_consumer();
                CanonicalCoreSourcePlanDispatchV1::Main {
                    source,
                    admission,
                    receipt,
                }
            }
            SealedNormalSourcePlanV1::ScalarRoot(SealedNormalScalarRootV1::Script(source)) => {
                let source_input = match transport {
                    CanonicalScriptSourceAInputTransportV1::SourceEnvelopeReady(envelope) => {
                        CanonicalScriptSourcePlanInputV1::SourceEnvelopeReady(envelope)
                    }
                    rejected => CanonicalScriptSourcePlanInputV1::Rejected(rejected),
                };
                CanonicalCoreSourcePlanDispatchV1::Script {
                    source,
                    admission,
                    receipt,
                    source_input,
                }
            }
            SealedNormalSourcePlanV1::CallableModule(source) => {
                transport.discard_before_a_consumer();
                CanonicalCoreSourcePlanDispatchV1::Callable {
                    source,
                    admission,
                    receipt,
                }
            }
        };
        Self {
            dispatch,
            _seal: CanonicalCoreSourcePlanCompileRequestSealV1,
        }
    }

    #[cfg(test)]
    pub(crate) const fn source_digest(&self) -> CanonicalSourceBytesDigestV1 {
        match &self.dispatch {
            CanonicalCoreSourcePlanDispatchV1::Main { receipt, .. }
            | CanonicalCoreSourcePlanDispatchV1::Script { receipt, .. }
            | CanonicalCoreSourcePlanDispatchV1::Callable { receipt, .. } => {
                receipt.source_digest()
            }
        }
    }

    #[cfg(test)]
    pub(crate) fn script_input_state(&self) -> &'static str {
        match &self.dispatch {
            CanonicalCoreSourcePlanDispatchV1::Main { .. }
            | CanonicalCoreSourcePlanDispatchV1::Callable { .. } => "ClosedBeforeDispatch",
            CanonicalCoreSourcePlanDispatchV1::Script { source_input, .. } => {
                source_input.state_name()
            }
        }
    }

    pub(crate) fn into_dispatch(self) -> CanonicalCoreSourcePlanDispatchV1 {
        self.dispatch
    }
}

#[derive(Debug)]
pub(crate) enum CanonicalCoreSourcePlanDispatchV1 {
    Main {
        source: SealedNormalMainSourceV1,
        admission: VerifiedCanonicalCoreSourcePlanAdmissionV1,
        receipt: NormalSourcePlanReceiptV1,
    },
    Script {
        source: SealedNormalScriptSourceV1,
        admission: VerifiedCanonicalCoreSourcePlanAdmissionV1,
        receipt: NormalSourcePlanReceiptV1,
        source_input: CanonicalScriptSourcePlanInputV1,
    },
    Callable {
        source: SealedNormalCallableModuleSourceV1,
        admission: VerifiedCanonicalCoreSourcePlanAdmissionV1,
        receipt: NormalSourcePlanReceiptV1,
    },
}

#[derive(Debug)]
pub(crate) enum CanonicalScriptSourcePlanInputV1 {
    SourceEnvelopeReady(CanonicalScriptSourcePlanEnvelopeV1),
    Rejected(CanonicalScriptSourceAInputTransportV1),
}

impl CanonicalScriptSourcePlanInputV1 {
    #[cfg(test)]
    fn state_name(&self) -> &'static str {
        match self {
            Self::SourceEnvelopeReady(_) => "SourceEnvelopeReady",
            Self::Rejected(transport) => transport.state_name(),
        }
    }

    pub(crate) fn discard_before_a_consumer(self) {
        match self {
            Self::SourceEnvelopeReady(envelope) => envelope.discard_before_a_consumer(),
            Self::Rejected(transport) => transport.discard_before_a_consumer(),
        }
    }
}
