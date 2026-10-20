//! Exact source-owned write operands and guards in both independent scans.
use super::*;
#[path = "borrowed_call_uses_owned_field_write.rs"]
mod owned;
pub(super) use owned::{OwnedFieldWriteUse, entry as local_entry};

pub(super) struct FieldWriteUse {
    instruction: MirInstruction,
    receiver: BindingRefV1,
    value: BindingRefV1,
    null_compare: (Binding, Coordinate),
    branch: (Binding, Coordinate),
    nonnull: BasicBlockId,
    integer_compare: Coordinate,
    guard_dependencies: Vec<(Binding, Coordinate)>,
}

pub(super) fn entry(state: &mut FunctionUses, source: &FinalizedRootSourceHandoffV1,
    owner: FunctionOwnerIdV1, function: &MirFunction,
    compares: &BTreeMap<crate::mir::resolved_semantics::OwnedExprSiteV1, Coordinate>,
) -> Result<(), String> {
    source.with_finished_borrowed_field_writes_v1(owner, function, |loan, invoke, pins, guard| {
        let integer_compare = *compares.get(loan.integer_guard_site_v1())
            .ok_or_else(|| fault("borrowed-write/integer-guard-missing"))?;
        let receiver = loan.receiver().1;
        let value = loan.value().1;
        if !state.compare_coordinates.as_ref().is_some_and(|rows| rows.get(&value)
            .is_some_and(|uses| uses.iter().any(|(coordinate, _)| *coordinate == integer_compare)))
        { return Err(fault("borrowed-write/integer-guard-formal")); }
        let MirInstruction::Invoke { operation: InvokeOperation::FieldSet { field, .. }, .. } = &invoke.0.1
            else { return Err(fault("borrowed-write/invoke-kind")) };
        if *field != loan.field_source().field() { return Err(fault("borrowed-write/field")); }
        for copies in pins {
            for (binding, coordinate) in copies { state.copy(binding, Some(*coordinate))?; }
        }
        if state.field_writes.insert(invoke.1, FieldWriteUse { instruction: invoke.0.1.clone(),
            receiver, value, null_compare: guard.0.clone(), branch: guard.1.clone(),
            nonnull: guard.2, integer_compare, guard_dependencies: guard.3.clone() }).is_some()
        { return Err(fault("borrowed-write/duplicate")); }
        Ok(())
    })
}

/// Recheck the original guard instructions and exact receiver role in each
/// scan; a different null comparison cannot lend its non-null successor.
pub(super) fn guards(state: &FunctionUses, tracked: &BTreeMap<ValueId, BindingRefV1>,
    views: &ViewScan, indexed: &[(Coordinate, &MirInstruction)],
) -> Result<(), String> {
    owned::guards(state, indexed)?;
    for write in state.field_writes.values() {
        for expected in [&write.null_compare, &write.branch].into_iter().chain(write.guard_dependencies.iter()) {
            if indexed.iter().filter(|(coordinate, row)| *coordinate == expected.1 && **row == expected.0.1).count() != 1 {
                return Err(fault("borrowed-write/guard-instruction"));
            }
        }
        let MirInstruction::Compare { op: crate::mir::CompareOp::Eq, lhs, rhs, .. } = &write.null_compare.0.1
            else { return Err(fault("borrowed-write/null-compare")) };
        let formal = |value: &ValueId| tracked.get(value).or(views.views.get(value));
        if !((formal(lhs) == Some(&write.receiver) && views.null_consts.contains(rhs))
            || (formal(rhs) == Some(&write.receiver) && views.null_consts.contains(lhs)))
            || !matches!(write.branch.0.1, MirInstruction::Branch { else_bb, .. } if else_bb == write.nonnull)
        { return Err(fault("borrowed-write/null-guard-formal")); }
    }
    Ok(())
}

pub(super) fn observe(state: &FunctionUses, tracked: &BTreeMap<ValueId, BindingRefV1>,
    coordinate: Coordinate, instruction: &MirInstruction, scan: &mut Scan,
    dominates: &dyn Fn(BasicBlockId, BasicBlockId) -> bool,
) -> Result<bool, String> {
    if owned::observe(state, coordinate, instruction, scan, dominates)? { return Ok(true); }
    let Some(expected) = state.field_writes.get(&coordinate) else { return Ok(false) };
    if instruction != &expected.instruction { return Err(fault("borrowed-write/instruction")); }
    let MirInstruction::Invoke { operation: InvokeOperation::FieldSet { mode: crate::mir::instruction::FieldSetModeV1::ExactSlot, field, base, value }, fault_frame, .. } = instruction
        else { return Err(fault("borrowed-write/invoke-kind")) };
    if tracked.get(base) != Some(&expected.receiver) || tracked.get(value) != Some(&expected.value)
        || tracked.contains_key(fault_frame)
        || state.object_views.get(&expected.receiver) != Some(&field.object())
    { return Err(fault("borrowed-write/operand-roles")); }
    let precedes = |from: Coordinate, to: Coordinate| dominates(from.0, to.0)
        && (from.0 != to.0 || from.1 < to.1);
    if !precedes(expected.null_compare.1, expected.branch.1)
        || !dominates(expected.nonnull, coordinate.0)
        || !precedes(expected.integer_compare, coordinate)
    { return Err(fault("borrowed-write/guard-dominance")); }
    if !scan.field_writes.insert(coordinate) { return Err(fault("borrowed-write/duplicate-use")); }
    Ok(true)
}

pub(super) fn coverage(state: &FunctionUses, scan: &Scan) -> Result<(), String> {
    owned::coverage(state, scan)?;
    if state.field_writes.keys().copied().collect::<BTreeSet<_>>() != scan.field_writes {
        return Err(fault("borrowed-write/coverage"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Independent operand-closure inputs; not source admission or runtime proof.
    #[test]
    fn borrowed_write_requires_exact_store_roles_guards_and_coverage() {
        for mutation in 0..8 {
            let (mut state, mut function, _) = super::super::tests::fixture();
            let receiver = *state.roots.values().next().unwrap();
            let value = BindingRefV1::new(receiver.owner(), hakorune_mir_core::BindingId(receiver.binding().0 + 1));
            let object = hakorune_mir_defs::CanonicalObjectIdV1::from_declaration_index(0).unwrap();
            let field = hakorune_mir_defs::CanonicalFieldRefV1::from_declaration_ordinal(object, 0).unwrap();
            state = FunctionUses::default();
            state.roots = [(ValueId(10), receiver), (ValueId(11), value)].into();
            state.object_views.insert(receiver, object);
            state.null_admissions.insert(receiver, 1);
            state.compare_admissions.insert(value, 1);
            state.compare_coordinates = Some([(value, BTreeSet::from([((BasicBlockId(0), 3), 0)]))].into());
            let null_compare = MirInstruction::Compare { dst: ValueId(92), op: crate::mir::CompareOp::Eq, lhs: ValueId(10), rhs: ValueId(90) };
            let branch = MirInstruction::Branch { condition: ValueId(92), then_bb: BasicBlockId(1), else_bb: BasicBlockId(2), then_edge_args: None, else_edge_args: None };
            let store = MirInstruction::Invoke { operation: InvokeOperation::FieldSet { mode: crate::mir::instruction::FieldSetModeV1::ExactSlot, field, base: ValueId(10), value: ValueId(11) },
                fault_frame: ValueId(94), normal_landing: BasicBlockId(3), fault_landing: BasicBlockId(4) };
            function.params = vec![ValueId(10), ValueId(11)];
            function.blocks.clear();
            for id in 0..5 { function.blocks.insert(BasicBlockId(id), crate::mir::BasicBlock::new(BasicBlockId(id))); }
            let entry = function.blocks.get_mut(&BasicBlockId(0)).unwrap();
            entry.instructions = vec![
                MirInstruction::Const { dst: ValueId(90), value: crate::mir::ConstValue::Null },
                MirInstruction::Const { dst: ValueId(91), value: crate::mir::ConstValue::Integer(0) },
                null_compare.clone(),
                MirInstruction::Compare { dst: ValueId(93), op: crate::mir::CompareOp::Le, lhs: ValueId(11), rhs: ValueId(91) }];
            entry.terminator = Some(branch.clone());
            entry.successors = BTreeSet::from([BasicBlockId(1), BasicBlockId(2)]);
            let live = function.blocks.get_mut(&BasicBlockId(2)).unwrap();
            live.terminator = Some(store.clone());
            live.successors = BTreeSet::from([BasicBlockId(3), BasicBlockId(4)]);
            for id in [1, 3] { function.blocks.get_mut(&BasicBlockId(id)).unwrap().terminator = Some(MirInstruction::Return { value: Some(ValueId(91)) }); }
            function.blocks.get_mut(&BasicBlockId(4)).unwrap().terminator = Some(MirInstruction::ReturnFault { fault_frame: ValueId(94) });
            let coordinate = (BasicBlockId(2), 0);
            let mut expected = FieldWriteUse { instruction: store, receiver, value,
                null_compare: ((BasicBlockId(0), null_compare), (BasicBlockId(0), 2)),
                branch: ((BasicBlockId(0), branch), (BasicBlockId(0), 4)),
                nonnull: BasicBlockId(2), integer_compare: (BasicBlockId(0), 3), guard_dependencies: Vec::new() };
            match mutation {
                1 => expected.receiver = value,
                2 => expected.value = receiver,
                3 => expected.integer_compare = (BasicBlockId(2), 0),
                4 => expected.nonnull = BasicBlockId(1),
                5 => expected.null_compare.1.1 = 1,
                6 => { if let MirInstruction::Invoke { operation: InvokeOperation::FieldSet { value, .. }, .. } = function.blocks.get_mut(&BasicBlockId(2)).unwrap().terminator.as_mut().unwrap() { *value = ValueId(91); } },
                7 => { function.blocks.get_mut(&BasicBlockId(2)).unwrap().terminator = Some(MirInstruction::Return { value: Some(ValueId(91)) }); },
                _ => {}
            }
            state.field_writes.insert(coordinate, expected);
            let tracked = state.tracked().unwrap();
            let result = state.verify_module(&tracked, &function);
            if mutation == 0 { result.unwrap(); } else { assert!(result.is_err(), "mutation={mutation}"); }
        }
    }
}
