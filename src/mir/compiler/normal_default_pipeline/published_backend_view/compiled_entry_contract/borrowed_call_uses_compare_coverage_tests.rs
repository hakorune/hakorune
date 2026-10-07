//! Exact physical comparison occurrences corroborate original source uses.
use super::*;

#[test]
fn borrowed_use_checked_compare_counts_each_physical_operand_occurrence() {
    // Reusing a carrier does not grant a second source execution.
    let (mut state, mut function, view, formal) = compare_fixture();
    for (index, dst) in [(0usize, ValueId(701)), (1, ValueId(702))] {
        let _ = index;
        function
            .blocks
            .get_mut(&BasicBlockId(0))
            .unwrap()
            .instructions
            .push(MirInstruction::Compare {
                dst,
                op: CompareOp::Gt,
                lhs: view,
                rhs: ValueId(800),
            });
    }
    assert!(verify(&mut state, &function)
        .unwrap_err()
        .contains("compare-coverage"));
    state.compare_admissions.insert(formal, 2);
    verify(&mut state, &function).unwrap();
    let coordinates: BTreeSet<_> = function.blocks[&BasicBlockId(0)]
        .instructions
        .iter()
        .enumerate()
        .filter_map(|(index, row)| {
            matches!(row, MirInstruction::Compare { .. }).then_some(((BasicBlockId(0), index), 0))
        })
        .collect();
    state.compare_coordinates = Some([(formal, coordinates.clone())].into());
    verify(&mut state, &function).unwrap();
    state.compare_coordinates = Some(
        [(
            formal,
            coordinates
                .into_iter()
                .map(|(coordinate, _)| (coordinate, 1))
                .collect(),
        )]
        .into(),
    );
    assert!(verify(&mut state, &function)
        .unwrap_err()
        .contains("compare-coverage"));
}

#[test]
fn borrowed_use_checked_compare_counts_shared_carrier_on_both_sides() {
    let (mut state, mut function, view, formal) = compare_fixture();
    function
        .blocks
        .get_mut(&BasicBlockId(0))
        .unwrap()
        .instructions
        .push(MirInstruction::Compare {
            dst: ValueId(701),
            op: CompareOp::Gt,
            lhs: view,
            rhs: view,
        });
    assert!(verify(&mut state, &function)
        .unwrap_err()
        .contains("compare-coverage"));
    state.compare_admissions.insert(formal, 2);
    verify(&mut state, &function).unwrap();
}
