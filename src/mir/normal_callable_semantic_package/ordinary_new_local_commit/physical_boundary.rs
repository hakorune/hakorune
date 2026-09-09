//! Physical correspondence for recorded lifecycle blocks, never source inference.
//! Only an original Jump into a deleted sole-predecessor block may concatenate.
use super::*;
use crate::mir::EdgeArgs;
use std::collections::{BTreeMap, BTreeSet};

type Bindings = [(BasicBlockId, MirInstruction)];
type Incoming = BTreeMap<
    (BasicBlockId, usize),
    (
        std::mem::Discriminant<MirInstruction>,
        BasicBlockId,
        Option<EdgeArgs>,
    ),
>;

#[derive(Debug)]
struct Node {
    instructions: Vec<MirInstruction>,
    terminal: MirInstruction,
    edges: Vec<BasicBlockId>,
}

#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) struct PhysicalBoundary {
    nodes: BTreeMap<BasicBlockId, Node>,
    incoming: Incoming,
    removable_constants: BTreeSet<ValueId>,
    removable_copies: BTreeSet<ValueId>,
}

pub(super) struct FinishedBindings {
    destinations: BTreeMap<BasicBlockId, BasicBlockId>,
    recorded: Vec<(BasicBlockId, MirInstruction)>,
    sequences: BTreeMap<BasicBlockId, (Vec<MirInstruction>, MirInstruction)>,
}

impl PhysicalBoundary {
    pub(super) fn capture(function: &MirFunction, bindings: &Bindings) -> Result<Self, String> {
        let used = used_values(function);
        let reachable = crate::mir::verification::utils::compute_reachable_blocks(function);
        let mut definitions = BTreeMap::<ValueId, usize>::new();
        for instruction in function
            .blocks
            .values()
            .filter(|block| reachable.contains(&block.id))
            .flat_map(|b| b.all_instructions())
        {
            if let Some(dst) = instruction.dst_value() {
                *definitions.entry(dst).or_default() += 1;
            }
        }
        let recorded: BTreeSet<_> = bindings.iter().filter_map(|(_, i)| i.dst_value()).collect();
        let mut removable_constants = BTreeSet::new();
        let mut removable_copies = BTreeSet::new();
        let ids: BTreeSet<_> = bindings.iter().map(|(id, _)| *id).collect();
        let mut nodes = BTreeMap::new();
        for id in &ids {
            let block = function
                .blocks
                .get(id)
                .ok_or_else(|| fault("missing-block"))?;
            if block
                .instructions
                .iter()
                .any(|i| matches!(i, MirInstruction::Phi { .. }))
            {
                return Err(fault("phi-in-recorded-block"));
            }
            for instruction in &block.instructions {
                let candidate = match instruction {
                    MirInstruction::Const { dst, .. } | MirInstruction::Copy { dst, .. } => {
                        Some(*dst)
                    }
                    _ => None,
                };
                if let Some(dst) = candidate {
                    if !used.contains(&dst)
                        && !recorded.contains(&dst)
                        && definitions.get(&dst) == Some(&1)
                    {
                        match instruction {
                            MirInstruction::Const { .. } => {
                                removable_constants.insert(dst);
                            }
                            MirInstruction::Copy { .. } => {
                                removable_copies.insert(dst);
                            }
                            _ => unreachable!("candidate is const or copy"),
                        }
                    }
                }
            }
            nodes.insert(
                *id,
                Node {
                    instructions: block.instructions.clone(),
                    terminal: block
                        .terminator
                        .clone()
                        .ok_or_else(|| fault("missing-terminal"))?,
                    edges: block.out_edges().iter().map(|edge| edge.target).collect(),
                },
            );
        }
        for (id, instruction) in bindings {
            let node = &nodes[id];
            if node.terminal != *instruction && !node.instructions.contains(instruction) {
                return Err(fault("original-binding"));
            }
        }
        let mut counts: BTreeMap<_, usize> = ids.iter().map(|id| (*id, 0)).collect();
        for node in nodes.values() {
            for target in &node.edges {
                if let Some(count) = counts.get_mut(target) {
                    *count += 1;
                }
            }
        }
        let mut ready: Vec<_> = counts
            .iter()
            .filter_map(|(id, n)| (*n == 0).then_some(*id))
            .collect();
        let mut consumed = 0;
        while let Some(id) = ready.pop() {
            consumed += 1;
            for target in &nodes[&id].edges {
                if let Some(count) = counts.get_mut(target) {
                    *count -= 1;
                    if *count == 0 {
                        ready.push(*target);
                    }
                }
            }
        }
        if consumed != nodes.len() {
            return Err(fault("cycle"));
        }
        Ok(Self {
            nodes,
            incoming: incoming(function, &ids),
            removable_constants,
            removable_copies,
        })
    }

    pub(super) fn project(&self, function: &MirFunction) -> Result<FinishedBindings, String> {
        let mut predecessors: BTreeMap<_, Vec<_>> = BTreeMap::new();
        for (id, node) in &self.nodes {
            for target in &node.edges {
                predecessors.entry(*target).or_default().push(*id);
            }
        }
        for ((id, _), (_, target, _)) in &self.incoming {
            predecessors.entry(*target).or_default().push(*id);
        }
        let mut destinations = BTreeMap::new();
        let mut expected = BTreeMap::new();
        for (id, node) in &self.nodes {
            if !function.blocks.contains_key(id) {
                continue;
            }
            let mut cursor = *id;
            let mut current = node;
            let mut instructions = Vec::new();
            loop {
                if destinations.insert(cursor, *id).is_some() {
                    return Err(fault("duplicate-or-cycle"));
                }
                instructions.extend(current.instructions.iter().cloned());
                let MirInstruction::Jump {
                    target,
                    edge_args: None,
                } = &current.terminal
                else {
                    break;
                };
                if function.blocks.contains_key(target) {
                    break;
                }
                if predecessors.get(target).map(Vec::as_slice) != Some(&[cursor][..]) {
                    return Err(fault("contraction-predecessor"));
                }
                current = self
                    .nodes
                    .get(target)
                    .ok_or_else(|| fault("foreign-target"))?;
                cursor = *target;
            }
            expected.insert(*id, (instructions, current.terminal.clone()));
        }
        Ok(FinishedBindings {
            destinations,
            recorded: Vec::new(),
            sequences: expected,
        })
    }

    pub(super) fn validate_complete(
        &self,
        function: &MirFunction,
        projection: &mut FinishedBindings,
        bindings: &Bindings,
    ) -> Result<(), String> {
        if projection.destinations.len() != self.nodes.len() {
            return Err(fault("unmapped-block"));
        }
        // DCE may remove an unrecorded Const or Copy already unused before
        // finishing. These are one-way omission permissions, never an
        // actual-side filter.
        let used = used_values(function);
        if !self.removable_constants.is_disjoint(&used) || !self.removable_copies.is_disjoint(&used)
        {
            return Err(fault("unused-constant-became-used"));
        }
        for (id, (instructions, terminal)) in &projection.sequences {
            let actual = &function.blocks[id];
            let instructions: Vec<_> = instructions
                .iter()
                .cloned()
                .map(|i| projection.instruction(i))
                .collect();
            let mut remaining = actual.instructions.iter().peekable();
            for expected in &instructions {
                if remaining.peek().is_some_and(|actual| *actual == expected) {
                    remaining.next();
                } else if matches!(expected, MirInstruction::Const { dst, .. }
                    if self.removable_constants.contains(dst))
                    || matches!(expected, MirInstruction::Copy { dst, .. }
                        if self.removable_copies.contains(dst) || !used.contains(dst))
                {
                    continue;
                } else {
                    return Err(fault("finished-sequence"));
                }
            }
            if remaining.next().is_some()
                || actual.terminator.as_ref() != Some(&projection.instruction(terminal.clone()))
            {
                return Err(fault("finished-sequence"));
            }
        }
        let surviving = projection.destinations.values().copied().collect();
        if incoming(function, &surviving) != self.incoming {
            return Err(fault("incoming-drift"));
        }
        for binding in projection.bindings(bindings)? {
            if !projection.recorded.contains(&binding) {
                projection.recorded.push(binding);
            }
        }
        Ok(())
    }
}

impl FinishedBindings {
    pub(super) fn destination(&self, id: BasicBlockId) -> Option<BasicBlockId> {
        self.destinations.get(&id).copied()
    }
    pub(super) fn recorded(&self) -> &Bindings {
        &self.recorded
    }
    fn instruction(&self, mut instruction: MirInstruction) -> MirInstruction {
        if let MirInstruction::InvokeNormalResult { invoke_block, .. } = &mut instruction {
            if let Some(mapped) = self.destinations.get(invoke_block) {
                *invoke_block = *mapped;
            }
        }
        instruction
    }
    pub(super) fn binding(
        &self,
        id: BasicBlockId,
        instruction: &MirInstruction,
    ) -> Result<Option<(BasicBlockId, MirInstruction)>, String> {
        let destination = *self
            .destinations
            .get(&id)
            .ok_or_else(|| fault("unrecorded-binding"))?;
        if matches!(instruction, MirInstruction::Jump { target, edge_args: None }
            if self.destinations.get(target) == Some(&destination))
        {
            return Ok(None);
        }
        Ok(Some((destination, self.instruction(instruction.clone()))))
    }
    pub(super) fn bindings(
        &self,
        bindings: &Bindings,
    ) -> Result<Vec<(BasicBlockId, MirInstruction)>, String> {
        bindings
            .iter()
            .filter_map(|(id, i)| self.binding(*id, i).transpose())
            .collect()
    }
}

pub(super) fn check_binding(
    function: &MirFunction,
    projection: Option<&FinishedBindings>,
    id: BasicBlockId,
    instruction: &MirInstruction,
) -> Result<bool, String> {
    if projection.is_some_and(|p| p.destination(id).is_none()) {
        return Ok(false);
    }
    let mapped = match projection {
        Some(projection) => projection.binding(id, instruction)?,
        None => Some((id, instruction.clone())),
    };
    Ok(mapped.is_none_or(|(id, instruction)| {
        function.blocks.get(&id).is_some_and(|block| {
            block
                .all_instructions()
                .any(|actual| *actual == instruction)
        })
    }))
}

fn incoming(function: &MirFunction, ids: &BTreeSet<BasicBlockId>) -> Incoming {
    let mut result = BTreeMap::new();
    for (id, block) in &function.blocks {
        if ids.contains(id) {
            continue;
        }
        for (slot, edge) in block.out_edges().into_iter().enumerate() {
            if ids.contains(&edge.target) {
                result.insert(
                    (*id, slot),
                    (
                        std::mem::discriminant(
                            block.terminator.as_ref().expect("edge has terminal"),
                        ),
                        edge.target,
                        edge.args,
                    ),
                );
            }
        }
    }
    result
}
fn fault(reason: &str) -> String {
    freeze(&format!("physical-boundary/{reason}"))
}

fn used_values(function: &MirFunction) -> BTreeSet<ValueId> {
    let reachable = crate::mir::verification::utils::compute_reachable_blocks(function);
    function
        .blocks
        .values()
        .filter(|block| reachable.contains(&block.id))
        .flat_map(|block| block.all_instructions())
        .flat_map(MirInstruction::used_values)
        .collect()
}

#[cfg(test)]
#[path = "physical_boundary_tests.rs"]
mod tests;
