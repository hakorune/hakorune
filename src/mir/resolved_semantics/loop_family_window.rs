//! Resolver-owned identity lease for the common Loop-family window.
//!
//! This is a source-brand prerequisite for the later admission assembler. It
//! owns no AST, forest, policy mode, coverage, Recipe, or lowering identity.
//! The wrapped `VerifiedResolvedLoopSourceV1` is the sole source identity
//! authority and keeps the lease non-`Clone`/non-`Copy`.

use super::loop_region::{ResolvedLoopRegionLookupErrorV1, VerifiedResolvedLoopSourceV1};
use super::source_site::SourceStmtSiteV1;
use super::VerifiedResolvedFunctionV1;
use super::{FunctionOriginV1, FunctionOwnerIdV1, LoopExecutionFrameKeyV1};

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum LoopFamilyWindowLeaseIssueV1 {
    Source(ResolvedLoopRegionLookupErrorV1),
}

/// One resolver-issued identity brand shared by the future five-row window.
///
/// This type intentionally has no public constructor and does not implement
/// `Clone` or `Copy`. A caller must consume the exact source lookup issued by
/// `VerifiedResolvedFunctionV1`; loose owner/site/frame tuples are not enough.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedLoopFamilyWindowLeaseV1 {
    owner: FunctionOwnerIdV1,
    source: VerifiedResolvedLoopSourceV1,
    _seal: VerifiedLoopFamilyWindowLeaseSealV1,
}

#[derive(Debug, PartialEq, Eq)]
struct VerifiedLoopFamilyWindowLeaseSealV1;

impl VerifiedResolvedFunctionV1 {
    /// Issue the common-window identity brand from one exact resolver lookup.
    ///
    /// Mode and source-window coverage belong to policy observation rows and
    /// are deliberately not inferred or stored here.
    pub(crate) fn issue_loop_family_window_lease_v1(
        &self,
        site: &SourceStmtSiteV1,
    ) -> Result<VerifiedLoopFamilyWindowLeaseV1, LoopFamilyWindowLeaseIssueV1> {
        let source = self
            .resolved_loop_source(site)
            .map_err(LoopFamilyWindowLeaseIssueV1::Source)?;
        Ok(VerifiedLoopFamilyWindowLeaseV1 {
            owner: self.owner(),
            source,
            _seal: VerifiedLoopFamilyWindowLeaseSealV1,
        })
    }
}

impl VerifiedLoopFamilyWindowLeaseV1 {
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) const fn function_origin(&self) -> FunctionOriginV1 {
        self.source.function_origin()
    }

    pub(crate) const fn source_kind(&self) -> super::SemanticOwnerSourceKindV1 {
        self.source.source_kind()
    }

    pub(crate) fn site(&self) -> &SourceStmtSiteV1 {
        self.source.site()
    }

    pub(crate) fn frame(&self) -> LoopExecutionFrameKeyV1 {
        self.source.frame_key()
    }
}
