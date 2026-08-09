//! Exact catalog-callable to resolver-owner co-seal.
//!
//! The normal semantic loan is the only place where one selected catalog key,
//! its catalog allocation, and the resolver-issued function owner coexist.
//! This module seals that relation once; downstream target issuers never pair
//! equal-looking keys and owners independently.

use super::{
    normal_callable_semantic_source::VerifiedNormalCallableSemanticLoanV1,
    normal_callable_semantic_source::VerifiedNormalCallableSourceIngressReceiptV1,
    raw_invocation_source_transport::RawInvocationRootLineageV1, CanonicalSameModuleCallableKeyV1,
    VerifiedSameModuleCallableDeclarationCatalogV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum CatalogCallableOwnerLinkIssueV1 {
    CatalogedCallableRequired,
    ForeignCatalog,
}

/// Non-Clone relation between one exact catalog callable and resolver owner.
#[derive(Debug)]
pub(in crate::mir) struct VerifiedCatalogCallableOwnerLinkV1<'source> {
    callable: CanonicalSameModuleCallableKeyV1,
    source: VerifiedNormalCallableSourceIngressReceiptV1<'source>,
}

impl<'source> VerifiedCatalogCallableOwnerLinkV1<'source> {
    pub(in crate::mir) fn callable(&self) -> &CanonicalSameModuleCallableKeyV1 {
        &self.callable
    }

    pub(in crate::mir) fn into_parts(
        self,
    ) -> (
        CanonicalSameModuleCallableKeyV1,
        VerifiedNormalCallableSourceIngressReceiptV1<'source>,
    ) {
        (self.callable, self.source)
    }
}

pub(in crate::mir) fn issue_catalog_callable_owner_link_v1<'source>(
    loan: VerifiedNormalCallableSemanticLoanV1<'_, 'source>,
    catalog: &VerifiedSameModuleCallableDeclarationCatalogV1,
) -> Result<VerifiedCatalogCallableOwnerLinkV1<'source>, CatalogCallableOwnerLinkIssueV1> {
    if !loan.catalog_brand.is_same(catalog.brand()) {
        return Err(CatalogCallableOwnerLinkIssueV1::ForeignCatalog);
    }
    let RawInvocationRootLineageV1::Cataloged(callable) = &loan.lineage else {
        return Err(CatalogCallableOwnerLinkIssueV1::CatalogedCallableRequired);
    };
    if catalog.declaration(callable).is_none() {
        return Err(CatalogCallableOwnerLinkIssueV1::ForeignCatalog);
    }
    let callable = callable.clone();
    let source = loan.source_ingress;
    Ok(VerifiedCatalogCallableOwnerLinkV1 { callable, source })
}
