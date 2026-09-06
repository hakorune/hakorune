//! Neutral source facts for selected direct-local ordinary `New` arguments.
//!
//! This module deliberately has no package, AST, MIR-value, ABI, or backend
//! dependency.  The source walk issues observations; the package may co-seal
//! them into its existing admission claim.

use super::{BindingRefV1, OwnedExprSiteV1, SourceExprSiteV1};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SelectedNewArgumentKindV1 {
    Integer(i64),
    Bool(bool),
    Local { binding: BindingRefV1 },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SelectedNewArgumentV1 {
    ordinal: u32,
    site: SourceExprSiteV1,
    kind: SelectedNewArgumentKindV1,
}

impl SelectedNewArgumentV1 {
    pub(crate) fn new(ordinal: u32, site: SourceExprSiteV1, kind: SelectedNewArgumentKindV1) -> Self {
        Self { ordinal, site, kind }
    }
    pub(crate) const fn ordinal(&self) -> u32 { self.ordinal }
    pub(crate) fn site(&self) -> &SourceExprSiteV1 { &self.site }
    pub(crate) fn kind(&self) -> &SelectedNewArgumentKindV1 { &self.kind }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SelectedNewArgumentUnavailableV1 {
    SourceMismatch { new_site: OwnedExprSiteV1 },
    ArgumentNotTrivial { new_site: OwnedExprSiteV1, site: SourceExprSiteV1 },
    ArgumentOrdinalOverflow { new_site: OwnedExprSiteV1 },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SelectedNewArgumentObservationV1 {
    new_site: OwnedExprSiteV1,
    arguments: Result<Box<[SelectedNewArgumentV1]>, SelectedNewArgumentUnavailableV1>,
}

impl SelectedNewArgumentObservationV1 {
    pub(crate) fn new(new_site: OwnedExprSiteV1, arguments: Result<Box<[SelectedNewArgumentV1]>, SelectedNewArgumentUnavailableV1>) -> Self {
        Self { new_site, arguments }
    }
    pub(crate) fn new_site(&self) -> &OwnedExprSiteV1 { &self.new_site }
    pub(crate) fn arguments(&self) -> Result<&[SelectedNewArgumentV1], &SelectedNewArgumentUnavailableV1> {
        self.arguments.as_deref()
    }
}
