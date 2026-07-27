//! Lifetime-free rejection surface for the bounded Stage-B body schedule.

use crate::mir::preloop_stageb_carrier::{
    PreparedPreloopStageBFunctionBodyRecipeV1, RejectedPreloopStageBFunctionIngressV1,
};
use crate::mir::source_instance_result_contract::{
    PreparedPreloopLocatedArgumentV1, RetainedNestedInstanceResultRebindAuthorityV1,
};

use super::super::preloop_located_outer_completion::OwnedRejectedPreloopLocatedOuterCompletionV1;
use super::super::preloop_outer_carrier_assignment::OwnedRejectedPreloopCarrierAssignmentV1;
use super::super::preloop_outer_carrier_transaction::OwnedRejectedPreloopOuterCarrierCallV1;
use super::super::preloop_outer_carrier_type::{
    CompletedPreloopStageBCarrierV1, OwnedRejectedPreloopOuterCarrierIntegerPublicationV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PreloopStageBBodyScheduleStageV1 {
    Preflight,
    Prefix,
    Selected,
    Suffix,
    Completion,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum PreloopStageBBodyScheduleCauseV1 {
    BodyCardinalityMismatch { expected: usize, actual: usize },
    SelectedIndexUnavailable { selected: usize, len: usize },
    SuffixStartMismatch { expected: usize, actual: usize },
    OrdinaryDescent { index: usize, detail: Box<str> },
    SelectedTransaction { detail: Box<str> },
    Driver { detail: Box<str> },
    SelectedNotReached,
}

#[derive(Debug)]
pub(super) enum OwnedPreloopStageBSelectedTransactionRejectionV1 {
    Outer(OwnedRejectedPreloopLocatedOuterCompletionV1),
    Carrier(OwnedRejectedPreloopOuterCarrierCallV1),
    Assignment(OwnedRejectedPreloopCarrierAssignmentV1),
    Publication(OwnedRejectedPreloopOuterCarrierIntegerPublicationV1),
}

#[derive(Debug)]
enum RetainedPreloopStageBBodyScheduleOwnerV1 {
    Ingress(RejectedPreloopStageBFunctionIngressV1),
    Pending {
        nested_result: RetainedNestedInstanceResultRebindAuthorityV1,
        recipe: PreparedPreloopStageBFunctionBodyRecipeV1,
    },
    Selected(OwnedPreloopStageBSelectedTransactionRejectionV1),
    Published(CompletedPreloopStageBCarrierV1),
}

#[derive(Debug)]
pub(super) struct RejectedPreloopStageBBodyScheduleV1 {
    owner: RetainedPreloopStageBBodyScheduleOwnerV1,
    stage: PreloopStageBBodyScheduleStageV1,
    cause: PreloopStageBBodyScheduleCauseV1,
}

impl RejectedPreloopStageBBodyScheduleV1 {
    pub(super) fn ingress(owner: RejectedPreloopStageBFunctionIngressV1) -> Self {
        let detail = owner.bounded_report();
        Self {
            owner: RetainedPreloopStageBBodyScheduleOwnerV1::Ingress(owner),
            stage: PreloopStageBBodyScheduleStageV1::Preflight,
            cause: PreloopStageBBodyScheduleCauseV1::Driver { detail },
        }
    }

    pub(super) fn pending(
        source: PreparedPreloopLocatedArgumentV1<'_, '_, '_>,
        recipe: PreparedPreloopStageBFunctionBodyRecipeV1,
        stage: PreloopStageBBodyScheduleStageV1,
        cause: PreloopStageBBodyScheduleCauseV1,
    ) -> Self {
        Self {
            owner: RetainedPreloopStageBBodyScheduleOwnerV1::Pending {
                nested_result: source.into_completed_retained_rebind_authority(),
                recipe,
            },
            stage,
            cause,
        }
    }

    pub(super) fn selected(
        owner: OwnedPreloopStageBSelectedTransactionRejectionV1,
        detail: Box<str>,
    ) -> Self {
        Self {
            owner: RetainedPreloopStageBBodyScheduleOwnerV1::Selected(owner),
            stage: PreloopStageBBodyScheduleStageV1::Selected,
            cause: PreloopStageBBodyScheduleCauseV1::SelectedTransaction { detail },
        }
    }

    pub(super) fn published(
        owner: CompletedPreloopStageBCarrierV1,
        stage: PreloopStageBBodyScheduleStageV1,
        cause: PreloopStageBBodyScheduleCauseV1,
    ) -> Self {
        Self {
            owner: RetainedPreloopStageBBodyScheduleOwnerV1::Published(owner),
            stage,
            cause,
        }
    }

    pub(super) const fn stage(&self) -> PreloopStageBBodyScheduleStageV1 {
        self.stage
    }

    pub(super) const fn cause(&self) -> &PreloopStageBBodyScheduleCauseV1 {
        &self.cause
    }

    pub(super) fn bounded_report(&self) -> Box<str> {
        format!(
            "[mir/preloop-stageb/body-schedule/{:?}] {:?}",
            self.stage, self.cause
        )
        .into_boxed_str()
    }

    #[cfg(test)]
    pub(super) const fn retained_published_carrier_for_test(
        &self,
    ) -> Option<&CompletedPreloopStageBCarrierV1> {
        match &self.owner {
            RetainedPreloopStageBBodyScheduleOwnerV1::Published(carrier) => Some(carrier),
            RetainedPreloopStageBBodyScheduleOwnerV1::Ingress(_)
            | RetainedPreloopStageBBodyScheduleOwnerV1::Pending { .. }
            | RetainedPreloopStageBBodyScheduleOwnerV1::Selected(_) => None,
        }
    }

    pub(super) fn discard(self) {
        match self.owner {
            RetainedPreloopStageBBodyScheduleOwnerV1::Ingress(owner) => owner.discard(),
            RetainedPreloopStageBBodyScheduleOwnerV1::Pending {
                nested_result,
                recipe,
            } => {
                nested_result.discard();
                let _ = recipe;
            }
            RetainedPreloopStageBBodyScheduleOwnerV1::Selected(owner) => match owner {
                OwnedPreloopStageBSelectedTransactionRejectionV1::Outer(owner) => owner.discard(),
                OwnedPreloopStageBSelectedTransactionRejectionV1::Carrier(owner) => owner.discard(),
                OwnedPreloopStageBSelectedTransactionRejectionV1::Assignment(owner) => {
                    owner.discard()
                }
                OwnedPreloopStageBSelectedTransactionRejectionV1::Publication(owner) => {
                    owner.discard()
                }
            },
            RetainedPreloopStageBBodyScheduleOwnerV1::Published(owner) => owner.discard(),
        }
    }
}
