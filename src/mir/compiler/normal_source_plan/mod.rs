//! Source-owned normal compilation family classification.
//!
//! This module observes one owned parsed source exactly once. It does not
//! select a profile, lower MIR, choose a backend, or execute a program.

mod classifier;
mod compatibility_origin;
mod inventory;
mod main_function_plan;
mod main_resolved_source;
mod main_source;
mod main_thunk_plan;
mod parser_bound_policy;
mod product;
mod rejection;
#[cfg(test)]
mod test_support;

pub(crate) use classifier::NormalSourcePlanClassifierV1;
pub(crate) use compatibility_origin::{
    NormalCallableCompatibilityOriginErrorV1, NormalCallableCompatibilityOriginV1,
};
pub(crate) use main_function_plan::{NormalMainFunctionPlanErrorV1, NormalMainFunctionPreflightV1};
pub(crate) use main_resolved_source::{
    NormalMainResolvedSourceErrorV1, VerifiedNormalMainResolvedSourceUnitV1,
    VerifiedNormalMainRoleV1,
};
pub(crate) use main_source::{
    NormalMainFunctionSourceErrorV1, VerifiedNormalMainFunctionSourceUnitV1,
};
pub(crate) use main_thunk_plan::{
    NormalMainThunkPlanErrorV1,
    VerifiedNormalMainEntryRelationV1, VerifiedNormalMainPhysicalRelationV1,
    VerifiedNormalMainThunkPlanV1, VerifiedNormalMainThunkResultV1,
};
pub(in crate::mir) use parser_bound_policy::require_static_main_v1;
pub(crate) use product::{
    PreparedNormalSourcePlanInputV1, SealedNormalCallableModuleSourceV1, SealedNormalMainSourceV1,
    SealedNormalScalarRootV1, SealedNormalScriptSourceV1, SealedNormalSourcePlanV1,
};
#[cfg(test)]
pub(crate) use rejection::NormalUnsupportedTopLevelKindV1;
pub(crate) use rejection::{
    NormalSourcePlanErrorV1, NormalSourcePlanIdentityFieldV1, NormalSourcePlanStageV1,
    RejectedNormalSourcePlanV1,
};

#[cfg(test)]
mod tests;
