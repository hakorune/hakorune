use crate::mir::verification::utils::{compute_dominators, compute_predecessors, DominatorTree};
use crate::mir::{BasicBlockId, MirFunction};
use std::collections::{BTreeSet, HashMap, HashSet};

pub(super) struct EphemeralReceiverCfgV1 {
    predecessors: HashMap<BasicBlockId, Vec<BasicBlockId>>,
    reachable: HashSet<BasicBlockId>,
    dominators: DominatorTree,
    block_count: usize,
    edge_count: usize,
}

impl EphemeralReceiverCfgV1 {
    pub(super) fn new(function: &MirFunction) -> Result<Self, super::SameRootReceiverProofErrorV1> {
        if !function.blocks.contains_key(&function.entry_block) {
            return Err(super::SameRootReceiverProofErrorV1::MissingCfgBlock);
        }
        let mut edge_count = 0usize;
        for block in function.blocks.values() {
            let derived = block.successors_from_terminator();
            if block.successors != derived {
                return Err(super::SameRootReceiverProofErrorV1::CfgSuccessorCacheMismatch);
            }
            for successor in &derived {
                if !function.blocks.contains_key(successor) {
                    return Err(super::SameRootReceiverProofErrorV1::MissingCfgBlock);
                }
            }
            edge_count = edge_count.saturating_add(derived.len());
        }
        let result = Self {
            predecessors: compute_predecessors(function),
            reachable: compute_normal_reachable(function),
            dominators: compute_dominators(function),
            block_count: function.blocks.len(),
            edge_count,
        };
        Ok(result)
    }

    pub(super) fn is_reachable(&self, block: BasicBlockId) -> bool {
        self.reachable.contains(&block)
    }

    pub(super) fn reachable_predecessors(&self, block: BasicBlockId) -> BTreeSet<BasicBlockId> {
        self.predecessors
            .get(&block)
            .into_iter()
            .flatten()
            .copied()
            .filter(|predecessor| self.reachable.contains(predecessor))
            .collect()
    }

    pub(super) fn dominates(&self, definition: BasicBlockId, use_block: BasicBlockId) -> bool {
        self.dominators.dominates(definition, use_block)
    }

    pub(super) fn edge_participates_in_cycle(
        &self,
        function: &MirFunction,
        phi_block: BasicBlockId,
        predecessor: BasicBlockId,
    ) -> Result<bool, ()> {
        if phi_block == predecessor {
            return Ok(true);
        }
        let mut visited = HashSet::new();
        let mut worklist = vec![phi_block];
        let mut budget = self
            .block_count
            .saturating_add(self.edge_count)
            .saturating_add(1);
        while let Some(block) = worklist.pop() {
            if budget == 0 {
                return Err(());
            }
            budget -= 1;
            if !visited.insert(block) {
                continue;
            }
            let Some(current) = function.blocks.get(&block) else {
                continue;
            };
            for successor in current.successors.iter().rev().copied() {
                if successor == predecessor {
                    return Ok(true);
                }
                if !visited.contains(&successor) {
                    worklist.push(successor);
                }
            }
        }
        Ok(false)
    }
}

fn compute_normal_reachable(function: &MirFunction) -> HashSet<BasicBlockId> {
    let mut reachable = HashSet::new();
    let mut worklist = vec![function.entry_block];
    while let Some(block_id) = worklist.pop() {
        if !reachable.insert(block_id) {
            continue;
        }
        let Some(block) = function.blocks.get(&block_id) else {
            continue;
        };
        for successor in block.successors.iter().rev().copied() {
            if !reachable.contains(&successor) {
                worklist.push(successor);
            }
        }
    }
    reachable
}
