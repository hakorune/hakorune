//! Seal-derived exact identity bundles for statement `Loop` regions.
//!
//! This module owns no syntax, flow, or lowering policy. Its index builder is
//! invoked exactly once during seal and publishes only an ID pair inside
//! `VerifiedResolvedFunctionV1`.

use std::collections::{BTreeMap, BTreeSet};

use super::ids::{RegionId, ScopeId};
use super::owner_source_kind::SemanticOwnerSourceKindV1;
use super::product::{
    ResolvedFunctionDataV1, ResolvedScopeRegionPairV1, VerifiedResolvedFunctionV1,
};
use super::records::{RegionKindV1, RegionOriginV1, ScopeKindV1, ScopeOriginV1};
use super::source_site::{
    FunctionOriginV1, SourceNodeSiteV1, SourcePathSegmentV1, SourcePathV1, SourceStmtSiteV1,
};
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

/// Owner-branded source identity for one Loop admitted by the sealed index.
///
/// This token is intentionally non-`Clone`: a downstream portable-source
/// adapter must consume the exact lookup result rather than minting source
/// authority from a route-local AST view or facts projection.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedResolvedLoopSourceV1 {
    function_origin: FunctionOriginV1,
    owner_source_kind: SemanticOwnerSourceKindV1,
    site: SourceStmtSiteV1,
}

/// One exact member of a Nested Loop source forest.
///
/// The parent index is local to this consumed forest. It is not a semantic
/// recipe key and must not be confused with a MIR block/value identity.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedResolvedLoopSourceForestMemberV1 {
    source: VerifiedResolvedLoopSourceV1,
    parent_index: Option<u32>,
}

impl VerifiedResolvedLoopSourceForestMemberV1 {
    pub(crate) fn source(&self) -> &VerifiedResolvedLoopSourceV1 {
        &self.source
    }

    pub(crate) const fn parent_index(&self) -> Option<u32> {
        self.parent_index
    }

    pub(crate) fn into_source(self) -> VerifiedResolvedLoopSourceV1 {
        self.source
    }
}

/// Non-`Clone`, source-owned preorder witness for one Loop and all nested Loop
/// sites below it. The resolver issues the members; consumers only consume it.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedResolvedLoopSourceForestV1 {
    members: Box<[VerifiedResolvedLoopSourceForestMemberV1]>,
}

impl VerifiedResolvedLoopSourceForestV1 {
    pub(crate) fn members(&self) -> &[VerifiedResolvedLoopSourceForestMemberV1] {
        &self.members
    }

    pub(crate) fn into_members(self) -> Box<[VerifiedResolvedLoopSourceForestMemberV1]> {
        self.members
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ResolvedLoopSourceForestRejectV1 {
    UnsupportedOwnerRoot(SemanticOwnerSourceKindV1),
    MissingRoot(SourceStmtSiteV1),
    DuplicateSite(SourceStmtSiteV1),
    OrphanDescendant(SourceStmtSiteV1),
    SkippedIntermediateLoop(SourceStmtSiteV1),
    UnsupportedAncestry {
        site: SourceStmtSiteV1,
        segment: SourcePathSegmentV1,
    },
}

/// Opaque execution-frame identity shared by one selected Loop handoff.
///
/// This is stronger than a route cursor and separate from MIR identity.  Only
/// the sealed resolved-source lookup can issue it; consumers may compare it,
/// but cannot construct a key from a route id or raw source index.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LoopExecutionFrameKeyV1 {
    function_origin: FunctionOriginV1,
    owner_source_kind: SemanticOwnerSourceKindV1,
    site: SourceStmtSiteV1,
    _seal: LoopExecutionFrameKeySealV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LoopExecutionFrameKeySealV1;

impl LoopExecutionFrameKeyV1 {
    fn from_source(source: &VerifiedResolvedLoopSourceV1) -> Self {
        Self {
            function_origin: source.function_origin,
            owner_source_kind: source.owner_source_kind,
            site: source.site.clone(),
            _seal: LoopExecutionFrameKeySealV1,
        }
    }

    pub(crate) fn matches(&self, other: &Self) -> bool {
        self == other
    }
}

#[cfg(test)]
pub(crate) fn loop_execution_frame_key_for_test() -> LoopExecutionFrameKeyV1 {
    LoopExecutionFrameKeyV1 {
        function_origin: FunctionOriginV1::new(0, 0),
        owner_source_kind: SemanticOwnerSourceKindV1::DeclaredFunction,
        site: SourceStmtSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
            SourcePathSegmentV1::Body(0),
        ])),
        _seal: LoopExecutionFrameKeySealV1,
    }
}

impl VerifiedResolvedLoopSourceV1 {
    /// Checks an independently-owned identity witness without exposing the
    /// source capability's fields or consuming its ownership.
    pub(crate) fn matches_identity(
        &self,
        function_origin: FunctionOriginV1,
        owner_source_kind: SemanticOwnerSourceKindV1,
        site: &SourceStmtSiteV1,
    ) -> bool {
        self.function_origin == function_origin
            && self.owner_source_kind == owner_source_kind
            && &self.site == site
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        FunctionOriginV1,
        SemanticOwnerSourceKindV1,
        SourceStmtSiteV1,
    ) {
        (self.function_origin, self.owner_source_kind, self.site)
    }

    pub(crate) fn site(&self) -> &SourceStmtSiteV1 {
        &self.site
    }

    pub(crate) fn frame_key(&self) -> LoopExecutionFrameKeyV1 {
        LoopExecutionFrameKeyV1::from_source(self)
    }
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

    fn sites(&self) -> impl Iterator<Item = &SourceStmtSiteV1> {
        self.by_site.keys()
    }
}

impl VerifiedResolvedFunctionV1 {
    /// Looks up `site` relative to this product's function owner.
    pub(crate) fn loop_region_bundle(
        &self,
        site: &SourceStmtSiteV1,
    ) -> Result<&ResolvedLoopRegionBundleV1, ResolvedLoopRegionLookupErrorV1> {
        self.core
            .loop_regions
            .get(site)
            .ok_or_else(|| ResolvedLoopRegionLookupErrorV1::MissingExactBundle(site.clone()))
    }

    /// Returns only the sealed cardinality for future source/flow bijection.
    pub(crate) fn loop_region_bundle_count(&self) -> usize {
        self.core.loop_regions.len()
    }

    /// Issues exact Loop source identity only after the sealed site lookup.
    pub(crate) fn resolved_loop_source(
        &self,
        site: &SourceStmtSiteV1,
    ) -> Result<VerifiedResolvedLoopSourceV1, ResolvedLoopRegionLookupErrorV1> {
        self.loop_region_bundle(site)?;
        Ok(VerifiedResolvedLoopSourceV1 {
            function_origin: self.function_origin(),
            owner_source_kind: self.source_kind(),
            site: site.clone(),
        })
    }

    /// Issues a consuming root+child source witness for Nested recipes.
    ///
    /// The loop-region index is the only source authority. Descendant
    /// membership and parentage are derived from its sealed source sites; no
    /// AST or route-local path reconstruction is allowed here.
    pub(crate) fn resolved_loop_source_forest(
        &self,
        root: &SourceStmtSiteV1,
    ) -> Result<VerifiedResolvedLoopSourceForestV1, ResolvedLoopSourceForestRejectV1> {
        if self.source_kind() != SemanticOwnerSourceKindV1::DeclaredFunction {
            return Err(ResolvedLoopSourceForestRejectV1::UnsupportedOwnerRoot(
                self.source_kind(),
            ));
        }
        if !self.core.loop_regions.by_site.contains_key(root) {
            return Err(ResolvedLoopSourceForestRejectV1::MissingRoot(root.clone()));
        }

        let root_segments = root.node().segments();
        let mut sites = self
            .core
            .loop_regions
            .sites()
            .filter(|site| site.node().segments().starts_with(root_segments))
            .cloned()
            .collect::<Vec<_>>();
        sites.sort_by(|left, right| left.node().segments().cmp(right.node().segments()));
        build_source_forest(self, root, sites)
    }
}

fn build_source_forest(
    owner: &VerifiedResolvedFunctionV1,
    root: &SourceStmtSiteV1,
    sites: Vec<SourceStmtSiteV1>,
) -> Result<VerifiedResolvedLoopSourceForestV1, ResolvedLoopSourceForestRejectV1> {
    let mut positions = BTreeMap::new();
    for (position, site) in sites.iter().enumerate() {
        if positions.insert(site.clone(), position).is_some() {
            return Err(ResolvedLoopSourceForestRejectV1::DuplicateSite(
                site.clone(),
            ));
        }
    }
    if sites.first() != Some(root) {
        return Err(ResolvedLoopSourceForestRejectV1::MissingRoot(root.clone()));
    }

    let mut members = Vec::with_capacity(sites.len());
    for site in &sites {
        let parent_index = forest_parent_index(root, site, &sites, &positions)?;
        let source = owner
            .resolved_loop_source(site)
            .map_err(|_| ResolvedLoopSourceForestRejectV1::MissingRoot(site.clone()))?;
        members.push(VerifiedResolvedLoopSourceForestMemberV1 {
            source,
            parent_index,
        });
    }
    Ok(VerifiedResolvedLoopSourceForestV1 {
        members: members.into_boxed_slice(),
    })
}

fn forest_parent_index(
    root: &SourceStmtSiteV1,
    site: &SourceStmtSiteV1,
    sites: &[SourceStmtSiteV1],
    positions: &BTreeMap<SourceStmtSiteV1, usize>,
) -> Result<Option<u32>, ResolvedLoopSourceForestRejectV1> {
    if site == root {
        return Ok(None);
    }
    let root_segments = root.node().segments();
    let segments = site.node().segments();
    let Some(relative) = segments.get(root_segments.len()..) else {
        return Err(ResolvedLoopSourceForestRejectV1::OrphanDescendant(
            site.clone(),
        ));
    };
    if !segments.starts_with(root_segments)
        || !matches!(relative.first(), Some(SourcePathSegmentV1::LoopBody(_)))
    {
        return Err(ResolvedLoopSourceForestRejectV1::OrphanDescendant(
            site.clone(),
        ));
    }
    if let Some(segment) = relative.iter().skip(1).find(|segment| {
        !matches!(
            segment,
            SourcePathSegmentV1::ScopeBody(_) | SourcePathSegmentV1::LoopBody(_)
        )
    }) {
        return Err(ResolvedLoopSourceForestRejectV1::UnsupportedAncestry {
            site: site.clone(),
            segment: segment.clone(),
        });
    }

    let mut best: Option<(&SourceStmtSiteV1, usize)> = None;
    for ancestor in sites.iter().filter(|ancestor| *ancestor != site) {
        let ancestor_segments = ancestor.node().segments();
        let Some(suffix) = segments.get(ancestor_segments.len()..) else {
            continue;
        };
        if !segments.starts_with(ancestor_segments)
            || !matches!(suffix.first(), Some(SourcePathSegmentV1::LoopBody(_)))
            || suffix
                .iter()
                .skip(1)
                .any(|segment| !matches!(segment, SourcePathSegmentV1::ScopeBody(_)))
        {
            continue;
        }
        if best
            .as_ref()
            .is_none_or(|(_, length)| ancestor_segments.len() > *length)
        {
            best = Some((ancestor, ancestor_segments.len()));
        }
    }

    let Some((parent, _)) = best else {
        return if relative
            .iter()
            .filter(|segment| matches!(segment, SourcePathSegmentV1::LoopBody(_)))
            .count()
            > 1
        {
            Err(ResolvedLoopSourceForestRejectV1::SkippedIntermediateLoop(
                site.clone(),
            ))
        } else {
            Err(ResolvedLoopSourceForestRejectV1::OrphanDescendant(
                site.clone(),
            ))
        };
    };
    Ok(Some(
        *positions
            .get(parent)
            .expect("validated source forest parent must be indexed") as u32,
    ))
}

#[cfg(test)]
mod source_forest_tests {
    use super::*;

    fn site(segments: &[SourcePathSegmentV1]) -> SourceStmtSiteV1 {
        SourceStmtSiteV1::from_node(SourceNodeSiteV1::from_segments(segments.to_vec()))
    }

    #[test]
    fn forest_parent_accepts_scope_steps_after_nested_loop_step() {
        let root = site(&[SourcePathSegmentV1::Body(0)]);
        let child = site(&[
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::LoopBody(0),
            SourcePathSegmentV1::ScopeBody(0),
        ]);
        let sites = vec![root.clone(), child.clone()];
        let positions = BTreeMap::from([(root.clone(), 0), (child, 1)]);

        assert_eq!(
            forest_parent_index(&root, &sites[1], &sites, &positions),
            Ok(Some(0))
        );
    }

    #[test]
    fn forest_parent_rejects_skipped_intermediate_loop() {
        let root = site(&[SourcePathSegmentV1::Body(0)]);
        let skipped = site(&[
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::LoopBody(0),
            SourcePathSegmentV1::LoopBody(0),
        ]);
        let sites = vec![root.clone(), skipped.clone()];
        let positions = BTreeMap::from([(root.clone(), 0), (skipped.clone(), 1)]);

        assert_eq!(
            forest_parent_index(&root, &skipped, &sites, &positions),
            Err(ResolvedLoopSourceForestRejectV1::SkippedIntermediateLoop(
                skipped,
            ))
        );
    }

    #[test]
    fn forest_parent_rejects_unsupported_ancestry_and_orphan_scope() {
        let root = site(&[SourcePathSegmentV1::Body(0)]);
        let unsupported = site(&[
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::LoopBody(0),
            SourcePathSegmentV1::IfThen(0),
        ]);
        let orphan = site(&[
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::ScopeBody(0),
        ]);
        let unsupported_sites = vec![root.clone(), unsupported.clone()];
        let orphan_sites = vec![root.clone(), orphan.clone()];
        let unsupported_positions = BTreeMap::from([(root.clone(), 0), (unsupported.clone(), 1)]);
        let orphan_positions = BTreeMap::from([(root.clone(), 0), (orphan.clone(), 1)]);

        assert_eq!(
            forest_parent_index(
                &root,
                &unsupported,
                &unsupported_sites,
                &unsupported_positions
            ),
            Err(ResolvedLoopSourceForestRejectV1::UnsupportedAncestry {
                site: unsupported,
                segment: SourcePathSegmentV1::IfThen(0),
            })
        );
        assert_eq!(
            forest_parent_index(&root, &orphan, &orphan_sites, &orphan_positions),
            Err(ResolvedLoopSourceForestRejectV1::OrphanDescendant(orphan))
        );
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
