//! Complete same-module callable declaration catalog.
//!
//! The catalog is sealed once from the complete root before declaration-index
//! effects. CUT0 makes it the sole same-module callable declaration authority.

mod brand;
mod catalog;
mod error;
mod key;
mod recovery;
mod selected_role;
mod selected_source_inventory;
mod source_backed;

// These are intentionally disconnected S0 exports. CUT0 supplies their first
// production producer/consumer, so keep the public module surface stable now.
pub(crate) use brand::SameModuleCallableCatalogBrandV1;
#[allow(unused_imports)]
pub(crate) use catalog::{
    VerifiedSameModuleCallableDeclarationCatalogV1, VerifiedSameModuleCallableDeclarationV1,
};
pub(crate) use error::{
    SameModuleCallableDeclarationCatalogErrorV1, SameModuleCallableDeclarationCatalogSessionErrorV1,
};
#[allow(unused_imports)]
pub(crate) use key::{CanonicalSameModuleCallableKeyV1, SameModuleCallableNamespaceV1};
#[allow(unused_imports)]
pub(crate) use recovery::{
    BareStaticRecoveryDecisionErrorV1, BareStaticRecoveryDecisionV1,
    BareStaticRecoveryNoRecoveryReasonV1,
};
pub(crate) use selected_role::SelectedCallableConsumptionRoleV1;
pub(crate) use selected_source_inventory::SelectedNormalCallableKeyV1;
pub(in crate::mir::builder) use selected_source_inventory::{
    SelectedCallableSemanticBlockerV1, SelectedNormalCallableSourceSiteV1,
    SelectedTopLevelFunctionKeyV1, VerifiedSelectedNormalCallableSourceInventoryV1,
};
pub(crate) use source_backed::{
    issue_source_backed_same_module_callable_catalog_v1, SourceBackedCallableCatalogIssueV1,
    VerifiedSourceBackedSameModuleCallableCatalogV1,
};

#[cfg(test)]
mod recovery_tests;
#[cfg(test)]
mod tests;
