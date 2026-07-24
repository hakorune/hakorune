//! FINAL0: direct RawDrainedInvocationV1 finalization handoff.
//!
//! FINAL0 seals the already-drained module against its retained witness.  It
//! does not re-read source/catalog or module inventory and does not run
//! postprocess.

use super::raw_root_callable_main::RawAppCallableMainOutcomeV1;
use super::raw_root_children::{RawPreRootChildrenCompletionV1, RawRootChildReceiptV1};
use super::raw_root_drain::{
    RawAppDrainedInvocationV1, RawDrainedInvocationCoreV1, RawDrainedInvocationV1,
    RawScriptDrainedInvocationV1,
};
use super::raw_root_postprocess::RawPostprocessReadyInvocationV1;
use super::raw_runtime_inputs::RawRuntimeInputSnapshotV1;
use super::raw_source_binding::RawPostCallableMainContinuationV1;
use crate::mir::builder::{
    RawFinalizedPhysicalV1, RawRootPhysicalFinalizationErrorV1, RejectedRawPhysicalFinalizationV1,
};
use crate::mir::raw_finalization_contract::RawFinalizationRouteEvidenceV1;
use crate::mir::raw_physical_drain::RawPhysicalCallableMainDispositionV1;

#[derive(Debug)]
pub(in crate::mir) enum RawFinalizedInvocationV1 {
    Script(RawScriptFinalizedInvocationV1),
    App(RawAppFinalizedInvocationV1),
}

#[derive(Debug)]
pub(in crate::mir) struct RawScriptFinalizedInvocationV1 {
    core: RawFinalizedInvocationCoreV1,
}

#[derive(Debug)]
pub(in crate::mir) struct RawAppFinalizedInvocationV1 {
    pub(in crate::mir) core: RawFinalizedInvocationCoreV1,
    pub(in crate::mir) callable_main: RawAppCallableMainOutcomeV1,
}

#[derive(Debug)]
pub(in crate::mir) struct RawFinalizedInvocationCoreV1 {
    continuation: RawPostCallableMainContinuationV1,
    module_name: Box<str>,
    runtime_inputs: RawRuntimeInputSnapshotV1,
    completion: RawPreRootChildrenCompletionV1,
    helper_receipts: Box<[RawRootChildReceiptV1]>,
    physical: RawFinalizedPhysicalV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum RawFinalizationFailureStageV1 {
    Physical,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum RawFinalizationErrorV1 {
    Physical(RawRootPhysicalFinalizationErrorV1),
}

#[derive(Debug)]
enum RejectedRawFinalizationOwnerV1 {
    Script {
        continuation: RawPostCallableMainContinuationV1,
        module_name: Box<str>,
        runtime_inputs: RawRuntimeInputSnapshotV1,
        completion: RawPreRootChildrenCompletionV1,
        helper_receipts: Box<[RawRootChildReceiptV1]>,
        physical: RejectedRawPhysicalFinalizationV1,
    },
    App {
        continuation: RawPostCallableMainContinuationV1,
        module_name: Box<str>,
        runtime_inputs: RawRuntimeInputSnapshotV1,
        completion: RawPreRootChildrenCompletionV1,
        helper_receipts: Box<[RawRootChildReceiptV1]>,
        callable_main: RawAppCallableMainOutcomeV1,
        physical: RejectedRawPhysicalFinalizationV1,
    },
}

#[derive(Debug)]
pub(in crate::mir) struct RejectedRawDrainFinalizationInvocationV1 {
    owner: RejectedRawFinalizationOwnerV1,
    stage: RawFinalizationFailureStageV1,
    error: RawFinalizationErrorV1,
}

impl RawDrainedInvocationV1 {
    pub(in crate::mir) fn prepare_finalization(
        self,
    ) -> Result<RawFinalizedInvocationV1, RejectedRawDrainFinalizationInvocationV1> {
        match self {
            Self::Script(wrapper) => finalize_script(wrapper),
            Self::App(wrapper) => finalize_app(wrapper),
        }
    }
}

impl RawFinalizedInvocationV1 {
    pub(in crate::mir) fn prepare_postprocess(self) -> RawPostprocessReadyInvocationV1 {
        match self {
            RawFinalizedInvocationV1::Script(wrapper) => {
                let RawScriptFinalizedInvocationV1 { core } = wrapper;
                let RawFinalizedInvocationCoreV1 {
                    continuation,
                    module_name,
                    runtime_inputs,
                    completion,
                    helper_receipts,
                    physical,
                } = core;
                RawPostprocessReadyInvocationV1::from_script(
                    continuation,
                    module_name,
                    runtime_inputs,
                    completion,
                    helper_receipts,
                    physical.begin_postprocess(),
                )
            }
            RawFinalizedInvocationV1::App(wrapper) => {
                let RawAppFinalizedInvocationV1 {
                    core,
                    callable_main,
                } = wrapper;
                let RawFinalizedInvocationCoreV1 {
                    continuation,
                    module_name,
                    runtime_inputs,
                    completion,
                    helper_receipts,
                    physical,
                } = core;
                RawPostprocessReadyInvocationV1::from_app(
                    continuation,
                    module_name,
                    runtime_inputs,
                    completion,
                    helper_receipts,
                    callable_main,
                    physical.begin_postprocess(),
                )
            }
        }
    }
}

fn finalize_script(
    wrapper: RawScriptDrainedInvocationV1,
) -> Result<RawFinalizedInvocationV1, RejectedRawDrainFinalizationInvocationV1> {
    let RawDrainedInvocationCoreV1 {
        continuation,
        module_name,
        runtime_inputs,
        completion,
        helper_receipts,
        physical,
    } = wrapper.core;
    let route = RawFinalizationRouteEvidenceV1::Script {
        module_name: &module_name,
        helper_count: helper_receipts.len(),
    };
    match physical.prepare_raw_finalization(route) {
        Ok(prepared) => Ok(RawFinalizedInvocationV1::Script(
            RawScriptFinalizedInvocationV1 {
                core: RawFinalizedInvocationCoreV1 {
                    continuation,
                    module_name,
                    runtime_inputs,
                    completion,
                    helper_receipts,
                    physical: prepared.commit(),
                },
            },
        )),
        Err(physical) => Err(reject_script(
            continuation,
            module_name,
            runtime_inputs,
            completion,
            helper_receipts,
            physical,
        )),
    }
}

fn finalize_app(
    wrapper: RawAppDrainedInvocationV1,
) -> Result<RawFinalizedInvocationV1, RejectedRawDrainFinalizationInvocationV1> {
    let RawAppDrainedInvocationV1 {
        core,
        callable_main,
    } = wrapper;
    let RawDrainedInvocationCoreV1 {
        continuation,
        module_name,
        runtime_inputs,
        completion,
        helper_receipts,
        physical,
    } = core;
    let route = RawFinalizationRouteEvidenceV1::App {
        module_name: &module_name,
        helper_count: helper_receipts.len(),
        callable_main: if callable_main.is_selected() {
            RawPhysicalCallableMainDispositionV1::Selected
        } else {
            RawPhysicalCallableMainDispositionV1::NotSelected
        },
    };
    match physical.prepare_raw_finalization(route) {
        Ok(prepared) => Ok(RawFinalizedInvocationV1::App(RawAppFinalizedInvocationV1 {
            core: RawFinalizedInvocationCoreV1 {
                continuation,
                module_name,
                runtime_inputs,
                completion,
                helper_receipts,
                physical: prepared.commit(),
            },
            callable_main,
        })),
        Err(physical) => Err(reject_app(
            continuation,
            module_name,
            runtime_inputs,
            completion,
            helper_receipts,
            callable_main,
            physical,
        )),
    }
}

fn reject_script(
    continuation: RawPostCallableMainContinuationV1,
    module_name: Box<str>,
    runtime_inputs: RawRuntimeInputSnapshotV1,
    completion: RawPreRootChildrenCompletionV1,
    helper_receipts: Box<[RawRootChildReceiptV1]>,
    physical: RejectedRawPhysicalFinalizationV1,
) -> RejectedRawDrainFinalizationInvocationV1 {
    let error = RawFinalizationErrorV1::Physical(physical.error().clone());
    RejectedRawDrainFinalizationInvocationV1 {
        owner: RejectedRawFinalizationOwnerV1::Script {
            continuation,
            module_name,
            runtime_inputs,
            completion,
            helper_receipts,
            physical,
        },
        stage: RawFinalizationFailureStageV1::Physical,
        error,
    }
}

fn reject_app(
    continuation: RawPostCallableMainContinuationV1,
    module_name: Box<str>,
    runtime_inputs: RawRuntimeInputSnapshotV1,
    completion: RawPreRootChildrenCompletionV1,
    helper_receipts: Box<[RawRootChildReceiptV1]>,
    callable_main: RawAppCallableMainOutcomeV1,
    physical: RejectedRawPhysicalFinalizationV1,
) -> RejectedRawDrainFinalizationInvocationV1 {
    let error = RawFinalizationErrorV1::Physical(physical.error().clone());
    RejectedRawDrainFinalizationInvocationV1 {
        owner: RejectedRawFinalizationOwnerV1::App {
            continuation,
            module_name,
            runtime_inputs,
            completion,
            helper_receipts,
            callable_main,
            physical,
        },
        stage: RawFinalizationFailureStageV1::Physical,
        error,
    }
}

impl RejectedRawDrainFinalizationInvocationV1 {
    pub(in crate::mir) const fn stage(&self) -> RawFinalizationFailureStageV1 {
        self.stage
    }

    pub(in crate::mir) fn error(&self) -> &RawFinalizationErrorV1 {
        &self.error
    }

    pub(in crate::mir) fn discard(self) {}
}
