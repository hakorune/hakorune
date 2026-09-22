//! Source-only binding row and lexical path validation.

use super::*;

pub(super) fn build_callable_loop_ready_rows(
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

pub(super) fn build_callable_loop_ready_row(
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

pub(super) fn build_callable_loop_outside_row(
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

pub(super) fn validate_projection_rows(
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

pub(super) fn is_direct_loop_body_descendant(
    loop_site: &SourceNodeSiteV1,
    site: &SourceNodeSiteV1,
) -> bool {
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

pub(super) fn relative_segments<'a>(
    root: &SourceNodeSiteV1,
    site: &'a SourceNodeSiteV1,
) -> Option<&'a [SourcePathSegmentV1]> {
    let root_segments = root.segments();
    site.segments().starts_with(root_segments).then(|| {
        let split = root_segments.len();
        &site.segments()[split..]
    })
}

pub(super) fn is_direct_child(
    root: &SourceNodeSiteV1,
    child: &SourceNodeSiteV1,
    expected: SourcePathSegmentV1,
) -> bool {
    relative_segments(root, child).is_some_and(|relative| relative == [expected])
}
