//! TX0 preparation of one source Main draft and its physical entry thunk.
//!
//! This owner extends an immutable helper prefix only. It deliberately has no
//! module batch, candidate verification, or publication terminal.

use crate::mir::builder::resolved_lowering::{
    NormalFunctionDraftLoweringStageV1, RejectedNormalFunctionDraftLoweringV1,
};
use crate::mir::compiler::normal_source_plan::{
    seal_normal_main_physical_relation_v1, NormalMainThunkPlanErrorV1,
    OpenNormalCallableModuleTransactionV1, RejectedNormalMainProofBindingV1,
    VerifiedNormalMainPhysicalRelationV1,
};
use crate::mir::MirFunction;

use super::super::MirBuilder;
use super::callable_draft_prefix::{
    PreparedNormalHelperDraftPrefixV1, RetainedNormalHelperDraftPrefixV1,
};
use super::physical_thunk::{
    NormalMainPhysicalThunkErrorV1, VerifiedNormalMainPhysicalThunkDraftV1,
};
use super::source_draft::{NormalMainSourceDraftErrorV1, VerifiedNormalMainSourceDraftV1};

#[derive(Debug)]
pub(in crate::mir) enum NormalCallableMainPhysicalStageV1 {
    PhysicalRelation(NormalMainThunkPlanErrorV1),
    SourceLowering(NormalFunctionDraftLoweringStageV1),
}

/// Prepared unpublished drafts for the next batch-only TX0 row.
#[derive(Debug)]
pub(in crate::mir) struct PreparedNormalCallableMainPhysicalV1 {
    transaction: OpenNormalCallableModuleTransactionV1,
    helpers: RetainedNormalHelperDraftPrefixV1,
    source: VerifiedNormalMainSourceDraftV1,
    physical: VerifiedNormalMainPhysicalThunkDraftV1,
    relation: VerifiedNormalMainPhysicalRelationV1,
}

impl PreparedNormalCallableMainPhysicalV1 {
    pub(in crate::mir) const fn helpers(&self) -> &RetainedNormalHelperDraftPrefixV1 {
        &self.helpers
    }

    pub(in crate::mir) const fn source(&self) -> &VerifiedNormalMainSourceDraftV1 {
        &self.source
    }

    pub(in crate::mir) const fn physical(&self) -> &VerifiedNormalMainPhysicalThunkDraftV1 {
        &self.physical
    }

    pub(in crate::mir) const fn relation(&self) -> &VerifiedNormalMainPhysicalRelationV1 {
        &self.relation
    }
}

/// All rejection variants retain every completed earlier draft. Main's
/// consumed F1 proof is represented by the typed rejection stage only.
#[derive(Debug)]
pub(in crate::mir) enum RejectedNormalCallableMainPhysicalV1 {
    Binding(RejectedNormalMainProofBindingV1),
    PhysicalRelation {
        transaction: OpenNormalCallableModuleTransactionV1,
        helpers: RetainedNormalHelperDraftPrefixV1,
        error: NormalMainThunkPlanErrorV1,
    },
    SourceLowering {
        transaction: OpenNormalCallableModuleTransactionV1,
        helpers: RetainedNormalHelperDraftPrefixV1,
        stage: NormalFunctionDraftLoweringStageV1,
    },
    SourceDraft {
        transaction: OpenNormalCallableModuleTransactionV1,
        helpers: RetainedNormalHelperDraftPrefixV1,
        relation: VerifiedNormalMainPhysicalRelationV1,
        draft: MirFunction,
        error: NormalMainSourceDraftErrorV1,
    },
    PhysicalThunk {
        transaction: OpenNormalCallableModuleTransactionV1,
        helpers: RetainedNormalHelperDraftPrefixV1,
        relation: VerifiedNormalMainPhysicalRelationV1,
        source: VerifiedNormalMainSourceDraftV1,
        error: NormalMainPhysicalThunkErrorV1,
    },
}

impl RejectedNormalCallableMainPhysicalV1 {
    pub(in crate::mir) fn discard(self) {
        drop(self);
    }

    #[cfg(test)]
    pub(super) fn retained_helper_count(&self) -> usize {
        match self {
            Self::Binding(_) => 0,
            Self::PhysicalRelation { helpers, .. }
            | Self::SourceLowering { helpers, .. }
            | Self::SourceDraft { helpers, .. }
            | Self::PhysicalThunk { helpers, .. } => helpers.drafts().len(),
        }
    }
}

#[cfg(test)]
#[derive(Debug, Clone, Copy)]
pub(crate) enum NormalCallableMainPhysicalTestStageV1 {
    SourceLowering,
    PhysicalThunk,
}

impl MirBuilder {
    /// Sole MAIN-PHYSICAL0 consumer. Main proof binding and all physical facts
    /// are completed before the source plan is consumed by the F1 lowerer.
    pub(in crate::mir) fn prepare_normal_callable_main_physical_v1(
        &mut self,
        prefix: PreparedNormalHelperDraftPrefixV1,
    ) -> Result<PreparedNormalCallableMainPhysicalV1, RejectedNormalCallableMainPhysicalV1> {
        let (transaction, helpers) = prefix.into_parts();
        let (transaction, outcome) = transaction
            .with_main_lowering_plan(|_, plan| {
                let header = plan
                    .seal_resolved_owner_header_v1()
                    .map_err(NormalMainThunkPlanErrorV1::Header)
                    .map_err(NormalCallableMainPhysicalStageV1::PhysicalRelation)?;
                let relation = seal_normal_main_physical_relation_v1(
                    header,
                    plan.completion(),
                    plan.terminal_profile(),
                )
                .map_err(NormalCallableMainPhysicalStageV1::PhysicalRelation)?;
                let draft = self
                    .lower_resolved_trivial_function_draft_retaining_failure_v1(plan)
                    .map_err(discard_rejected_lowering)?;
                let source = VerifiedNormalMainSourceDraftV1::seal(
                    draft,
                    relation.source_header().symbol().as_mir_name(),
                    relation.source_header().arity(),
                    relation.source_result(),
                );
                Ok((relation, source))
            })
            .map_err(RejectedNormalCallableMainPhysicalV1::Binding)?;

        let (relation, source) = match outcome {
            Err(NormalCallableMainPhysicalStageV1::PhysicalRelation(error)) => {
                return Err(RejectedNormalCallableMainPhysicalV1::PhysicalRelation {
                    transaction,
                    helpers,
                    error,
                })
            }
            Err(NormalCallableMainPhysicalStageV1::SourceLowering(stage)) => {
                return Err(RejectedNormalCallableMainPhysicalV1::SourceLowering {
                    transaction,
                    helpers,
                    stage,
                })
            }
            Ok((relation, Ok(source))) => (relation, source),
            Ok((relation, Err((draft, error)))) => {
                return Err(RejectedNormalCallableMainPhysicalV1::SourceDraft {
                    transaction,
                    helpers,
                    relation,
                    draft,
                    error,
                })
            }
        };
        let physical = match VerifiedNormalMainPhysicalThunkDraftV1::prepare(
            relation.source_header(),
            relation.source_result(),
            relation.entry(),
        ) {
            Ok(physical) => physical,
            Err(error) => {
                return Err(RejectedNormalCallableMainPhysicalV1::PhysicalThunk {
                    transaction,
                    helpers,
                    relation,
                    source,
                    error,
                })
            }
        };
        Ok(PreparedNormalCallableMainPhysicalV1 {
            transaction,
            helpers,
            source,
            physical,
            relation,
        })
    }
}

fn discard_rejected_lowering(
    rejected: RejectedNormalFunctionDraftLoweringV1,
) -> NormalCallableMainPhysicalStageV1 {
    let stage = rejected.stage();
    debug_assert!(rejected.has_restoration_receipt());
    drop(rejected);
    NormalCallableMainPhysicalStageV1::SourceLowering(stage)
}

#[cfg(test)]
pub(crate) fn reject_normal_callable_main_physical_at_stage_for_test(
    builder: &mut MirBuilder,
    prefix: PreparedNormalHelperDraftPrefixV1,
    stage: NormalCallableMainPhysicalTestStageV1,
) -> RejectedNormalCallableMainPhysicalV1 {
    let (transaction, helpers) = prefix.into_parts();
    let (transaction, outcome) = transaction
        .with_main_lowering_plan(|_, plan| {
            if matches!(stage, NormalCallableMainPhysicalTestStageV1::SourceLowering) {
                return Err(NormalCallableMainPhysicalStageV1::SourceLowering(
                    NormalFunctionDraftLoweringStageV1::BodyLowering,
                ));
            }
            let header = plan
                .seal_resolved_owner_header_v1()
                .expect("test injection requires sealed Main header");
            let relation = seal_normal_main_physical_relation_v1(
                header,
                plan.completion(),
                plan.terminal_profile(),
            )
            .expect("test injection requires sealed Main relation");
            let draft = builder
                .lower_resolved_trivial_function_draft_retaining_failure_v1(plan)
                .expect("test injection requires source Main draft");
            let source = VerifiedNormalMainSourceDraftV1::seal(
                draft,
                relation.source_header().symbol().as_mir_name(),
                relation.source_header().arity(),
                relation.source_result(),
            )
            .expect("test injection requires source Main correspondence");
            Ok((relation, source))
        })
        .expect("test injection requires Main proof binding");
    match (stage, outcome) {
        (
            NormalCallableMainPhysicalTestStageV1::SourceLowering,
            Err(NormalCallableMainPhysicalStageV1::SourceLowering(stage)),
        ) => RejectedNormalCallableMainPhysicalV1::SourceLowering {
            transaction,
            helpers,
            stage,
        },
        (NormalCallableMainPhysicalTestStageV1::PhysicalThunk, Ok((relation, source))) => {
            RejectedNormalCallableMainPhysicalV1::PhysicalThunk {
                transaction,
                helpers,
                relation,
                source,
                error: NormalMainPhysicalThunkErrorV1::PhysicalArityMismatch { actual: 1 },
            }
        }
        _ => unreachable!("test stage must retain its exact prepared prefix"),
    }
}
