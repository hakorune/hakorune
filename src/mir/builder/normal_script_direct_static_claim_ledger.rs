//! Operational one-shot claim ledger for Script direct-static rows.
//!
//! The Bundle and Join remain the semantic authorities.  This module only
//! co-seals their already-issued rows for one lowering scope and makes
//! consumption linear.  It does not emit MIR, publish a type, or select a
//! physical route.

use std::collections::{BTreeMap, BTreeSet};

use crate::mir::resolved_semantics::SourceExprSiteV1;

use super::super::normal_script_direct_static_join_handoff::{
    VerifiedScriptDirectStaticJoinHandoffV1, VerifiedScriptDirectStaticJoinRowV1,
};
use super::super::normal_script_direct_static_result_bundle::VerifiedScriptDirectStaticResultBundleV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum ScriptDirectStaticClaimLedgerIssueV1 {
    PartialSourceProducts,
    SourceIdentityMismatch,
    SourceOwnerMismatch,
    CardinalityMismatch,
    BundleSiteMissing(SourceExprSiteV1),
    ForeignJoinSite(SourceExprSiteV1),
    DuplicateJoinSite(SourceExprSiteV1),
    DuplicateClaim(SourceExprSiteV1),
    UnknownClaimState,
    PendingRows(usize),
    InFlightRows(usize),
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) struct ScriptDirectStaticClaimedRowV1 {
    row: VerifiedScriptDirectStaticJoinRowV1,
}

impl ScriptDirectStaticClaimedRowV1 {
    /// Read-only handoff view for the future physical consumer.
    ///
    /// The row is still owned by this non-Clone claim token.  Exposing a
    /// reference does not transfer, duplicate, or re-issue the semantic row.
    pub(in crate::mir::builder) const fn row(&self) -> &VerifiedScriptDirectStaticJoinRowV1 {
        &self.row
    }

    pub(in crate::mir::builder) const fn site(&self) -> &SourceExprSiteV1 {
        self.row.call_site()
    }

    pub(in crate::mir::builder) const fn target(
        &self,
    ) -> &crate::mir::builder::CanonicalSameModuleCallableKeyV1 {
        self.row.target()
    }

    pub(in crate::mir::builder) fn argument_sites(&self) -> &[SourceExprSiteV1] {
        self.row.argument_sites()
    }

    pub(in crate::mir::builder) const fn representation(
        &self,
    ) -> &crate::mir::callable_result_representation::VerifiedCallableResultRepresentationV1 {
        self.row.representation()
    }

    pub(in crate::mir::builder) fn required_callee_i64_arguments(&self) -> &[u32] {
        self.row.required_callee_i64_arguments()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum ScriptDirectStaticClaimTakeV1 {
    Unavailable,
    Absent,
    Claimed(ScriptDirectStaticClaimedRowV1),
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) struct ScriptDirectStaticClaimLedgerV1 {
    pending: BTreeMap<SourceExprSiteV1, VerifiedScriptDirectStaticJoinRowV1>,
    in_flight: BTreeSet<SourceExprSiteV1>,
    completed: BTreeSet<SourceExprSiteV1>,
}

impl ScriptDirectStaticClaimLedgerV1 {
    pub(super) fn issue(
        bundle: Option<VerifiedScriptDirectStaticResultBundleV1>,
        handoff: Option<VerifiedScriptDirectStaticJoinHandoffV1>,
    ) -> Result<Self, ScriptDirectStaticClaimLedgerIssueV1> {
        let (bundle, handoff) = match (bundle, handoff) {
            (None, None) => return Ok(Self::empty()),
            (Some(bundle), Some(handoff)) => (bundle, handoff),
            _ => return Err(ScriptDirectStaticClaimLedgerIssueV1::PartialSourceProducts),
        };
        if bundle.source_identity() != handoff.source_identity() {
            return Err(ScriptDirectStaticClaimLedgerIssueV1::SourceIdentityMismatch);
        }
        if bundle.source_owner() != handoff.source_owner() {
            return Err(ScriptDirectStaticClaimLedgerIssueV1::SourceOwnerMismatch);
        }
        if bundle.len() != handoff.len() {
            return Err(ScriptDirectStaticClaimLedgerIssueV1::CardinalityMismatch);
        }

        let bundle_sites = bundle
            .rows()
            .map(|(site, _)| site.clone())
            .collect::<BTreeSet<_>>();
        let mut pending = BTreeMap::new();
        for row in handoff.into_site_rows() {
            let site = row.call_site().clone();
            if !bundle_sites.contains(&site) {
                return Err(ScriptDirectStaticClaimLedgerIssueV1::ForeignJoinSite(site));
            }
            if pending.insert(site.clone(), row).is_some() {
                return Err(ScriptDirectStaticClaimLedgerIssueV1::DuplicateJoinSite(
                    site,
                ));
            }
        }
        for site in bundle_sites {
            if !pending.contains_key(&site) {
                return Err(ScriptDirectStaticClaimLedgerIssueV1::BundleSiteMissing(
                    site,
                ));
            }
        }
        Ok(Self {
            pending,
            in_flight: BTreeSet::new(),
            completed: BTreeSet::new(),
        })
    }

    pub(super) fn empty() -> Self {
        Self {
            pending: BTreeMap::new(),
            in_flight: BTreeSet::new(),
            completed: BTreeSet::new(),
        }
    }

    pub(super) fn take(
        &mut self,
        site: &SourceExprSiteV1,
    ) -> Result<ScriptDirectStaticClaimTakeV1, ScriptDirectStaticClaimLedgerIssueV1> {
        if self.completed.contains(site) || self.in_flight.contains(site) {
            return Err(ScriptDirectStaticClaimLedgerIssueV1::DuplicateClaim(
                site.clone(),
            ));
        }
        let Some(row) = self.pending.remove(site) else {
            return Ok(ScriptDirectStaticClaimTakeV1::Absent);
        };
        if !self.in_flight.insert(site.clone()) {
            return Err(ScriptDirectStaticClaimLedgerIssueV1::UnknownClaimState);
        }
        Ok(ScriptDirectStaticClaimTakeV1::Claimed(
            ScriptDirectStaticClaimedRowV1 { row },
        ))
    }

    pub(super) fn complete(
        &mut self,
        claimed: ScriptDirectStaticClaimedRowV1,
    ) -> Result<(), ScriptDirectStaticClaimLedgerIssueV1> {
        let site = claimed.site().clone();
        if !self.in_flight.remove(&site) {
            return Err(ScriptDirectStaticClaimLedgerIssueV1::UnknownClaimState);
        }
        if !self.completed.insert(site) {
            return Err(ScriptDirectStaticClaimLedgerIssueV1::UnknownClaimState);
        }
        Ok(())
    }

    pub(super) fn finish(self) -> Result<(), ScriptDirectStaticClaimLedgerIssueV1> {
        if !self.pending.is_empty() {
            return Err(ScriptDirectStaticClaimLedgerIssueV1::PendingRows(
                self.pending.len(),
            ));
        }
        if !self.in_flight.is_empty() {
            return Err(ScriptDirectStaticClaimLedgerIssueV1::InFlightRows(
                self.in_flight.len(),
            ));
        }
        Ok(())
    }

    #[cfg(test)]
    pub(super) fn pending_len(&self) -> usize {
        self.pending.len()
    }

    #[cfg(test)]
    pub(super) fn in_flight_len(&self) -> usize {
        self.in_flight.len()
    }
}

#[cfg(test)]
#[path = "normal_script_direct_static_claim_ledger_tests.rs"]
mod tests;
