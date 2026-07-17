//! Complete same-module static-callable declaration catalog.
//!
//! L0a is disconnected: the existing declaration index and lowering behavior
//! remain unchanged until the behavior-neutral L0b cutover.

mod catalog;
mod error;
mod key;

// These are intentionally disconnected L0a exports. L0b supplies their first
// production producer/consumer, so keep the public module surface stable now.
#[allow(unused_imports)]
pub(crate) use catalog::VerifiedSameModuleCallableDeclarationCatalogV1;
pub(crate) use error::SameModuleCallableDeclarationCatalogErrorV1;
#[allow(unused_imports)]
pub(crate) use key::{CanonicalSameModuleCallableKeyV1, SameModuleCallableNamespaceV1};

#[cfg(test)]
mod tests;
