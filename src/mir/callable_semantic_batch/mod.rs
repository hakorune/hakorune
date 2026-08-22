//! Sole resolved owner for one retained parser callable-source batch.

mod issuer;
mod model;
// I0 is intentionally caller-zero until the source-bound relation D0/I0
// names its sole consumer. Keep the allowance scoped to this one module.
#[allow(dead_code)]
mod s6c_typed_input;

#[cfg(test)]
mod s6c_typed_input_tests;
#[cfg(test)]
mod tests;

pub(crate) use issuer::{
    issue_resolved_callable_semantic_batch_v1,
    issue_resolved_callable_semantic_batch_with_brand_catalog_v1,
    ResolvedCallableSemanticBatchIssueV1,
};
pub(crate) use model::{
    ResolvedCallableDeclarationModeV1, ResolvedCallableSemanticBatchLoanErrorV1,
    VerifiedResolvedCallableParameterSourceRefV1, VerifiedResolvedCallableSemanticBatchRefV1,
    VerifiedResolvedCallableSemanticBatchV1, VerifiedResolvedCallableSemanticDeclarationRefV1,
    VerifiedResolvedCallableSemanticRowRefV1, VerifiedResolvedCallableSourceIdentityV1,
};
#[allow(unused_imports)]
pub(crate) use s6c_typed_input::{
    issue_s6c_typed_input_relation_v1, S6CBinaryRelationV1, S6CBinaryRoleV1, S6CCallSitePairRefV1,
    S6CLogicalValueClassV1, S6CTypedBindingV1, S6CTypedInputRelationRejectV1, S6CTypedInputRoleV1,
    VerifiedS6CTypedInputRelationV1,
};
