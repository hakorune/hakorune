//! Seal-derived exact function and function-body lowering roots.
//!
//! The authoritative records remain in the scope/region arenas. This owned
//! ID-only carrier is a rebuildable seal witness so Lower never discovers the
//! body `Sequence` root by scanning those arenas.

use super::ids::{RegionId, ScopeId};
use super::product::{ResolvedFunctionDataV1, ResolvedScopeRegionPairV1};
use super::records::{RegionKindV1, RegionOriginV1, ScopeKindV1, ScopeOriginV1};
use super::source_site::SourcePathSegmentV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ResolvedFunctionLoweringRootsV1 {
    function_pair: ResolvedScopeRegionPairV1,
    body_pair: ResolvedScopeRegionPairV1,
}

impl ResolvedFunctionLoweringRootsV1 {
    pub(crate) const fn function_pair(self) -> ResolvedScopeRegionPairV1 {
        self.function_pair
    }

    pub(crate) const fn body_pair(self) -> ResolvedScopeRegionPairV1 {
        self.body_pair
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ResolvedFunctionRootVerificationErrorV1 {
    FunctionPairContractMismatch,
    BodyRegionCardinality { actual: usize },
    BodyScopeCardinality { actual: usize },
    BodyPairContractMismatch,
}

pub(super) fn build_verified_function_lowering_roots_v1(
    data: &ResolvedFunctionDataV1,
) -> Result<ResolvedFunctionLoweringRootsV1, ResolvedFunctionRootVerificationErrorV1> {
    verify_function_pair(data)?;

    let body_regions = data
        .regions
        .iter()
        .filter(|(_, record)| is_body_region_origin(record.origin()))
        .collect::<Vec<_>>();
    let [(&body_region, body_region_record)] = body_regions.as_slice() else {
        return Err(
            ResolvedFunctionRootVerificationErrorV1::BodyRegionCardinality {
                actual: body_regions.len(),
            },
        );
    };
    let body_scopes = data
        .scopes
        .iter()
        .filter(|(_, record)| is_body_scope_origin(record.origin()))
        .collect::<Vec<_>>();
    let [(&body_scope, body_scope_record)] = body_scopes.as_slice() else {
        return Err(
            ResolvedFunctionRootVerificationErrorV1::BodyScopeCardinality {
                actual: body_scopes.len(),
            },
        );
    };

    let origins_match = match (body_scope_record.origin(), body_region_record.origin()) {
        (ScopeOriginV1::Source(scope), RegionOriginV1::Source(region)) => scope == region,
        _ => false,
    };
    if body_region_record.kind() != RegionKindV1::Sequence
        || body_region_record.parent() != Some(data.function_region)
        || body_region_record.lexical_scope() != Some(body_scope)
        || body_scope_record.kind() != ScopeKindV1::LexicalBlock
        || body_scope_record.parent() != Some(data.function_scope)
        || body_scope_record.owner_region() != body_region
        || !origins_match
    {
        return Err(ResolvedFunctionRootVerificationErrorV1::BodyPairContractMismatch);
    }

    Ok(ResolvedFunctionLoweringRootsV1 {
        function_pair: ResolvedScopeRegionPairV1::from_verified(
            data.function_scope,
            data.function_region,
        ),
        body_pair: ResolvedScopeRegionPairV1::from_verified(body_scope, body_region),
    })
}

fn verify_function_pair(
    data: &ResolvedFunctionDataV1,
) -> Result<(), ResolvedFunctionRootVerificationErrorV1> {
    let Some(scope) = data.scopes.get(&data.function_scope) else {
        return Err(ResolvedFunctionRootVerificationErrorV1::FunctionPairContractMismatch);
    };
    let Some(region) = data.regions.get(&data.function_region) else {
        return Err(ResolvedFunctionRootVerificationErrorV1::FunctionPairContractMismatch);
    };
    if data.function_scope.owner() != data.owner
        || data.function_region.owner() != data.owner
        || scope.kind() != ScopeKindV1::Function
        || scope.parent().is_some()
        || scope.owner_region() != data.function_region
        || scope.origin() != &ScopeOriginV1::Function(data.function_origin)
        || region.kind() != RegionKindV1::Function
        || region.parent().is_some()
        || region.lexical_scope() != Some(data.function_scope)
        || region.origin() != &RegionOriginV1::Function(data.function_origin)
    {
        return Err(ResolvedFunctionRootVerificationErrorV1::FunctionPairContractMismatch);
    }
    Ok(())
}

fn is_body_region_origin(origin: &RegionOriginV1) -> bool {
    matches!(
        origin,
        RegionOriginV1::Source(site) if is_exact_body_root(site.segments())
    )
}

fn is_body_scope_origin(origin: &ScopeOriginV1) -> bool {
    matches!(
        origin,
        ScopeOriginV1::Source(site) if is_exact_body_root(site.segments())
    )
}

fn is_exact_body_root(segments: &[SourcePathSegmentV1]) -> bool {
    matches!(
        segments,
        [SourcePathSegmentV1::FunctionBody] | [SourcePathSegmentV1::LambdaBodyRoot]
    )
}
