//! Ordered capture receipts derived from one recursive shadow traversal.

use std::collections::{BTreeMap, BTreeSet};

use super::owner_forest::{
    SemanticOwnerForestVerificationErrorV1, UpvarAccessKindV1, UpvarObservationV1,
};
use super::owner_forest_payload::VerifiedSemanticOwnerProductV1;
use super::{BindingRefV1, FunctionOwnerIdV1, OwnedExprSiteV1, SourceExprSiteV1, UpvarRefV1};

/// ABI-neutral, first-demand projection of one child's canonical upvars.
///
/// The forest remains the authority for whether a capture is valid; this row
/// preserves source traversal order for a later capture-materialization owner.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct OrderedCaptureDemandV1 {
    source_binding: BindingRefV1,
    first_demand: SourceExprSiteV1,
    first_access: UpvarAccessKindV1,
}

impl OrderedCaptureDemandV1 {
    pub(crate) const fn new(
        source_binding: BindingRefV1,
        first_demand: SourceExprSiteV1,
        first_access: UpvarAccessKindV1,
    ) -> Self {
        Self {
            source_binding,
            first_demand,
            first_access,
        }
    }

    pub(crate) const fn source_binding(&self) -> BindingRefV1 {
        self.source_binding
    }

    pub(crate) const fn first_demand(&self) -> &SourceExprSiteV1 {
        &self.first_demand
    }

    pub(crate) const fn first_access(&self) -> UpvarAccessKindV1 {
        self.first_access
    }
}

pub(super) fn verify_ordered_capture_demands(
    receipts: &BTreeMap<FunctionOwnerIdV1, Box<[OrderedCaptureDemandV1]>>,
    owners: &BTreeMap<FunctionOwnerIdV1, VerifiedSemanticOwnerProductV1>,
    observations: &[UpvarObservationV1],
    upvars: &[UpvarRefV1],
) -> Result<(), SemanticOwnerForestVerificationErrorV1> {
    for (owner, rows) in receipts {
        if !owners.contains_key(owner) {
            return Err(SemanticOwnerForestVerificationErrorV1::OrderedCaptureOwnerMissing(*owner));
        }
        let actual = rows
            .iter()
            .map(OrderedCaptureDemandV1::source_binding)
            .collect::<BTreeSet<_>>();
        if actual.len() != rows.len() {
            return Err(SemanticOwnerForestVerificationErrorV1::OrderedCaptureSetMismatch(*owner));
        }
        let expected = upvars
            .iter()
            .filter(|upvar| upvar.capturing_owner() == *owner)
            .map(|upvar| upvar.source())
            .collect::<BTreeSet<_>>();
        if actual != expected {
            return Err(SemanticOwnerForestVerificationErrorV1::OrderedCaptureSetMismatch(*owner));
        }
        for row in rows.iter() {
            let upvar = UpvarRefV1::new(*owner, row.source_binding());
            if !observations.iter().any(|observation| {
                observation.upvar() == upvar
                    && observation.site()
                        == &OwnedExprSiteV1::new(*owner, row.first_demand().clone())
                    && observation.access() == row.first_access()
            }) {
                return Err(
                    SemanticOwnerForestVerificationErrorV1::OrderedCaptureFirstDemandMismatch(
                        *owner,
                    ),
                );
            }
        }
    }
    Ok(())
}
