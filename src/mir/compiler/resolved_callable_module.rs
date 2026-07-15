//! Passive MP0-S0 multi-function resolved-module carrier.
//!
//! This schema owns no resolver or construction entry. MP0-R0 will consume the
//! CAT0 resolver continuation and publish this carrier only after every
//! function has one same-catalog forest/projection pair.

use std::collections::BTreeMap;

use crate::mir::resolved_semantics::{
    CanonicalCallableKeyV1, SourceCallableDeclarationSiteV1, VerifiedCallableCatalogSourceUnitV1,
    VerifiedSemanticOwnerForestV1,
};

use super::source_projection::VerifiedSourceProjectionV1;

/// One top-level declaration's closed semantic/source pair.
#[derive(Debug)]
pub(crate) struct VerifiedResolvedFunctionUnitV1 {
    declaration_site: SourceCallableDeclarationSiteV1,
    forest: VerifiedSemanticOwnerForestV1,
    projection: VerifiedSourceProjectionV1,
}

impl VerifiedResolvedFunctionUnitV1 {
    pub(crate) const fn declaration_site(&self) -> SourceCallableDeclarationSiteV1 {
        self.declaration_site
    }

    pub(crate) const fn forest(&self) -> &VerifiedSemanticOwnerForestV1 {
        &self.forest
    }

    pub(crate) const fn projection(&self) -> &VerifiedSourceProjectionV1 {
        &self.projection
    }
}

/// Exact Program/catalog source paired with one primary canonical-key map.
#[derive(Debug)]
pub(crate) struct VerifiedResolvedCallableModuleV1 {
    source: VerifiedCallableCatalogSourceUnitV1,
    functions_by_key: BTreeMap<CanonicalCallableKeyV1, VerifiedResolvedFunctionUnitV1>,
}

impl VerifiedResolvedCallableModuleV1 {
    pub(crate) const fn source(&self) -> &VerifiedCallableCatalogSourceUnitV1 {
        &self.source
    }

    pub(crate) const fn functions_by_key(
        &self,
    ) -> &BTreeMap<CanonicalCallableKeyV1, VerifiedResolvedFunctionUnitV1> {
        &self.functions_by_key
    }

    pub(crate) fn function(
        &self,
        key: &CanonicalCallableKeyV1,
    ) -> Option<&VerifiedResolvedFunctionUnitV1> {
        self.functions_by_key.get(key)
    }
}
