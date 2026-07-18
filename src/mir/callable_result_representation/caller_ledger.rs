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
use super::{
    CallableResultActivationDispositionV1, CallableResultCallerLedgerErrorV1, LegacyBodyInputV1,
    LegacyExprInputV1, LegacyStmtInputV1, VerifiedCallableResultActivationPlanV1,
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
        if self.claimed.contains(parts.site) {
            return Err(CallableResultCallerLedgerErrorV1::Duplicate {
                site: parts.site.clone(),
            });
        }
        let Some(expected) = self
            .rows
            .iter()
            .find(|row| !self.claimed.contains(row.site()))
        else {
            return Err(CallableResultCallerLedgerErrorV1::Unexpected {
                site: parts.site.clone(),
            });
        };
        if expected.site() == parts.site {
            self.claimed.insert(parts.site.clone());
            return Ok(ClaimedCallableResultActivationSiteV1 {
                site: expected.site(),
                disposition: expected.disposition(),
                _plan: PhantomData,
            });
        }
        if self.rows.iter().any(|row| row.site() == parts.site) {
            return Err(CallableResultCallerLedgerErrorV1::WrongOrder {
                expected: expected.site().clone(),
                actual: parts.site.clone(),
            });
        }
        Err(CallableResultCallerLedgerErrorV1::Unexpected {
            site: parts.site.clone(),
        })
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
