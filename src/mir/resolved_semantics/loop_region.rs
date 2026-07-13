//! Seal-derived exact identity bundles for statement `Loop` regions.
//!
//! This module owns no syntax, flow, or lowering policy. Its index builder is
//! invoked exactly once during seal and publishes only an ID pair inside
//! `VerifiedResolvedFunctionV1`.

use std::collections::{BTreeMap, BTreeSet};

use super::ids::{RegionId, ScopeId};
use super::product::{
    ResolvedFunctionDataV1, ResolvedScopeRegionPairV1, VerifiedResolvedFunctionV1,
};
use super::records::{RegionKindV1, RegionOriginV1, ScopeKindV1, ScopeOriginV1};
use super::source_site::{SourcePathSegmentV1, SourcePathV1, SourceStmtSiteV1};
use super::verifier::exact_source_region_v1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ResolvedLoopRegionBundleV1 {
    loop_pair: ResolvedScopeRegionPairV1,
}

impl ResolvedLoopRegionBundleV1 {
    pub(crate) const fn loop_pair(self) -> ResolvedScopeRegionPairV1 {
        self.loop_pair
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ResolvedLoopRegionLookupErrorV1 {
    MissingExactBundle(SourceStmtSiteV1),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolvedLoopRegionVerificationErrorV1 {
    LoopContractMismatch(RegionId),
    MissingLoopBodyScope(RegionId),
    LoopBodyContractMismatch(ScopeId),
    OrphanLoopBodyScope(ScopeId),
    DuplicateLoopSite(SourceStmtSiteV1),
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct ResolvedLoopRegionIndexV1 {
    by_site: BTreeMap<SourceStmtSiteV1, ResolvedLoopRegionBundleV1>,
}

impl ResolvedLoopRegionIndexV1 {
    fn get(&self, site: &SourceStmtSiteV1) -> Option<&ResolvedLoopRegionBundleV1> {
        self.by_site.get(site)
    }

    fn len(&self) -> usize {
        self.by_site.len()
    }
}

impl VerifiedResolvedFunctionV1 {
    /// Looks up `site` relative to this product's function owner.
    pub(crate) fn loop_region_bundle(
        &self,
        site: &SourceStmtSiteV1,
    ) -> Result<&ResolvedLoopRegionBundleV1, ResolvedLoopRegionLookupErrorV1> {
        self.loop_regions
            .get(site)
            .ok_or_else(|| ResolvedLoopRegionLookupErrorV1::MissingExactBundle(site.clone()))
    }

    /// Returns only the sealed cardinality for future source/flow bijection.
    pub(crate) fn loop_region_bundle_count(&self) -> usize {
        self.loop_regions.len()
    }
}

pub(super) fn build_verified_loop_region_index_v1(
    data: &ResolvedFunctionDataV1,
) -> Result<ResolvedLoopRegionIndexV1, ResolvedLoopRegionVerificationErrorV1> {
    let mut by_site = BTreeMap::new();
    let mut consumed_loop_body_scopes = BTreeSet::new();

    for (&region, region_record) in &data.regions {
        if region_record.kind() != RegionKindV1::Loop {
            continue;
        }
        let RegionOriginV1::Source(origin) = region_record.origin() else {
            return Err(ResolvedLoopRegionVerificationErrorV1::LoopContractMismatch(
                region,
            ));
        };
        let site = SourceStmtSiteV1::from_node(origin.clone());
        let surrounding_region = exact_source_region_v1(data, site.node()).ok_or(
            ResolvedLoopRegionVerificationErrorV1::LoopContractMismatch(region),
        )?;
        let surrounding_scope = data
            .regions
            .get(&surrounding_region)
            .and_then(|record| record.lexical_scope())
            .ok_or(ResolvedLoopRegionVerificationErrorV1::LoopContractMismatch(
                region,
            ))?;
        if region_record.parent() != Some(surrounding_region) {
            return Err(ResolvedLoopRegionVerificationErrorV1::LoopContractMismatch(
                region,
            ));
        }

        let scope = region_record.lexical_scope().ok_or(
            ResolvedLoopRegionVerificationErrorV1::MissingLoopBodyScope(region),
        )?;
        let scope_record = data.scopes.get(&scope).ok_or(
            ResolvedLoopRegionVerificationErrorV1::MissingLoopBodyScope(region),
        )?;
        let body_origin = SourcePathV1::from_node(site.node())
            .child(SourcePathSegmentV1::LoopBodyRoot)
            .node();
        if scope_record.kind() != ScopeKindV1::LoopBody
            || scope_record.parent() != Some(surrounding_scope)
            || scope_record.owner_region() != region
            || scope_record.origin() != &ScopeOriginV1::Source(body_origin)
        {
            return Err(ResolvedLoopRegionVerificationErrorV1::LoopBodyContractMismatch(scope));
        }
        consumed_loop_body_scopes.insert(scope);

        let bundle = ResolvedLoopRegionBundleV1 {
            loop_pair: ResolvedScopeRegionPairV1::from_verified(scope, region),
        };
        if by_site.insert(site.clone(), bundle).is_some() {
            return Err(ResolvedLoopRegionVerificationErrorV1::DuplicateLoopSite(
                site,
            ));
        }
    }

    for (&scope, record) in &data.scopes {
        if record.kind() == ScopeKindV1::LoopBody && !consumed_loop_body_scopes.contains(&scope) {
            return Err(ResolvedLoopRegionVerificationErrorV1::OrphanLoopBodyScope(
                scope,
            ));
        }
    }

    Ok(ResolvedLoopRegionIndexV1 { by_site })
}
