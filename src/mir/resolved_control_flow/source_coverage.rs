//! Generic exact-source coverage schema for future canonical control products.
//!
//! This module does not navigate syntax or prove a control family's subtree
//! completeness. It verifies only the reusable owner/range/preorder shape.

use crate::mir::compiler::located::{
    ConsumedSourceRangeV1, LocatedBodyV1, LocatedExprV1, LocatedStmtV1, SourceBodySiteV1,
};
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, SourceExprSiteV1, SourceStmtSiteV1};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum CoveredSourceSiteV1 {
    Body(SourceBodySiteV1),
    Statement {
        owner: FunctionOwnerIdV1,
        site: SourceStmtSiteV1,
    },
    Expression {
        owner: FunctionOwnerIdV1,
        site: SourceExprSiteV1,
    },
}

impl CoveredSourceSiteV1 {
    pub(super) fn body(body: &LocatedBodyV1<'_>) -> Self {
        Self::Body(body.site().clone())
    }

    pub(super) fn statement(statement: &LocatedStmtV1<'_>) -> Self {
        Self::Statement {
            owner: statement.owner(),
            site: statement.site().clone(),
        }
    }

    pub(super) fn expression(expression: &LocatedExprV1<'_>) -> Self {
        Self::Expression {
            owner: expression.owner(),
            site: expression.site().clone(),
        }
    }

    const fn owner(&self) -> FunctionOwnerIdV1 {
        match self {
            Self::Body(body) => body.owner(),
            Self::Statement { owner, .. } | Self::Expression { owner, .. } => *owner,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum SourceCoverageVerificationErrorV1 {
    ForeignOuterOwner {
        expected: FunctionOwnerIdV1,
        actual: FunctionOwnerIdV1,
    },
    ForeignCoveredOwner {
        index: u32,
        expected: FunctionOwnerIdV1,
        actual: FunctionOwnerIdV1,
    },
    EmptyPreorder,
    CoverageIndexOverflow,
    DuplicateSite {
        first_index: u32,
        duplicate_index: u32,
    },
}

/// Reusable verified sidecar for a future family-specific co-sealed product.
///
/// It intentionally is not `Clone` and exposes neither a public constructor
/// nor `into_parts`.
#[derive(Debug, PartialEq, Eq)]
pub(super) struct VerifiedLocatedSourceCoverageV1 {
    outer: ConsumedSourceRangeV1,
    preorder: Box<[CoveredSourceSiteV1]>,
}

impl VerifiedLocatedSourceCoverageV1 {
    pub(super) fn owner(&self) -> FunctionOwnerIdV1 {
        self.outer.body().owner()
    }

    pub(super) const fn outer(&self) -> &ConsumedSourceRangeV1 {
        &self.outer
    }

    pub(super) const fn preorder(&self) -> &[CoveredSourceSiteV1] {
        &self.preorder
    }
}

pub(super) fn verify_located_source_coverage_v1(
    expected_owner: FunctionOwnerIdV1,
    outer: ConsumedSourceRangeV1,
    preorder: Vec<CoveredSourceSiteV1>,
) -> Result<VerifiedLocatedSourceCoverageV1, SourceCoverageVerificationErrorV1> {
    let actual_owner = outer.body().owner();
    if actual_owner != expected_owner {
        return Err(SourceCoverageVerificationErrorV1::ForeignOuterOwner {
            expected: expected_owner,
            actual: actual_owner,
        });
    }
    if preorder.is_empty() {
        return Err(SourceCoverageVerificationErrorV1::EmptyPreorder);
    }

    for (index, site) in preorder.iter().enumerate() {
        let index = checked_coverage_index(index)?;
        let actual = site.owner();
        if actual != expected_owner {
            return Err(SourceCoverageVerificationErrorV1::ForeignCoveredOwner {
                index,
                expected: expected_owner,
                actual,
            });
        }
    }

    for duplicate_index in 1..preorder.len() {
        if let Some(first_index) = preorder[..duplicate_index]
            .iter()
            .position(|site| site == &preorder[duplicate_index])
        {
            return Err(SourceCoverageVerificationErrorV1::DuplicateSite {
                first_index: checked_coverage_index(first_index)?,
                duplicate_index: checked_coverage_index(duplicate_index)?,
            });
        }
    }

    Ok(VerifiedLocatedSourceCoverageV1 {
        outer,
        preorder: preorder.into_boxed_slice(),
    })
}

fn checked_coverage_index(index: usize) -> Result<u32, SourceCoverageVerificationErrorV1> {
    u32::try_from(index).map_err(|_| SourceCoverageVerificationErrorV1::CoverageIndexOverflow)
}
