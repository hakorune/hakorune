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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum CallableLoopReadyBindingClassV1 {
    Carrier,
    ReadOnlyOperand,
    IterationLocal,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum CallableLoopBindingProjectionDispositionV1 {
    Ready(VerifiedCallableSemanticLoopBindingScheduleV1),
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
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct CallableSemanticLoopHandoffPreEffectReceiptV1 {
    owner: FunctionOwnerIdV1,
    loop_site: SourceNodeSiteV1,
    rows: Box<[CallableLoopReadyBindingRowV1]>,
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
        let iteration_locals = self
            .locals
            .iter()
            .filter(|(site, _)| is_direct_loop_body_descendant(&loop_site, site))
            .flat_map(|(_, bindings)| bindings.iter().copied())
            .filter(|binding| observed_bindings.contains(binding))
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
            let ready_iteration_locals = iteration_locals
                .iter()
                .filter(|binding| !outside_bindings.contains(binding))
                .copied()
                .collect();
            validate_ready_remainder(
                self.owner,
                loop_site.clone(),
                ready_receipts,
                ready_iteration_locals,
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
            return Ok(CallableLoopBindingProjectionDispositionV1::Outside(
                CallableLoopOutsideReasonV1 {
                    owner: self.owner,
                    loop_site,
                    rows,
                },
            ));
        }
        Ok(CallableLoopBindingProjectionDispositionV1::Ready(
            VerifiedCallableSemanticLoopBindingScheduleV1::seal(
                self.owner,
                loop_site,
                receipts,
                iteration_locals,
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
}

impl VerifiedCallableSemanticLoopBindingScheduleV1 {
    pub(super) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(super) fn seal(
        owner: FunctionOwnerIdV1,
        loop_site: SourceNodeSiteV1,
        receipts: Vec<CallableLoopBindingReceiptV1>,
        iteration_locals: BTreeSet<BindingRefV1>,
    ) -> Result<Self, String> {
        let rows = build_callable_loop_ready_rows(owner, &loop_site, receipts, iteration_locals)?;
        Ok(Self {
            owner,
            loop_site,
            rows,
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
    iteration_locals: BTreeSet<BindingRefV1>,
) -> Result<(), String> {
    let _ = build_callable_loop_ready_rows(owner, &loop_site, receipts, iteration_locals)?;
    Ok(())
}

fn build_callable_loop_ready_rows(
    owner: FunctionOwnerIdV1,
    loop_site: &SourceNodeSiteV1,
    receipts: Vec<CallableLoopBindingReceiptV1>,
    iteration_locals: BTreeSet<BindingRefV1>,
) -> Result<Box<[CallableLoopReadyBindingRowV1]>, String> {
    let receipts_by_binding =
        validate_projection_rows(owner, loop_site, &receipts, &iteration_locals)?;
    let mut carrier_count = 0;
    let mut rows = Vec::with_capacity(receipts_by_binding.len());
    for (binding, receipts) in receipts_by_binding {
        let row = build_callable_loop_ready_row(binding, receipts, &iteration_locals);
        let has_read = row.receipts().iter().any(|receipt| {
            matches!(
                receipt.role(),
                CallableLoopBindingRoleV1::ConditionRead | CallableLoopBindingRoleV1::BodyRead
            )
        });
        let has_rebind = row
            .receipts()
            .iter()
            .any(|receipt| receipt.role() == CallableLoopBindingRoleV1::BodyRebind);
        let has_condition_read = row
            .receipts()
            .iter()
            .any(|receipt| receipt.role() == CallableLoopBindingRoleV1::ConditionRead);
        let has_body_read = row
            .receipts()
            .iter()
            .any(|receipt| receipt.role() == CallableLoopBindingRoleV1::BodyRead);
        if row.class() == CallableLoopReadyBindingClassV1::Carrier {
            carrier_count += 1;
        }
        if !has_read
            || (row.class() == CallableLoopReadyBindingClassV1::Carrier
                && (!has_condition_read || !has_body_read || !has_rebind))
        {
            return Err(freeze("incomplete-binding-coverage"));
        }
        rows.push(row);
    }
    if carrier_count != 1 {
        return Err(freeze("carrier-cardinality"));
    }
    for binding in iteration_locals {
        if !rows.iter().any(|row| row.binding() == binding) {
            return Err(freeze("unconsumed-iteration-local"));
        }
    }
    Ok(rows.into_boxed_slice())
}

fn build_callable_loop_ready_row(
    binding: BindingRefV1,
    receipts: Vec<CallableLoopBindingReceiptV1>,
    iteration_locals: &BTreeSet<BindingRefV1>,
) -> CallableLoopReadyBindingRowV1 {
    CallableLoopReadyBindingRowV1 {
        binding,
        class: classify_ready_binding(binding, &receipts, iteration_locals),
        receipts: receipts.into_boxed_slice(),
    }
}

fn classify_ready_binding(
    binding: BindingRefV1,
    receipts: &[CallableLoopBindingReceiptV1],
    iteration_locals: &BTreeSet<BindingRefV1>,
) -> CallableLoopReadyBindingClassV1 {
    let has_rebind = receipts
        .iter()
        .any(|receipt| receipt.role() == CallableLoopBindingRoleV1::BodyRebind);
    if iteration_locals.contains(&binding) {
        CallableLoopReadyBindingClassV1::IterationLocal
    } else if has_rebind {
        CallableLoopReadyBindingClassV1::Carrier
    } else {
        CallableLoopReadyBindingClassV1::ReadOnlyOperand
    }
}

fn build_callable_loop_outside_row(
    binding: BindingRefV1,
    receipts: Vec<CallableLoopBindingReceiptV1>,
) -> CallableLoopOutsideRowV1 {
    CallableLoopOutsideRowV1 {
        observed: CallableLoopObservedBindingRowV1 {
            binding,
            receipts: receipts.into_boxed_slice(),
        },
        kind: CallableLoopOutsideKindV1::BodyOnlyRebind,
    }
}

fn validate_projection_rows(
    owner: FunctionOwnerIdV1,
    loop_site: &SourceNodeSiteV1,
    receipts: &[CallableLoopBindingReceiptV1],
    iteration_locals: &BTreeSet<BindingRefV1>,
) -> Result<BTreeMap<BindingRefV1, Vec<CallableLoopBindingReceiptV1>>, String> {
    if loop_site.segments().is_empty() {
        return Err(freeze("empty-loop-site"));
    }
    if receipts.is_empty() {
        return Err(freeze("incomplete-coverage"));
    }
    if iteration_locals
        .iter()
        .any(|binding| binding.owner() != owner)
    {
        return Err(freeze("foreign-iteration-local"));
    }
    let mut sites = BTreeSet::new();
    let mut receipts_by_binding = BTreeMap::<_, Vec<_>>::new();
    for receipt in receipts {
        if receipt.binding().owner() != owner {
            return Err(freeze("foreign-binding"));
        }
        classify_suffix(loop_site, receipt.site(), receipt.role())?;
        if !sites.insert(receipt.site().clone()) {
            return Err(freeze("duplicate-source-site"));
        }
        receipts_by_binding
            .entry(receipt.binding())
            .or_default()
            .push(receipt.clone());
    }
    for rows in receipts_by_binding.values() {
        let has_read = rows.iter().any(|receipt| {
            matches!(
                receipt.role(),
                CallableLoopBindingRoleV1::ConditionRead | CallableLoopBindingRoleV1::BodyRead
            )
        });
        if !has_read {
            return Err(freeze("incomplete-binding-coverage"));
        }
    }
    for binding in iteration_locals {
        if !receipts_by_binding.contains_key(binding) {
            return Err(freeze("unconsumed-iteration-local"));
        }
    }
    Ok(receipts_by_binding)
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
            SourcePathSegmentV1::LoopCondition
                | SourcePathSegmentV1::LoopBodyRoot
                | SourcePathSegmentV1::LoopBody(_)
                | SourcePathSegmentV1::LambdaBodyRoot
                | SourcePathSegmentV1::LambdaBody(_)
        )
    });
    if is_nested {
        return Err(freeze("nested-loop-profile-not-admitted"));
    }
    let valid = match (role, relative.first()) {
        (CallableLoopBindingRoleV1::ConditionRead, Some(SourcePathSegmentV1::LoopCondition)) => {
            true
        }
        (
            CallableLoopBindingRoleV1::BodyRead | CallableLoopBindingRoleV1::BodyRebind,
            Some(SourcePathSegmentV1::LoopBody(_)),
        ) => true,
        _ => false,
    };
    valid
        .then_some(())
        .ok_or_else(|| freeze("role-source-mismatch"))
}

fn is_direct_loop_body_descendant(loop_site: &SourceNodeSiteV1, site: &SourceNodeSiteV1) -> bool {
    let Some(relative) = relative_segments(loop_site, site) else {
        return false;
    };
    matches!(relative.first(), Some(SourcePathSegmentV1::LoopBody(_)))
        && !relative.iter().skip(1).any(|segment| {
            matches!(
                segment,
                SourcePathSegmentV1::LoopCondition
                    | SourcePathSegmentV1::LoopBodyRoot
                    | SourcePathSegmentV1::LoopBody(_)
                    | SourcePathSegmentV1::LambdaBodyRoot
                    | SourcePathSegmentV1::LambdaBody(_)
            )
        })
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
#[path = "normal_callable_loop_handoff_tests.rs"]
mod tests;
