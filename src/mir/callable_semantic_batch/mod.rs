//! Sole resolved owner for one retained parser callable-source batch.

pub(crate) mod builtin_map_construction;
mod issuer;
mod model;
// I0 is intentionally caller-zero until the source-bound relation D0/I0
// names its sole consumer. Keep the allowance scoped to this one module.

#[cfg(test)]
mod declared_instance_call_effect_tests;
#[cfg(test)]
mod declared_instance_call_relation_tests;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod builtin_map_construction_tests;

#[cfg(test)]
pub(crate) use issuer::{
    issue_resolved_callable_semantic_batch_v1,
    issue_resolved_callable_semantic_batch_with_brand_catalog_v1,
};
pub(crate) use issuer::{
    issue_resolved_callable_semantic_batch_with_freestatic_targets_v1,
    issue_resolved_callable_semantic_batch_with_main_freestatic_targets_v1,
    ResolvedCallableSemanticBatchIssueV1,
};
pub(crate) use model::{
    ResolvedCallableDeclarationModeV1, ResolvedCallableSemanticBatchLoanErrorV1,
    ResolvedCallableBorrowedResultDeclarationV1, ResolvedCallableBorrowedResultSupportV1,
    VerifiedResolvedCallableParameterSourceRefV1, VerifiedResolvedCallableSemanticBatchV1,
    VerifiedResolvedCallableSemanticDeclarationRefV1, VerifiedResolvedCallableSemanticRowRefV1,
    VerifiedResolvedCallableSourceIdentityV1,
};
