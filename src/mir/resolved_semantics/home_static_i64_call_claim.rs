//! Original static route membership for the homes-aware I64 observer.
//! Qualified and bounded CurrentOwner source laws stay distinct.
use super::{OwnedExprSiteV1, QualifiedStaticCallClaimV1};
use crate::mir::resolved_semantics::ResolvedMethodCallReceiverSourceV1;

#[derive(Debug, Clone, PartialEq, Eq)]
enum StaticI64CallSourceV1 {
    Qualified(QualifiedStaticCallClaimV1),
    CurrentOwnerZeroArg(OwnedExprSiteV1),
    CurrentOwnerSource {
        site: OwnedExprSiteV1,
        arity: u32,
        required_i64_arguments: Box<[u32]>,
    },
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

    /// The original target/result index issues this source-only input law.
    pub(crate) fn current_owner_source(
        site: OwnedExprSiteV1,
        arity: u32,
        required_i64_arguments: Box<[u32]>,
    ) -> Self {
        Self {
            source: StaticI64CallSourceV1::CurrentOwnerSource {
                site,
                arity,
                required_i64_arguments,
            },
        }
    }

    pub(crate) fn current_owner_source_required_i64_arguments(&self) -> Option<&[u32]> {
        match &self.source {
            StaticI64CallSourceV1::CurrentOwnerSource {
                required_i64_arguments,
                ..
            } => Some(required_i64_arguments),
            _ => None,
        }
    }

    pub(crate) fn qualified_claim(&self) -> Option<&QualifiedStaticCallClaimV1> {
        match &self.source {
            StaticI64CallSourceV1::Qualified(claim) => Some(claim),
            StaticI64CallSourceV1::CurrentOwnerZeroArg(_) => None,
            StaticI64CallSourceV1::CurrentOwnerSource { .. } => None,
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
            StaticI64CallSourceV1::CurrentOwnerSource {
                site: original,
                arity: original_arity,
                ..
            } => {
                original == site
                    && receiver == ResolvedMethodCallReceiverSourceV1::CurrentOwner
                    && arity == *original_arity
            }
        }
    }
}
