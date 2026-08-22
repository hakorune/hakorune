//! Complete source coverage for one parser-owned Script MethodCall walk.

use std::collections::BTreeMap;

use crate::mir::resolved_semantics::SourceExprSiteV1;

/// Source-only route evidence for one observed Script MethodCall.
///
/// `QualifiedUnboundOrdinary` is only an admission to the later lookup step;
/// it is not an A/C candidate or a target decision.  Every other observed
/// route remains visible instead of being discarded by the lookup filter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum VerifiedScriptCallCoverageDispositionV1 {
    QualifiedUnboundOrdinary,
    NonDirect(VerifiedScriptNonDirectCallReasonV1),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum VerifiedScriptNonDirectCallReasonV1 {
    CurrentOwner,
    QualifiedReceiverBound,
    DynamicReceiver,
    ReceiverShapeUnsupported,
    TypeOperation,
    ReservedRoute,
}

/// One complete source coverage row.  It owns only parser-site relations and
/// route evidence; target/result facts stay in the selected lookup row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VerifiedScriptCallCoverageRowV1 {
    site: SourceExprSiteV1,
    receiver_site: SourceExprSiteV1,
    argument_sites: Box<[SourceExprSiteV1]>,
    result_site: SourceExprSiteV1,
    disposition: VerifiedScriptCallCoverageDispositionV1,
}

impl VerifiedScriptCallCoverageRowV1 {
    pub(super) fn new(
        site: SourceExprSiteV1,
        receiver_site: SourceExprSiteV1,
        argument_sites: Box<[SourceExprSiteV1]>,
        result_site: SourceExprSiteV1,
        disposition: VerifiedScriptCallCoverageDispositionV1,
    ) -> Self {
        Self {
            site,
            receiver_site,
            argument_sites,
            result_site,
            disposition,
        }
    }

    pub(crate) const fn site(&self) -> &SourceExprSiteV1 {
        &self.site
    }

    pub(crate) const fn receiver_site(&self) -> &SourceExprSiteV1 {
        &self.receiver_site
    }

    pub(crate) fn argument_sites(&self) -> &[SourceExprSiteV1] {
        &self.argument_sites
    }

    pub(crate) const fn result_site(&self) -> &SourceExprSiteV1 {
        &self.result_site
    }

    pub(crate) const fn disposition(&self) -> VerifiedScriptCallCoverageDispositionV1 {
        self.disposition
    }
}

/// Total source coverage for the one parser invocation observed by the
/// source-package issuer.  `CompleteRows` is never constructed with zero
/// rows; an actual zero-call Script uses `CompleteEmpty`.
#[derive(Debug)]
pub(crate) enum VerifiedScriptCallCoverageV1 {
    CompleteEmpty {
        invocation: crate::parser::ParserInvocationWitnessV1,
    },
    CompleteRows {
        invocation: crate::parser::ParserInvocationWitnessV1,
        rows: BTreeMap<SourceExprSiteV1, VerifiedScriptCallCoverageRowV1>,
    },
}

impl VerifiedScriptCallCoverageV1 {
    pub(super) fn from_rows(
        invocation: crate::parser::ParserInvocationWitnessV1,
        rows: BTreeMap<SourceExprSiteV1, VerifiedScriptCallCoverageRowV1>,
    ) -> Self {
        if rows.is_empty() {
            Self::CompleteEmpty { invocation }
        } else {
            Self::CompleteRows { invocation, rows }
        }
    }

    pub(crate) fn rows(
        &self,
    ) -> Option<&BTreeMap<SourceExprSiteV1, VerifiedScriptCallCoverageRowV1>> {
        match self {
            Self::CompleteEmpty { .. } => None,
            Self::CompleteRows { rows, .. } => Some(rows),
        }
    }

    pub(crate) fn row(&self, site: &SourceExprSiteV1) -> Option<&VerifiedScriptCallCoverageRowV1> {
        self.rows().and_then(|rows| rows.get(site))
    }

    pub(crate) const fn is_empty(&self) -> bool {
        matches!(self, Self::CompleteEmpty { .. })
    }

    pub(crate) fn len(&self) -> usize {
        self.rows().map_or(0, BTreeMap::len)
    }

    pub(crate) fn is_from_invocation(
        &self,
        witness: &crate::parser::ParserInvocationWitnessV1,
    ) -> bool {
        match self {
            Self::CompleteEmpty { invocation } | Self::CompleteRows { invocation, .. } => {
                invocation.same_as(witness)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ScriptDirectStaticCallCoverageIssueV1 {
    ForeignInvocation,
    MissingSite { site: SourceExprSiteV1 },
    ReceiverSiteMismatch { site: SourceExprSiteV1 },
    DuplicateSite { site: SourceExprSiteV1 },
}
