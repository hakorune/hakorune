//! Exact original field-index Set occurrences, independently checked per scan.
use super::*;

pub(super) struct ArraySetUse {
    instruction: MirInstruction,
    value: BindingRefV1,
    index_formal: BindingRefV1,
    index_field: hakorune_mir_defs::CanonicalFieldRefV1,
    reads: [(Binding, Coordinate); 2],
    dependencies: Vec<(Binding, Coordinate)>,
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
    source.with_finished_borrowed_array_sets_v1(owner, function, |loan, write, reads, dependencies, guard| {
        let integer_compare = *compares.get(loan.integer_guard())
            .ok_or_else(|| fault("borrowed-set/integer-guard-missing"))?;
        if !state.compare_coordinates.as_ref().is_some_and(|rows| rows.get(&loan.value_formal())
            .is_some_and(|uses| uses.iter().any(|(coordinate, _)| *coordinate == integer_compare))) {
            return Err(fault("borrowed-set/integer-guard-formal"));
        }
        let MirInstruction::ArrayElementWrite { value, .. } = write.0.1 else {
            return Err(fault("borrowed-set/write-kind"));
        };
        // Only the value's original Copy chain carries the borrowed scalar.
        // Receiver/index dependency copies never gain formal-view permission.
        let mut current = value;
        let mut seen = BTreeSet::new();
        while !state.roots.contains_key(&current) && !state.copies.contains_key(&current) {
            if !seen.insert(current) { return Err(fault("borrowed-set/value-copy-cycle")); }
            let Some((binding, coordinate)) = dependencies.iter().find(|(binding, _)|
                matches!(binding.1, MirInstruction::Copy { dst, .. } if dst == current)) else { break };
            let MirInstruction::Copy { src, .. } = binding.1 else { unreachable!() };
            state.copy(binding, Some(*coordinate))?;
            current = src;
        }
        if state.array_sets.insert(write.1, ArraySetUse {
            instruction: write.0.1.clone(), value: loan.value_formal(), index_formal: loan.index_formal(),
            index_field: loan.index_field(), reads: reads.clone(), dependencies: dependencies.to_vec(),
            null_compare: guard.0.clone(), branch: guard.1.clone(), nonnull: guard.2,
            integer_compare, guard_dependencies: guard.3.clone(),
        }).is_some() { return Err(fault("borrowed-set/duplicate")); }
        Ok(())
    })
}
pub(super) fn guards(state: &FunctionUses, tracked: &BTreeMap<ValueId, BindingRefV1>,
    views: &ViewScan, indexed: &[(Coordinate, &MirInstruction)],
) -> Result<(), String> {
    for expected in state.array_sets.values() {
        for row in [&expected.null_compare, &expected.branch].into_iter()
            .chain(expected.guard_dependencies.iter()).chain(expected.dependencies.iter()).chain(expected.reads.iter()) {
            if indexed.iter().filter(|(coordinate, instruction)| *coordinate == row.1 && **instruction == row.0.1).count() != 1 {
                return Err(fault("borrowed-set/dependency-instruction"));
            }
        }
        let MirInstruction::Compare { op: crate::mir::CompareOp::Eq, lhs, rhs, .. } = expected.null_compare.0.1
            else { return Err(fault("borrowed-set/null-compare")) };
        let formal = |value: &ValueId| tracked.get(value).or(views.views.get(value));
        if !((formal(&lhs) == Some(&expected.index_formal) && views.null_consts.contains(&rhs))
            || (formal(&rhs) == Some(&expected.index_formal) && views.null_consts.contains(&lhs)))
            || !matches!(expected.branch.0.1, MirInstruction::Branch { else_bb, .. } if else_bb == expected.nonnull) {
            return Err(fault("borrowed-set/null-guard-formal"));
        }
    }
    Ok(())
}
pub(super) fn observe(state: &FunctionUses, tracked: &BTreeMap<ValueId, BindingRefV1>, views: &ViewScan,
    coordinate: Coordinate, instruction: &MirInstruction, scan: &mut Scan,
    dominates: &dyn Fn(BasicBlockId, BasicBlockId) -> bool,
) -> Result<bool, String> {
    let Some(expected) = state.array_sets.get(&coordinate) else { return Ok(false) };
    if instruction != &expected.instruction { return Err(fault("borrowed-set/instruction")); }
    let MirInstruction::ArrayElementWrite { kind: crate::mir::ArrayElementWriteKind::Set,
        producer: crate::mir::ArrayWriteProducerKind::MethodCall, receiver, index: Some(index), value, .. } = instruction
        else { return Err(fault("borrowed-set/write-kind")) };
    let formal = |value: &ValueId| tracked.get(value).or(views.views.get(value));
    let MirInstruction::ObjectFieldGet { field, base, .. } = expected.reads[1].0.1
        else { return Err(fault("borrowed-set/index-read-kind")) };
    if formal(value) != Some(&expected.value) || formal(receiver).is_some() || formal(index).is_some()
        || field != expected.index_field || formal(&base) != Some(&expected.index_formal)
        || state.object_views.get(&expected.index_formal) != Some(&field.object()) {
        return Err(fault("borrowed-set/operand-roles"));
    }
    let precedes = |from: Coordinate, to: Coordinate| dominates(from.0, to.0)
        && (from.0 != to.0 || from.1 < to.1);
    if !precedes(expected.null_compare.1, expected.branch.1)
        || !dominates(expected.nonnull, expected.reads[1].1.0)
        || !precedes(expected.integer_compare, coordinate)
        || expected.reads.iter().any(|(_, read)| !precedes(*read, coordinate)) {
        return Err(fault("borrowed-set/guard-dominance"));
    }
    if !scan.array_sets.insert(coordinate) { return Err(fault("borrowed-set/duplicate-use")); }
    Ok(true)
}
pub(super) fn coverage(state: &FunctionUses, scan: &Scan) -> Result<(), String> {
    if state.array_sets.keys().copied().collect::<BTreeSet<_>>() != scan.array_sets {
        return Err(fault("borrowed-set/coverage"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Independent final operand/CFG inputs. This is not source or runtime proof.
    #[test]
    fn field_index_set_requires_exact_occurrence_reads_roles_and_guards() {
        for mutation in 0..10 {
            let (original, mut function, _) = super::super::tests::fixture();
            let index_formal = *original.roots.values().next().unwrap();
            let value_formal = BindingRefV1::new(index_formal.owner(),
                hakorune_mir_core::BindingId(index_formal.binding().0 + 1));
            let object = hakorune_mir_defs::CanonicalObjectIdV1::from_declaration_index(0).unwrap();
            let field = hakorune_mir_defs::CanonicalFieldRefV1::from_declaration_ordinal(object, 0).unwrap();
            let mut state = FunctionUses::default();
            state.roots = [(ValueId(10), index_formal), (ValueId(11), value_formal)].into();
            state.object_views.insert(index_formal, object);
            state.null_admissions.insert(index_formal, 1);
            state.field_admissions.insert(index_formal, 1);
            state.compare_admissions.insert(value_formal, 1);
            state.compare_coordinates = Some([(value_formal,
                BTreeSet::from([((BasicBlockId(0), 3), 0)]))].into());
            let null = MirInstruction::Compare { dst: ValueId(92), op: crate::mir::CompareOp::Eq,
                lhs: ValueId(10), rhs: ValueId(90) };
            let branch = MirInstruction::Branch { condition: ValueId(92), then_bb: BasicBlockId(1),
                else_bb: BasicBlockId(2), then_edge_args: None, else_edge_args: None };
            let array = MirInstruction::FieldGet { dst: ValueId(20), base: ValueId(99),
                field: "sizes".into(), declared_type: Some(crate::mir::MirType::Box("ArrayBox".into())) };
            let index = MirInstruction::ObjectFieldGet { dst: ValueId(21), base: ValueId(10), field };
            let write = MirInstruction::ArrayElementWrite { site_id: crate::mir::ArrayWriteSiteId(1),
                dst: None, kind: crate::mir::ArrayElementWriteKind::Set,
                producer: crate::mir::ArrayWriteProducerKind::MethodCall,
                receiver: ValueId(20), index: Some(ValueId(21)), value: ValueId(11) };
            function.params = vec![ValueId(99), ValueId(10), ValueId(11)];
            function.blocks.clear();
            for id in 0..3 { function.blocks.insert(BasicBlockId(id), crate::mir::BasicBlock::new(BasicBlockId(id))); }
            let entry = function.blocks.get_mut(&BasicBlockId(0)).unwrap();
            entry.instructions = vec![
                MirInstruction::Const { dst: ValueId(90), value: crate::mir::ConstValue::Null },
                MirInstruction::Const { dst: ValueId(91), value: crate::mir::ConstValue::Integer(0) },
                null.clone(),
                MirInstruction::Compare { dst: ValueId(93), op: crate::mir::CompareOp::Le,
                    lhs: ValueId(11), rhs: ValueId(91) }];
            entry.terminator = Some(branch.clone());
            entry.successors = BTreeSet::from([BasicBlockId(1), BasicBlockId(2)]);
            function.blocks.get_mut(&BasicBlockId(1)).unwrap().terminator =
                Some(MirInstruction::Return { value: Some(ValueId(91)) });
            let live = function.blocks.get_mut(&BasicBlockId(2)).unwrap();
            live.instructions = vec![array.clone(), index.clone(), write.clone()];
            live.terminator = Some(MirInstruction::Return { value: Some(ValueId(91)) });
            let mut expected = ArraySetUse { instruction: write, value: value_formal, index_formal,
                index_field: field, reads: [((BasicBlockId(2), array), (BasicBlockId(2), 0)),
                    ((BasicBlockId(2), index), (BasicBlockId(2), 1))], dependencies: Vec::new(),
                null_compare: ((BasicBlockId(0), null), (BasicBlockId(0), 2)),
                branch: ((BasicBlockId(0), branch), (BasicBlockId(0), 4)),
                nonnull: BasicBlockId(2), integer_compare: (BasicBlockId(0), 3),
                guard_dependencies: Vec::new() };
            match mutation {
                1 => expected.value = index_formal,
                2 => expected.index_formal = value_formal,
                3 => expected.integer_compare = (BasicBlockId(2), 2),
                4 => expected.nonnull = BasicBlockId(1),
                5 => expected.reads[1].1.1 = 0,
                6 => {
                    if let MirInstruction::ArrayElementWrite { value, .. } =
                        &mut function.blocks.get_mut(&BasicBlockId(2)).unwrap().instructions[2] {
                        *value = ValueId(91);
                    }
                }
                7 => { function.blocks.get_mut(&BasicBlockId(2)).unwrap().instructions.pop(); }
                8 => {
                    let extra = expected.instruction.clone();
                    function.blocks.get_mut(&BasicBlockId(2)).unwrap().instructions.push(extra);
                }
                9 => expected.reads[1].0.1 = MirInstruction::ObjectFieldGet {
                    dst: ValueId(21), base: ValueId(99), field },
                _ => {}
            }
            state.array_sets.insert((BasicBlockId(2), 2), expected);
            let tracked = state.tracked().unwrap();
            let result = state.verify_module(&tracked, &function);
            if mutation == 0 { result.unwrap(); }
            else { assert!(result.is_err(), "mutation={mutation}"); }
        }
    }
}
