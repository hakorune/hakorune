//! Raw root BODY transaction owned by an installed environment.
//!
//! This child module is a behavior-neutral extraction from the environment
//! installation boundary. It owns the BODY drive, completion handoff, and
//! discard-only rejection products; environment installation remains in the
//! parent module.

use super::super::module_invocation_session::ModuleBuilderInvocationSessionV1;
use super::super::raw_root_body_exit::{
    RawOpenRootFunctionV1, RawRootBodyExitSealErrorV1, RawRootBodyExitWitnessV1,
};
use super::super::raw_root_physical::{
    RawRootBodyPhysicalDriveV1, RawRootBodyPhysicalErrorV1, RawRootPhysicalStateV1,
    RawRootPostBodyPhysicalStateV1,
};
use super::super::root_batch_slot::RawRootBatchSlotV1;
use super::super::root_body_completion::{CompletedRootBodyV1, RootBodyResultV1};
use super::super::script_physical_exit::LoweredScriptTerminalV1;
use super::InstalledRawRootEnvironmentV1;
use crate::mir::raw_root_body_recipe::{RawRootBodyRecipeV1, RawScriptTerminalRecipeV1};
use crate::mir::MirFunction;

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
    ExitSeal(RawRootBodyExitSealErrorV1),
}

#[derive(Debug)]
pub(in crate::mir) struct RejectedRawRootBodyPhysicalV1 {
    owner: RawRootBodyRejectedOwnerV1,
    error: RawRootBodyLoweringErrorV1,
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
                    .map(legacy_root_body_result_from_script_terminal)
                    .map_err(|error| error.to_string()),
                None => builder.lower_linear_scalar_recipe_v1(&recipe),
            }
        };
        let result = match lower_result {
            Ok(result) => result,
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
        let completion_result = if recipe.script().is_some() {
            result
        } else {
            RootBodyResultV1::NoValue
        };
        let completion_plan = match physical.prepare_root_body_completion(completion_result) {
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
        let plan = match recipe.script().and_then(|script| match script.terminal() {
            RawScriptTerminalRecipeV1::UnitExpression { origin, .. } => Some(*origin),
            _ => None,
        }) {
            Some(origin) => match session.builder().prepare_raw_script_unit_exit_v1(
                &open,
                result,
                physical.tracker(),
                origin,
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
            },
            None => {
                match session
                    .builder()
                    .prepare_raw_root_exit_v1(&open, result, physical.tracker())
                {
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
                }
            }
        };
        let brand = physical.brand();
        let (draft, exit) = session
            .builder_mut()
            .commit_raw_root_exit_v1(open, plan, brand);
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

/// Temporary Raw-only adaptation retained until RAW-SCRIPT-EXIT-ADAPTER0.
/// It carries no Return authority; the exact terminal remains the shared
/// Script kernel product and Raw still owns its brand-bound tracker here.
fn legacy_root_body_result_from_script_terminal(
    terminal: LoweredScriptTerminalV1,
) -> RootBodyResultV1 {
    match terminal {
        LoweredScriptTerminalV1::Value { value }
        | LoweredScriptTerminalV1::Unit {
            payload:
                super::super::script_physical_exit::LoweredScriptUnitPayloadV1::ExistingVoid { value },
            ..
        } => RootBodyResultV1::Value(value),
        LoweredScriptTerminalV1::Unit {
            payload: super::super::script_physical_exit::LoweredScriptUnitPayloadV1::SyntheticVoid,
            ..
        } => RootBodyResultV1::NoValue,
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
