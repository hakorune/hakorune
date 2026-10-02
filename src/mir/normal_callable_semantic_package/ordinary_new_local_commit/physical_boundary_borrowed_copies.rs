//! Original borrowed alias Copies, separate from the lifecycle contraction DAG.
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
        let protected: BTreeSet<_> = mandatory
            .iter()
            .filter_map(|(_, i)| i.dst_value())
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
                            match instruction {
                                MirInstruction::Copy { dst: consumer, .. } => {
                                    dead.contains(consumer)
                                        && originals.get(consumer).is_some_and(|original| {
                                            original.0 == block.id && &original.1 == instruction
                                        })
                                }
                                _ => false,
                            }
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
        let MirInstruction::Copy { dst, .. } = instruction else {
            return false;
        };
        self.contains(instruction)
            && self.dead.contains(dst)
            && definition_count(function, *dst) == 0
            && !operand_used(function, *dst)
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
            self.borrowed_copy_coordinate(function, original)?;
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
