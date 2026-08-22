use super::super::source_coverage::{CoveredSourceSiteV1, VerifiedLocatedSourceCoverageV1};
use super::use_ledger::{
    FunctionIfControlUseLedgerV1, IfControlCoverageUseErrorV1, IfControlCoverageUseV1,
};
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::located::{LocatedBodyV1, LocatedExprV1, LocatedStmtV1};
use crate::mir::resolved_semantics::{
    FunctionOwnerIdV1, ResolvedIfRegionBundleV1, SourceStmtSiteV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ResolvedIfFallthroughPortV1(());

impl ResolvedIfFallthroughPortV1 {
    pub(in crate::mir::resolved_control_flow) const fn verified() -> Self {
        Self(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ResolvedIfElsePortV1 {
    ImplicitIdentity,
    Explicit(ResolvedIfFallthroughPortV1),
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::resolved_control_flow) struct VerifiedLocatedIfControlV1 {
    pub(in crate::mir::resolved_control_flow) site: SourceStmtSiteV1,
    pub(in crate::mir::resolved_control_flow) regions: ResolvedIfRegionBundleV1,
    pub(in crate::mir::resolved_control_flow) then_port: ResolvedIfFallthroughPortV1,
    pub(in crate::mir::resolved_control_flow) else_port: ResolvedIfElsePortV1,
    pub(in crate::mir::resolved_control_flow) coverage: VerifiedLocatedSourceCoverageV1,
}

impl VerifiedLocatedIfControlV1 {
    pub(in crate::mir::resolved_control_flow) const fn site(&self) -> &SourceStmtSiteV1 {
        &self.site
    }

    pub(in crate::mir::resolved_control_flow) const fn regions(&self) -> ResolvedIfRegionBundleV1 {
        self.regions
    }

    pub(in crate::mir::resolved_control_flow) const fn then_port(
        &self,
    ) -> ResolvedIfFallthroughPortV1 {
        self.then_port
    }

    pub(in crate::mir::resolved_control_flow) const fn else_port(&self) -> ResolvedIfElsePortV1 {
        self.else_port
    }

    pub(in crate::mir::resolved_control_flow) const fn outer_range(
        &self,
    ) -> &crate::mir::compiler::located::ConsumedSourceRangeV1 {
        self.coverage.outer()
    }

    pub(in crate::mir::resolved_control_flow) const fn coverage_preorder(
        &self,
    ) -> &[CoveredSourceSiteV1] {
        self.coverage.preorder()
    }

    pub(in crate::mir::resolved_control_flow) fn coverage_use(&self) -> IfControlCoverageUseV1 {
        IfControlCoverageUseV1 {
            owner: self.coverage.owner(),
            expected: self.coverage.preorder().into(),
            next: 0,
        }
    }

    pub(super) fn into_materialization(self) -> ResolvedIfControlMaterializationV1 {
        ResolvedIfControlMaterializationV1 {
            site: self.site,
            regions: self.regions,
            then_port: self.then_port,
            else_port: self.else_port,
            outer_range: self.coverage.outer().clone(),
            coverage: IfControlCoverageUseV1 {
                owner: self.coverage.owner(),
                expected: self.coverage.preorder().into(),
                next: 0,
            },
        }
    }
}

/// Owned exact row handed to one canonical If materializer.
///
/// It contains control identity and source coverage only. Binding effects and
/// MIR value/block materialization remain outside this product.
#[derive(Debug)]
pub(crate) struct ResolvedIfControlMaterializationV1 {
    site: SourceStmtSiteV1,
    regions: ResolvedIfRegionBundleV1,
    then_port: ResolvedIfFallthroughPortV1,
    else_port: ResolvedIfElsePortV1,
    outer_range: crate::mir::compiler::located::ConsumedSourceRangeV1,
    coverage: IfControlCoverageUseV1,
}

impl ResolvedIfControlMaterializationV1 {
    pub(crate) const fn site(&self) -> &SourceStmtSiteV1 {
        &self.site
    }

    pub(crate) const fn regions(&self) -> ResolvedIfRegionBundleV1 {
        self.regions
    }

    pub(crate) const fn then_port(&self) -> ResolvedIfFallthroughPortV1 {
        self.then_port
    }

    pub(crate) const fn else_port(&self) -> ResolvedIfElsePortV1 {
        self.else_port
    }

    pub(crate) const fn outer_range(
        &self,
    ) -> &crate::mir::compiler::located::ConsumedSourceRangeV1 {
        &self.outer_range
    }

    pub(crate) fn claim_statement(
        &mut self,
        statement: &LocatedStmtV1<'_>,
    ) -> Result<(), IfControlCoverageUseErrorV1> {
        self.coverage
            .claim(&CoveredSourceSiteV1::statement(statement))
    }

    pub(crate) fn claim_body(
        &mut self,
        body: &LocatedBodyV1<'_>,
    ) -> Result<(), IfControlCoverageUseErrorV1> {
        self.coverage.claim(&CoveredSourceSiteV1::body(body))
    }

    pub(crate) fn claim_expression(
        &mut self,
        expression: &LocatedExprV1<'_>,
    ) -> Result<(), IfControlCoverageUseErrorV1> {
        self.coverage
            .claim(&CoveredSourceSiteV1::expression(expression))
    }

    pub(crate) fn finish_coverage(self) -> Result<(), IfControlCoverageUseErrorV1> {
        self.coverage.finish()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct IfControlCoverageClaimV1 {
    pub(super) row: u32,
    pub(super) site: CoveredSourceSiteV1,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedResolvedFunctionIfControlV1 {
    pub(in crate::mir::resolved_control_flow) owner: FunctionOwnerIdV1,
    pub(in crate::mir::resolved_control_flow) rows: Box<[VerifiedLocatedIfControlV1]>,
    pub(in crate::mir::resolved_control_flow) coverage_partition: Box<[IfControlCoverageClaimV1]>,
}

impl VerifiedResolvedFunctionIfControlV1 {
    pub(crate) fn empty_for_loop_profile(
        input: ResolvedFunctionLoweringInputV1<'_>,
    ) -> Result<Self, String> {
        super::super::loop_owned_if::verify_empty_loop_if_partition_v1(input)
    }

    /// Close the outer If-control ledger when every exact If belongs to one
    /// selected Loop source. The Loop logical/physical owner must consume those
    /// rows; this function does not silently discard unrelated If control.
    pub(crate) fn empty_for_owned_loop_profile(
        input: ResolvedFunctionLoweringInputV1<'_>,
        loop_site: &crate::mir::resolved_semantics::SourceNodeSiteV1,
    ) -> Result<Self, String> {
        super::super::loop_owned_if::verify_owned_loop_if_partition_v1(input, loop_site)
    }

    pub(in crate::mir::resolved_control_flow) fn empty_verified(owner: FunctionOwnerIdV1) -> Self {
        Self {
            owner,
            rows: Box::new([]),
            coverage_partition: Box::new([]),
        }
    }

    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(in crate::mir::resolved_control_flow) const fn rows(
        &self,
    ) -> &[VerifiedLocatedIfControlV1] {
        &self.rows
    }

    pub(in crate::mir::resolved_control_flow) fn if_control(
        &self,
        site: &SourceStmtSiteV1,
    ) -> Option<&VerifiedLocatedIfControlV1> {
        self.rows.iter().find(|row| row.site() == site)
    }

    pub(in crate::mir::resolved_control_flow) const fn coverage_partition_len(&self) -> usize {
        self.coverage_partition.len()
    }

    pub(crate) fn exact_if_sites(&self) -> impl Iterator<Item = &SourceStmtSiteV1> {
        self.rows.iter().map(VerifiedLocatedIfControlV1::site)
    }

    pub(crate) const fn row_count(&self) -> usize {
        self.rows.len()
    }

    pub(crate) fn explicit_else_count(&self) -> usize {
        self.rows
            .iter()
            .filter(|row| matches!(row.else_port(), ResolvedIfElsePortV1::Explicit(_)))
            .count()
    }

    /// Exact preorder row consumer for the production canonical lowerer.
    ///
    /// A caller cannot scan the product and later claim a different row set:
    /// every statement If must be consumed once, in the sealed source order.
    pub(crate) fn into_use_ledger(self) -> FunctionIfControlUseLedgerV1 {
        let expected_sites = self
            .rows
            .iter()
            .map(|row| row.site().clone())
            .collect::<Vec<_>>()
            .into_boxed_slice();
        FunctionIfControlUseLedgerV1 {
            owner: self.owner,
            expected_sites,
            rows: self.rows.into_vec().into_iter().map(Some).collect(),
            next: 0,
        }
    }
}
