//! Mandatory source-ordered literal-to-Compare correspondence.
use super::*;
use crate::mir::{BasicBlockId, MirFunction, MirInstruction};
use std::rc::Rc;
type Binding = (BasicBlockId, MirInstruction);
type Literal = integer_literal::BorrowedCompareIntegerLiteralMaterializationV1;

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir) fn record_borrowed_compare_literal_consumers_v1<'a>(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
        observations: impl Iterator<Item = (&'a Rc<Literal>, Vec<Binding>)>,
    ) -> Result<(), String> {
        let observations: Vec<_> = observations.collect();
        let mut entries = self.borrowed_entry_values.borrow_mut();
        let Some(entry) = entries.get_mut(&owner) else {
            return if observations.is_empty() {
                Ok(())
            } else {
                Err(freeze("borrowed-literal/entry-missing"))
            };
        };
        if observations.len() != entry.integer_literals.len() {
            return Err(freeze("borrowed-literal/reuse-coverage"));
        }
        let mut consumers = BTreeMap::new();
        for (record, copies) in observations {
            if record.owner() != owner
                || entry
                    .integer_literals
                    .get(&record.value())
                    .is_none_or(|r| !Rc::ptr_eq(r, record))
                || consumers.insert(record.value(), copies).is_some()
            {
                return Err(freeze("borrowed-literal/reuse-identity"));
            }
        }
        if entry.literal_consumers.is_some() {
            return Err(freeze("borrowed-literal/duplicate-handoff"));
        }
        drop(entries);
        self.verify_literal_consumers(owner, function, Some(&consumers), |binding| {
            Ok(binding.clone())
        })?;
        let mut entries = self.borrowed_entry_values.borrow_mut();
        let entry = entries
            .get_mut(&owner)
            .ok_or_else(|| freeze("borrowed-literal/entry-missing"))?;
        if entry.literal_consumers.is_some() {
            return Err(freeze("borrowed-literal/duplicate-handoff"));
        }
        entry.literal_consumers = Some(consumers);
        Ok(())
    }
    pub(super) fn borrowed_compare_literal_bindings_v1(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> Result<Vec<Binding>, String> {
        let entries = self.borrowed_entry_values.borrow();
        let Some(entry) = entries.get(&owner) else {
            return Ok(Vec::new());
        };
        if entry.integer_literals.is_empty() {
            return Ok(Vec::new());
        }
        let copies = entry
            .literal_consumers
            .as_ref()
            .ok_or_else(|| freeze("borrowed-literal/handoff-missing"))?;
        if !copies.keys().eq(entry.integer_literals.keys()) {
            return Err(freeze("borrowed-literal/reuse-coverage"));
        }
        let mut result = Vec::new();
        for (value, record) in &entry.integer_literals {
            result.push(record.original().clone());
            result.extend_from_slice(&copies[value]);
        }
        Ok(result)
    }
    pub(super) fn verify_finished_borrowed_compare_literals_v1(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
        project: impl FnMut(&Binding) -> Result<Binding, String>,
    ) -> Result<(), String> {
        self.verify_literal_consumers(owner, function, None, project)
    }
    fn verify_literal_consumers(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
        staged: Option<&BTreeMap<ValueId, Vec<Binding>>>,
        mut project: impl FnMut(&Binding) -> Result<Binding, String>,
    ) -> Result<(), String> {
        let entries = self.borrowed_entry_values.borrow();
        let Some(entry) = entries.get(&owner) else {
            return Ok(());
        };
        let source = self
            .borrowed_formal_source
            .as_ref()
            .and_then(|s| s.as_ref().ok());
        let definition = source.and_then(|s| s.definitions.get(&owner));
        let mut used = std::collections::BTreeSet::new();
        for compare in entry.comparisons.values() {
            let Some(definition) = definition else {
                return Err(freeze("borrowed-literal/source-missing"));
            };
            for row in &definition.uses {
                let super::super::super::borrowed_formal_uses::BorrowedFormalUseDraftKindV1::CompareOperand { binary, source } = &row.kind else { continue; };
                let Some((site, integer)) = source.integer_literal() else {
                    continue;
                };
                if binary != compare.source().site() {
                    continue;
                }
                let (left, right) = compare.source().operand_sites();
                let (child, side) = if site == left {
                    (compare.children().0, 0)
                } else if site == right {
                    (compare.children().1, 1)
                } else {
                    return Err(freeze("borrowed-literal/operand-site"));
                };
                let record = entry
                    .integer_literals
                    .get(&child)
                    .ok_or_else(|| freeze("borrowed-literal/operand-root-missing"))?;
                if record.owner() != owner
                    || record.binary() != binary
                    || record.site() != site
                    || !matches!(&record.original().1, MirInstruction::Const { value: crate::mir::ConstValue::Integer(value), .. } if *value == integer)
                {
                    return Err(freeze("borrowed-literal/operand-source"));
                }
                let consumers = staged
                    .or(entry.literal_consumers.as_ref())
                    .ok_or_else(|| freeze("borrowed-literal/handoff-missing"))?;
                if !consumers.keys().eq(entry.integer_literals.keys()) {
                    return Err(freeze("borrowed-literal/reuse-coverage"));
                }
                let original = project(record.original())?;
                let copies = consumers[&child]
                    .iter()
                    .map(&mut project)
                    .collect::<Result<Vec<_>, _>>()?;
                let compare_binding = project(compare.original())?;
                check_literal_group(function, &original, &copies, &compare_binding, side)?;
                used.insert(child);
            }
        }
        if !used.iter().eq(entry.integer_literals.keys()) {
            return Err(freeze("borrowed-literal/operand-coverage"));
        }
        Ok(())
    }
}
fn check_literal_group(
    function: &MirFunction,
    root: &Binding,
    copies: &[Binding],
    compare: &Binding,
    side: usize,
) -> Result<(), String> {
    let MirInstruction::Const {
        dst,
        value: crate::mir::ConstValue::Integer(_),
    } = root.1
    else {
        return Err(freeze("borrowed-literal/root-kind"));
    };
    super::compare_consumers::exact(function, root)?;
    let mut definitions = BTreeMap::new();
    definitions.insert(dst, root);
    for copy in copies {
        let MirInstruction::Copy { dst: copied, src } = copy.1 else {
            return Err(freeze("borrowed-literal/copy-kind"));
        };
        if src != dst || definitions.insert(copied, copy).is_some() {
            return Err(freeze("borrowed-literal/copy-source"));
        }
        super::compare_consumers::exact(function, copy)?;
        precedes(function, root, copy)?;
    }
    let MirInstruction::Compare { lhs, rhs, .. } = compare.1 else {
        return Err(freeze("borrowed-literal/compare-kind"));
    };
    super::compare_consumers::exact(function, compare)?;
    let operand = if side == 0 { lhs } else { rhs };
    let definition = definitions
        .get(&operand)
        .ok_or_else(|| freeze("borrowed-literal/operand-identity"))?;
    precedes(function, root, compare)?;
    precedes(function, definition, compare)
}
fn precedes(function: &MirFunction, source: &Binding, target: &Binding) -> Result<(), String> {
    let dominators = crate::mir::verification::utils::compute_dominators(function);
    if !dominators.is_reachable(source.0)
        || !dominators.is_reachable(target.0)
        || !dominators.dominates(source.0, target.0)
    {
        return Err(freeze("borrowed-literal/dominance"));
    }
    if source.0 == target.0 {
        let rows = &function.blocks[&source.0].instructions;
        let from = rows
            .iter()
            .position(|i| i == &source.1)
            .ok_or_else(|| freeze("borrowed-literal/root-missing"))?;
        let to = rows
            .iter()
            .position(|i| i == &target.1)
            .ok_or_else(|| freeze("borrowed-literal/consumer-missing"))?;
        if from >= to {
            return Err(freeze("borrowed-literal/order"));
        }
    }
    Ok(())
}

#[cfg(test)]
#[path = "ordinary_new_borrowed_compare_literal_consumer_tests.rs"]
mod tests;
