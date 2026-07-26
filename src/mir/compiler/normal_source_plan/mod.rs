//! Source-owned normal compilation family classification.
//!
//! This module observes one owned parsed source exactly once. It does not
//! select a profile, lower MIR, choose a backend, or execute a program.

mod callable_catalog_source;
mod callable_source;
mod classifier;
mod inventory;
mod main_direct_call_plan;
mod main_direct_call_source;
mod main_function_plan;
mod main_resolved_source;
mod main_source;
mod main_thunk_plan;
mod normal_acyclic_module_plan;
mod normal_callable_transaction_handoff;
mod product;
mod rejection;
mod script_physical_entry;
mod script_recipe;
#[cfg(test)]
mod test_support;

#[allow(unused_imports)]
pub(crate) use callable_catalog_source::{
    NormalCallableCatalogSourceErrorV1, NormalCallableCatalogSourceStageV1,
    RejectedNormalCallableCatalogSourceV1, VerifiedNormalCallableCatalogSourceUnitV1,
};
#[allow(unused_imports)]
pub(crate) use callable_source::{
    NormalCallableSourceErrorV1, NormalCallableSourceStageV1, RejectedNormalCallableSourceV1,
    VerifiedNormalCallableSourceUnitV1,
};
pub(crate) use classifier::NormalSourcePlanClassifierV1;
#[allow(unused_imports)]
pub(crate) use main_direct_call_plan::{
    NormalMainDirectCallPlanErrorV1, NormalMainDirectCallPreflightV1,
    RejectedNormalMainDirectCallPlanV1, VerifiedNormalMainDirectCallPlanV1,
};
#[allow(unused_imports)]
pub(crate) use main_direct_call_source::{
    NormalMainDirectCallSourceErrorV1, NormalMainDirectCallSourceStageV1,
    RejectedNormalMainDirectCallSourceV1, VerifiedNormalMainDirectCallSourceUnitV1,
};
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
pub(crate) use main_thunk_plan::{
    seal_normal_main_physical_relation_v1, NormalMainThunkPlanErrorV1,
    RejectedNormalMainThunkPlanV1, VerifiedNormalMainEntryRelationV1,
    VerifiedNormalMainPhysicalRelationV1, VerifiedNormalMainThunkPlanV1,
    VerifiedNormalMainThunkResultV1,
};
#[allow(unused_imports)]
pub(crate) use normal_acyclic_module_plan::{
    CompletedNormalMainHelperResolutionV1, NormalAcyclicCallableModuleErrorV1,
    NormalMainHelperResolutionStageV1, PreparedNormalMainHelperResolutionV1,
    RejectedNormalMainHelperResolutionV1, VerifiedNormalAcyclicCallableModulePlanV1,
    VerifiedNormalHelperTopologyPlanV1, VerifiedNormalRecursiveCallableModulePlanV1,
};
#[allow(unused_imports)]
pub(crate) use normal_callable_transaction_handoff::{
    ConsumableNormalMainLoweringProofV1, NormalCallableHandoffStageV1,
    NormalHelperDraftAbiExpectationErrorV1, OpenNormalCallableModuleTransactionV1,
    OwnedNormalHelperLoweringScheduleV1, PreparedNormalHelperTopologyReceiptV1,
    RejectedNormalCallableHandoffV1, RejectedNormalMainProofBindingV1,
    RetainedNormalCallableSourceAuthorityV1,
};
#[allow(unused_imports)]
pub(crate) use product::{
    PreparedNormalSourcePlanInputV1, SealedNormalCallableModuleSourceV1, SealedNormalMainSourceV1,
    SealedNormalScalarRootV1, SealedNormalScriptSourceV1, SealedNormalSourcePlanV1,
};
pub(crate) use rejection::{
    NormalSourcePlanErrorV1, NormalSourcePlanStageV1, RejectedNormalSourcePlanV1,
};
pub(crate) use script_physical_entry::{
    CompletedScriptPhysicalExitV1, NormalScriptPhysicalEntryStageV1, OpenScriptPhysicalEntryV1,
    RejectedNormalScriptPhysicalEntryV1,
};
#[allow(unused_imports)]
pub(crate) use script_recipe::{
    NormalScriptRecipeStageV1, RejectedNormalScriptRecipeV1, RetainedNormalScriptSourceV1,
    VerifiedNormalScriptRecipeV1,
};
#[cfg(test)]
pub(crate) use test_support::with_main_thunk_for_test;

#[cfg(test)]
mod tests;
