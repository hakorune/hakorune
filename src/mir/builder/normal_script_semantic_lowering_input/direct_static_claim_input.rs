//! Required C disposition consumed by one Script lowering scope.
//!
//! Selected Script never represents complete-zero as missing Bundle/Recipe
//! state. C supplies either an opaque source-backed no-claim witness or one
//! complete nonempty downstream product pack.

use super::super::normal_script_direct_static_join_handoff::{
    VerifiedScriptDirectStaticJoinHandoffV1,
    VerifiedScriptDirectStaticRequiredArgumentProofV1,
};
use super::super::normal_script_direct_static_recipe::VerifiedScriptDirectStaticRecipeV1;
use super::super::normal_script_direct_static_result_bundle::VerifiedScriptDirectStaticResultBundleV1;
use super::super::normal_script_direct_static_result_publication_owner::VerifiedScriptDirectStaticResultPublicationOwnerV1;
use crate::mir::resolved_semantics::SourceExprSiteV1;
use crate::mir::source_call_target::VerifiedScriptNonDirectCallReasonV1;

/// Source-only coverage retained for a call that did not enter A's direct
/// candidate arm.  It carries no target, Recipe key, or physical identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) struct CanonicalScriptANonDirectRowV1 {
    site: SourceExprSiteV1,
    receiver_site: SourceExprSiteV1,
    argument_sites: Box<[SourceExprSiteV1]>,
    result_site: SourceExprSiteV1,
    reason: VerifiedScriptNonDirectCallReasonV1,
}

impl CanonicalScriptANonDirectRowV1 {
    pub(in crate::mir::builder) fn from_coverage(
        site: SourceExprSiteV1,
        receiver_site: SourceExprSiteV1,
        argument_sites: Box<[SourceExprSiteV1]>,
        result_site: SourceExprSiteV1,
        reason: VerifiedScriptNonDirectCallReasonV1,
    ) -> Self {
        Self {
            site,
            receiver_site,
            argument_sites,
            result_site,
            reason,
        }
    }

    pub(in crate::mir::builder) fn site(&self) -> &SourceExprSiteV1 {
        &self.site
    }

    #[cfg(test)]
    pub(in crate::mir::builder) const fn reason(&self) -> VerifiedScriptNonDirectCallReasonV1 {
        self.reason
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum CanonicalScriptACompleteZeroKindV1 {
    EmptyScript,
    NoMethodCalls,
    ObservedNonDirect,
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) struct CanonicalScriptCNoDirectClaimsV1 {
    kind: CanonicalScriptACompleteZeroKindV1,
    observed_method_calls: usize,
    non_direct_rows: Box<[CanonicalScriptANonDirectRowV1]>,
    _seal: CanonicalScriptCNoDirectClaimsSealV1,
}

#[derive(Debug, PartialEq, Eq)]
struct CanonicalScriptCNoDirectClaimsSealV1;

impl CanonicalScriptCNoDirectClaimsV1 {
    /// Transport constructor. The sole semantic caller is the private C
    /// issuer; this type does not classify source on its own.
    pub(in crate::mir::builder) fn from_issued_c(
        kind: CanonicalScriptACompleteZeroKindV1,
        observed_method_calls: usize,
        non_direct_rows: Box<[CanonicalScriptANonDirectRowV1]>,
    ) -> Self {
        Self {
            kind,
            observed_method_calls,
            non_direct_rows,
            _seal: CanonicalScriptCNoDirectClaimsSealV1,
        }
    }

    #[cfg(test)]
    pub(in crate::mir::builder) const fn kind(&self) -> CanonicalScriptACompleteZeroKindV1 {
        self.kind
    }

    #[cfg(test)]
    pub(in crate::mir::builder) const fn observed_method_calls(&self) -> usize {
        self.observed_method_calls
    }

    #[cfg(test)]
    pub(in crate::mir::builder) const fn non_direct_count(&self) -> usize {
        self.non_direct_rows.len()
    }
}

#[derive(Debug)]
pub(in crate::mir::builder) struct VerifiedScriptDirectStaticClaimProductsV1 {
    bundle: VerifiedScriptDirectStaticResultBundleV1,
    publication_owner: VerifiedScriptDirectStaticResultPublicationOwnerV1,
    recipe: VerifiedScriptDirectStaticRecipeV1,
    join_handoff: VerifiedScriptDirectStaticJoinHandoffV1,
    required_argument_proof: VerifiedScriptDirectStaticRequiredArgumentProofV1,
}

impl VerifiedScriptDirectStaticClaimProductsV1 {
    pub(in crate::mir::builder) fn from_canonical_c(
        bundle: VerifiedScriptDirectStaticResultBundleV1,
        publication_owner: VerifiedScriptDirectStaticResultPublicationOwnerV1,
        recipe: VerifiedScriptDirectStaticRecipeV1,
        join_handoff: VerifiedScriptDirectStaticJoinHandoffV1,
        required_argument_proof: VerifiedScriptDirectStaticRequiredArgumentProofV1,
    ) -> Self {
        Self {
            bundle,
            publication_owner,
            recipe,
            join_handoff,
            required_argument_proof,
        }
    }

    pub(in crate::mir::builder) fn into_parts(
        self,
    ) -> (
        VerifiedScriptDirectStaticResultBundleV1,
        VerifiedScriptDirectStaticResultPublicationOwnerV1,
        VerifiedScriptDirectStaticRecipeV1,
        VerifiedScriptDirectStaticJoinHandoffV1,
        VerifiedScriptDirectStaticRequiredArgumentProofV1,
    ) {
        (
            self.bundle,
            self.publication_owner,
            self.recipe,
            self.join_handoff,
            self.required_argument_proof,
        )
    }
}

#[derive(Debug)]
pub(in crate::mir::builder) enum ScriptDirectStaticClaimInputV1 {
    CompleteNoDirectStaticClaims(CanonicalScriptCNoDirectClaimsV1),
    DirectStaticClaims(VerifiedScriptDirectStaticClaimProductsV1),
}
