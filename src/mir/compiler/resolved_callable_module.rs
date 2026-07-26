//! MP0-S0/R0 multi-function resolved-module carrier and sole resolver entry.
//!
//! MP0-R0 consumes the CAT0 resolver continuation and publishes this carrier
//! only after every function has one same-catalog forest/projection pair.

use std::collections::BTreeMap;

use crate::mir::resolved_semantics::{
    CallableCatalogSealOutcomeV1, CallableLookupErrorV1, CanonicalCallableKeyV1,
    CatalogSealedResolverContinuationV1, ResolveOwnerForestErrorV1,
    SourceCallableDeclarationSiteV1, VerifiedCallableCatalogSourceUnitV1,
    VerifiedSemanticOwnerForestV1,
};

use super::source_projection::{SourceNavigationErrorV1, VerifiedSourceProjectionV1};

#[derive(Debug)]
pub(crate) enum ResolveCallableModuleErrorV1 {
    MissingDeclaration(SourceCallableDeclarationSiteV1),
    MissingCallableHeader(SourceCallableDeclarationSiteV1, CallableLookupErrorV1),
    MissingFunctionSyntax(SourceCallableDeclarationSiteV1),
    SiteMismatch {
        expected: SourceCallableDeclarationSiteV1,
        actual: SourceCallableDeclarationSiteV1,
    },
    OwnerForest(SourceCallableDeclarationSiteV1, ResolveOwnerForestErrorV1),
    Projection(SourceCallableDeclarationSiteV1, SourceNavigationErrorV1),
    RootOwnerMismatch(SourceCallableDeclarationSiteV1),
    DuplicateCanonicalKey(CanonicalCallableKeyV1),
    CardinalityMismatch {
        declarations: usize,
        functions: usize,
    },
}

/// One top-level declaration's closed semantic/source pair.
#[derive(Debug)]
pub(crate) struct VerifiedResolvedFunctionUnitV1 {
    declaration_site: SourceCallableDeclarationSiteV1,
    forest: VerifiedSemanticOwnerForestV1,
    projection: VerifiedSourceProjectionV1,
}

impl VerifiedResolvedFunctionUnitV1 {
    fn new(
        declaration_site: SourceCallableDeclarationSiteV1,
        forest: VerifiedSemanticOwnerForestV1,
        projection: VerifiedSourceProjectionV1,
    ) -> Self {
        Self {
            declaration_site,
            forest,
            projection,
        }
    }

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

#[derive(Debug)]
pub(crate) struct RejectedResolvedCallableModuleV1 {
    owner: CallableCatalogSealOutcomeV1,
    error: ResolveCallableModuleErrorV1,
}

impl RejectedResolvedCallableModuleV1 {
    pub(crate) const fn error(&self) -> &ResolveCallableModuleErrorV1 {
        &self.error
    }

    pub(crate) fn discard(self) {
        drop(self);
    }

    fn into_error(self) -> ResolveCallableModuleErrorV1 {
        self.error
    }

    pub(in crate::mir) fn into_normal_composition_parts(
        self,
    ) -> (CallableCatalogSealOutcomeV1, ResolveCallableModuleErrorV1) {
        (self.owner, self.error)
    }
}

impl VerifiedResolvedCallableModuleV1 {
    pub(crate) fn resolve(
        catalog_outcome: CallableCatalogSealOutcomeV1,
    ) -> Result<Self, ResolveCallableModuleErrorV1> {
        Self::resolve_retaining(catalog_outcome)
            .map_err(RejectedResolvedCallableModuleV1::into_error)
    }

    pub(crate) fn resolve_retaining(
        catalog_outcome: CallableCatalogSealOutcomeV1,
    ) -> Result<Self, RejectedResolvedCallableModuleV1> {
        let (source_unit, continuation) = catalog_outcome.into_parts();
        let source = source_unit.into_resolution_source();
        let sites = source.declaration_sites().to_vec();
        let mut resolver = continuation.into_resolver();
        let resolved = (|| {
            let mut functions_by_key = BTreeMap::new();
            for site in sites {
                let declaration = source
                    .catalog()
                    .declaration(site)
                    .ok_or(ResolveCallableModuleErrorV1::MissingDeclaration(site))?;
                let header = source
                    .catalog()
                    .index()
                    .header_for_callable(declaration.callable())
                    .map_err(|error| {
                        ResolveCallableModuleErrorV1::MissingCallableHeader(site, error)
                    })?;
                let key = header.source_key().clone();
                let located = source
                    .located_function(site)
                    .ok_or(ResolveCallableModuleErrorV1::MissingFunctionSyntax(site))?;
                if located.site() != site {
                    return Err(ResolveCallableModuleErrorV1::SiteMismatch {
                        expected: site,
                        actual: located.site(),
                    });
                }
                let forest = resolver
                    .resolve_forest_with_reserved_root(
                        located.function(),
                        declaration.origin(),
                        declaration.callable().owner(),
                        source.catalog().index(),
                    )
                    .map_err(|error| ResolveCallableModuleErrorV1::OwnerForest(site, error))?;
                if forest.roots() != [declaration.callable().owner()] {
                    return Err(ResolveCallableModuleErrorV1::RootOwnerMismatch(site));
                }
                let projection = VerifiedSourceProjectionV1::seal(located.root(), &forest)
                    .map_err(|error| ResolveCallableModuleErrorV1::Projection(site, error))?;
                let unit = VerifiedResolvedFunctionUnitV1::new(site, forest, projection);
                if functions_by_key.insert(key.clone(), unit).is_some() {
                    return Err(ResolveCallableModuleErrorV1::DuplicateCanonicalKey(key));
                }
            }

            let declarations = source.catalog().len();
            if declarations != functions_by_key.len() {
                return Err(ResolveCallableModuleErrorV1::CardinalityMismatch {
                    declarations,
                    functions: functions_by_key.len(),
                });
            }
            Ok(functions_by_key)
        })();
        let functions_by_key = match resolved {
            Ok(functions) => functions,
            Err(error) => {
                return Err(RejectedResolvedCallableModuleV1 {
                    owner: CallableCatalogSealOutcomeV1::restore(
                        source.finish(),
                        CatalogSealedResolverContinuationV1::restore(resolver),
                    ),
                    error,
                })
            }
        };
        Ok(Self {
            source: source.finish(),
            functions_by_key,
        })
    }

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
