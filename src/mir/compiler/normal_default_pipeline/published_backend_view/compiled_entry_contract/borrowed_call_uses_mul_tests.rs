//! Final Mul scan pins exact source-selected coordinate, side, Copy, and guard.
use super::*;
use crate::mir::CompareOp;

fn mul_fixture() -> (FunctionUses, MirFunction, Coordinate) {
    let (mut state, mut function, _) = super::tests::fixture();
    let carrier = *state.roots.keys().next().unwrap();
    let formal = *state.roots.values().next().unwrap();
    let block = BasicBlockId(0);
    let rows = &mut function.blocks.get_mut(&block).unwrap().instructions;
    let guard = (block, rows.len());
    rows.push(MirInstruction::Compare {
        dst: ValueId(701),
        op: CompareOp::Le,
        lhs: carrier,
        rhs: ValueId(800),
    });
    let copy = (block, rows.len());
    rows.push(MirInstruction::Copy {
        dst: ValueId(702),
        src: carrier,
    });
    let mul = (block, rows.len());
    rows.push(MirInstruction::BinOp {
        dst: ValueId(703),
        op: crate::mir::BinaryOp::Mul,
        lhs: ValueId(702),
        rhs: ValueId(800),
    });
    state.compare_admissions.insert(formal, 1);
    state.compare_coordinates = Some([(formal, BTreeSet::from([(guard, 0)]))].into());
    state.mul_expected.insert(
        (mul, 0),
        mul::MulOperandUse {
            formal,
            operand: ValueId(702),
            copy: Some(copy),
            guard,
        },
    );
    (state, function, mul)
}

#[test]
fn borrowed_mul_selected_copy_and_guard_pass_final_scan() {
    let (mut state, function, _) = mul_fixture();
    super::tests::verify(&mut state, &function).unwrap();
}

#[test]
fn borrowed_mul_refuses_side_copy_guard_and_unselected_use_drift() {
    for mutation in 0..5 {
        let (mut state, mut function, mul) = mul_fixture();
        let mut expected = state.mul_expected.remove(&(mul, 0)).unwrap();
        let mut selected_side = 0;
        match mutation {
            0 => expected.operand = ValueId(999),
            1 => expected.copy = Some((mul.0, mul.1 + 1)),
            2 => expected.guard = (mul.0, mul.1 + 1),
            3 => selected_side = 1,
            4 => {
                function
                    .blocks
                    .get_mut(&mul.0)
                    .unwrap()
                    .instructions
                    .push(MirInstruction::BinOp {
                        dst: ValueId(704),
                        op: crate::mir::BinaryOp::Mul,
                        lhs: *state.roots.keys().next().unwrap(),
                        rhs: ValueId(800),
                    });
            }
            _ => unreachable!(),
        }
        state.mul_expected.insert((mul, selected_side), expected);
        let error = super::tests::verify(&mut state, &function).unwrap_err();
        assert!(
            error.contains("mul-operand") || error.contains("forbidden-operand"),
            "mutation={mutation}: {error}"
        );
    }
}
