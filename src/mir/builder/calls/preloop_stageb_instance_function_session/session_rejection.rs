//! Owned failure surfaces for the bounded Stage-B instance-function session.

use crate::mir::builder::calls::function_session::LegacyFunctionPayloadSessionErrorV1;
use crate::mir::preloop_stageb_carrier::{
    PreloopStageBInstanceDraftSourceErrorV1, PreparedPreloopStageBFunctionIngressV1,
};

use super::body_schedule::CompletedPreloopStageBBodyScheduleV1;
use super::rejection::RejectedPreloopStageBBodyScheduleV1;
use super::session::{
    CompletedPreloopStageBInstanceFunctionPayloadV1, PreparedPreloopStageBInstanceFunctionV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PreloopStageBInstanceFunctionStageV1 {
    SourceProjection,
    Preparation,
    StepTree,
    BodySchedule,
    Finalizer,
    SessionCleanup,
}

#[derive(Debug)]
pub(super) enum PreloopStageBInstanceFunctionCauseV1 {
    SourceProjection(PreloopStageBInstanceDraftSourceErrorV1),
    Preparation(Box<str>),
    StepTree(Box<str>),
    BodySchedule(Box<str>),
    Finalizer(Box<str>),
    CleanupAfterSuccess(Box<str>),
    DuringCleanup(Box<str>),
}

#[derive(Debug)]
enum RetainedPreloopStageBInstanceFunctionPrimaryOwnerV1 {
    Ingress(PreparedPreloopStageBFunctionIngressV1),
    Prepared(PreparedPreloopStageBInstanceFunctionV1),
    BodySchedule(RejectedPreloopStageBBodyScheduleV1),
    Finalizer(CompletedPreloopStageBBodyScheduleV1),
}

#[derive(Debug)]
pub(super) struct PreloopStageBInstanceFunctionPrimaryRejectionV1 {
    owner: RetainedPreloopStageBInstanceFunctionPrimaryOwnerV1,
    stage: PreloopStageBInstanceFunctionStageV1,
    cause: PreloopStageBInstanceFunctionCauseV1,
}

#[derive(Debug)]
enum RetainedPreloopStageBInstanceFunctionSessionOwnerV1 {
    Primary(PreloopStageBInstanceFunctionPrimaryRejectionV1),
    CleanupAfterSuccess(CompletedPreloopStageBInstanceFunctionPayloadV1),
    DuringCleanup(PreloopStageBInstanceFunctionPrimaryRejectionV1),
}

#[derive(Debug)]
pub(super) struct RejectedPreloopStageBInstanceFunctionSessionV1 {
    owner: RetainedPreloopStageBInstanceFunctionSessionOwnerV1,
    stage: PreloopStageBInstanceFunctionStageV1,
    cause: PreloopStageBInstanceFunctionCauseV1,
}

impl PreloopStageBInstanceFunctionPrimaryRejectionV1 {
    pub(super) fn source(
        ingress: PreparedPreloopStageBFunctionIngressV1,
        cause: PreloopStageBInstanceDraftSourceErrorV1,
    ) -> Self {
        Self {
            owner: RetainedPreloopStageBInstanceFunctionPrimaryOwnerV1::Ingress(ingress),
            stage: PreloopStageBInstanceFunctionStageV1::SourceProjection,
            cause: PreloopStageBInstanceFunctionCauseV1::SourceProjection(cause),
        }
    }

    pub(super) fn prepared(
        owner: PreparedPreloopStageBInstanceFunctionV1,
        stage: PreloopStageBInstanceFunctionStageV1,
        detail: String,
    ) -> Self {
        let cause = match stage {
            PreloopStageBInstanceFunctionStageV1::Preparation => {
                PreloopStageBInstanceFunctionCauseV1::Preparation(detail.into_boxed_str())
            }
            PreloopStageBInstanceFunctionStageV1::StepTree => {
                PreloopStageBInstanceFunctionCauseV1::StepTree(detail.into_boxed_str())
            }
            _ => unreachable!("prepared owner is retained only before body descent"),
        };
        Self {
            owner: RetainedPreloopStageBInstanceFunctionPrimaryOwnerV1::Prepared(owner),
            stage,
            cause,
        }
    }

    pub(super) fn body(rejected: RejectedPreloopStageBBodyScheduleV1) -> Self {
        let detail = rejected.bounded_report();
        Self {
            owner: RetainedPreloopStageBInstanceFunctionPrimaryOwnerV1::BodySchedule(rejected),
            stage: PreloopStageBInstanceFunctionStageV1::BodySchedule,
            cause: PreloopStageBInstanceFunctionCauseV1::BodySchedule(detail),
        }
    }

    pub(super) fn finalizer(
        schedule: CompletedPreloopStageBBodyScheduleV1,
        detail: String,
    ) -> Self {
        Self {
            owner: RetainedPreloopStageBInstanceFunctionPrimaryOwnerV1::Finalizer(schedule),
            stage: PreloopStageBInstanceFunctionStageV1::Finalizer,
            cause: PreloopStageBInstanceFunctionCauseV1::Finalizer(detail.into_boxed_str()),
        }
    }
}

impl RejectedPreloopStageBInstanceFunctionSessionV1 {
    pub(super) fn from_session(
        error: LegacyFunctionPayloadSessionErrorV1<
            PreloopStageBInstanceFunctionPrimaryRejectionV1,
            CompletedPreloopStageBInstanceFunctionPayloadV1,
        >,
    ) -> Self {
        match error {
            LegacyFunctionPayloadSessionErrorV1::Primary(primary) => Self {
                stage: primary.stage,
                cause: primary.cause_for_projection(),
                owner: RetainedPreloopStageBInstanceFunctionSessionOwnerV1::Primary(primary),
            },
            LegacyFunctionPayloadSessionErrorV1::CleanupAfterSuccess { payload, detail } => Self {
                owner: RetainedPreloopStageBInstanceFunctionSessionOwnerV1::CleanupAfterSuccess(
                    payload,
                ),
                stage: PreloopStageBInstanceFunctionStageV1::SessionCleanup,
                cause: PreloopStageBInstanceFunctionCauseV1::CleanupAfterSuccess(detail),
            },
            LegacyFunctionPayloadSessionErrorV1::DuringCleanup { primary, detail } => Self {
                owner: RetainedPreloopStageBInstanceFunctionSessionOwnerV1::DuringCleanup(primary),
                stage: PreloopStageBInstanceFunctionStageV1::SessionCleanup,
                cause: PreloopStageBInstanceFunctionCauseV1::DuringCleanup(detail),
            },
        }
    }

    pub(super) const fn stage(&self) -> PreloopStageBInstanceFunctionStageV1 {
        self.stage
    }

    pub(super) const fn cause(&self) -> &PreloopStageBInstanceFunctionCauseV1 {
        &self.cause
    }

    pub(super) fn bounded_report(&self) -> Box<str> {
        format!(
            "[mir/preloop-stageb/instance-function/{:?}] {:?}",
            self.stage, self.cause
        )
        .into_boxed_str()
    }

    pub(super) fn discard(self) {
        match self.owner {
            RetainedPreloopStageBInstanceFunctionSessionOwnerV1::Primary(primary)
            | RetainedPreloopStageBInstanceFunctionSessionOwnerV1::DuringCleanup(primary) => {
                primary.discard()
            }
            RetainedPreloopStageBInstanceFunctionSessionOwnerV1::CleanupAfterSuccess(payload) => {
                payload.discard()
            }
        }
    }
}

impl PreloopStageBInstanceFunctionPrimaryRejectionV1 {
    fn cause_for_projection(&self) -> PreloopStageBInstanceFunctionCauseV1 {
        match &self.cause {
            PreloopStageBInstanceFunctionCauseV1::SourceProjection(cause) => {
                PreloopStageBInstanceFunctionCauseV1::SourceProjection(*cause)
            }
            PreloopStageBInstanceFunctionCauseV1::Preparation(detail) => {
                PreloopStageBInstanceFunctionCauseV1::Preparation(detail.clone())
            }
            PreloopStageBInstanceFunctionCauseV1::StepTree(detail) => {
                PreloopStageBInstanceFunctionCauseV1::StepTree(detail.clone())
            }
            PreloopStageBInstanceFunctionCauseV1::BodySchedule(detail) => {
                PreloopStageBInstanceFunctionCauseV1::BodySchedule(detail.clone())
            }
            PreloopStageBInstanceFunctionCauseV1::Finalizer(detail) => {
                PreloopStageBInstanceFunctionCauseV1::Finalizer(detail.clone())
            }
            PreloopStageBInstanceFunctionCauseV1::CleanupAfterSuccess(_)
            | PreloopStageBInstanceFunctionCauseV1::DuringCleanup(_) => {
                unreachable!("primary rejection cannot own a session-cleanup cause")
            }
        }
    }

    fn discard(self) {
        match self.owner {
            RetainedPreloopStageBInstanceFunctionPrimaryOwnerV1::Ingress(ingress) => {
                let _ = ingress;
            }
            RetainedPreloopStageBInstanceFunctionPrimaryOwnerV1::Prepared(prepared) => {
                prepared.discard();
            }
            RetainedPreloopStageBInstanceFunctionPrimaryOwnerV1::BodySchedule(rejected) => {
                rejected.discard();
            }
            RetainedPreloopStageBInstanceFunctionPrimaryOwnerV1::Finalizer(schedule) => {
                schedule.discard();
            }
        }
    }
}
