use super::classify::classify;
use super::error::OwnershipSsaErrorV1;
use super::model::{
    collect_ownership_operations, FunctionResultOwnershipV1, MirOwnershipKindV1,
    OwnershipDispositionV1, OwnershipFunctionAbiV1, VerifiedOwnershipSsaV1,
};
use crate::mir::{BasicBlockId, Callee, MirFunction, MirInstruction, ValueId};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

type LiveSet = BTreeSet<ValueId>;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ObjectStage { Unpublished, Published, BirthFaulted }
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
    let shared_blocks: Vec<_> = function.blocks.iter().filter_map(|(id, block)| match &block.terminator {
        Some(MirInstruction::Invoke { operation: crate::mir::instruction::InvokeOperation::Shared(operation), normal_landing, fault_landing, .. }) => Some((*id, *normal_landing, *fault_landing, operation.normal_result_kind().is_some())),
        _ => None,
    }).collect();
    if !shared_blocks.is_empty() {
        // These structural checks are independent of ambient verifier flags.
        let definitions = crate::mir::verification::utils::compute_def_blocks(function);
        let dominators = crate::mir::verification::utils::compute_dominators(function);
        if crate::mir::verification::ssa::check_ssa_form(function).is_err()
            || crate::mir::verification::dom::check_dominance_with_policy(function, &definitions, &dominators, false).is_err() {
            return Err(OwnershipSsaErrorV1::InvalidSharedCfg { block: function.entry_block });
        }
        for (origin, normal, fault, has_result) in shared_blocks {
            if normal == fault || !function.blocks.contains_key(&normal) || !function.blocks.contains_key(&fault)
                || (has_result && (normal == function.entry_block || normal == origin
                    || function.blocks.iter().filter(|(_, block)| block.successors.contains(&normal)).map(|(id, _)| *id).collect::<Vec<_>>() != vec![origin])) {
                return Err(OwnershipSsaErrorV1::InvalidSharedCfg { block: origin });
            }
        }
    }
    let mut kinds = classify(function, abi)?;
    let reachable = reachable_blocks(function);
    for block in function.blocks.keys().copied() {
        if !reachable.contains(&block) {
            return Err(OwnershipSsaErrorV1::UnreachableBlock { block });
        }
    }
    reject_edge_arguments(function)?;
    verify_phi_predecessors(function)?;
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
                OwnershipSsaErrorV1::InvalidObjectLifecycle { block: function.entry_block, value: *value })?;
            entry_objects.insert(*value, (*object, ObjectStage::Published));
        }
    }
    for value in abi.owned_parameters().keys() {
        if !abi.owner().is_source() || !initial.contains(value)
            || !function.params.contains(value) || abi.aliases().contains_key(value) {
            return Err(OwnershipSsaErrorV1::InvalidObjectLifecycle { block: function.entry_block, value: *value });
        }
    }
    let mut entries = BTreeMap::from([(function.entry_block, (initial, entry_objects, SharedState::default()))]);
    let mut queue = VecDeque::from([function.entry_block]);

    while let Some(block_id) = queue.pop_front() {
        let (mut live, mut objects, mut shared) = entries[&block_id].clone();
        let block = &function.blocks[&block_id];
        for instruction in &block.instructions {
            if abi.owner().is_source() && matches!(instruction, MirInstruction::CopyOwned { .. } | MirInstruction::DestroyOwned { .. }) {
                return Err(OwnershipSsaErrorV1::UnsupportedManagedInstruction { block: block.id });
            }
            process_instruction(block_id, instruction, &kinds, abi, &mut live)?;
            if let (true, MirInstruction::ObjectFieldGet { base, field, .. }) = (abi.owner().is_source(), instruction) {
                let value = *abi.aliases().get(base).unwrap_or(base);
                let valid = match kinds.get(&value) {
                    Some(MirOwnershipKindV1::Owned) => objects.get(&value) == Some(&(field.object(), ObjectStage::Published)),
                    Some(MirOwnershipKindV1::Borrowed) => true,
                    Some(MirOwnershipKindV1::None) if module.is_some() => {
                        let (root, object, views) = super::classify::local_field_borrow(
                            function, module.unwrap(), block_id, value, *field)?;
                        let root = *abi.aliases().get(&root).unwrap_or(&root);
                        if kinds.get(&root) != Some(&MirOwnershipKindV1::Owned)
                            || !live.contains(&root)
                            || objects.get(&root) != Some(&(object, ObjectStage::Published)) {
                            return Err(OwnershipSsaErrorV1::InvalidObjectLifecycle { block: block_id, value });
                        }
                        for view in views { kinds.insert(view, MirOwnershipKindV1::Borrowed); }
                        true
                    }
                    _ => false,
                };
                if !valid { return Err(OwnershipSsaErrorV1::InvalidObjectLifecycle { block: block_id, value }); }
            }
            if abi.owner().is_source() {
                if let MirInstruction::InvokeNormalResult { invoke_block, dst } = instruction {
                    if let Some(MirInstruction::Invoke { operation: crate::mir::instruction::InvokeOperation::NewBox { object }, .. }) =
                        function.blocks.get(invoke_block).and_then(|block| block.terminator.as_ref()) {
                        let stage = if abi.initially_published().contains(dst) { ObjectStage::Published } else { ObjectStage::Unpublished };
                        if objects.insert(*dst, (*object, stage)).is_some() {
                            return Err(OwnershipSsaErrorV1::InvalidObjectLifecycle { block: block_id, value: *dst });
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
                    return Err(OwnershipSsaErrorV1::InvalidSharedLifecycle { block: block_id, value: token });
                }
            }
            if abi.owner().is_source() {
                if let MirInstruction::Invoke { operation, normal_landing, fault_landing, .. } = terminator {
                    use crate::mir::instruction::InvokeOperation;
                    let token = |value: &ValueId| *abi.aliases().get(value).unwrap_or(value);
                    match operation {
                        InvokeOperation::Birth(row) => {
                            if normal_landing == fault_landing || !function.blocks.contains_key(normal_landing)
                                || !function.blocks.contains_key(fault_landing) {
                                return Err(OwnershipSsaErrorV1::InvalidObjectLifecycle { block: block_id, value: row.receiver });
                            }
                            let value = token(&row.receiver);
                            if objects.get(&value).is_none_or(|(_, stage)| *stage != ObjectStage::Unpublished) { return Err(OwnershipSsaErrorV1::InvalidObjectLifecycle { block: block_id, value }); }
                            let mut takes = BTreeSet::new();
                            for (ordinal, argument) in row.arguments.iter().enumerate() {
                                let Some(actual) = argument.value() else { continue; };
                                let actual = token(&actual);
                                if kinds.get(&actual) != Some(&MirOwnershipKindV1::Owned) { continue; }
                                if !abi.birth_takes(&row.key, ordinal) || actual == value || !takes.insert(actual)
                                    || objects.get(&actual).is_none_or(|(_, stage)| *stage != ObjectStage::Published) {
                                    return Err(OwnershipSsaErrorV1::InvalidObjectLifecycle { block: block_id, value: actual });
                                }
                            }
                            birth = Some((value, *normal_landing, takes));
                        }
                        InvokeOperation::HomeRelease { object, value } | InvokeOperation::ObjectEndIfOwned { object, value, .. }
                        | InvokeOperation::ReclaimUnpublished { object, value } => {
                            let value = token(value);
                            let state = objects.remove(&value);
                            let valid = match (operation, state) {
                                (InvokeOperation::ReclaimUnpublished { .. }, Some((actual, ObjectStage::Unpublished | ObjectStage::BirthFaulted))) => actual == *object,
                                (InvokeOperation::HomeRelease { .. } | InvokeOperation::ObjectEndIfOwned { .. }, Some((actual, ObjectStage::Published))) => actual == *object,
                                _ => false,
                            };
                            if !valid {
                                return Err(OwnershipSsaErrorV1::InvalidObjectLifecycle { block: block_id, value });
                            }
                        }
                        InvokeOperation::OwnedObjectFieldRelease { field, base, .. } => {
                            let value = token(base);
                            if objects.get(&value) != Some(&(field.object(), ObjectStage::Published)) {
                                return Err(OwnershipSsaErrorV1::InvalidObjectLifecycle { block: block_id, value });
                            }
                        }
                        InvokeOperation::FieldGet { field, base } => {
                            let value = token(base);
                            if let Some(handle) = shared.borrows.remove(&value) {
                                let valid = shared.handles.get(&handle).is_some_and(|(desc, _)| desc.payload == field.object());
                                let uses = function.blocks.values().flat_map(|b| b.all_instructions())
                                    .flat_map(|i| i.used_values()).filter(|v| *v == value).count();
                                if !valid || uses != 1 { return Err(OwnershipSsaErrorV1::InvalidSharedLifecycle { block: block_id, value }); }
                            } else if objects.get(&value) != Some(&(field.object(), ObjectStage::Published)) {
                                return Err(OwnershipSsaErrorV1::InvalidObjectLifecycle { block: block_id, value });
                            }
                        }
                        InvokeOperation::Shared(operation) => {
                            use crate::mir::instruction::SharedInvokeOperation as Shared;
                            if !abi.admits_shared_descriptor(operation.descriptor()) {
                                return Err(OwnershipSsaErrorV1::InvalidSharedLifecycle { block: block_id, value: operation.used_values()[0] });
                            }
                            match operation {
                                Shared::New { descriptor, payload } => {
                                    let value = token(payload);
                                    if objects.get(&value) != Some(&(descriptor.payload, ObjectStage::Published)) {
                                        return Err(OwnershipSsaErrorV1::InvalidSharedLifecycle { block: block_id, value });
                                    }
                                }
                                Shared::Clone { descriptor, handle } | Shared::Borrow { descriptor, handle } | Shared::Release { descriptor, handle } => {
                                    let value = token(handle);
                                    if shared.handles.get(&value).is_none_or(|(actual, _)| actual != descriptor) {
                                        return Err(OwnershipSsaErrorV1::InvalidSharedLifecycle { block: block_id, value });
                                    }
                                    if matches!(operation, Shared::Release { .. }) {
                                        if shared.borrows.values().any(|support| *support == value) {
                                            return Err(OwnershipSsaErrorV1::InvalidSharedLifecycle { block: block_id, value });
                                        }
                                        shared.handles.remove(&value);
                                    }
                                }
                            }
                        }
                        InvokeOperation::Call { call, .. } => {
                            let receiver = if let Callee::SameModuleInstance { receiver, .. } = &call.callee {
                                let value = token(receiver);
                                if objects.get(&value).is_none_or(|(_, stage)| *stage != ObjectStage::Published) {
                                    return Err(OwnershipSsaErrorV1::InvalidObjectLifecycle { block: block_id, value });
                                }
                                Some(value)
                            } else { None };
                            if let Some((arity, takes)) = abi.call_takes(&call.callee) {
                                if call.args.len() != arity as usize || takes.keys().any(|ordinal| *ordinal >= arity) {
                                    return Err(OwnershipSsaErrorV1::ManagedCallOwnershipUnsupported { block: block_id });
                                }
                                let mut transferred = BTreeSet::new();
                                for (ordinal, object) in takes {
                                    let value = token(&call.args[*ordinal as usize]);
                                    if receiver == Some(value) || !transferred.insert(value)
                                        || kinds.get(&value) != Some(&MirOwnershipKindV1::Owned)
                                        || objects.remove(&value) != Some((*object, ObjectStage::Published)) {
                                        return Err(OwnershipSsaErrorV1::InvalidObjectLifecycle { block: block_id, value });
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
            if let Some(MirInstruction::Invoke { operation: crate::mir::instruction::InvokeOperation::Shared(operation), normal_landing, .. }) = block.terminator.as_ref() {
                if successor == *normal_landing {
                    use crate::mir::instruction::SharedInvokeOperation as Shared;
                    if !matches!(operation, Shared::Release { .. }) {
                        let results: Vec<_> = function.blocks[normal_landing].instructions.iter().filter_map(|row| match row {
                            MirInstruction::InvokeNormalResult { invoke_block, dst } if *invoke_block == block_id => Some(*dst), _ => None,
                        }).collect();
                        if results.len() != 1 { return Err(OwnershipSsaErrorV1::InvalidAcquisition { block: *normal_landing }); }
                        let dst = results[0];
                        match operation {
                            Shared::New { descriptor, payload } => {
                                let value = *abi.aliases().get(payload).unwrap_or(payload);
                                if !before_phi.remove(&value) || edge_objects.remove(&value).is_none() {
                                    return Err(OwnershipSsaErrorV1::InvalidSharedLifecycle { block: block_id, value });
                                }
                                edge_shared.handles.insert(dst, (descriptor.clone(), dst));
                            }
                            Shared::Clone { handle, .. } => {
                                let value = *abi.aliases().get(handle).unwrap_or(handle);
                                let row = edge_shared.handles.get(&value).cloned().ok_or(OwnershipSsaErrorV1::InvalidSharedLifecycle { block: block_id, value })?;
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
                transfer_shared_phi_edge(function, block_id, successor, &kinds, abi, &mut edge_shared)?;
            }
            if let Some((value, normal, takes)) = &birth {
                if successor == *normal {
                    for actual in takes {
                        if !before_phi.remove(actual) || edge_objects.remove(actual).is_none() {
                            return Err(OwnershipSsaErrorV1::OwnedUseAfterConsume { block: block_id, value: *actual });
                        }
                    }
                }
                edge_objects.get_mut(value).expect("checked Birth token").1 =
                    if successor == *normal { ObjectStage::Published } else { ObjectStage::BirthFaulted };
            }
            let edge_live = transfer_phi_edge(function, block_id, successor, &kinds, abi, &before_phi)?;
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

fn verify_instruction_kinds(
    function: &MirFunction,
    kinds: &BTreeMap<ValueId, MirOwnershipKindV1>,
    abi: &OwnershipFunctionAbiV1,
) -> Result<(), OwnershipSsaErrorV1> {
    for block in function.blocks.values() {
        for instruction in &block.instructions {
            match instruction {
                MirInstruction::CopyOwned { src, .. }
                    if kinds.get(src) == Some(&MirOwnershipKindV1::None) =>
                {
                    return Err(OwnershipSsaErrorV1::CopyOwnedSourceNotStrong {
                        block: block.id,
                        value: *src,
                    });
                }
                MirInstruction::DestroyOwned { value }
                    if kinds.get(value) != Some(&MirOwnershipKindV1::Owned) =>
                {
                    return Err(OwnershipSsaErrorV1::DestroyRequiresOwned {
                        block: block.id,
                        value: *value,
                    });
                }
                MirInstruction::Copy { src, .. }
                    if kinds.get(src) == Some(&MirOwnershipKindV1::Owned)
                        && !matches!(instruction, MirInstruction::Copy { dst, .. } if abi.aliases().get(dst) == Some(src)) =>
                {
                    return Err(OwnershipSsaErrorV1::CopyOnOwned {
                        block: block.id,
                        value: *src,
                    });
                }
                MirInstruction::LegacyCallV0 {
                    dst,
                    func,
                    callee,
                    args,
                    ..
                } => verify_call_ownership(block.id, *func, callee.as_ref(), args, *dst, kinds)?,
                _ => {}
            }
        }
    }
    Ok(())
}

fn verify_call_ownership(
    block: BasicBlockId,
    func: ValueId,
    callee: Option<&Callee>,
    args: &[ValueId],
    dst: Option<ValueId>,
    kinds: &BTreeMap<ValueId, MirOwnershipKindV1>,
) -> Result<(), OwnershipSsaErrorV1> {
    let target = match callee {
        None => Some(func),
        Some(Callee::Method {
            receiver: Some(receiver),
            ..
        })
        | Some(Callee::SameModuleInstance { receiver, .. })
        | Some(Callee::BirthConstructor { receiver, .. })
        | Some(Callee::Value(receiver)) => Some(*receiver),
        Some(
            Callee::Global(_)
            | Callee::Extern(_)
            | Callee::Constructor { .. }
            | Callee::Method { receiver: None, .. }
            | Callee::Closure { .. },
        ) => None,
    };

    let target_is_managed =
        target.is_some_and(|value| kinds.get(&value) != Some(&MirOwnershipKindV1::None));
    let args_are_managed = args
        .iter()
        .any(|value| kinds.get(value) != Some(&MirOwnershipKindV1::None));
    let destination_is_managed =
        dst.is_some_and(|value| kinds.get(&value) != Some(&MirOwnershipKindV1::None));
    if target_is_managed || args_are_managed || destination_is_managed {
        return Err(OwnershipSsaErrorV1::ManagedCallOwnershipUnsupported { block });
    }
    Ok(())
}

fn process_instruction(
    block: BasicBlockId,
    instruction: &MirInstruction,
    kinds: &BTreeMap<ValueId, MirOwnershipKindV1>,
    abi: &OwnershipFunctionAbiV1,
    live: &mut LiveSet,
) -> Result<(), OwnershipSsaErrorV1> {
    match instruction {
        MirInstruction::Phi { .. } => return Ok(()),
        MirInstruction::InvokeNormalResult { dst, .. } if kinds.get(dst) == Some(&MirOwnershipKindV1::Owned) => {
            if !live.insert(*dst) { return Err(OwnershipSsaErrorV1::OwnedDestinationAlreadyLive { block, value: *dst }); }
            return Ok(());
        }
        MirInstruction::Copy { dst, src } if abi.aliases().get(dst) == Some(src) => {
            require_live_if_owned(block, *src, kinds, live)?;
            return Ok(());
        }
        // A scalar read keeps its base; the exact object stage is checked
        // by the block walk, which owns the per-path object states.
        MirInstruction::ObjectFieldGet { base, .. } => {
            require_live_if_owned(block, *abi.aliases().get(base).unwrap_or(base), kinds, live)?;
            return Ok(());
        }
        MirInstruction::CopyOwned { dst, src } => {
            require_live_if_owned(block, *src, kinds, live)?;
            if !live.insert(*dst) {
                return Err(OwnershipSsaErrorV1::OwnedDestinationAlreadyLive {
                    block,
                    value: *dst,
                });
            }
            return Ok(());
        }
        MirInstruction::DestroyOwned { value } => {
            if !live.remove(value) {
                return Err(OwnershipSsaErrorV1::OwnedUseAfterConsume {
                    block,
                    value: *value,
                });
            }
            return Ok(());
        }
        _ => {}
    }
    if abi.owner().is_source() && instruction.used_values().into_iter().any(|value| {
        let token = *abi.aliases().get(&value).unwrap_or(&value);
        kinds.get(&token) != Some(&MirOwnershipKindV1::None)
    }) {
        return Err(OwnershipSsaErrorV1::UnsupportedManagedInstruction { block });
    }
    for value in instruction.used_values() {
        require_live_if_owned(block, *abi.aliases().get(&value).unwrap_or(&value), kinds, live)?;
    }
    Ok(())
}

fn process_terminator(
    block: BasicBlockId,
    terminator: &MirInstruction,
    abi: &OwnershipFunctionAbiV1,
    kinds: &BTreeMap<ValueId, MirOwnershipKindV1>,
    live: &mut LiveSet,
) -> Result<(), OwnershipSsaErrorV1> {
    if let MirInstruction::Return { value } = terminator {
        match (abi.result(), value) {
            (FunctionResultOwnershipV1::Owned, Some(value)) => match kinds.get(value) {
                Some(MirOwnershipKindV1::Owned) if live.remove(value) => {}
                Some(MirOwnershipKindV1::Borrowed) => {
                    return Err(OwnershipSsaErrorV1::BorrowedReturnForbidden {
                        block,
                        value: *value,
                    })
                }
                _ => return Err(OwnershipSsaErrorV1::ResultOwnershipMismatch { block }),
            },
            (FunctionResultOwnershipV1::None, None) => {}
            (FunctionResultOwnershipV1::None, Some(value)) if kinds.get(value) == Some(&MirOwnershipKindV1::None) => {}
            _ => return Err(OwnershipSsaErrorV1::ResultOwnershipMismatch { block }),
        }
        return Ok(());
    }
    for value in terminator.used_values() {
        require_live_if_owned(block, *abi.aliases().get(&value).unwrap_or(&value), kinds, live)?;
    }
    if abi.owner().is_source() && !matches!(terminator, MirInstruction::Invoke { .. })
        && terminator.used_values().into_iter().any(|value| kinds.get(&value) != Some(&MirOwnershipKindV1::None)) {
        return Err(OwnershipSsaErrorV1::UnsupportedManagedInstruction { block });
    }
    if let MirInstruction::Invoke { operation, .. } = terminator {
        use crate::mir::instruction::InvokeOperation;
        if !abi.owner().is_source() && matches!(operation, InvokeOperation::Shared(_)) {
            return Err(OwnershipSsaErrorV1::UnsupportedManagedInstruction { block });
        }
        if abi.owner().is_source() && terminator.used_values().into_iter().any(|value|
            kinds.get(&value) == Some(&MirOwnershipKindV1::Borrowed))
            && !matches!(operation, InvokeOperation::FieldGet { .. }) {
            return Err(OwnershipSsaErrorV1::UnsupportedManagedInstruction { block });
        }
        let has_managed_operand = terminator.used_values().into_iter().any(|value| {
            let token = *abi.aliases().get(&value).unwrap_or(&value);
            kinds.get(&token) == Some(&MirOwnershipKindV1::Owned)
        });
        let supported = match operation {
            InvokeOperation::Birth(birth) => birth.arguments.iter().enumerate().all(|(ordinal, argument)| {
                argument.value().is_none_or(|value| {
                    let token = *abi.aliases().get(&value).unwrap_or(&value);
                    kinds.get(&token) == Some(&MirOwnershipKindV1::None) ||
                        (abi.birth_takes(&birth.key, ordinal) && kinds.get(&token) == Some(&MirOwnershipKindV1::Owned))
                })
            }),
            InvokeOperation::HomeRelease { .. } | InvokeOperation::ObjectEndIfOwned { .. } | InvokeOperation::ReclaimUnpublished { .. }
            | InvokeOperation::FieldGet { .. } | InvokeOperation::OwnedObjectFieldRelease { .. } => true,
            InvokeOperation::Call { call, .. } => {
                if let Some((arity, takes)) = abi.call_takes(&call.callee) {
                    call.args.len() == arity as usize && takes.keys().all(|ordinal| *ordinal < arity)
                        && call.args.iter().enumerate().all(|(ordinal, value)| {
                            let value = *abi.aliases().get(value).unwrap_or(value);
                            kinds.get(&value) == Some(&if takes.contains_key(&(ordinal as u32)) { MirOwnershipKindV1::Owned } else { MirOwnershipKindV1::None })
                        }) && call.text_argument_cleanup.is_empty()
                } else { matches!(call.callee, Callee::SameModuleInstance { .. }) && call.args.is_empty() }
            },
            InvokeOperation::Shared(_) => true,
            _ => false,
        };
        if has_managed_operand && !supported {
            return Err(OwnershipSsaErrorV1::ManagedCallOwnershipUnsupported { block });
        }
        if let InvokeOperation::Call { call, .. } = operation {
            if let Some((_, takes)) = abi.call_takes(&call.callee) {
                if !supported { return Err(OwnershipSsaErrorV1::ManagedCallOwnershipUnsupported { block }); }
                for ordinal in takes.keys() {
                    let value = call.args[*ordinal as usize];
                    let value = *abi.aliases().get(&value).unwrap_or(&value);
                    if !live.remove(&value) { return Err(OwnershipSsaErrorV1::OwnedUseAfterConsume { block, value }); }
                }
            }
        }
        if let InvokeOperation::Shared(crate::mir::instruction::SharedInvokeOperation::Release { handle, .. }) = operation {
            let token = *abi.aliases().get(handle).unwrap_or(handle);
            if !live.remove(&token) { return Err(OwnershipSsaErrorV1::OwnedUseAfterConsume { block, value: token }); }
        }
        if let InvokeOperation::HomeRelease { value, .. } | InvokeOperation::ObjectEndIfOwned { value, .. } | InvokeOperation::ReclaimUnpublished { value, .. } = operation {
            let token = *abi.aliases().get(value).unwrap_or(value);
            if kinds.get(&token) == Some(&MirOwnershipKindV1::Owned) && !live.remove(&token) {
                return Err(OwnershipSsaErrorV1::OwnedUseAfterConsume { block, value: token });
            }
        }
    }
    Ok(())
}

fn transfer_phi_edge(
    function: &MirFunction,
    source: BasicBlockId,
    target: BasicBlockId,
    kinds: &BTreeMap<ValueId, MirOwnershipKindV1>,
    abi: &OwnershipFunctionAbiV1,
    input: &LiveSet,
) -> Result<LiveSet, OwnershipSsaErrorV1> {
    let mut output = input.clone();
    let mut sources = BTreeSet::new();
    let mut destinations = Vec::new();
    for instruction in &function.blocks[&target].instructions {
        let MirInstruction::Phi { dst, inputs, .. } = instruction else {
            continue;
        };
        if kinds.get(dst) != Some(&MirOwnershipKindV1::Owned) {
            continue;
        }
        let Some((_, value)) = inputs.iter().find(|(pred, _)| *pred == source) else {
            return Err(OwnershipSsaErrorV1::PhiInputMissing {
                source,
                target,
                dst: *dst,
            });
        };
        let value = if abi.owner().is_source() { *abi.aliases().get(value).unwrap_or(value) } else { *value };
        if !sources.insert(value) {
            return Err(OwnershipSsaErrorV1::DuplicateConsumeOnEdge {
                source,
                target,
                value,
            });
        }
        destinations.push(*dst);
    }
    for value in sources {
        if !output.remove(&value) {
            return Err(OwnershipSsaErrorV1::OwnedUseAfterConsume {
                block: source,
                value,
            });
        }
    }
    for value in destinations {
        if !output.insert(value) {
            return Err(OwnershipSsaErrorV1::OwnedDestinationAlreadyLive {
                block: target,
                value,
            });
        }
    }
    Ok(output)
}

/// Transfer one already-owned Shared token on each actual CFG edge. The first
/// supported join is a rename of the same token on every predecessor; distinct
/// cells, active borrows and non-Shared owned PHIs stay unproved and rejected.
fn transfer_shared_phi_edge(
    function: &MirFunction, source: BasicBlockId, target: BasicBlockId,
    kinds: &BTreeMap<ValueId, MirOwnershipKindV1>, abi: &OwnershipFunctionAbiV1,
    state: &mut SharedState,
) -> Result<(), OwnershipSsaErrorV1> {
    let mut transfers = Vec::new();
    let mut sources = BTreeSet::new();
    let mut destinations = BTreeSet::new();
    for instruction in &function.blocks[&target].instructions {
        let MirInstruction::Phi { dst, inputs, .. } = instruction else { continue; };
        if kinds.get(dst) != Some(&MirOwnershipKindV1::Owned) { continue; }
        let Some((_, incoming)) = inputs.iter().find(|(pred, _)| *pred == source) else {
            return Err(OwnershipSsaErrorV1::PhiInputMissing { source, target, dst: *dst });
        };
        let token = *abi.aliases().get(incoming).unwrap_or(incoming);
        if inputs.iter().any(|(_, value)| *abi.aliases().get(value).unwrap_or(value) != token)
            || !sources.insert(token) || !destinations.insert(*dst)
            || state.borrows.values().any(|support| *support == token)
            || !state.handles.contains_key(&token) {
            return Err(OwnershipSsaErrorV1::InvalidSharedLifecycle { block: source, value: token });
        }
        transfers.push((token, *dst));
    }
    let mut renamed = Vec::new();
    for (token, dst) in transfers {
        let row = state.handles.remove(&token).ok_or(OwnershipSsaErrorV1::InvalidSharedLifecycle { block: source, value: token })?;
        renamed.push((dst, row));
    }
    for (dst, row) in renamed {
        if state.handles.insert(dst, row).is_some() {
            return Err(OwnershipSsaErrorV1::InvalidSharedLifecycle { block: target, value: dst });
        }
    }
    Ok(())
}

fn require_live_if_owned(
    block: BasicBlockId,
    value: ValueId,
    kinds: &BTreeMap<ValueId, MirOwnershipKindV1>,
    live: &LiveSet,
) -> Result<(), OwnershipSsaErrorV1> {
    if kinds.get(&value) == Some(&MirOwnershipKindV1::Owned) && !live.contains(&value) {
        return Err(OwnershipSsaErrorV1::OwnedUseAfterConsume { block, value });
    }
    Ok(())
}

fn reject_edge_arguments(function: &MirFunction) -> Result<(), OwnershipSsaErrorV1> {
    for block in function.blocks.values() {
        for edge in block.out_edges() {
            if edge
                .args
                .as_ref()
                .is_some_and(|args| !args.is_empty_carriers())
            {
                if crate::config::env::builder_debug_enabled() {
                    eprintln!(
                        "[ownership/edge-arguments] source={:?} target={:?} args={:?}",
                        block.id, edge.target, edge.args
                    );
                }
                return Err(OwnershipSsaErrorV1::EdgeArgumentsForbidden {
                    source: block.id,
                    target: edge.target,
                });
            }
        }
    }
    Ok(())
}

fn verify_phi_predecessors(function: &MirFunction) -> Result<(), OwnershipSsaErrorV1> {
    let mut predecessors: BTreeMap<BasicBlockId, BTreeSet<BasicBlockId>> = function
        .blocks
        .keys()
        .copied()
        .map(|block| (block, BTreeSet::new()))
        .collect();
    for block in function.blocks.values() {
        for successor in block.successors.iter().copied() {
            predecessors.entry(successor).or_default().insert(block.id);
        }
    }
    for block in function.blocks.values() {
        for instruction in &block.instructions {
            let MirInstruction::Phi { dst, inputs, .. } = instruction else {
                continue;
            };
            let actual = inputs
                .iter()
                .map(|(predecessor, _)| *predecessor)
                .collect::<BTreeSet<_>>();
            if actual.len() != inputs.len() || predecessors[&block.id] != actual {
                return Err(OwnershipSsaErrorV1::PhiPredecessorMismatch {
                    block: block.id,
                    dst: *dst,
                });
            }
        }
    }
    Ok(())
}

fn reachable_blocks(function: &MirFunction) -> BTreeSet<BasicBlockId> {
    let mut seen = BTreeSet::new();
    let mut queue = VecDeque::from([function.entry_block]);
    while let Some(block) = queue.pop_front() {
        if !seen.insert(block) {
            continue;
        }
        if let Some(raw) = function.blocks.get(&block) {
            queue.extend(raw.successors.iter().copied());
        }
    }
    seen
}

fn collect_dispositions(
    function: &MirFunction,
    kinds: &BTreeMap<ValueId, MirOwnershipKindV1>,
) -> BTreeMap<ValueId, Box<[OwnershipDispositionV1]>> {
    let mut rows: BTreeMap<ValueId, Vec<OwnershipDispositionV1>> = BTreeMap::new();
    for block in function.blocks.values() {
        for instruction in &block.instructions {
            if let MirInstruction::DestroyOwned { value } = instruction {
                rows.entry(*value)
                    .or_default()
                    .push(OwnershipDispositionV1::Destroy { block: block.id });
            }
            if let MirInstruction::Phi { dst, inputs, .. } = instruction {
                if kinds.get(dst) != Some(&MirOwnershipKindV1::Owned) {
                    continue;
                }
                for (predecessor, source) in inputs {
                    rows.entry(*source)
                        .or_default()
                        .push(OwnershipDispositionV1::PhiEdge {
                            predecessor: *predecessor,
                            successor: block.id,
                            destination: *dst,
                        });
                }
            }
        }
        if let Some(MirInstruction::Return { value: Some(value) }) = &block.terminator {
            if kinds.get(value) == Some(&MirOwnershipKindV1::Owned) {
                rows.entry(*value)
                    .or_default()
                    .push(OwnershipDispositionV1::Return { block: block.id });
            }
        }
    }
    rows.into_iter()
        .map(|(value, mut dispositions)| {
            dispositions.sort();
            (value, dispositions.into_boxed_slice())
        })
        .collect()
}
