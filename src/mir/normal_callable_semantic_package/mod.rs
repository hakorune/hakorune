//! One owned semantic package for the final parser callable source.
//!
//! The issuer resolves the complete callable batch once, projects parameter
//! demands, and publishes either a valid-unselected or exact selected Dynamic
//! projection before Builder effects begin. It owns no CFG, Completion
//! consumption, physical ABI, or fallback route.

mod completion_seed;
mod declared_instance_locator;
mod direct_call_loan;
mod dynamic_admission;
mod install;
mod instance_construction;
mod instance_constructor_loan;
mod instance_constructor_non_escape;
mod instance_constructor_semantic;
pub(crate) use instance_construction::{ConstructionEligibilityV1, ConstructionUnavailableV1};
mod issuer;
mod model;
mod ordinary_new_coseal;
mod physical_header;
mod physical_signature;
mod result_contract;
mod s6c_child;
mod s6c_effects;
mod s6c_storage_header;
mod selected_mapping;

/// Opaque capability created only by the package-owned install bridge.  The
/// type name is visible to the Builder bridge, while its constructor and
/// fields remain private to this package family.
pub(in crate::mir) struct BuilderInstallTokenV1 {
    _private: (),
}

impl BuilderInstallTokenV1 {
    fn issue() -> Self {
        Self { _private: () }
    }
}

#[cfg(test)]
mod brand_catalog_tests;
#[cfg(test)]
mod declared_instance_locator_tests;
#[cfg(test)]
mod main_static_child_tests;
#[cfg(test)]
mod physical_header_tests;
#[cfg(test)]
mod physical_signature_tests;
#[cfg(test)]
mod resolved_selected_handoff_tests;
#[cfg(test)]
mod resolver_deferred_tests;
#[cfg(test)]
mod s6c_child_tests;
#[cfg(test)]
mod tests;

pub(in crate::mir) use declared_instance_locator::{
    DeclaredInstanceCallLocatorScopeV1, DeclaredInstanceCallLocatorViewV1,
};
pub(crate) use direct_call_loan::{
    AppMainDirectCallDispositionLoanV1, AppMainDirectCallDispositionRowV1,
};
pub(in crate::mir) use install::SelectedCallableSemanticRefV1;
pub(crate) use install::{
    InstalledNormalCallableSemanticPackageV1, NormalCallableSemanticPackageInstallIssueV1,
    NormalCallableSemanticPackagePortV1, ResolvedCallablePhysicalSignatureLoanV1,
    S6CCommonV2PreSessionLoanRefV1, SelectedCallableLoweringInputRefV1,
    SelectedCatalogedCallableLoweringInputV1,
};
#[cfg(test)]
pub(in crate::mir) use issuer::issue_normal_callable_semantic_package_v1;
pub(in crate::mir) use issuer::{
    issue_normal_callable_semantic_package_with_brand_catalog_v1,
    NormalCallableSemanticPackageIssueV1,
};
pub(in crate::mir) use model::NormalCallableDynamicProjectionRefV1;
pub(crate) use model::VerifiedNormalCallableSemanticPackageV1;
pub(crate) use ordinary_new_coseal::{
    BirthAbiHandoffV1, BirthResultAbiV1, FinalizedRootBirthHandoffV1,
    FinalizedRootResultAbiV1,
    OrdinaryNewAdmissionClaimV1, OrdinaryNewClaimLedgerV1,
    OrdinaryNewConstructorDispositionV1, PreparedTerminalI64AddReturnV1,
};
pub(crate) use physical_header::CallablePhysicalHeaderRefV1;
pub(crate) use physical_signature::{
    PhysicalCallableLaneRoleV1, PhysicalCallableLaneV1, PhysicalCallableSignatureRowRefV1,
};
pub(crate) use s6c_effects::VerifiedS6CPhysicalFunctionEffectsV1;
