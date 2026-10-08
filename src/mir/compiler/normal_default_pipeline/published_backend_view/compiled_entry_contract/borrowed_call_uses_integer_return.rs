//! Exact source-issued Return occurrence and its original checked Compare.
//! Shared by independent module and published scans; never count-only admission.
use super::*;

pub(super) struct IntegerReturnUse {
    formal: BindingRefV1,
    value: ValueId,
    guard: Coordinate,
    compare: Binding,
}

pub(super) fn entry(
    state: &mut FunctionUses,
    source: &FinalizedRootSourceHandoffV1,
    owner: FunctionOwnerIdV1,
    function: &MirFunction,
) -> Result<(), String> {
    source.with_borrowed_ordinary_integer_returns_v1(
        owner,
        function,
        |_, formal, _, binding, coordinate, guard, compare| {
            let MirInstruction::Return { value: Some(value) } = binding.1 else {
                return Err(fault("borrowed-return/return-kind"));
            };
            if state
                .integer_returns
                .insert(
                    coordinate,
                    IntegerReturnUse {
                        formal,
                        value,
                        guard,
                        compare: compare.clone(),
                    },
                )
                .is_some()
            {
                return Err(fault("borrowed-return/duplicate-source"));
            }
            Ok(())
        },
    )
}

pub(super) fn guards(
    state: &FunctionUses,
    indexed: &[(Coordinate, &MirInstruction)],
) -> Result<(), String> {
    for expected in state.integer_returns.values() {
        let mut actual = indexed.iter().filter(|(site, _)| *site == expected.guard);
        let Some((_, instruction)) = actual.next() else {
            return Err(fault("borrowed-return/guard-missing"));
        };
        if **instruction != expected.compare.1 || actual.next().is_some() {
            return Err(fault("borrowed-return/guard-correspondence"));
        }
    }
    Ok(())
}

pub(super) fn observe(
    state: &FunctionUses,
    tracked: &BTreeMap<ValueId, BindingRefV1>,
    coordinate: Coordinate,
    value: &Option<ValueId>,
    scan: &mut Scan,
    dominates: &dyn Fn(BasicBlockId, BasicBlockId) -> bool,
) -> Result<(), String> {
    let formal = value.and_then(|value| tracked.get(&value));
    let expected = state
        .integer_returns
        .get(&coordinate)
        .ok_or_else(|| fault("borrowed-return/unproved-return"))?;
    if *value != Some(expected.value) || formal != Some(&expected.formal) {
        return Err(fault("borrowed-return/operand-correspondence"));
    }
    if !dominates(expected.guard.0, coordinate.0)
        || (expected.guard.0 == coordinate.0 && expected.guard.1 >= coordinate.1)
    {
        return Err(fault("borrowed-return/undominated-return"));
    }
    if !scan.integer_returns.insert(coordinate) {
        return Err(fault("borrowed-return/duplicate-return"));
    }
    Ok(())
}

pub(super) fn coverage(state: &FunctionUses, scan: &Scan) -> Result<(), String> {
    if state
        .integer_returns
        .keys()
        .copied()
        .collect::<BTreeSet<_>>()
        != scan.integer_returns
    {
        return Err(fault("borrowed-return/return-coverage"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> (FunctionUses, MirFunction, Coordinate, Coordinate, ValueId) {
        let (mut state, mut function, _) = super::super::tests::fixture();
        state.arguments.clear();
        let (&value, &formal) = state.roots.iter().next().unwrap();
        let block = function.blocks.get_mut(&BasicBlockId(0)).unwrap();
        let guard = (block.id, block.instructions.len());
        let compare = MirInstruction::Compare {
            dst: ValueId(701),
            op: crate::mir::CompareOp::Le,
            lhs: value,
            rhs: ValueId(800),
        };
        block.instructions.push(compare.clone());
        let returned = (block.id, block.instructions.len());
        block.terminator = Some(MirInstruction::Return { value: Some(value) });
        state.compare_admissions.insert(formal, 1);
        state.integer_returns.insert(
            returned,
            IntegerReturnUse {
                formal,
                value,
                guard,
                compare: (block.id, compare),
            },
        );
        (state, function, guard, returned, value)
    }

    #[test]
    fn checked_integer_return_exact_coordinate_guard_and_coverage() {
        let (mut state, function, guard, returned, value) = fixture();
        super::super::tests::verify(&mut state, &function).unwrap();
        for case in ["operand", "missing", "guard", "ordering", "extra"] {
            let (mut state, mut function, _, _, _) = fixture();
            let block = function.blocks.get_mut(&BasicBlockId(0)).unwrap();
            match case {
                "operand" => {
                    block.terminator = Some(MirInstruction::Return {
                        value: Some(ValueId(900)),
                    })
                }
                "missing" => block.terminator = None,
                "guard" => {
                    if let MirInstruction::Compare { op, .. } = &mut block.instructions[guard.1] {
                        *op = crate::mir::CompareOp::Gt;
                    }
                }
                "ordering" => state.integer_returns.get_mut(&returned).unwrap().guard = returned,
                "extra" => block
                    .instructions
                    .push(MirInstruction::Return { value: Some(value) }),
                _ => unreachable!(),
            }
            assert!(
                super::super::tests::verify(&mut state, &function).is_err(),
                "{case}"
            );
        }
    }

    #[test]
    fn checked_integer_return_does_not_admit_new_view_copy() {
        let (mut state, mut function, _, returned, value) = fixture();
        let block = function.blocks.get_mut(&BasicBlockId(0)).unwrap();
        block.instructions.push(MirInstruction::Copy {
            dst: ValueId(702),
            src: value,
        });
        block.terminator = Some(MirInstruction::Return {
            value: Some(ValueId(702)),
        });
        // Even aligned expected coordinates/value cannot grant an unissued source alias.
        let mut expected = state.integer_returns.remove(&returned).unwrap();
        expected.value = ValueId(702);
        state
            .integer_returns
            .insert((block.id, block.instructions.len()), expected);
        let error = super::super::tests::verify(&mut state, &function).unwrap_err();
        assert!(error.contains("unproved-copy"), "{error}");
    }
    #[test]
    fn checked_integer_return_rejects_existing_compare_view_escape() {
        let (mut state, mut function, guard, _, value) = fixture();
        state.integer_returns.clear();
        let block = function.blocks.get_mut(&BasicBlockId(0)).unwrap();
        block.instructions.insert(
            guard.1,
            MirInstruction::Copy {
                dst: ValueId(702),
                src: value,
            },
        );
        if let MirInstruction::Compare { lhs, .. } = &mut block.instructions[guard.1 + 1] {
            *lhs = ValueId(702);
        }
        block.terminator = Some(MirInstruction::Return {
            value: Some(ValueId(702)),
        });
        // The Compare may lend its own view; no source Return was admitted.
        let error = super::super::tests::verify(&mut state, &function).unwrap_err();
        assert!(error.contains("forbidden-operand"), "{error}");
    }
}
