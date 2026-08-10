//! One owned semantic package for the final parser callable source.
//!
//! The issuer resolves the complete callable batch once, projects parameter
//! demands, and admits the exact Dynamic full-body candidate before Builder
//! effects begin.  It owns no CFG, Completion consumption, physical ABI, or
//! fallback route.

mod dynamic_admission;
mod issuer;
mod model;

#[cfg(test)]
mod tests;

pub(crate) use issuer::{
    issue_normal_callable_semantic_dynamic_package_v1,
    NormalCallableSemanticDynamicPackageIssueV1,
};
pub(crate) use model::VerifiedNormalCallableSemanticDynamicPackageV1;
