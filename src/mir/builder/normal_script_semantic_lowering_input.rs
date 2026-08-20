//! Complete Script lowering input: projection plus retained source products.
//!
//! The semantic source owns these products until the lowering scope is opened.
//! This carrier only transfers ownership; it does not issue Recipe keys or
//! infer any physical continuation.

use super::normal_script_direct_static_result_bundle::VerifiedScriptDirectStaticResultBundleV1;
use super::normal_script_direct_static_result_publication_owner::VerifiedScriptDirectStaticResultPublicationOwnerV1;
use super::normal_script_semantic_lowering_projection::VerifiedScriptLoweringProjectionV1;
use super::normal_script_source_continuation::VerifiedScriptSourceContinuationV1;

#[derive(Debug)]
pub(super) struct VerifiedScriptSemanticLoweringInputV1 {
    projection: VerifiedScriptLoweringProjectionV1,
    continuation: VerifiedScriptSourceContinuationV1,
    direct_static_result_bundle: Option<VerifiedScriptDirectStaticResultBundleV1>,
    direct_static_result_publication_owner:
        Option<VerifiedScriptDirectStaticResultPublicationOwnerV1>,
}

impl VerifiedScriptSemanticLoweringInputV1 {
    pub(super) fn new(
        projection: VerifiedScriptLoweringProjectionV1,
        continuation: VerifiedScriptSourceContinuationV1,
        direct_static_result_bundle: Option<VerifiedScriptDirectStaticResultBundleV1>,
        direct_static_result_publication_owner: Option<
            VerifiedScriptDirectStaticResultPublicationOwnerV1,
        >,
    ) -> Self {
        Self {
            projection,
            continuation,
            direct_static_result_bundle,
            direct_static_result_publication_owner,
        }
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        VerifiedScriptLoweringProjectionV1,
        VerifiedScriptSourceContinuationV1,
        Option<VerifiedScriptDirectStaticResultBundleV1>,
        Option<VerifiedScriptDirectStaticResultPublicationOwnerV1>,
    ) {
        (
            self.projection,
            self.continuation,
            self.direct_static_result_bundle,
            self.direct_static_result_publication_owner,
        )
    }
}
