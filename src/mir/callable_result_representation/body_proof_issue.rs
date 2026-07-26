//! Opaque unannotated callable-body proof issued by the static result catalog.
//!
//! This module does not select an instance route.  It only packages the
//! existing private body proof so another source-only owner can consume its
//! declaration-paired result without borrowing the static result catalog.

use crate::mir::builder::{
    CanonicalSameModuleCallableKeyV1, VerifiedSameModuleCallableDeclarationV1,
};

use super::{CallableResultCatalogErrorV1, CallableResultUnavailableReasonV1};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CallableBodyProofIssueErrorV1 {
    TargetOutsideCatalog {
        target: CanonicalSameModuleCallableKeyV1,
    },
    DeclaredResultAuthorityForbidden {
        target: CanonicalSameModuleCallableKeyV1,
    },
    Catalog(CallableResultCatalogErrorV1),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum VerifiedUnannotatedCallableBodyResultOutcomeV1 {
    ExactI64 {
        required_i64_arguments: Box<[u32]>,
    },
    Unavailable(CallableResultUnavailableReasonV1),
    PendingDependency,
}

#[derive(Debug)]
pub(crate) struct VerifiedUnannotatedCallableBodyResultProofV1<'catalog> {
    declaration: &'catalog VerifiedSameModuleCallableDeclarationV1,
    outcome: VerifiedUnannotatedCallableBodyResultOutcomeV1,
}

impl<'catalog> VerifiedUnannotatedCallableBodyResultProofV1<'catalog> {
    pub(super) fn new(
        declaration: &'catalog VerifiedSameModuleCallableDeclarationV1,
        outcome: VerifiedUnannotatedCallableBodyResultOutcomeV1,
    ) -> Self {
        Self {
            declaration,
            outcome,
        }
    }

    pub(crate) fn matches_declaration(
        &self,
        declaration: &'catalog VerifiedSameModuleCallableDeclarationV1,
    ) -> bool {
        std::ptr::eq(self.declaration, declaration)
    }

    pub(crate) const fn outcome(&self) -> &VerifiedUnannotatedCallableBodyResultOutcomeV1 {
        &self.outcome
    }

    pub(crate) fn discard(self) {}
}
