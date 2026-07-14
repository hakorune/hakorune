use super::classify::classify;
use super::error::OwnershipSsaErrorV1;
use super::model::{
    collect_ownership_operations, FunctionResultOwnershipV1, MirOwnershipKindV1,
    OwnershipDispositionV1, OwnershipFunctionAbiV1, VerifiedOwnershipSsaV1,
};
use crate::mir::{BasicBlockId, MirFunction, MirInstruction, ValueId};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

type LiveSet = BTreeSet<ValueId>;

pub(super) fn verify(
    function: &MirFunction,
    abi: &OwnershipFunctionAbiV1,
) -> Result<VerifiedOwnershipSsaV1, OwnershipSsaErrorV1> {
    let kinds = classify(function, abi)?;
    let reachable = reachable_blocks(function);
    for block in function.blocks.keys().copied() {
        if !reachable.contains(&block) {
            return Err(OwnershipSsaErrorV1::UnreachableBlock { block });
        }
    }
    reject_edge_arguments(function)?;
    verify_phi_predecessors(function)?;
    verify_instruction_kinds(function, &kinds)?;

    let initial = function
        .params
        .iter()
        .copied()
        .filter(|value| kinds.get(value) == Some(&MirOwnershipKindV1::Owned))
        .collect::<LiveSet>();
    let mut entries = BTreeMap::from([(function.entry_block, initial)]);
    let mut queue = VecDeque::from([function.entry_block]);

    while let Some(block_id) = queue.pop_front() {
        let mut live = entries[&block_id].clone();
        let block = &function.blocks[&block_id];
        for instruction in &block.instructions {
            process_instruction(block_id, instruction, &kinds, &mut live)?;
        }
        if let Some(terminator) = &block.terminator {
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
            let edge_live = transfer_phi_edge(function, block_id, successor, &kinds, &live)?;
            match entries.get(&successor) {
                Some(existing) if existing != &edge_live => {
                    return Err(OwnershipSsaErrorV1::LiveSetMismatch { block: successor });
                }
                Some(_) => {}
                None => {
                    entries.insert(successor, edge_live);
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
                    if kinds.get(src) == Some(&MirOwnershipKindV1::Owned) =>
                {
                    return Err(OwnershipSsaErrorV1::CopyOnOwned {
                        block: block.id,
                        value: *src,
                    });
                }
                MirInstruction::Call {
                    dst, func, args, ..
                } if kinds.get(func) != Some(&MirOwnershipKindV1::None)
                    || args
                        .iter()
                        .any(|value| kinds.get(value) != Some(&MirOwnershipKindV1::None))
                    || dst.is_some_and(|value| {
                        kinds.get(&value) != Some(&MirOwnershipKindV1::None)
                    }) =>
                {
                    return Err(OwnershipSsaErrorV1::ManagedCallOwnershipUnsupported {
                        block: block.id,
                    });
                }
                _ => {}
            }
        }
    }
    Ok(())
}

fn process_instruction(
    block: BasicBlockId,
    instruction: &MirInstruction,
    kinds: &BTreeMap<ValueId, MirOwnershipKindV1>,
    live: &mut LiveSet,
) -> Result<(), OwnershipSsaErrorV1> {
    match instruction {
        MirInstruction::Phi { .. } => return Ok(()),
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
    for value in instruction.used_values() {
        require_live_if_owned(block, value, kinds, live)?;
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
            _ => return Err(OwnershipSsaErrorV1::ResultOwnershipMismatch { block }),
        }
        return Ok(());
    }
    for value in terminator.used_values() {
        require_live_if_owned(block, value, kinds, live)?;
    }
    Ok(())
}

fn transfer_phi_edge(
    function: &MirFunction,
    source: BasicBlockId,
    target: BasicBlockId,
    kinds: &BTreeMap<ValueId, MirOwnershipKindV1>,
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
        if !sources.insert(*value) {
            return Err(OwnershipSsaErrorV1::DuplicateConsumeOnEdge {
                source,
                target,
                value: *value,
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
            if edge.args.is_some() {
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
