//! Original empty Jump forwarding certificates for the SAME finished projection.
//! Neither final Return operands nor arbitrary matching final blocks issue a route.
use super::*;

#[derive(Debug, Default)]
pub(super) struct ThreadedBranches {
    rows: BTreeMap<BasicBlockId, ThreadedBranch>,
}
#[derive(Debug)]
struct ThreadedBranch {
    original: MirInstruction,
    then_arm: bool,
    else_arm: bool,
}

impl ThreadedBranches {
    pub(super) fn project(
        recorded: &BTreeMap<BasicBlockId, Node>,
        controls: &BTreeSet<BasicBlockId>,
        graph: &BTreeMap<BasicBlockId, Node>,
        function: &MirFunction,
        destinations: &mut BTreeMap<BasicBlockId, BasicBlockId>,
        expected: &BTreeMap<BasicBlockId, (Vec<MirInstruction>, MirInstruction)>,
    ) -> Result<Self, String> {
        let mut proof = Self::default();
        for (&middle, _) in graph.iter().filter(|(id, _)| recorded.contains_key(id) || controls.contains(id)) {
            if destinations.contains_key(&middle) { continue; }
            let Some((destination, mut component)) = Self::forwarding_chain(
                middle, graph, function, destinations, expected)? else { continue };
            let mut pending: Vec<_> = component.iter().copied().collect();
            let mut visited = BTreeSet::new();
            let mut certified = Vec::new();
            while let Some(member) = pending.pop() {
                if !visited.insert(member) { continue; }
                for (&source, row) in graph.iter().filter(|(_, row)| row.edges.contains(&member)) {
                    if let MirInstruction::Branch { then_bb, else_bb, then_edge_args, else_edge_args, .. } = &row.terminal {
                        if !destinations.get(&source).is_some_and(|id| expected.contains_key(id))
                            || graph.values().filter(|other| other.terminal == row.terminal).count() != 1 {
                            return Err(fault("threading-branch-unvalidated"));
                        }
                        let then_arm = *then_bb == member;
                        let else_arm = *else_bb == member;
                        for (selected, args) in [(then_arm, then_edge_args), (else_arm, else_edge_args)] {
                            if selected && args.as_ref().is_some_and(|args| !args.values.is_empty()) {
                                return Err(fault("threading-edge-values"));
                            }
                        }
                        certified.push((source, row.terminal.clone(), then_arm, else_arm));
                    } else {
                        // Only deleted empty Jump nodes within this same proved
                        // chain are internal edges, never arbitrary Jump/Invoke roots.
                        let Some((same, chain)) = Self::forwarding_chain(source, graph, function, destinations, expected)?
                            else { return Err(fault("threading-incoming-kind")) };
                        if same != destination { return Err(fault("threading-incoming-kind")); }
                        for id in chain {
                            if component.insert(id) { pending.push(id); }
                        }
                    }
                }
            }
            if certified.is_empty() { continue; }
            if component.iter().any(|id| destinations.get(id).is_some_and(|old| *old != destination)) {
                return Err(fault("threading-destination-conflict"));
            }
            // Publish only after the entire original component and every
            // external Branch root have passed; partial paths prove nothing.
            for (source, original, then_arm, else_arm) in certified {
                let row = proof.rows.entry(source).or_insert(ThreadedBranch {
                    original, then_arm: false, else_arm: false,
                });
                row.then_arm |= then_arm;
                row.else_arm |= else_arm;
            }
            for id in component { destinations.insert(id, destination); }
        }
        Ok(proof)
    }

    fn forwarding_chain(
        start: BasicBlockId, graph: &BTreeMap<BasicBlockId, Node>, function: &MirFunction,
        destinations: &BTreeMap<BasicBlockId, BasicBlockId>,
        expected: &BTreeMap<BasicBlockId, (Vec<MirInstruction>, MirInstruction)>,
    ) -> Result<Option<(BasicBlockId, BTreeSet<BasicBlockId>)>, String> {
        let mut chain = BTreeSet::new();
        let mut cursor = start;
        loop {
            if !chain.insert(cursor) { return Err(fault("threading-cycle")); }
            if cursor == function.entry_block || function.blocks.contains_key(&cursor) { return Ok(None); }
            let Some(node) = graph.get(&cursor) else { return Ok(None) };
            let MirInstruction::Jump { target, edge_args: None } = &node.terminal else { return Ok(None) };
            if !node.instructions.is_empty() || graph.get(target).is_none_or(|next|
                next.instructions.iter().any(|row| matches!(row, MirInstruction::Phi { .. }))) {
                return Ok(None);
            }
            if let Some(&destination) = destinations.get(target) {
                if expected.contains_key(&destination) { return Ok(Some((destination, chain))); }
                return Ok(None);
            }
            cursor = *target;
        }
    }

    pub(super) fn instruction(&self, mut instruction: MirInstruction) -> MirInstruction {
        let Some(proof) = self.rows.values().find(|row| row.original == instruction) else {
            return instruction;
        };
        if let MirInstruction::Branch {
            then_edge_args,
            else_edge_args,
            ..
        } = &mut instruction
        {
            if proof.then_arm {
                *then_edge_args = None
            }
            if proof.else_arm {
                *else_edge_args = None
            }
        }
        instruction
    }

    pub(super) fn edge_args(
        &self,
        source: BasicBlockId,
        target: BasicBlockId,
        args: &Option<EdgeArgs>,
    ) -> Option<EdgeArgs> {
        let Some(proof) = self.rows.get(&source) else {
            return args.clone();
        };
        let MirInstruction::Branch {
            then_bb,
            else_bb,
            then_edge_args,
            else_edge_args,
            ..
        } = &proof.original
        else {
            unreachable!()
        };
        if (proof.then_arm && *then_bb == target && then_edge_args == args)
            || (proof.else_arm && *else_bb == target && else_edge_args == args)
        {
            None
        } else {
            args.clone()
        }
    }
}
