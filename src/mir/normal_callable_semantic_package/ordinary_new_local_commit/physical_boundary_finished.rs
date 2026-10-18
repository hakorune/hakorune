//! Final binding projection owned by the captured physical boundary.
use super::*;

impl FinishedBindings {
    /// Lend the exact captured producer of an observed raw child, never a
    /// final instruction selected by its payload or operation shape.
    pub(in super::super) fn captured_definition(
        &self,
        value: ValueId,
    ) -> Result<&(BasicBlockId, MirInstruction), String> {
        let mut definitions = self
            .captured
            .iter()
            .filter(|(_, row)| row.dst_value() == Some(value));
        let original = definitions
            .next()
            .ok_or_else(|| fault("captured-child-missing"))?;
        if definitions.next().is_some() {
            return Err(fault("captured-child-duplicate"));
        }
        Ok(original)
    }

    pub(in super::super) fn destination(&self, id: BasicBlockId) -> Option<BasicBlockId> {
        self.destinations.get(&id).copied()
    }
    pub(in super::super) fn recorded(&self) -> &Bindings {
        &self.recorded
    }

    /// A relation borrows exact captured membership and validated omission permission.
    pub(in super::super) fn relation_binding(
        &self,
        function: &MirFunction,
        original: &(BasicBlockId, MirInstruction),
    ) -> Result<Option<(BasicBlockId, MirInstruction)>, String> {
        let value = original
            .1
            .dst_value()
            .ok_or_else(|| fault("relation-dst"))?;
        if function.params.contains(&value) {
            return Err(fault("relation-parameter-collision"));
        }
        let mut captured = self
            .captured
            .iter()
            .filter(|(_, row)| row.dst_value() == Some(value));
        if captured.next() != Some(original) || captured.next().is_some() {
            return Err(fault("relation-captured-membership"));
        }
        let mapped = self
            .binding(original.0, &original.1)?
            .ok_or_else(|| fault("relation-destination"))?;
        let definitions: Vec<_> = function
            .blocks
            .iter()
            .flat_map(|(block, body)| body.all_instructions().map(move |row| (*block, row)))
            .filter(|(_, row)| row.dst_value() == Some(value))
            .collect();
        if definitions.len() == 1 && definitions[0] == (mapped.0, &mapped.1) {
            return Ok(Some(mapped));
        }
        let unused = function.blocks.values().all(|block| {
            block
                .all_instructions()
                .all(|row| !row.used_values().contains(&value))
                && block
                    .return_env
                    .as_ref()
                    .is_none_or(|values| !values.contains(&value))
        });
        if definitions.is_empty()
            && unused
            && matches!(
                original.1,
                MirInstruction::Const { .. } | MirInstruction::Copy { .. }
            )
            && self.validated_sequence_omissions.contains(&mapped)
        {
            return Ok(None);
        }
        Err(format!("{} function={} original={original:?} mapped={mapped:?} actual={definitions:?}",
            fault("relation-finished-producer"), function.signature.name))
    }

    /// Before the source-owned boundary seal, check original capture and the
    /// actual projected effect. This lends correspondence, not publication.
    pub(in super::super) fn captured_effect_binding(
        &self, function: &MirFunction, original: &(BasicBlockId, MirInstruction),
    ) -> Result<(BasicBlockId, MirInstruction), String> {
        if original.1.dst_value().is_some()
            || self.captured.iter().filter(|row| *row == original).count() != 1 {
            return Err(fault("effect-captured-membership"));
        }
        let mapped = self.binding(original.0, &original.1)?.ok_or_else(|| fault("effect-removed"))?;
        if function.blocks.get(&mapped.0).is_none_or(|block|
            block.all_instructions().filter(|row| **row == mapped.1).count() != 1) {
            return Err(fault("effect-actual-membership"));
        }
        Ok(mapped)
    }
    /// Structural Jumps may contract only through the original destination map.
    /// Recheck the surviving terminator even for an omitted trampoline.
    pub(in super::super) fn structural_jump_binding(
        &self, function: &MirFunction, original: &(BasicBlockId, MirInstruction),
        finalized: bool,
    ) -> Result<Option<(BasicBlockId, MirInstruction)>, String> {
        let MirInstruction::Jump { target, edge_args: None } = original.1 else {
            return Err(fault("structural-jump-shape"));
        };
        if self.captured.iter().filter(|row| *row == original).count() != 1 {
            return Err(fault("structural-jump-capture"));
        }
        let mapped = self.binding(original.0, &original.1)?;
        let destination = self.destination(original.0).ok_or_else(|| fault("structural-jump-destination"))?;
        let expected = match &mapped {
            Some((_, instruction)) => {
                if finalized && self.recorded.iter().filter(|row| **row == (destination, instruction.clone())).count() != 1 {
                    return Err(fault("structural-jump-finished-membership"));
                }
                instruction.clone()
            }
            None => {
                if self.destination(target) != Some(destination) { return Err(fault("structural-jump-contraction")); }
                self.instruction(self.sequences.get(&destination).ok_or_else(|| fault("structural-jump-sequence"))?.1.clone())
            }
        };
        if function.blocks.get(&destination).and_then(|block| block.terminator.as_ref()) != Some(&expected) {
            return Err(fault("structural-jump-actual"));
        }
        Ok(mapped)
    }

    /// Metadata may disappear during finishing, but only the same sealed
    /// ordered sequence decides surviving occurrences at the final consumer.
    pub(in super::super) fn validate_metadata_correspondence_v2(
        &self, function: &MirFunction, original: &(BasicBlockId, MirInstruction), finalized: bool,
    ) -> Result<(), String> {
        if !matches!(original.1, MirInstruction::KeepAlive { .. }) || !self.captured.contains(original) {
            return Err(fault("metadata-captured-membership"));
        }
        let mapped = self.binding(original.0, &original.1)?.ok_or_else(|| fault("metadata-destination"))?;
        if self.sequences.get(&mapped.0).is_none_or(|(rows, _)|
            !rows.iter().any(|row| self.instruction(row.clone()) == mapped.1)) {
            return Err(fault("metadata-sequence-membership"));
        }
        if finalized { self.validate_finished_sequences_v1(function)?; }
        Ok(())
    }

    /// Final effects additionally require the sealed source-owned census.
    pub(in super::super) fn required_effect_binding(
        &self, function: &MirFunction, original: &(BasicBlockId, MirInstruction),
    ) -> Result<(BasicBlockId, MirInstruction), String> {
        let mapped = self.captured_effect_binding(function, original)?;
        if self.recorded.iter().filter(|row| **row == mapped).count() != 1 {
            return Err(fault("effect-finished-membership"));
        }
        Ok(mapped)
    }

    /// Lend the captured local producer through the same exact Copy contract.
    pub(in super::super) fn source_local_copy_binding_v1(
        &self, function: &MirFunction, local: ValueId, result: ValueId,
    ) -> Result<Option<(BasicBlockId, MirInstruction)>, String> {
        if !self.check_source_local_copy(function, local, result)? { return Err(fault("source-copy-drift")); }
        let (block, instruction) = self.source_copies.get(&local).ok_or_else(|| fault("source-copy-missing"))?;
        Ok(Some((*block, instruction.clone())))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn relation_requires_validated_omission_and_exact_unique_finished_producer() {
        let (original, bindings) = super::super::tests::fixture();
        let raw = (
            BasicBlockId(0),
            original.blocks[&BasicBlockId(0)].instructions[1].clone(),
        );
        let boundary = PhysicalBoundary::capture(&original, &bindings).unwrap();
        let mut projection = boundary.project(&original).unwrap();
        boundary
            .validate_complete(&original, &mut projection, &bindings)
            .unwrap();
        assert_eq!(
            projection.relation_binding(&original, &raw).unwrap(),
            Some(raw.clone())
        );
        let mut collision = original.clone();
        collision.params.push(ValueId(2));
        assert!(projection
            .relation_binding(&collision, &raw)
            .unwrap_err()
            .contains("parameter-collision"));
        let mut foreign = raw.clone();
        if let MirInstruction::Const { value, .. } = &mut foreign.1 {
            *value = crate::mir::ConstValue::Integer(99);
        }
        assert!(projection
            .relation_binding(&original, &foreign)
            .unwrap_err()
            .contains("captured-membership"));
        let mut changed = original.clone();
        changed
            .blocks
            .get_mut(&BasicBlockId(0))
            .unwrap()
            .instructions[1] = foreign.1.clone();
        assert!(projection.relation_binding(&changed, &raw).is_err());
        let mut duplicate = original.clone();
        duplicate
            .blocks
            .get_mut(&BasicBlockId(0))
            .unwrap()
            .instructions
            .push(raw.1.clone());
        assert!(projection.relation_binding(&duplicate, &raw).is_err());
        let mut omitted = original.clone();
        omitted
            .blocks
            .get_mut(&BasicBlockId(0))
            .unwrap()
            .instructions
            .truncate(1);
        let mut projection = boundary.project(&omitted).unwrap();
        assert!(projection.relation_binding(&omitted, &raw).is_err());
        boundary
            .validate_complete(&omitted, &mut projection, &bindings)
            .unwrap();
        assert!(projection
            .relation_binding(&omitted, &raw)
            .unwrap()
            .is_none());
        let mut live = omitted.clone();
        live.blocks.get_mut(&BasicBlockId(0)).unwrap().return_env = Some(vec![ValueId(2)]);
        assert!(projection.relation_binding(&live, &raw).is_err());
        let mut resurrected = omitted.clone();
        resurrected
            .blocks
            .get_mut(&BasicBlockId(0))
            .unwrap()
            .instructions
            .push(foreign.1);
        assert!(projection.relation_binding(&resurrected, &raw).is_err());
    }
}
