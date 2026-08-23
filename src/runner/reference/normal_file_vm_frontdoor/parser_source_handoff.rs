//! One-shot parser evidence retained by the canonical normal-file front door.
//!
//! The parser postpass is the semantic issuer. This module only co-seals that
//! already-issued product with the front-door profile and source receipt, and
//! provides one move-only handoff into source-plan classification.

use super::script_source_input::{
    co_seal_script_source_input, CanonicalScriptSourceInputDispositionV1,
};
use super::source_plan_input::PreparedNormalFileSourcePlanRequestV1;
use super::{NormalFileSourceReceiptV1, SealedNormalEntryProfileV1};
use crate::parser::callable_parameter_source::{
    CanonicalScriptSourceRowsDispositionV1, ParsedProgramWithCallableParameterSourceV1,
};

#[derive(Debug)]
pub(crate) struct CanonicalParserSourceHandoffV1 {
    source: ParsedProgramWithCallableParameterSourceV1,
    script_input: CanonicalScriptSourceInputDispositionV1,
    profile: SealedNormalEntryProfileV1,
    receipt: NormalFileSourceReceiptV1,
    _seal: CanonicalParserSourceHandoffSealV1,
}

#[derive(Debug)]
struct CanonicalParserSourceHandoffSealV1;

impl CanonicalParserSourceHandoffV1 {
    pub(super) fn new(
        source: ParsedProgramWithCallableParameterSourceV1,
        script_rows: CanonicalScriptSourceRowsDispositionV1,
        profile: SealedNormalEntryProfileV1,
        receipt: NormalFileSourceReceiptV1,
    ) -> Self {
        Self {
            source,
            script_input: co_seal_script_source_input(script_rows, &profile, &receipt),
            profile,
            receipt,
            _seal: CanonicalParserSourceHandoffSealV1,
        }
    }

    pub(super) fn profile_is_canonical_core(&self) -> bool {
        self.profile.is_canonical_core()
    }

    pub(super) fn receipt(&self) -> &NormalFileSourceReceiptV1 {
        &self.receipt
    }

    pub(super) fn into_source_plan_request(self) -> PreparedNormalFileSourcePlanRequestV1 {
        let Self {
            source,
            script_input,
            profile,
            receipt,
            _seal,
        } = self;
        drop(_seal);
        PreparedNormalFileSourcePlanRequestV1::from_parser_product(
            source,
            script_input,
            profile,
            receipt,
        )
    }

    pub(super) fn discard_at_wrong_route_terminal(self) {
        let Self {
            source,
            script_input,
            profile,
            receipt,
            _seal,
        } = self;
        source.discard_after_source_plan_rejection();
        script_input.discard_before_a_consumer();
        profile.discard_after_source_plan_terminal();
        receipt.discard_after_source_plan_terminal();
        drop(_seal);
    }

    pub(super) fn script_input(&self) -> &CanonicalScriptSourceInputDispositionV1 {
        &self.script_input
    }
}
