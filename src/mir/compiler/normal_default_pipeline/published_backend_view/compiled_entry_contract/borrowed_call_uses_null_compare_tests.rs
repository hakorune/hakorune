//! Null-equality view closure: the lent tagged view may answer `==` only
//! against the exact `ConstValue::Null` producer, once per admitted use.
use super::*;
use crate::mir::ConstValue;

fn null_fixture() -> (FunctionUses, MirFunction, ValueId, BindingRefV1) {
    let (mut state, mut function, _) = fixture();
    let carrier = *state.roots.keys().next().unwrap();
    let formal = *state.roots.values().next().unwrap();
    state.null_admissions.insert(formal, 1);
    function
        .blocks
        .get_mut(&BasicBlockId(0))
        .unwrap()
        .instructions
        .push(MirInstruction::Copy {
            dst: ValueId(700),
            src: carrier,
        });
    (state, function, ValueId(700), formal)
}

fn null_const(dst: ValueId) -> MirInstruction {
    MirInstruction::Const {
        dst,
        value: ConstValue::Null,
    }
}

fn i64_const(dst: ValueId) -> MirInstruction {
    MirInstruction::Const {
        dst,
        value: ConstValue::Integer(0),
    }
}

#[test]
fn borrowed_use_null_compare_view_passes_in_either_operand_order() {
    // The lent view serves the `==` operand; the sibling is the exact
    // null producer, never a forged zero.
    let (mut state, mut function, view, _) = null_fixture();
    function
        .blocks
        .get_mut(&BasicBlockId(0))
        .unwrap()
        .instructions
        .extend([
            null_const(ValueId(800)),
            MirInstruction::Compare {
                dst: ValueId(701),
                op: CompareOp::Eq,
                lhs: view,
                rhs: ValueId(800),
            },
        ]);
    verify(&mut state, &function).unwrap();

    // The null producer may sit in the lhs position as well.
    let (mut state, mut function, view, _) = null_fixture();
    function
        .blocks
        .get_mut(&BasicBlockId(0))
        .unwrap()
        .instructions
        .extend([
            null_const(ValueId(800)),
            MirInstruction::Compare {
                dst: ValueId(701),
                op: CompareOp::Eq,
                lhs: ValueId(800),
                rhs: view,
            },
        ]);
    verify(&mut state, &function).unwrap();

    // The tracked carrier itself may serve the operand directly, without
    // a lent view copy.
    let (mut state, mut function, _) = fixture();
    let carrier = *state.roots.keys().next().unwrap();
    let formal = *state.roots.values().next().unwrap();
    state.null_admissions.insert(formal, 1);
    function
        .blocks
        .get_mut(&BasicBlockId(0))
        .unwrap()
        .instructions
        .extend([
            null_const(ValueId(800)),
            MirInstruction::Compare {
                dst: ValueId(701),
                op: CompareOp::Eq,
                lhs: carrier,
                rhs: ValueId(800),
            },
        ]);
    verify(&mut state, &function).unwrap();
}

#[test]
fn borrowed_use_rejects_null_compare_operator_and_sibling_drift() {
    // `!=` stays outside the bounded envelope.
    let (mut state, mut function, view, _) = null_fixture();
    function
        .blocks
        .get_mut(&BasicBlockId(0))
        .unwrap()
        .instructions
        .extend([
            null_const(ValueId(800)),
            MirInstruction::Compare {
                dst: ValueId(701),
                op: CompareOp::Ne,
                lhs: view,
                rhs: ValueId(800),
            },
        ]);
    assert!(verify(&mut state, &function)
        .unwrap_err()
        .contains("forbidden-operand"));

    // An Integer literal is not the null producer — a forged zero may
    // never stand in for the sentinel.
    let (mut state, mut function, view, _) = null_fixture();
    function
        .blocks
        .get_mut(&BasicBlockId(0))
        .unwrap()
        .instructions
        .extend([
            i64_const(ValueId(800)),
            MirInstruction::Compare {
                dst: ValueId(701),
                op: CompareOp::Eq,
                lhs: view,
                rhs: ValueId(800),
            },
        ]);
    assert!(verify(&mut state, &function)
        .unwrap_err()
        .contains("forbidden-operand"));

    // General tagged equality between two lent views stays outside.
    let (mut state, mut function, view, _) = null_fixture();
    let carrier = *state.roots.keys().next().unwrap();
    function
        .blocks
        .get_mut(&BasicBlockId(0))
        .unwrap()
        .instructions
        .push(MirInstruction::Compare {
            dst: ValueId(701),
            op: CompareOp::Eq,
            lhs: view,
            rhs: carrier,
        });
    assert!(verify(&mut state, &function)
        .unwrap_err()
        .contains("forbidden-operand"));
}

#[test]
fn borrowed_use_null_compare_coverage_is_per_admission() {
    // The admitted use never observed: coverage stays per admission.
    let (mut state, function, _) = fixture();
    let formal = *state.roots.values().next().unwrap();
    state.null_admissions.insert(formal, 1);
    assert!(verify(&mut state, &function)
        .unwrap_err()
        .contains("null-coverage"));

    // An observed equality without an admission is coverage drift, never
    // a silently widened use.
    let (mut state, mut function, view, _) = null_fixture();
    state.null_admissions.clear();
    function
        .blocks
        .get_mut(&BasicBlockId(0))
        .unwrap()
        .instructions
        .extend([
            null_const(ValueId(800)),
            MirInstruction::Compare {
                dst: ValueId(701),
                op: CompareOp::Eq,
                lhs: view,
                rhs: ValueId(800),
            },
        ]);
    assert!(verify(&mut state, &function)
        .unwrap_err()
        .contains("null-coverage"));
}
