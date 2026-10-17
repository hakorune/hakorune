//! Exact original guarded forwarding, independently checked in both final scans.
use super::*;
use crate::mir::resolved_semantics::OwnedExprSiteV1;

#[derive(Default)]
pub(super) struct GuardedArguments {
    sources: BTreeMap<(OwnedExprSiteV1, u32), (BindingRefV1, BindingRefV1, Coordinate)>,
    calls: BTreeMap<OwnedExprSiteV1, (Coordinate, BTreeMap<u32, (BindingRefV1, BindingRefV1)>)>,
    expected: BTreeMap<(BasicBlockId, usize, usize), (BindingRefV1, Coordinate)>,
}

pub(super) fn entry(
    state: &mut FunctionUses,
    source: &FinalizedRootSourceHandoffV1,
    owner: FunctionOwnerIdV1,
    function: &MirFunction,
    compares: &BTreeMap<OwnedExprSiteV1, Coordinate>,
) -> Result<(), String> {
    let entry = source.borrowed_ordinary_entry_source_for_function_v1(owner, function)?;
    for (call, ordinal, binding, formal, binary) in entry.guarded_argument_uses()? {
        let guard = *compares
            .get(&binary)
            .ok_or_else(|| fault("borrowed-use/argument-guard-missing"))?;
        if !state.compare_coordinates.as_ref().is_some_and(|rows| {
            rows.get(&formal)
                .is_some_and(|uses| uses.iter().any(|(coordinate, _)| *coordinate == guard))
        }) {
            return Err(fault("borrowed-use/argument-guard-formal"));
        }
        if state
            .guarded_arguments
            .sources
            .insert((call, ordinal), (binding, formal, guard))
            .is_some()
        {
            return Err(fault("borrowed-use/argument-source-duplicate"));
        }
    }
    Ok(())
}

impl BorrowedCallUses {
    pub(in crate::mir::compiler::normal_default_pipeline::published_backend_view::compiled_entry_contract::borrowed_call_incoming) fn guarded_call(
        &mut self,
        call: &OwnedExprSiteV1,
        caller: &MirFunction,
        coordinate: Coordinate,
        actuals: &[PreparedBorrowedFormalActualV1],
    ) -> Result<(), String> {
        let state = self
            .functions
            .entry(caller.signature.name.clone())
            .or_default();
        let mut formals = BTreeMap::new();
        for actual in actuals {
            if let Some((binding, formal)) = actual.source.caller_formal_origin_v1() {
                if formals.insert(actual.ordinal, (binding, formal)).is_some() {
                    return Err(fault("borrowed-use/guarded-actual-duplicate"));
                }
            }
        }
        if state
            .guarded_arguments
            .calls
            .insert(call.clone(), (coordinate, formals))
            .is_some()
        {
            return Err(fault("borrowed-use/guarded-call-duplicate"));
        }
        Ok(())
    }
}

/// Entry and call visitation may occur in either order; join only at closure.
pub(super) fn prepare(state: &mut FunctionUses) -> Result<(), String> {
    for ((call, ordinal), (binding, formal, guard)) in &state.guarded_arguments.sources {
        let (coordinate, actuals) = state
            .guarded_arguments
            .calls
            .get(call)
            .ok_or_else(|| fault("borrowed-use/guarded-call-missing"))?;
        let slot = (coordinate.0, coordinate.1, *ordinal as usize);
        if actuals.get(ordinal) != Some(&(*binding, *formal))
            || state.arguments.get(&slot) != Some(formal)
            || state
                .guarded_arguments
                .expected
                .insert(slot, (*formal, *guard))
                .is_some()
        {
            return Err(fault("borrowed-use/guarded-call-correspondence"));
        }
    }
    Ok(())
}

pub(super) fn observe(
    state: &FunctionUses,
    coordinate: Coordinate,
    ordinal: usize,
    formal: BindingRefV1,
    dominates: &dyn Fn(BasicBlockId, BasicBlockId) -> bool,
) -> Result<(), String> {
    if let Some((expected, guard)) =
        state
            .guarded_arguments
            .expected
            .get(&(coordinate.0, coordinate.1, ordinal))
    {
        if *expected != formal
            || !dominates(guard.0, coordinate.0)
            || (guard.0 == coordinate.0 && guard.1 >= coordinate.1)
        {
            return Err(fault("borrowed-use/guarded-call-dominance"));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guarded_forward_requires_exact_compare_order_and_cfg_dominance() {
        for mutation in 0..3 {
            let (mut state, mut function, _) = super::super::tests::fixture();
            let carrier = *state.roots.keys().next().unwrap();
            let formal = state.roots[&carrier];
            state.compare_admissions.insert(formal, 1);
            let block = function.blocks.get_mut(&BasicBlockId(0)).unwrap();
            let guard = (BasicBlockId(0), block.instructions.len());
            block.instructions.push(MirInstruction::Compare {
                dst: ValueId(701),
                op: crate::mir::CompareOp::Le,
                lhs: carrier,
                rhs: ValueId(800),
            });
            let call_index = block.instructions.len();
            let ordinal = state.arguments.keys().next().unwrap().2;
            let expected_guard = match mutation {
                1 => (BasicBlockId(0), call_index),
                2 => (BasicBlockId(90), 0),
                _ => guard,
            };
            state.guarded_arguments.expected.insert(
                (BasicBlockId(0), call_index, ordinal),
                (formal, expected_guard),
            );
            let result = super::super::tests::verify(&mut state, &function);
            if mutation == 0 {
                result.unwrap();
            } else {
                assert!(result.unwrap_err().contains("guarded-call-dominance"));
            }
        }
    }
}
