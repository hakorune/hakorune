//! COMMIT0: RawDirect external-commit preparation.
//!
//! This module consumes the complete POST0 owner, validates all retained
//! evidence without mutation, and produces a typed prepared product.  It
//! deliberately does not call the live Builder commit or expose a module.

use super::module_postprocess::ModulePostprocessScheduleV1;
use super::raw_root_postprocess::{
    RawPostprocessEvidenceV1, RawPostprocessRouteKindV1, RawPostprocessStageEvidenceV1,
    RawPostprocessedInvocationCoreV1, RawPostprocessedInvocationV1,
    RawScriptPostprocessedInvocationV1, RawAppPostprocessedInvocationV1,
};
use crate::mir::builder::{
    RawExternalCommitModuleV1, RawExternalCommitPhysicalErrorV1,
    RawExternalCommitPhysicalHandoffV1, PreparedBuilderExternalCommitV1,
};
use crate::mir::module_invocation_identity::ModuleInvocationFamilyV1;
use crate::mir::raw_physical_drain::{
    RawPhysicalCallableMainDispositionV1, RawPhysicalDrainRouteV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum RawExternalCommitFailureStageV1 {
    RouteEvidence,
    Identity,
    PostprocessEvidence,
    Physical,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum RawExternalCommitErrorV1 {
    NonRawFamily,
    RouteEvidenceMismatch,
    ModuleNameMismatch,
    ScheduleMismatch,
    VerificationMismatch,
    ProgressMismatch,
    ReceiptEvidenceMismatch,
    Physical(RawExternalCommitPhysicalErrorV1),
}

#[derive(Debug)]
pub(in crate::mir) struct PreparedRawExternalCommitPhysicalV1 {
    token: crate::mir::module_invocation_identity::ModuleInvocationTokenV1,
    builder: PreparedBuilderExternalCommitV1,
    module: RawExternalCommitModuleV1,
    _seal: PreparedRawExternalCommitPhysicalSealV1,
}

#[derive(Debug)]
struct PreparedRawExternalCommitPhysicalSealV1;

#[derive(Debug)]
pub(in crate::mir) struct PreparedRawExternalCommitCoreV1 {
    pub(super) physical: PreparedRawExternalCommitPhysicalV1,
    pub(super) evidence: RawPostprocessEvidenceV1,
}

#[derive(Debug)]
pub(in crate::mir) enum PreparedRawExternalCommitV1 {
    Script(PreparedRawScriptExternalCommitV1),
    App(PreparedRawAppExternalCommitV1),
}

#[derive(Debug)]
pub(in crate::mir) struct PreparedRawScriptExternalCommitV1 {
    pub(super) core: PreparedRawExternalCommitCoreV1,
}

#[derive(Debug)]
pub(in crate::mir) struct PreparedRawAppExternalCommitV1 {
    pub(super) core: PreparedRawExternalCommitCoreV1,
}

#[derive(Debug)]
pub(in crate::mir) struct RejectedRawExternalCommitInvocationV1 {
    owner: RawPostprocessedInvocationV1,
    stage: RawExternalCommitFailureStageV1,
    error: RawExternalCommitErrorV1,
}

impl RejectedRawExternalCommitInvocationV1 {
    pub(in crate::mir) fn stage(&self) -> RawExternalCommitFailureStageV1 {
        self.stage
    }

    pub(in crate::mir) fn error(&self) -> &RawExternalCommitErrorV1 {
        &self.error
    }

    pub(in crate::mir) fn discard(self) {}
}

impl RawPostprocessedInvocationV1 {
    pub(in crate::mir) fn prepare_external_commit(
        self,
    ) -> Result<
        PreparedRawExternalCommitV1,
        RejectedRawExternalCommitInvocationV1,
    > {
        if let Err((stage, error)) = validate_invocation(&self) {
            return Err(RejectedRawExternalCommitInvocationV1 {
                owner: self,
                stage,
                error,
            });
        }
        match self {
            RawPostprocessedInvocationV1::Script(wrapper) => {
                Ok(PreparedRawExternalCommitV1::Script(prepare_script(wrapper)))
            }
            RawPostprocessedInvocationV1::App(wrapper) => {
                Ok(PreparedRawExternalCommitV1::App(prepare_app(wrapper)))
            }
        }
    }
}

fn validate_invocation(
    invocation: &RawPostprocessedInvocationV1,
) -> Result<(), (RawExternalCommitFailureStageV1, RawExternalCommitErrorV1)> {
    match invocation {
        RawPostprocessedInvocationV1::Script(wrapper) => {
            validate_core(&wrapper.core, RawPostprocessRouteKindV1::Script)
        }
        RawPostprocessedInvocationV1::App(wrapper) => {
            validate_core(&wrapper.core, RawPostprocessRouteKindV1::App)
        }
    }
}

fn validate_core(
    core: &RawPostprocessedInvocationCoreV1,
    expected_route: RawPostprocessRouteKindV1,
) -> Result<(), (RawExternalCommitFailureStageV1, RawExternalCommitErrorV1)> {
    let stage = &core.stage_evidence;
    if stage.route_kind() != expected_route
        || !stage.brands_match(core.physical.brand())
    {
        return Err((
            RawExternalCommitFailureStageV1::RouteEvidence,
            RawExternalCommitErrorV1::RouteEvidenceMismatch,
        ));
    }
    if stage.module_name() != core.physical.module_name() {
        return Err((
            RawExternalCommitFailureStageV1::Identity,
            RawExternalCommitErrorV1::ModuleNameMismatch,
        ));
    }
    let expected_schedule = ModulePostprocessScheduleV1::for_family(ModuleInvocationFamilyV1::Raw);
    if stage.schedule != expected_schedule {
        return Err((
            RawExternalCommitFailureStageV1::PostprocessEvidence,
            RawExternalCommitErrorV1::ScheduleMismatch,
        ));
    }
    if !matches!(
        stage.verification,
        super::module_postprocess::ModuleVerificationEvidenceV1::Raw { .. }
    ) {
        return Err((
            RawExternalCommitFailureStageV1::PostprocessEvidence,
            RawExternalCommitErrorV1::VerificationMismatch,
        ));
    }
    if stage.progress != core.physical.progress() {
        return Err((
            RawExternalCommitFailureStageV1::PostprocessEvidence,
            RawExternalCommitErrorV1::ProgressMismatch,
        ));
    }
    let expected_route = match expected_route {
        RawPostprocessRouteKindV1::Script => RawPhysicalDrainRouteV1::Script,
        RawPostprocessRouteKindV1::App => RawPhysicalDrainRouteV1::App,
    };
    let expected_callable_main = if stage.callable_main_selected() {
        RawPhysicalCallableMainDispositionV1::Selected
    } else {
        RawPhysicalCallableMainDispositionV1::NotSelected
    };
    core.physical
        .validate_external_commit(
            stage.module_name(),
            expected_route,
            expected_callable_main,
            stage.helper_count(),
        )
        .map_err(|error| {
            (
                RawExternalCommitFailureStageV1::Physical,
                RawExternalCommitErrorV1::Physical(error),
            )
        })
}

fn prepare_script(
    wrapper: RawScriptPostprocessedInvocationV1,
) -> PreparedRawScriptExternalCommitV1 {
    let RawScriptPostprocessedInvocationV1 { core } = wrapper;
    PreparedRawScriptExternalCommitV1 {
        core: prepare_core(core),
    }
}

fn prepare_app(
    wrapper: RawAppPostprocessedInvocationV1,
) -> PreparedRawAppExternalCommitV1 {
    let RawAppPostprocessedInvocationV1 { core } = wrapper;
    PreparedRawAppExternalCommitV1 {
        core: prepare_core(core),
    }
}

fn prepare_core(core: RawPostprocessedInvocationCoreV1) -> PreparedRawExternalCommitCoreV1 {
    let RawPostprocessedInvocationCoreV1 {
        physical,
        stage_evidence,
    } = core;
    let handoff = physical.into_external_commit_preflighted();
    let RawExternalCommitPhysicalHandoffV1 {
        token,
        builder,
        module,
        witness,
        finalization_parity,
        postprocess_parity,
        progress,
    } = handoff;
    let RawPostprocessStageEvidenceV1 {
        route,
        schedule,
        verification,
        progress: stage_progress,
    } = stage_evidence;
    debug_assert_eq!(stage_progress, progress);
    let evidence = RawPostprocessEvidenceV1 {
        route,
        schedule,
        verification,
        progress,
        witness,
        finalization_parity,
        postprocess_parity,
    };
    PreparedRawExternalCommitCoreV1 {
        physical: PreparedRawExternalCommitPhysicalV1 {
            token,
            builder,
            module,
            _seal: PreparedRawExternalCommitPhysicalSealV1,
        },
        evidence,
    }
}
