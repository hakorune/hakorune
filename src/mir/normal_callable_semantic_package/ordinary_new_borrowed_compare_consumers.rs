//! SAME original checked execution and its mandatory physical Bool consumers.
use super::*;
use crate::mir::{BasicBlockId, MirFunction, MirInstruction};
use std::rc::Rc;

type Binding = (BasicBlockId, MirInstruction);

impl OrdinaryNewClaimLedgerV1 {
    /// Hand back successful append observations to the existing entry owner.
    pub(in crate::mir) fn record_borrowed_compare_consumers_v1<'a>(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
        observations: impl Iterator<
            Item = (
                &'a Rc<compare_materialization::BorrowedCompareMaterializationV1>,
                Vec<Binding>,
                Vec<Binding>,
            ),
        >,
    ) -> Result<(), String> {
        let observations: Vec<_> = observations.collect();
        self.verify_borrowed_compare_reuse_v1(
            owner,
            observations.iter().map(|(record, _, _)| *record),
        )?;
        if observations.is_empty() {
            return Ok(());
        }
        let mut consumers = BTreeMap::new();
        for (record, copies, branches) in observations {
            check_group(function, record.original(), &copies, &branches)?;
            if consumers
                .insert(record.value(), (copies, branches))
                .is_some()
            {
                return Err(freeze("borrowed-compare/duplicate-consumer"));
            }
        }
        let mut entries = self.borrowed_entry_values.borrow_mut();
        let entry = entries
            .get_mut(&owner)
            .ok_or_else(|| freeze("borrowed-compare/entry-missing"))?;
        if entry.comparison_consumers.is_some() {
            return Err(freeze("borrowed-compare/duplicate-consumer-handoff"));
        }
        entry.comparison_consumers = Some(consumers);
        Ok(())
    }

    pub(in crate::mir::normal_callable_semantic_package) fn borrowed_compare_bindings_v1(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> Result<Vec<Binding>, String> {
        let entries = self.borrowed_entry_values.borrow();
        self.verify_borrowed_compare_reuse_v1(
            owner,
            entries
                .get(&owner)
                .into_iter()
                .flat_map(|entry| entry.comparisons.values()),
        )?;
        let Some(entry) = entries.get(&owner) else {
            return Ok(Vec::new());
        };
        if entry.comparisons.is_empty() {
            return Ok(Vec::new());
        }
        let consumers = entry
            .comparison_consumers
            .as_ref()
            .ok_or_else(|| freeze("borrowed-compare/consumer-handoff-missing"))?;
        if !consumers.keys().eq(entry.comparisons.keys()) {
            return Err(freeze("borrowed-compare/consumer-coverage"));
        }
        let mut result = Vec::new();
        for (value, record) in &entry.comparisons {
            let (copies, branches) = consumers
                .get(value)
                .ok_or_else(|| freeze("borrowed-compare/consumer-coverage"))?;
            result.push(record.original().clone());
            result.extend_from_slice(copies);
            result.extend_from_slice(branches);
        }
        result.extend(self.borrowed_compare_carrier_bindings_v1(owner)?);
        result.extend(self.borrowed_compare_literal_bindings_v1(owner)?);
        Ok(result)
    }

    /// Only the existing FinishedBindings owner maps coordinates. This validator
    /// independently checks the mapped execution and exact Bool dataflow.
    pub(in crate::mir::normal_callable_semantic_package) fn verify_finished_borrowed_compares_v1(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
        mut project: impl FnMut(&Binding) -> Result<Binding, String>,
    ) -> Result<(), String> {
        let entries = self.borrowed_entry_values.borrow();
        self.verify_borrowed_compare_reuse_v1(
            owner,
            entries
                .get(&owner)
                .into_iter()
                .flat_map(|entry| entry.comparisons.values()),
        )?;
        let Some(entry) = entries.get(&owner) else {
            return Ok(());
        };
        if entry.comparisons.is_empty() {
            return Ok(());
        }
        let consumers = entry
            .comparison_consumers
            .as_ref()
            .ok_or_else(|| freeze("borrowed-compare/consumer-handoff-missing"))?;
        if !consumers.keys().eq(entry.comparisons.keys()) {
            return Err(freeze("borrowed-compare/consumer-coverage"));
        }
        for (value, record) in &entry.comparisons {
            let (copies, branches) = consumers
                .get(value)
                .ok_or_else(|| freeze("borrowed-compare/consumer-coverage"))?;
            let original = project(record.original())?;
            let copies = copies
                .iter()
                .map(&mut project)
                .collect::<Result<Vec<_>, _>>()?;
            let branches = branches
                .iter()
                .map(&mut project)
                .collect::<Result<Vec<_>, _>>()?;
            check_group(function, &original, &copies, &branches)?;
        }
        self.verify_finished_borrowed_compare_carriers_v1(owner, function, &mut project)?;
        self.verify_finished_borrowed_compare_literals_v1(owner, function, project)?;
        Ok(())
    }
}

fn check_group(
    function: &MirFunction,
    original: &Binding,
    copies: &[Binding],
    branches: &[Binding],
) -> Result<(), String> {
    let MirInstruction::Compare { dst, .. } = &original.1 else {
        return Err(freeze("borrowed-compare/consumer-original"));
    };
    exact(function, original)?;
    if branches.is_empty() {
        return Err(freeze("borrowed-compare/condition-missing"));
    }
    let dominators = crate::mir::verification::utils::compute_dominators(function);
    let dominates = |source: &Binding, target: &Binding| -> Result<(), String> {
        if !dominators.is_reachable(source.0)
            || !dominators.is_reachable(target.0)
            || !dominators.dominates(source.0, target.0)
        {
            return Err(freeze("borrowed-compare/consumer-dominance"));
        }
        if source.0 == target.0 {
            let block = &function.blocks[&source.0];
            let rows: Vec<_> = block.all_instructions().collect();
            let from = rows
                .iter()
                .position(|row| **row == source.1)
                .ok_or_else(|| freeze("borrowed-compare/consumer-identity"))?;
            let to = rows
                .iter()
                .position(|row| **row == target.1)
                .ok_or_else(|| freeze("borrowed-compare/consumer-identity"))?;
            if from >= to {
                return Err(freeze("borrowed-compare/consumer-order"));
            }
        }
        Ok(())
    };
    let mut definitions = BTreeMap::new();
    definitions.insert(*dst, original);
    for copy in copies {
        let MirInstruction::Copy { dst: copied, src } = &copy.1 else {
            return Err(freeze("borrowed-compare/consumer-copy"));
        };
        if src != dst || definitions.insert(*copied, copy).is_some() {
            return Err(freeze("borrowed-compare/consumer-copy"));
        }
        exact(function, copy)?;
        dominates(original, copy)?;
    }
    let mut blocks = std::collections::BTreeSet::new();
    for branch in branches {
        let MirInstruction::Branch { condition, .. } = &branch.1 else {
            return Err(freeze("borrowed-compare/consumer-branch"));
        };
        let definition = definitions
            .get(condition)
            .ok_or_else(|| freeze("borrowed-compare/consumer-condition"))?;
        if !blocks.insert(branch.0) {
            return Err(freeze("borrowed-compare/duplicate-condition"));
        }
        exact(function, branch)?;
        if function.blocks[&branch.0].terminator.as_ref() != Some(&branch.1) {
            return Err(freeze("borrowed-compare/consumer-terminal"));
        }
        dominates(original, branch)?;
        dominates(definition, branch)?;
    }
    Ok(())
}

pub(super) fn exact(function: &MirFunction, binding: &Binding) -> Result<(), String> {
    let count = function
        .blocks
        .get(&binding.0)
        .map(|block| {
            block
                .all_instructions()
                .filter(|row| *row == &binding.1)
                .count()
        })
        .unwrap_or(0);
    if count != 1 {
        return Err(freeze("borrowed-compare/consumer-identity"));
    }
    if let Some(dst) = binding.1.dst_value() {
        if function
            .blocks
            .values()
            .flat_map(|block| block.all_instructions())
            .filter(|row| row.dst_value() == Some(dst))
            .count()
            != 1
        {
            return Err(freeze("borrowed-compare/consumer-definition"));
        }
    }
    Ok(())
}

#[cfg(test)]
#[path = "ordinary_new_borrowed_compare_consumer_tests.rs"]
mod tests;
