//! Canonical pre-Builder source-call target proofs.
//!
//! Q0 is intentionally disconnected. See `README.md` before adding a producer
//! or consumer.

#![allow(dead_code)]

mod callable_receiver_policy;
mod core_method;
mod current_owner;
mod dynamic_member;
mod error;
mod model;
mod qualified;
mod qualified_receiver_lexical;
mod qualified_route_facts;
mod raw_callable_source_view;
mod raw_source_cursor_error;
mod source_method_call_site;
mod script_direct_static;
mod whole_source_inventory;
mod whole_source_inventory_error;

#[allow(unused_imports)]
pub(crate) use callable_receiver_policy::SameModuleCallableSourceReceiverPolicyV1;
#[allow(unused_imports)]
pub(crate) use core_method::{
    issue_source_bound_s6c_call_relation_v1, S6CSourceBoundCallRelationRefV1,
    S6CSourceBoundCallRelationRejectV1, S6CSourceBoundCallRoleV1,
    VerifiedSourceBoundS6CCallRelationV1,
};
#[allow(unused_imports)]
pub(crate) use dynamic_member::{
    issue_source_bound_dynamic_member_calls_v1, DynamicMemberArgumentSourceV1,
    DynamicMemberDispatchKeyV1, DynamicMemberSourceIssueV1, DynamicMemberSourceRejectV1,
    DynamicMemberSourceUnresolvedV1, VerifiedSourceBoundDynamicMemberCallV1,
};
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
    VerifiedSourceCallTargetCatalogV1, VerifiedSourceCallTargetV1,
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
pub(crate) use script_direct_static::{
    ScriptDirectStaticCallTargetErrorV1, ScriptStaticCallSourceOwnerIdV1,
    VerifiedScriptDirectStaticCallSiteV1, VerifiedScriptDirectStaticCallTargetInventoryV1,
    VerifiedScriptDirectStaticCallTargetV1,
};
#[allow(unused_imports)]
pub(crate) use whole_source_inventory::{
    VerifiedWholeSourceMethodCallSiteV1, VerifiedWholeSourceStaticCallTargetInventoryV1,
};
#[allow(unused_imports)]
pub(crate) use whole_source_inventory_error::WholeSourceStaticCallTargetInventoryErrorV1;

#[cfg(test)]
mod callable_receiver_policy_tests;
#[cfg(test)]
mod core_method_tests;
#[cfg(test)]
mod current_owner_tests;
#[cfg(test)]
mod dynamic_member_tests;
#[cfg(test)]
mod qualified_receiver_lexical_tests;
#[cfg(test)]
mod qualified_route_facts_tests;
#[cfg(test)]
mod raw_callable_source_view_tests;
#[cfg(test)]
mod source_method_call_site_tests;
#[cfg(test)]
mod script_direct_static_tests;
#[cfg(test)]
mod test_support;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod whole_source_inventory_tests;
