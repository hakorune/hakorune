//! AST-free source handoff for one selected callable Loop.
//!
//! This is the S0 bridge between the resolver-issued callable ledger and the
//! portable Loop route.  It deliberately carries source sites and BindingRefs
//! only.  It never reads/writes a ValueId and it never owns physical lowering.

use std::collections::{BTreeMap, BTreeSet};

use crate::mir::resolved_semantics::{
    BindingRefV1, FunctionOwnerIdV1, SourceBindingSiteV1, SourceNodeSiteV1, SourcePathSegmentV1,
    SourceStmtSiteV1,
};
#[path = "normal_callable_loop_handoff_loop_true.rs"]
mod loop_true_projection;
#[path = "normal_callable_loop_handoff_validation.rs"]
mod validation;
use validation::*;
#[path = "normal_callable_loop_handoff_declarations.rs"]
mod declarations;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum CallableLoopBindingRoleV1 {
    ConditionRead,
    BodyRead,
    BodyRebind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum CallableLoopReadyBindingClassV1 {
    Carrier,
    ReadOnlyOperand,
    IterationLocal,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum CallableLoopBindingProjectionDispositionV1 {
    Ready(VerifiedCallableSemanticLoopBindingScheduleV1),
    ReadyWithBodyOnly(CallableLoopReadyBodyOnlyProductV1),
    Outside(CallableLoopOutsideReasonV1),
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct CallableLoopOutsideReasonV1 {
    owner: FunctionOwnerIdV1,
    loop_site: SourceNodeSiteV1,
    rows: Box<[CallableLoopOutsideRowV1]>,
}

impl CallableLoopOutsideReasonV1 {
    pub(super) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(super) fn loop_site(&self) -> &SourceNodeSiteV1 {
        &self.loop_site
    }

    pub(super) fn rows(&self) -> &[CallableLoopOutsideRowV1] {
        &self.rows
    }

    pub(super) fn into_rows(self) -> Box<[CallableLoopOutsideRowV1]> {
        self.rows
    }

    pub(super) fn into_terminal_error(self) -> String {
        let receipt_count = self
            .rows
            .iter()
            .map(|row| row.receipts().len())
            .sum::<usize>();
        format!(
            "[freeze:contract][callable-loop-handoff/outside-first-cohort] owner={:?} loop_site={:?} rows={} receipts={}",
            self.owner,
            self.loop_site.segments(),
            self.rows.len(),
            receipt_count,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct CallableLoopBindingReceiptV1 {
    site: SourceNodeSiteV1,
    binding: BindingRefV1,
    role: CallableLoopBindingRoleV1,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct CallableLoopReadyBindingRowV1 {
    binding: BindingRefV1,
    class: CallableLoopReadyBindingClassV1,
    receipts: Box<[CallableLoopBindingReceiptV1]>,
}

impl CallableLoopReadyBindingRowV1 {
    pub(super) const fn binding(&self) -> BindingRefV1 {
        self.binding
    }

    pub(super) const fn class(&self) -> CallableLoopReadyBindingClassV1 {
        self.class
    }

    pub(super) fn receipts(&self) -> &[CallableLoopBindingReceiptV1] {
        &self.receipts
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct CallableLoopObservedBindingRowV1 {
    binding: BindingRefV1,
    receipts: Box<[CallableLoopBindingReceiptV1]>,
}

impl CallableLoopObservedBindingRowV1 {
    pub(super) const fn binding(&self) -> BindingRefV1 {
        self.binding
    }

    pub(super) fn receipts(&self) -> &[CallableLoopBindingReceiptV1] {
        &self.receipts
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum CallableLoopOutsideKindV1 {
    BodyOnlyRebind,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct CallableLoopOutsideRowV1 {
    observed: CallableLoopObservedBindingRowV1,
    kind: CallableLoopOutsideKindV1,
}

impl CallableLoopOutsideRowV1 {
    pub(super) const fn binding(&self) -> BindingRefV1 {
        self.observed.binding()
    }

    pub(super) const fn kind(&self) -> CallableLoopOutsideKindV1 {
        self.kind
    }

    pub(super) fn receipts(&self) -> &[CallableLoopBindingReceiptV1] {
        self.observed.receipts()
    }
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
    rows: Box<[CallableLoopReadyBindingRowV1]>,
    local_declarations: BTreeMap<BindingRefV1, SourceBindingSiteV1>,
}

/// One affine source product for the first callable-loop body-only cohort.
///
/// The ready remainder and the grouped body-only rows are issued by the same
/// projection and move together into the existing Facts claim.  No second
/// resolver or semantic authority is introduced for the deferred rows.
#[derive(Debug, PartialEq, Eq)]
pub(super) struct CallableLoopReadyBodyOnlyProductV1 {
    ready: VerifiedCallableSemanticLoopBindingScheduleV1,
    body_only: CallableLoopOutsideReasonV1,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct CallableSemanticLoopHandoffPreEffectReceiptV1 {
    owner: FunctionOwnerIdV1,
    loop_site: SourceNodeSiteV1,
    rows: Box<[CallableLoopReadyBindingRowV1]>,
    body_only_rows: Box<[CallableLoopOutsideRowV1]>,
    local_declarations: BTreeMap<BindingRefV1, SourceBindingSiteV1>,
}

/// Immutable source-only view used by the projector.
///
/// The view deliberately exposes no physical `ValueId` map.  The caller may
/// borrow it from the request-local semantic state, but the projector owns
/// the source-role classification and schedule construction.
pub(super) struct CallableLoopSourceProjectionV1<'a> {
    owner: FunctionOwnerIdV1,
    locals: &'a BTreeMap<SourceNodeSiteV1, Box<[BindingRefV1]>>,
    variables: &'a BTreeMap<SourceNodeSiteV1, BindingRefV1>,
    assignments: &'a BTreeMap<SourceNodeSiteV1, BindingRefV1>,
}

impl<'a> CallableLoopSourceProjectionV1<'a> {
    pub(super) fn new(
        owner: FunctionOwnerIdV1,
        locals: &'a BTreeMap<SourceNodeSiteV1, Box<[BindingRefV1]>>,
        variables: &'a BTreeMap<SourceNodeSiteV1, BindingRefV1>,
        assignments: &'a BTreeMap<SourceNodeSiteV1, BindingRefV1>,
    ) -> Self {
        Self {
            owner,
            locals,
            variables,
            assignments,
        }
    }

    pub(super) fn project(
        self,
        loop_site: SourceNodeSiteV1,
    ) -> Result<VerifiedCallableSemanticLoopBindingScheduleV1, String> {
        match self.project_disposition(loop_site)? {
            CallableLoopBindingProjectionDispositionV1::Ready(schedule) => Ok(schedule),
            CallableLoopBindingProjectionDispositionV1::ReadyWithBodyOnly(_) => {
                Err(freeze("body-only-first-cohort"))
            }
            CallableLoopBindingProjectionDispositionV1::Outside(_) => {
                Err(freeze("outside-first-cohort"))
            }
        }
    }

    pub(super) fn project_disposition(
        self,
        loop_site: SourceNodeSiteV1,
    ) -> Result<CallableLoopBindingProjectionDispositionV1, String> {
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
        let observed_bindings = receipts
            .iter()
            .map(CallableLoopBindingReceiptV1::binding)
            .collect::<BTreeSet<_>>();
        let local_declarations = self.local_declarations(&loop_site)?;
        let iteration_locals = local_declarations
            .keys()
            .filter(|binding| observed_bindings.contains(binding))
            .copied()
            .collect();
        let receipts_by_binding =
            validate_projection_rows(self.owner, &loop_site, &receipts, &iteration_locals)?;
        let outside_bindings = receipts_by_binding
            .iter()
            .filter_map(|(binding, rows)| {
                let has_rebind = rows
                    .iter()
                    .any(|receipt| receipt.role() == CallableLoopBindingRoleV1::BodyRebind);
                let has_condition_read = rows
                    .iter()
                    .any(|receipt| receipt.role() == CallableLoopBindingRoleV1::ConditionRead);
                (has_rebind && !has_condition_read).then_some(*binding)
            })
            .collect::<BTreeSet<_>>();
        if !outside_bindings.is_empty() {
            let ready_receipts = receipts
                .iter()
                .filter(|receipt| !outside_bindings.contains(&receipt.binding()))
                .cloned()
                .collect();
            let ready = validate_ready_remainder(
                self.owner,
                loop_site.clone(),
                ready_receipts,
                local_declarations,
            )?;
            let rows = receipts_by_binding
                .into_iter()
                .filter_map(|(binding, receipts)| {
                    outside_bindings
                        .contains(&binding)
                        .then(|| build_callable_loop_outside_row(binding, receipts))
                })
                .collect::<Vec<_>>()
                .into_boxed_slice();
            return Ok(
                CallableLoopBindingProjectionDispositionV1::ReadyWithBodyOnly(
                    CallableLoopReadyBodyOnlyProductV1 {
                        ready,
                        body_only: CallableLoopOutsideReasonV1 {
                            owner: self.owner,
                            loop_site,
                            rows,
                        },
                    },
                ),
            );
        }
        Ok(CallableLoopBindingProjectionDispositionV1::Ready(
            VerifiedCallableSemanticLoopBindingScheduleV1::seal(
                self.owner,
                loop_site,
                receipts,
                local_declarations,
            )?,
        ))
    }
}

impl CallableSemanticLoopHandoffPreEffectReceiptV1 {
    pub(super) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(super) fn loop_site(&self) -> &SourceNodeSiteV1 {
        &self.loop_site
    }

    pub(super) fn rows(&self) -> &[CallableLoopReadyBindingRowV1] {
        &self.rows
    }

    pub(super) fn body_only_rows(&self) -> &[CallableLoopOutsideRowV1] {
        &self.body_only_rows
    }
}

impl CallableLoopReadyBodyOnlyProductV1 {
    pub(super) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.ready.owner()
    }

    pub(super) fn loop_site(&self) -> &SourceNodeSiteV1 {
        self.ready.loop_site()
    }

    pub(super) fn body_only_rows(&self) -> &[CallableLoopOutsideRowV1] {
        self.body_only.rows()
    }

    pub(super) fn without_body_only(
        schedule: VerifiedCallableSemanticLoopBindingScheduleV1,
    ) -> Self {
        let owner = schedule.owner();
        let loop_site = schedule.loop_site().clone();
        Self {
            ready: schedule,
            body_only: CallableLoopOutsideReasonV1 {
                owner,
                loop_site,
                rows: Box::new([]),
            },
        }
    }

    pub(super) fn consume_pre_effect(
        self,
        parent_site: &SourceNodeSiteV1,
        condition_site: &SourceNodeSiteV1,
        body_site: &SourceNodeSiteV1,
    ) -> Result<CallableSemanticLoopHandoffPreEffectReceiptV1, String> {
        if self.body_only.owner() != self.owner() || self.body_only.loop_site() != self.loop_site()
        {
            return Err(freeze("body-only-owner-site-mismatch"));
        }
        let body_only_rows = self.body_only.into_rows();
        let receipt = self
            .ready
            .consume_pre_effect(parent_site, condition_site, body_site)?;
        Ok(CallableSemanticLoopHandoffPreEffectReceiptV1 {
            body_only_rows,
            ..receipt
        })
    }
}

impl VerifiedCallableSemanticLoopBindingScheduleV1 {
    pub(super) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(super) fn seal(
        owner: FunctionOwnerIdV1,
        loop_site: SourceNodeSiteV1,
        receipts: Vec<CallableLoopBindingReceiptV1>,
        local_declarations: BTreeMap<BindingRefV1, SourceBindingSiteV1>,
    ) -> Result<Self, String> {
        let iteration_locals = declarations::observed_iteration_locals(
            owner,
            &loop_site,
            &receipts,
            &local_declarations,
        )?;
        let rows = build_callable_loop_ready_rows(owner, &loop_site, receipts, iteration_locals)?;
        Ok(Self {
            owner,
            loop_site,
            rows,
            local_declarations,
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
        Ok(CallableSemanticLoopHandoffPreEffectReceiptV1 {
            owner: self.owner,
            loop_site: self.loop_site,
            rows: self.rows,
            body_only_rows: Box::new([]),
            local_declarations: self.local_declarations,
        })
    }

    pub(super) fn loop_site(&self) -> &SourceNodeSiteV1 {
        &self.loop_site
    }

    pub(super) fn receipt_count(&self) -> usize {
        self.rows.iter().map(|row| row.receipts().len()).sum()
    }

    pub(super) fn receipts(&self) -> impl Iterator<Item = &CallableLoopBindingReceiptV1> {
        self.rows.iter().flat_map(|row| row.receipts())
    }

    pub(super) fn rows(&self) -> &[CallableLoopReadyBindingRowV1] {
        &self.rows
    }
}

fn validate_ready_remainder(
    owner: FunctionOwnerIdV1,
    loop_site: SourceNodeSiteV1,
    receipts: Vec<CallableLoopBindingReceiptV1>,
    local_declarations: BTreeMap<BindingRefV1, SourceBindingSiteV1>,
) -> Result<VerifiedCallableSemanticLoopBindingScheduleV1, String> {
    VerifiedCallableSemanticLoopBindingScheduleV1::seal(
        owner,
        loop_site,
        receipts,
        local_declarations,
    )
}

fn freeze(reason: &str) -> String {
    format!("[freeze:contract][callable-loop-handoff/{reason}]")
}

#[cfg(test)]
#[path = "normal_callable_loop_handoff_tests.rs"]
mod tests;
