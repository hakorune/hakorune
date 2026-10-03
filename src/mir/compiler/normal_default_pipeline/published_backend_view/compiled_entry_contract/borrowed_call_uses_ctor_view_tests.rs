//! Dominated constructor-argument view closure: the lent tagged view may
//! reach a `new` argument ordinal only through the admitted source site.
use super::*;

fn ctor_call(receiver: ValueId, args: Vec<ValueId>) -> MirInstruction {
    MirInstruction::call(
        None,
        Callee::BirthConstructor {
            key: hakorune_mir_defs::callable_key::CanonicalSameModuleCallableKeyV1::birth_constructor(
                "Item", 2,
            ),
            receiver,
        },
        args,
        crate::mir::EffectMask::PURE,
    )
}

#[test]
fn borrowed_use_dominated_ctor_view_passes() {
    // Same block, ordered after the compare's site check: the lent view
    // serves the `new`-argument ordinal; the receiver lane stays clean.
    let (mut state, mut function, view, formal) = compare_fixture();
    state.ctor_admissions.insert(formal, 1);
    function
        .blocks
        .get_mut(&BasicBlockId(0))
        .unwrap()
        .instructions
        .extend([
            MirInstruction::Compare {
                dst: ValueId(701),
                op: CompareOp::Gt,
                lhs: view,
                rhs: ValueId(800),
            },
            ctor_call(ValueId(820), vec![view, ValueId(830)]),
        ]);
    verify(&mut state, &function).unwrap();
    // The tracked carrier itself may serve the argument directly.
    let (mut state, mut function, view, formal) = compare_fixture();
    state.ctor_admissions.insert(formal, 1);
    let carrier = *state.roots.keys().next().unwrap();
    function
        .blocks
        .get_mut(&BasicBlockId(0))
        .unwrap()
        .instructions
        .extend([
            MirInstruction::Compare {
                dst: ValueId(701),
                op: CompareOp::Gt,
                lhs: view,
                rhs: ValueId(800),
            },
            ctor_call(ValueId(820), vec![carrier, ValueId(830)]),
        ]);
    verify(&mut state, &function).unwrap();
}

#[test]
fn borrowed_use_rejects_undominated_and_drifting_ctor_view() {
    // Ordered `new` before the compare's site check in the same block.
    let (mut state, mut function, view, formal) = compare_fixture();
    state.ctor_admissions.insert(formal, 1);
    function
        .blocks
        .get_mut(&BasicBlockId(0))
        .unwrap()
        .instructions
        .extend([
            ctor_call(ValueId(820), vec![view, ValueId(830)]),
            MirInstruction::Compare {
                dst: ValueId(701),
                op: CompareOp::Gt,
                lhs: view,
                rhs: ValueId(800),
            },
        ]);
    assert!(verify(&mut state, &function)
        .unwrap_err()
        .contains("undominated-view"));

    // The admitted `new` use never observed: coverage stays per admission.
    let (mut state, mut function, view, formal) = compare_fixture();
    state.ctor_admissions.insert(formal, 1);
    function
        .blocks
        .get_mut(&BasicBlockId(0))
        .unwrap()
        .instructions
        .push(MirInstruction::Compare {
            dst: ValueId(701),
            op: CompareOp::Gt,
            lhs: view,
            rhs: ValueId(800),
        });
    assert!(verify(&mut state, &function)
        .unwrap_err()
        .contains("ctor-coverage"));

    // The lent view may not move into the receiver lane.
    let (mut state, mut function, view, formal) = compare_fixture();
    state.ctor_admissions.insert(formal, 1);
    let carrier = *state.roots.keys().next().unwrap();
    function
        .blocks
        .get_mut(&BasicBlockId(0))
        .unwrap()
        .instructions
        .extend([
            MirInstruction::Compare {
                dst: ValueId(701),
                op: CompareOp::Gt,
                lhs: view,
                rhs: ValueId(800),
            },
            ctor_call(carrier, vec![view, ValueId(830)]),
        ]);
    assert!(verify(&mut state, &function)
        .unwrap_err()
        .contains("callee-operand"));
}
