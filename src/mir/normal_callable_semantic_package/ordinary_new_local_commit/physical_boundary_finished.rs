//! Final binding projection owned by the captured physical boundary.
use super::*;

impl FinishedBindings {
    pub(in super::super) fn destination(&self, id: BasicBlockId) -> Option<BasicBlockId> {
        self.destinations.get(&id).copied()
    }
    pub(in super::super) fn recorded(&self) -> &Bindings {
        &self.recorded
    }

    /// Check the source-issued local Copy against the same finished
    /// projection used by lifecycle bindings. A DCE omission is accepted only
    /// when the captured boundary explicitly marked that Copy removable and
    /// the final function still has no use of its destination.
    pub(in super::super) fn check_source_local_copy(
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
    pub(super) fn instruction(&self, mut instruction: MirInstruction) -> MirInstruction {
        instruction = self.literal_controls.instruction(instruction);
        instruction = self.threaded_branches.instruction(instruction);
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
    pub(in super::super) fn binding(
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
    pub(in super::super) fn bindings(
        &self,
        bindings: &Bindings,
    ) -> Result<Vec<(BasicBlockId, MirInstruction)>, String> {
        bindings
            .iter()
            .filter_map(|(id, i)| self.binding(*id, i).transpose())
            .collect()
    }
}
