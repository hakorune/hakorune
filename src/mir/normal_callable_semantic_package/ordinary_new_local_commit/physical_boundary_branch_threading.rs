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
        graph: &BTreeMap<BasicBlockId, Node>,
        function: &MirFunction,
        destinations: &mut BTreeMap<BasicBlockId, BasicBlockId>,
        expected: &BTreeMap<BasicBlockId, (Vec<MirInstruction>, MirInstruction)>,
    ) -> Result<Self, String> {
        let mut proof = Self::default();
        for (&middle, node) in recorded {
            if destinations.contains_key(&middle) || function.blocks.contains_key(&middle) {
                continue;
            }
            let MirInstruction::Jump {
                target,
                edge_args: None,
            } = &node.terminal
            else {
                continue;
            };
            if middle == function.entry_block || *target == middle || !node.instructions.is_empty()
            {
                continue;
            }
            let Some(successor) = graph.get(target) else {
                continue;
            };
            if successor
                .instructions
                .iter()
                .any(|i| matches!(i, MirInstruction::Phi { .. }))
            {
                continue;
            }
            let Some(&destination) = destinations.get(target) else {
                continue;
            };
            // A forwarded recorded node must land in an independently validated sequence.
            if !expected.contains_key(&destination) {
                continue;
            }
            let incoming: Vec<_> = graph
                .iter()
                .filter(|(_, row)| row.edges.contains(&middle))
                .collect();
            if incoming.is_empty() {
                continue;
            }
            let mut certified = Vec::new();
            for (&source, row) in incoming {
                let MirInstruction::Branch {
                    then_bb,
                    else_bb,
                    then_edge_args,
                    else_edge_args,
                    ..
                } = &row.terminal
                else {
                    return Err(fault("threading-incoming-kind"));
                };
                if !destinations
                    .get(&source)
                    .is_some_and(|id| expected.contains_key(id))
                    || graph
                        .values()
                        .filter(|other| other.terminal == row.terminal)
                        .count()
                        != 1
                {
                    return Err(fault("threading-branch-unvalidated"));
                }
                let then_arm = *then_bb == middle;
                let else_arm = *else_bb == middle;
                for (selected, args) in [(then_arm, then_edge_args), (else_arm, else_edge_args)] {
                    if selected && args.as_ref().is_some_and(|args| !args.values.is_empty()) {
                        return Err(fault("threading-edge-values"));
                    }
                }
                certified.push((source, row.terminal.clone(), then_arm, else_arm));
            }
            for (source, original, then_arm, else_arm) in certified {
                let row = proof.rows.entry(source).or_insert(ThreadedBranch {
                    original,
                    then_arm: false,
                    else_arm: false,
                });
                row.then_arm |= then_arm;
                row.else_arm |= else_arm;
            }
            destinations.insert(middle, destination);
        }
        Ok(proof)
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
