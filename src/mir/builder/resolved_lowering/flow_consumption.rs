//! One-shot source-preorder consumption of the verified statement-`If` flow.
//!
//! This box owns the lowering copy of every row. It also keeps ordered direct
//! assignment coverage in a lexical frame stack, so nested rows cannot claim
//! their parent's sites.

use std::collections::{BTreeSet, VecDeque};

use crate::mir::resolved_region_flow::{
    ResolvedElseFallthroughV1, VerifiedResolvedFunctionFlowV1, VerifiedResolvedIfFlowV1,
};
use crate::mir::resolved_semantics::{
    BindingRefV1, FunctionOwnerIdV1, SourceExprSiteV1, SourceStmtSiteV1,
};

use super::branch_transaction::ResolvedEffectBindingClassV1;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum CoverageOwnerV1 {
    Function,
    Condition(SourceStmtSiteV1),
    Then(SourceStmtSiteV1),
    Else(SourceStmtSiteV1),
}

#[derive(Debug)]
struct CoverageFrameV1 {
    owner: CoverageOwnerV1,
    expected: Box<[SourceExprSiteV1]>,
    next: usize,
    visible_effects: Option<BTreeSet<BindingRefV1>>,
}

impl CoverageFrameV1 {
    fn new(
        owner: CoverageOwnerV1,
        expected: &[SourceExprSiteV1],
        visible_effects: Option<&[BindingRefV1]>,
    ) -> Self {
        Self {
            owner,
            expected: expected.to_vec().into_boxed_slice(),
            next: 0,
            visible_effects: visible_effects.map(|items| items.iter().copied().collect()),
        }
    }
}

#[derive(Debug)]
pub(super) struct ResolvedFlowConsumptionV1 {
    owner: FunctionOwnerIdV1,
    rows: VecDeque<VerifiedResolvedIfFlowV1>,
    expected_if_controls: usize,
    expected_if_branches: usize,
    claimed_rows: BTreeSet<SourceStmtSiteV1>,
    expected_coverage: BTreeSet<CoverageOwnerV1>,
    opened_coverage: BTreeSet<CoverageOwnerV1>,
    coverage: Vec<CoverageFrameV1>,
}

impl ResolvedFlowConsumptionV1 {
    pub(super) fn new(flow: VerifiedResolvedFunctionFlowV1) -> Self {
        let (owner, rows, function_coverage) = flow.into_parts();
        let expected_if_controls = rows.len();
        let expected_if_branches = rows
            .iter()
            .map(|row| 1 + usize::from(row.else_port().explicit_port().is_some()))
            .sum();
        let mut expected_coverage = BTreeSet::new();
        for row in &rows {
            expected_coverage.insert(CoverageOwnerV1::Condition(row.site().clone()));
            expected_coverage.insert(CoverageOwnerV1::Then(row.site().clone()));
            if row.else_port().explicit_port().is_some() {
                expected_coverage.insert(CoverageOwnerV1::Else(row.site().clone()));
            }
        }
        Self {
            owner,
            rows: rows.into_vec().into(),
            expected_if_controls,
            expected_if_branches,
            claimed_rows: BTreeSet::new(),
            expected_coverage,
            opened_coverage: BTreeSet::new(),
            coverage: vec![CoverageFrameV1::new(
                CoverageOwnerV1::Function,
                function_coverage.function_direct(),
                None,
            )],
        }
    }

    pub(super) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(super) const fn expected_if_control_regions(&self) -> usize {
        self.expected_if_controls
    }

    pub(super) const fn expected_if_branch_pairs(&self) -> usize {
        self.expected_if_branches
    }

    pub(super) fn claim_next_if(
        &mut self,
        site: &SourceStmtSiteV1,
    ) -> Result<VerifiedResolvedIfFlowV1, String> {
        let row = self.rows.front().ok_or_else(|| {
            "[freeze:contract][canonical_flow/unexpected_if_after_rows]".to_string()
        })?;
        if row.site() != site {
            return Err(format!(
                "[freeze:contract][canonical_flow/source_preorder_mismatch] expected={:?} actual={site:?}",
                row.site()
            ));
        }
        if !self.claimed_rows.insert(site.clone()) {
            return Err("[freeze:contract][canonical_flow/if_row_reclaimed]".to_string());
        }
        Ok(self.rows.pop_front().expect("front row exists"))
    }

    pub(super) fn begin_condition(&mut self, row: &VerifiedResolvedIfFlowV1) -> Result<(), String> {
        self.require_claimed(row.site())?;
        self.push_coverage(
            CoverageOwnerV1::Condition(row.site().clone()),
            row.coverage().condition_direct(),
            Some(row.condition_effects().may_rebind_outer()),
        )
    }

    pub(super) fn finish_condition(&mut self, site: &SourceStmtSiteV1) -> Result<(), String> {
        self.pop_coverage(CoverageOwnerV1::Condition(site.clone()))
    }

    pub(super) fn abort_condition(&mut self, site: &SourceStmtSiteV1) -> Result<(), String> {
        self.abort_coverage(CoverageOwnerV1::Condition(site.clone()))
    }

    pub(super) fn begin_then(&mut self, row: &VerifiedResolvedIfFlowV1) -> Result<(), String> {
        self.require_claimed(row.site())?;
        self.push_coverage(
            CoverageOwnerV1::Then(row.site().clone()),
            row.coverage().then_direct(),
            Some(row.then_port().may_rebind_outer()),
        )
    }

    pub(super) fn finish_then(&mut self, site: &SourceStmtSiteV1) -> Result<(), String> {
        self.pop_coverage(CoverageOwnerV1::Then(site.clone()))
    }

    pub(super) fn abort_then(&mut self, site: &SourceStmtSiteV1) -> Result<(), String> {
        self.abort_coverage(CoverageOwnerV1::Then(site.clone()))
    }

    pub(super) fn begin_else(&mut self, row: &VerifiedResolvedIfFlowV1) -> Result<(), String> {
        self.require_claimed(row.site())?;
        let ResolvedElseFallthroughV1::Explicit(port) = row.else_port() else {
            return Err("[freeze:contract][canonical_flow/implicit_else_coverage]".to_string());
        };
        self.push_coverage(
            CoverageOwnerV1::Else(row.site().clone()),
            row.coverage().else_direct(),
            Some(port.may_rebind_outer()),
        )
    }

    pub(super) fn finish_else(&mut self, site: &SourceStmtSiteV1) -> Result<(), String> {
        self.pop_coverage(CoverageOwnerV1::Else(site.clone()))
    }

    pub(super) fn abort_else(&mut self, site: &SourceStmtSiteV1) -> Result<(), String> {
        self.abort_coverage(CoverageOwnerV1::Else(site.clone()))
    }

    pub(super) fn claim_assignment(
        &mut self,
        site: &SourceExprSiteV1,
        binding: BindingRefV1,
        class: ResolvedEffectBindingClassV1,
    ) -> Result<(), String> {
        let frame = self.coverage.last_mut().ok_or_else(|| {
            "[freeze:contract][canonical_flow/assignment_without_coverage]".to_string()
        })?;
        let expected = frame.expected.get(frame.next).ok_or_else(|| {
            format!(
                "[freeze:contract][canonical_flow/unexpected_assignment] owner={:?} site={site:?}",
                frame.owner
            )
        })?;
        if expected != site {
            return Err(format!(
                "[freeze:contract][canonical_flow/assignment_order_mismatch] owner={:?} expected={expected:?} actual={site:?}",
                frame.owner
            ));
        }
        if class == ResolvedEffectBindingClassV1::Visible
            && frame
                .visible_effects
                .as_ref()
                .is_some_and(|effects| !effects.contains(&binding))
        {
            return Err(format!(
                "[freeze:contract][canonical_flow/assignment_effect_mismatch] owner={:?} binding={binding:?}",
                frame.owner
            ));
        }
        frame.next += 1;
        Ok(())
    }

    pub(super) fn finish(&self) -> Result<(), String> {
        if !self.rows.is_empty()
            || self.coverage.len() != 1
            || self.opened_coverage != self.expected_coverage
        {
            return Err(format!(
                "[freeze:contract][canonical_flow/finish_structure_mismatch] rows={} coverage_depth={} opened={}/{}",
                self.rows.len(),
                self.coverage.len(),
                self.opened_coverage.len(),
                self.expected_coverage.len(),
            ));
        }
        let function = &self.coverage[0];
        if function.next != function.expected.len() {
            return Err(format!(
                "[freeze:contract][canonical_flow/function_coverage_incomplete] consumed={}/{}",
                function.next,
                function.expected.len()
            ));
        }
        Ok(())
    }

    #[cfg(test)]
    pub(super) fn coverage_depth(&self) -> usize {
        self.coverage.len()
    }

    fn require_claimed(&self, site: &SourceStmtSiteV1) -> Result<(), String> {
        if self.claimed_rows.contains(site) {
            Ok(())
        } else {
            Err("[freeze:contract][canonical_flow/coverage_before_row_claim]".to_string())
        }
    }

    fn push_coverage(
        &mut self,
        owner: CoverageOwnerV1,
        expected: &[SourceExprSiteV1],
        effects: Option<&[BindingRefV1]>,
    ) -> Result<(), String> {
        if !self.opened_coverage.insert(owner.clone()) {
            return Err("[freeze:contract][canonical_flow/coverage_reopened]".to_string());
        }
        self.coverage
            .push(CoverageFrameV1::new(owner, expected, effects));
        Ok(())
    }

    fn pop_coverage(&mut self, expected_owner: CoverageOwnerV1) -> Result<(), String> {
        let frame = self.coverage.last().ok_or_else(|| {
            "[freeze:contract][canonical_flow/coverage_pop_without_frame]".to_string()
        })?;
        if frame.owner != expected_owner {
            return Err("[freeze:contract][canonical_flow/coverage_pop_mismatch]".to_string());
        }
        if frame.next != frame.expected.len() {
            return Err(format!(
                "[freeze:contract][canonical_flow/coverage_incomplete] owner={:?} consumed={}/{}",
                frame.owner,
                frame.next,
                frame.expected.len()
            ));
        }
        self.coverage.pop();
        Ok(())
    }

    fn abort_coverage(&mut self, expected_owner: CoverageOwnerV1) -> Result<(), String> {
        let frame = self.coverage.last().ok_or_else(|| {
            "[freeze:contract][canonical_flow/coverage_abort_without_frame]".to_string()
        })?;
        if frame.owner != expected_owner {
            return Err("[freeze:contract][canonical_flow/coverage_abort_mismatch]".to_string());
        }
        self.coverage.pop();
        Ok(())
    }
}
