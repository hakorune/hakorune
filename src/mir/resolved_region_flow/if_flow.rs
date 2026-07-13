//! Owned statement-`If` rows and construction-only function draft.

use crate::mir::resolved_semantics::{
    BindingRefV1, FunctionOwnerIdV1, ResolvedIfRegionBundleV1, SourceStmtSiteV1,
    VerifiedResolvedFunctionV1,
};

use super::coverage::{
    FunctionFlowCoverageDraftV1, IfFlowCoverageDraftV1, VerifiedFunctionFlowCoverageV1,
    VerifiedIfFlowCoverageV1,
};
use super::ports::{
    ResolvedElseFallthroughV1, ResolvedFallthroughPortV1, ResolvedIfConditionEffectsV1,
    ResolvedIfJoinContractV1, ResolvedIfWholeEffectsV1,
};
use super::verifier::{seal_resolved_function_flow_v1, ResolvedRegionFlowVerificationErrorV1};

#[derive(Debug)]
pub(super) struct ResolvedIfFlowDraftV1 {
    pub(super) site: SourceStmtSiteV1,
    pub(super) syntax_has_else: bool,
    pub(super) condition_effects: Vec<BindingRefV1>,
    pub(super) then_effects: Vec<BindingRefV1>,
    pub(super) else_effects: Option<Vec<BindingRefV1>>,
    pub(super) coverage: IfFlowCoverageDraftV1,
}

impl ResolvedIfFlowDraftV1 {
    pub(super) fn new(
        site: SourceStmtSiteV1,
        syntax_has_else: bool,
        condition_effects: Vec<BindingRefV1>,
        then_effects: Vec<BindingRefV1>,
        else_effects: Option<Vec<BindingRefV1>>,
        coverage: IfFlowCoverageDraftV1,
    ) -> Self {
        Self {
            site,
            syntax_has_else,
            condition_effects,
            then_effects,
            else_effects,
            coverage,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VerifiedResolvedIfFlowV1 {
    site: SourceStmtSiteV1,
    regions: ResolvedIfRegionBundleV1,
    condition_effects: ResolvedIfConditionEffectsV1,
    then_port: ResolvedFallthroughPortV1,
    else_port: ResolvedElseFallthroughV1,
    join: ResolvedIfJoinContractV1,
    whole_effects: ResolvedIfWholeEffectsV1,
    coverage: VerifiedIfFlowCoverageV1,
}

impl VerifiedResolvedIfFlowV1 {
    pub(super) fn from_verified(
        site: SourceStmtSiteV1,
        regions: ResolvedIfRegionBundleV1,
        condition_effects: ResolvedIfConditionEffectsV1,
        then_port: ResolvedFallthroughPortV1,
        else_port: ResolvedElseFallthroughV1,
        join: ResolvedIfJoinContractV1,
        whole_effects: ResolvedIfWholeEffectsV1,
        coverage: VerifiedIfFlowCoverageV1,
    ) -> Self {
        Self {
            site,
            regions,
            condition_effects,
            then_port,
            else_port,
            join,
            whole_effects,
            coverage,
        }
    }

    pub(crate) const fn site(&self) -> &SourceStmtSiteV1 {
        &self.site
    }

    pub(crate) const fn regions(&self) -> ResolvedIfRegionBundleV1 {
        self.regions
    }

    pub(crate) const fn condition_effects(&self) -> &ResolvedIfConditionEffectsV1 {
        &self.condition_effects
    }

    pub(crate) const fn then_port(&self) -> &ResolvedFallthroughPortV1 {
        &self.then_port
    }

    pub(crate) const fn else_port(&self) -> &ResolvedElseFallthroughV1 {
        &self.else_port
    }

    pub(crate) const fn join(&self) -> &ResolvedIfJoinContractV1 {
        &self.join
    }

    pub(crate) const fn whole_effects(&self) -> &ResolvedIfWholeEffectsV1 {
        &self.whole_effects
    }

    pub(crate) const fn coverage(&self) -> &VerifiedIfFlowCoverageV1 {
        &self.coverage
    }
}

#[derive(Debug)]
pub(super) struct ResolvedFunctionFlowDraftV1 {
    owner: FunctionOwnerIdV1,
    expected_if_sites: Vec<SourceStmtSiteV1>,
    rows: Vec<Option<VerifiedResolvedIfFlowV1>>,
    coverage: FunctionFlowCoverageDraftV1,
}

impl ResolvedFunctionFlowDraftV1 {
    pub(super) fn new(owner: FunctionOwnerIdV1) -> Self {
        Self {
            owner,
            expected_if_sites: Vec::new(),
            rows: Vec::new(),
            coverage: FunctionFlowCoverageDraftV1::default(),
        }
    }

    /// Reserve publication order before recursively analyzing child flows.
    pub(super) fn reserve_if(&mut self, site: SourceStmtSiteV1) -> usize {
        let slot = self.rows.len();
        self.expected_if_sites.push(site);
        self.rows.push(None);
        slot
    }

    pub(super) fn install_if(
        &mut self,
        slot: usize,
        row: VerifiedResolvedIfFlowV1,
    ) -> Result<(), ResolvedRegionFlowVerificationErrorV1> {
        let expected = self
            .expected_if_sites
            .get(slot)
            .ok_or(ResolvedRegionFlowVerificationErrorV1::InvalidIfRowSlot { slot })?;
        if expected != row.site() {
            return Err(ResolvedRegionFlowVerificationErrorV1::IfRowSiteMismatch {
                expected: expected.clone(),
                actual: row.site().clone(),
            });
        }
        let target = self
            .rows
            .get_mut(slot)
            .ok_or(ResolvedRegionFlowVerificationErrorV1::InvalidIfRowSlot { slot })?;
        if target.is_some() {
            return Err(ResolvedRegionFlowVerificationErrorV1::DuplicateIfRowSlot { slot });
        }
        *target = Some(row);
        Ok(())
    }

    pub(super) fn coverage_mut(&mut self) -> &mut FunctionFlowCoverageDraftV1 {
        &mut self.coverage
    }

    pub(super) fn seal(
        self,
        function: &VerifiedResolvedFunctionV1,
    ) -> Result<VerifiedResolvedFunctionFlowV1, ResolvedRegionFlowVerificationErrorV1> {
        seal_resolved_function_flow_v1(self, function)
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        FunctionOwnerIdV1,
        Vec<SourceStmtSiteV1>,
        Vec<Option<VerifiedResolvedIfFlowV1>>,
        FunctionFlowCoverageDraftV1,
    ) {
        (self.owner, self.expected_if_sites, self.rows, self.coverage)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VerifiedResolvedFunctionFlowV1 {
    owner: FunctionOwnerIdV1,
    if_flows: Box<[VerifiedResolvedIfFlowV1]>,
    coverage: VerifiedFunctionFlowCoverageV1,
}

impl VerifiedResolvedFunctionFlowV1 {
    pub(super) fn from_verified(
        owner: FunctionOwnerIdV1,
        rows: Vec<VerifiedResolvedIfFlowV1>,
        coverage: VerifiedFunctionFlowCoverageV1,
    ) -> Self {
        Self {
            owner,
            if_flows: rows.into_boxed_slice(),
            coverage,
        }
    }

    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) fn if_flows(&self) -> &[VerifiedResolvedIfFlowV1] {
        &self.if_flows
    }

    pub(crate) fn if_flow(&self, site: &SourceStmtSiteV1) -> Option<&VerifiedResolvedIfFlowV1> {
        self.if_flows.iter().find(|flow| flow.site() == site)
    }

    pub(crate) const fn coverage(&self) -> &VerifiedFunctionFlowCoverageV1 {
        &self.coverage
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        FunctionOwnerIdV1,
        Box<[VerifiedResolvedIfFlowV1]>,
        VerifiedFunctionFlowCoverageV1,
    ) {
        (self.owner, self.if_flows, self.coverage)
    }
}
