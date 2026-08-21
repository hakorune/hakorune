//! Small ownership boundary for the canonical-core source-plan request.
//!
//! Kept separate from family dispatch so the dispatch owner stays below the
//! 760-line split trigger while the parser-backed carrier is transported.

use super::super::canonical_script_source_a_input::CanonicalScriptSourceAInputTransportV1;
use super::super::canonical_source_identity::CanonicalSourceBytesDigestV1;
use super::super::normal_source_plan::SealedNormalSourcePlanV1;

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
    plan: SealedNormalSourcePlanV1,
    admission: VerifiedCanonicalCoreSourcePlanAdmissionV1,
    receipt: NormalSourcePlanReceiptV1,
    script_input: CanonicalScriptSourceAInputTransportV1,
    _seal: CanonicalCoreSourcePlanCompileRequestSealV1,
}

#[derive(Debug)]
struct CanonicalCoreSourcePlanCompileRequestSealV1;

impl CanonicalCoreSourcePlanCompileRequestV1 {
    pub(crate) fn new(
        plan: SealedNormalSourcePlanV1,
        admission: VerifiedCanonicalCoreSourcePlanAdmissionV1,
        receipt: NormalSourcePlanReceiptV1,
        script_input: CanonicalScriptSourceAInputTransportV1,
    ) -> Self {
        Self {
            plan,
            admission,
            receipt,
            script_input,
            _seal: CanonicalCoreSourcePlanCompileRequestSealV1,
        }
    }

    #[cfg(test)]
    pub(crate) const fn source_digest(&self) -> CanonicalSourceBytesDigestV1 {
        self.receipt.source_digest()
    }

    #[cfg(test)]
    pub(crate) fn script_input_state(&self) -> &'static str {
        self.script_input.state_name()
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        SealedNormalSourcePlanV1,
        VerifiedCanonicalCoreSourcePlanAdmissionV1,
        NormalSourcePlanReceiptV1,
        CanonicalScriptSourceAInputTransportV1,
    ) {
        (self.plan, self.admission, self.receipt, self.script_input)
    }
}
