//! Bidirectional final Call coverage, demanded before ordinary result filtering.
//! Metadata selects inspection; original source and finished packets prove it.
use super::*;
use crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1 as Carrier;
use crate::mir::MirModule;

#[path = "borrowed_call_uses.rs"]
mod borrowed_uses;
#[cfg(test)]
pub(in crate::mir::compiler::normal_default_pipeline::published_backend_view) use borrowed_uses::projection_fixture;

pub(in crate::mir::compiler::normal_default_pipeline::published_backend_view) fn verify_borrowed_call_incoming<
    'module,
>(
    program: &PublishedLifecyclePhysicalProgramV1<'module>,
    module: &MirModule,
) -> Result<
    (
        BTreeMap<
            (usize, crate::mir::BasicBlockId, usize),
            &'module [PreparedBorrowedFormalActualV1],
        >,
        BTreeMap<u32, BTreeMap<u32, u32>>,
    ),
    String,
> {
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
        if !matches!(
            key.namespace(),
            hakorune_mir_defs::SameModuleCallableNamespaceV1::InstanceBoxMethod
                | hakorune_mir_defs::SameModuleCallableNamespaceV1::StaticBoxMethod
        ) || (key.namespace()
            == hakorune_mir_defs::SameModuleCallableNamespaceV1::StaticBoxMethod
            && !matches!(
                function.role(),
                PublishedLifecyclePhysicalFunctionRoleV1::OrdinaryI64 { .. }
            ))
            || carriers.len() != function.params().len()
            || callees.insert(key, function).is_some()
        {
            return Err(fault("borrowed-incoming/callee-metadata"));
        }
    }
    let Some(source) = program.handoff().root_source() else {
        return if callees.is_empty() {
            Ok((BTreeMap::new(), BTreeMap::new()))
        } else {
            Err(fault("borrowed-incoming/source-missing"))
        };
    };
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
                let callee = match &call.callee {
                    Callee::SameModuleInstance { key, .. } => callees.get(key).copied(),
                    Callee::Global(global) => {
                        let mut matching = Vec::new();
                        for (key, function) in &callees {
                            if key.namespace()
                                == hakorune_mir_defs::SameModuleCallableNamespaceV1::StaticBoxMethod
                                && &key.canonical_global_target_v1()? == global
                            {
                                let symbol = module
                                    .canonical_callable_definition_symbol(key)
                                    .ok_or_else(|| {
                                        fault("borrowed-incoming/static-definition-missing")
                                    })?;
                                if symbol != function.name() {
                                    return Err(fault("borrowed-incoming/static-definition-drift"));
                                }
                                matching.push(*function);
                            }
                        }
                        if matching.len() > 1 {
                            return Err(fault("borrowed-incoming/static-definition-duplicate"));
                        }
                        matching.pop()
                    }
                    _ => None,
                };
                let Some(callee) = callee else {
                    continue;
                };
                // The callee's published role names the one result contract
                // its call sites may carry — scalar transport or the
                // checked-release object transport. `Map` roles remain
                // unadmitted on the borrowed lane.
                let expected = match callee.role() {
                    PublishedLifecyclePhysicalFunctionRoleV1::OrdinaryI64 { .. } => {
                        InvokeCallResultKind::I64
                    }
                    PublishedLifecyclePhysicalFunctionRoleV1::OrdinaryHandle { .. } => {
                        InvokeCallResultKind::Handle
                    }
                    PublishedLifecyclePhysicalFunctionRoleV1::OrdinaryNullableHandle { .. } => {
                        InvokeCallResultKind::NullableHandle
                    }
                    _ => return Err(fault("borrowed-incoming/result-mismatch")),
                };
                if *result != expected {
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
    let mut uses = borrowed_uses::BorrowedCallUses::default();
    let mut witnessed = BTreeMap::new();
    let mut owners = BTreeMap::new();
    let mut incoming = BTreeSet::new();
    let mut original = BTreeSet::new();
    source.visit_finalized_lexical_call_nodes_v1(
        module,
        |_, _, packet, arguments, caller, (block, index), copies| {
            let row = packet.original_source();
            let selected_source = arguments.iter().any(|argument| matches!(argument,
                crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1::BorrowedActual { .. }));
            let callee = callees.get(row.target());
            if !selected_source && callee.is_none() {
                return Ok(());
            }
            let Some(callee) = callee else {
                source.borrowed_packet_actuals_v1(packet)?
                    .ok_or_else(|| fault("borrowed-incoming/actuals-missing"))?;
                return Err(fault("borrowed-incoming/callee-carrier-missing"));
            };
            let first_entry = !owners.contains_key(row.target());
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
                let target = target?;
                if target.target() != row.target() || target.callee_owner() != row.callee_owner() {
                    return Err(fault("borrowed-incoming/source-target"));
                }
                incoming.insert(target.call_site().clone());
            }
            let actuals = source.borrowed_packet_actuals_v1(packet)?
                .ok_or_else(|| fault("borrowed-incoming/actuals-missing"))?;
            // Source use admissions are function obligations; incoming actuals
            // and their entry correspondence remain checked on every call.
            if first_entry {
                uses.entry(source, row.callee_owner(), callee_function)?;
            }
            uses.call(caller, (block, index), actuals, copies)?;
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
            if witnessed.insert(coordinate, actuals).is_some() {
                return Err(fault("borrowed-incoming/coordinate-mismatch"));
            }
            Ok(())
        },
    )?;
    if owners.len() != callees.len()
        || incoming != original
        || witnessed.keys().copied().collect::<BTreeSet<_>>() != physical.keys().copied().collect()
    {
        return Err(fault("borrowed-incoming/coverage-mismatch"));
    }
    let object_views = uses.finish(program, module)?;
    Ok((witnessed, object_views))
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
