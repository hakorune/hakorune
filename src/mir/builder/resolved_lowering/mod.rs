//! First closed canonical function-family lowering.
//!
//! The compiler preflight produces the only input accepted here. This module
//! owns exact source traversal and BindingRef-based value publication; legacy
//! statement/expression dispatch is intentionally not reachable from it.

mod callable_module_transaction;
pub(in crate::mir::builder) mod canonical_cfg;
pub(in crate::mir) mod canonical_ssa;
mod completion_consumption;
mod draft_seal;
mod draft_seal_owner;
mod function_draft_entry;
pub(in crate::mir::builder) use draft_seal_owner::SelectedDynamicCandidateMetadataV1;
mod identity;
mod semantic_stack;
mod trivial_ssa;

#[cfg(test)]
pub(in crate::mir) use callable_module_transaction::{
    CallableModuleTransactionErrorV1, VerifiedUnpublishedCallableDraftSetV1,
};

#[cfg(test)]
mod block_expr_tests;
#[cfg(test)]
mod completion_consumption_tests;
#[cfg(test)]
mod completion_draft_seal_tests;
#[cfg(test)]
mod completion_test_support;
#[cfg(test)]
mod normal_function_draft_lowering_tests;
#[cfg(test)]
mod null_tests;
#[cfg(test)]
mod parameter_tests;
#[cfg(test)]
mod return_tests;
#[cfg(test)]
mod semantic_stack_tests;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod void_tests;

use crate::mir::compiler::capability::CanonicalTrivialBindingSsaPlanV1;
use crate::mir::MirFunction;
#[cfg(test)]
use crate::mir::MirModule;

use super::calls::CanonicalFunctionSessionErrorV1;
use super::MirBuilder;
use draft_seal_owner::{FunctionDraftSealStageV1, RejectedFunctionDraftSealV1};
use trivial_ssa::{install_trivial_callable_abi_v1, CanonicalTrivialSsaLowererV1};

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir) enum CanonicalResolvedBuildErrorV1 {
    BuilderContract(String),
    DuplicateFunctionPublication { function_name: String },
}

impl From<String> for CanonicalResolvedBuildErrorV1 {
    fn from(detail: String) -> Self {
        Self::BuilderContract(detail)
    }
}

impl From<CanonicalFunctionSessionErrorV1> for CanonicalResolvedBuildErrorV1 {
    fn from(error: CanonicalFunctionSessionErrorV1) -> Self {
        match error.duplicate_function_name() {
            Some(function_name) => Self::DuplicateFunctionPublication {
                function_name: function_name.to_string(),
            },
            None => Self::BuilderContract(error.to_string()),
        }
    }
}

/// Exact failure boundary for the TX0 retaining draft-lowering terminal.
///
/// The outer stage is issued at the operation that failed; callers never
/// classify a formatted legacy diagnostic to recover it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum NormalFunctionDraftLoweringStageV1 {
    BindingInstall,
    Skeleton,
    BodyLowering,
    DraftSeal(FunctionDraftSealStageV1),
}

#[derive(Debug)]
pub(in crate::mir) enum NormalFunctionDraftLoweringCauseV1 {
    BuilderContract(Box<str>),
    DraftSeal(Box<str>),
}

/// Sealed evidence that the unpublished child session was discarded and its
/// caller context restored before the typed rejection was issued.
#[derive(Debug)]
pub(in crate::mir) struct NormalFunctionDraftBuilderRestorationReceiptV1 {
    session: super::calls::CanonicalFunctionSessionRestorationReceiptV1,
    _seal: NormalFunctionDraftBuilderRestorationReceiptSealV1,
}

#[derive(Debug)]
struct NormalFunctionDraftBuilderRestorationReceiptSealV1;

#[derive(Debug)]
pub(in crate::mir) struct RejectedNormalFunctionDraftLoweringV1 {
    stage: NormalFunctionDraftLoweringStageV1,
    cause: NormalFunctionDraftLoweringCauseV1,
    restoration: NormalFunctionDraftBuilderRestorationReceiptV1,
}

impl RejectedNormalFunctionDraftLoweringV1 {
    pub(in crate::mir) const fn stage(&self) -> NormalFunctionDraftLoweringStageV1 {
        self.stage
    }

    pub(in crate::mir) const fn cause(&self) -> &NormalFunctionDraftLoweringCauseV1 {
        &self.cause
    }

    pub(in crate::mir) const fn has_restoration_receipt(&self) -> bool {
        let _ = &self.restoration.session;
        true
    }

    pub(in crate::mir) fn into_compatibility_error(self) -> CanonicalResolvedBuildErrorV1 {
        match self.cause {
            NormalFunctionDraftLoweringCauseV1::BuilderContract(detail) => {
                CanonicalResolvedBuildErrorV1::BuilderContract(detail.into())
            }
            NormalFunctionDraftLoweringCauseV1::DraftSeal(detail) => {
                let NormalFunctionDraftLoweringStageV1::DraftSeal(stage) = self.stage else {
                    unreachable!("draft-seal cause keeps one draft-seal stage")
                };
                CanonicalResolvedBuildErrorV1::BuilderContract(format!(
                    "[freeze:contract][f1_draft_seal/{:?}] {detail}",
                    stage
                ))
            }
        }
    }
}

fn reject_after_session_discard(
    session: super::calls::CanonicalFunctionLoweringSessionV1<'_>,
    stage: NormalFunctionDraftLoweringStageV1,
    detail: String,
) -> RejectedNormalFunctionDraftLoweringV1 {
    let session = session.discard_unpublished();
    RejectedNormalFunctionDraftLoweringV1 {
        stage,
        cause: NormalFunctionDraftLoweringCauseV1::BuilderContract(detail.into_boxed_str()),
        restoration: NormalFunctionDraftBuilderRestorationReceiptV1 {
            session,
            _seal: NormalFunctionDraftBuilderRestorationReceiptSealV1,
        },
    }
}

fn reject_draft_seal_typed(
    rejected: RejectedFunctionDraftSealV1<'_>,
) -> RejectedNormalFunctionDraftLoweringV1 {
    let stage = rejected.stage();
    let detail = format!("{:?}", rejected.error()).into_boxed_str();
    let session = rejected.discard_with_restoration_receipt();
    RejectedNormalFunctionDraftLoweringV1 {
        stage: NormalFunctionDraftLoweringStageV1::DraftSeal(stage),
        cause: NormalFunctionDraftLoweringCauseV1::DraftSeal(detail),
        restoration: NormalFunctionDraftBuilderRestorationReceiptV1 {
            session,
            _seal: NormalFunctionDraftBuilderRestorationReceiptSealV1,
        },
    }
}

#[cfg(test)]
fn reject_draft_seal<'builder>(
    rejected: RejectedFunctionDraftSealV1<'builder>,
) -> CanonicalResolvedBuildErrorV1 {
    let stage = rejected.stage();
    let error = format!("{:?}", rejected.error());
    rejected.discard();
    CanonicalResolvedBuildErrorV1::BuilderContract(format!(
        "[freeze:contract][f1_draft_seal/{stage:?}] {error}"
    ))
}
