//! Passive owner-scoped semantic arena schema.
//!
//! See `README.md` before adding resolver or consumer connections.

// SA0 is intentionally disconnected. Remove these scoped allowances as SA1
// gives the schema its first shadow-only producer/consumer.
#![allow(dead_code, unused_imports)]

mod ids;
mod product;
mod records;
mod source_site;

pub use ids::{BindingRefV1, FunctionOwnerIdV1, RegionId, ScopeId};
pub use product::VerifiedResolvedFunctionV1;
pub use records::{
    BindingKindV1, BindingOriginV1, RegionKindV1, RegionOriginV1, ResolvedAssignmentTargetV1,
    ResolvedBindingRecordV1, ResolvedControlExitV1, ResolvedRegionRecordV1, ResolvedScopeRecordV1,
    ScopeKindV1, ScopeOriginV1, SyntheticBindingKindV1,
};
pub use source_site::{
    FunctionOriginV1, SourceBindingSiteV1, SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1,
    SourceStmtSiteV1,
};

#[cfg(test)]
mod tests;
