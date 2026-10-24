//! Corroborate source-issued borrowed Return provenance in both scans.
//! These expected occurrences grant neither a new origin nor ownership.
use super::*;
use crate::mir::normal_callable_semantic_package::FinalizedObjectReturnOriginV1;

pub(super) struct FormalReturnUse {
    source: BorrowedReturnSource,
    returned: Binding,
}
enum BorrowedReturnSource {
    Formal(BindingRefV1),
    MapSlot {
        values: BTreeSet<ValueId>,
        bindings: BTreeMap<Coordinate, MirInstruction>,
    },
}

fn coordinate(function: &MirFunction, binding: &Binding) -> Result<Coordinate, String> {
    let block = function.blocks.get(&binding.0)
        .ok_or_else(|| fault("borrowed-formal-return/block"))?;
    let mut matches = block.all_instructions().enumerate()
        .filter(|(_, instruction)| **instruction == binding.1);
    let (index, _) = matches.next().ok_or_else(|| fault("borrowed-formal-return/missing"))?;
    if matches.next().is_some() { return Err(fault("borrowed-formal-return/duplicate")); }
    Ok((binding.0, index))
}

pub(super) fn prepare(
    state: &mut FunctionUses,
    function: &MirFunction,
    rows: &[PublishedLifecyclePhysicalFunctionV1<'_>],
) -> Result<(), String> {
    let mut matches = rows.iter().filter(|row| row.name() == function.signature.name);
    let published = matches.next().ok_or_else(|| fault("borrowed-formal-return/function"))?;
    if matches.next().is_some() { return Err(fault("borrowed-formal-return/duplicate-function")); }
    prepare_role(state, function, published.role())
}

fn prepare_role(
    state: &mut FunctionUses,
    function: &MirFunction,
    role: &super::super::PublishedLifecyclePhysicalFunctionRoleV1,
) -> Result<(), String> {
    let Some(result @ InvokeCallResultKind::BorrowedNullableHandle { .. }) = role.ordinary_result() else {
        return Ok(());
    };
    for outcome in role.object_return_outcomes()
        .ok_or_else(|| fault("borrowed-formal-return/outcomes"))?
    {
        outcome.validate_physical_function_v1(function, result, role.has_receiver())?;
        let source = match outcome.origin() {
            FinalizedObjectReturnOriginV1::BorrowedFormal { value, .. } => {
                let formal = *state.roots.get(value)
                    .ok_or_else(|| fault("borrowed-formal-return/original-entry"))?;
                for original in outcome.transport_bindings() {
                    state.copy(original, Some(coordinate(function, original)?))?;
                }
                BorrowedReturnSource::Formal(formal)
            }
            FinalizedObjectReturnOriginV1::MapSlot(receipt) => {
                let value = receipt.producer().1.dst_value()
                    .ok_or_else(|| fault("borrowed-formal-return/original-entry"))?;
                let mut values = BTreeSet::from([value]);
                for (_, instruction) in outcome.transport_bindings() {
                    if let MirInstruction::Copy { dst, .. } = instruction { values.insert(*dst); }
                }
                let mut bindings = BTreeMap::new();
                for binding in receipt.bindings()
                    .chain(outcome.transport_bindings()).chain(std::iter::once(outcome.returned())) {
                    let site = coordinate(function, binding)?;
                    if bindings.insert(site, binding.1.clone()).is_some() {
                        return Err(fault("borrowed-formal-return/duplicate-source"));
                    }
                }
                BorrowedReturnSource::MapSlot { values, bindings }
            }
            _ => continue, // Null has no borrowed operand; other roles grant no exception.
        };
        let site = coordinate(function, outcome.returned())?;
        if state.integer_returns.contains_key(&site)
            || state.formal_returns.insert(site, FormalReturnUse {
                source, returned: outcome.returned().clone(),
            }).is_some()
        { return Err(fault("borrowed-formal-return/duplicate-source")); }
    }
    Ok(())
}

pub(super) fn observe(
    state: &FunctionUses,
    tracked: &BTreeMap<ValueId, BindingRefV1>,
    site: Coordinate,
    instruction: &MirInstruction,
    scan: &mut Scan,
) -> Result<(), String> {
    let expected = state.formal_returns.get(&site)
        .ok_or_else(|| fault("borrowed-formal-return/unproved"))?;
    let MirInstruction::Return { value: Some(value) } = instruction else {
        return Err(fault("borrowed-formal-return/instruction"));
    };
    let valid = match &expected.source {
        BorrowedReturnSource::Formal(formal) => tracked.get(value) == Some(formal),
        BorrowedReturnSource::MapSlot { values, .. } => values.contains(value) && !tracked.contains_key(value),
    };
    if expected.returned != (site.0, instruction.clone()) || !valid {
        return Err(fault("borrowed-formal-return/correspondence"));
    }
    if !scan.formal_returns.insert(site) {
        return Err(fault("borrowed-formal-return/duplicate-return"));
    }
    Ok(())
}

/// Inspect exact producer/effect/Copy/Return coordinates in both independent
/// scans. Map payloads never become entry roots or lend formal-use exemptions.
pub(super) fn observe_map_slots(state: &FunctionUses, site: Coordinate,
    instruction: &MirInstruction, scan: &mut Scan,
) -> Result<(), String> {
    for expected in state.formal_returns.values() {
        let BorrowedReturnSource::MapSlot { values, bindings } = &expected.source else { continue; };
        if let Some(original) = bindings.get(&site) {
            if original != instruction { return Err(fault("borrowed-formal-return/correspondence")); }
            scan.map_slot_returns.insert(site);
        } else if instruction.used_values().iter().any(|value| values.contains(value))
            && !matches!(instruction, MirInstruction::KeepAlive { .. }) {
            return Err(fault("borrowed-formal-return/unproved"));
        }
    }
    Ok(())
}

pub(super) fn coverage(state: &FunctionUses, scan: &Scan) -> Result<(), String> {
    if state.formal_returns.keys().copied().collect::<BTreeSet<_>>() != scan.formal_returns {
        return Err(fault("borrowed-formal-return/coverage"));
    }
    let expected: BTreeSet<_> = state.formal_returns.values().flat_map(|row| match &row.source {
        BorrowedReturnSource::MapSlot { bindings, .. } => bindings.keys().copied().collect::<Vec<_>>(),
        _ => Vec::new(),
    }).collect();
    if expected != scan.map_slot_returns { return Err(fault("borrowed-formal-return/coverage")); }
    Ok(())
}

#[cfg(test)]
pub(in crate::mir::compiler::normal_default_pipeline::published_backend_view) fn verify_original_formal_return(
    source: &FinalizedRootSourceHandoffV1,
    module: &crate::mir::MirModule,
    published: &PublishedLifecyclePhysicalFunctionV1<'_>,
) -> Result<(), String> {
    let function = &module.functions[published.name()];
    let key = published.role().ordinary_target().unwrap();
    let mut owner = None;
    source.visit_finalized_borrowed_call_nodes_v1(module, |packet, _, _, _, _| {
        if packet.target() == key {
            if owner.is_some_and(|previous| previous != packet.callee_owner()) {
                return Err(fault("borrowed-formal-return/test-owner"));
            }
            owner = Some(packet.callee_owner());
        }
        Ok(())
    })?;
    let mut uses = BorrowedCallUses::default();
    uses.entry(source, owner.ok_or_else(|| fault("borrowed-formal-return/test-incoming"))?, function)?;
    let mut state = uses.functions.remove(published.name()).unwrap();
    guarded_arguments::prepare(&mut state)?;
    prepare_role(&mut state, function, published.role())?;
    assert!(!state.formal_returns.is_empty(), "actual source lends a formal Return");
    state.verify_module(&state.tracked()?, function)?;
    state.verify_published(&state.tracked()?, function, std::slice::from_ref(published))?;
    for mutation in ["operand", "missing", "extra"] {
        let mut changed = function.clone();
        let (&site, expected) = state.formal_returns.iter().next().unwrap();
        let block = changed.blocks.get_mut(&site.0).unwrap();
        match mutation {
            "operand" => block.terminator = Some(MirInstruction::Return { value: Some(ValueId(999999)) }),
            "missing" => block.terminator = None,
            _ => block.instructions.push(expected.returned.1.clone()),
        }
        assert!(state.verify_module(&state.tracked()?, &changed).is_err(), "formal Return {mutation}");
    }
    Ok(())
}
