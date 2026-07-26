//! CAT0-C0b immutable callable catalog and resolver continuation.
//!
//! This row consumes the fully validated owner-free header source unit, issues
//! every top-level origin/owner pair from one resolver session, and co-seals
//! the Program with the immutable index. Function bodies remain unread.

use std::collections::BTreeMap;

use super::{
    CallableIndexSealErrorV1, EmbeddedCallableFunctionSyntaxViewV1, FunctionOriginV1,
    FunctionSemanticResolverSessionV1, LocatedCallableHeaderSyntaxViewV1, ResolveFunctionErrorV1,
    ResolvedCallableRefV1, SourceCallableDeclarationSiteV1, VerifiedCallableHeaderSourceUnitV1,
    VerifiedCallableIndexV1, VerifiedOwnerFreeCallableCatalogSourceUnitV1,
};

use super::callable_catalog_resolution_source::CallableCatalogResolutionSourceV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CallableCatalogOwnerSealErrorV1 {
    OwnerIssueExhausted {
        site: SourceCallableDeclarationSiteV1,
        reason: ResolveFunctionErrorV1,
    },
    MissingOwnerFreeCandidate {
        site: SourceCallableDeclarationSiteV1,
    },
    DuplicateDeclarationSite {
        site: SourceCallableDeclarationSiteV1,
    },
    MixedCompilationBrand,
    CatalogCardinalityMismatch {
        declarations: usize,
        headers: usize,
    },
    CallableIndex(CallableIndexSealErrorV1),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VerifiedCallableDeclarationV1 {
    site: SourceCallableDeclarationSiteV1,
    origin: FunctionOriginV1,
    callable: ResolvedCallableRefV1,
}

impl VerifiedCallableDeclarationV1 {
    pub(crate) const fn site(&self) -> SourceCallableDeclarationSiteV1 {
        self.site
    }

    pub(crate) const fn origin(&self) -> FunctionOriginV1 {
        self.origin
    }

    pub(crate) const fn callable(&self) -> ResolvedCallableRefV1 {
        self.callable
    }
}

#[derive(Debug)]
pub(crate) struct VerifiedCallableCatalogV1 {
    index: VerifiedCallableIndexV1,
    declarations_by_site: BTreeMap<SourceCallableDeclarationSiteV1, VerifiedCallableDeclarationV1>,
}

impl VerifiedCallableCatalogV1 {
    pub(crate) const fn index(&self) -> &VerifiedCallableIndexV1 {
        &self.index
    }

    pub(crate) fn declaration(
        &self,
        site: SourceCallableDeclarationSiteV1,
    ) -> Option<&VerifiedCallableDeclarationV1> {
        self.declarations_by_site.get(&site)
    }

    pub(crate) fn len(&self) -> usize {
        self.declarations_by_site.len()
    }
}

#[derive(Debug)]
pub(crate) struct VerifiedCallableCatalogSourceUnitV1 {
    source: VerifiedCallableHeaderSourceUnitV1,
    catalog: VerifiedCallableCatalogV1,
}

impl VerifiedCallableCatalogSourceUnitV1 {
    pub(crate) fn declaration_sites(&self) -> &[SourceCallableDeclarationSiteV1] {
        self.source.declaration_sites()
    }

    pub(crate) fn located_header(
        &self,
        site: SourceCallableDeclarationSiteV1,
    ) -> Option<LocatedCallableHeaderSyntaxViewV1<'_>> {
        self.source.located_header(site)
    }

    pub(crate) const fn catalog(&self) -> &VerifiedCallableCatalogV1 {
        &self.catalog
    }

    pub(in crate::mir) fn embedded_function(
        &self,
        statement_index: usize,
        method_key: &str,
    ) -> Option<EmbeddedCallableFunctionSyntaxViewV1<'_>> {
        self.source.embedded_function(statement_index, method_key)
    }

    pub(super) const fn resolution_syntax(&self) -> &VerifiedCallableHeaderSourceUnitV1 {
        &self.source
    }

    pub(in crate::mir) fn into_resolution_source(self) -> CallableCatalogResolutionSourceV1 {
        CallableCatalogResolutionSourceV1::begin(self)
    }

    pub(super) fn into_resolution_parts(
        self,
    ) -> (
        VerifiedCallableHeaderSourceUnitV1,
        VerifiedCallableCatalogV1,
    ) {
        (self.source, self.catalog)
    }

    pub(super) fn restore_after_resolution(
        source: VerifiedCallableHeaderSourceUnitV1,
        catalog: VerifiedCallableCatalogV1,
    ) -> Self {
        Self { source, catalog }
    }
}

#[derive(Debug)]
pub(crate) struct CatalogSealedResolverContinuationV1 {
    resolver: FunctionSemanticResolverSessionV1,
}

impl CatalogSealedResolverContinuationV1 {
    pub(in crate::mir) fn into_resolver(self) -> FunctionSemanticResolverSessionV1 {
        self.resolver
    }

    pub(in crate::mir) fn restore(resolver: FunctionSemanticResolverSessionV1) -> Self {
        Self { resolver }
    }
}

#[derive(Debug)]
pub(crate) struct PreparedCallableCatalogSealV1 {
    catalog: VerifiedCallableCatalogV1,
    resolver: FunctionSemanticResolverSessionV1,
}

#[derive(Debug)]
pub(crate) struct CallableCatalogSealOutcomeV1 {
    source_unit: VerifiedCallableCatalogSourceUnitV1,
    continuation: CatalogSealedResolverContinuationV1,
}

impl CallableCatalogSealOutcomeV1 {
    pub(crate) fn seal(
        owner_free: VerifiedOwnerFreeCallableCatalogSourceUnitV1,
        compilation_unit_ordinal: u32,
    ) -> Result<Self, CallableCatalogOwnerSealErrorV1> {
        let prepared =
            PreparedCallableCatalogSealV1::prepare(&owner_free, compilation_unit_ordinal)?;
        Ok(prepared.commit(owner_free))
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        VerifiedCallableCatalogSourceUnitV1,
        CatalogSealedResolverContinuationV1,
    ) {
        (self.source_unit, self.continuation)
    }

    pub(in crate::mir) fn restore(
        source_unit: VerifiedCallableCatalogSourceUnitV1,
        continuation: CatalogSealedResolverContinuationV1,
    ) -> Self {
        Self {
            source_unit,
            continuation,
        }
    }
}

impl PreparedCallableCatalogSealV1 {
    pub(crate) fn prepare(
        owner_free: &VerifiedOwnerFreeCallableCatalogSourceUnitV1,
        compilation_unit_ordinal: u32,
    ) -> Result<Self, CallableCatalogOwnerSealErrorV1> {
        let first_site = owner_free
            .source()
            .declaration_sites()
            .first()
            .copied()
            .ok_or(
                CallableCatalogOwnerSealErrorV1::CatalogCardinalityMismatch {
                    declarations: 0,
                    headers: 0,
                },
            )?;
        let mut resolver = FunctionSemanticResolverSessionV1::new(compilation_unit_ordinal)
            .map_err(
                |reason| CallableCatalogOwnerSealErrorV1::OwnerIssueExhausted {
                    site: first_site,
                    reason,
                },
            )?;
        let sites = owner_free.source().declaration_sites();
        let mut headers = Vec::with_capacity(sites.len());
        let mut declarations_by_site = BTreeMap::new();
        let mut compilation_brand = None;

        for &site in sites {
            let candidate = owner_free
                .candidate(site)
                .cloned()
                .ok_or(CallableCatalogOwnerSealErrorV1::MissingOwnerFreeCandidate { site })?;
            let (origin, owner) = resolver.issue_owner().map_err(|reason| {
                CallableCatalogOwnerSealErrorV1::OwnerIssueExhausted { site, reason }
            })?;
            match compilation_brand {
                Some(expected) if expected != owner.compilation_brand() => {
                    return Err(CallableCatalogOwnerSealErrorV1::MixedCompilationBrand)
                }
                None => compilation_brand = Some(owner.compilation_brand()),
                Some(_) => {}
            }
            let header = candidate.attach_owner(owner);
            let declaration = VerifiedCallableDeclarationV1 {
                site,
                origin,
                callable: header.callable(),
            };
            if declarations_by_site.insert(site, declaration).is_some() {
                return Err(CallableCatalogOwnerSealErrorV1::DuplicateDeclarationSite { site });
            }
            headers.push(header);
        }

        let index = VerifiedCallableIndexV1::seal_many(headers)
            .map_err(CallableCatalogOwnerSealErrorV1::CallableIndex)?;
        if declarations_by_site.len() != owner_free.candidate_count()
            || declarations_by_site.len() != index.len()
        {
            return Err(
                CallableCatalogOwnerSealErrorV1::CatalogCardinalityMismatch {
                    declarations: declarations_by_site.len(),
                    headers: index.len(),
                },
            );
        }

        Ok(Self {
            catalog: VerifiedCallableCatalogV1 {
                index,
                declarations_by_site,
            },
            resolver,
        })
    }

    pub(crate) fn commit(
        self,
        owner_free: VerifiedOwnerFreeCallableCatalogSourceUnitV1,
    ) -> CallableCatalogSealOutcomeV1 {
        let (source, _) = owner_free.into_parts();
        CallableCatalogSealOutcomeV1 {
            source_unit: VerifiedCallableCatalogSourceUnitV1 {
                source,
                catalog: self.catalog,
            },
            continuation: CatalogSealedResolverContinuationV1 {
                resolver: self.resolver,
            },
        }
    }
}
