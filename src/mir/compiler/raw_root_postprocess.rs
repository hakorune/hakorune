//! POST-CARRIER: direct FINAL0 -> shared postprocess handoff.
//!
//! The route owner remains Script/App-specific while the existing
//! `ModulePostprocessOwnerV1` executes the only stage kernel.  The Builder-side
//! physical owner exposes named stage operations, never a mutable module.

use super::module_postprocess::{
    ModulePostprocessOwnerV1, ModulePostprocessScheduleV1, ModuleVerificationEvidenceV1,
    PostprocessFailureStageV1,
};
use super::module_postprocess_stages::{
    run_postprocess_stages, PostprocessStageFailureV1, PostprocessStageTarget,
};
use super::raw_root_callable_main::RawAppCallableMainOutcomeV1;
use super::raw_root_children::{RawPreRootChildrenCompletionV1, RawRootChildReceiptV1};
use super::raw_runtime_inputs::RawRuntimeInputSnapshotV1;
use super::raw_source_binding::RawPostCallableMainContinuationV1;
use crate::mir::builder::{
    RawPostprocessCarrierParityErrorV1, RawPostprocessPhysicalOwnerV1, RawPostprocessProgressV1,
    RawPostprocessedPhysicalV1,
};
use crate::mir::verification_types::VerificationError;

#[derive(Debug)]
pub(in crate::mir) enum RawPostprocessReadyInvocationV1 {
    Script(RawScriptPostprocessReadyInvocationV1),
    App(RawAppPostprocessReadyInvocationV1),
}

#[derive(Debug)]
pub(in crate::mir) struct RawScriptPostprocessReadyInvocationV1 {
    core: RawPostprocessReadyCoreV1,
}

#[derive(Debug)]
pub(in crate::mir) struct RawAppPostprocessReadyInvocationV1 {
    core: RawPostprocessReadyCoreV1,
    callable_main: RawAppCallableMainOutcomeV1,
}

#[derive(Debug)]
struct RawPostprocessReadyCoreV1 {
    continuation: RawPostCallableMainContinuationV1,
    module_name: Box<str>,
    runtime_inputs: RawRuntimeInputSnapshotV1,
    completion: RawPreRootChildrenCompletionV1,
    helper_receipts: Box<[RawRootChildReceiptV1]>,
    physical: RawPostprocessPhysicalOwnerV1,
}

#[derive(Debug)]
pub(in crate::mir) enum RawPostprocessRouteEvidenceV1 {
    Script {
        continuation: RawPostCallableMainContinuationV1,
        module_name: Box<str>,
        runtime_inputs: RawRuntimeInputSnapshotV1,
        completion: RawPreRootChildrenCompletionV1,
        helpers: Box<[RawRootChildReceiptV1]>,
    },
    App {
        continuation: RawPostCallableMainContinuationV1,
        module_name: Box<str>,
        runtime_inputs: RawRuntimeInputSnapshotV1,
        completion: RawPreRootChildrenCompletionV1,
        helpers: Box<[RawRootChildReceiptV1]>,
        callable_main: RawAppCallableMainOutcomeV1,
    },
}

#[derive(Debug)]
pub(in crate::mir) struct RawPostprocessStageEvidenceV1 {
    pub(in crate::mir) route: RawPostprocessRouteEvidenceV1,
    pub(in crate::mir) schedule: ModulePostprocessScheduleV1,
    pub(in crate::mir) verification: ModuleVerificationEvidenceV1,
    pub(in crate::mir) progress: RawPostprocessProgressV1,
}

#[derive(Debug)]
pub(in crate::mir) struct RawPostprocessEvidenceV1 {
    pub(in crate::mir) route: RawPostprocessRouteEvidenceV1,
    pub(in crate::mir) schedule: ModulePostprocessScheduleV1,
    pub(in crate::mir) verification: ModuleVerificationEvidenceV1,
    pub(in crate::mir) progress: RawPostprocessProgressV1,
    pub(in crate::mir) witness: crate::mir::builder::RawDrainWitnessV1,
    pub(in crate::mir) finalization_parity: crate::mir::builder::RawFinalizationParitySealV1,
    pub(in crate::mir) postprocess_parity: crate::mir::builder::RawPostprocessParitySealV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum RawPostprocessRouteKindV1 {
    Script,
    App,
}

impl RawPostprocessStageEvidenceV1 {
    pub(super) fn route_kind(&self) -> RawPostprocessRouteKindV1 {
        match self.route {
            RawPostprocessRouteEvidenceV1::Script { .. } => RawPostprocessRouteKindV1::Script,
            RawPostprocessRouteEvidenceV1::App { .. } => RawPostprocessRouteKindV1::App,
        }
    }

    pub(super) fn module_name(&self) -> &str {
        match &self.route {
            RawPostprocessRouteEvidenceV1::Script { module_name, .. }
            | RawPostprocessRouteEvidenceV1::App { module_name, .. } => module_name,
        }
    }

    pub(super) fn helper_count(&self) -> usize {
        match &self.route {
            RawPostprocessRouteEvidenceV1::Script { helpers, .. }
            | RawPostprocessRouteEvidenceV1::App { helpers, .. } => helpers.len(),
        }
    }

    pub(super) fn callable_main_selected(&self) -> bool {
        match &self.route {
            RawPostprocessRouteEvidenceV1::Script { .. } => false,
            RawPostprocessRouteEvidenceV1::App { callable_main, .. } => {
                callable_main.is_selected()
            }
        }
    }

    pub(super) fn brands_match(
        &self,
        brand: crate::mir::module_invocation_identity::ModuleInvocationBrandV1,
    ) -> bool {
        let (completion_brand, helper_brands, callable_brand) = match &self.route {
            RawPostprocessRouteEvidenceV1::Script {
                completion,
                helpers,
                ..
            } => (
                completion.brand(),
                helpers.iter().map(|receipt| receipt.brand()).collect::<Vec<_>>(),
                None,
            ),
            RawPostprocessRouteEvidenceV1::App {
                completion,
                helpers,
                callable_main,
                ..
            } => (
                completion.brand(),
                helpers.iter().map(|receipt| receipt.brand()).collect::<Vec<_>>(),
                callable_main.selected_receipt().map(|receipt| receipt.receipt_brand()),
            ),
        };
        completion_brand == brand
            && helper_brands.into_iter().all(|candidate| candidate == brand)
            && callable_brand.map_or(true, |candidate| candidate == brand)
    }
}

#[derive(Debug)]
pub(in crate::mir) struct RawPostprocessedInvocationCoreV1 {
    pub(in crate::mir) physical: RawPostprocessedPhysicalV1,
    pub(in crate::mir) stage_evidence: RawPostprocessStageEvidenceV1,
}

#[derive(Debug)]
pub(in crate::mir) enum RawPostprocessedInvocationV1 {
    Script(RawScriptPostprocessedInvocationV1),
    App(RawAppPostprocessedInvocationV1),
}

#[derive(Debug)]
pub(in crate::mir) struct RawScriptPostprocessedInvocationV1 {
    pub(in crate::mir) core: RawPostprocessedInvocationCoreV1,
}

#[derive(Debug)]
pub(in crate::mir) struct RawAppPostprocessedInvocationV1 {
    pub(in crate::mir) core: RawPostprocessedInvocationCoreV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum RawPostprocessFailureStageV1 {
    Optimizer,
    ContractRefresh,
    CarrierParity,
    FinalVerification,
}

#[derive(Debug)]
pub(in crate::mir) enum RawPostprocessErrorV1 {
    OptimizerDiagnostics { count: usize },
    ContractRefresh(String),
    CarrierParity(RawPostprocessCarrierParityErrorV1),
    FinalVerification(Box<[VerificationError]>),
}

#[derive(Debug)]
pub(in crate::mir) struct RejectedRawPostprocessInvocationV1 {
    owner: RawPostprocessReadyInvocationV1,
    stage: RawPostprocessFailureStageV1,
    error: RawPostprocessErrorV1,
    verification: Option<ModuleVerificationEvidenceV1>,
}

impl RejectedRawPostprocessInvocationV1 {
    pub(in crate::mir) fn stage(&self) -> RawPostprocessFailureStageV1 {
        self.stage.clone()
    }

    pub(in crate::mir) fn error(&self) -> &RawPostprocessErrorV1 {
        &self.error
    }

    pub(in crate::mir) fn verification(&self) -> Option<&ModuleVerificationEvidenceV1> {
        self.verification.as_ref()
    }

    pub(in crate::mir) fn progress(&self) -> RawPostprocessProgressV1 {
        match &self.owner {
            RawPostprocessReadyInvocationV1::Script(ready) => ready.core.physical.progress(),
            RawPostprocessReadyInvocationV1::App(ready) => ready.core.physical.progress(),
        }
    }

    pub(in crate::mir) fn discard(self) {}
}

impl RawPostprocessReadyInvocationV1 {
    pub(in crate::mir) fn from_script(
        continuation: RawPostCallableMainContinuationV1,
        module_name: Box<str>,
        runtime_inputs: RawRuntimeInputSnapshotV1,
        completion: RawPreRootChildrenCompletionV1,
        helper_receipts: Box<[RawRootChildReceiptV1]>,
        physical: RawPostprocessPhysicalOwnerV1,
    ) -> Self {
        Self::Script(RawScriptPostprocessReadyInvocationV1 {
            core: RawPostprocessReadyCoreV1 {
                continuation,
                module_name,
                runtime_inputs,
                completion,
                helper_receipts,
                physical,
            },
        })
    }

    pub(in crate::mir) fn from_app(
        continuation: RawPostCallableMainContinuationV1,
        module_name: Box<str>,
        runtime_inputs: RawRuntimeInputSnapshotV1,
        completion: RawPreRootChildrenCompletionV1,
        helper_receipts: Box<[RawRootChildReceiptV1]>,
        callable_main: RawAppCallableMainOutcomeV1,
        physical: RawPostprocessPhysicalOwnerV1,
    ) -> Self {
        Self::App(RawAppPostprocessReadyInvocationV1 {
            core: RawPostprocessReadyCoreV1 {
                continuation,
                module_name,
                runtime_inputs,
                completion,
                helper_receipts,
                physical,
            },
            callable_main,
        })
    }
}

impl PostprocessStageTarget for RawPostprocessPhysicalOwnerV1 {
    fn refresh_rune_plans(&mut self) {
        self.refresh_rune_plans();
    }

    fn optimize(&mut self) -> crate::mir::optimizer_stats::OptimizationStats {
        self.optimize()
    }

    fn refresh_contracts(&mut self) -> Result<(), String> {
        self.refresh_contracts()
    }

    fn verify(
        &mut self,
        verifier: &mut crate::mir::verification::MirVerifier,
    ) -> Result<(), Box<[VerificationError]>> {
        self.verify(verifier)
    }

    fn insert_rc(&mut self) {
        self.insert_rc();
    }

    fn refresh_semantic_metadata(&mut self) {
        self.refresh_semantic_metadata();
    }

    fn canonicalize_callsites(&mut self) -> usize {
        self.canonicalize_callsites()
    }
}

impl<'a> ModulePostprocessOwnerV1<'a> {
    pub(in crate::mir) fn run_raw_ready(
        self,
        ready: RawPostprocessReadyInvocationV1,
    ) -> Result<RawPostprocessedInvocationV1, RejectedRawPostprocessInvocationV1> {
        match ready {
            RawPostprocessReadyInvocationV1::Script(ready) => run_script_ready(self, ready),
            RawPostprocessReadyInvocationV1::App(ready) => run_app_ready(self, ready),
        }
    }
}

fn map_stage_failure(
    failure: PostprocessStageFailureV1,
) -> (RawPostprocessFailureStageV1, RawPostprocessErrorV1) {
    match failure.stage {
        PostprocessFailureStageV1::Optimizer => (
            RawPostprocessFailureStageV1::Optimizer,
            match failure.error {
                super::module_postprocess::ModulePostprocessErrorV1::OptimizerDiagnostics {
                    count,
                } => RawPostprocessErrorV1::OptimizerDiagnostics { count },
                other => RawPostprocessErrorV1::ContractRefresh(format!("{other:?}")),
            },
        ),
        PostprocessFailureStageV1::ContractRefresh => (
            RawPostprocessFailureStageV1::ContractRefresh,
            match failure.error {
                super::module_postprocess::ModulePostprocessErrorV1::ContractRefresh(detail) => {
                    RawPostprocessErrorV1::ContractRefresh(detail)
                }
                other => RawPostprocessErrorV1::ContractRefresh(format!("{other:?}")),
            },
        ),
        PostprocessFailureStageV1::FinalVerification => (
            RawPostprocessFailureStageV1::FinalVerification,
            match failure.error {
                super::module_postprocess::ModulePostprocessErrorV1::FinalVerification(errors) => {
                    RawPostprocessErrorV1::FinalVerification(errors)
                }
                other => RawPostprocessErrorV1::ContractRefresh(format!("{other:?}")),
            },
        ),
    }
}

fn run_script_ready<'a>(
    owner: ModulePostprocessOwnerV1<'a>,
    ready: RawScriptPostprocessReadyInvocationV1,
) -> Result<RawPostprocessedInvocationV1, RejectedRawPostprocessInvocationV1> {
    let RawScriptPostprocessReadyInvocationV1 { core } = ready;
    let RawPostprocessReadyCoreV1 {
        continuation,
        module_name,
        runtime_inputs,
        completion,
        helper_receipts,
        mut physical,
    } = core;
    let schedule = ModulePostprocessScheduleV1::for_family(
        crate::mir::module_invocation_identity::ModuleInvocationFamilyV1::Raw,
    );
    let (verifier, optimize) = owner.into_stage_parts();
    let verification = match run_postprocess_stages(&mut physical, schedule, verifier, optimize) {
        Ok(verification) => verification,
        Err(failure) => {
            let (stage, error) = map_stage_failure(failure);
            return Err(RejectedRawPostprocessInvocationV1 {
                owner: RawPostprocessReadyInvocationV1::from_script(
                    continuation,
                    module_name,
                    runtime_inputs,
                    completion,
                    helper_receipts,
                    physical,
                ),
                stage,
                error,
                verification: None,
            });
        }
    };
    let postprocess_parity = match physical.prepare_parity(&module_name) {
        Ok(parity) => parity,
        Err(error) => {
            return Err(RejectedRawPostprocessInvocationV1 {
                owner: RawPostprocessReadyInvocationV1::from_script(
                    continuation,
                    module_name,
                    runtime_inputs,
                    completion,
                    helper_receipts,
                    physical,
                ),
                stage: RawPostprocessFailureStageV1::CarrierParity,
                error: RawPostprocessErrorV1::CarrierParity(error),
                verification: Some(verification),
            });
        }
    };
    let route = RawPostprocessRouteEvidenceV1::Script {
        continuation,
        module_name,
        runtime_inputs,
        completion,
        helpers: helper_receipts,
    };
    let physical = physical.finish(postprocess_parity);
    let stage_evidence = RawPostprocessStageEvidenceV1 {
        route,
        schedule,
        verification,
        progress: physical.progress(),
    };
    Ok(RawPostprocessedInvocationV1::Script(
        RawScriptPostprocessedInvocationV1 {
            core: RawPostprocessedInvocationCoreV1 {
                physical,
                stage_evidence,
            },
        },
    ))
}

fn run_app_ready<'a>(
    owner: ModulePostprocessOwnerV1<'a>,
    ready: RawAppPostprocessReadyInvocationV1,
) -> Result<RawPostprocessedInvocationV1, RejectedRawPostprocessInvocationV1> {
    let RawAppPostprocessReadyInvocationV1 {
        core,
        callable_main,
    } = ready;
    let RawPostprocessReadyCoreV1 {
        continuation,
        module_name,
        runtime_inputs,
        completion,
        helper_receipts,
        mut physical,
    } = core;
    let schedule = ModulePostprocessScheduleV1::for_family(
        crate::mir::module_invocation_identity::ModuleInvocationFamilyV1::Raw,
    );
    let (verifier, optimize) = owner.into_stage_parts();
    let verification = match run_postprocess_stages(&mut physical, schedule, verifier, optimize) {
        Ok(verification) => verification,
        Err(failure) => {
            let (stage, error) = map_stage_failure(failure);
            return Err(RejectedRawPostprocessInvocationV1 {
                owner: RawPostprocessReadyInvocationV1::from_app(
                    continuation,
                    module_name,
                    runtime_inputs,
                    completion,
                    helper_receipts,
                    callable_main,
                    physical,
                ),
                stage,
                error,
                verification: None,
            });
        }
    };
    let postprocess_parity = match physical.prepare_parity(&module_name) {
        Ok(parity) => parity,
        Err(error) => {
            return Err(RejectedRawPostprocessInvocationV1 {
                owner: RawPostprocessReadyInvocationV1::from_app(
                    continuation,
                    module_name,
                    runtime_inputs,
                    completion,
                    helper_receipts,
                    callable_main,
                    physical,
                ),
                stage: RawPostprocessFailureStageV1::CarrierParity,
                error: RawPostprocessErrorV1::CarrierParity(error),
                verification: Some(verification),
            });
        }
    };
    let route = RawPostprocessRouteEvidenceV1::App {
        continuation,
        module_name,
        runtime_inputs,
        completion,
        helpers: helper_receipts,
        callable_main,
    };
    let physical = physical.finish(postprocess_parity);
    let stage_evidence = RawPostprocessStageEvidenceV1 {
        route,
        schedule,
        verification,
        progress: physical.progress(),
    };
    Ok(RawPostprocessedInvocationV1::App(
        RawAppPostprocessedInvocationV1 {
            core: RawPostprocessedInvocationCoreV1 {
                physical,
                stage_evidence,
            },
        },
    ))
}
