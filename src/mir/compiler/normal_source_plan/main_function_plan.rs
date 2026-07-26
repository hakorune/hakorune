//! Explicit Main-role entry into the shared canonical F1 preflight.

use crate::mir::compiler::capability::{
    CanonicalLoweringPreflightV1, CanonicalTrivialBindingSsaPlanV1, ResolvedOwnerHeaderSealErrorV1,
    VerifiedResolvedOwnerHeaderV1,
};
use crate::mir::compiler::lowering_input::CanonicalLoweringErrorV1;
use crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1;
use crate::mir::resolved_value_profile::product::TrivialTerminalProfileV1;

use super::main_resolved_source::{
    VerifiedNormalMainResolvedSourceUnitV1, VerifiedNormalMainRoleV1,
};

pub(crate) struct NormalMainFunctionPreflightV1;

#[derive(Debug)]
pub(crate) enum NormalMainFunctionPlanErrorV1 {
    FunctionInput(CanonicalLoweringErrorV1),
    CanonicalPreflight(CanonicalLoweringErrorV1),
}

#[derive(Debug)]
pub(crate) struct VerifiedNormalMainFunctionPlanV1<'unit> {
    unit: &'unit VerifiedNormalMainResolvedSourceUnitV1,
    lowering: CanonicalTrivialBindingSsaPlanV1<'unit>,
    role: VerifiedNormalMainRoleV1,
    _seal: VerifiedNormalMainFunctionPlanSealV1,
}

#[derive(Debug)]
struct VerifiedNormalMainFunctionPlanSealV1;

impl NormalMainFunctionPreflightV1 {
    pub(crate) fn seal(
        unit: &VerifiedNormalMainResolvedSourceUnitV1,
    ) -> Result<VerifiedNormalMainFunctionPlanV1<'_>, RejectedNormalMainFunctionPlanV1<'_>> {
        let function =
            unit.borrow_function_input()
                .map_err(|error| RejectedNormalMainFunctionPlanV1 {
                    owner: unit,
                    error: NormalMainFunctionPlanErrorV1::FunctionInput(error),
                })?;
        let role = unit.role();
        let lowering = CanonicalLoweringPreflightV1::verify_normal_main0_function_v1(
            function, role,
        )
        .map_err(|error| RejectedNormalMainFunctionPlanV1 {
            owner: unit,
            error: NormalMainFunctionPlanErrorV1::CanonicalPreflight(error),
        })?;
        Ok(VerifiedNormalMainFunctionPlanV1 {
            unit,
            lowering,
            role,
            _seal: VerifiedNormalMainFunctionPlanSealV1,
        })
    }
}

impl<'unit> VerifiedNormalMainFunctionPlanV1<'unit> {
    pub(crate) fn completion(&self) -> &VerifiedFunctionCompletionV1 {
        self.lowering.completion()
    }

    pub(crate) const fn role(&self) -> VerifiedNormalMainRoleV1 {
        self.role
    }

    pub(crate) fn seal_source_header(
        &self,
    ) -> Result<VerifiedResolvedOwnerHeaderV1, ResolvedOwnerHeaderSealErrorV1> {
        self.lowering.seal_resolved_owner_header_v1()
    }

    pub(crate) fn terminal_profile(&self) -> &TrivialTerminalProfileV1 {
        self.lowering.terminal_profile()
    }

    pub(crate) fn into_lowering(self) -> CanonicalTrivialBindingSsaPlanV1<'unit> {
        self.lowering
    }

    #[cfg(test)]
    fn owner_for_test(&self) -> &VerifiedNormalMainResolvedSourceUnitV1 {
        self.unit
    }
}

#[derive(Debug)]
pub(crate) struct RejectedNormalMainFunctionPlanV1<'unit> {
    owner: &'unit VerifiedNormalMainResolvedSourceUnitV1,
    error: NormalMainFunctionPlanErrorV1,
}

impl RejectedNormalMainFunctionPlanV1<'_> {
    pub(crate) fn error(&self) -> &NormalMainFunctionPlanErrorV1 {
        &self.error
    }

    pub(crate) fn discard(self) {
        drop(self);
    }

    #[cfg(test)]
    fn owner_for_test(&self) -> &VerifiedNormalMainResolvedSourceUnitV1 {
        self.owner
    }
}

#[cfg(test)]
#[path = "main_function_plan_tests.rs"]
mod tests;
