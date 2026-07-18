//! Canonical source-order claim schedule for one exact located Loop.
//!
//! This product is read-only and disconnected from the caller ledger. It
//! retains the activation plan, canonical caller, exact Loop statement root,
//! and borrowed activation rows in their existing source order.

use std::collections::BTreeSet;

use crate::ast::ASTNode;
use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
use crate::mir::resolved_semantics::{
    BodyChildRoleV1, ExprChildRoleV1, SourceExprSiteV1, SourceStmtSiteV1,
};

use super::activation::VerifiedCallableResultActivationSiteV1;
use super::{
    CallableResultLegacyLocationErrorV1, LegacyStmtInputV1, VerifiedCallableResultActivationPlanV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CallableResultLoopClaimScheduleErrorV1 {
    LegacyLocation(CallableResultLegacyLocationErrorV1),
    UnknownCaller(CanonicalSameModuleCallableKeyV1),
    ForeignPlan,
    ForeignCaller {
        expected: CanonicalSameModuleCallableKeyV1,
        actual: CanonicalSameModuleCallableKeyV1,
    },
    ExpectedLocatedLoop,
    DuplicateActivationSite(SourceExprSiteV1),
    NoActivationRowsUnderLoop(SourceStmtSiteV1),
}

/// Non-Clone source-order schedule branded by one activation plan and caller.
#[derive(Debug)]
pub(crate) struct VerifiedCallableResultLoopClaimScheduleV1<'plan> {
    activation_plan: &'plan VerifiedCallableResultActivationPlanV1,
    caller: &'plan CanonicalSameModuleCallableKeyV1,
    loop_root: SourceStmtSiteV1,
    rows: Box<[&'plan VerifiedCallableResultActivationSiteV1]>,
}

impl<'plan> VerifiedCallableResultLoopClaimScheduleV1<'plan> {
    pub(crate) fn verify(
        activation_plan: &'plan VerifiedCallableResultActivationPlanV1,
        expected_caller: &CanonicalSameModuleCallableKeyV1,
        loop_statement: LegacyStmtInputV1<'plan>,
    ) -> Result<Self, CallableResultLoopClaimScheduleErrorV1> {
        let parts = loop_statement
            .activation_prefix_parts()
            .map_err(CallableResultLoopClaimScheduleErrorV1::LegacyLocation)?;
        if parts.plan_identity != activation_plan as *const _ as usize {
            return Err(CallableResultLoopClaimScheduleErrorV1::ForeignPlan);
        }
        let declaration = activation_plan
            .declaration_catalog()
            .declaration(expected_caller)
            .ok_or_else(|| {
                CallableResultLoopClaimScheduleErrorV1::UnknownCaller(expected_caller.clone())
            })?;
        let caller = declaration.key();
        if !std::ptr::eq(parts.caller, caller) {
            return Err(CallableResultLoopClaimScheduleErrorV1::ForeignCaller {
                expected: caller.clone(),
                actual: parts.caller.clone(),
            });
        }

        let node = loop_statement.node();
        if !matches!(node, ASTNode::Loop { .. }) {
            return Err(CallableResultLoopClaimScheduleErrorV1::ExpectedLocatedLoop);
        }
        let Some(root) = parts.prefix else {
            return Err(CallableResultLoopClaimScheduleErrorV1::ExpectedLocatedLoop);
        };
        let Some(condition_segment) = ExprChildRoleV1::LoopCondition.segment_for(node) else {
            return Err(CallableResultLoopClaimScheduleErrorV1::ExpectedLocatedLoop);
        };
        let Some(body_kind) = BodyChildRoleV1::LoopBody.kind_for(node) else {
            return Err(CallableResultLoopClaimScheduleErrorV1::ExpectedLocatedLoop);
        };

        let mut seen = BTreeSet::new();
        let mut rows = Vec::new();
        for row in activation_plan
            .rows_for(caller)
            .ok_or_else(|| CallableResultLoopClaimScheduleErrorV1::UnknownCaller(caller.clone()))?
        {
            let Some(first) = row
                .site()
                .node()
                .segments()
                .strip_prefix(root.segments())
                .and_then(|tail| tail.first())
            else {
                continue;
            };
            if first != &condition_segment && !body_kind.owns_item_segment(first) {
                continue;
            }
            if !seen.insert(row.site().clone()) {
                return Err(
                    CallableResultLoopClaimScheduleErrorV1::DuplicateActivationSite(
                        row.site().clone(),
                    ),
                );
            }
            rows.push(row);
        }

        let loop_root = SourceStmtSiteV1::from_node(root.clone());
        if rows.is_empty() {
            return Err(
                CallableResultLoopClaimScheduleErrorV1::NoActivationRowsUnderLoop(loop_root),
            );
        }
        Ok(Self {
            activation_plan,
            caller,
            loop_root,
            rows: rows.into_boxed_slice(),
        })
    }

    pub(crate) const fn caller(&self) -> &CanonicalSameModuleCallableKeyV1 {
        self.caller
    }

    pub(crate) const fn loop_root(&self) -> &SourceStmtSiteV1 {
        &self.loop_root
    }

    pub(crate) fn len(&self) -> usize {
        self.rows.len()
    }

    pub(crate) fn sites_in_source_order(
        &self,
    ) -> impl ExactSizeIterator<Item = &SourceExprSiteV1> + '_ {
        self.rows.iter().map(|row| row.site())
    }

    pub(crate) fn is_branded_by(
        &self,
        activation_plan: &VerifiedCallableResultActivationPlanV1,
    ) -> bool {
        std::ptr::eq(self.activation_plan, activation_plan)
    }
}
