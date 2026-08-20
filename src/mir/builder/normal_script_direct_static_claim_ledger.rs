//! Operational one-shot claim ledger for Script direct-static rows.
//!
//! The Bundle and Join remain the semantic authorities.  This module only
//! co-seals their already-issued rows for one lowering scope and makes
//! consumption linear.  It does not emit MIR, publish a type, or select a
//! physical route.

use std::collections::{BTreeMap, BTreeSet};

use crate::mir::resolved_semantics::SourceExprSiteV1;

use super::super::normal_script_direct_static_join_handoff::{
    ScriptDirectStaticRequiredArgumentProofDispositionV1,
    VerifiedScriptDirectStaticJoinHandoffV1, VerifiedScriptDirectStaticJoinRowV1,
    VerifiedScriptDirectStaticRequiredArgumentProofV1,
};
use super::super::normal_script_direct_static_recipe::ScriptDirectStaticRecipeKeyV1;
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
    RequiredArgumentProofMissing,
    RequiredArgumentProofIdentityMismatch,
    RequiredArgumentProofOwnerMismatch,
    RequiredArgumentProofCardinalityMismatch,
    RequiredArgumentProofRowMissing(ScriptDirectStaticRecipeKeyV1),
    RequiredArgumentProofSiteMismatch(SourceExprSiteV1),
    RequiredArgumentProofForeignKey(ScriptDirectStaticRecipeKeyV1),
    DuplicateClaim(SourceExprSiteV1),
    UnknownClaimState,
    RequiredArgumentProofUnconsumed(SourceExprSiteV1),
    PendingRows(usize),
    InFlightRows(usize),
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) struct ScriptDirectStaticClaimedRowV1 {
    row: VerifiedScriptDirectStaticJoinRowV1,
    required_argument_proof: ScriptDirectStaticRequiredArgumentProofDispositionV1,
    required_argument_proof_consumed: bool,
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

    pub(in crate::mir::builder) const fn required_argument_proof(
        &self,
    ) -> &ScriptDirectStaticRequiredArgumentProofDispositionV1 {
        &self.required_argument_proof
    }

    pub(in crate::mir::builder) fn consume_required_argument_proof(
        &mut self,
    ) -> Result<(), &'static str> {
        if self.required_argument_proof_consumed {
            return Err("duplicate required-argument proof consumption");
        }
        match &self.required_argument_proof {
            ScriptDirectStaticRequiredArgumentProofDispositionV1::ExactI64Empty => {
                if !self.row.required_callee_i64_arguments().is_empty() {
                    return Err("required-argument proof is empty for required ordinals");
                }
            }
            ScriptDirectStaticRequiredArgumentProofDispositionV1::ExactI64Required(arguments) => {
                if arguments.len() != self.row.required_callee_i64_arguments().len() {
                    return Err("required-argument proof cardinality mismatch");
                }
                for (expected, argument) in self
                    .row
                    .required_callee_i64_arguments()
                    .iter()
                    .zip(arguments.iter())
                {
                    let Some(expected_site) = self.row.argument_sites().get(*expected as usize)
                    else {
                        return Err("required-argument proof ordinal out of bounds");
                    };
                    if argument.ordinal() != *expected
                        || argument.site() != expected_site
                        || argument.tree().site() != argument.site()
                    {
                        return Err("required-argument proof site mismatch");
                    }
                }
            }
            ScriptDirectStaticRequiredArgumentProofDispositionV1::NonExact(_) => {
                return Err("non-exact result cannot consume required-argument proof");
            }
        }
        self.required_argument_proof_consumed = true;
        Ok(())
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
    pending: BTreeMap<SourceExprSiteV1, PendingClaimRowV1>,
    in_flight: BTreeSet<SourceExprSiteV1>,
    completed: BTreeSet<SourceExprSiteV1>,
}

#[derive(Debug, PartialEq, Eq)]
struct PendingClaimRowV1 {
    row: VerifiedScriptDirectStaticJoinRowV1,
    required_argument_proof: ScriptDirectStaticRequiredArgumentProofDispositionV1,
}

impl ScriptDirectStaticClaimLedgerV1 {
    #[cfg(test)]
    pub(super) fn issue(
        bundle: Option<VerifiedScriptDirectStaticResultBundleV1>,
        handoff: Option<VerifiedScriptDirectStaticJoinHandoffV1>,
    ) -> Result<Self, ScriptDirectStaticClaimLedgerIssueV1> {
        Self::issue_inner(bundle, handoff, None)
    }

    pub(super) fn issue_with_required_argument_proof(
        bundle: Option<VerifiedScriptDirectStaticResultBundleV1>,
        handoff: Option<VerifiedScriptDirectStaticJoinHandoffV1>,
        proof: Option<VerifiedScriptDirectStaticRequiredArgumentProofV1>,
    ) -> Result<Self, ScriptDirectStaticClaimLedgerIssueV1> {
        Self::issue_inner(bundle, handoff, Some(proof))
    }

    fn issue_inner(
        bundle: Option<VerifiedScriptDirectStaticResultBundleV1>,
        handoff: Option<VerifiedScriptDirectStaticJoinHandoffV1>,
        proof: Option<Option<VerifiedScriptDirectStaticRequiredArgumentProofV1>>,
    ) -> Result<Self, ScriptDirectStaticClaimLedgerIssueV1> {
        let (bundle, handoff) = match (bundle, handoff) {
            (None, None) => return Ok(Self::empty()),
            (Some(bundle), Some(handoff)) => (bundle, handoff),
            _ => return Err(ScriptDirectStaticClaimLedgerIssueV1::PartialSourceProducts),
        };
        let proof = match proof {
            Some(Some(proof)) => Some(proof),
            Some(None) => {
                return Err(ScriptDirectStaticClaimLedgerIssueV1::RequiredArgumentProofMissing)
            }
            None => {
                if handoff
                    .rows()
                    .any(|(_, row)| !row.required_callee_i64_arguments().is_empty())
                {
                    return Err(ScriptDirectStaticClaimLedgerIssueV1::RequiredArgumentProofMissing);
                }
                None
            }
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
        if let Some(proof) = proof.as_ref() {
            if proof.source_identity() != handoff.source_identity() {
                return Err(
                    ScriptDirectStaticClaimLedgerIssueV1::RequiredArgumentProofIdentityMismatch,
                );
            }
            if proof.source_owner() != handoff.source_owner() {
                return Err(
                    ScriptDirectStaticClaimLedgerIssueV1::RequiredArgumentProofOwnerMismatch,
                );
            }
            if proof.len() != handoff.len() {
                return Err(
                    ScriptDirectStaticClaimLedgerIssueV1::RequiredArgumentProofCardinalityMismatch,
                );
            }
        }

        let bundle_sites = bundle
            .rows()
            .map(|(site, _)| site.clone())
            .collect::<BTreeSet<_>>();
        let mut pending = BTreeMap::new();
        for (key, row) in handoff.rows() {
            let site = row.call_site().clone();
            if !bundle_sites.contains(&site) {
                return Err(ScriptDirectStaticClaimLedgerIssueV1::ForeignJoinSite(site));
            }
            let required_argument_proof = if let Some(proof) = proof.as_ref() {
                let Some(proof_row) = proof.row(*key) else {
                    return Err(
                        ScriptDirectStaticClaimLedgerIssueV1::RequiredArgumentProofRowMissing(
                            *key,
                        ),
                    );
                };
                if proof_row.call_site() != &site {
                    return Err(
                        ScriptDirectStaticClaimLedgerIssueV1::RequiredArgumentProofSiteMismatch(
                            site,
                        ),
                    );
                }
                proof_row.disposition().clone()
            } else {
                ScriptDirectStaticRequiredArgumentProofDispositionV1::ExactI64Empty
            };
            if pending
                .insert(
                    site.clone(),
                    PendingClaimRowV1 {
                        row: row.clone(),
                        required_argument_proof,
                    },
                )
                .is_some()
            {
                return Err(ScriptDirectStaticClaimLedgerIssueV1::DuplicateJoinSite(
                    site,
                ));
            }
        }
        if let Some(proof) = proof.as_ref() {
            for (key, proof_row) in proof.rows() {
                if !handoff.rows().any(|(row_key, _)| row_key == key) {
                    return Err(
                        ScriptDirectStaticClaimLedgerIssueV1::RequiredArgumentProofForeignKey(
                            *key,
                        ),
                    );
                }
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
        let Some(pending) = self.pending.remove(site) else {
            return Ok(ScriptDirectStaticClaimTakeV1::Absent);
        };
        if !self.in_flight.insert(site.clone()) {
            return Err(ScriptDirectStaticClaimLedgerIssueV1::UnknownClaimState);
        }
        Ok(ScriptDirectStaticClaimTakeV1::Claimed(
            ScriptDirectStaticClaimedRowV1 {
                row: pending.row,
                required_argument_proof: pending.required_argument_proof,
                required_argument_proof_consumed: false,
            },
        ))
    }

    /// Inspect a pending row without changing the linear claim state.
    ///
    /// The transport validates the observed route against this borrowed row
    /// first, then calls `take`.  A failed observation therefore cannot leave
    /// a candidate in `in_flight`, and no state-restoration operation is
    /// needed.
    pub(super) fn peek(
        &self,
        site: &SourceExprSiteV1,
    ) -> Result<Option<&VerifiedScriptDirectStaticJoinRowV1>, ScriptDirectStaticClaimLedgerIssueV1>
    {
        if self.completed.contains(site) || self.in_flight.contains(site) {
            return Err(ScriptDirectStaticClaimLedgerIssueV1::DuplicateClaim(
                site.clone(),
            ));
        }
        Ok(self.pending.get(site).map(|pending| &pending.row))
    }

    pub(super) fn complete(
        &mut self,
        claimed: ScriptDirectStaticClaimedRowV1,
    ) -> Result<(), ScriptDirectStaticClaimLedgerIssueV1> {
        let site = claimed.site().clone();
        if !claimed.required_argument_proof_consumed {
            return Err(ScriptDirectStaticClaimLedgerIssueV1::RequiredArgumentProofUnconsumed(
                site,
            ));
        }
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
