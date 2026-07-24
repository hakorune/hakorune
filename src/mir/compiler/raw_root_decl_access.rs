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
use super::raw_source_binding::RawPostCallableMainContinuationV1;
use crate::mir::builder::OwnedRawSourceV1;
use crate::mir::builder::{
    InstalledRawRootEnvironmentV1, ModuleBuilderInvocationSessionV1,
    RawRootEnvironmentInstallErrorV1, RawRootEnvironmentInstallOwnerV1, RawRootPhysicalStateV1,
    RejectedRawRootEnvironmentInstallV1,
};
use crate::mir::module_invocation_identity::ModuleInvocationTokenV1;

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
