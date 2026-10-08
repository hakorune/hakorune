//! Physical correspondence for recorded lifecycle blocks, never source inference.
//! Proven literal folds and original sole-predecessor edges permit contraction.
use super::*;
use crate::mir::EdgeArgs;
use std::collections::{BTreeMap, BTreeSet};

#[path = "physical_boundary_borrowed_copies.rs"]
mod borrowed_copies;
use borrowed_copies::OriginalBorrowedCopies;
#[path = "physical_boundary_literal_controls.rs"]
mod literal_controls;
use literal_controls::LiteralControls;

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
    /// Recorded-binding blocks only — the validation scope.
    nodes: BTreeMap<BasicBlockId, Node>,
    /// Every draft block — the contraction-walk scope. A recorded block may
    /// merge into an uncaptured trampoline, so destinations must be found
    /// from any surviving draft block, not only from binding carriers.
    walk_graph: BTreeMap<BasicBlockId, Node>,
    incoming: Incoming,
    /// Every draft edge `(source, target, args)` in block order — the
    /// unfiltered counterpart of `incoming`. `incoming` only records raw
    /// edges ending on a recorded block, which loses an approach that
    /// reaches a recorded region through a contractible unrecorded
    /// bridge; the projected expected-edge set is derived from this list
    /// with the same destination predicate the actual side applies.
    all_edges: Vec<(BasicBlockId, BasicBlockId, Option<EdgeArgs>)>,
    recorded_dsts: BTreeSet<ValueId>,
    single_definitions: BTreeSet<ValueId>,
    removable_constants: BTreeSet<ValueId>,
    removable_copies: BTreeSet<ValueId>,
    source_copies: BTreeMap<ValueId, (BasicBlockId, MirInstruction)>,
    borrowed_copies: OriginalBorrowedCopies,
    literal_controls: LiteralControls,
}

#[derive(Debug)]
pub(super) struct FinishedBindings {
    destinations: BTreeMap<BasicBlockId, BasicBlockId>,
    recorded: Vec<(BasicBlockId, MirInstruction)>,
    sequences: BTreeMap<BasicBlockId, (Vec<MirInstruction>, MirInstruction)>,
    removable_copies: BTreeSet<ValueId>,
    source_copies: BTreeMap<ValueId, (BasicBlockId, MirInstruction)>,
    borrowed_copies: OriginalBorrowedCopies,
    literal_controls: LiteralControls,
}

impl PhysicalBoundary {
    pub(super) fn capture(function: &MirFunction, bindings: &Bindings) -> Result<Self, String> {
        Self::capture_with_source_copies(function, bindings, &[], &[])
    }

    pub(super) fn capture_with_source_copies(
        function: &MirFunction,
        bindings: &Bindings,
        copies: &[(ValueId, ValueId)],
        borrowed_copies: &Bindings,
    ) -> Result<Self, String> {
        let borrowed_copies = OriginalBorrowedCopies::capture(function, bindings, borrowed_copies)?;
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
            // A recorded block may share join semantics: a leading run of
            // Phi instructions is legal only on a genuine join — at least
            // two incoming edges — which the sole-predecessor concatenation
            // below can never contract into, and whose phi inputs stay
            // pinned by the exact sequence check at `validate_complete`.
            // A phi anywhere else (mid-block, or in a sole-predecessor
            // block that contraction could still reach) keeps the
            // fail-closed rejection.
            let mut saw_non_phi = false;
            let mut has_phi = false;
            for instruction in &block.instructions {
                match instruction {
                    MirInstruction::Phi { .. } if !saw_non_phi => has_phi = true,
                    MirInstruction::Phi { .. } => {
                        return Err(fault("phi-in-recorded-block"));
                    }
                    _ => saw_non_phi = true,
                }
            }
            if has_phi {
                let predecessors = function
                    .blocks
                    .values()
                    .flat_map(|other| other.out_edges())
                    .filter(|edge| edge.target == *id)
                    .count();
                if predecessors < 2 {
                    return Err(fault("phi-in-recorded-block"));
                }
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
        let mut source_copies = BTreeMap::new();
        for (local, result) in copies {
            let expected = MirInstruction::Copy {
                dst: *local,
                src: *result,
            };
            let matches: Vec<_> = function
                .blocks
                .values()
                .flat_map(|block| {
                    block
                        .all_instructions()
                        .filter(|instruction| *instruction == &expected)
                        .map(|_| block.id)
                })
                .collect();
            if matches.len() != 1 {
                return Err(fault("source-copy"));
            }
            source_copies.insert(*local, (matches[0], expected));
            if !used.contains(local) && definitions.get(local) == Some(&1) {
                removable_copies.insert(*local);
            }
        }
        let mut walk_graph = BTreeMap::new();
        for block in function.blocks.values() {
            // A block without a terminator can never be a merge middle, so
            // it contributes no contraction walk entry — if it is removed
            // later it can only be dead-pruned, and any chain reaching it
            // stays fail-closed at `foreign-target`.
            let Some(terminal) = block.terminator.clone() else {
                continue;
            };
            walk_graph.insert(
                block.id,
                Node {
                    instructions: block.instructions.clone(),
                    terminal,
                    edges: block.out_edges().iter().map(|edge| edge.target).collect(),
                },
            );
        }
        Ok(Self {
            nodes,
            walk_graph,
            incoming: incoming(function, &ids),
            all_edges: function
                .blocks
                .values()
                .flat_map(|block| {
                    block
                        .out_edges()
                        .into_iter()
                        .map(|edge| (block.id, edge.target, edge.args))
                        .collect::<Vec<_>>()
                })
                .collect(),
            recorded_dsts: recorded,
            single_definitions: definitions
                .iter()
                .filter_map(|(dst, count)| (*count == 1).then_some(*dst))
                .collect(),
            removable_constants,
            removable_copies,
            source_copies,
            borrowed_copies,
            literal_controls: LiteralControls::capture(function),
        })
    }

    pub(super) fn project(&self, function: &MirFunction) -> Result<FinishedBindings, String> {
        let literal_controls = self.literal_controls.select(function)?;
        let walk_graph: BTreeMap<_, _> = self
            .walk_graph
            .iter()
            .map(|(id, node)| {
                let terminal = literal_controls.instruction(node.terminal.clone());
                let edges = match &terminal {
                    MirInstruction::Jump { target, .. } => vec![*target],
                    _ => node.edges.clone(),
                };
                (
                    *id,
                    Node {
                        instructions: node
                            .instructions
                            .iter()
                            .cloned()
                            .map(|instruction| literal_controls.instruction(instruction))
                            .collect(),
                        terminal,
                        edges,
                    },
                )
            })
            .collect();
        // A sole predecessor means one predecessor *block*, matching the
        // finishing merge's `predecessors.len() == 1` on the deduplicated
        // predecessor set — a both-arms-equal `Branch` is still one.
        let mut predecessors: BTreeMap<_, Vec<_>> = BTreeMap::new();
        for (id, node) in &walk_graph {
            for target in &node.edges {
                let preds = predecessors.entry(*target).or_default();
                if !preds.contains(id) {
                    preds.push(*id);
                }
            }
        }
        for ((id, _), (_, target, _)) in &self.incoming {
            if !literal_controls.permits_edge(*id, *target) {
                continue;
            }
            let preds = predecessors.entry(*target).or_default();
            if !preds.contains(id) {
                preds.push(*id);
            }
        }
        let mut destinations = BTreeMap::new();
        let mut expected = BTreeMap::new();
        for (id, node) in &walk_graph {
            if !function.blocks.contains_key(id) {
                continue;
            }
            let mut cursor = *id;
            let mut current = node;
            let mut instructions = Vec::new();
            // A recorded block may contract into an uncaptured surviving
            // predecessor: the merged sequence still lands at the surviving
            // walk start, so the expected sequence is recorded whenever the
            // walk carried any binding block, not only when the start is one.
            let mut carries_binding =
                self.nodes.contains_key(id) || literal_controls.carries_validation(*id);
            loop {
                if destinations.insert(cursor, *id).is_some() {
                    return Err(fault("duplicate-or-cycle"));
                }
                instructions.extend(current.instructions.iter().cloned());
                // Finishing contracts a block through either a plain `Jump`
                // or a `Branch` whose both arms reach it: the merge folds
                // the equal-arm branch to a jump first, so the draft-side
                // walk must recognize the same effective edge. Edge args
                // carrying values would substitute into merged phis, which
                // stays outside this bounded projection — only value-free
                // edges may contract.
                let consumable = |args: &Option<EdgeArgs>| {
                    args.as_ref().is_none_or(|args| args.values.is_empty())
                };
                let target = match &current.terminal {
                    MirInstruction::Jump { target, edge_args } if consumable(edge_args) => *target,
                    MirInstruction::Branch {
                        then_bb,
                        else_bb,
                        then_edge_args,
                        else_edge_args,
                        ..
                    } if then_bb == else_bb
                        && then_edge_args == else_edge_args
                        && consumable(then_edge_args) =>
                    {
                        *then_bb
                    }
                    _ => break,
                };
                if function.blocks.contains_key(&target) {
                    break;
                }
                if predecessors.get(&target).map(Vec::as_slice) != Some(&[cursor][..]) {
                    return Err(fault("contraction-predecessor"));
                }
                current = walk_graph
                    .get(&target)
                    .ok_or_else(|| fault("foreign-target"))?;
                cursor = target;
                carries_binding |= self.nodes.contains_key(&cursor);
                carries_binding |= literal_controls.carries_validation(cursor);
            }
            if carries_binding {
                expected.insert(*id, (instructions, current.terminal.clone()));
            }
        }
        literal_controls.validate_carrier_destinations(&walk_graph, &destinations)?;
        Ok(FinishedBindings {
            destinations,
            recorded: Vec::new(),
            sequences: expected,
            removable_copies: self.removable_copies.clone(),
            source_copies: self.source_copies.clone(),
            borrowed_copies: self.borrowed_copies.clone(),
            literal_controls,
        })
    }

    pub(super) fn validate_complete(
        &self,
        function: &MirFunction,
        projection: &mut FinishedBindings,
        bindings: &Bindings,
    ) -> Result<(), String> {
        if self
            .nodes
            .keys()
            .any(|id| !projection.destinations.contains_key(id))
        {
            return Err(fault("unmapped-block"));
        }
        // DCE may remove an unrecorded Const or Copy already unused before
        // finishing. These are one-way omission permissions, never an
        // actual-side filter.
        projection.validate_borrowed_copies(function)?;
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
                } else if projection
                    .literal_controls
                    .accepts_omission(expected, &self.recorded_dsts)?
                {
                    // The original proof cone owns this no-definition/no-use omission.
                } else if self.borrowed_copies.contains(expected) {
                    if !self.borrowed_copies.may_omit(function, expected) {
                        return Err(fault("borrowed-copy-omission"));
                    }
                } else if matches!(expected, MirInstruction::Const { dst, .. }
                    if self.removable_constants.contains(dst))
                    || matches!(expected, MirInstruction::Copy { dst, .. }
                        if self.removable_copies.contains(dst))
                    || (expected.effects().is_pure()
                        && expected.dst_value().is_some_and(|dst| {
                            !used.contains(&dst)
                                && !self.recorded_dsts.contains(&dst)
                                && self.single_definitions.contains(&dst)
                        }))
                {
                    // Finishing may drop an unrecorded, uniquely-defined pure
                    // definition whose result became dead — a CSE'd
                    // duplicate, a folded phi, or plain DCE. A recorded
                    // binding never rides this permission, and an actual-side
                    // extra instruction still fails the sequence.
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
        let surviving: BTreeSet<BasicBlockId> = self
            .walk_graph
            .keys()
            .filter(|id| {
                self.nodes.contains_key(id) || projection.literal_controls.carries_validation(**id)
            })
            .filter_map(|id| projection.destinations.get(id))
            .copied()
            .collect();
        // Entry-edge correspondence is checked through the destination map:
        // a draft edge `s -> t` lands as `dest(s) -> dest(t)` keeping its
        // terminator shape and args, a source that was itself contracted or
        // pruned contributes nothing, and a both-arms-equal `Branch` is
        // normalized to the `Jump` finishing folds it into — so the two
        // identical slots collapse onto the single surviving edge. An edge
        // is expected exactly when the actual side would record it: it
        // enters a surviving recorded destination from outside — including
        // through a contractible unrecorded bridge the raw `incoming` map
        // could not see — and never between two projected positions inside
        // the same recorded region.
        let jump_discriminant = std::mem::discriminant(&MirInstruction::Jump {
            target: BasicBlockId(0),
            edge_args: None,
        });
        // `EdgeArgs` carries no `Ord`; both sides are built from ordered
        // `BTreeMap` iteration, so a slot-stable sort by (source, target)
        // keeps equal-arm duplicates comparable element-wise.
        let mut expected_edges: Vec<_> = self
            .all_edges
            .iter()
            .filter(|(source, target, _)| {
                projection.literal_controls.permits_edge(*source, *target)
            })
            .filter_map(|(source, target, args)| {
                let mapped_source = *projection.destinations.get(source)?;
                let mapped_target = *projection.destinations.get(target)?;
                if mapped_source == mapped_target
                    || !surviving.contains(&mapped_target)
                    || surviving.contains(&mapped_source)
                {
                    return None;
                }
                let terminal = projection.literal_controls.instruction(
                    self.walk_graph
                        .get(source)
                        .expect("walk-graph edge source is a walk node")
                        .terminal
                        .clone(),
                );
                let discriminant = std::mem::discriminant(&terminal);
                let (discriminant, args) = match &terminal {
                    MirInstruction::Branch {
                        then_bb,
                        else_bb,
                        then_edge_args,
                        else_edge_args,
                        ..
                    } if then_bb == else_bb && then_edge_args == else_edge_args => {
                        (jump_discriminant, then_edge_args.clone())
                    }
                    _ => (discriminant, args.clone()),
                };
                Some((mapped_source, discriminant, mapped_target, args))
            })
            .collect();
        expected_edges.sort_by_key(|(source, _, target, _)| (*source, *target));
        expected_edges.dedup();
        let mut actual_edges: Vec<_> = incoming(function, &surviving)
            .iter()
            .map(|((source, _), (discriminant, target, args))| {
                (*source, *discriminant, *target, args.clone())
            })
            .collect();
        actual_edges.sort_by_key(|(source, _, target, _)| (*source, *target));
        actual_edges.dedup();
        if actual_edges != expected_edges {
            let missing: Vec<_> = expected_edges
                .iter()
                .filter(|edge| !actual_edges.contains(edge))
                .map(|(s, _, t, _)| (*s, *t))
                .collect();
            let extra: Vec<_> = actual_edges
                .iter()
                .filter(|edge| !expected_edges.contains(edge))
                .map(|(s, _, t, _)| (*s, *t))
                .collect();
            return Err(fault(&format!(
                "incoming-drift function={} missing={missing:?} extra={extra:?}",
                function.signature.name
            )));
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

    /// Check the source-issued local Copy against the same finished
    /// projection used by lifecycle bindings. A DCE omission is accepted only
    /// when the captured boundary explicitly marked that Copy removable and
    /// the final function still has no use of its destination.
    pub(super) fn check_source_local_copy(
        &self,
        function: &MirFunction,
        local: ValueId,
        result: ValueId,
    ) -> Result<bool, String> {
        let Some((source_block, expected)) = self.source_copies.get(&local) else {
            return Ok(false);
        };
        let MirInstruction::Copy { src, .. } = expected else {
            return Ok(false);
        };
        if *src != result {
            return Ok(false);
        }
        let Some(destination) = self.destinations.get(source_block).copied().or_else(|| {
            function
                .blocks
                .contains_key(source_block)
                .then_some(*source_block)
        }) else {
            return Ok(
                self.removable_copies.contains(&local) && !used_values(function).contains(&local)
            );
        };
        let actual = function
            .blocks
            .get(&destination)
            .ok_or_else(|| fault("missing-copy-block"))?;
        let surviving_count = actual
            .all_instructions()
            .filter(|instruction| *instruction == expected)
            .count();
        if surviving_count == 1 {
            return Ok(true);
        }
        Ok(surviving_count == 0
            && self.removable_copies.contains(&local)
            && !used_values(function).contains(&local))
    }
    fn instruction(&self, mut instruction: MirInstruction) -> MirInstruction {
        instruction = self.literal_controls.instruction(instruction);
        // Finishing may contract or re-route a draft block; every block id
        // embedded in a recorded instruction rewrites through the same
        // destination map the binding block itself resolves against.
        let map = |id: &mut BasicBlockId| {
            if let Some(mapped) = self.destinations.get(id) {
                *id = *mapped;
            }
        };
        match &mut instruction {
            MirInstruction::Branch {
                then_bb, else_bb, ..
            } => {
                map(then_bb);
                map(else_bb);
            }
            MirInstruction::Jump { target, .. } => map(target),
            MirInstruction::Invoke {
                normal_landing,
                fault_landing,
                ..
            }
            | MirInstruction::CheckedCallOut {
                normal_landing,
                fault_landing,
                ..
            } => {
                map(normal_landing);
                map(fault_landing);
            }
            MirInstruction::InvokeNormalResult { invoke_block, .. } => map(invoke_block),
            MirInstruction::PinnedTextResidenceEnter {
                normal_landing,
                trap_landing,
                ..
            } => {
                map(normal_landing);
                map(trap_landing);
            }
            MirInstruction::Phi { inputs, .. } => {
                for (predecessor, _) in inputs {
                    map(predecessor);
                }
            }
            MirInstruction::Catch { handler_bb, .. } => map(handler_bb),
            _ => {}
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
