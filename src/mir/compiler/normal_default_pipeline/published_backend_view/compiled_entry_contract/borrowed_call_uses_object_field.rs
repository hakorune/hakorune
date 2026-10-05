//! Guarded `formal.field` physical closure: non-null successor anchors,
//! the `object_field_get` operand admission, and the final count.
//! The sealed object view and the original draft pin admission; this
//! child mints no class from layout and no liveness from the compare.
use super::*;
use crate::mir::resolved_semantics::BindingRefV1;

/// Exact dominance by reachability cut: `from` dominates `to` when every
/// entry->`to` path visits `from`. An unreachable `to` counts as dominated,
/// matching the verifier's DominatorTree convention.
pub(super) fn path_dominates(
    entry: BasicBlockId,
    successors: &BTreeMap<BasicBlockId, Vec<BasicBlockId>>,
    from: BasicBlockId,
    to: BasicBlockId,
) -> bool {
    if from == to {
        return true;
    }
    let mut seen = BTreeSet::from([from]);
    let mut stack = vec![entry];
    while let Some(node) = stack.pop() {
        if node == to {
            return false;
        }
        if !seen.insert(node) {
            continue;
        }
        if let Some(next) = successors.get(&node) {
            stack.extend(next.iter().copied());
        }
    }
    true
}

/// Each formal's non-null successor blocks: for every borrowed null
/// compare (tracked or view operand against the exact `null` producer),
/// the `else` successor of the consuming `if` branch is the surviving
/// path where the borrowed object is live. The compare must come first —
/// `null_consts` is complete only after the first instruction pass, so
/// the anchor pass runs last.
pub(super) fn collect_nonnull_successors(
    tracked: &BTreeMap<ValueId, BindingRefV1>,
    views: &BTreeMap<ValueId, BindingRefV1>,
    null_consts: &BTreeSet<ValueId>,
    indexed: &[(Coordinate, &MirInstruction)],
) -> BTreeMap<BindingRefV1, Vec<BasicBlockId>> {
    let mut branches: BTreeMap<ValueId, BasicBlockId> = BTreeMap::new();
    for (_, instruction) in indexed {
        if let MirInstruction::Branch {
            condition, else_bb, ..
        } = instruction
        {
            branches.insert(*condition, *else_bb);
        }
    }
    let mut successors: BTreeMap<BindingRefV1, Vec<BasicBlockId>> = BTreeMap::new();
    for (_, instruction) in indexed {
        let MirInstruction::Compare {
            op: crate::mir::CompareOp::Eq,
            lhs,
            rhs,
            dst,
            ..
        } = instruction
        else {
            continue;
        };
        for (operand, sibling) in [(lhs, rhs), (rhs, lhs)] {
            let formal = tracked.get(operand).or(views.get(operand));
            if formal.is_some() && null_consts.contains(sibling) {
                if let Some(else_bb) = branches.get(dst) {
                    successors
                        .entry(*formal.unwrap())
                        .or_default()
                        .push(*else_bb);
                }
            }
        }
    }
    successors
}

/// One `object_field_get` on a lent view: the read must sit inside the
/// formal's non-null dominance cone and its canonical object must equal
/// the sealed object view exactly — the field ordinal and object are
/// already canonical, so a class or slot mismatch is drift, never a
/// second interpretation.
#[allow(clippy::too_many_arguments)]
pub(super) fn observe_operand(
    tracked: &BTreeMap<ValueId, BindingRefV1>,
    views: &ViewScan,
    object_views: &BTreeMap<BindingRefV1, hakorune_mir_defs::CanonicalObjectIdV1>,
    base: &ValueId,
    field: &hakorune_mir_defs::CanonicalFieldRefV1,
    coordinate: Coordinate,
    field_uses: &mut BTreeMap<BindingRefV1, usize>,
    dominates: &dyn Fn(BasicBlockId, BasicBlockId) -> bool,
) -> Result<(), String> {
    let Some(formal) = tracked.get(base).or(views.views.get(base)) else {
        return Ok(());
    };
    let dominated = views.nonnull_successors.get(formal).is_some_and(|blocks| {
        blocks.iter().any(|&block| dominates(block, coordinate.0))
    });
    if !dominated {
        return Err(fault("borrowed-use/undominated-view"));
    }
    if object_views.get(formal) != Some(&field.object()) {
        return Err(fault("borrowed-use/object-view"));
    }
    *field_uses.entry(*formal).or_default() += 1;
    Ok(())
}

/// Each sealed formal object view as `param value -> canonical object
/// declaration index`. Every original class/null view is retained, including
/// ignored declared formals and forwarding-only opaque roots. A view without
/// its exact entry root is drift; field-use coverage remains independent.
pub(super) fn object_view_values(state: &FunctionUses) -> Result<BTreeMap<u32, u32>, String> {
    let mut rows = BTreeMap::new();
    for (formal, object) in &state.object_views {
        let mut values = state.roots.iter().filter(|(_, root)| *root == formal);
        let (value, _) = values
            .next()
            .ok_or_else(|| fault("borrowed-use/object-view-root"))?;
        if values.next().is_some() || rows.insert(value.0, object.declaration_index()).is_some() {
            return Err(fault("borrowed-use/object-view-duplicate"));
        }
    }
    Ok(rows)
}

/// Final per-formal coverage: every admitted guarded field read produced
/// exactly its physical `object_field_get` row.
pub(super) fn coverage(
    field_uses: &BTreeMap<BindingRefV1, usize>,
    field_admissions: &BTreeMap<BindingRefV1, usize>,
) -> Result<(), String> {
    if field_uses != field_admissions {
        return Err(fault("borrowed-use/field-coverage"));
    }
    Ok(())
}
