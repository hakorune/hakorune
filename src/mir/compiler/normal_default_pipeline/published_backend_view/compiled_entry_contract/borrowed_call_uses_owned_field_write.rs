//! LOCAL writes have source-owned coordinates, never invented formal roots.
use super::*;

pub(in super::super) struct OwnedFieldWriteUse {
    instruction: MirInstruction,
    new_invoke: (Binding, Coordinate),
    bindings: Vec<(Binding, Coordinate)>,
    before_store: Vec<Coordinate>,
    ready_reads: Vec<Coordinate>,
    handle_values: BTreeSet<ValueId>,
    read_dependencies: Vec<(Coordinate, Coordinate)>,
}

pub(in super::super) fn entry(state: &mut FunctionUses, source: &FinalizedRootSourceHandoffV1,
    owner: FunctionOwnerIdV1, function: &MirFunction,
) -> Result<(), String> {
    source.with_finished_owned_field_writes_v1(owner, function, |row| {
        if state.field_writes.contains_key(&row.invoke.1)
            || state.owned_field_writes.insert(row.invoke.1, OwnedFieldWriteUse {
                instruction: row.invoke.0.1, new_invoke: row.new_invoke,
                bindings: row.bindings, before_store: row.before_store, ready_reads: row.ready_reads,
                handle_values: row.handle_values,
                read_dependencies: row.read_dependencies,
            }).is_some() { return Err(fault("owned-write/duplicate-source")); }
        Ok(())
    })
}

/// Run independently on module and published coordinates, including the whole
/// RHS, exact New/Normal/copy source and the writer's Normal/Fault graph.
pub(super) fn guards(state: &FunctionUses, indexed: &[(Coordinate, &MirInstruction)]) -> Result<(), String> {
    for row in state.owned_field_writes.values() {
        for (binding, coordinate) in &row.bindings {
            if indexed.iter().filter(|(site, actual)| site == coordinate && **actual == binding.1).count() != 1 {
                return Err(fault("owned-write/coordinate-inventory"));
            }
            if let Some(dst) = binding.1.dst_value() {
                if indexed.iter().filter(|(_, actual)| actual.dst_value() == Some(dst)).count() != 1 {
                    return Err(fault("owned-write/producer-uniqueness"));
                }
            }
        }
        for (coordinate, instruction) in indexed {
            if instruction.used_values().iter().any(|value| row.handle_values.contains(value))
                && !state.owned_field_writes.values().any(|write| write.bindings.iter()
                    .any(|(binding, expected)| coordinate == expected && **instruction == binding.1)) {
                return Err(fault("owned-write/unowned-handle-use"));
            }
        }
    }
    Ok(())
}

pub(super) fn observe(state: &FunctionUses, coordinate: Coordinate, instruction: &MirInstruction,
    scan: &mut Scan, dominates: &dyn Fn(BasicBlockId, BasicBlockId) -> bool,
) -> Result<bool, String> {
    let Some(expected) = state.owned_field_writes.get(&coordinate) else { return Ok(false) };
    let MirInstruction::Invoke { operation: InvokeOperation::FieldSet { .. }, normal_landing, fault_landing, .. } = instruction
        else { return Err(fault("owned-write/store-shape")); };
    let MirInstruction::Invoke { operation: InvokeOperation::NewBox { .. }, normal_landing: new_normal, .. } = expected.new_invoke.0.1
        else { return Err(fault("owned-write/new-shape")); };
    let precedes = |from: Coordinate, to: Coordinate| dominates(from.0, to.0) && (from.0 != to.0 || from.1 < to.1);
    if instruction != &expected.instruction || normal_landing == fault_landing
        || !dominates(new_normal, coordinate.0)
        || !precedes(expected.new_invoke.1, coordinate)
        || expected.before_store.iter().any(|before| !precedes(*before, coordinate))
        || expected.ready_reads.iter().any(|read| !dominates(*normal_landing, read.0))
        || expected.read_dependencies.iter().any(|(before, read)| !precedes(*before, *read))
    { return Err(fault("owned-write/normal-order")); }
    if !scan.owned_field_writes.insert(coordinate) { return Err(fault("owned-write/duplicate-use")); }
    Ok(true)
}

pub(super) fn coverage(state: &FunctionUses, scan: &Scan) -> Result<(), String> {
    if state.owned_field_writes.keys().copied().collect::<BTreeSet<_>>() != scan.owned_field_writes {
        return Err(fault("owned-write/coverage"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Independent scan inputs, not source admission or a fabricated New plan.
    #[test]
    fn owned_write_scan_closes_producers_handle_uses_and_normal_order() {
        let object = hakorune_mir_defs::CanonicalObjectIdV1::from_declaration_index(0).unwrap();
        let field = hakorune_mir_defs::CanonicalFieldRefV1::from_declaration_ordinal(object, 0).unwrap();
        let bb = BasicBlockId;
        let v = ValueId;
        let new = MirInstruction::Invoke { operation: InvokeOperation::NewBox { object },
            fault_frame: v(20), normal_landing: bb(1), fault_landing: bb(8) };
        let store = MirInstruction::Invoke { operation: InvokeOperation::FieldSet { field, base: v(2), value: v(3) },
            fault_frame: v(21), normal_landing: bb(2), fault_landing: bb(8) };
        let read = MirInstruction::Invoke { operation: InvokeOperation::FieldGet { field, base: v(2) },
            fault_frame: v(22), normal_landing: bb(3), fault_landing: bb(8) };
        let rows = vec![((bb(0), 0), new.clone()),
            ((bb(1), 0), MirInstruction::InvokeNormalResult { dst: v(1), invoke_block: bb(0) }),
            ((bb(1), 1), MirInstruction::Copy { dst: v(2), src: v(1) }),
            ((bb(1), 2), MirInstruction::Const { dst: v(3), value: crate::mir::ConstValue::Integer(7) }),
            ((bb(1), 3), store.clone()), ((bb(2), 0), read.clone())];
        for mutation in 0..9 {
            let mut state = FunctionUses::default();
            state.owned_field_writes.insert((bb(1), 3), OwnedFieldWriteUse {
                instruction: store.clone(), new_invoke: ((bb(0), new.clone()), (bb(0), 0)),
                bindings: rows.iter().map(|(coordinate, row)| ((coordinate.0, row.clone()), *coordinate)).collect(),
                before_store: vec![(bb(1), 0), (bb(1), 1), (bb(1), 2)],
                ready_reads: vec![(bb(2), 0)], handle_values: BTreeSet::from([v(1), v(2)]),
                read_dependencies: vec![((bb(1), 1), (bb(2), 0))],
            });
            let mut actual = rows.clone();
            match mutation {
                1 => actual.push(((bb(9), 0), MirInstruction::Const { dst: v(3), value: crate::mir::ConstValue::Integer(99) })),
                2 => actual.push(((bb(9), 0), store.clone())),
                3 => actual.push(((bb(9), 0), read.clone())),
                4 => actual.push(((bb(9), 0), MirInstruction::Copy { dst: v(9), src: v(2) })),
                5 => { actual.pop(); },
                6 => actual[3].1 = MirInstruction::Const { dst: v(3), value: crate::mir::ConstValue::Integer(99) },
                _ => {},
            }
            let indexed: Vec<_> = actual.iter().map(|(coordinate, row)| (*coordinate, row)).collect();
            let result = (|| {
                guards(&state, &indexed)?;
                let mut scan = Scan::default();
                let dominates = |from, to| (from == to || matches!((from, to),
                    (BasicBlockId(0), BasicBlockId(1) | BasicBlockId(2)) | (BasicBlockId(1), BasicBlockId(2))))
                    && !(mutation == 7 && from == bb(2) && to == bb(2))
                    && !(mutation == 8 && from == bb(1) && to == bb(2));
                assert!(observe(&state, (bb(1), 3), &store, &mut scan, &dominates)?);
                coverage(&state, &scan)
            })();
            if mutation == 0 { result.unwrap(); } else { assert!(result.is_err(), "mutation={mutation}"); }
        }
    }
}
