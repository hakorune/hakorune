//! One owned semantic package for the final parser callable source.
//!
//! The issuer resolves the complete callable batch once, projects parameter
//! demands, and publishes either a valid-unselected or exact selected Dynamic
//! projection before Builder effects begin. It owns no CFG, Completion
//! consumption, physical ABI, or fallback route.

mod completion_seed;
mod dynamic_admission;
mod install;
mod issuer;
mod model;
mod physical_header;
mod physical_signature;
mod s6c_child;
mod s6c_effects;
mod s6c_storage_header;
mod selected_mapping;

#[cfg(test)]
mod main_static_child_tests;
#[cfg(test)]
mod physical_header_tests;
#[cfg(test)]
mod physical_signature_tests;
#[cfg(test)]
mod resolved_selected_handoff_tests;
#[cfg(test)]
mod s6c_child_tests;
#[cfg(test)]
mod tests;

pub(crate) use install::{
    InstalledNormalCallableSemanticPackageV1, NormalCallableSemanticPackageInstallIssueV1,
    NormalCallableSemanticPackagePortV1, ResolvedCallablePhysicalSignatureLoanV1,
    S6CCommonV2PreSessionLoanRefV1, SelectedCallableLoweringInputRefV1,
    SelectedCallableSemanticRefV1, SelectedCatalogedCallableLoweringInputV1,
};
pub(crate) use issuer::{
    issue_normal_callable_semantic_package_v1,
    issue_normal_callable_semantic_package_with_brand_catalog_v1,
    NormalCallableSemanticPackageIssueV1,
};
pub(crate) use model::{
    NormalCallableDynamicProjectionRefV1, VerifiedNormalCallableSemanticPackageV1,
};
pub(crate) use physical_header::CallablePhysicalHeaderRefV1;
pub(crate) use physical_signature::{
    PhysicalCallableLaneRoleV1, PhysicalCallableLaneV1, PhysicalCallableSignatureRowRefV1,
    VerifiedCallablePhysicalSignatureCohortV1,
};
pub(crate) use s6c_effects::VerifiedS6CPhysicalFunctionEffectsV1;
