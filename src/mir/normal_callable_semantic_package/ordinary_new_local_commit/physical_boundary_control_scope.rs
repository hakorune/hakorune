//! Keep recorded scalar Calls' governing control in the SAME boundary sequence.
//! This is physical correspondence, never additional source or Loop permission.
use super::*;

pub(super) fn governing_predecessors(
    boundary: &PhysicalBoundary,
    graph: &BTreeMap<BasicBlockId, Node>,
) -> BTreeSet<BasicBlockId> {
    let mut reachable = BTreeSet::new();
    let mut pending = vec![boundary.entry];
    while let Some(id) = pending.pop() {
        if reachable.insert(id) {
            if let Some(node) = graph.get(&id) { pending.extend(node.edges.iter().copied()); }
        }
    }
    let mut predecessors = BTreeMap::<_, Vec<_>>::new();
    for (id, node) in graph.iter().filter(|(id, _)| reachable.contains(id)) {
        for target in &node.edges { predecessors.entry(*target).or_default().push(*id); }
    }
    let mut pending: Vec<_> = boundary.nodes.iter().filter_map(|(id, node)| {
        let selected = node.instructions.iter().any(|row|
            matches!(row, MirInstruction::Call(call) if call.dst.is_some_and(|dst| boundary.recorded_dsts.contains(&dst))))
            || matches!(&node.terminal, MirInstruction::Invoke {
                operation: crate::mir::instruction::InvokeOperation::Call {
                    result: crate::mir::instruction::InvokeCallResultKind::I64, .. }, ..
            });
        (selected && reachable.contains(id)).then_some(*id)
    }).collect();
    let mut scope = BTreeSet::new();
    while let Some(id) = pending.pop() {
        if scope.insert(id) {
            if let Some(incoming) = predecessors.get(&id) { pending.extend(incoming.iter().copied()); }
        }
    }
    scope
}

impl FinishedBindings {
    /// Both final source consumers replay the already sealed sequence contract.
    /// Only omissions proved by validate_complete survive; no new DCE permission
    /// can be inferred from the module presented by a later consumer.
    pub(in super::super) fn validate_finished_sequences_v1(&self, function: &MirFunction) -> Result<(), String> {
        if function.entry_block != self.entry { return Err(fault("control-entry")); }
        let scope = self.sequences.keys().copied().collect();
        if self.sealed_incoming.as_ref() != Some(&incoming(function, &scope)) {
            return Err(fault("control-finished-incoming"));
        }
        let mut definitions = BTreeMap::<_, Vec<_>>::new();
        for (block, body) in &function.blocks {
            for row in body.all_instructions() {
                if let Some(dst) = row.dst_value() {
                    if function.params.contains(&dst) { return Err(fault("control-parameter-collision")); }
                    definitions.entry(dst).or_default().push((*block, row));
                }
            }
        }
        for (id, (instructions, terminal)) in &self.sequences {
            let actual = function.blocks.get(id).ok_or_else(|| fault("control-sequence-block"))?;
            let expected: Vec<_> = instructions.iter().enumerate()
                .filter(|(index, _)| !self.metadata_omissions.contains(&(*id, *index)))
                .map(|(_, row)| self.instruction(row.clone()))
                .filter(|row| !self.validated_sequence_omissions.contains(&(*id, row.clone()))).collect();
            for row in &expected {
                if let Some(dst) = row.dst_value() {
                    if definitions.get(&dst).map(Vec::as_slice) != Some(&[(*id, row)][..]) {
                        return Err(fault("control-producer-unique"));
                    }
                }
            }
            if actual.instructions != expected || actual.terminator.as_ref() != Some(&self.instruction(terminal.clone())) {
                return Err(fault("control-finished-sequence"));
            }
        }
        for (id, index) in &self.metadata_omissions {
            if !matches!(self.sequences.get(id).and_then(|(rows, _)| rows.get(*index)), Some(MirInstruction::KeepAlive { .. })) {
                return Err(fault("control-metadata-omission"));
            }
        }
        for (_, row) in &self.validated_sequence_omissions {
            if row.dst_value().is_some_and(|dst| function.params.contains(&dst)
                || function.blocks.values().any(|block|
                    block.all_instructions().any(|row| row.dst_value() == Some(dst) || row.used_values().contains(&dst))
                        || block.return_env.as_ref().is_some_and(|values| values.contains(&dst)))) {
                return Err(fault("control-omitted-producer-used"));
            }
        }
        Ok(())
    }
}
