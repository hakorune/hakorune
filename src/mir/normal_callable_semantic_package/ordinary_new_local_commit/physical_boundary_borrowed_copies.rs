//! Captured borrowed aliases and source-proven scalar read cones, separate from control contraction.
//! This is physical correspondence only; the original entry loan owns authority.
use super::*;

#[derive(Debug, Clone, Default)]
pub(super) struct OriginalBorrowedCopies {
    originals: BTreeMap<ValueId, (BasicBlockId, MirInstruction)>,
    dead: BTreeSet<ValueId>,
}

impl OriginalBorrowedCopies {
    pub(super) fn capture(
        function: &MirFunction,
        mandatory: &Bindings,
        copies: &Bindings,
        reads: &Bindings,
    ) -> Result<Self, String> {
        let mut originals = BTreeMap::new();
        for (block, instruction) in copies {
            let MirInstruction::Copy { dst, .. } = instruction else {
                return Err(fault("borrowed-copy-instruction"));
            };
            if definition_count(function, *dst) != 1
                || function.params.contains(dst)
                || function.blocks.get(block).is_none_or(|row| {
                    row.all_instructions()
                        .filter(|actual| *actual == instruction)
                        .count()
                        != 1
                })
            {
                return Err(fault("borrowed-copy-original"));
            }
            if let Some(previous) = originals.insert(*dst, (*block, instruction.clone())) {
                if previous != (*block, instruction.clone()) {
                    return Err(fault("borrowed-copy-conflicting-prefix"));
                }
            }
        }
        let local_reads = crate::mir::passes::dce::local_fields::LocalReadInfo::default();
        let removable_read = |row: &MirInstruction| {
            crate::mir::passes::dce::local_fields::is_removable_effect_sensitive_read_instruction(
                row,
                &local_reads,
            )
        };
        let mut optional = BTreeSet::new();
        for binding in reads {
            if !(matches!(
                binding.1,
                MirInstruction::Copy { .. }
                    | MirInstruction::Const {
                        value: crate::mir::ConstValue::Null,
                        ..
                    }
            ) || removable_read(&binding.1))
            {
                return Err(fault("borrowed-read-kind"));
            }
            let dst = binding
                .1
                .dst_value()
                .ok_or_else(|| fault("borrowed-read-dst"))?;
            if definition_count(function, dst) != 1
                || function.params.contains(&dst)
                || function.blocks.get(&binding.0).is_none_or(|block| {
                    block
                        .all_instructions()
                        .filter(|row| *row == &binding.1)
                        .count()
                        != 1
                })
            {
                return Err(fault("borrowed-read-original"));
            }
            if originals
                .insert(dst, binding.clone())
                .is_some_and(|old| old != *binding)
            {
                return Err(fault("borrowed-copy-conflicting-prefix"));
            }
            optional.insert(dst);
        }
        // Transparent result Copies can extend beyond the initializer receipt.
        // Other mandatory consumers never become optional through this closure.
        loop {
            let mut added = false;
            for (id, block) in &function.blocks {
                for row in &block.instructions {
                    let MirInstruction::Copy { dst, src } = row else {
                        continue;
                    };
                    if optional.contains(src)
                        && !originals.contains_key(dst)
                        && !mandatory
                            .iter()
                            .any(|(_, row)| row.dst_value() == Some(*dst))
                    {
                        if definition_count(function, *dst) != 1 || function.params.contains(dst) {
                            return Err(fault("borrowed-read-copy-definition"));
                        }
                        originals.insert(*dst, (*id, row.clone()));
                        optional.insert(*dst);
                        added = true;
                    }
                }
            }
            if !added {
                break;
            }
        }
        let protected: BTreeSet<_> = mandatory
            .iter()
            .filter_map(|(_, i)| i.dst_value())
            .filter(|dst| !optional.contains(dst))
            .collect();
        let mut dead = BTreeSet::new();
        loop {
            let mut added = false;
            for dst in originals.keys() {
                if protected.contains(dst) || dead.contains(dst) {
                    continue;
                }
                // Include every physical block and return-env metadata. A use
                // outside already-dead original Copies prevents omission.
                let only_dead_users = function.blocks.values().all(|block| {
                    !block
                        .return_env
                        .as_ref()
                        .is_some_and(|values| values.contains(dst))
                        && block.all_instructions().all(|instruction| {
                            if !instruction.used_values().contains(dst) {
                                return true;
                            }
                            instruction.dst_value().is_some_and(|consumer| {
                                (matches!(instruction, MirInstruction::Copy { .. })
                                    || removable_read(instruction))
                                    && dead.contains(&consumer)
                                    && originals.get(&consumer).is_some_and(|original| {
                                        original.0 == block.id && &original.1 == instruction
                                    })
                            })
                        })
                });
                if only_dead_users {
                    dead.insert(*dst);
                    added = true;
                }
            }
            if !added {
                break;
            }
        }
        Ok(Self { originals, dead })
    }

    pub(super) fn contains(&self, instruction: &MirInstruction) -> bool {
        instruction
            .dst_value()
            .and_then(|dst| self.originals.get(&dst))
            .is_some_and(|(_, original)| original == instruction)
    }

    pub(super) fn may_omit(&self, function: &MirFunction, instruction: &MirInstruction) -> bool {
        let Some(dst) = instruction.dst_value() else {
            return false;
        };
        let allowed = self.contains(instruction)
            && self.dead.contains(&dst)
            && definition_count(function, dst) == 0
            && !operand_used(function, dst);
        allowed
    }
}

impl PhysicalBoundary {
    /// This is the captured pre-optimization permission, not final omission
    /// approval. Projection still requires no definition and no use afterward.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn permits_dead_source_elision_v1(
        &self,
        original: &(BasicBlockId, MirInstruction),
    ) -> bool {
        original.1.dst_value().is_some_and(|dst| {
            self.borrowed_copies.originals.get(&dst) == Some(original)
                && self.borrowed_copies.dead.contains(&dst)
        })
    }
}

impl FinishedBindings {
    /// A missing optional Copy is not a Jump contraction. Only the captured
    /// dead cone plus final no-use/no-definition authorizes its omission.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn borrowed_copy_coordinate(
        &self,
        function: &MirFunction,
        original: &(BasicBlockId, MirInstruction),
    ) -> Result<Option<(BasicBlockId, usize)>, String> {
        let MirInstruction::Copy { dst, .. } = &original.1 else {
            return Err(fault("borrowed-copy-instruction"));
        };
        if self.borrowed_copies.originals.get(dst) != Some(original) {
            return Err(fault("borrowed-copy-membership"));
        }
        let destination = self.destinations.get(&original.0).copied().or_else(|| {
            function
                .blocks
                .contains_key(&original.0)
                .then_some(original.0)
        });
        if let Some(block) = destination.and_then(|id| function.blocks.get(&id)) {
            let mut matches = block
                .instructions
                .iter()
                .enumerate()
                .filter(|(_, instruction)| *instruction == &original.1);
            if let Some((index, _)) = matches.next() {
                if matches.next().is_some() || definition_count(function, *dst) != 1 {
                    return Err(fault("borrowed-copy-final-definition"));
                }
                return Ok(Some((block.id, index)));
            }
        }
        if self.borrowed_copies.may_omit(function, &original.1) {
            return Ok(None);
        }
        Err(fault("borrowed-copy-final-correspondence"))
    }

    pub(super) fn validate_borrowed_copies(&self, function: &MirFunction) -> Result<(), String> {
        for original in self.borrowed_copies.originals.values() {
            if matches!(original.1, MirInstruction::Copy { .. }) {
                self.borrowed_copy_coordinate(function, original)?;
            }
        }
        Ok(())
    }
}

fn definition_count(function: &MirFunction, value: ValueId) -> usize {
    function
        .params
        .iter()
        .filter(|parameter| **parameter == value)
        .count()
        + function
            .blocks
            .values()
            .flat_map(|block| block.all_instructions())
            .filter(|instruction| instruction.dst_value() == Some(value))
            .count()
}

fn operand_used(function: &MirFunction, value: ValueId) -> bool {
    function.blocks.values().any(|block| {
        block
            .return_env
            .as_ref()
            .is_some_and(|values| values.contains(&value))
            || block
                .all_instructions()
                .any(|instruction| instruction.used_values().contains(&value))
    })
}

#[cfg(test)]
#[path = "physical_boundary_borrowed_copies_tests.rs"]
mod tests;
