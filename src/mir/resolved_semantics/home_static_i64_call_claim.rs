//! Original static route membership for the homes-aware I64 observer.
//! Qualified argument laws and the bounded CurrentOwner zeroarg law stay distinct.
use super::{OwnedExprSiteV1, QualifiedStaticCallClaimV1};
use crate::mir::resolved_semantics::ResolvedMethodCallReceiverSourceV1;

#[derive(Debug, Clone, PartialEq, Eq)]
enum StaticI64CallSourceV1 {
    Qualified(QualifiedStaticCallClaimV1),
    CurrentOwnerZeroArg(OwnedExprSiteV1),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StaticI64CallClaimV1 {
    source: StaticI64CallSourceV1,
}

impl StaticI64CallClaimV1 {
    pub(crate) fn qualified(claim: QualifiedStaticCallClaimV1) -> Self {
        Self {
            source: StaticI64CallSourceV1::Qualified(claim),
        }
    }

    /// Only the package's sealed current-owner result index issues this row.
    /// This grants source membership, never executable entry or packet authority.
    pub(crate) fn current_owner_zeroarg(site: OwnedExprSiteV1) -> Self {
        Self {
            source: StaticI64CallSourceV1::CurrentOwnerZeroArg(site),
        }
    }

    pub(crate) fn is_current_owner_zeroarg(&self) -> bool {
        matches!(self.source, StaticI64CallSourceV1::CurrentOwnerZeroArg(_))
    }

    pub(crate) fn qualified_claim(&self) -> Option<&QualifiedStaticCallClaimV1> {
        match &self.source {
            StaticI64CallSourceV1::Qualified(claim) => Some(claim),
            StaticI64CallSourceV1::CurrentOwnerZeroArg(_) => None,
        }
    }

    pub(crate) fn corroborates_source(
        &self,
        site: &OwnedExprSiteV1,
        receiver: ResolvedMethodCallReceiverSourceV1,
        arity: u32,
    ) -> bool {
        match &self.source {
            StaticI64CallSourceV1::Qualified(_) => {
                receiver == ResolvedMethodCallReceiverSourceV1::QualifiedUnbound
            }
            StaticI64CallSourceV1::CurrentOwnerZeroArg(original) => {
                original == site
                    && receiver == ResolvedMethodCallReceiverSourceV1::CurrentOwner
                    && arity == 0
            }
        }
    }
}
