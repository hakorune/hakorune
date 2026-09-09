//! Bounded physical comparison of the existing recorded Home cleanup bindings.
//! This owns no source obligations and never searches MIR for a replacement
//! release. Only a recorded Jump to a deleted, sole-predecessor node may fold.

use super::{freeze, BasicBlockId, MirFunction, MirInstruction};
use crate::mir::instruction::InvokeOperation;
use crate::mir::EdgeArgs;
use std::collections::{BTreeMap, BTreeSet};
use std::mem::{discriminant, Discriminant};

type Bindings = [(BasicBlockId, MirInstruction)];
type Incoming = BTreeMap<(BasicBlockId, usize), (Discriminant<MirInstruction>, Option<EdgeArgs>)>;

#[path = "root_call_cleanup_graph.rs"]
pub(super) mod call;

/// Root-specific source-origin count and graph-shape validation.
pub(super) fn validate_original(
    function: &MirFunction,
    bindings: &Bindings,
    home_count: usize,
) -> Result<(), String> {
    // The emitter has N clean releases and N-1 pending-Fault releases.
    let release_count = home_count
        .checked_mul(2)
        .and_then(|n| n.checked_sub(1))
        .ok_or_else(|| fault("home-count"))?;
    let nodes = recorded_nodes(bindings, release_count)?;
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
    Ok(())
}

#[cfg(test)]
#[derive(Debug)]
struct RootCleanupBoundary(super::physical_boundary::PhysicalBoundary);
#[cfg(test)]
impl RootCleanupBoundary {
    fn capture(
        function: &MirFunction,
        bindings: &Bindings,
        home_count: usize,
    ) -> Result<Self, String> {
        validate_original(function, bindings, home_count)?;
        Ok(Self(super::physical_boundary::PhysicalBoundary::capture(
            function, bindings,
        )?))
    }
    fn project(
        &self,
        function: &MirFunction,
        bindings: &Bindings,
    ) -> Result<Vec<(BasicBlockId, MirInstruction)>, String> {
        let mut projection = self.0.project(function)?;
        self.0
            .validate_complete(function, &mut projection, bindings)?;
        projection.bindings(bindings)
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

fn recorded_nodes(
    bindings: &Bindings,
    release_count: usize,
) -> Result<BTreeMap<BasicBlockId, &MirInstruction>, String> {
    let mut nodes = BTreeMap::new();
    let mut releases = 0;
    for (id, terminal) in bindings {
        if nodes.insert(*id, terminal).is_some() {
            return Err(fault("duplicate-node"));
        }
        edges(terminal)?;
        releases += usize::from(matches!(terminal, MirInstruction::Invoke { .. }));
    }
    if releases != release_count {
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
            operation:
                InvokeOperation::HomeRelease { .. }
                | InvokeOperation::Map(crate::mir::instruction::MapInvokeOperation::End { .. }),
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

pub(super) fn validate_projected_ingress(
    function: &MirFunction,
    bindings: &Bindings,
    entry: BasicBlockId,
) -> Result<(), String> {
    let nodes = bindings.iter().map(|(id, _)| *id).collect();
    boundary_incoming(function, &nodes, entry).map(drop)
}
