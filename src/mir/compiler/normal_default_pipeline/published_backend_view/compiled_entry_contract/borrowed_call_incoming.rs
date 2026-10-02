//! Bidirectional final Call coverage, demanded before ordinary result filtering.
//! Metadata selects inspection; original source and finished packets prove it.
use super::*;
use crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1 as Carrier;
use crate::mir::MirModule;

pub(in crate::mir::compiler::normal_default_pipeline::published_backend_view) fn verify_borrowed_call_incoming(
    program: &PublishedLifecyclePhysicalProgramV1<'_>,
    module: &MirModule,
) -> Result<(), String> {
    let mut callees = BTreeMap::new();
    for function in program.functions() {
        let Some(carriers) = function.param_carriers() else {
            continue;
        };
        if !carriers.contains(&Carrier::BorrowedTaggedValue) {
            continue;
        }
        let key = function
            .role()
            .ordinary_target()
            .ok_or_else(|| fault("borrowed-incoming/nonordinary-carrier"))?;
        if key.namespace() != hakorune_mir_defs::SameModuleCallableNamespaceV1::InstanceBoxMethod
            || carriers.len() != function.params().len()
            || callees.insert(key, function).is_some()
        {
            return Err(fault("borrowed-incoming/callee-metadata"));
        }
    }
    if callees.is_empty() {
        return Ok(());
    }
    let source = program
        .handoff()
        .root_source()
        .ok_or_else(|| fault("borrowed-incoming/source-missing"))?;
    let mut physical = BTreeMap::new();
    // Inspect every caller and every Call result, including Birth/Unit/Handle.
    for (caller_index, function) in program.functions().iter().enumerate() {
        for block in function.blocks() {
            for instruction in block
                .instructions()
                .iter()
                .copied()
                .chain(std::iter::once(block.terminator()))
            {
                let MirInstruction::Invoke {
                    operation: InvokeOperation::Call { call, result },
                    ..
                } = instruction.instruction()
                else {
                    continue;
                };
                let Callee::SameModuleInstance { key, .. } = &call.callee else {
                    continue;
                };
                if !callees.contains_key(key) {
                    continue;
                }
                if *result != InvokeCallResultKind::I64 {
                    return Err(fault("borrowed-incoming/result-mismatch"));
                }
                if physical
                    .insert(
                        (caller_index, block.id(), instruction.index() as usize),
                        (call, result),
                    )
                    .is_some()
                {
                    return Err(fault("borrowed-incoming/physical-coordinate-duplicate"));
                }
            }
        }
    }
    let mut witnessed = BTreeSet::new();
    let mut owners = BTreeMap::new();
    let mut incoming = BTreeSet::new();
    let mut original = BTreeSet::new();
    source.visit_finalized_lexical_call_nodes_v1(
        module,
        |_, _, packet, _, caller, (block, index), _copies| {
            let row = packet.original_row();
            let Some(callee) = callees.get(row.target()) else {
                return Ok(());
            };
            if let Some(previous) = owners.insert(row.target().clone(), row.callee_owner()) {
                if previous != row.callee_owner() {
                    return Err(fault("borrowed-incoming/callee-owner"));
                }
            }
            let callee_function = module
                .functions
                .get(callee.name())
                .ok_or_else(|| fault("borrowed-incoming/callee-function-missing"))?;
            let entry = source.borrowed_ordinary_entry_source_for_function_v1(
                row.callee_owner(),
                callee_function,
            )?;
            let values = source.borrowed_ordinary_entry_values_v1(row.callee_owner())?;
            let offset = usize::from(callee.role().has_receiver());
            let carriers = callee.param_carriers().expect("selected borrowed column");
            let selected_slots: BTreeSet<_> = carriers
                .iter()
                .enumerate()
                .filter_map(|(slot, carrier)| {
                    (*carrier == Carrier::BorrowedTaggedValue).then_some(slot)
                })
                .collect();
            let mut proven_slots = BTreeSet::new();
            for (ordinal, _, value) in values.iter() {
                let slot = *ordinal as usize + offset;
                if callee.params().get(slot) != Some(value)
                    || carriers.get(slot) != Some(&Carrier::BorrowedTaggedValue)
                {
                    return Err(fault("borrowed-incoming/entry-correspondence"));
                }
                proven_slots.insert(slot);
            }
            if selected_slots != proven_slots {
                return Err(fault("borrowed-incoming/entry-coverage"));
            }
            for target in entry.incoming_targets() {
                if target.target() != row.target() || target.callee_owner() != row.callee_owner() {
                    return Err(fault("borrowed-incoming/source-target"));
                }
                incoming.insert(target.call_site().clone());
            }
            if source.borrowed_call_actuals_v1(row)?.is_none() {
                return Err(fault("borrowed-incoming/actuals-missing"));
            }
            original.insert(row.call_site().clone());
            let mut callers = program
                .functions()
                .iter()
                .enumerate()
                .filter(|(_, function)| function.name() == caller.signature.name);
            let caller_index = callers
                .next()
                .ok_or_else(|| fault("borrowed-incoming/caller-missing"))?
                .0;
            if callers.next().is_some() {
                return Err(fault("borrowed-incoming/caller-duplicate"));
            }
            let coordinate = (caller_index, block, index);
            let published_call = physical
                .get(&coordinate)
                .ok_or_else(|| fault("borrowed-incoming/coordinate-mismatch"))?;
            corroborate_final_call(published_call.0, *published_call.1, caller, (block, index))?;
            if !witnessed.insert(coordinate) {
                return Err(fault("borrowed-incoming/coordinate-mismatch"));
            }
            Ok(())
        },
    )?;
    if owners.len() != callees.len()
        || incoming != original
        || witnessed != physical.keys().copied().collect()
    {
        return Err(fault("borrowed-incoming/coverage-mismatch"));
    }
    Ok(())
}

/// Exact consumer boundary: a coordinate alone cannot prove a published Call.
pub(in crate::mir::compiler::normal_default_pipeline::published_backend_view) fn corroborate_final_call(
    published: &crate::mir::definitions::MirCall,
    published_result: InvokeCallResultKind,
    caller: &crate::mir::MirFunction,
    (block, index): (crate::mir::BasicBlockId, usize),
) -> Result<(), String> {
    let actual = caller
        .blocks
        .get(&block)
        .and_then(|block| block.all_instructions().nth(index))
        .ok_or_else(|| fault("borrowed-incoming/actual-coordinate-missing"))?;
    if !matches!(actual, MirInstruction::Invoke {
        operation: InvokeOperation::Call { call, result }, ..
    } if call == published && *result == published_result)
    {
        return Err(fault("borrowed-incoming/actual-call-mismatch"));
    }
    Ok(())
}
