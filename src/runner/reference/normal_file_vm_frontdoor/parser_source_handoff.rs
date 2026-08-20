//! One-shot parser evidence retained by the canonical normal-file front door.
//!
//! The parser postpass is the semantic issuer. This module only co-seals that
//! already-issued product with the front-door profile and source receipt, and
//! provides one move-only handoff into source-plan classification.

use super::script_source_input::{
    co_seal_script_source_input, CanonicalScriptSourceInputDispositionV1,
};
use super::{NormalFileSourceReceiptV1, SealedNormalEntryProfileV1};
use crate::mir::normal_source_plan::NormalParserCallableSourceHandoffV1;
use crate::parser::callable_parameter_source::CanonicalScriptSourceRowsDispositionV1;
use crate::parser::{NormalParserSourceLineageV1, ParserCallableSourceDispositionV1};

#[derive(Debug)]
pub(crate) struct CanonicalParserSourceHandoffV1 {
    callable_source: NormalParserCallableSourceHandoffV1,
    script_input: CanonicalScriptSourceInputDispositionV1,
    profile: SealedNormalEntryProfileV1,
    receipt: NormalFileSourceReceiptV1,
    _seal: CanonicalParserSourceHandoffSealV1,
}

#[derive(Debug)]
struct CanonicalParserSourceHandoffSealV1;

impl CanonicalParserSourceHandoffV1 {
    pub(super) fn new(
        disposition: ParserCallableSourceDispositionV1,
        script_rows: CanonicalScriptSourceRowsDispositionV1,
        profile: SealedNormalEntryProfileV1,
        receipt: NormalFileSourceReceiptV1,
    ) -> Self {
        let lineage = NormalParserSourceLineageV1::issue(
            receipt.source_identity.clone(),
            receipt.source_digest,
            hakorune_frontend_parser::parser::GrammarProfile::Canonical,
            receipt.utf8_len,
            receipt.read_count,
            receipt.parse_count,
        )
        .expect("sealed normal-file receipt must be one-read/one-parse");
        Self {
            callable_source: NormalParserCallableSourceHandoffV1::new(disposition, lineage),
            script_input: co_seal_script_source_input(script_rows, &profile, &receipt),
            profile,
            receipt,
            _seal: CanonicalParserSourceHandoffSealV1,
        }
    }

    pub(super) fn ast(&self) -> &crate::ast::ASTNode {
        self.callable_source.ast()
    }

    pub(super) fn profile_is_canonical_core(&self) -> bool {
        self.profile.is_canonical_core()
    }

    pub(super) fn receipt(&self) -> &NormalFileSourceReceiptV1 {
        &self.receipt
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        NormalParserCallableSourceHandoffV1,
        CanonicalScriptSourceInputDispositionV1,
        SealedNormalEntryProfileV1,
        NormalFileSourceReceiptV1,
    ) {
        (
            self.callable_source,
            self.script_input,
            self.profile,
            self.receipt,
        )
    }

    pub(super) fn script_input(&self) -> &CanonicalScriptSourceInputDispositionV1 {
        &self.script_input
    }
}
