//! Original MIR integer-literal fold certificates, never inferred from final targets.
use super::*;

#[derive(Debug, Clone, Default)]
pub(super) struct LiteralControls {
    comparisons: BTreeMap<ValueId, (MirInstruction, MirInstruction)>,
    branches: BTreeMap<ValueId, (BasicBlockId, MirInstruction, MirInstruction)>,
    dependencies: BTreeMap<ValueId, MirInstruction>,
    carriers: BTreeSet<BasicBlockId>,
    omitted: BTreeSet<ValueId>,
    entry: Option<BasicBlockId>,
}

impl LiteralControls {
    pub(super) fn capture(function: &MirFunction) -> Self {
        let mut counts = BTreeMap::<ValueId, usize>::new();
        let mut branches = BTreeMap::<ValueId, usize>::new();
        for parameter in &function.params {
            *counts.entry(*parameter).or_default() += 1;
        }
        for instruction in function
            .blocks
            .values()
            .flat_map(|block| block.all_instructions())
        {
            if let Some(dst) = instruction.dst_value() {
                *counts.entry(dst).or_default() += 1;
            }
            if let MirInstruction::Branch { condition, .. } = instruction {
                *branches.entry(*condition).or_default() += 1;
            }
        }
        if counts.values().any(|count| *count != 1) {
            return Self::default(); // An ambiguous original definition proves no fold.
        }
        let definitions = crate::mir::value_origin::build_value_def_map(function);
        let mut captured = Self::default();
        for instruction in function
            .blocks
            .values()
            .flat_map(|block| &block.instructions)
        {
            let MirInstruction::Compare { dst, op, lhs, rhs } = instruction else {
                continue;
            };
            let Some(value) = crate::mir::passes::simplify_cfg::literal_integer_compare_to_bool(
                function,
                &definitions,
                *op,
                *lhs,
                *rhs,
            ) else {
                continue;
            };
            captured.comparisons.insert(
                *dst,
                (
                    instruction.clone(),
                    MirInstruction::Const {
                        dst: *dst,
                        value: crate::mir::ConstValue::Bool(value),
                    },
                ),
            );
            for value in [*dst, *lhs, *rhs] {
                captured.capture_dependency(function, &definitions, value);
            }
        }
        for (id, block) in &function.blocks {
            let Some(
                original @ MirInstruction::Branch {
                    condition,
                    then_bb,
                    else_bb,
                    then_edge_args,
                    else_edge_args,
                },
            ) = block.terminator.as_ref()
            else {
                continue;
            };
            if branches.get(condition) != Some(&1) {
                continue;
            }
            let origin =
                crate::mir::value_origin::resolve_value_origin(function, &definitions, *condition);
            let Some((
                _,
                MirInstruction::Const {
                    value: crate::mir::ConstValue::Bool(value),
                    ..
                },
            )) = captured.comparisons.get(&origin)
            else {
                continue;
            };
            let (target, edge_args) = if *value {
                (*then_bb, then_edge_args.clone())
            } else {
                (*else_bb, else_edge_args.clone())
            };
            if target != *id {
                captured.capture_dependency(function, &definitions, *condition);
                captured.carriers.insert(*id);
                captured.branches.insert(
                    *condition,
                    (
                        *id,
                        original.clone(),
                        MirInstruction::Jump { target, edge_args },
                    ),
                );
            }
        }
        if !captured.comparisons.is_empty() {
            captured.entry = Some(function.entry_block);
        }
        captured
    }

    fn capture_dependency(
        &mut self,
        function: &MirFunction,
        definitions: &crate::mir::value_origin::ValueDefMap,
        value: ValueId,
    ) {
        let (_, chain) = crate::mir::value_origin::trace_value_origin(function, definitions, value);
        for value in chain {
            let (block, index) = definitions[&value];
            self.carriers.insert(block);
            self.dependencies
                .insert(value, function.blocks[&block].instructions[index].clone());
        }
    }

    /// Select between already-proven original/folded forms, without learning
    /// any value or target from the finished program.
    pub(super) fn select(&self, function: &MirFunction) -> Result<Self, String> {
        if self
            .entry
            .is_some_and(|entry| entry != function.entry_block)
        {
            return Err(fault("literal-control-entry"));
        }
        let mut definitions = BTreeMap::<ValueId, Vec<&MirInstruction>>::new();
        let mut branches = BTreeMap::<ValueId, usize>::new();
        let mut used: BTreeSet<_> = function
            .blocks
            .values()
            .filter_map(|block| block.return_env.as_ref())
            .flatten()
            .copied()
            .collect();
        for instruction in function
            .blocks
            .values()
            .flat_map(|block| block.all_instructions())
        {
            if let Some(dst) = instruction.dst_value() {
                definitions.entry(dst).or_default().push(instruction);
            }
            used.extend(instruction.used_values());
            if let MirInstruction::Branch { condition, .. } = instruction {
                *branches.entry(*condition).or_default() += 1;
            }
        }
        let mut selected = self.clone();
        if self
            .branches
            .keys()
            .any(|condition| branches.get(condition).is_some_and(|count| *count > 1))
        {
            return Err(fault("literal-control-branch-duplicate"));
        }
        for (dst, original) in &self.dependencies {
            if function.params.contains(dst) {
                return Err(fault("literal-control-definition"));
            }
            let actual = definitions.get(dst).map(Vec::as_slice).unwrap_or_default();
            match actual {
                [actual]
                    if *actual == original
                        || self
                            .comparisons
                            .get(dst)
                            .is_some_and(|(_, folded)| *actual == folded) => {}
                [] if !used.contains(dst) => {
                    selected.omitted.insert(*dst);
                }
                _ => return Err(fault("literal-control-definition")),
            }
        }
        selected.comparisons.retain(|_, (original, _)| {
            !function
                .blocks
                .values()
                .flat_map(|block| &block.instructions)
                .any(|actual| actual == original)
        });
        selected.branches.retain(|condition, _| {
            !function.blocks.values().any(|block| {
                matches!(&block.terminator, Some(MirInstruction::Branch { condition: actual, .. })
                if actual == condition)
            })
        });
        Ok(selected)
    }

    pub(super) fn carries_validation(&self, block: BasicBlockId) -> bool {
        self.carriers.contains(&block)
    }

    /// Proof definitions use the all-block census above, including unreachable
    /// uses. A surviving definition elsewhere cannot be treated as DCE omission.
    fn definition_omission(&self, instruction: &MirInstruction) -> Option<bool> {
        let dst = instruction.dst_value()?;
        self.dependencies
            .contains_key(&dst)
            .then(|| self.omitted.contains(&dst))
    }

    pub(super) fn accepts_omission(
        &self,
        instruction: &MirInstruction,
        recorded: &BTreeSet<ValueId>,
    ) -> Result<bool, String> {
        match self.definition_omission(instruction) {
            None => Ok(false),
            Some(true)
                if instruction.effects().is_pure()
                    && !recorded.contains(&instruction.dst_value().expect("proof definition")) =>
            {
                Ok(true)
            }
            Some(_) => Err(fault("literal-control-omission")),
        }
    }

    pub(super) fn validate_carrier_destinations(
        &self,
        graph: &BTreeMap<BasicBlockId, Node>,
        destinations: &BTreeMap<BasicBlockId, BasicBlockId>,
    ) -> Result<(), String> {
        let mut reachable = BTreeSet::new();
        let mut pending: Vec<_> = self.entry.into_iter().collect();
        while let Some(block) = pending.pop() {
            if reachable.insert(block) {
                if let Some(node) = graph.get(&block) {
                    pending.extend(&node.edges);
                }
            }
        }
        for block in self
            .carriers
            .iter()
            .filter(|block| !destinations.contains_key(block))
        {
            let node = graph
                .get(block)
                .ok_or_else(|| fault("literal-control-carrier"))?;
            let retained_definition = node
                .instructions
                .iter()
                .filter_map(MirInstruction::dst_value)
                .any(|dst| self.dependencies.contains_key(&dst) && !self.omitted.contains(&dst));
            let retained_branch = matches!(&node.terminal, MirInstruction::Branch { condition, .. }
                if self.dependencies.contains_key(condition));
            if reachable.contains(block) || retained_definition || retained_branch {
                return Err(fault("literal-control-carrier"));
            }
        }
        Ok(())
    }

    pub(super) fn instruction(&self, instruction: MirInstruction) -> MirInstruction {
        let replacement = match &instruction {
            MirInstruction::Compare { dst, .. } => self
                .comparisons
                .get(dst)
                .filter(|(original, _)| original == &instruction)
                .map(|(_, folded)| folded),
            MirInstruction::Branch { condition, .. } => self
                .branches
                .get(condition)
                .filter(|(_, original, _)| original == &instruction)
                .map(|(_, _, folded)| folded),
            _ => None,
        };
        replacement.cloned().unwrap_or(instruction)
    }

    pub(super) fn permits_edge(&self, source: BasicBlockId, target: BasicBlockId) -> bool {
        self.branches.values().all(|(block, _, folded)| {
            if *block != source {
                return true;
            }
            matches!(folded, MirInstruction::Jump { target: chosen, .. } if *chosen == target)
        })
    }
}
