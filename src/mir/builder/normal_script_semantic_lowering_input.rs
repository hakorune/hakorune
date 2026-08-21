//! Complete Script lowering input: projection plus retained source products.
//!
//! The semantic source owns these products until the lowering scope is opened.
//! This carrier only transfers ownership; it does not issue Recipe keys or
//! infer any physical continuation.

use super::normal_script_semantic_lowering_projection::VerifiedScriptLoweringProjectionV1;
use super::normal_script_source_continuation::VerifiedScriptSourceContinuationV1;

#[path = "normal_script_semantic_lowering_input/direct_static_claim_input.rs"]
mod direct_static_claim_input;

pub(in crate::mir::builder) use direct_static_claim_input::{
    CanonicalScriptACompleteZeroKindV1, CanonicalScriptANonDirectRowV1,
    CanonicalScriptCNoDirectClaimsV1, ScriptDirectStaticClaimInputV1,
    VerifiedScriptDirectStaticClaimProductsV1,
};

#[derive(Debug)]
pub(super) struct VerifiedScriptSemanticLoweringInputV1 {
    projection: VerifiedScriptLoweringProjectionV1,
    continuation: VerifiedScriptSourceContinuationV1,
    direct_static_claim_input: ScriptDirectStaticClaimInputV1,
}

impl VerifiedScriptSemanticLoweringInputV1 {
    pub(super) fn new(
        projection: VerifiedScriptLoweringProjectionV1,
        continuation: VerifiedScriptSourceContinuationV1,
        direct_static_claim_input: ScriptDirectStaticClaimInputV1,
    ) -> Self {
        Self {
            projection,
            continuation,
            direct_static_claim_input,
        }
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        VerifiedScriptLoweringProjectionV1,
        VerifiedScriptSourceContinuationV1,
        ScriptDirectStaticClaimInputV1,
    ) {
        (
            self.projection,
            self.continuation,
            self.direct_static_claim_input,
        )
    }
}
