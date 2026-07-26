//! TX0 helper-draft prefix preparation.
//!
//! This box consumes the HANDOFF0 callback-scoped schedule exactly once.  It
//! retains only completed drafts and an exact consumed-operation receipt; a
//! consumed source plan is never reconstructed or stored.

use crate::mir::builder::resolved_lowering::{
    NormalFunctionDraftLoweringStageV1, RejectedNormalFunctionDraftLoweringV1,
};
use crate::mir::compiler::capability::CanonicalTrivialBindingSsaPlanV1;
use crate::mir::compiler::normal_source_plan::{
    NormalCallableHandoffStageV1, NormalHelperDraftAbiExpectationErrorV1,
    OpenNormalCallableModuleTransactionV1, OwnedNormalHelperLoweringScheduleV1,
    PreparedNormalHelperTopologyReceiptV1, RejectedNormalCallableHandoffV1,
    RetainedNormalCallableSourceAuthorityV1,
};
use crate::mir::resolved_semantics::CanonicalCallableKeyV1;
use crate::mir::MirFunction;

use super::super::MirBuilder;

/// One completed helper draft paired to the catalog key that admitted it.
#[derive(Debug)]
pub(in crate::mir) struct VerifiedNormalHelperDraftV1 {
    key: CanonicalCallableKeyV1,
    draft: MirFunction,
    _seal: VerifiedNormalHelperDraftSealV1,
}

#[derive(Debug)]
struct VerifiedNormalHelperDraftSealV1;

impl VerifiedNormalHelperDraftV1 {
    pub(in crate::mir) const fn key(&self) -> &CanonicalCallableKeyV1 {
        &self.key
    }

    pub(in crate::mir) const fn draft(&self) -> &MirFunction {
        &self.draft
    }

    pub(in crate::mir) fn into_draft(self) -> MirFunction {
        self.draft
    }
}

/// The complete, unpublished helper prefix that later TX0 rows may extend.
#[derive(Debug)]
pub(in crate::mir) struct RetainedNormalHelperDraftPrefixV1 {
    topology: PreparedNormalHelperTopologyReceiptV1,
    drafts: Vec<VerifiedNormalHelperDraftV1>,
}

impl RetainedNormalHelperDraftPrefixV1 {
    pub(in crate::mir) const fn topology(&self) -> &PreparedNormalHelperTopologyReceiptV1 {
        &self.topology
    }

    pub(in crate::mir) fn drafts(&self) -> &[VerifiedNormalHelperDraftV1] {
        &self.drafts
    }

    pub(in crate::mir) fn into_drafts(self) -> Vec<VerifiedNormalHelperDraftV1> {
        self.drafts
    }
}

/// Evidence for the single plan that was consumed when helper lowering failed.
#[derive(Debug)]
pub(in crate::mir) struct ConsumedNormalHelperLoweringReceiptV1 {
    key: CanonicalCallableKeyV1,
    ordinal: usize,
    stage: NormalFunctionDraftLoweringStageV1,
    _seal: ConsumedNormalHelperLoweringReceiptSealV1,
}

#[derive(Debug)]
struct ConsumedNormalHelperLoweringReceiptSealV1;

impl ConsumedNormalHelperLoweringReceiptV1 {
    pub(in crate::mir) const fn key(&self) -> &CanonicalCallableKeyV1 {
        &self.key
    }

    pub(in crate::mir) const fn ordinal(&self) -> usize {
        self.ordinal
    }

    pub(in crate::mir) const fn stage(&self) -> NormalFunctionDraftLoweringStageV1 {
        self.stage
    }
}

#[derive(Debug)]
pub(in crate::mir) enum NormalHelperDraftCorrespondenceErrorV1 {
    MissingHeader(NormalHelperDraftAbiExpectationErrorV1),
    SymbolMismatch {
        key: CanonicalCallableKeyV1,
        expected: Box<str>,
        actual: Box<str>,
    },
    ArityMismatch {
        key: CanonicalCallableKeyV1,
        expected: usize,
        actual: usize,
    },
}

#[derive(Debug)]
pub(in crate::mir) enum NormalHelperDraftPrefixFailureV1 {
    Lowering(ConsumedNormalHelperLoweringReceiptV1),
    Correspondence(NormalHelperDraftCorrespondenceErrorV1),
}

/// Success owner for the next TX0 row. The source authority is still open,
/// while every helper plan has been consumed and its borrow is gone.
#[derive(Debug)]
pub(in crate::mir) struct PreparedNormalHelperDraftPrefixV1 {
    transaction: OpenNormalCallableModuleTransactionV1,
    prefix: RetainedNormalHelperDraftPrefixV1,
}

impl PreparedNormalHelperDraftPrefixV1 {
    pub(in crate::mir) const fn prefix(&self) -> &RetainedNormalHelperDraftPrefixV1 {
        &self.prefix
    }

    pub(in crate::mir) fn into_parts(
        self,
    ) -> (
        OpenNormalCallableModuleTransactionV1,
        RetainedNormalHelperDraftPrefixV1,
    ) {
        (self.transaction, self.prefix)
    }
}

/// Failure owner retaining the durable source transaction and exactly the
/// drafts completed before the failed helper operation.
#[derive(Debug)]
pub(in crate::mir) enum RejectedNormalHelperDraftPrefixV1 {
    Handoff(RejectedNormalCallableHandoffV1),
    Lowering {
        transaction: OpenNormalCallableModuleTransactionV1,
        prefix: RetainedNormalHelperDraftPrefixV1,
        failure: NormalHelperDraftPrefixFailureV1,
    },
}

impl RejectedNormalHelperDraftPrefixV1 {
    pub(in crate::mir) fn discard(self) {
        drop(self);
    }
}

impl MirBuilder {
    /// Sole TX0 helper-schedule consumer. The callback result owns no source
    /// borrow, so the open transaction can be retained only after the schedule
    /// is fully consumed or rejected.
    pub(in crate::mir) fn prepare_normal_helper_draft_prefix_v1(
        &mut self,
        transaction: OpenNormalCallableModuleTransactionV1,
    ) -> Result<PreparedNormalHelperDraftPrefixV1, RejectedNormalHelperDraftPrefixV1> {
        let (transaction, outcome) = transaction
            .with_helper_plans(|source, schedule| {
                lower_helper_schedule_with_v1(source, schedule, |plan| {
                    self.lower_resolved_trivial_function_draft_retaining_failure_v1(plan)
                        .map_err(discard_rejected_lowering)
                })
            })
            .map_err(RejectedNormalHelperDraftPrefixV1::Handoff)?;
        match outcome {
            Ok(prefix) => Ok(PreparedNormalHelperDraftPrefixV1 {
                transaction,
                prefix,
            }),
            Err((prefix, failure)) => Err(RejectedNormalHelperDraftPrefixV1::Lowering {
                transaction,
                prefix,
                failure,
            }),
        }
    }
}

fn lower_helper_schedule_with_v1<'source>(
    source: &'source RetainedNormalCallableSourceAuthorityV1,
    schedule: OwnedNormalHelperLoweringScheduleV1<'source>,
    mut lower: impl FnMut(
        CanonicalTrivialBindingSsaPlanV1<'source>,
    ) -> Result<MirFunction, NormalFunctionDraftLoweringStageV1>,
) -> Result<
    RetainedNormalHelperDraftPrefixV1,
    (
        RetainedNormalHelperDraftPrefixV1,
        NormalHelperDraftPrefixFailureV1,
    ),
> {
    let (topology, plans) = schedule.into_parts();
    let mut prefix = RetainedNormalHelperDraftPrefixV1 {
        topology,
        drafts: Vec::with_capacity(plans.len()),
    };

    for (ordinal, (key, plan)) in plans.into_iter().enumerate() {
        let draft = match lower(plan) {
            Ok(draft) => draft,
            Err(stage) => {
                return Err((
                    prefix,
                    NormalHelperDraftPrefixFailureV1::Lowering(
                        ConsumedNormalHelperLoweringReceiptV1 {
                            key,
                            ordinal,
                            stage,
                            _seal: ConsumedNormalHelperLoweringReceiptSealV1,
                        },
                    ),
                ));
            }
        };
        if let Err(error) = verify_helper_draft(source, &key, &draft) {
            return Err((
                prefix,
                NormalHelperDraftPrefixFailureV1::Correspondence(error),
            ));
        }
        prefix.drafts.push(VerifiedNormalHelperDraftV1 {
            key,
            draft,
            _seal: VerifiedNormalHelperDraftSealV1,
        });
    }
    Ok(prefix)
}

fn discard_rejected_lowering(
    rejected: RejectedNormalFunctionDraftLoweringV1,
) -> NormalFunctionDraftLoweringStageV1 {
    let stage = rejected.stage();
    debug_assert!(rejected.has_restoration_receipt());
    drop(rejected);
    stage
}

fn verify_helper_draft(
    source: &RetainedNormalCallableSourceAuthorityV1,
    key: &CanonicalCallableKeyV1,
    draft: &MirFunction,
) -> Result<(), NormalHelperDraftCorrespondenceErrorV1> {
    let expected = source
        .helper_draft_abi(key)
        .map_err(NormalHelperDraftCorrespondenceErrorV1::MissingHeader)?;
    if draft.signature.name != expected.symbol() {
        return Err(NormalHelperDraftCorrespondenceErrorV1::SymbolMismatch {
            key: key.clone(),
            expected: expected.symbol().into(),
            actual: draft.signature.name.clone().into(),
        });
    }
    if draft.signature.params.len() != expected.arity() {
        return Err(NormalHelperDraftCorrespondenceErrorV1::ArityMismatch {
            key: key.clone(),
            expected: expected.arity(),
            actual: draft.signature.params.len(),
        });
    }
    Ok(())
}

#[cfg(test)]
#[path = "callable_draft_prefix_tests.rs"]
mod tests;
#[cfg(test)]
pub(super) use tests::completed_for_main_physical;
