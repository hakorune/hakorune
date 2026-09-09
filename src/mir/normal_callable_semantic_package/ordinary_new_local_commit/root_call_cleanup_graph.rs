//! Compare two recorded Call cleanup ingresses against the same ordered Homes.
//! Coordinates are physical observations, not a new continuation authority.

use super::*;
use crate::mir::instruction::InvokeCallResultKind;
use crate::mir::ValueId;

pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_original(
    function: &MirFunction,
    bindings: &Bindings,
    invoke: &(BasicBlockId, MirInstruction),
    projection: &(BasicBlockId, MirInstruction),
    operations: &[&InvokeOperation],
) -> Result<(), String> {
    let (clean, pending, frame, result) = ingress(function, bindings, invoke, projection)?;
    let count = operations
        .len()
        .checked_mul(2)
        .ok_or_else(|| fault("home-count"))?;
    let nodes = recorded_nodes(bindings, count)?;
    require_acyclic(&nodes)?;
    let mut reached = BTreeSet::new();
    let mut work = vec![clean, pending];
    while let Some(id) = work.pop() {
        if reached.insert(id) {
            work.extend(
                edges(nodes.get(&id).ok_or_else(|| fault("outward-edge"))?)?
                    .into_iter()
                    .map(|(_, id)| id),
            );
        }
    }
    if reached.len() != nodes.len() {
        return Err(fault("unreachable-node"));
    }
    for (id, terminal) in &nodes {
        let block = function
            .blocks
            .get(id)
            .ok_or_else(|| fault("missing-node"))?;
        let expected = if *id == clean {
            std::slice::from_ref(&projection.1)
        } else {
            &[]
        };
        if block.terminator.as_ref() != Some(*terminal) || block.instructions != expected {
            return Err(fault("original-node"));
        }
    }
    let mut clean = skip_jumps(&nodes, clean)?;
    let mut pending = skip_jumps(&nodes, pending)?;
    for operation in operations {
        let (next_clean, clean_fault) = release(&nodes, clean, operation, frame)?;
        let (next_pending, pending_fault) = release(&nodes, pending, operation, frame)?;
        if next_pending != pending_fault || clean_fault != next_pending || clean == pending {
            return Err(fault("call-cleanup-order"));
        }
        clean = next_clean;
        pending = next_pending;
    }
    if nodes.get(&clean)
        != Some(&&MirInstruction::Return {
            value: Some(result),
        })
        || nodes.get(&pending) != Some(&&MirInstruction::ReturnFault { fault_frame: frame })
    {
        return Err(fault("call-cleanup-terminal"));
    }
    Ok(())
}

/// Called with existing FinishedBindings projections after contraction.
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn ingress(
    function: &MirFunction,
    bindings: &Bindings,
    invoke: &(BasicBlockId, MirInstruction),
    projection: &(BasicBlockId, MirInstruction),
) -> Result<(BasicBlockId, BasicBlockId, ValueId, ValueId), String> {
    let MirInstruction::Invoke {
        operation:
            InvokeOperation::Call {
                call,
                result: InvokeCallResultKind::I64,
            },
        fault_frame,
        normal_landing,
        fault_landing,
    } = &invoke.1
    else {
        return Err(fault("call-ingress"));
    };
    let MirInstruction::InvokeNormalResult {
        dst,
        invoke_block: origin,
    } = &projection.1
    else {
        return Err(fault("call-projection"));
    };
    let nodes: BTreeSet<_> = bindings.iter().map(|(id, _)| *id).collect();
    if call.dst.is_some()
        || *origin != invoke.0
        || projection.0 != *normal_landing
        || normal_landing == fault_landing
        || nodes.contains(&invoke.0)
        || !nodes.contains(normal_landing)
        || !nodes.contains(fault_landing)
        || nodes.contains(&function.entry_block)
        || function
            .blocks
            .get(&invoke.0)
            .and_then(|b| b.terminator.as_ref())
            != Some(&invoke.1)
    {
        return Err(fault("call-ingress"));
    }
    let mut incoming = BTreeSet::new();
    for (id, block) in &function.blocks {
        if nodes.contains(id) {
            continue;
        }
        for (slot, edge) in block.out_edges().into_iter().enumerate() {
            if !nodes.contains(&edge.target) {
                continue;
            }
            if *id != invoke.0
                || edge.args.is_some()
                || !matches!((slot, edge.target), (0, target) if target == *normal_landing)
                    && !matches!((slot, edge.target), (1, target) if target == *fault_landing)
            {
                return Err(fault("call-foreign-incoming"));
            }
            incoming.insert(slot);
        }
    }
    if incoming != BTreeSet::from([0, 1]) {
        return Err(fault("call-missing-incoming"));
    }
    Ok((*normal_landing, *fault_landing, *fault_frame, *dst))
}

fn skip_jumps(
    nodes: &BTreeMap<BasicBlockId, &MirInstruction>,
    mut id: BasicBlockId,
) -> Result<BasicBlockId, String> {
    // Acyclicity is already checked; keep a finite bound for standalone safety.
    for _ in 0..=nodes.len() {
        match nodes.get(&id).ok_or_else(|| fault("outward-edge"))? {
            MirInstruction::Jump {
                target,
                edge_args: None,
            } => id = *target,
            _ => return Ok(id),
        }
    }
    Err(fault("original-cycle"))
}

fn release(
    nodes: &BTreeMap<BasicBlockId, &MirInstruction>,
    id: BasicBlockId,
    expected: &InvokeOperation,
    frame: ValueId,
) -> Result<(BasicBlockId, BasicBlockId), String> {
    match nodes.get(&id) {
        Some(MirInstruction::Invoke {
            operation,
            fault_frame,
            normal_landing,
            fault_landing,
        }) if operation == expected && *fault_frame == frame => Ok((
            skip_jumps(nodes, *normal_landing)?,
            skip_jumps(nodes, *fault_landing)?,
        )),
        _ => Err(fault("call-cleanup-operation")),
    }
}
