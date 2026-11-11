#[path = "verify_transfers.rs"]
mod transfers;
use super::classify::classify;
use super::error::OwnershipSsaErrorV1;
use super::model::{
    collect_ownership_operations, FunctionResultOwnershipV1, MirOwnershipKindV1,
    OwnershipDispositionV1, OwnershipFunctionAbiV1, VerifiedOwnershipSsaV1,
};
use crate::mir::{BasicBlockId, Callee, MirFunction, MirInstruction, ValueId};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use transfers::*;

type LiveSet = BTreeSet<ValueId>;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ObjectStage {
    Unpublished,
    Published,
    BirthFaulted,
}
type ObjectStates = BTreeMap<ValueId, (hakorune_mir_defs::CanonicalObjectIdV1, ObjectStage)>;
#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct SharedState {
    handles: BTreeMap<ValueId, (crate::mir::instruction::SharedDescriptorV1, ValueId)>,
    borrows: BTreeMap<ValueId, ValueId>,
}

pub(super) fn verify(
    function: &MirFunction,
    abi: &OwnershipFunctionAbiV1,
    module: Option<&crate::mir::MirModule>,
) -> Result<VerifiedOwnershipSsaV1, OwnershipSsaErrorV1> {
    let geometry = crate::mir::verification::geometry::FunctionVerificationGeometry::new(function);
    let shared_blocks: Vec<_> = function
        .blocks
        .iter()
        .filter_map(|(id, block)| match &block.terminator {
            Some(MirInstruction::Invoke {
                operation: crate::mir::instruction::InvokeOperation::Shared(operation),
                normal_landing,
                fault_landing,
                ..
            }) => Some((
                *id,
                *normal_landing,
                *fault_landing,
                operation.normal_result_kind().is_some(),
            )),
            _ => None,
        })
        .collect();
    if !shared_blocks.is_empty() {
        // These structural checks are independent of ambient verifier flags.
        if !geometry.ssa_errors().is_empty() || !geometry.dominance_errors().is_empty() {
            return Err(OwnershipSsaErrorV1::InvalidSharedCfg {
                block: function.entry_block,
            });
        }
        for (origin, normal, fault, has_result) in shared_blocks {
            if normal == fault
                || !function.blocks.contains_key(&normal)
                || !function.blocks.contains_key(&fault)
                || (has_result
                    && (normal == function.entry_block
                        || normal == origin
                        || function
                            .blocks
                            .iter()
                            .filter(|(_, block)| block.successors.contains(&normal))
                            .map(|(id, _)| *id)
                            .collect::<Vec<_>>()
                            != vec![origin]))
            {
                return Err(OwnershipSsaErrorV1::InvalidSharedCfg { block: origin });
            }
        }
    }
    // Only the exact immutable function in the source module can delegate
    // managed Text to the mandatory final module String lifetime verifier.
    // Standalone functions and foreign/cloned module images stay fail-closed.
    let managed_string_checked_in_module = module
        .and_then(|module| module.functions.get(&function.signature.name))
        .is_some_and(|same| std::ptr::eq(same, function));
    let mut kinds = classify(function, abi, managed_string_checked_in_module)?;
    let reachable = geometry.reachable();
    for block in function.blocks.keys().copied() {
        // Catch setup contributes reachability, not a managed ownership entry
        // state. Until that state has a contract, retain the old fail-closed
        // boundary for handlers outside the ordinary execution CFG.
        if !reachable.contains(&block) || !geometry.dominators().is_reachable(block) {
            return Err(OwnershipSsaErrorV1::UnreachableBlock { block });
        }
    }
    reject_edge_arguments(function)?;
    verify_phi_predecessors(function, geometry.predecessors())?;
    verify_instruction_kinds(function, &kinds, abi)?;

    let initial = function
        .params
        .iter()
        .copied()
        .filter(|value| kinds.get(value) == Some(&MirOwnershipKindV1::Owned))
        .collect::<LiveSet>();
    let mut entry_objects = ObjectStates::new();
    if abi.owner().is_source() {
        for value in &initial {
            let object = abi.owned_parameters().get(value).ok_or(
                OwnershipSsaErrorV1::InvalidObjectLifecycle {
                    block: function.entry_block,
                    value: *value,
                },
            )?;
            entry_objects.insert(*value, (*object, ObjectStage::Published));
        }
    }
    for value in abi.owned_parameters().keys() {
        if !abi.owner().is_source()
            || !initial.contains(value)
            || !function.params.contains(value)
            || abi.aliases().contains_key(value)
        {
            return Err(OwnershipSsaErrorV1::InvalidObjectLifecycle {
                block: function.entry_block,
                value: *value,
            });
        }
    }
    let mut entries = BTreeMap::from([(
        function.entry_block,
        (initial, entry_objects, SharedState::default()),
    )]);
    let mut queue = VecDeque::from([function.entry_block]);

    while let Some(block_id) = queue.pop_front() {
        let (mut live, mut objects, mut shared) = entries[&block_id].clone();
        let block = &function.blocks[&block_id];
        for instruction in &block.instructions {
            if abi.owner().is_source()
                && matches!(
                    instruction,
                    MirInstruction::CopyOwned { .. } | MirInstruction::DestroyOwned { .. }
                )
            {
                return Err(OwnershipSsaErrorV1::UnsupportedManagedInstruction { block: block.id });
            }
            process_instruction(block_id, instruction, &kinds, abi, &mut live)?;
            if let (true, MirInstruction::ObjectFieldGet { base, field, .. }) =
                (abi.owner().is_source(), instruction)
            {
                let value = *abi.aliases().get(base).unwrap_or(base);
                let valid = match kinds.get(&value) {
                    Some(MirOwnershipKindV1::Owned) => {
                        objects.get(&value) == Some(&(field.object(), ObjectStage::Published))
                    }
                    Some(MirOwnershipKindV1::Borrowed) => true,
                    Some(MirOwnershipKindV1::None) if module.is_some() => {
                        let (root, object, views) = super::classify::local_field_borrow(
                            &geometry,
                            module.unwrap(),
                            block_id,
                            value,
                            *field,
                        )?;
                        let root = *abi.aliases().get(&root).unwrap_or(&root);
                        if kinds.get(&root) != Some(&MirOwnershipKindV1::Owned)
                            || !live.contains(&root)
                            || objects.get(&root) != Some(&(object, ObjectStage::Published))
                        {
                            return Err(OwnershipSsaErrorV1::InvalidObjectLifecycle {
                                block: block_id,
                                value,
                            });
                        }
                        for view in views {
                            kinds.insert(view, MirOwnershipKindV1::Borrowed);
                        }
                        true
                    }
                    _ => false,
                };
                if !valid {
                    return Err(OwnershipSsaErrorV1::InvalidObjectLifecycle {
                        block: block_id,
                        value,
                    });
                }
            }
            if abi.owner().is_source() {
                if let MirInstruction::InvokeNormalResult { invoke_block, dst } = instruction {
                    if let Some(MirInstruction::Invoke {
                        operation: crate::mir::instruction::InvokeOperation::NewBox { object },
                        ..
                    }) = function
                        .blocks
                        .get(invoke_block)
                        .and_then(|block| block.terminator.as_ref())
                    {
                        let stage = if abi.initially_published().contains(dst) {
                            ObjectStage::Published
                        } else {
                            ObjectStage::Unpublished
                        };
                        if objects.insert(*dst, (*object, stage)).is_some() {
                            return Err(OwnershipSsaErrorV1::InvalidObjectLifecycle {
                                block: block_id,
                                value: *dst,
                            });
                        }
                    }
                }
            }
        }
        let mut birth = None;
        if let Some(terminator) = &block.terminator {
            if let MirInstruction::Return { value: Some(value) } = terminator {
                let token = *abi.aliases().get(value).unwrap_or(value);
                if shared.handles.contains_key(&token) || shared.borrows.contains_key(&token) {
                    return Err(OwnershipSsaErrorV1::InvalidSharedLifecycle {
                        block: block_id,
                        value: token,
                    });
                }
            }
            if abi.owner().is_source() {
                if let MirInstruction::Invoke {
                    operation,
                    normal_landing,
                    fault_landing,
                    ..
                } = terminator
                {
                    use crate::mir::instruction::InvokeOperation;
                    let token = |value: &ValueId| *abi.aliases().get(value).unwrap_or(value);
                    match operation {
                        InvokeOperation::Birth(row) => {
                            if normal_landing == fault_landing
                                || !function.blocks.contains_key(normal_landing)
                                || !function.blocks.contains_key(fault_landing)
                            {
                                return Err(OwnershipSsaErrorV1::InvalidObjectLifecycle {
                                    block: block_id,
                                    value: row.receiver,
                                });
                            }
                            let value = token(&row.receiver);
                            if objects
                                .get(&value)
                                .is_none_or(|(_, stage)| *stage != ObjectStage::Unpublished)
                            {
                                return Err(OwnershipSsaErrorV1::InvalidObjectLifecycle {
                                    block: block_id,
                                    value,
                                });
                            }
                            let mut takes = BTreeSet::new();
                            for (ordinal, argument) in row.arguments.iter().enumerate() {
                                let Some(actual) = argument.value() else {
                                    continue;
                                };
                                let actual = token(&actual);
                                if kinds.get(&actual) != Some(&MirOwnershipKindV1::Owned) {
                                    continue;
                                }
                                if !abi.birth_takes(&row.key, ordinal)
                                    || actual == value
                                    || !takes.insert(actual)
                                    || objects
                                        .get(&actual)
                                        .is_none_or(|(_, stage)| *stage != ObjectStage::Published)
                                {
                                    return Err(OwnershipSsaErrorV1::InvalidObjectLifecycle {
                                        block: block_id,
                                        value: actual,
                                    });
                                }
                            }
                            birth = Some((value, *normal_landing, takes));
                        }
                        InvokeOperation::HomeRelease { object, value }
                        | InvokeOperation::ObjectEndIfOwned { object, value, .. }
                        | InvokeOperation::ReclaimUnpublished { object, value } => {
                            let value = token(value);
                            let state = objects.remove(&value);
                            let valid = match (operation, state) {
                                (
                                    InvokeOperation::ReclaimUnpublished { .. },
                                    Some((
                                        actual,
                                        ObjectStage::Unpublished | ObjectStage::BirthFaulted,
                                    )),
                                ) => actual == *object,
                                (
                                    InvokeOperation::HomeRelease { .. }
                                    | InvokeOperation::ObjectEndIfOwned { .. },
                                    Some((actual, ObjectStage::Published)),
                                ) => actual == *object,
                                _ => false,
                            };
                            if !valid {
                                return Err(OwnershipSsaErrorV1::InvalidObjectLifecycle {
                                    block: block_id,
                                    value,
                                });
                            }
                        }
                        InvokeOperation::OwnedObjectFieldRelease { field, base, .. } => {
                            let value = token(base);
                            if objects.get(&value)
                                != Some(&(field.object(), ObjectStage::Published))
                            {
                                return Err(OwnershipSsaErrorV1::InvalidObjectLifecycle {
                                    block: block_id,
                                    value,
                                });
                            }
                        }
                        InvokeOperation::FieldGet { field, base } => {
                            let value = token(base);
                            if let Some(handle) = shared.borrows.remove(&value) {
                                let valid = shared
                                    .handles
                                    .get(&handle)
                                    .is_some_and(|(desc, _)| desc.payload == field.object());
                                let uses = function
                                    .blocks
                                    .values()
                                    .flat_map(|b| b.all_instructions())
                                    .flat_map(|i| i.used_values())
                                    .filter(|v| *v == value)
                                    .count();
                                if !valid || uses != 1 {
                                    return Err(OwnershipSsaErrorV1::InvalidSharedLifecycle {
                                        block: block_id,
                                        value,
                                    });
                                }
                            } else if objects.get(&value)
                                != Some(&(field.object(), ObjectStage::Published))
                            {
                                return Err(OwnershipSsaErrorV1::InvalidObjectLifecycle {
                                    block: block_id,
                                    value,
                                });
                            }
                        }
                        InvokeOperation::Shared(operation) => {
                            use crate::mir::instruction::SharedInvokeOperation as Shared;
                            if !abi.admits_shared_descriptor(operation.descriptor()) {
                                return Err(OwnershipSsaErrorV1::InvalidSharedLifecycle {
                                    block: block_id,
                                    value: operation.used_values()[0],
                                });
                            }
                            match operation {
                                Shared::New {
                                    descriptor,
                                    payload,
                                } => {
                                    let value = token(payload);
                                    if objects.get(&value)
                                        != Some(&(descriptor.payload, ObjectStage::Published))
                                    {
                                        return Err(OwnershipSsaErrorV1::InvalidSharedLifecycle {
                                            block: block_id,
                                            value,
                                        });
                                    }
                                }
                                Shared::Clone { descriptor, handle }
                                | Shared::Borrow { descriptor, handle }
                                | Shared::Release { descriptor, handle } => {
                                    let value = token(handle);
                                    if shared
                                        .handles
                                        .get(&value)
                                        .is_none_or(|(actual, _)| actual != descriptor)
                                    {
                                        return Err(OwnershipSsaErrorV1::InvalidSharedLifecycle {
                                            block: block_id,
                                            value,
                                        });
                                    }
                                    if matches!(operation, Shared::Release { .. }) {
                                        if shared.borrows.values().any(|support| *support == value)
                                        {
                                            return Err(
                                                OwnershipSsaErrorV1::InvalidSharedLifecycle {
                                                    block: block_id,
                                                    value,
                                                },
                                            );
                                        }
                                        shared.handles.remove(&value);
                                    }
                                }
                            }
                        }
                        InvokeOperation::Call { call, .. } => {
                            let receiver =
                                if let Callee::SameModuleInstance { receiver, .. } = &call.callee {
                                    let value = token(receiver);
                                    if objects
                                        .get(&value)
                                        .is_none_or(|(_, stage)| *stage != ObjectStage::Published)
                                    {
                                        return Err(OwnershipSsaErrorV1::InvalidObjectLifecycle {
                                            block: block_id,
                                            value,
                                        });
                                    }
                                    Some(value)
                                } else {
                                    None
                                };
                            if let Some((arity, takes)) = abi.call_takes(&call.callee) {
                                if call.args.len() != arity as usize
                                    || takes.keys().any(|ordinal| *ordinal >= arity)
                                {
                                    return Err(
                                        OwnershipSsaErrorV1::ManagedCallOwnershipUnsupported {
                                            block: block_id,
                                        },
                                    );
                                }
                                let mut transferred = BTreeSet::new();
                                for (ordinal, object) in takes {
                                    let value = token(&call.args[*ordinal as usize]);
                                    if receiver == Some(value)
                                        || !transferred.insert(value)
                                        || kinds.get(&value) != Some(&MirOwnershipKindV1::Owned)
                                        || objects.remove(&value)
                                            != Some((*object, ObjectStage::Published))
                                    {
                                        return Err(OwnershipSsaErrorV1::InvalidObjectLifecycle {
                                            block: block_id,
                                            value,
                                        });
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            process_terminator(block_id, terminator, abi, &kinds, &mut live)?;
        }

        if block.successors.is_empty() {
            if !live.is_empty() {
                return Err(OwnershipSsaErrorV1::MissingDispositionAtExit {
                    block: block_id,
                    values: live.into_iter().collect::<Vec<_>>().into_boxed_slice(),
                });
            }
            continue;
        }

        for successor in block.successors.iter().copied() {
            let mut before_phi = live.clone();
            let mut edge_objects = objects.clone();
            let mut edge_shared = shared.clone();
            if let Some(MirInstruction::Invoke {
                operation: crate::mir::instruction::InvokeOperation::Shared(operation),
                normal_landing,
                ..
            }) = block.terminator.as_ref()
            {
                if successor == *normal_landing {
                    use crate::mir::instruction::SharedInvokeOperation as Shared;
                    if !matches!(operation, Shared::Release { .. }) {
                        let results: Vec<_> = function.blocks[normal_landing]
                            .instructions
                            .iter()
                            .filter_map(|row| match row {
                                MirInstruction::InvokeNormalResult { invoke_block, dst }
                                    if *invoke_block == block_id =>
                                {
                                    Some(*dst)
                                }
                                _ => None,
                            })
                            .collect();
                        if results.len() != 1 {
                            return Err(OwnershipSsaErrorV1::InvalidAcquisition {
                                block: *normal_landing,
                            });
                        }
                        let dst = results[0];
                        match operation {
                            Shared::New {
                                descriptor,
                                payload,
                            } => {
                                let value = *abi.aliases().get(payload).unwrap_or(payload);
                                if !before_phi.remove(&value)
                                    || edge_objects.remove(&value).is_none()
                                {
                                    return Err(OwnershipSsaErrorV1::InvalidSharedLifecycle {
                                        block: block_id,
                                        value,
                                    });
                                }
                                edge_shared.handles.insert(dst, (descriptor.clone(), dst));
                            }
                            Shared::Clone { handle, .. } => {
                                let value = *abi.aliases().get(handle).unwrap_or(handle);
                                let row = edge_shared.handles.get(&value).cloned().ok_or(
                                    OwnershipSsaErrorV1::InvalidSharedLifecycle {
                                        block: block_id,
                                        value,
                                    },
                                )?;
                                edge_shared.handles.insert(dst, row);
                            }
                            Shared::Borrow { handle, .. } => {
                                let value = *abi.aliases().get(handle).unwrap_or(handle);
                                edge_shared.borrows.insert(dst, value);
                            }
                            Shared::Release { .. } => unreachable!(),
                        }
                    }
                }
            }
            if abi.owner().is_source() {
                transfer_shared_phi_edge(
                    function,
                    block_id,
                    successor,
                    &kinds,
                    abi,
                    &mut edge_shared,
                )?;
            }
            if let Some((value, normal, takes)) = &birth {
                if successor == *normal {
                    for actual in takes {
                        if !before_phi.remove(actual) || edge_objects.remove(actual).is_none() {
                            return Err(OwnershipSsaErrorV1::OwnedUseAfterConsume {
                                block: block_id,
                                value: *actual,
                            });
                        }
                    }
                }
                edge_objects.get_mut(value).expect("checked Birth token").1 =
                    if successor == *normal {
                        ObjectStage::Published
                    } else {
                        ObjectStage::BirthFaulted
                    };
            }
            let edge_live =
                transfer_phi_edge(function, block_id, successor, &kinds, abi, &before_phi)?;
            let edge_state = (edge_live, edge_objects, edge_shared);
            match entries.get(&successor) {
                Some(existing) if existing != &edge_state => {
                    return Err(OwnershipSsaErrorV1::LiveSetMismatch { block: successor });
                }
                Some(_) => {}
                None => {
                    entries.insert(successor, edge_state);
                    queue.push_back(successor);
                }
            }
        }
    }

    let dispositions = collect_dispositions(function, &kinds);
    let operations = collect_ownership_operations(function);
    Ok(VerifiedOwnershipSsaV1::new(
        abi.clone(),
        kinds,
        dispositions,
        operations,
    ))
}
