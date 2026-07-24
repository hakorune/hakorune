//! RAW-SOURCE0-LOWER0-ROOT0-DRAIN0-S0: compiler-owned Raw drain handoff.

use super::raw_root_callable_main::RawAppCallableMainOutcomeV1;
use super::raw_root_children::{RawPreRootChildrenCompletionV1, RawRootChildReceiptV1};
use super::raw_root_decl_access::{
    RawAppRootBatchCompleteInvocationV1, RawRootBatchCompleteCoreV1,
    RawRootBatchCompleteInvocationV1, RawScriptRootBatchCompleteInvocationV1,
};
use super::raw_runtime_inputs::RawRuntimeInputSnapshotV1;
use super::raw_source_binding::RawPostCallableMainContinuationV1;
use crate::mir::builder::{
    PreparedRawPhysicalDrainV1, RawDrainedPhysicalV1, RawPhysicalDrainErrorV1,
    RejectedRawPhysicalDrainV1,
};
use crate::mir::raw_physical_drain::{
    RawPhysicalCallableMainDispositionV1, RawPhysicalDrainRouteV1,
};

#[derive(Debug)]
pub(in crate::mir) enum RawDrainedInvocationV1 {
    Script(RawScriptDrainedInvocationV1),
    App(RawAppDrainedInvocationV1),
}

#[derive(Debug)]
pub(in crate::mir) struct RawScriptDrainedInvocationV1 {
    pub(in crate::mir) core: RawDrainedInvocationCoreV1,
}

#[derive(Debug)]
pub(in crate::mir) struct RawAppDrainedInvocationV1 {
    pub(in crate::mir) core: RawDrainedInvocationCoreV1,
    pub(in crate::mir) callable_main: RawAppCallableMainOutcomeV1,
}

#[derive(Debug)]
pub(in crate::mir) struct RawDrainedInvocationCoreV1 {
    pub(in crate::mir) continuation: RawPostCallableMainContinuationV1,
    pub(in crate::mir) module_name: Box<str>,
    pub(in crate::mir) runtime_inputs: RawRuntimeInputSnapshotV1,
    pub(in crate::mir) completion: RawPreRootChildrenCompletionV1,
    pub(in crate::mir) helper_receipts: Box<[RawRootChildReceiptV1]>,
    pub(in crate::mir) physical: RawDrainedPhysicalV1,
}

#[derive(Debug)]
pub(in crate::mir) enum PreparedRawDrainInvocationV1 {
    Script(PreparedRawScriptDrainInvocationV1),
    App(PreparedRawAppDrainInvocationV1),
}

#[derive(Debug)]
struct PreparedRawScriptDrainInvocationV1 {
    continuation: RawPostCallableMainContinuationV1,
    module_name: Box<str>,
    runtime_inputs: RawRuntimeInputSnapshotV1,
    completion: RawPreRootChildrenCompletionV1,
    helper_receipts: Box<[RawRootChildReceiptV1]>,
    physical: PreparedRawPhysicalDrainV1,
}

#[derive(Debug)]
struct PreparedRawAppDrainInvocationV1 {
    continuation: RawPostCallableMainContinuationV1,
    module_name: Box<str>,
    runtime_inputs: RawRuntimeInputSnapshotV1,
    completion: RawPreRootChildrenCompletionV1,
    helper_receipts: Box<[RawRootChildReceiptV1]>,
    callable_main: RawAppCallableMainOutcomeV1,
    physical: PreparedRawPhysicalDrainV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum RawDrainFailureStageV1 {
    Physical,
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir) enum RawDrainErrorV1 {
    Physical(RawPhysicalDrainErrorV1),
}

#[derive(Debug)]
enum RejectedRawDrainOwnerV1 {
    Script {
        continuation: RawPostCallableMainContinuationV1,
        module_name: Box<str>,
        runtime_inputs: RawRuntimeInputSnapshotV1,
        completion: RawPreRootChildrenCompletionV1,
        helper_receipts: Box<[RawRootChildReceiptV1]>,
        physical: RejectedRawPhysicalDrainV1,
    },
    App {
        continuation: RawPostCallableMainContinuationV1,
        module_name: Box<str>,
        runtime_inputs: RawRuntimeInputSnapshotV1,
        completion: RawPreRootChildrenCompletionV1,
        helper_receipts: Box<[RawRootChildReceiptV1]>,
        callable_main: RawAppCallableMainOutcomeV1,
        physical: RejectedRawPhysicalDrainV1,
    },
}

#[derive(Debug)]
pub(in crate::mir) struct RejectedRawDrainInvocationV1 {
    owner: RejectedRawDrainOwnerV1,
    stage: RawDrainFailureStageV1,
    error: RawDrainErrorV1,
}

impl RawRootBatchCompleteInvocationV1 {
    pub(in crate::mir) fn prepare_drain(
        self,
    ) -> Result<PreparedRawDrainInvocationV1, RejectedRawDrainInvocationV1> {
        match self {
            Self::Script(wrapper) => prepare_script(wrapper),
            Self::App(wrapper) => prepare_app(wrapper),
        }
    }
}

fn prepare_script(
    wrapper: RawScriptRootBatchCompleteInvocationV1,
) -> Result<PreparedRawDrainInvocationV1, RejectedRawDrainInvocationV1> {
    let RawRootBatchCompleteCoreV1 {
        continuation,
        module_name,
        runtime_inputs,
        completion,
        helper_receipts,
        physical,
    } = wrapper.core;
    let result = physical.prepare_raw_drain(
        RawPhysicalDrainRouteV1::Script,
        RawPhysicalCallableMainDispositionV1::NotSelected,
    );
    match result {
        Ok(physical) => Ok(PreparedRawDrainInvocationV1::Script(
            PreparedRawScriptDrainInvocationV1 {
                continuation,
                module_name,
                runtime_inputs,
                completion,
                helper_receipts,
                physical,
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

fn prepare_app(
    wrapper: RawAppRootBatchCompleteInvocationV1,
) -> Result<PreparedRawDrainInvocationV1, RejectedRawDrainInvocationV1> {
    let RawAppRootBatchCompleteInvocationV1 {
        core,
        callable_main,
    } = wrapper;
    let RawRootBatchCompleteCoreV1 {
        continuation,
        module_name,
        runtime_inputs,
        completion,
        helper_receipts,
        physical,
    } = core;
    let disposition = if callable_main.is_selected() {
        RawPhysicalCallableMainDispositionV1::Selected
    } else {
        RawPhysicalCallableMainDispositionV1::NotSelected
    };
    match physical.prepare_raw_drain(RawPhysicalDrainRouteV1::App, disposition) {
        Ok(physical) => Ok(PreparedRawDrainInvocationV1::App(
            PreparedRawAppDrainInvocationV1 {
                continuation,
                module_name,
                runtime_inputs,
                completion,
                helper_receipts,
                callable_main,
                physical,
            },
        )),
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

impl PreparedRawDrainInvocationV1 {
    pub(in crate::mir) fn drain(self) -> RawDrainedInvocationV1 {
        match self {
            Self::Script(prepared) => {
                RawDrainedInvocationV1::Script(RawScriptDrainedInvocationV1 {
                    core: drain_core(
                        prepared.continuation,
                        prepared.module_name,
                        prepared.runtime_inputs,
                        prepared.completion,
                        prepared.helper_receipts,
                        prepared.physical,
                    ),
                })
            }
            Self::App(prepared) => RawDrainedInvocationV1::App(RawAppDrainedInvocationV1 {
                core: drain_core(
                    prepared.continuation,
                    prepared.module_name,
                    prepared.runtime_inputs,
                    prepared.completion,
                    prepared.helper_receipts,
                    prepared.physical,
                ),
                callable_main: prepared.callable_main,
            }),
        }
    }
}

fn drain_core(
    continuation: RawPostCallableMainContinuationV1,
    module_name: Box<str>,
    runtime_inputs: RawRuntimeInputSnapshotV1,
    completion: RawPreRootChildrenCompletionV1,
    helper_receipts: Box<[RawRootChildReceiptV1]>,
    prepared: PreparedRawPhysicalDrainV1,
) -> RawDrainedInvocationCoreV1 {
    RawDrainedInvocationCoreV1 {
        continuation,
        module_name,
        runtime_inputs,
        completion,
        helper_receipts,
        physical: prepared.drain(),
    }
}

fn reject_script(
    continuation: RawPostCallableMainContinuationV1,
    module_name: Box<str>,
    runtime_inputs: RawRuntimeInputSnapshotV1,
    completion: RawPreRootChildrenCompletionV1,
    helper_receipts: Box<[RawRootChildReceiptV1]>,
    physical: RejectedRawPhysicalDrainV1,
) -> RejectedRawDrainInvocationV1 {
    let error = RawDrainErrorV1::Physical(physical.error().clone());
    RejectedRawDrainInvocationV1 {
        owner: RejectedRawDrainOwnerV1::Script {
            continuation,
            module_name,
            runtime_inputs,
            completion,
            helper_receipts,
            physical,
        },
        stage: RawDrainFailureStageV1::Physical,
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
    physical: RejectedRawPhysicalDrainV1,
) -> RejectedRawDrainInvocationV1 {
    let error = RawDrainErrorV1::Physical(physical.error().clone());
    RejectedRawDrainInvocationV1 {
        owner: RejectedRawDrainOwnerV1::App {
            continuation,
            module_name,
            runtime_inputs,
            completion,
            helper_receipts,
            callable_main,
            physical,
        },
        stage: RawDrainFailureStageV1::Physical,
        error,
    }
}

impl RejectedRawDrainInvocationV1 {
    pub(in crate::mir) const fn stage(&self) -> RawDrainFailureStageV1 {
        self.stage
    }

    pub(in crate::mir) fn error(&self) -> &RawDrainErrorV1 {
        &self.error
    }

    pub(in crate::mir) fn discard(self) {}
}
