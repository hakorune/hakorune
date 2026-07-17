//! Complete same-module callable declaration catalog.
//!
//! L0b-S0 remains disconnected: the existing declaration index and lowering
//! behavior stay unchanged until the atomic CUT0.

mod catalog;
mod error;
mod key;

// These are intentionally disconnected S0 exports. CUT0 supplies their first
// production producer/consumer, so keep the public module surface stable now.
#[allow(unused_imports)]
pub(crate) use catalog::{
    VerifiedSameModuleCallableDeclarationCatalogV1, VerifiedSameModuleCallableDeclarationV1,
};
pub(crate) use error::SameModuleCallableDeclarationCatalogErrorV1;
#[allow(unused_imports)]
pub(crate) use key::{CanonicalSameModuleCallableKeyV1, SameModuleCallableNamespaceV1};

#[cfg(test)]
mod tests;
