//! Detached multi-site exit claims for the strict draft-seal seam.
//!
//! This module is intentionally smaller than the live DraftSeal owner. It
//! consumes the already site-keyed Completion witness and produces a
//! canonical claim set; the detached projection consumes that set on a copied
//! function image only. It never writes to a live Builder/session or creates a
//! second Completion/Return authority. The fresh physical session will
//! consume the same set in a later slice.

use crate::mir::resolved_semantics::SourceStmtSiteV1;
use crate::mir::{MirType, ValueId};

use super::super::completion_consumption::ExplicitReturnWitnessV1;
use super::{
    FunctionDraftSealPreparationErrorV1, FunctionDraftSealProjectionErrorV1,
    FunctionDraftSealProjectionV1, PreparedFunctionExitPlanV1, PreparedFunctionExitV1,
    PreparedFunctionResultV1, PreparedFunctionSignatureV1, ReadyFunctionCompletionV1,
    ReadyFunctionDraftSealV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder::resolved_lowering) enum MultiSiteExitPreparationErrorV1 {
    NoExplicitReturnClaims,
    ExplicitReturnClaimCountNotTwo { actual: usize },
    ExplicitReturnUnitClaim,
}

/// The exit vocabulary consumed by one DraftSeal projection.  The single
/// variant preserves the existing path; `ExactTwo` is the bounded selected
/// Dynamic cohort and keeps each source site attached to its physical claim.
#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder::resolved_lowering) enum PreparedFunctionExitSetV1 {
    Single(PreparedFunctionExitV1),
    ExactTwo([DetachedFunctionExitClaimV1; 2]),
}

impl PreparedFunctionExitSetV1 {
    pub(in crate::mir::builder::resolved_lowering) fn single(exit: PreparedFunctionExitV1) -> Self {
        Self::Single(exit)
    }

    pub(in crate::mir::builder::resolved_lowering) fn exact_two(
        claims: [DetachedFunctionExitClaimV1; 2],
    ) -> Self {
        Self::ExactTwo(claims)
    }

    pub(in crate::mir::builder::resolved_lowering) fn block_for_site(
        &self,
        site: &SourceStmtSiteV1,
    ) -> Option<crate::mir::BasicBlockId> {
        match self {
            Self::Single(_) => None,
            Self::ExactTwo(claims) => {
                claims
                    .iter()
                    .find(|claim| claim.site() == site)
                    .map(|claim| match claim.exit() {
                        PreparedFunctionExitV1::ExplicitValue { block, .. }
                        | PreparedFunctionExitV1::ExplicitUnit { block }
                        | PreparedFunctionExitV1::ImplicitUnit { block } => block,
                    })
            }
        }
    }

    pub(in crate::mir::builder::resolved_lowering) fn try_for_each_exit<E>(
        &self,
        mut visit: impl FnMut(PreparedFunctionExitV1) -> Result<(), E>,
    ) -> Result<(), E> {
        match self {
            Self::Single(exit) => visit(*exit),
            Self::ExactTwo(claims) => {
                visit(claims[0].exit())?;
                visit(claims[1].exit())
            }
        }
    }
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

    #[cfg(test)]
    pub(in crate::mir::builder::resolved_lowering) fn from_test(
        site: SourceStmtSiteV1,
        exit: PreparedFunctionExitV1,
    ) -> Self {
        Self { site, exit }
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

    pub(in crate::mir::builder::resolved_lowering) fn into_prepared_exit_set(
        self,
    ) -> Result<PreparedFunctionExitSetV1, MultiSiteExitPreparationErrorV1> {
        self.into_exact_two()
            .map(PreparedFunctionExitSetV1::exact_two)
    }
}

impl ReadyFunctionDraftSealV1 {
    pub(in crate::mir::builder::resolved_lowering) fn prepare_exact_two(
        self,
    ) -> Result<PreparedFunctionExitPlanV1, FunctionDraftSealPreparationErrorV1> {
        let claims = DetachedFunctionExitClaimSetV1::prepare(&self)
            .map_err(FunctionDraftSealPreparationErrorV1::MultiSite)?
            .into_prepared_exit_set()
            .map_err(FunctionDraftSealPreparationErrorV1::MultiSite)?;
        Ok(PreparedFunctionExitPlanV1 {
            completion: self.completion,
            exit: claims,
        })
    }
}

impl PreparedFunctionExitPlanV1 {
    pub(in crate::mir::builder::resolved_lowering) fn into_parts(
        self,
    ) -> (ReadyFunctionCompletionV1, PreparedFunctionExitSetV1) {
        (self.completion, self.exit)
    }

    pub(in crate::mir::builder::resolved_lowering) fn exit_block_for_site(
        &self,
        site: &SourceStmtSiteV1,
    ) -> Option<crate::mir::BasicBlockId> {
        self.exit.block_for_site(site)
    }

    #[cfg(test)]
    pub(in crate::mir::builder::resolved_lowering) fn exit(&self) -> PreparedFunctionExitV1 {
        match &self.exit {
            PreparedFunctionExitSetV1::Single(exit) => *exit,
            PreparedFunctionExitSetV1::ExactTwo(_) => {
                panic!("single-site test accessor used for an exact-two exit set")
            }
        }
    }
}

impl FunctionDraftSealProjectionV1 {
    /// Resolve the result/signature relation from the already projected exit
    /// plan. This deliberately does not scan Return instructions or infer from
    /// the last produced ValueId.
    pub(super) fn prepare_signature(
        &self,
    ) -> Result<PreparedFunctionSignatureV1, FunctionDraftSealProjectionErrorV1> {
        let mut values = Vec::new();
        let mut return_type: Option<MirType> = None;
        let mut unit = false;
        self.exit.try_for_each_exit(|exit| {
            match exit {
                PreparedFunctionExitV1::ExplicitValue { value, .. } => {
                    let Some(actual) = self.type_ctx.value_types.get(&value).cloned() else {
                        return Err(FunctionDraftSealProjectionErrorV1::ReturnValueTypeMissing {
                            value,
                        });
                    };
                    if actual == MirType::Unknown {
                        return Err(FunctionDraftSealProjectionErrorV1::UnknownReturnValueType {
                            value,
                        });
                    }
                    if !matches!(
                        actual,
                        MirType::Integer | MirType::Bool | MirType::Float | MirType::Void
                    ) {
                        return Err(
                            FunctionDraftSealProjectionErrorV1::UnsupportedReturnValueType {
                                value,
                                actual,
                            },
                        );
                    }
                    if let Some(expected) = return_type.as_ref() {
                        if expected != &actual {
                            return Err(
                                FunctionDraftSealProjectionErrorV1::ReturnSignatureMismatch {
                                    expected: expected.clone(),
                                    actual,
                                },
                            );
                        }
                    } else {
                        return_type = Some(actual.clone());
                    }
                    values.push(value);
                }
                PreparedFunctionExitV1::ExplicitUnit { .. }
                | PreparedFunctionExitV1::ImplicitUnit { .. } => unit = true,
            }
            Ok(())
        })?;

        let result = if unit {
            if !values.is_empty() {
                return Err(
                    FunctionDraftSealProjectionErrorV1::ReturnSignatureMismatch {
                        expected: MirType::Void,
                        actual: return_type.unwrap_or(MirType::Unknown),
                    },
                );
            }
            PreparedFunctionResultV1::Unit
        } else {
            let return_type =
                return_type.ok_or(FunctionDraftSealProjectionErrorV1::ReturnValueTypeMissing {
                    value: ValueId::new(0),
                })?;
            if values.len() == 1 {
                PreparedFunctionResultV1::ExactOperand {
                    value: values[0],
                    return_type,
                }
            } else {
                PreparedFunctionResultV1::ExactOperands {
                    values: values.into_boxed_slice(),
                    return_type,
                }
            }
        };
        Ok(PreparedFunctionSignatureV1 { result })
    }
}
