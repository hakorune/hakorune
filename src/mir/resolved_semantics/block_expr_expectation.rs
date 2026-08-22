//! Resolver-owned typed expectation for resolved `BlockExpr` sites.
//!
//! This product is intentionally smaller than a lowering plan.  It proves
//! that the source body-shape inventory and the resolved scope/region arena
//! contain the same exact BlockExpr set; consumers must not reconstruct a
//! count from strings, AST, MIR, or arena lengths alone.

use std::collections::BTreeSet;

use super::body_shape::{BodyExpressionShapeV1, VerifiedResolvedBodyShapeInventoryV1};
use super::ids::FunctionOwnerIdV1;
use super::product::VerifiedResolvedFunctionV1;
use super::records::{RegionKindV1, RegionOriginV1, ScopeKindV1, ScopeOriginV1};
use super::source_site::{
    FunctionOriginV1, SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1, SourcePathV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ResolvedBlockExpressionExpectationIssueV1 {
    OwnerMismatch,
    BodyRootMismatch,
    DuplicateSourceSite(SourceExprSiteV1),
    MissingExactPair(SourceExprSiteV1),
    PairContractMismatch(SourceExprSiteV1),
    NonSourceArenaOrigin,
    PairCoverageMismatch {
        source: usize,
        scopes: usize,
        regions: usize,
    },
    CountOverflow,
}

/// A non-reconstructible count receipt for one resolver-sealed function body.
#[derive(Debug)]
pub(crate) struct VerifiedResolvedBlockExpressionExpectationV1 {
    owner: FunctionOwnerIdV1,
    function_origin: FunctionOriginV1,
    body_root: SourcePathSegmentV1,
    pair_count: u32,
}

impl VerifiedResolvedBlockExpressionExpectationV1 {
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) const fn function_origin(&self) -> FunctionOriginV1 {
        self.function_origin
    }

    pub(crate) fn body_root(&self) -> SourcePathSegmentV1 {
        self.body_root.clone()
    }

    pub(crate) const fn pair_count(&self) -> u32 {
        self.pair_count
    }
}

pub(crate) fn issue_resolved_block_expr_expectation_v1(
    function: &VerifiedResolvedFunctionV1,
    body_shape: &VerifiedResolvedBodyShapeInventoryV1,
) -> Result<VerifiedResolvedBlockExpressionExpectationV1, ResolvedBlockExpressionExpectationIssueV1>
{
    if function.owner() != body_shape.owner() {
        return Err(ResolvedBlockExpressionExpectationIssueV1::OwnerMismatch);
    }
    if function.root_profile().body_root() != *body_shape.body_root() {
        return Err(ResolvedBlockExpressionExpectationIssueV1::BodyRootMismatch);
    }

    let mut source_origins = BTreeSet::<SourceNodeSiteV1>::new();
    for site in body_shape.expressions().iter().filter_map(|expression| {
        matches!(expression, BodyExpressionShapeV1::BlockExpr { .. }).then(|| match expression {
            BodyExpressionShapeV1::BlockExpr { site } => site.clone(),
            _ => unreachable!(),
        })
    }) {
        let origin = SourcePathV1::from_node(site.node())
            .child(SourcePathSegmentV1::BlockExprPreludeRoot)
            .node();
        if !source_origins.insert(origin) {
            return Err(ResolvedBlockExpressionExpectationIssueV1::DuplicateSourceSite(site));
        }
        match function.block_expr_scope_region_pair(function.owner(), &site) {
            Ok(_) => {}
            Err(super::product::ResolvedScopeRegionLookupErrorV1::MissingExactPair) => {
                return Err(ResolvedBlockExpressionExpectationIssueV1::MissingExactPair(
                    site,
                ));
            }
            Err(_) => {
                return Err(ResolvedBlockExpressionExpectationIssueV1::PairContractMismatch(site));
            }
        }
    }

    let mut scope_rows = Vec::new();
    for (_, record) in function
        .scopes()
        .filter(|(_, record)| record.kind() == ScopeKindV1::BlockExpr)
    {
        let ScopeOriginV1::Source(origin) = record.origin() else {
            return Err(ResolvedBlockExpressionExpectationIssueV1::NonSourceArenaOrigin);
        };
        scope_rows.push(origin.clone());
    }
    let mut region_rows = Vec::new();
    for (_, record) in function
        .regions()
        .filter(|(_, record)| record.kind() == RegionKindV1::BlockExpr)
    {
        let RegionOriginV1::Source(origin) = record.origin() else {
            return Err(ResolvedBlockExpressionExpectationIssueV1::NonSourceArenaOrigin);
        };
        region_rows.push(origin.clone());
    }
    let scope_origins = scope_rows.iter().cloned().collect::<BTreeSet<_>>();
    let region_origins = region_rows.iter().cloned().collect::<BTreeSet<_>>();

    if source_origins.len() != scope_rows.len()
        || source_origins.len() != region_rows.len()
        || source_origins != scope_origins
        || source_origins != region_origins
    {
        return Err(
            ResolvedBlockExpressionExpectationIssueV1::PairCoverageMismatch {
                source: source_origins.len(),
                scopes: scope_rows.len(),
                regions: region_rows.len(),
            },
        );
    }

    let pair_count = u32::try_from(source_origins.len())
        .map_err(|_| ResolvedBlockExpressionExpectationIssueV1::CountOverflow)?;
    Ok(VerifiedResolvedBlockExpressionExpectationV1 {
        owner: function.owner(),
        function_origin: function.function_origin(),
        body_root: function.root_profile().body_root(),
        pair_count,
    })
}
