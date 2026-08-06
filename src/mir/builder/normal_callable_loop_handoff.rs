//! AST-free source handoff for one selected callable Loop.
//!
//! This is the S0 bridge between the resolver-issued callable ledger and the
//! portable Loop route.  It deliberately carries source sites and BindingRefs
//! only.  It never reads/writes a ValueId and it never owns physical lowering.

use std::collections::{BTreeMap, BTreeSet};

use crate::mir::resolved_semantics::{
    BindingRefV1, FunctionOwnerIdV1, SourceNodeSiteV1, SourcePathSegmentV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum CallableLoopBindingRoleV1 {
    ConditionRead,
    BodyRead,
    BodyRebind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct CallableLoopBindingReceiptV1 {
    site: SourceNodeSiteV1,
    binding: BindingRefV1,
    role: CallableLoopBindingRoleV1,
}

impl CallableLoopBindingReceiptV1 {
    pub(super) fn new(
        site: SourceNodeSiteV1,
        binding: BindingRefV1,
        role: CallableLoopBindingRoleV1,
    ) -> Self {
        Self {
            site,
            binding,
            role,
        }
    }

    pub(super) fn site(&self) -> &SourceNodeSiteV1 {
        &self.site
    }

    pub(super) const fn binding(&self) -> BindingRefV1 {
        self.binding
    }

    pub(super) const fn role(&self) -> CallableLoopBindingRoleV1 {
        self.role
    }
}

/// One move-only source contract for a callable Loop.
///
/// The product is intentionally not `Clone`: a route gets one schedule and
/// must consume it at its pre-effect boundary.  The physicalizer will later
/// receive a separate materializer capability; this product is not that
/// capability and does not publish physical values.
#[derive(Debug, PartialEq, Eq)]
pub(super) struct VerifiedCallableSemanticLoopBindingScheduleV1 {
    owner: FunctionOwnerIdV1,
    loop_site: SourceNodeSiteV1,
    receipts: Box<[CallableLoopBindingReceiptV1]>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct CallableSemanticLoopHandoffPreEffectReceiptV1 {
    owner: FunctionOwnerIdV1,
    variable_reads: u32,
    assignments: u32,
}

/// Immutable source-only view used by the projector.
///
/// The view deliberately exposes no physical `ValueId` map.  The caller may
/// borrow it from the request-local semantic state, but the projector owns
/// the source-role classification and schedule construction.
pub(super) struct CallableLoopSourceProjectionV1<'a> {
    owner: FunctionOwnerIdV1,
    variables: &'a BTreeMap<SourceNodeSiteV1, BindingRefV1>,
    assignments: &'a BTreeMap<SourceNodeSiteV1, BindingRefV1>,
}

impl<'a> CallableLoopSourceProjectionV1<'a> {
    pub(super) fn new(
        owner: FunctionOwnerIdV1,
        variables: &'a BTreeMap<SourceNodeSiteV1, BindingRefV1>,
        assignments: &'a BTreeMap<SourceNodeSiteV1, BindingRefV1>,
    ) -> Self {
        Self {
            owner,
            variables,
            assignments,
        }
    }

    pub(super) fn project(
        self,
        loop_site: SourceNodeSiteV1,
    ) -> Result<VerifiedCallableSemanticLoopBindingScheduleV1, String> {
        let mut receipts = Vec::new();
        for (site, binding) in self.variables {
            let Some(relative) = relative_segments(&loop_site, site) else {
                continue;
            };
            let Some(first) = relative.first() else {
                continue;
            };
            let role = match first {
                SourcePathSegmentV1::LoopCondition => CallableLoopBindingRoleV1::ConditionRead,
                SourcePathSegmentV1::LoopBodyRoot | SourcePathSegmentV1::LoopBody(_) => {
                    CallableLoopBindingRoleV1::BodyRead
                }
                _ => continue,
            };
            receipts.push(CallableLoopBindingReceiptV1::new(
                site.clone(),
                *binding,
                role,
            ));
        }
        for (site, binding) in self.assignments {
            let Some(relative) = relative_segments(&loop_site, site) else {
                continue;
            };
            let Some(first) = relative.first() else {
                continue;
            };
            if !matches!(
                first,
                SourcePathSegmentV1::LoopBodyRoot | SourcePathSegmentV1::LoopBody(_)
            ) {
                continue;
            }
            receipts.push(CallableLoopBindingReceiptV1::new(
                site.clone(),
                *binding,
                CallableLoopBindingRoleV1::BodyRebind,
            ));
        }
        VerifiedCallableSemanticLoopBindingScheduleV1::seal(self.owner, loop_site, receipts)
    }
}

impl CallableSemanticLoopHandoffPreEffectReceiptV1 {
    pub(super) const fn owner(self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(super) const fn variable_reads(self) -> u32 {
        self.variable_reads
    }

    pub(super) const fn assignments(self) -> u32 {
        self.assignments
    }
}

impl VerifiedCallableSemanticLoopBindingScheduleV1 {
    pub(super) fn seal(
        owner: FunctionOwnerIdV1,
        loop_site: SourceNodeSiteV1,
        receipts: Vec<CallableLoopBindingReceiptV1>,
    ) -> Result<Self, String> {
        if loop_site.segments().is_empty() {
            return Err(freeze("empty-loop-site"));
        }
        if receipts.is_empty() {
            return Err(freeze("incomplete-coverage"));
        }
        let mut sites = BTreeSet::new();
        let mut condition_reads = 0;
        let mut body_reads = 0;
        let mut body_rebinds = 0;
        for receipt in &receipts {
            if receipt.binding().owner() != owner {
                return Err(freeze("foreign-binding"));
            }
            classify_suffix(&loop_site, receipt.site(), receipt.role())?;
            match receipt.role() {
                CallableLoopBindingRoleV1::ConditionRead => condition_reads += 1,
                CallableLoopBindingRoleV1::BodyRead => body_reads += 1,
                CallableLoopBindingRoleV1::BodyRebind => body_rebinds += 1,
            }
            if !sites.insert(receipt.site().clone()) {
                return Err(freeze("duplicate-source-site"));
            }
        }
        if (condition_reads, body_reads, body_rebinds) != (1, 1, 1) {
            return Err(freeze("incomplete-coverage"));
        }
        Ok(Self {
            owner,
            loop_site,
            receipts: receipts.into_boxed_slice(),
        })
    }

    pub(super) fn consume_pre_effect(
        self,
        parent_site: &SourceNodeSiteV1,
        condition_site: &SourceNodeSiteV1,
        body_site: &SourceNodeSiteV1,
    ) -> Result<CallableSemanticLoopHandoffPreEffectReceiptV1, String> {
        if parent_site != &self.loop_site {
            return Err(freeze("loop-owner-site-mismatch"));
        }
        if !is_direct_child(
            parent_site,
            condition_site,
            SourcePathSegmentV1::LoopCondition,
        ) {
            return Err(freeze("condition-source-mismatch"));
        }
        if !is_direct_child(parent_site, body_site, SourcePathSegmentV1::LoopBodyRoot) {
            return Err(freeze("body-source-mismatch"));
        }
        let mut variable_reads = 0;
        let mut assignments = 0;
        for receipt in &self.receipts {
            match receipt.role() {
                CallableLoopBindingRoleV1::ConditionRead | CallableLoopBindingRoleV1::BodyRead => {
                    variable_reads += 1
                }
                CallableLoopBindingRoleV1::BodyRebind => assignments += 1,
            }
        }
        Ok(CallableSemanticLoopHandoffPreEffectReceiptV1 {
            owner: self.owner,
            variable_reads,
            assignments,
        })
    }

    pub(super) fn loop_site(&self) -> &SourceNodeSiteV1 {
        &self.loop_site
    }

    pub(super) fn receipts(&self) -> &[CallableLoopBindingReceiptV1] {
        &self.receipts
    }
}

fn classify_suffix(
    loop_site: &SourceNodeSiteV1,
    site: &SourceNodeSiteV1,
    role: CallableLoopBindingRoleV1,
) -> Result<(), String> {
    let Some(relative) = relative_segments(loop_site, site) else {
        return Err(freeze("source-site-outside-loop"));
    };
    if relative.is_empty() {
        return Err(freeze("source-site-is-loop-root"));
    }
    let is_nested = relative.iter().skip(1).any(|segment| {
        matches!(
            segment,
            SourcePathSegmentV1::LoopCondition | SourcePathSegmentV1::LoopBodyRoot
        )
    });
    if is_nested {
        return Err(freeze("nested-loop-profile-not-admitted"));
    }
    let valid = match role {
        CallableLoopBindingRoleV1::ConditionRead => {
            relative == [SourcePathSegmentV1::LoopCondition, SourcePathSegmentV1::Lhs]
        }
        CallableLoopBindingRoleV1::BodyRead => {
            relative
                == [
                    SourcePathSegmentV1::LoopBody(0),
                    SourcePathSegmentV1::Value,
                    SourcePathSegmentV1::Lhs,
                ]
        }
        CallableLoopBindingRoleV1::BodyRebind => {
            relative
                == [
                    SourcePathSegmentV1::LoopBody(0),
                    SourcePathSegmentV1::Target,
                ]
        }
    };
    valid
        .then_some(())
        .ok_or_else(|| freeze("role-source-mismatch"))
}

fn relative_segments<'a>(
    root: &SourceNodeSiteV1,
    site: &'a SourceNodeSiteV1,
) -> Option<&'a [SourcePathSegmentV1]> {
    let root_segments = root.segments();
    site.segments().starts_with(root_segments).then(|| {
        let split = root_segments.len();
        &site.segments()[split..]
    })
}

fn is_direct_child(
    root: &SourceNodeSiteV1,
    child: &SourceNodeSiteV1,
    expected: SourcePathSegmentV1,
) -> bool {
    relative_segments(root, child).is_some_and(|relative| relative == [expected])
}

fn freeze(reason: &str) -> String {
    format!("[freeze:contract][callable-loop-handoff/{reason}]")
}

#[cfg(test)]
mod tests {
    use hakorune_mir_core::BindingId;

    use super::*;
    use crate::mir::resolved_semantics::{FunctionOwnerIssuerV1, SourcePathV1};

    fn owner() -> FunctionOwnerIdV1 {
        let mut issuer = FunctionOwnerIssuerV1::new_for_compilation().unwrap();
        issuer.issue().unwrap()
    }

    fn binding(owner: FunctionOwnerIdV1, slot: u32) -> BindingRefV1 {
        BindingRefV1::new(owner, BindingId::new(slot))
    }

    #[test]
    fn seals_exact_condition_body_and_assignment_roles() {
        let owner = owner();
        let loop_site = SourcePathV1::root_body(2).node();
        let condition = SourcePathV1::from_node(&loop_site)
            .child(SourcePathSegmentV1::LoopCondition)
            .child(SourcePathSegmentV1::Lhs)
            .node();
        let body_read = SourcePathV1::from_node(&loop_site)
            .child(SourcePathSegmentV1::LoopBody(0))
            .child(SourcePathSegmentV1::Value)
            .child(SourcePathSegmentV1::Lhs)
            .node();
        let target = SourcePathV1::from_node(&loop_site)
            .child(SourcePathSegmentV1::LoopBody(0))
            .child(SourcePathSegmentV1::Target)
            .node();
        let schedule = VerifiedCallableSemanticLoopBindingScheduleV1::seal(
            owner,
            loop_site.clone(),
            vec![
                CallableLoopBindingReceiptV1::new(
                    condition.clone(),
                    binding(owner, 0),
                    CallableLoopBindingRoleV1::ConditionRead,
                ),
                CallableLoopBindingReceiptV1::new(
                    body_read.clone(),
                    binding(owner, 0),
                    CallableLoopBindingRoleV1::BodyRead,
                ),
                CallableLoopBindingReceiptV1::new(
                    target.clone(),
                    binding(owner, 0),
                    CallableLoopBindingRoleV1::BodyRebind,
                ),
            ],
        )
        .unwrap();
        let receipt = schedule
            .consume_pre_effect(
                &loop_site,
                &SourcePathV1::from_node(&loop_site)
                    .child(SourcePathSegmentV1::LoopCondition)
                    .node(),
                &SourcePathV1::from_node(&loop_site)
                    .child(SourcePathSegmentV1::LoopBodyRoot)
                    .node(),
            )
            .unwrap();
        assert_eq!(receipt.owner(), owner);
        assert_eq!(receipt.variable_reads(), 2);
        assert_eq!(receipt.assignments(), 1);
    }

    #[test]
    fn rejects_foreign_duplicate_and_nested_receipts() {
        let owner_id = owner();
        let foreign = owner();
        let loop_site = SourcePathV1::root_body(2).node();
        let condition = SourcePathV1::from_node(&loop_site)
            .child(SourcePathSegmentV1::LoopCondition)
            .child(SourcePathSegmentV1::Lhs)
            .node();
        let receipt = || {
            CallableLoopBindingReceiptV1::new(
                condition.clone(),
                binding(owner_id, 0),
                CallableLoopBindingRoleV1::ConditionRead,
            )
        };
        assert!(VerifiedCallableSemanticLoopBindingScheduleV1::seal(
            owner_id,
            loop_site.clone(),
            vec![CallableLoopBindingReceiptV1::new(
                condition.clone(),
                binding(foreign, 0),
                CallableLoopBindingRoleV1::ConditionRead,
            )],
        )
        .is_err());
        assert!(VerifiedCallableSemanticLoopBindingScheduleV1::seal(
            owner_id,
            loop_site.clone(),
            vec![receipt(), receipt()],
        )
        .is_err());
        assert!(VerifiedCallableSemanticLoopBindingScheduleV1::seal(
            owner_id,
            loop_site.clone(),
            Vec::new(),
        )
        .is_err());
        assert!(VerifiedCallableSemanticLoopBindingScheduleV1::seal(
            owner_id,
            loop_site.clone(),
            vec![receipt()],
        )
        .is_err());
        let nested = SourcePathV1::from_node(&loop_site)
            .child(SourcePathSegmentV1::LoopBody(0))
            .child(SourcePathSegmentV1::LoopCondition)
            .child(SourcePathSegmentV1::Lhs)
            .node();
        assert!(VerifiedCallableSemanticLoopBindingScheduleV1::seal(
            owner_id,
            loop_site,
            vec![CallableLoopBindingReceiptV1::new(
                nested,
                binding(owner_id, 0),
                CallableLoopBindingRoleV1::BodyRead,
            )],
        )
        .is_err());
    }
}
