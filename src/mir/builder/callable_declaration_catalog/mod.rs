//! Complete same-module callable declaration catalog.
//!
//! The catalog is sealed once from the complete root before declaration-index
//! effects. CUT0 makes it the sole same-module callable declaration authority.

mod catalog;
mod error;
mod key;
mod recovery;

// These are intentionally disconnected S0 exports. CUT0 supplies their first
// production producer/consumer, so keep the public module surface stable now.
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

#[cfg(test)]
mod recovery_tests;
#[cfg(test)]
mod tests;
