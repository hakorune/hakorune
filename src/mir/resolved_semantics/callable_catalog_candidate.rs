//! Owner-free exact callable candidates for CAT0-C0a.
//!
//! This layer consumes the S0 Program-owned source unit and rejects every
//! profile/key/symbol conflict before C0b is allowed to issue callable owner
//! identities.

use std::collections::BTreeMap;

use super::{
    CallableIndexSealErrorV1, CanonicalCallableKeyV1, CanonicalCallableSymbolV1,
    SourceCallableDeclarationSiteV1, VerifiedCallableCatalogSourceUnitV1,
    VerifiedOwnerFreeCallableHeaderV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CallableCatalogCandidateSealErrorV1 {
    HeaderOutsideExactI64Profile {
        site: SourceCallableDeclarationSiteV1,
        reason: CallableIndexSealErrorV1,
    },
    DuplicateSourceKey {
        key: CanonicalCallableKeyV1,
        first_site: SourceCallableDeclarationSiteV1,
        second_site: SourceCallableDeclarationSiteV1,
    },
    PhysicalSymbolCollision {
        symbol: CanonicalCallableSymbolV1,
        first_site: SourceCallableDeclarationSiteV1,
        second_site: SourceCallableDeclarationSiteV1,
    },
    MissingVerifiedHeader {
        site: SourceCallableDeclarationSiteV1,
    },
    DuplicateDeclarationSite {
        site: SourceCallableDeclarationSiteV1,
    },
}

#[derive(Debug)]
pub(crate) struct VerifiedOwnerFreeCallableCatalogSourceUnitV1 {
    source: VerifiedCallableCatalogSourceUnitV1,
    candidates_by_site:
        BTreeMap<SourceCallableDeclarationSiteV1, VerifiedOwnerFreeCallableHeaderV1>,
    site_by_key: BTreeMap<CanonicalCallableKeyV1, SourceCallableDeclarationSiteV1>,
    site_by_symbol: BTreeMap<CanonicalCallableSymbolV1, SourceCallableDeclarationSiteV1>,
}

impl VerifiedOwnerFreeCallableCatalogSourceUnitV1 {
    pub(crate) fn seal(
        source: VerifiedCallableCatalogSourceUnitV1,
    ) -> Result<Self, CallableCatalogCandidateSealErrorV1> {
        let mut candidates_by_site = BTreeMap::new();
        let mut site_by_key = BTreeMap::new();
        let mut site_by_symbol = BTreeMap::new();

        for &site in source.declaration_sites() {
            let located = source
                .located_header(site)
                .ok_or(CallableCatalogCandidateSealErrorV1::MissingVerifiedHeader { site })?;
            let candidate =
                VerifiedOwnerFreeCallableHeaderV1::seal(located.header()).map_err(|reason| {
                    CallableCatalogCandidateSealErrorV1::HeaderOutsideExactI64Profile {
                        site,
                        reason,
                    }
                })?;

            if let Some(first_site) = site_by_key.insert(candidate.source_key().clone(), site) {
                return Err(CallableCatalogCandidateSealErrorV1::DuplicateSourceKey {
                    key: candidate.source_key().clone(),
                    first_site,
                    second_site: site,
                });
            }
            if let Some(first_site) = site_by_symbol.insert(candidate.symbol().clone(), site) {
                return Err(
                    CallableCatalogCandidateSealErrorV1::PhysicalSymbolCollision {
                        symbol: candidate.symbol().clone(),
                        first_site,
                        second_site: site,
                    },
                );
            }
            if candidates_by_site.insert(site, candidate).is_some() {
                return Err(CallableCatalogCandidateSealErrorV1::DuplicateDeclarationSite { site });
            }
        }

        Ok(Self {
            source,
            candidates_by_site,
            site_by_key,
            site_by_symbol,
        })
    }

    pub(crate) fn candidate_count(&self) -> usize {
        self.candidates_by_site.len()
    }

    pub(crate) fn candidate(
        &self,
        site: SourceCallableDeclarationSiteV1,
    ) -> Option<&VerifiedOwnerFreeCallableHeaderV1> {
        self.candidates_by_site.get(&site)
    }

    pub(crate) fn source(&self) -> &VerifiedCallableCatalogSourceUnitV1 {
        &self.source
    }

    pub(crate) fn source_site_for_key(
        &self,
        key: &CanonicalCallableKeyV1,
    ) -> Option<SourceCallableDeclarationSiteV1> {
        self.site_by_key.get(key).copied()
    }

    pub(crate) fn source_site_for_symbol(
        &self,
        symbol: &CanonicalCallableSymbolV1,
    ) -> Option<SourceCallableDeclarationSiteV1> {
        self.site_by_symbol.get(symbol).copied()
    }
}
