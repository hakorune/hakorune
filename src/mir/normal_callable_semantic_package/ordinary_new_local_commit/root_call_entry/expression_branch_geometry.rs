use super::*;

/// One geometry verifier for source-qualified Normal branch values. Payload
/// type, ownership and original incoming producers remain with each caller.
pub(super) fn validate_branch_geometry_v1(
    pending: &crate::mir::builder::IfConditionMaterializationV1,
    branch: &Binding,
    exits: [BasicBlockId; 2],
    jumps: &[Binding; 2],
    values: [ValueId; 2],
    relation: &crate::mir::builder::LocalSsaPhysicalRelationV1,
    function: &crate::mir::verification::geometry::FunctionVerificationGeometry<'_>,
    finishing: Option<&Finished>,
    kind_refusal: &'static str,
) -> Result<(), String> {
    let map_block = |block| {
        finishing
            .and_then(|projection| projection.destination(block))
            .unwrap_or(block)
    };
    let actual = |row: &Binding| -> Result<Option<Binding>, String> {
        match finishing {
            Some(projection) if row.1.dst_value().is_some() => {
                projection.relation_binding(function, row)
            }
            Some(projection) => projection.captured_effect_binding(function, row).map(Some),
            None => {
                if function.blocks.get(&row.0).is_none_or(|body| {
                    body.all_instructions().filter(|old| **old == row.1).count() != 1
                }) {
                    return Err(freeze("expression/reference-dependency-missing"));
                }
                Ok(Some(row.clone()))
            }
        }
    };
    relation.require_completed_v1()?;
    if finishing.is_some() {
        relation.verify_projected_with_geometry(function, &mut |row| actual(row))?;
        pending
            .relation()
            .verify_projected_with_geometry(function, &mut |row| actual(row))?;
    } else {
        relation.verify_with_geometry(function)?;
        pending.relation().verify_with_geometry(function)?;
    }
    let branch =
        actual(branch)?.ok_or_else(|| freeze("expression/reference-dependency-missing"))?;
    let merge = map_block(pending.merge());
    let arms = [map_block(pending.arms().0), map_block(pending.arms().1)];
    let exits = exits.map(map_block);
    if branch.0 != map_block(pending.block())
        || !matches!(branch.1, MirInstruction::Branch { condition, then_bb, else_bb, .. }
            if condition == pending.values().1 && [then_bb, else_bb] == arms)
    {
        return Err(freeze(kind_refusal));
    }
    for (index, jump) in jumps.iter().enumerate() {
        let jump =
            actual(jump)?.ok_or_else(|| freeze("expression/reference-dependency-missing"))?;
        if jump.0 != exits[index]
            || !matches!(jump.1, MirInstruction::Jump { target, .. } if target == merge)
            || !branch_reaches_v1(function, arms[index], exits[index], merge)
            || branch_reaches_v1(function, arms[index], exits[1 - index], merge)
        {
            return Err(freeze(kind_refusal));
        }
    }
    let expected: std::collections::BTreeSet<_> = exits.into_iter().collect();
    let actual_preds: std::collections::BTreeSet<_> = function
        .predecessors()
        .get(&merge)
        .into_iter()
        .flatten()
        .copied()
        .collect();
    if expected.len() != 2
        || actual_preds != expected
        || function
            .blocks
            .get(&merge)
            .is_none_or(|body| body.predecessors != expected)
    {
        return Err(freeze(kind_refusal));
    }
    // The merge writer may retain one already-dominating SSA value instead
    // of emitting a Phi. Both original arms and all geometry above still apply.
    if values[0] == values[1] && relation.values() == (values[0], values[0]) {
        if !relation.is_identity_v1()
            || !crate::mir::builder::reusable_branch_value_v1(function, merge, &exits, values[0])
        {
            return Err(freeze(kind_refusal));
        }
        return Ok(());
    }
    let phi = actual(relation.identity_phi_binding_v1()?)?
        .ok_or_else(|| freeze("expression/reference-dependency-missing"))?;
    let mut expected_inputs = [(exits[0], values[0]), (exits[1], values[1])];
    expected_inputs.sort_by_key(|(block, _)| block.0);
    if phi.0 != merge
        || !matches!(phi.1, MirInstruction::Phi { dst, ref inputs, .. }
        if dst == relation.values().0 && inputs.as_slice() == expected_inputs)
    {
        return Err(freeze(kind_refusal));
    }
    Ok(())
}

fn branch_reaches_v1(
    function: &MirFunction,
    entry: BasicBlockId,
    exit: BasicBlockId,
    merge: BasicBlockId,
) -> bool {
    let mut pending = vec![entry];
    let mut seen = std::collections::BTreeSet::new();
    while let Some(block) = pending.pop() {
        if block == merge || !seen.insert(block) {
            continue;
        }
        if block == exit {
            return true;
        }
        if let Some(body) = function.blocks.get(&block) {
            pending.extend(body.out_edges().into_iter().map(|edge| edge.target));
        }
    }
    false
}
