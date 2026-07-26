//! Source-owned normal compilation family classification.
//!
//! This module observes one owned parsed source exactly once. It does not
//! select a profile, lower MIR, choose a backend, or execute a program.

mod classifier;
mod inventory;
mod main_function_plan;
mod main_resolved_source;
mod main_source;
mod product;
mod rejection;

pub(crate) use classifier::NormalSourcePlanClassifierV1;
#[allow(unused_imports)]
pub(crate) use main_function_plan::{
    NormalMainFunctionPlanErrorV1, NormalMainFunctionPreflightV1, RejectedNormalMainFunctionPlanV1,
    VerifiedNormalMainFunctionPlanV1,
};
#[allow(unused_imports)]
pub(crate) use main_resolved_source::{
    NormalMainResolvedSourceErrorV1, RejectedNormalMainResolvedSourceV1,
    VerifiedNormalMainResolvedSourceUnitV1, VerifiedNormalMainRoleV1,
};
#[allow(unused_imports)]
pub(crate) use main_source::{
    NormalMainFunctionSourceErrorV1, NormalMainFunctionSourceViewV1,
    RejectedNormalMainFunctionSourceV1, VerifiedNormalMainFunctionSourceUnitV1,
};
#[allow(unused_imports)]
pub(crate) use product::{
    PreparedNormalSourcePlanInputV1, SealedNormalCallableModuleSourceV1, SealedNormalScalarRootV1,
    SealedNormalSourcePlanV1,
};
pub(crate) use rejection::{
    NormalSourcePlanErrorV1, NormalSourcePlanStageV1, RejectedNormalSourcePlanV1,
};

#[cfg(test)]
mod tests;
