//! Passive owner-scoped semantic arena schema.
//!
//! See `README.md` before adding resolver or consumer connections.

// SA0 is intentionally disconnected. Remove these scoped allowances as SA1
// gives the schema its first shadow-only producer/consumer.
#![allow(dead_code, unused_imports)]

mod function_view;
mod ids;
mod normalized;
mod product;
mod records;
mod resolver;
mod shadow;
mod source_site;
mod verifier;

pub(crate) use function_view::FunctionSyntaxViewV1;
pub use ids::{BindingRefV1, FunctionOwnerIdV1, RegionId, ScopeId};
pub use normalized::{
    NormalizedAssignmentTargetV1, NormalizedAssignmentV1, NormalizedBindingKeyV1,
    NormalizedBindingRecordV1, NormalizedControlTransferV1, NormalizedDeclarationV1,
    NormalizedExitV1, NormalizedRegionKeyV1, NormalizedRegionRecordV1,
    NormalizedResolvedFunctionGraphV1, NormalizedScopeKeyV1, NormalizedScopeRecordV1,
    NormalizedVariableUseV1,
};
pub use product::VerifiedResolvedFunctionV1;
pub use records::{
    BindingKindV1, BindingOriginV1, RegionKindV1, RegionOriginV1, ResolvedAssignmentTargetV1,
    ResolvedBindingRecordV1, ResolvedControlTransferV1, ResolvedExitOriginV1, ResolvedExitRecordV1,
    ResolvedRegionRecordV1, ResolvedScopeRecordV1, ScopeKindV1, ScopeOriginV1,
    SyntheticBindingKindV1,
};
pub(crate) use resolver::{FunctionSemanticResolverSessionV1, ResolveFunctionErrorV1};
pub use source_site::{
    FunctionOriginV1, OwnedExprSiteV1, ResolvedExitSiteV1, SourceBindingSiteV1, SourceExprSiteV1,
    SourceNodeSiteV1, SourcePathSegmentV1, SourceStmtSiteV1,
};
pub use verifier::ResolvedFunctionVerificationErrorV1;

#[cfg(test)]
mod resolver_tests;
#[cfg(test)]
mod tests;
