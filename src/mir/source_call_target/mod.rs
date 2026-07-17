//! Canonical pre-Builder source-call target proofs.
//!
//! Q0 is intentionally disconnected. See `README.md` before adding a producer
//! or consumer.

#![allow(dead_code)]

mod error;
mod model;
mod qualified;

#[allow(unused_imports)]
pub(crate) use error::{QualifiedStaticCallTargetErrorV1, StaticImportAliasViewErrorV1};
#[allow(unused_imports)]
pub(crate) use model::{
    QualifiedReceiverLexicalFactV1, QualifiedStaticCallCandidateV1, QualifiedStaticReceiverV1,
    ReservedQualifiedReceiverRouteV1, VerifiedQualifiedStaticCallTargetV1,
    VerifiedSourceStaticCallTargetCatalogV1, VerifiedSourceStaticCallTargetV1,
    VerifiedStaticImportAliasViewV1,
};

#[cfg(test)]
mod tests;
