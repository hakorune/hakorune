//! CALLMAIN0: one-shot callable-Main compatibility handoff.
//!
//! This module consumes CHILDREN0's complete owner.  It never opens a second
//! session or physical owner, and it publishes only a ready product for the
//! future BODY0 row.

use super::raw_root_children::{
    RawAppChildrenCompleteInvocationV1, RawChildrenCompleteInvocationV1,
    RawPreRootChildrenCompletionV1, RawRootChildCoreV1, RawRootChildReceiptV1,
    RawScriptChildrenCompleteInvocationV1,
};
use super::raw_root_eligibility::RawRootInvocationV1;
use super::raw_root_plan0::RawPostCallableMainPlanV1;
use super::raw_source_binding::RawPostCallableMainContinuationV1;
use crate::mir::builder::{
    CollectedDraftAdmissionReceiptV1, InvocationBranded, RawCallableMainCompatibilityDispositionV1,
    RawRootPhysicalCallableMainErrorV1, RawRootPhysicalStateV1, RawRootStaticChildWorkErrorV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum RawCallableMainSelectionErrorV1 {
    ScriptSelected,
    MissingAppLocator,
    PhysicalDispositionMismatch,
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir) enum RawCallableMainWorkErrorV1 {
    Source(RawRootStaticChildWorkErrorV1),
    NotCallableMain,
}

#[derive(Debug)]
pub(in crate::mir) struct RawCallableMainReceiptV1 {
    locator: crate::mir::builder::RawSourceLocatorV1,
    receipt: InvocationBranded<CollectedDraftAdmissionReceiptV1>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum RawCallableMainRoleV1 {
    CallableMainCompatibility,
}

impl RawCallableMainReceiptV1 {
    pub(in crate::mir) fn locator(&self) -> &crate::mir::builder::RawSourceLocatorV1 {
        &self.locator
    }

    pub(in crate::mir) const fn role(&self) -> RawCallableMainRoleV1 {
        RawCallableMainRoleV1::CallableMainCompatibility
    }

    pub(in crate::mir) fn receipt_brand(
        &self,
    ) -> crate::mir::module_invocation_identity::ModuleInvocationBrandV1 {
        self.receipt.brand()
    }
}

#[derive(Debug)]
pub(in crate::mir) enum RawAppCallableMainOutcomeV1 {
    NotSelected {
        locator: crate::mir::builder::RawSourceLocatorV1,
    },
    Selected {
        receipt: RawCallableMainReceiptV1,
    },
}

impl RawAppCallableMainOutcomeV1 {
    pub(in crate::mir) const fn is_selected(&self) -> bool {
        matches!(self, Self::Selected { .. })
    }

    pub(in crate::mir) fn locator(&self) -> &crate::mir::builder::RawSourceLocatorV1 {
        match self {
            Self::NotSelected { locator } => locator,
            Self::Selected { receipt } => receipt.locator(),
        }
    }

    pub(in crate::mir) fn selected_receipt(&self) -> Option<&RawCallableMainReceiptV1> {
        match self {
            Self::NotSelected { .. } => None,
            Self::Selected { receipt } => Some(receipt),
        }
    }
}

#[derive(Debug)]
struct RawCallableMainCoreV1 {
    token: crate::mir::module_invocation_identity::ModuleInvocationTokenV1,
    source: crate::mir::builder::OwnedRawSourceV1,
    continuation: RawPostCallableMainContinuationV1,
    config: crate::mir::builder::BuilderInvocationConfigV1,
    module_name: Box<str>,
    plan: RawPostCallableMainPlanV1,
    session: crate::mir::builder::ModuleBuilderInvocationSessionV1,
    physical: RawRootPhysicalStateV1,
}

#[derive(Debug)]
pub(in crate::mir) enum RawCallableMainReadyInvocationV1 {
    Script(RawScriptCallableMainReadyInvocationV1),
    App(RawAppCallableMainReadyInvocationV1),
}

#[derive(Debug)]
pub(in crate::mir) struct RawScriptCallableMainReadyInvocationV1 {
    core: RawCallableMainCoreV1,
    completion: RawPreRootChildrenCompletionV1,
    receipts: Box<[RawRootChildReceiptV1]>,
}

#[derive(Debug)]
pub(in crate::mir) struct RawAppCallableMainReadyInvocationV1 {
    core: RawCallableMainCoreV1,
    completion: RawPreRootChildrenCompletionV1,
    helper_receipts: Box<[RawRootChildReceiptV1]>,
    outcome: RawAppCallableMainOutcomeV1,
}

impl RawCallableMainReadyInvocationV1 {
    pub(in crate::mir) fn app_outcome(&self) -> Option<&RawAppCallableMainOutcomeV1> {
        match self {
            Self::Script(_) => None,
            Self::App(ready) => Some(&ready.outcome),
        }
    }

    pub(in crate::mir) fn tracker_completed_children(&self) -> usize {
        match self {
            Self::Script(ready) => ready.core.physical.tracker_completed_children(),
            Self::App(ready) => ready.core.physical.tracker_completed_children(),
        }
    }

    pub(in crate::mir) fn physical_brand(
        &self,
    ) -> crate::mir::module_invocation_identity::ModuleInvocationBrandV1 {
        match self {
            Self::Script(ready) => ready.core.physical.brand(),
            Self::App(ready) => ready.core.physical.brand(),
        }
    }

    pub(in crate::mir) fn session_brand(
        &self,
    ) -> crate::mir::module_invocation_identity::ModuleInvocationBrandV1 {
        match self {
            Self::Script(ready) => ready.core.session.brand(),
            Self::App(ready) => ready.core.session.brand(),
        }
    }

    pub(in crate::mir) fn token_brand(
        &self,
    ) -> crate::mir::module_invocation_identity::ModuleInvocationBrandV1 {
        match self {
            Self::Script(ready) => ready.core.token.brand(),
            Self::App(ready) => ready.core.token.brand(),
        }
    }
}

#[derive(Debug)]
struct RawCallableMainRejectedOwnerV1 {
    core: RawCallableMainCoreV1,
    completion: RawPreRootChildrenCompletionV1,
    helper_receipts: Box<[RawRootChildReceiptV1]>,
    failed_locator: Option<crate::mir::builder::RawSourceLocatorV1>,
    issued_receipt: Option<InvocationBranded<CollectedDraftAdmissionReceiptV1>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum RawCallableMainFailureStageV1 {
    Selection,
    Source,
    Physical,
}

#[derive(Debug)]
pub(in crate::mir) enum RawCallableMainErrorV1 {
    Selection(RawCallableMainSelectionErrorV1),
    Source(RawCallableMainWorkErrorV1),
    Physical(RawRootPhysicalCallableMainErrorV1),
}

#[derive(Debug)]
pub(in crate::mir) struct RejectedRawCallableMainInvocationV1 {
    owner: RawCallableMainRejectedOwnerV1,
    stage: RawCallableMainFailureStageV1,
    error: RawCallableMainErrorV1,
}

impl RejectedRawCallableMainInvocationV1 {
    pub(in crate::mir) const fn stage(&self) -> RawCallableMainFailureStageV1 {
        self.stage
    }

    pub(in crate::mir) fn error(&self) -> &RawCallableMainErrorV1 {
        &self.error
    }

    pub(in crate::mir) fn helper_receipt_count(&self) -> usize {
        self.owner.helper_receipts.len()
    }

    pub(in crate::mir) fn failed_locator(
        &self,
    ) -> Option<&crate::mir::builder::RawSourceLocatorV1> {
        self.owner.failed_locator.as_ref()
    }

    pub(in crate::mir) fn discard(self) {}
}

impl RawChildrenCompleteInvocationV1 {
    pub(in crate::mir) fn finish_callable_main(
        self,
    ) -> Result<RawCallableMainReadyInvocationV1, RejectedRawCallableMainInvocationV1> {
        match self {
            Self::Script(complete) => finish_script(complete),
            Self::App(complete) => finish_app(complete),
        }
    }
}

fn finish_script(
    complete: RawScriptChildrenCompleteInvocationV1,
) -> Result<RawCallableMainReadyInvocationV1, RejectedRawCallableMainInvocationV1> {
    let RawScriptChildrenCompleteInvocationV1 {
        core,
        completion,
        receipts,
    } = complete;
    let (core, disposition, locator) = split_core(core);
    if disposition != RawCallableMainCompatibilityDispositionV1::NotSelected || locator.is_some() {
        return Err(reject(
            core,
            completion,
            receipts.into_vec().into_boxed_slice(),
            None,
            None,
            RawCallableMainFailureStageV1::Selection,
            RawCallableMainErrorV1::Selection(RawCallableMainSelectionErrorV1::ScriptSelected),
        ));
    }
    Ok(RawCallableMainReadyInvocationV1::Script(
        RawScriptCallableMainReadyInvocationV1 {
            core,
            completion,
            receipts,
        },
    ))
}

fn finish_app(
    complete: RawAppChildrenCompleteInvocationV1,
) -> Result<RawCallableMainReadyInvocationV1, RejectedRawCallableMainInvocationV1> {
    let RawAppChildrenCompleteInvocationV1 {
        core,
        completion,
        receipts,
    } = complete;
    let (mut core, disposition, locator) = split_core(core);
    if core.physical.callable_main() != disposition {
        return Err(reject(
            core,
            completion,
            receipts,
            None,
            None,
            RawCallableMainFailureStageV1::Selection,
            RawCallableMainErrorV1::Selection(
                RawCallableMainSelectionErrorV1::PhysicalDispositionMismatch,
            ),
        ));
    }
    let locator = match locator {
        Some(locator) => locator,
        None => {
            return Err(reject(
                core,
                completion,
                receipts,
                None,
                None,
                RawCallableMainFailureStageV1::Selection,
                RawCallableMainErrorV1::Selection(
                    RawCallableMainSelectionErrorV1::MissingAppLocator,
                ),
            ))
        }
    };
    if disposition == RawCallableMainCompatibilityDispositionV1::NotSelected {
        return Ok(RawCallableMainReadyInvocationV1::App(
            RawAppCallableMainReadyInvocationV1 {
                core,
                completion,
                helper_receipts: receipts,
                outcome: RawAppCallableMainOutcomeV1::NotSelected { locator },
            },
        ));
    }
    let work = match core.source.prepare_static_child(locator.clone(), 0) {
        Ok(work) => match work.into_callable_main() {
            Ok(work) => work,
            Err(error) => {
                return Err(reject(
                    core,
                    completion,
                    receipts,
                    Some(locator),
                    None,
                    RawCallableMainFailureStageV1::Source,
                    RawCallableMainErrorV1::Source(RawCallableMainWorkErrorV1::Source(error)),
                ))
            }
        },
        Err(error) => {
            return Err(reject(
                core,
                completion,
                receipts,
                Some(locator),
                None,
                RawCallableMainFailureStageV1::Source,
                RawCallableMainErrorV1::Source(RawCallableMainWorkErrorV1::Source(error)),
            ))
        }
    };
    let RawCallableMainCoreV1 {
        token,
        source,
        continuation,
        config,
        module_name,
        plan,
        mut session,
        physical,
    } = core;
    let result = physical.complete_callable_main(session.builder_mut(), work);
    match result {
        Ok(completed) => {
            let (physical, receipt) = completed.into_parts();
            Ok(RawCallableMainReadyInvocationV1::App(
                RawAppCallableMainReadyInvocationV1 {
                    core: RawCallableMainCoreV1 {
                        token,
                        source,
                        continuation,
                        config,
                        module_name,
                        plan,
                        session,
                        physical,
                    },
                    completion,
                    helper_receipts: receipts,
                    outcome: RawAppCallableMainOutcomeV1::Selected {
                        receipt: RawCallableMainReceiptV1 { locator, receipt },
                    },
                },
            ))
        }
        Err(failure) => {
            let (physical, issued_receipt, error) = failure.into_parts();
            Err(reject(
                RawCallableMainCoreV1 {
                    token,
                    source,
                    continuation,
                    config,
                    module_name,
                    plan,
                    session,
                    physical,
                },
                completion,
                receipts,
                Some(locator),
                issued_receipt,
                RawCallableMainFailureStageV1::Physical,
                RawCallableMainErrorV1::Physical(error),
            ))
        }
    }
}

fn split_core(
    core: RawRootChildCoreV1,
) -> (
    RawCallableMainCoreV1,
    RawCallableMainCompatibilityDispositionV1,
    Option<crate::mir::builder::RawSourceLocatorV1>,
) {
    let RawRootChildCoreV1 {
        token,
        source,
        continuation,
        config,
        module_name,
        plan,
        session,
        physical,
    } = core;
    let (continuation, disposition) = continuation.into_callable_main_decision();
    let (plan, locator) = plan.into_post_callable_main();
    debug_assert!(matches!(
        (&plan, locator.as_ref()),
        (RawPostCallableMainPlanV1::Script(_), None) | (RawPostCallableMainPlanV1::App(_), Some(_))
    ));
    (
        RawCallableMainCoreV1 {
            token,
            source,
            continuation,
            config,
            module_name,
            plan,
            session,
            physical,
        },
        disposition,
        locator,
    )
}

fn reject(
    core: RawCallableMainCoreV1,
    completion: RawPreRootChildrenCompletionV1,
    helper_receipts: Box<[RawRootChildReceiptV1]>,
    failed_locator: Option<crate::mir::builder::RawSourceLocatorV1>,
    issued_receipt: Option<InvocationBranded<CollectedDraftAdmissionReceiptV1>>,
    stage: RawCallableMainFailureStageV1,
    error: RawCallableMainErrorV1,
) -> RejectedRawCallableMainInvocationV1 {
    RejectedRawCallableMainInvocationV1 {
        owner: RawCallableMainRejectedOwnerV1 {
            core,
            completion,
            helper_receipts,
            failed_locator,
            issued_receipt,
        },
        stage,
        error,
    }
}
