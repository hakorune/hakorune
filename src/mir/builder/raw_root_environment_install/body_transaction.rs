//! Raw root BODY transaction owned by an installed environment.
//!
//! This child module is a behavior-neutral extraction from the environment
//! installation boundary. It owns the BODY drive, completion handoff, and
//! discard-only rejection products; environment installation remains in the
//! parent module.

use super::super::module_invocation_session::ModuleBuilderInvocationSessionV1;
use super::super::raw_root_body_exit::{
    PreparedRawScriptCompletionAdapterV1, RawOpenRootFunctionV1, RawRootBodyExitSealErrorV1,
    RawRootBodyExitWitnessV1,
};
use super::super::raw_root_physical::{
    RawRootBodyPhysicalDriveV1, RawRootBodyPhysicalErrorV1, RawRootPhysicalStateV1,
    RawRootPostBodyPhysicalStateV1,
};
use super::super::root_batch_slot::RawRootBatchSlotV1;
use super::super::root_body_completion::{CompletedRootBodyV1, RootBodyResultV1};
use super::super::script_physical_exit::{
    LoweredScriptTerminalV1, PreparedScriptPhysicalExitCoreV1, ScriptPhysicalExitCommitV1,
    ScriptPhysicalExitErrorV1, ScriptPhysicalExitOpenContractV1,
};
use super::InstalledRawRootEnvironmentV1;
use crate::mir::raw_root_body_recipe::{
    RawRootBodyRecipeV1, RawRootBodyRouteV1, RawScriptUnitOriginV1,
};
use crate::mir::{MirBuilder, MirFunction, MirType};

#[derive(Debug)]
pub(in crate::mir) struct CompletedRawRootBodyPhysicalV1 {
    session: ModuleBuilderInvocationSessionV1,
    physical: RawRootPostBodyPhysicalStateV1,
    draft: MirFunction,
    completion: CompletedRootBodyV1,
    exit: RawRootBodyExitWitnessV1,
}

#[derive(Debug)]
enum RawRootBodyRejectedOwnerV1 {
    BeforeDrive {
        session: ModuleBuilderInvocationSessionV1,
        physical: RawRootPhysicalStateV1,
        recipe: RawRootBodyRecipeV1,
    },
    DuringDrive {
        session: ModuleBuilderInvocationSessionV1,
        physical: RawRootBodyPhysicalDriveV1,
        recipe: RawRootBodyRecipeV1,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum RawRootBodyLoweringErrorV1 {
    Physical(RawRootBodyPhysicalErrorV1),
    Lower(String),
    ScriptExit(ScriptPhysicalExitErrorV1),
    ExitSeal(RawRootBodyExitSealErrorV1),
}

#[derive(Debug)]
pub(in crate::mir) struct RejectedRawRootBodyPhysicalV1 {
    owner: RawRootBodyRejectedOwnerV1,
    error: RawRootBodyLoweringErrorV1,
}

#[derive(Debug)]
enum RawBodyLoweringResultV1 {
    Script(LoweredScriptTerminalV1),
    App(RootBodyResultV1),
}

impl InstalledRawRootEnvironmentV1 {
    /// BODY0 paired terminal. It owns the candidate Builder session and
    /// physical carrier together, while the physical subproduct keeps the
    /// collector/ledger untouched and only advances the root-body tracker.
    pub(in crate::mir) fn drive_root_body(
        self,
        recipe: RawRootBodyRecipeV1,
    ) -> Result<CompletedRawRootBodyPhysicalV1, RejectedRawRootBodyPhysicalV1> {
        let Self {
            mut session,
            physical,
            _seal: _,
        } = self;
        let mut physical = match physical.begin_root_body() {
            Ok(physical) => physical,
            Err((physical, error)) => {
                return Err(RejectedRawRootBodyPhysicalV1 {
                    owner: RawRootBodyRejectedOwnerV1::BeforeDrive {
                        session,
                        physical,
                        recipe,
                    },
                    error: RawRootBodyLoweringErrorV1::Physical(error),
                });
            }
        };
        let open: RawOpenRootFunctionV1 = {
            let builder = session.builder_mut();
            match builder
                .begin_raw_root_function_v1(RawRootBatchSlotV1::Main.contract(), *recipe.entry())
            {
                Ok(open) => open,
                Err(error) => {
                    return Err(RejectedRawRootBodyPhysicalV1 {
                        owner: RawRootBodyRejectedOwnerV1::DuringDrive {
                            session,
                            physical,
                            recipe,
                        },
                        error: RawRootBodyLoweringErrorV1::ExitSeal(error),
                    });
                }
            }
        };
        let lower_result = {
            let builder = session.builder_mut();
            let _scope = super::super::vars::lexical_scope::LexicalScopeGuard::new(builder);
            match recipe.script() {
                Some(script) => builder
                    .lower_script_body_recipe_v1(script)
                    .map(RawBodyLoweringResultV1::Script)
                    .map_err(|error| error.to_string()),
                None => builder
                    .lower_linear_scalar_recipe_v1(&recipe)
                    .map(|result| match recipe.entry().route() {
                        RawRootBodyRouteV1::Script => RawBodyLoweringResultV1::Script(
                            legacy_script_terminal_from_root_result(builder, result),
                        ),
                        RawRootBodyRouteV1::AppMain0 { .. } => RawBodyLoweringResultV1::App(result),
                    }),
            }
        };
        let lowered = match lower_result {
            Ok(lowered) => lowered,
            Err(error) => {
                return Err(RejectedRawRootBodyPhysicalV1 {
                    owner: RawRootBodyRejectedOwnerV1::DuringDrive {
                        session,
                        physical,
                        recipe,
                    },
                    error: RawRootBodyLoweringErrorV1::Lower(error),
                });
            }
        };
        let brand = physical.brand();
        let (draft, exit, completion_plan) = match lowered {
            RawBodyLoweringResultV1::Script(terminal) => {
                let prepared = match PreparedScriptPhysicalExitCoreV1::prepare(
                    session.builder(),
                    terminal,
                    ScriptPhysicalExitOpenContractV1::ProvisionalUnknown,
                ) {
                    Ok(prepared) => prepared,
                    Err(error) => {
                        return Err(RejectedRawRootBodyPhysicalV1 {
                            owner: RawRootBodyRejectedOwnerV1::DuringDrive {
                                session,
                                physical,
                                recipe,
                            },
                            error: RawRootBodyLoweringErrorV1::ScriptExit(error),
                        });
                    }
                };
                let completion_adapter =
                    match PreparedRawScriptCompletionAdapterV1::prepare(prepared.completion()) {
                        Ok(adapter) => adapter,
                        Err(error) => {
                            return Err(RejectedRawRootBodyPhysicalV1 {
                                owner: RawRootBodyRejectedOwnerV1::DuringDrive {
                                    session,
                                    physical,
                                    recipe,
                                },
                                error: RawRootBodyLoweringErrorV1::ExitSeal(error),
                            });
                        }
                    };
                let completion_plan = match physical
                    .prepare_root_body_completion(completion_adapter.tracker_result())
                {
                    Ok(plan) => plan,
                    Err(error) => {
                        return Err(RejectedRawRootBodyPhysicalV1 {
                            owner: RawRootBodyRejectedOwnerV1::DuringDrive {
                                session,
                                physical,
                                recipe,
                            },
                            error: RawRootBodyLoweringErrorV1::Physical(
                                RawRootBodyPhysicalErrorV1::SealTracker(error),
                            ),
                        });
                    }
                };
                let completed =
                    ScriptPhysicalExitCommitV1::commit_projected(session.builder_mut(), prepared);
                let (draft, exit) = session
                    .builder_mut()
                    .commit_raw_script_exit_v1(open, completed, brand);
                (draft, exit, completion_plan)
            }
            RawBodyLoweringResultV1::App(result) => {
                let completion_plan =
                    match physical.prepare_root_body_completion(RootBodyResultV1::NoValue) {
                        Ok(plan) => plan,
                        Err(error) => {
                            return Err(RejectedRawRootBodyPhysicalV1 {
                                owner: RawRootBodyRejectedOwnerV1::DuringDrive {
                                    session,
                                    physical,
                                    recipe,
                                },
                                error: RawRootBodyLoweringErrorV1::Physical(
                                    RawRootBodyPhysicalErrorV1::SealTracker(error),
                                ),
                            });
                        }
                    };
                let plan = match session.builder().prepare_raw_root_exit_v1(
                    &open,
                    result,
                    physical.tracker(),
                ) {
                    Ok(plan) => plan,
                    Err(error) => {
                        return Err(RejectedRawRootBodyPhysicalV1 {
                            owner: RawRootBodyRejectedOwnerV1::DuringDrive {
                                session,
                                physical,
                                recipe,
                            },
                            error: RawRootBodyLoweringErrorV1::ExitSeal(error),
                        });
                    }
                };
                let (draft, exit) = session
                    .builder_mut()
                    .commit_raw_root_exit_v1(open, plan, brand);
                (draft, exit, completion_plan)
            }
        };
        let (physical, completion) = physical.seal_root_body_prepared(completion_plan);
        Ok(CompletedRawRootBodyPhysicalV1 {
            session,
            physical,
            draft,
            completion,
            exit,
        })
    }
}

/// Legacy Raw recipes without a source-classified Script payload still enter
/// the shared physical exit kernel. This bridge preserves the old root result
/// only until RAW-SCRIPT-EXIT-ADAPTER0 removes the unclassified recipe shape.
fn legacy_script_terminal_from_root_result(
    builder: &MirBuilder,
    result: RootBodyResultV1,
) -> LoweredScriptTerminalV1 {
    match result {
        RootBodyResultV1::Value(value) if builder.value_type(value) == Some(&MirType::Void) => {
            LoweredScriptTerminalV1::Unit {
                origin: RawScriptUnitOriginV1::EmptyBody,
                payload:
                    super::super::script_physical_exit::LoweredScriptUnitPayloadV1::ExistingVoid {
                        value,
                    },
            }
        }
        RootBodyResultV1::Value(value) => LoweredScriptTerminalV1::Value { value },
        RootBodyResultV1::NoValue => LoweredScriptTerminalV1::Unit {
            origin: RawScriptUnitOriginV1::EmptyBody,
            payload: super::super::script_physical_exit::LoweredScriptUnitPayloadV1::SyntheticVoid,
        },
    }
}

impl CompletedRawRootBodyPhysicalV1 {
    pub(in crate::mir) fn into_raw_root_batch_input(
        self,
    ) -> super::super::raw_root_physical::root_batch_terminal::RawRootBatchPhysicalInputV1 {
        let (session, physical, draft, completion, exit) = self.into_parts();
        super::super::raw_root_physical::root_batch_terminal::RawRootBatchPhysicalInputV1 {
            session,
            physical,
            draft,
            completion,
            exit,
        }
    }

    pub(in crate::mir) fn into_parts(
        self,
    ) -> (
        ModuleBuilderInvocationSessionV1,
        RawRootPostBodyPhysicalStateV1,
        MirFunction,
        CompletedRootBodyV1,
        RawRootBodyExitWitnessV1,
    ) {
        (
            self.session,
            self.physical,
            self.draft,
            self.completion,
            self.exit,
        )
    }
}

impl RejectedRawRootBodyPhysicalV1 {
    pub(in crate::mir) fn error(&self) -> &RawRootBodyLoweringErrorV1 {
        &self.error
    }

    pub(in crate::mir) fn discard(self) {}
}
