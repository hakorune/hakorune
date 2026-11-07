//! Original static route membership for the homes-aware scalar observer.
//! Qualified and bounded CurrentOwner source laws stay distinct. The claim
//! carries the callee's result kind (R12); there is one claim for I64 and Bool.
use super::{LocalCallResultClassV1, OwnedExprSiteV1, QualifiedStaticCallClaimV1};
use crate::mir::resolved_semantics::ResolvedMethodCallReceiverSourceV1;

#[derive(Debug, Clone, PartialEq, Eq)]
enum StaticScalarCallSourceV1 {
    Qualified(QualifiedStaticCallClaimV1),
    QualifiedTerminal {
        site: OwnedExprSiteV1,
        claim: QualifiedStaticCallClaimV1,
    },
    CurrentOwnerZeroArg(OwnedExprSiteV1),
    CurrentOwnerSource {
        site: OwnedExprSiteV1,
        arity: u32,
        required_i64_arguments: Box<[u32]>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StaticScalarCallClaimV1 {
    source: StaticScalarCallSourceV1,
    result: LocalCallResultClassV1,
}

impl StaticScalarCallClaimV1 {
    pub(crate) fn qualified(claim: QualifiedStaticCallClaimV1) -> Self {
        Self {
            result: claim.result(),
            source: StaticScalarCallSourceV1::Qualified(claim),
        }
    }

    /// The callee's result kind, read from its contract row at the index (R12).
    pub(crate) const fn result(&self) -> LocalCallResultClassV1 {
        self.result
    }

    /// Original canonical caller and exact ReturnValue issue membership.
    /// A finished incoming cohort is still required for physical emission.
    pub(crate) fn qualified_terminal(
        site: OwnedExprSiteV1,
        claim: QualifiedStaticCallClaimV1,
    ) -> Self {
        Self {
            result: claim.result(),
            source: StaticScalarCallSourceV1::QualifiedTerminal { site, claim },
        }
    }

    pub(crate) fn is_qualified_terminal(&self) -> bool {
        matches!(self.source, StaticScalarCallSourceV1::QualifiedTerminal { .. })
    }

    /// Only the package's sealed current-owner result index issues this row.
    /// This grants source membership, never executable entry or packet authority.
    pub(crate) fn current_owner_zeroarg(site: OwnedExprSiteV1, result: LocalCallResultClassV1) -> Self {
        Self {
            result,
            source: StaticScalarCallSourceV1::CurrentOwnerZeroArg(site),
        }
    }

    pub(crate) fn is_current_owner_zeroarg(&self) -> bool {
        matches!(self.source, StaticScalarCallSourceV1::CurrentOwnerZeroArg(_))
    }

    /// The original target/result index issues this source-only input law.
    pub(crate) fn current_owner_source(
        site: OwnedExprSiteV1,
        arity: u32,
        required_i64_arguments: Box<[u32]>,
        result: LocalCallResultClassV1,
    ) -> Self {
        Self {
            result,
            source: StaticScalarCallSourceV1::CurrentOwnerSource {
                site,
                arity,
                required_i64_arguments,
            },
        }
    }

    pub(crate) fn current_owner_source_required_i64_arguments(&self) -> Option<&[u32]> {
        match &self.source {
            StaticScalarCallSourceV1::CurrentOwnerSource {
                required_i64_arguments,
                ..
            } => Some(required_i64_arguments),
            _ => None,
        }
    }

    pub(crate) fn qualified_claim(&self) -> Option<&QualifiedStaticCallClaimV1> {
        match &self.source {
            StaticScalarCallSourceV1::Qualified(claim)
            | StaticScalarCallSourceV1::QualifiedTerminal { claim, .. } => Some(claim),
            StaticScalarCallSourceV1::CurrentOwnerZeroArg(_) => None,
            StaticScalarCallSourceV1::CurrentOwnerSource { .. } => None,
        }
    }

    pub(crate) fn corroborates_source(
        &self,
        site: &OwnedExprSiteV1,
        receiver: ResolvedMethodCallReceiverSourceV1,
        arity: u32,
    ) -> bool {
        match &self.source {
            StaticScalarCallSourceV1::QualifiedTerminal { site: original, .. } => {
                original == site && receiver == ResolvedMethodCallReceiverSourceV1::QualifiedUnbound
            }
            StaticScalarCallSourceV1::Qualified(_) => {
                receiver == ResolvedMethodCallReceiverSourceV1::QualifiedUnbound
            }
            StaticScalarCallSourceV1::CurrentOwnerZeroArg(original) => {
                original == site
                    && receiver == ResolvedMethodCallReceiverSourceV1::CurrentOwner
                    && arity == 0
            }
            StaticScalarCallSourceV1::CurrentOwnerSource {
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
