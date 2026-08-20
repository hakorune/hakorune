//! One-shot parser evidence retained by the canonical normal-file front door.
//!
//! The parser postpass is the semantic issuer. This module only co-seals that
//! already-issued product with the front-door profile and source receipt, and
//! provides one move-only handoff into source-plan classification.

use super::{NormalFileSourceReceiptV1, SealedNormalEntryProfileV1};
use crate::parser::postpass_envelope::CompletedParserPostpassV1;

#[derive(Debug)]
pub(crate) struct CanonicalParserSourceHandoffV1 {
    postpass: CompletedParserPostpassV1,
    profile: SealedNormalEntryProfileV1,
    receipt: NormalFileSourceReceiptV1,
    _seal: CanonicalParserSourceHandoffSealV1,
}

#[derive(Debug)]
struct CanonicalParserSourceHandoffSealV1;

impl CanonicalParserSourceHandoffV1 {
    pub(super) fn new(
        postpass: CompletedParserPostpassV1,
        profile: SealedNormalEntryProfileV1,
        receipt: NormalFileSourceReceiptV1,
    ) -> Self {
        Self {
            postpass,
            profile,
            receipt,
            _seal: CanonicalParserSourceHandoffSealV1,
        }
    }

    pub(super) fn ast(&self) -> &crate::ast::ASTNode {
        self.postpass.ast()
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
        CompletedParserPostpassV1,
        SealedNormalEntryProfileV1,
        NormalFileSourceReceiptV1,
    ) {
        (self.postpass, self.profile, self.receipt)
    }
}
