//! Detached multi-site exit claims for the strict draft-seal seam.
//!
//! This module is intentionally smaller than the live DraftSeal projection.
//! It consumes the already site-keyed Completion witness and produces only a
//! canonical, Builder-free claim set.  It does not write Returns, inspect CFG,
//! or infer a result type.  The fresh physical session will consume this set
//! in a later slice.

use crate::mir::resolved_semantics::SourceStmtSiteV1;

use super::super::completion_consumption::ExplicitReturnWitnessV1;
use super::{PreparedFunctionExitV1, ReadyFunctionDraftSealV1};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder::resolved_lowering) enum MultiSiteExitPreparationErrorV1 {
    NoExplicitReturnClaims,
    ExplicitReturnClaimCountNotTwo { actual: usize },
    ExplicitReturnUnitClaim,
}

/// One source-keyed exit claim prepared without a Builder or physical writer.
/// The claim reuses the existing single-site exit vocabulary; it does not
/// create a second Completion or Return authority.
#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder::resolved_lowering) struct DetachedFunctionExitClaimV1 {
    site: SourceStmtSiteV1,
    exit: PreparedFunctionExitV1,
}

impl DetachedFunctionExitClaimV1 {
    pub(in crate::mir::builder::resolved_lowering) fn site(&self) -> &SourceStmtSiteV1 {
        &self.site
    }

    pub(in crate::mir::builder::resolved_lowering) fn exit(&self) -> PreparedFunctionExitV1 {
        self.exit
    }
}

/// Canonical source-order collection of explicit value exits.  It is
/// deliberately non-Clone and exposes no parts API so a later session can
/// consume it exactly once.
#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder::resolved_lowering) struct DetachedFunctionExitClaimSetV1 {
    claims: Box<[DetachedFunctionExitClaimV1]>,
}

impl DetachedFunctionExitClaimSetV1 {
    pub(in crate::mir::builder::resolved_lowering) fn prepare(
        ready: &ReadyFunctionDraftSealV1,
    ) -> Result<Self, MultiSiteExitPreparationErrorV1> {
        let claims = ready.completion.explicit_claims();
        if claims.is_empty() {
            return Err(MultiSiteExitPreparationErrorV1::NoExplicitReturnClaims);
        }
        if claims.len() != 2 {
            return Err(
                MultiSiteExitPreparationErrorV1::ExplicitReturnClaimCountNotTwo {
                    actual: claims.len(),
                },
            );
        }

        let claims = claims
            .iter()
            .map(|claim| match claim.witness() {
                ExplicitReturnWitnessV1::Value(witness) => Ok(DetachedFunctionExitClaimV1 {
                    site: claim.site().clone(),
                    exit: PreparedFunctionExitV1::ExplicitValue {
                        block: witness.block(),
                        value: witness.value(),
                    },
                }),
                ExplicitReturnWitnessV1::Unit => {
                    Err(MultiSiteExitPreparationErrorV1::ExplicitReturnUnitClaim)
                }
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_boxed_slice();

        Ok(Self { claims })
    }

    pub(in crate::mir::builder::resolved_lowering) fn claims(
        &self,
    ) -> &[DetachedFunctionExitClaimV1] {
        &self.claims
    }

    /// Narrow admission for the bounded selected Dynamic cohort.  The
    /// canonical Completion order is preserved; no sorting, zipping, or
    /// source-name repair is performed here.
    pub(in crate::mir::builder::resolved_lowering) fn into_exact_two(
        self,
    ) -> Result<[DetachedFunctionExitClaimV1; 2], MultiSiteExitPreparationErrorV1> {
        let actual = self.claims.len();
        if actual != 2 {
            return Err(MultiSiteExitPreparationErrorV1::ExplicitReturnClaimCountNotTwo { actual });
        }
        let mut claims = self.claims.into_vec().into_iter();
        let first = claims.next().expect("validated two-site claim set");
        let second = claims.next().expect("validated two-site claim set");
        debug_assert!(claims.next().is_none());
        Ok([first, second])
    }
}
