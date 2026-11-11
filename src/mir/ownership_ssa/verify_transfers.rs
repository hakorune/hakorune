//! Ownership operation and edge-transfer rules; no source-plan authority.
use super::*;

pub(super) fn verify_instruction_kinds(
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

pub(super) fn verify_call_ownership(
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

pub(super) fn process_instruction(
    block: BasicBlockId,
    instruction: &MirInstruction,
    kinds: &BTreeMap<ValueId, MirOwnershipKindV1>,
    abi: &OwnershipFunctionAbiV1,
    live: &mut LiveSet,
) -> Result<(), OwnershipSsaErrorV1> {
    match instruction {
        MirInstruction::Phi { .. } => return Ok(()),
        MirInstruction::InvokeNormalResult { dst, .. }
            if kinds.get(dst) == Some(&MirOwnershipKindV1::Owned) =>
        {
            if !live.insert(*dst) {
                return Err(OwnershipSsaErrorV1::OwnedDestinationAlreadyLive {
                    block,
                    value: *dst,
                });
            }
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
    if abi.owner().is_source()
        && instruction.used_values().into_iter().any(|value| {
            let token = *abi.aliases().get(&value).unwrap_or(&value);
            kinds.get(&token) != Some(&MirOwnershipKindV1::None)
        })
    {
        return Err(OwnershipSsaErrorV1::UnsupportedManagedInstruction { block });
    }
    for value in instruction.used_values() {
        require_live_if_owned(
            block,
            *abi.aliases().get(&value).unwrap_or(&value),
            kinds,
            live,
        )?;
    }
    Ok(())
}

pub(super) fn process_terminator(
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
            (FunctionResultOwnershipV1::None, Some(value))
                if kinds.get(value) == Some(&MirOwnershipKindV1::None) => {}
            _ => return Err(OwnershipSsaErrorV1::ResultOwnershipMismatch { block }),
        }
        return Ok(());
    }
    for value in terminator.used_values() {
        require_live_if_owned(
            block,
            *abi.aliases().get(&value).unwrap_or(&value),
            kinds,
            live,
        )?;
    }
    if abi.owner().is_source()
        && !matches!(terminator, MirInstruction::Invoke { .. })
        && terminator
            .used_values()
            .into_iter()
            .any(|value| kinds.get(&value) != Some(&MirOwnershipKindV1::None))
    {
        return Err(OwnershipSsaErrorV1::UnsupportedManagedInstruction { block });
    }
    if let MirInstruction::Invoke { operation, .. } = terminator {
        use crate::mir::instruction::InvokeOperation;
        if !abi.owner().is_source() && matches!(operation, InvokeOperation::Shared(_)) {
            return Err(OwnershipSsaErrorV1::UnsupportedManagedInstruction { block });
        }
        if abi.owner().is_source()
            && terminator
                .used_values()
                .into_iter()
                .any(|value| kinds.get(&value) == Some(&MirOwnershipKindV1::Borrowed))
            && !matches!(operation, InvokeOperation::FieldGet { .. })
        {
            return Err(OwnershipSsaErrorV1::UnsupportedManagedInstruction { block });
        }
        let has_managed_operand = terminator.used_values().into_iter().any(|value| {
            let token = *abi.aliases().get(&value).unwrap_or(&value);
            kinds.get(&token) == Some(&MirOwnershipKindV1::Owned)
        });
        let supported = match operation {
            InvokeOperation::Birth(birth) => {
                birth
                    .arguments
                    .iter()
                    .enumerate()
                    .all(|(ordinal, argument)| {
                        argument.value().is_none_or(|value| {
                            let token = *abi.aliases().get(&value).unwrap_or(&value);
                            kinds.get(&token) == Some(&MirOwnershipKindV1::None)
                                || (abi.birth_takes(&birth.key, ordinal)
                                    && kinds.get(&token) == Some(&MirOwnershipKindV1::Owned))
                        })
                    })
            }
            InvokeOperation::HomeRelease { .. }
            | InvokeOperation::ObjectEndIfOwned { .. }
            | InvokeOperation::ReclaimUnpublished { .. }
            | InvokeOperation::FieldGet { .. }
            | InvokeOperation::OwnedObjectFieldRelease { .. } => true,
            InvokeOperation::Call { call, .. } => {
                if let Some((arity, takes)) = abi.call_takes(&call.callee) {
                    call.args.len() == arity as usize
                        && takes.keys().all(|ordinal| *ordinal < arity)
                        && call.args.iter().enumerate().all(|(ordinal, value)| {
                            let value = *abi.aliases().get(value).unwrap_or(value);
                            kinds.get(&value)
                                == Some(&if takes.contains_key(&(ordinal as u32)) {
                                    MirOwnershipKindV1::Owned
                                } else {
                                    MirOwnershipKindV1::None
                                })
                        })
                        && call.text_argument_cleanup.is_empty()
                } else {
                    matches!(call.callee, Callee::SameModuleInstance { .. }) && call.args.is_empty()
                }
            }
            InvokeOperation::Shared(_) => true,
            _ => false,
        };
        if has_managed_operand && !supported {
            return Err(OwnershipSsaErrorV1::ManagedCallOwnershipUnsupported { block });
        }
        if let InvokeOperation::Call { call, .. } = operation {
            if let Some((_, takes)) = abi.call_takes(&call.callee) {
                if !supported {
                    return Err(OwnershipSsaErrorV1::ManagedCallOwnershipUnsupported { block });
                }
                for ordinal in takes.keys() {
                    let value = call.args[*ordinal as usize];
                    let value = *abi.aliases().get(&value).unwrap_or(&value);
                    if !live.remove(&value) {
                        return Err(OwnershipSsaErrorV1::OwnedUseAfterConsume { block, value });
                    }
                }
            }
        }
        if let InvokeOperation::Shared(crate::mir::instruction::SharedInvokeOperation::Release {
            handle,
            ..
        }) = operation
        {
            let token = *abi.aliases().get(handle).unwrap_or(handle);
            if !live.remove(&token) {
                return Err(OwnershipSsaErrorV1::OwnedUseAfterConsume {
                    block,
                    value: token,
                });
            }
        }
        if let InvokeOperation::HomeRelease { value, .. }
        | InvokeOperation::ObjectEndIfOwned { value, .. }
        | InvokeOperation::ReclaimUnpublished { value, .. } = operation
        {
            let token = *abi.aliases().get(value).unwrap_or(value);
            if kinds.get(&token) == Some(&MirOwnershipKindV1::Owned) && !live.remove(&token) {
                return Err(OwnershipSsaErrorV1::OwnedUseAfterConsume {
                    block,
                    value: token,
                });
            }
        }
    }
    Ok(())
}

pub(super) fn transfer_phi_edge(
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
        let value = if abi.owner().is_source() {
            *abi.aliases().get(value).unwrap_or(value)
        } else {
            *value
        };
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
pub(super) fn transfer_shared_phi_edge(
    function: &MirFunction,
    source: BasicBlockId,
    target: BasicBlockId,
    kinds: &BTreeMap<ValueId, MirOwnershipKindV1>,
    abi: &OwnershipFunctionAbiV1,
    state: &mut SharedState,
) -> Result<(), OwnershipSsaErrorV1> {
    let mut transfers = Vec::new();
    let mut sources = BTreeSet::new();
    let mut destinations = BTreeSet::new();
    for instruction in &function.blocks[&target].instructions {
        let MirInstruction::Phi { dst, inputs, .. } = instruction else {
            continue;
        };
        if kinds.get(dst) != Some(&MirOwnershipKindV1::Owned) {
            continue;
        }
        let Some((_, incoming)) = inputs.iter().find(|(pred, _)| *pred == source) else {
            return Err(OwnershipSsaErrorV1::PhiInputMissing {
                source,
                target,
                dst: *dst,
            });
        };
        let token = *abi.aliases().get(incoming).unwrap_or(incoming);
        if inputs
            .iter()
            .any(|(_, value)| *abi.aliases().get(value).unwrap_or(value) != token)
            || !sources.insert(token)
            || !destinations.insert(*dst)
            || state.borrows.values().any(|support| *support == token)
            || !state.handles.contains_key(&token)
        {
            return Err(OwnershipSsaErrorV1::InvalidSharedLifecycle {
                block: source,
                value: token,
            });
        }
        transfers.push((token, *dst));
    }
    let mut renamed = Vec::new();
    for (token, dst) in transfers {
        let row =
            state
                .handles
                .remove(&token)
                .ok_or(OwnershipSsaErrorV1::InvalidSharedLifecycle {
                    block: source,
                    value: token,
                })?;
        renamed.push((dst, row));
    }
    for (dst, row) in renamed {
        if state.handles.insert(dst, row).is_some() {
            return Err(OwnershipSsaErrorV1::InvalidSharedLifecycle {
                block: target,
                value: dst,
            });
        }
    }
    Ok(())
}

pub(super) fn require_live_if_owned(
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

pub(super) fn reject_edge_arguments(function: &MirFunction) -> Result<(), OwnershipSsaErrorV1> {
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

pub(super) fn verify_phi_predecessors(
    function: &MirFunction,
    predecessors: &std::collections::HashMap<BasicBlockId, Vec<BasicBlockId>>,
) -> Result<(), OwnershipSsaErrorV1> {
    for block in function.blocks.values() {
        for instruction in &block.instructions {
            let MirInstruction::Phi { dst, inputs, .. } = instruction else {
                continue;
            };
            let actual = inputs
                .iter()
                .map(|(predecessor, _)| *predecessor)
                .collect::<BTreeSet<_>>();
            if actual.len() != inputs.len()
                || predecessors
                    .get(&block.id)
                    .into_iter()
                    .flatten()
                    .copied()
                    .collect::<BTreeSet<_>>()
                    != actual
            {
                return Err(OwnershipSsaErrorV1::PhiPredecessorMismatch {
                    block: block.id,
                    dst: *dst,
                });
            }
        }
    }
    Ok(())
}

pub(super) fn collect_dispositions(
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
