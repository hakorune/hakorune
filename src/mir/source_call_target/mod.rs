//! Canonical pre-Builder source-call target proofs.
//!
//! Q0 is intentionally disconnected. See `README.md` before adding a producer
//! or consumer.

#![allow(dead_code)]

mod current_owner;
mod error;
mod model;
mod qualified;
mod qualified_receiver_lexical;
mod qualified_route_facts;
mod raw_callable_source_view;
mod raw_source_cursor_error;
mod source_method_call_site;
mod whole_source_inventory;
mod whole_source_inventory_error;

#[allow(unused_imports)]
pub(crate) use error::{
    CurrentOwnerStaticCallTargetErrorV1, QualifiedCallRouteFactsErrorV1,
    QualifiedReceiverLexicalDispositionErrorV1, QualifiedStaticCallTargetErrorV1,
    SourceMethodCallSiteErrorV1, StaticImportAliasViewErrorV1,
};
#[allow(unused_imports)]
pub(crate) use model::{
    CurrentOwnerStaticReceiverV1, QualifiedStaticReceiverV1,
    VerifiedCurrentOwnerStaticCallTargetV1, VerifiedQualifiedStaticCallTargetV1,
    VerifiedSourceStaticCallTargetCatalogV1, VerifiedSourceStaticCallTargetV1,
    VerifiedStaticImportAliasViewV1,
};
#[allow(unused_imports)]
pub(crate) use qualified_receiver_lexical::{
    QualifiedReceiverLexicalDispositionV1, VerifiedQualifiedReceiverLexicalDispositionsV1,
};
#[allow(unused_imports)]
pub(crate) use qualified_route_facts::{
    QualifiedReceiverAdmissionV1, VerifiedQualifiedCallRouteFactsV1,
};
#[allow(unused_imports)]
pub(crate) use raw_callable_source_view::{
    RawLocatedBodyInputV1, RawLocatedCallArgumentStageV1, RawLocatedExprInputV1,
    RawLocatedMethodCallInputV1, RawLocatedStmtInputV1, RejectedRawLocatedCallArgumentV1,
    VerifiedRawCallableSourceViewV1, VerifiedRawLocatedCallArgumentV1,
};
#[allow(unused_imports)]
pub(crate) use raw_source_cursor_error::RawSourceCursorErrorV1;
#[allow(unused_imports)]
pub(crate) use source_method_call_site::VerifiedSourceMethodCallSiteV1;
#[allow(unused_imports)]
pub(crate) use whole_source_inventory::{
    VerifiedWholeSourceMethodCallSiteV1, VerifiedWholeSourceStaticCallTargetInventoryV1,
};
#[allow(unused_imports)]
pub(crate) use whole_source_inventory_error::WholeSourceStaticCallTargetInventoryErrorV1;

#[cfg(test)]
mod current_owner_tests;
#[cfg(test)]
mod qualified_receiver_lexical_tests;
#[cfg(test)]
mod qualified_route_facts_tests;
#[cfg(test)]
mod raw_callable_source_view_tests;
#[cfg(test)]
mod source_method_call_site_tests;
#[cfg(test)]
mod test_support;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod whole_source_inventory_tests;
