//! DECLACCESS0-S0: one-shot Raw root environment installation.
//!
//! This is the compiler-side terminal over the Builder-owned co-install
//! product.  It consumes one CALLMAIN0 ready owner, performs no source scan,
//! and exposes only the route-specific declared owner on success.

use super::raw_root_callable_main::{
    RawAppCallableMainOutcomeV1, RawCallableMainReadyEnvironmentPartsV1,
    RawCallableMainReadyInvocationV1,
};
use super::raw_root_children::{RawPreRootChildrenCompletionV1, RawRootChildReceiptV1};
use super::raw_root_environment_manifest::RawRootPostInstallManifestV1;
use super::raw_root_plan0::RawPostCallableMainPlanV1;
use super::raw_runtime_inputs::RawRuntimeInputSnapshotV1;
use super::raw_source_binding::RawPostCallableMainContinuationV1;
use crate::mir::builder::OwnedRawSourceV1;
use crate::mir::builder::{
    CompletedRawRootBatchPhysicalV1, RawRootBatchPhysicalErrorV1, RejectedRawRootBatchPhysicalV1,
};
use crate::mir::builder::{
    CompletedRawRootBodyPhysicalV1, InstalledRawRootEnvironmentV1,
    ModuleBuilderInvocationSessionV1, RawRootBodyLoweringErrorV1, RawRootEnvironmentInstallErrorV1,
    RawRootEnvironmentInstallOwnerV1, RawRootPhysicalStateV1, RejectedRawRootBodyPhysicalV1,
    RejectedRawRootEnvironmentInstallV1,
};
use crate::mir::module_invocation_identity::ModuleInvocationTokenV1;
use crate::mir::raw_root_body_recipe::RawRootBodyEntryV1;

#[derive(Debug)]
pub(in crate::mir) enum DeclaredRawRootInvocationV1 {
    Script(DeclaredRawScriptRootInvocationV1),
    App(DeclaredRawAppRootInvocationV1),
}

#[derive(Debug)]
pub(in crate::mir) struct DeclaredRawScriptRootInvocationV1 {
    core: DeclaredRawRootCoreV1,
}

#[derive(Debug)]
pub(in crate::mir) struct DeclaredRawAppRootInvocationV1 {
    core: DeclaredRawRootCoreV1,
    callable_main: RawAppCallableMainOutcomeV1,
}

#[derive(Debug)]
struct DeclaredRawRootCoreV1 {
    token: ModuleInvocationTokenV1,
    continuation: RawPostCallableMainContinuationV1,
    module_name: Box<str>,
    completion: RawPreRootChildrenCompletionV1,
    helper_receipts: Box<[RawRootChildReceiptV1]>,
    installed: InstalledRawRootEnvironmentV1,
    post_install_manifest: RawRootPostInstallManifestV1,
}

#[derive(Debug)]
enum RejectedRawRootEnvironmentOwnerV1 {
    Script(RejectedRawScriptRootEnvironmentOwnerV1),
    App(RejectedRawAppRootEnvironmentOwnerV1),
}

#[derive(Debug)]
struct RejectedRawScriptRootEnvironmentOwnerV1 {
    token: ModuleInvocationTokenV1,
    source: OwnedRawSourceV1,
    continuation: RawPostCallableMainContinuationV1,
    module_name: Box<str>,
    plan: RawPostCallableMainPlanV1,
    completion: RawPreRootChildrenCompletionV1,
    helper_receipts: Box<[RawRootChildReceiptV1]>,
    post_install_manifest: RawRootPostInstallManifestV1,
    install: RejectedRawRootEnvironmentInstallV1,
}

#[derive(Debug)]
struct RejectedRawAppRootEnvironmentOwnerV1 {
    token: ModuleInvocationTokenV1,
    source: OwnedRawSourceV1,
    continuation: RawPostCallableMainContinuationV1,
    module_name: Box<str>,
    plan: RawPostCallableMainPlanV1,
    completion: RawPreRootChildrenCompletionV1,
    helper_receipts: Box<[RawRootChildReceiptV1]>,
    callable_main: RawAppCallableMainOutcomeV1,
    post_install_manifest: RawRootPostInstallManifestV1,
    install: RejectedRawRootEnvironmentInstallV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum RawRootEnvironmentFailureStageV1 {
    Install,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum RawRootEnvironmentErrorV1 {
    Install(RawRootEnvironmentInstallErrorV1),
}

#[derive(Debug)]
pub(in crate::mir) struct RejectedRawRootEnvironmentInvocationV1 {
    owner: RejectedRawRootEnvironmentOwnerV1,
    stage: RawRootEnvironmentFailureStageV1,
    error: RawRootEnvironmentErrorV1,
}

#[derive(Debug)]
pub(in crate::mir) enum RawRootBodyCompleteInvocationV1 {
    Script(RawScriptRootBodyCompleteInvocationV1),
    App(RawAppRootBodyCompleteInvocationV1),
}

#[derive(Debug)]
pub(in crate::mir) struct RawScriptRootBodyCompleteInvocationV1 {
    core: RawRootBodyCompleteCoreV1,
}

#[derive(Debug)]
pub(in crate::mir) struct RawAppRootBodyCompleteInvocationV1 {
    core: RawRootBodyCompleteCoreV1,
    callable_main: RawAppCallableMainOutcomeV1,
}

/// The only successful compiler-side ROOTBATCH0 handoff.  The physical
/// terminal has already paired collector receipts with the sealed ledger;
/// no shell publication occurs here.
#[derive(Debug)]
pub(in crate::mir) enum RawRootBatchCompleteInvocationV1 {
    Script(RawScriptRootBatchCompleteInvocationV1),
    App(RawAppRootBatchCompleteInvocationV1),
}

#[derive(Debug)]
pub(in crate::mir) struct RawScriptRootBatchCompleteInvocationV1 {
    core: RawRootBatchCompleteCoreV1,
}

#[derive(Debug)]
pub(in crate::mir) struct RawAppRootBatchCompleteInvocationV1 {
    core: RawRootBatchCompleteCoreV1,
    callable_main: RawAppCallableMainOutcomeV1,
}

#[derive(Debug)]
struct RawRootBatchCompleteCoreV1 {
    continuation: RawPostCallableMainContinuationV1,
    module_name: Box<str>,
    runtime_inputs: RawRuntimeInputSnapshotV1,
    completion: RawPreRootChildrenCompletionV1,
    helper_receipts: Box<[RawRootChildReceiptV1]>,
    physical: CompletedRawRootBatchPhysicalV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum RawRootBatchFailureStageV1 {
    Physical,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum RawRootBatchErrorV1 {
    Physical(RawRootBatchPhysicalErrorV1),
}

#[derive(Debug)]
enum RejectedRawRootBatchOwnerV1 {
    Script {
        continuation: RawPostCallableMainContinuationV1,
        module_name: Box<str>,
        runtime_inputs: RawRuntimeInputSnapshotV1,
        completion: RawPreRootChildrenCompletionV1,
        helper_receipts: Box<[RawRootChildReceiptV1]>,
        physical: RejectedRawRootBatchPhysicalV1,
    },
    App {
        continuation: RawPostCallableMainContinuationV1,
        module_name: Box<str>,
        runtime_inputs: RawRuntimeInputSnapshotV1,
        completion: RawPreRootChildrenCompletionV1,
        helper_receipts: Box<[RawRootChildReceiptV1]>,
        callable_main: RawAppCallableMainOutcomeV1,
        physical: RejectedRawRootBatchPhysicalV1,
    },
}

#[derive(Debug)]
pub(in crate::mir) struct RejectedRawRootBatchInvocationV1 {
    owner: RejectedRawRootBatchOwnerV1,
    stage: RawRootBatchFailureStageV1,
    error: RawRootBatchErrorV1,
}

#[derive(Debug)]
struct RawRootBodyCompleteCoreV1 {
    token: ModuleInvocationTokenV1,
    continuation: RawPostCallableMainContinuationV1,
    module_name: Box<str>,
    runtime_inputs: RawRuntimeInputSnapshotV1,
    completion: RawPreRootChildrenCompletionV1,
    helper_receipts: Box<[RawRootChildReceiptV1]>,
    body: CompletedRawRootBodyPhysicalV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum RawRootBodyFailureStageV1 {
    Preflight,
    Lower,
    Finalize,
    Seal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum RawRootBodyErrorV1 {
    RouteRecipeMismatch,
    Physical(RawRootBodyLoweringErrorV1),
}

#[derive(Debug)]
enum RejectedRawRootBodyOwnerV1 {
    ScriptPreflight {
        token: ModuleInvocationTokenV1,
        continuation: RawPostCallableMainContinuationV1,
        module_name: Box<str>,
        runtime_inputs: RawRuntimeInputSnapshotV1,
        completion: RawPreRootChildrenCompletionV1,
        helper_receipts: Box<[RawRootChildReceiptV1]>,
        physical: InstalledRawRootEnvironmentV1,
    },
    Script {
        token: ModuleInvocationTokenV1,
        continuation: RawPostCallableMainContinuationV1,
        module_name: Box<str>,
        runtime_inputs: RawRuntimeInputSnapshotV1,
        completion: RawPreRootChildrenCompletionV1,
        helper_receipts: Box<[RawRootChildReceiptV1]>,
        physical: RejectedRawRootBodyPhysicalV1,
    },
    AppPreflight {
        token: ModuleInvocationTokenV1,
        continuation: RawPostCallableMainContinuationV1,
        module_name: Box<str>,
        runtime_inputs: RawRuntimeInputSnapshotV1,
        completion: RawPreRootChildrenCompletionV1,
        helper_receipts: Box<[RawRootChildReceiptV1]>,
        callable_main: RawAppCallableMainOutcomeV1,
        physical: InstalledRawRootEnvironmentV1,
    },
    App {
        token: ModuleInvocationTokenV1,
        continuation: RawPostCallableMainContinuationV1,
        module_name: Box<str>,
        runtime_inputs: RawRuntimeInputSnapshotV1,
        completion: RawPreRootChildrenCompletionV1,
        helper_receipts: Box<[RawRootChildReceiptV1]>,
        callable_main: RawAppCallableMainOutcomeV1,
        physical: RejectedRawRootBodyPhysicalV1,
    },
}

#[derive(Debug)]
pub(in crate::mir) struct RejectedRawRootBodyInvocationV1 {
    owner: RejectedRawRootBodyOwnerV1,
    stage: RawRootBodyFailureStageV1,
    error: RawRootBodyErrorV1,
}

impl RawRootBodyCompleteInvocationV1 {
    /// The sole compiler-visible ROOTBATCH0 entry.  The Builder sibling
    /// consumes the already paired BODY0 physical owner; no shell/collector
    /// parts are unpacked in this module.
    pub(in crate::mir) fn prepare_root_batch(
        self,
    ) -> Result<RawRootBatchCompleteInvocationV1, RejectedRawRootBatchInvocationV1> {
        match self {
            Self::Script(wrapper) => {
                let RawRootBodyCompleteCoreV1 {
                    token,
                    continuation,
                    module_name,
                    runtime_inputs,
                    completion,
                    helper_receipts,
                    body,
                } = wrapper.core;
                match body
                    .into_raw_root_batch_input()
                    .prepare_raw_root_batch(token)
                {
                    Ok(physical) => Ok(RawRootBatchCompleteInvocationV1::Script(
                        RawScriptRootBatchCompleteInvocationV1 {
                            core: RawRootBatchCompleteCoreV1 {
                                continuation,
                                module_name,
                                runtime_inputs,
                                completion,
                                helper_receipts,
                                physical,
                            },
                        },
                    )),
                    Err(rejected) => {
                        let error = rejected.error().clone();
                        Err(RejectedRawRootBatchInvocationV1 {
                            owner: RejectedRawRootBatchOwnerV1::Script {
                                continuation,
                                module_name,
                                runtime_inputs,
                                completion,
                                helper_receipts,
                                physical: rejected,
                            },
                            stage: RawRootBatchFailureStageV1::Physical,
                            error: RawRootBatchErrorV1::Physical(error),
                        })
                    }
                }
            }
            Self::App(wrapper) => {
                let RawRootBodyCompleteCoreV1 {
                    token,
                    continuation,
                    module_name,
                    runtime_inputs,
                    completion,
                    helper_receipts,
                    body,
                } = wrapper.core;
                let callable_main = wrapper.callable_main;
                match body
                    .into_raw_root_batch_input()
                    .prepare_raw_root_batch(token)
                {
                    Ok(physical) => Ok(RawRootBatchCompleteInvocationV1::App(
                        RawAppRootBatchCompleteInvocationV1 {
                            core: RawRootBatchCompleteCoreV1 {
                                continuation,
                                module_name,
                                runtime_inputs,
                                completion,
                                helper_receipts,
                                physical,
                            },
                            callable_main,
                        },
                    )),
                    Err(rejected) => {
                        let error = rejected.error().clone();
                        Err(RejectedRawRootBatchInvocationV1 {
                            owner: RejectedRawRootBatchOwnerV1::App {
                                continuation,
                                module_name,
                                runtime_inputs,
                                completion,
                                helper_receipts,
                                callable_main,
                                physical: rejected,
                            },
                            stage: RawRootBatchFailureStageV1::Physical,
                            error: RawRootBatchErrorV1::Physical(error),
                        })
                    }
                }
            }
        }
    }
}

impl RejectedRawRootBatchInvocationV1 {
    pub(in crate::mir) const fn stage(&self) -> RawRootBatchFailureStageV1 {
        self.stage
    }

    pub(in crate::mir) fn error(&self) -> &RawRootBatchErrorV1 {
        &self.error
    }

    pub(in crate::mir) fn discard(self) {}
}

impl DeclaredRawRootInvocationV1 {
    /// The sole BODY0 compiler entry.  The recipe was sealed before physical
    /// opening; this terminal only consumes it and the paired installed owner.
    pub(in crate::mir) fn begin_body(
        self,
    ) -> Result<RawRootBodyCompleteInvocationV1, RejectedRawRootBodyInvocationV1> {
        match self {
            Self::Script(ready) => begin_script_body(ready.core),
            Self::App(ready) => begin_app_body(ready.core, ready.callable_main),
        }
    }
}

fn begin_script_body(
    core: DeclaredRawRootCoreV1,
) -> Result<RawRootBodyCompleteInvocationV1, RejectedRawRootBodyInvocationV1> {
    let DeclaredRawRootCoreV1 {
        token,
        continuation,
        module_name,
        completion,
        helper_receipts,
        installed,
        post_install_manifest,
    } = core;
    let (recipe, runtime_inputs) = post_install_manifest.into_body_parts();
    if !matches!(recipe.entry(), RawRootBodyEntryV1::Script) {
        return Err(RejectedRawRootBodyInvocationV1 {
            owner: RejectedRawRootBodyOwnerV1::ScriptPreflight {
                token,
                continuation,
                module_name,
                runtime_inputs,
                completion,
                helper_receipts,
                physical: installed,
            },
            stage: RawRootBodyFailureStageV1::Preflight,
            error: RawRootBodyErrorV1::RouteRecipeMismatch,
        });
    }
    match installed.drive_root_body(recipe) {
        Ok(body) => Ok(RawRootBodyCompleteInvocationV1::Script(
            RawScriptRootBodyCompleteInvocationV1 {
                core: RawRootBodyCompleteCoreV1 {
                    token,
                    continuation,
                    module_name,
                    runtime_inputs,
                    completion,
                    helper_receipts,
                    body,
                },
            },
        )),
        Err(rejected) => {
            let error = rejected.error().clone();
            Err(RejectedRawRootBodyInvocationV1 {
                owner: RejectedRawRootBodyOwnerV1::Script {
                    token,
                    continuation,
                    module_name,
                    runtime_inputs,
                    completion,
                    helper_receipts,
                    physical: rejected,
                },
                stage: body_failure_stage(&error),
                error: RawRootBodyErrorV1::Physical(error),
            })
        }
    }
}

fn begin_app_body(
    core: DeclaredRawRootCoreV1,
    callable_main: RawAppCallableMainOutcomeV1,
) -> Result<RawRootBodyCompleteInvocationV1, RejectedRawRootBodyInvocationV1> {
    let DeclaredRawRootCoreV1 {
        token,
        continuation,
        module_name,
        completion,
        helper_receipts,
        installed,
        post_install_manifest,
    } = core;
    let (recipe, runtime_inputs) = post_install_manifest.into_body_parts();
    if !matches!(recipe.entry(), RawRootBodyEntryV1::AppMain0Void { .. }) {
        return Err(RejectedRawRootBodyInvocationV1 {
            owner: RejectedRawRootBodyOwnerV1::AppPreflight {
                token,
                continuation,
                module_name,
                runtime_inputs,
                completion,
                helper_receipts,
                callable_main,
                physical: installed,
            },
            stage: RawRootBodyFailureStageV1::Preflight,
            error: RawRootBodyErrorV1::RouteRecipeMismatch,
        });
    }
    match installed.drive_root_body(recipe) {
        Ok(body) => Ok(RawRootBodyCompleteInvocationV1::App(
            RawAppRootBodyCompleteInvocationV1 {
                core: RawRootBodyCompleteCoreV1 {
                    token,
                    continuation,
                    module_name,
                    runtime_inputs,
                    completion,
                    helper_receipts,
                    body,
                },
                callable_main,
            },
        )),
        Err(rejected) => {
            let error = rejected.error().clone();
            Err(RejectedRawRootBodyInvocationV1 {
                owner: RejectedRawRootBodyOwnerV1::App {
                    token,
                    continuation,
                    module_name,
                    runtime_inputs,
                    completion,
                    helper_receipts,
                    callable_main,
                    physical: rejected,
                },
                stage: body_failure_stage(&error),
                error: RawRootBodyErrorV1::Physical(error),
            })
        }
    }
}

fn body_failure_stage(error: &RawRootBodyLoweringErrorV1) -> RawRootBodyFailureStageV1 {
    match error {
        RawRootBodyLoweringErrorV1::Physical(
            crate::mir::builder::RawRootBodyPhysicalErrorV1::BeginTracker(_),
        ) => RawRootBodyFailureStageV1::Preflight,
        RawRootBodyLoweringErrorV1::Physical(
            crate::mir::builder::RawRootBodyPhysicalErrorV1::SealTracker(_),
        ) => RawRootBodyFailureStageV1::Seal,
        RawRootBodyLoweringErrorV1::Lower(_) => RawRootBodyFailureStageV1::Lower,
        RawRootBodyLoweringErrorV1::Finalize(_) => RawRootBodyFailureStageV1::Finalize,
    }
}

impl RejectedRawRootBodyInvocationV1 {
    pub(in crate::mir) const fn stage(&self) -> RawRootBodyFailureStageV1 {
        self.stage
    }

    pub(in crate::mir) fn error(&self) -> &RawRootBodyErrorV1 {
        &self.error
    }

    pub(in crate::mir) fn discard(self) {}
}

impl RawCallableMainReadyInvocationV1 {
    pub(in crate::mir) fn declare_environment(
        self,
    ) -> Result<DeclaredRawRootInvocationV1, RejectedRawRootEnvironmentInvocationV1> {
        match self.into_environment_parts() {
            RawCallableMainReadyEnvironmentPartsV1::Script {
                token,
                source,
                continuation,
                module_name,
                plan,
                manifest,
                session,
                physical,
                completion,
                helper_receipts,
            } => declare_script(
                token,
                source,
                continuation,
                module_name,
                plan,
                manifest,
                session,
                physical,
                completion,
                helper_receipts,
            ),
            RawCallableMainReadyEnvironmentPartsV1::App {
                token,
                source,
                continuation,
                module_name,
                plan,
                manifest,
                session,
                physical,
                completion,
                helper_receipts,
                outcome,
            } => declare_app(
                token,
                source,
                continuation,
                module_name,
                plan,
                manifest,
                session,
                physical,
                completion,
                helper_receipts,
                outcome,
            ),
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn declare_script(
    token: ModuleInvocationTokenV1,
    source: OwnedRawSourceV1,
    continuation: RawPostCallableMainContinuationV1,
    module_name: Box<str>,
    plan: RawPostCallableMainPlanV1,
    manifest: super::raw_root_environment_manifest::RawRootPhysicalManifestV1,
    session: ModuleBuilderInvocationSessionV1,
    physical: RawRootPhysicalStateV1,
    completion: RawPreRootChildrenCompletionV1,
    helper_receipts: Box<[RawRootChildReceiptV1]>,
) -> Result<DeclaredRawRootInvocationV1, RejectedRawRootEnvironmentInvocationV1> {
    let source_file = session.source_file();
    let (projection, post_install_manifest) = manifest.into_install_parts(source_file);
    let install = RawRootEnvironmentInstallOwnerV1::new(session, physical, projection);
    let prepared = match install.prepare() {
        Ok(prepared) => prepared,
        Err(rejected) => {
            let error = RawRootEnvironmentErrorV1::Install(rejected.error().clone());
            return Err(RejectedRawRootEnvironmentInvocationV1 {
                owner: RejectedRawRootEnvironmentOwnerV1::Script(
                    RejectedRawScriptRootEnvironmentOwnerV1 {
                        token,
                        source,
                        continuation,
                        module_name,
                        plan,
                        completion,
                        helper_receipts,
                        post_install_manifest,
                        install: rejected,
                    },
                ),
                stage: RawRootEnvironmentFailureStageV1::Install,
                error,
            });
        }
    };
    let installed = prepared.commit();
    drop(source);
    drop(plan);
    Ok(DeclaredRawRootInvocationV1::Script(
        DeclaredRawScriptRootInvocationV1 {
            core: DeclaredRawRootCoreV1 {
                token,
                continuation,
                module_name,
                completion,
                helper_receipts,
                installed,
                post_install_manifest,
            },
        },
    ))
}

#[allow(clippy::too_many_arguments)]
fn declare_app(
    token: ModuleInvocationTokenV1,
    source: OwnedRawSourceV1,
    continuation: RawPostCallableMainContinuationV1,
    module_name: Box<str>,
    plan: RawPostCallableMainPlanV1,
    manifest: super::raw_root_environment_manifest::RawRootPhysicalManifestV1,
    session: ModuleBuilderInvocationSessionV1,
    physical: RawRootPhysicalStateV1,
    completion: RawPreRootChildrenCompletionV1,
    helper_receipts: Box<[RawRootChildReceiptV1]>,
    callable_main: RawAppCallableMainOutcomeV1,
) -> Result<DeclaredRawRootInvocationV1, RejectedRawRootEnvironmentInvocationV1> {
    let source_file = session.source_file();
    let (projection, post_install_manifest) = manifest.into_install_parts(source_file);
    let install = RawRootEnvironmentInstallOwnerV1::new(session, physical, projection);
    let prepared = match install.prepare() {
        Ok(prepared) => prepared,
        Err(rejected) => {
            let error = RawRootEnvironmentErrorV1::Install(rejected.error().clone());
            return Err(RejectedRawRootEnvironmentInvocationV1 {
                owner: RejectedRawRootEnvironmentOwnerV1::App(
                    RejectedRawAppRootEnvironmentOwnerV1 {
                        token,
                        source,
                        continuation,
                        module_name,
                        plan,
                        completion,
                        helper_receipts,
                        callable_main,
                        post_install_manifest,
                        install: rejected,
                    },
                ),
                stage: RawRootEnvironmentFailureStageV1::Install,
                error,
            });
        }
    };
    let installed = prepared.commit();
    drop(source);
    drop(plan);
    Ok(DeclaredRawRootInvocationV1::App(
        DeclaredRawAppRootInvocationV1 {
            core: DeclaredRawRootCoreV1 {
                token,
                continuation,
                module_name,
                completion,
                helper_receipts,
                installed,
                post_install_manifest,
            },
            callable_main,
        },
    ))
}

impl RejectedRawRootEnvironmentInvocationV1 {
    pub(in crate::mir) const fn stage(&self) -> RawRootEnvironmentFailureStageV1 {
        self.stage
    }

    pub(in crate::mir) fn error(&self) -> &RawRootEnvironmentErrorV1 {
        &self.error
    }

    pub(in crate::mir) fn discard(self) {}
}

impl DeclaredRawRootInvocationV1 {
    pub(in crate::mir) fn catalog_installed(&self) -> bool {
        match self {
            Self::Script(ready) => ready.core.installed.catalog_installed(),
            Self::App(ready) => ready.core.installed.catalog_installed(),
        }
    }

    pub(in crate::mir) fn app_callable_main_selected(&self) -> bool {
        match self {
            Self::Script(_) => false,
            Self::App(ready) => ready.callable_main.is_selected(),
        }
    }

    pub(in crate::mir) fn app_callable_main_not_selected(&self) -> bool {
        match self {
            Self::Script(_) => false,
            Self::App(ready) => !ready.callable_main.is_selected(),
        }
    }

    pub(in crate::mir) fn tracker_completed_children(&self) -> usize {
        match self {
            Self::Script(ready) => ready.core.installed.tracker_completed_children(),
            Self::App(ready) => ready.core.installed.tracker_completed_children(),
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
