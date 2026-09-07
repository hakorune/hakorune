//! Bounded physical comparison of the existing single-Home cleanup bindings.
//! This owns no source obligations and never searches MIR for a replacement
//! release. Only a recorded Jump to a deleted, sole-predecessor node may fold.

use super::{freeze, BasicBlockId, MirFunction, MirInstruction};
use crate::mir::instruction::InvokeOperation;
use crate::mir::EdgeArgs;
use std::collections::{BTreeMap, BTreeSet};
use std::mem::{discriminant, Discriminant};

type Bindings = [(BasicBlockId, MirInstruction)];
type Incoming = BTreeMap<(BasicBlockId, usize), (Discriminant<MirInstruction>, Option<EdgeArgs>)>;

/// Physical context missing from the emitter's existing terminator bindings.
/// Non-entry cleanup prefixes are checked empty, rather than copied.
#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package) struct RootCleanupBoundary {
    entry: BasicBlockId,
    prefix: Vec<MirInstruction>,
    incoming: Incoming,
}

impl RootCleanupBoundary {
    pub(super) fn capture(function: &MirFunction, bindings: &Bindings) -> Result<Self, String> {
        let nodes = recorded_nodes(bindings)?;
        // The emitter records its final entry Jump after the cleanup nodes.
        let (entry, terminal) = bindings.last().ok_or_else(|| fault("empty"))?;
        if !matches!(
            terminal,
            MirInstruction::Jump {
                edge_args: None,
                ..
            }
        ) || *entry == function.entry_block
        {
            return Err(fault("entry"));
        }
        let mut reached = BTreeSet::new();
        let mut pending = vec![*entry];
        while let Some(id) = pending.pop() {
            if !reached.insert(id) {
                continue;
            }
            let terminal = nodes.get(&id).ok_or_else(|| fault("outward-edge"))?;
            pending.extend(edges(terminal)?.into_iter().map(|(_, target)| target));
        }
        if reached.len() != nodes.len() {
            return Err(fault("unreachable-node"));
        }
        require_acyclic(&nodes)?;
        for (id, terminal) in &nodes {
            let block = function
                .blocks
                .get(id)
                .ok_or_else(|| fault("missing-node"))?;
            if block.terminator.as_ref() != Some(*terminal)
                || (*id != *entry && !block.instructions.is_empty())
                || block
                    .instructions
                    .iter()
                    .any(|i| matches!(i, MirInstruction::Phi { .. }))
            {
                return Err(fault("original-node"));
            }
        }
        let incoming = boundary_incoming(function, &reached, *entry)?;
        if incoming.is_empty() {
            return Err(fault("missing-entry-incoming"));
        }
        Ok(Self {
            entry: *entry,
            prefix: function.blocks[entry].instructions.clone(),
            incoming,
        })
    }

    pub(super) fn project(
        &self,
        function: &MirFunction,
        bindings: &Bindings,
    ) -> Result<Vec<(BasicBlockId, MirInstruction)>, String> {
        let nodes = recorded_nodes(bindings)?;
        if !function.blocks.contains_key(&self.entry) {
            return Err(fault("entry-removed"));
        }
        let mut predecessors: BTreeMap<BasicBlockId, Vec<BasicBlockId>> = BTreeMap::new();
        for (id, terminal) in &nodes {
            for (_, target) in edges(terminal)? {
                predecessors.entry(target).or_default().push(*id);
            }
        }
        let mut consumed = BTreeSet::new();
        let mut surviving = BTreeSet::new();
        let mut projected = Vec::new();
        for (id, original) in &nodes {
            let Some(actual) = function.blocks.get(id) else {
                continue;
            };
            surviving.insert(*id);
            let mut cursor = *id;
            let mut terminal = *original;
            loop {
                if !consumed.insert(cursor) {
                    return Err(fault("duplicate-or-cycle"));
                }
                let MirInstruction::Jump {
                    target,
                    edge_args: None,
                } = terminal
                else {
                    break;
                };
                if function.blocks.contains_key(target) {
                    break;
                }
                if *target == self.entry
                    || predecessors.get(target).map(Vec::as_slice) != Some(&[cursor][..])
                {
                    return Err(fault("contraction-predecessor"));
                }
                terminal = nodes
                    .get(target)
                    .copied()
                    .ok_or_else(|| fault("foreign-target"))?;
                cursor = *target;
            }
            let expected_prefix = if *id == self.entry {
                self.prefix.as_slice()
            } else {
                &[]
            };
            if actual.instructions != expected_prefix
                || actual.terminator.as_ref() != Some(terminal)
            {
                return Err(fault("finished-node"));
            }
            projected.push((*id, terminal.clone()));
        }
        if consumed.len() != nodes.len() {
            return Err(fault("residual-node"));
        }
        if boundary_incoming(function, &surviving, self.entry)? != self.incoming {
            return Err(fault("incoming-drift"));
        }
        Ok(projected)
    }
}

// Reachability alone admits a release loop. The original emitted cleanup must
// be a DAG, including the no-opt case where no deleted node is followed.
fn require_acyclic(nodes: &BTreeMap<BasicBlockId, &MirInstruction>) -> Result<(), String> {
    let mut incoming: BTreeMap<_, usize> = nodes.keys().map(|id| (*id, 0)).collect();
    for terminal in nodes.values() {
        for (_, target) in edges(terminal)? {
            *incoming
                .get_mut(&target)
                .ok_or_else(|| fault("outward-edge"))? += 1;
        }
    }
    let mut ready: Vec<_> = incoming
        .iter()
        .filter_map(|(id, count)| (*count == 0).then_some(*id))
        .collect();
    let mut consumed = 0;
    while let Some(id) = ready.pop() {
        consumed += 1;
        for (_, target) in edges(nodes[&id])? {
            let count = incoming.get_mut(&target).expect("checked internal target");
            *count -= 1;
            if *count == 0 {
                ready.push(target);
            }
        }
    }
    if consumed != nodes.len() {
        return Err(fault("original-cycle"));
    }
    Ok(())
}

fn recorded_nodes(bindings: &Bindings) -> Result<BTreeMap<BasicBlockId, &MirInstruction>, String> {
    let mut nodes = BTreeMap::new();
    let mut releases = 0;
    for (id, terminal) in bindings {
        if nodes.insert(*id, terminal).is_some() {
            return Err(fault("duplicate-node"));
        }
        edges(terminal)?;
        releases += usize::from(matches!(terminal, MirInstruction::Invoke { .. }));
    }
    if releases != 1 {
        return Err(fault("release-count"));
    }
    Ok(nodes)
}

/// Slot order and instruction variant preserve Normal/Fault and branch identity.
fn boundary_incoming(
    function: &MirFunction,
    nodes: &BTreeSet<BasicBlockId>,
    entry: BasicBlockId,
) -> Result<Incoming, String> {
    let mut incoming = BTreeMap::new();
    for (id, block) in &function.blocks {
        if nodes.contains(id) {
            continue;
        }
        for (slot, edge) in block.out_edges().into_iter().enumerate() {
            if !nodes.contains(&edge.target) {
                continue;
            }
            if edge.target != entry {
                return Err(fault("internal-incoming"));
            }
            let terminal = block
                .terminator
                .as_ref()
                .ok_or_else(|| fault("incoming-terminal"))?;
            incoming.insert((*id, slot), (discriminant(terminal), edge.args));
        }
    }
    Ok(incoming)
}

fn edges(terminal: &MirInstruction) -> Result<Vec<(usize, BasicBlockId)>, String> {
    match terminal {
        MirInstruction::Jump {
            target,
            edge_args: None,
        } => Ok(vec![(0, *target)]),
        MirInstruction::Invoke {
            operation: InvokeOperation::HomeRelease { .. },
            normal_landing,
            fault_landing,
            ..
        } => Ok(vec![(0, *normal_landing), (1, *fault_landing)]),
        MirInstruction::Return { .. } | MirInstruction::ReturnFault { .. } => Ok(Vec::new()),
        _ => Err(fault("terminal-vocabulary")),
    }
}

fn fault(reason: &str) -> String {
    freeze(&format!("root-cleanup-graph/{reason}"))
}

#[cfg(test)]
#[path = "root_cleanup_graph_tests.rs"]
mod tests;
