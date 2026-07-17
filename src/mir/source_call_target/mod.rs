//! Canonical pre-Builder source-call target proofs.
//!
//! Q0 is intentionally disconnected. See `README.md` before adding a producer
//! or consumer.

#![allow(dead_code)]

mod current_owner;
mod error;
mod model;
mod qualified;
mod source_method_call_site;

#[allow(unused_imports)]
pub(crate) use error::{
    CurrentOwnerStaticCallTargetErrorV1, QualifiedStaticCallTargetErrorV1,
    SourceMethodCallSiteErrorV1, StaticImportAliasViewErrorV1,
};
#[allow(unused_imports)]
pub(crate) use model::{
    CurrentOwnerStaticCallCandidateV1, CurrentOwnerStaticReceiverV1,
    QualifiedReceiverLexicalFactV1, QualifiedStaticCallCandidateV1, QualifiedStaticReceiverV1,
    ReservedQualifiedReceiverRouteV1, VerifiedCurrentOwnerStaticCallTargetV1,
    VerifiedQualifiedStaticCallTargetV1, VerifiedSourceStaticCallTargetCatalogV1,
    VerifiedSourceStaticCallTargetV1, VerifiedStaticImportAliasViewV1,
};
#[allow(unused_imports)]
pub(crate) use source_method_call_site::VerifiedSourceMethodCallSiteV1;

#[cfg(test)]
mod current_owner_tests;
#[cfg(test)]
mod source_method_call_site_tests;
#[cfg(test)]
mod tests;
