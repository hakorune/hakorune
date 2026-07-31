//! Seal-derived exact identity bundles for statement `If` regions.
//!
//! This module owns no syntax or lowering policy. Its index builder is invoked
//! exactly once during seal, validates the authoritative If/branch topology,
//! and publishes an ID-only lookup witness inside `VerifiedResolvedFunctionV1`.

use std::collections::{BTreeMap, BTreeSet};

use super::ids::{RegionId, ScopeId};
use super::product::{
    ResolvedFunctionDataV1, ResolvedScopeRegionPairV1, VerifiedResolvedFunctionV1,
};
use super::records::{RegionKindV1, RegionOriginV1, ScopeKindV1, ScopeOriginV1};
use super::source_site::{SourcePathSegmentV1, SourcePathV1, SourceStmtSiteV1};
use super::verifier::exact_source_region_v1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ResolvedIfRegionBundleV1 {
    control: RegionId,
    then_pair: ResolvedScopeRegionPairV1,
    else_pair: Option<ResolvedScopeRegionPairV1>,
}

impl ResolvedIfRegionBundleV1 {
    pub(crate) const fn control(self) -> RegionId {
        self.control
    }

    pub(crate) const fn then_pair(self) -> ResolvedScopeRegionPairV1 {
        self.then_pair
    }

    pub(crate) const fn else_pair(self) -> Option<ResolvedScopeRegionPairV1> {
        self.else_pair
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ResolvedIfRegionLookupErrorV1 {
    MissingExactBundle(SourceStmtSiteV1),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolvedIfRegionVerificationErrorV1 {
    ControlContractMismatch(RegionId),
    MissingThenPair(RegionId),
    DuplicateThenPair(RegionId),
    DuplicateElsePair(RegionId),
    BranchContractMismatch(RegionId),
    OrphanBranchRegion(RegionId),
    OrphanBranchScope(ScopeId),
    DuplicateControlSite(SourceStmtSiteV1),
}

#[derive(Debug)]
pub(super) struct ResolvedIfRegionIndexV1 {
    by_site: BTreeMap<SourceStmtSiteV1, ResolvedIfRegionBundleV1>,
}

impl ResolvedIfRegionIndexV1 {
    fn get(&self, site: &SourceStmtSiteV1) -> Option<&ResolvedIfRegionBundleV1> {
        self.by_site.get(site)
    }

    fn len(&self) -> usize {
        self.by_site.len()
    }
}

impl VerifiedResolvedFunctionV1 {
    /// Looks up `site` relative to this product's function owner.
    ///
    /// `SourceStmtSiteV1` has no owner brand. Cross-owner closure is therefore
    /// proved by the future owner-closed RegionFlow input, not by this query.
    pub(crate) fn if_region_bundle(
        &self,
        site: &SourceStmtSiteV1,
    ) -> Result<&ResolvedIfRegionBundleV1, ResolvedIfRegionLookupErrorV1> {
        self.core
            .if_regions
            .get(site)
            .ok_or_else(|| ResolvedIfRegionLookupErrorV1::MissingExactBundle(site.clone()))
    }

    /// Returns only the sealed bundle cardinality for flow/site bijection.
    ///
    /// RegionFlow still has to prove every exact source site by point lookup;
    /// this count does not expose arena or index iteration as another authority.
    pub(crate) fn if_region_bundle_count(&self) -> usize {
        self.core.if_regions.len()
    }
}

pub(super) fn build_verified_if_region_index_v1(
    data: &ResolvedFunctionDataV1,
) -> Result<ResolvedIfRegionIndexV1, ResolvedIfRegionVerificationErrorV1> {
    let mut by_site = BTreeMap::new();
    let mut consumed_branch_regions = BTreeSet::new();
    let mut consumed_branch_scopes = BTreeSet::new();

    for (&control, control_record) in &data.regions {
        if control_record.kind() != RegionKindV1::If {
            continue;
        }
        let RegionOriginV1::Source(origin) = control_record.origin() else {
            return Err(ResolvedIfRegionVerificationErrorV1::ControlContractMismatch(control));
        };
        let site = SourceStmtSiteV1::from_node(origin.clone());
        let surrounding_region = exact_source_region_v1(data, site.node())
            .ok_or(ResolvedIfRegionVerificationErrorV1::ControlContractMismatch(control))?;
        let surrounding_scope = data
            .regions
            .get(&surrounding_region)
            .and_then(|record| record.lexical_scope())
            .ok_or(ResolvedIfRegionVerificationErrorV1::ControlContractMismatch(control))?;
        if control_record.lexical_scope().is_some()
            || control_record.parent() != Some(surrounding_region)
        {
            return Err(ResolvedIfRegionVerificationErrorV1::ControlContractMismatch(control));
        }

        let then_origin = branch_origin(&site, SourcePathSegmentV1::IfThenBody);
        let then_pair = required_branch_pair(
            data,
            control,
            surrounding_scope,
            RegionKindV1::IfThen,
            ScopeKindV1::IfThen,
            &then_origin,
        )?;
        account_pair(
            then_pair,
            &mut consumed_branch_regions,
            &mut consumed_branch_scopes,
        );

        let else_origin = branch_origin(&site, SourcePathSegmentV1::IfElseBody);
        let else_pair = optional_else_pair(data, control, surrounding_scope, &else_origin)?;
        if let Some(pair) = else_pair {
            account_pair(
                pair,
                &mut consumed_branch_regions,
                &mut consumed_branch_scopes,
            );
        }

        let bundle = ResolvedIfRegionBundleV1 {
            control,
            then_pair,
            else_pair,
        };
        if by_site.insert(site.clone(), bundle).is_some() {
            return Err(ResolvedIfRegionVerificationErrorV1::DuplicateControlSite(
                site,
            ));
        }
    }

    for (&region, record) in &data.regions {
        if matches!(record.kind(), RegionKindV1::IfThen | RegionKindV1::IfElse)
            && !consumed_branch_regions.contains(&region)
        {
            return Err(ResolvedIfRegionVerificationErrorV1::OrphanBranchRegion(
                region,
            ));
        }
    }
    for (&scope, record) in &data.scopes {
        if matches!(record.kind(), ScopeKindV1::IfThen | ScopeKindV1::IfElse)
            && !consumed_branch_scopes.contains(&scope)
        {
            return Err(ResolvedIfRegionVerificationErrorV1::OrphanBranchScope(
                scope,
            ));
        }
    }

    Ok(ResolvedIfRegionIndexV1 { by_site })
}

fn branch_origin(site: &SourceStmtSiteV1, role: SourcePathSegmentV1) -> super::SourceNodeSiteV1 {
    SourcePathV1::from_node(site.node()).child(role).node()
}

fn required_branch_pair(
    data: &ResolvedFunctionDataV1,
    control: RegionId,
    surrounding_scope: ScopeId,
    region_kind: RegionKindV1,
    scope_kind: ScopeKindV1,
    origin: &super::SourceNodeSiteV1,
) -> Result<ResolvedScopeRegionPairV1, ResolvedIfRegionVerificationErrorV1> {
    let matches = matching_branch_regions(data, region_kind, origin);
    match matches.as_slice() {
        [] => Err(ResolvedIfRegionVerificationErrorV1::MissingThenPair(
            control,
        )),
        [region] => verify_branch_pair(
            data,
            control,
            surrounding_scope,
            *region,
            scope_kind,
            origin,
        ),
        _ => Err(ResolvedIfRegionVerificationErrorV1::DuplicateThenPair(
            control,
        )),
    }
}

fn optional_else_pair(
    data: &ResolvedFunctionDataV1,
    control: RegionId,
    surrounding_scope: ScopeId,
    origin: &super::SourceNodeSiteV1,
) -> Result<Option<ResolvedScopeRegionPairV1>, ResolvedIfRegionVerificationErrorV1> {
    let matches = matching_branch_regions(data, RegionKindV1::IfElse, origin);
    match matches.as_slice() {
        [] => Ok(None),
        [region] => verify_branch_pair(
            data,
            control,
            surrounding_scope,
            *region,
            ScopeKindV1::IfElse,
            origin,
        )
        .map(Some),
        _ => Err(ResolvedIfRegionVerificationErrorV1::DuplicateElsePair(
            control,
        )),
    }
}

fn matching_branch_regions(
    data: &ResolvedFunctionDataV1,
    kind: RegionKindV1,
    origin: &super::SourceNodeSiteV1,
) -> Vec<RegionId> {
    data.regions
        .iter()
        .filter_map(|(region, record)| {
            (record.kind() == kind && record.origin() == &RegionOriginV1::Source(origin.clone()))
                .then_some(*region)
        })
        .collect()
}

fn verify_branch_pair(
    data: &ResolvedFunctionDataV1,
    control: RegionId,
    surrounding_scope: ScopeId,
    region: RegionId,
    scope_kind: ScopeKindV1,
    origin: &super::SourceNodeSiteV1,
) -> Result<ResolvedScopeRegionPairV1, ResolvedIfRegionVerificationErrorV1> {
    let region_record = data.regions.get(&region).ok_or(
        ResolvedIfRegionVerificationErrorV1::BranchContractMismatch(region),
    )?;
    let scope = region_record.lexical_scope().ok_or(
        ResolvedIfRegionVerificationErrorV1::BranchContractMismatch(region),
    )?;
    let scope_record = data.scopes.get(&scope).ok_or(
        ResolvedIfRegionVerificationErrorV1::BranchContractMismatch(region),
    )?;
    if region_record.parent() != Some(control)
        || region_record.origin() != &RegionOriginV1::Source(origin.clone())
        || scope_record.kind() != scope_kind
        || scope_record.parent() != Some(surrounding_scope)
        || scope_record.owner_region() != region
        || scope_record.origin() != &ScopeOriginV1::Source(origin.clone())
    {
        return Err(ResolvedIfRegionVerificationErrorV1::BranchContractMismatch(
            region,
        ));
    }
    Ok(ResolvedScopeRegionPairV1::from_verified(scope, region))
}

fn account_pair(
    pair: ResolvedScopeRegionPairV1,
    regions: &mut BTreeSet<RegionId>,
    scopes: &mut BTreeSet<ScopeId>,
) {
    regions.insert(pair.region());
    scopes.insert(pair.scope());
}
