//! SITE0-R0-LDG0 exact caller-row consumption, disconnected from lowering.

use std::collections::BTreeSet;
use std::marker::PhantomData;

use crate::ast::ASTNode;
use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
use crate::mir::resolved_semantics::{
    SourceBodyKindV1, SourceExprSiteV1, SourceNodeSiteV1, SourcePathV1,
};

use super::activation::VerifiedCallableResultActivationSiteV1;
use super::located_legacy::{
    LegacyActivationBodyDomainPartsV1, LegacyActivationClaimPartsV1, LegacyActivationPrefixPartsV1,
};
use super::loop_claim_schedule::CallableResultLoopClaimSchedulePartsV1;
use super::{
    CallableResultActivationDispositionV1, CallableResultCallerLedgerErrorV1, LegacyBodyInputV1,
    LegacyExprInputV1, LegacyStmtInputV1, LocatedLegacyBodySuffixV1,
    VerifiedCallableResultActivationPlanV1,
};

#[derive(Debug)]
pub(crate) struct VerifiedCallableResultInactivePrefixV1<'plan> {
    caller: &'plan CanonicalSameModuleCallableKeyV1,
    prefix: Option<SourceNodeSiteV1>,
    _plan: PhantomData<&'plan VerifiedCallableResultActivationPlanV1>,
}

#[derive(Debug)]
pub(crate) struct VerifiedCallableResultInactiveBodyV1<'plan> {
    _caller: &'plan CanonicalSameModuleCallableKeyV1,
    _parent: Option<SourceNodeSiteV1>,
    _kind: SourceBodyKindV1,
    _plan: PhantomData<&'plan VerifiedCallableResultActivationPlanV1>,
}

#[derive(Debug)]
pub(crate) struct VerifiedCallableResultInactiveBodySuffixV1<'plan> {
    _caller: &'plan CanonicalSameModuleCallableKeyV1,
    _parent: Option<SourceNodeSiteV1>,
    _domain_parent: Option<SourceNodeSiteV1>,
    _kind: SourceBodyKindV1,
    _start: u32,
    statements: &'plan [ASTNode],
    _plan: PhantomData<&'plan VerifiedCallableResultActivationPlanV1>,
}

impl AsRef<[ASTNode]> for VerifiedCallableResultInactiveBodySuffixV1<'_> {
    fn as_ref(&self) -> &[ASTNode] {
        self.statements
    }
}

#[derive(Debug)]
pub(crate) enum CallableResultBodySuffixDecisionV1<'plan> {
    Inactive(VerifiedCallableResultInactiveBodySuffixV1<'plan>),
    Active { first: &'plan SourceExprSiteV1 },
}

#[derive(Debug)]
pub(crate) struct ClaimedCallableResultActivationSiteV1<'plan> {
    site: &'plan SourceExprSiteV1,
    disposition: &'plan CallableResultActivationDispositionV1,
    _plan: PhantomData<&'plan VerifiedCallableResultActivationPlanV1>,
}

impl ClaimedCallableResultActivationSiteV1<'_> {
    pub(crate) const fn site(&self) -> &SourceExprSiteV1 {
        self.site
    }

    pub(crate) const fn disposition(&self) -> &CallableResultActivationDispositionV1 {
        self.disposition
    }
}

impl<'plan> ClaimedCallableResultActivationSiteV1<'plan> {
    fn from_row(row: &'plan VerifiedCallableResultActivationSiteV1) -> Self {
        Self {
            site: row.site(),
            disposition: row.disposition(),
            _plan: PhantomData,
        }
    }
}

impl VerifiedCallableResultInactivePrefixV1<'_> {
    pub(crate) const fn caller(&self) -> &CanonicalSameModuleCallableKeyV1 {
        self.caller
    }

    pub(crate) const fn prefix(&self) -> Option<&SourceNodeSiteV1> {
        self.prefix.as_ref()
    }
}

#[derive(Debug)]
pub(crate) struct VerifiedCallableResultCallerLedgerV1<'plan> {
    plan_identity: usize,
    caller: &'plan CanonicalSameModuleCallableKeyV1,
    rows: &'plan [VerifiedCallableResultActivationSiteV1],
    claimed: BTreeSet<SourceExprSiteV1>,
}

impl<'plan> VerifiedCallableResultCallerLedgerV1<'plan> {
    pub(crate) fn verify(
        plan: &'plan VerifiedCallableResultActivationPlanV1,
        caller: &CanonicalSameModuleCallableKeyV1,
    ) -> Result<Self, CallableResultCallerLedgerErrorV1> {
        let declaration = plan
            .declaration_catalog()
            .declaration(caller)
            .ok_or_else(|| CallableResultCallerLedgerErrorV1::UnknownCaller(caller.clone()))?;
        let caller = declaration.key();
        let rows = plan
            .rows_for(caller)
            .ok_or_else(|| CallableResultCallerLedgerErrorV1::UnknownCaller(caller.clone()))?;
        Ok(Self {
            plan_identity: plan as *const _ as usize,
            caller,
            rows,
            claimed: BTreeSet::new(),
        })
    }

    pub(crate) fn claim(
        &mut self,
        expression: &LegacyExprInputV1<'plan>,
    ) -> Result<ClaimedCallableResultActivationSiteV1<'plan>, CallableResultCallerLedgerErrorV1>
    {
        let parts = expression
            .activation_claim_parts()
            .map_err(CallableResultCallerLedgerErrorV1::LegacyLocation)?;
        self.require_carrier(parts.plan_identity, parts.caller)?;
        if !matches!(expression.node(), ASTNode::MethodCall { .. }) {
            return Err(CallableResultCallerLedgerErrorV1::ClaimRequiresMethodCall {
                site: parts.site.clone(),
            });
        }
        self.claim_parts(parts)
    }

    pub(crate) fn prove_body_inactive(
        &self,
        body: &LegacyBodyInputV1<'plan>,
    ) -> Result<VerifiedCallableResultInactiveBodyV1<'plan>, CallableResultCallerLedgerErrorV1>
    {
        let parts = body
            .activation_body_domain_parts()
            .map_err(CallableResultCallerLedgerErrorV1::LegacyLocation)?;
        self.prove_body_domain(parts)
    }

    pub(crate) fn classify_body_suffix(
        &self,
        suffix: LocatedLegacyBodySuffixV1<'plan>,
    ) -> Result<CallableResultBodySuffixDecisionV1<'plan>, CallableResultCallerLedgerErrorV1> {
        let parts = suffix.into_activation_parts();
        self.require_carrier(parts.plan_identity, parts.caller)?;
        if let Some(row) = self.rows.iter().find(|row| {
            body_suffix_contains(
                parts.domain_parent.as_ref(),
                parts.kind,
                parts.start,
                row.site().node(),
            )
        }) {
            return Ok(CallableResultBodySuffixDecisionV1::Active { first: row.site() });
        }
        Ok(CallableResultBodySuffixDecisionV1::Inactive(
            VerifiedCallableResultInactiveBodySuffixV1 {
                _caller: self.caller,
                _parent: parts.parent,
                _domain_parent: parts.domain_parent,
                _kind: parts.kind,
                _start: parts.start,
                statements: parts.statements,
                _plan: PhantomData,
            },
        ))
    }

    pub(crate) fn prove_stmt_inactive(
        &self,
        statement: &LegacyStmtInputV1<'plan>,
    ) -> Result<VerifiedCallableResultInactivePrefixV1<'plan>, CallableResultCallerLedgerErrorV1>
    {
        let parts = statement
            .activation_prefix_parts()
            .map_err(CallableResultCallerLedgerErrorV1::LegacyLocation)?;
        self.prove_prefix(parts)
    }

    pub(crate) fn prove_expr_inactive(
        &self,
        expression: &LegacyExprInputV1<'plan>,
    ) -> Result<VerifiedCallableResultInactivePrefixV1<'plan>, CallableResultCallerLedgerErrorV1>
    {
        let parts = expression
            .activation_prefix_parts()
            .map_err(CallableResultCallerLedgerErrorV1::LegacyLocation)?;
        self.prove_prefix(parts)
    }

    pub(crate) fn finish(self) -> Result<(), CallableResultCallerLedgerErrorV1> {
        let Some(row) = self
            .rows
            .iter()
            .find(|row| !self.claimed.contains(row.site()))
        else {
            return Ok(());
        };
        Err(CallableResultCallerLedgerErrorV1::Missing {
            site: row.site().clone(),
            remaining: self.rows.len() - self.claimed.len(),
        })
    }

    fn claim_parts(
        &mut self,
        parts: LegacyActivationClaimPartsV1<'_>,
    ) -> Result<ClaimedCallableResultActivationSiteV1<'plan>, CallableResultCallerLedgerErrorV1>
    {
        let staged = BTreeSet::new();
        let expected = self.prevalidate_claim_site(parts.site, &staged)?;
        let claim = ClaimedCallableResultActivationSiteV1::from_row(expected);
        self.claimed.insert(parts.site.clone());
        Ok(claim)
    }

    pub(super) fn prevalidate_and_commit_loop_schedule(
        &mut self,
        parts: &CallableResultLoopClaimSchedulePartsV1<'plan>,
    ) -> Result<
        Box<[ClaimedCallableResultActivationSiteV1<'plan>]>,
        CallableResultCallerLedgerErrorV1,
    > {
        self.prevalidate_and_commit_rows(parts.activation_plan, parts.caller, &parts.rows)
    }

    fn prevalidate_and_commit_rows(
        &mut self,
        activation_plan: &'plan VerifiedCallableResultActivationPlanV1,
        caller: &'plan CanonicalSameModuleCallableKeyV1,
        rows: &[&'plan VerifiedCallableResultActivationSiteV1],
    ) -> Result<
        Box<[ClaimedCallableResultActivationSiteV1<'plan>]>,
        CallableResultCallerLedgerErrorV1,
    > {
        self.require_carrier(activation_plan as *const _ as usize, caller)?;
        let mut staged = BTreeSet::new();
        let mut expected_rows = Vec::with_capacity(rows.len());
        for requested in rows {
            let expected = self.prevalidate_claim_site(requested.site(), &staged)?;
            if !std::ptr::eq(*requested, expected) {
                return Err(CallableResultCallerLedgerErrorV1::ForeignPlan);
            }
            staged.insert(expected.site().clone());
            expected_rows.push(expected);
        }
        let claims = expected_rows
            .into_iter()
            .map(ClaimedCallableResultActivationSiteV1::from_row)
            .collect::<Vec<_>>()
            .into_boxed_slice();
        self.claimed.extend(staged);
        Ok(claims)
    }

    #[cfg(test)]
    pub(super) fn claim_rows_for_atomicity_test(
        &mut self,
        activation_plan: &'plan VerifiedCallableResultActivationPlanV1,
        caller: &'plan CanonicalSameModuleCallableKeyV1,
        rows: &[&'plan VerifiedCallableResultActivationSiteV1],
    ) -> Result<
        Box<[ClaimedCallableResultActivationSiteV1<'plan>]>,
        CallableResultCallerLedgerErrorV1,
    > {
        self.prevalidate_and_commit_rows(activation_plan, caller, rows)
    }

    fn prevalidate_claim_site(
        &self,
        site: &SourceExprSiteV1,
        staged: &BTreeSet<SourceExprSiteV1>,
    ) -> Result<&'plan VerifiedCallableResultActivationSiteV1, CallableResultCallerLedgerErrorV1>
    {
        if self.claimed.contains(site) || staged.contains(site) {
            return Err(CallableResultCallerLedgerErrorV1::Duplicate { site: site.clone() });
        }
        let Some(expected) = self
            .rows
            .iter()
            .find(|row| !self.claimed.contains(row.site()) && !staged.contains(row.site()))
        else {
            return Err(CallableResultCallerLedgerErrorV1::Unexpected { site: site.clone() });
        };
        if expected.site() == site {
            return Ok(expected);
        }
        if self.rows.iter().any(|row| row.site() == site) {
            return Err(CallableResultCallerLedgerErrorV1::WrongOrder {
                expected: expected.site().clone(),
                actual: site.clone(),
            });
        }
        Err(CallableResultCallerLedgerErrorV1::Unexpected { site: site.clone() })
    }

    fn prove_prefix(
        &self,
        parts: LegacyActivationPrefixPartsV1<'_>,
    ) -> Result<VerifiedCallableResultInactivePrefixV1<'plan>, CallableResultCallerLedgerErrorV1>
    {
        self.require_carrier(parts.plan_identity, parts.caller)?;
        let prefix_segments = parts.prefix.map(SourceNodeSiteV1::segments).unwrap_or(&[]);
        if let Some(row) = self
            .rows
            .iter()
            .find(|row| row.site().node().segments().starts_with(prefix_segments))
        {
            return Err(CallableResultCallerLedgerErrorV1::RowsUnderPrefix {
                prefix: parts.prefix.cloned(),
                first: row.site().clone(),
            });
        }
        Ok(VerifiedCallableResultInactivePrefixV1 {
            caller: self.caller,
            prefix: parts.prefix.cloned(),
            _plan: PhantomData,
        })
    }

    fn prove_body_domain(
        &self,
        parts: LegacyActivationBodyDomainPartsV1<'_>,
    ) -> Result<VerifiedCallableResultInactiveBodyV1<'plan>, CallableResultCallerLedgerErrorV1>
    {
        self.require_carrier(parts.plan_identity, parts.caller)?;
        if let Some(row) = self
            .rows
            .iter()
            .find(|row| body_domain_contains(parts.parent, parts.kind, row.site().node()))
        {
            return Err(CallableResultCallerLedgerErrorV1::RowsUnderPrefix {
                prefix: body_root_diagnostic_site(parts.parent, parts.kind),
                first: row.site().clone(),
            });
        }
        Ok(VerifiedCallableResultInactiveBodyV1 {
            _caller: self.caller,
            _parent: parts.parent.cloned(),
            _kind: parts.kind,
            _plan: PhantomData,
        })
    }

    fn require_carrier(
        &self,
        plan_identity: usize,
        caller: &CanonicalSameModuleCallableKeyV1,
    ) -> Result<(), CallableResultCallerLedgerErrorV1> {
        if plan_identity != self.plan_identity {
            return Err(CallableResultCallerLedgerErrorV1::ForeignPlan);
        }
        if !std::ptr::eq(caller, self.caller) {
            return Err(CallableResultCallerLedgerErrorV1::ForeignCaller {
                expected: self.caller.clone(),
                actual: caller.clone(),
            });
        }
        Ok(())
    }
}

fn body_domain_contains(
    parent: Option<&SourceNodeSiteV1>,
    kind: SourceBodyKindV1,
    site: &SourceNodeSiteV1,
) -> bool {
    let parent_segments = parent.map(SourceNodeSiteV1::segments).unwrap_or(&[]);
    let segments = site.segments();
    let Some(item) = segments
        .strip_prefix(parent_segments)
        .and_then(|tail| tail.first())
    else {
        return false;
    };
    kind.owns_item_segment(item)
}

fn body_suffix_contains(
    domain_parent: Option<&SourceNodeSiteV1>,
    kind: SourceBodyKindV1,
    start: u32,
    site: &SourceNodeSiteV1,
) -> bool {
    let parent_segments = domain_parent.map(SourceNodeSiteV1::segments).unwrap_or(&[]);
    site.segments()
        .strip_prefix(parent_segments)
        .and_then(|tail| tail.first())
        .and_then(|item| kind.owned_item_index(item))
        .is_some_and(|index| index >= start)
}

fn body_root_diagnostic_site(
    parent: Option<&SourceNodeSiteV1>,
    kind: SourceBodyKindV1,
) -> Option<SourceNodeSiteV1> {
    let root = kind.root_segment()?;
    Some(match parent {
        Some(parent) => SourcePathV1::from_node(parent).child(root).node(),
        None => SourcePathV1::function_body().child(root).node(),
    })
}
