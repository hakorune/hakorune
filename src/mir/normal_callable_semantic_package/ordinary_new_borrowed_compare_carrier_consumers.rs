//! SAME source carrier loans, exact ordered consumers and mandatory bindings.
use super::*;
use crate::mir::{BasicBlockId, MirFunction, MirInstruction};
use std::rc::Rc;
type Binding = (BasicBlockId, MirInstruction);
type Loan = BorrowedCompareCarrierOperandLoanV1;
type Key = (OwnedExprSiteV1, OwnedExprSiteV1);
type Consumers = BTreeMap<Key, (Rc<Loan>, Vec<Binding>)>;
impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir) fn record_borrowed_compare_carrier_consumers_v1<'a>(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
        observations: impl Iterator<Item = (&'a Rc<Loan>, Vec<Binding>)>,
    ) -> Result<(), String> {
        let mut staged = Consumers::new();
        for (loan, copies) in observations {
            if loan.owner() != owner
                || staged
                    .insert(
                        (loan.binary().clone(), loan.site().clone()),
                        (Rc::clone(loan), copies),
                    )
                    .is_some()
            {
                return Err(freeze("borrowed-carrier/duplicate-loan"));
            }
        }
        self.verify_carrier_consumers(owner, function, Some(&staged), |binding| {
            Ok(binding.clone())
        })?;
        if staged.is_empty() {
            return Ok(());
        }
        let mut entries = self.borrowed_entry_values.borrow_mut();
        let entry = entries
            .get_mut(&owner)
            .ok_or_else(|| freeze("borrowed-carrier/entry-missing"))?;
        if entry.carrier_consumers.is_some() {
            return Err(freeze("borrowed-carrier/duplicate-handoff"));
        }
        entry.carrier_consumers = Some(staged);
        Ok(())
    }
    pub(super) fn borrowed_compare_carrier_bindings_v1(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> Result<Vec<Binding>, String> {
        let entries = self.borrowed_entry_values.borrow();
        let Some(entry) = entries.get(&owner) else {
            return Ok(vec![]);
        };
        if entry.comparisons.is_empty() {
            return Ok(vec![]);
        }
        let consumers = entry
            .carrier_consumers
            .as_ref()
            .ok_or_else(|| freeze("borrowed-carrier/handoff-missing"))?;
        let mut result = Vec::new();
        for (loan, copies) in consumers.values() {
            for binding in loan.original_copies()?.iter().chain(copies.iter()) {
                if !result.contains(binding) {
                    result.push(binding.clone());
                }
            }
        }
        Ok(result)
    }
    pub(super) fn verify_finished_borrowed_compare_carriers_v1(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
        project: impl FnMut(&Binding) -> Result<Binding, String>,
    ) -> Result<(), String> {
        self.verify_carrier_consumers(owner, function, None, project)
    }
    fn verify_carrier_consumers(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
        staged: Option<&Consumers>,
        mut project: impl FnMut(&Binding) -> Result<Binding, String>,
    ) -> Result<(), String> {
        let entries = self.borrowed_entry_values.borrow();
        let Some(entry) = entries.get(&owner) else {
            return if staged.is_some_and(|staged| !staged.is_empty()) {
                Err(freeze("borrowed-carrier/entry-missing"))
            } else {
                Ok(())
            };
        };
        let consumers = staged.or(entry.carrier_consumers.as_ref());
        let mut required = std::collections::BTreeSet::new();
        for compare in entry.comparisons.values() {
            let loans = self.borrow_compare_carrier_operands_v1(compare.source())?;
            for expected in loans {
                let key = (expected.binary().clone(), expected.site().clone());
                if !required.insert(key.clone()) {
                    return Err(freeze("borrowed-carrier/source-duplicate"));
                }
                let (loan, copies) = consumers
                    .and_then(|rows| rows.get(&key))
                    .ok_or_else(|| freeze("borrowed-carrier/coverage"))?;
                self.verify_compare_carrier_operand_loan_v1(compare.source(), loan)?;
                let (left, right) = compare.source().operand_sites();
                let (child, side) = if loan.site() == left {
                    (compare.children().0, 0)
                } else if loan.site() == right {
                    (compare.children().1, 1)
                } else {
                    return Err(freeze("borrowed-carrier/side"));
                };
                if child != loan.value() {
                    return Err(freeze("borrowed-carrier/raw-child"));
                }
                let original = loan
                    .original_copies()?
                    .iter()
                    .map(&mut project)
                    .collect::<Result<Vec<_>, _>>()?;
                let copies = copies
                    .iter()
                    .map(&mut project)
                    .collect::<Result<Vec<_>, _>>()?;
                let compare = project(compare.original())?;
                check_group(
                    function,
                    loan.entry_value(),
                    loan.value(),
                    &original,
                    &copies,
                    &compare,
                    side,
                )?;
            }
        }
        if consumers.map_or(0, |rows| rows.len()) != required.len() {
            return Err(freeze("borrowed-carrier/coverage"));
        }
        Ok(())
    }
}
fn check_group(
    function: &MirFunction,
    entry: ValueId,
    value: ValueId,
    originals: &[Binding],
    copies: &[Binding],
    compare: &Binding,
    side: usize,
) -> Result<(), String> {
    if function
        .params
        .iter()
        .filter(|parameter| **parameter == entry)
        .count()
        != 1
        || function
            .blocks
            .values()
            .flat_map(|block| block.all_instructions())
            .any(|row| row.dst_value() == Some(entry))
    {
        return Err(freeze("borrowed-carrier/parameter"));
    }
    let mut current = entry;
    let mut previous: Option<&Binding> = None;
    for binding in originals {
        let MirInstruction::Copy { dst, src } = binding.1 else {
            return Err(freeze("borrowed-carrier/original-copy"));
        };
        if src != current {
            return Err(freeze("borrowed-carrier/chain"));
        }
        compare_consumers::exact(function, binding)?;
        precedes(function, previous, binding)?;
        current = dst;
        previous = Some(binding);
    }
    if current != value {
        return Err(freeze("borrowed-carrier/root"));
    }
    let mut definitions = BTreeMap::new();
    definitions.insert(value, previous);
    for copy in copies {
        let MirInstruction::Copy { dst, src } = copy.1 else {
            return Err(freeze("borrowed-carrier/copy-kind"));
        };
        if src != value || definitions.insert(dst, Some(copy)).is_some() {
            return Err(freeze("borrowed-carrier/copy-root"));
        }
        compare_consumers::exact(function, copy)?;
        precedes(function, previous, copy)?;
    }
    let MirInstruction::Compare { lhs, rhs, .. } = compare.1 else {
        return Err(freeze("borrowed-carrier/compare-kind"));
    };
    compare_consumers::exact(function, compare)?;
    let operand = if side == 0 { lhs } else { rhs };
    let definition = definitions
        .get(&operand)
        .ok_or_else(|| freeze("borrowed-carrier/operand"))?;
    precedes(function, previous, compare)?;
    precedes(function, *definition, compare)
}
fn precedes(
    function: &MirFunction,
    source: Option<&Binding>,
    target: &Binding,
) -> Result<(), String> {
    let block = source.map_or(function.entry_block, |source| source.0);
    let dominators = crate::mir::verification::utils::compute_dominators(function);
    if !dominators.is_reachable(block)
        || !dominators.is_reachable(target.0)
        || !dominators.dominates(block, target.0)
    {
        return Err(freeze("borrowed-carrier/dominance"));
    }
    if let Some(source) = source.filter(|source| source.0 == target.0) {
        let rows = &function.blocks[&block].instructions;
        let from = rows
            .iter()
            .position(|row| *row == source.1)
            .ok_or_else(|| freeze("borrowed-carrier/original-missing"))?;
        let to = rows
            .iter()
            .position(|row| *row == target.1)
            .ok_or_else(|| freeze("borrowed-carrier/consumer-missing"))?;
        if from >= to {
            return Err(freeze("borrowed-carrier/order"));
        }
    }
    Ok(())
}

#[cfg(test)]
#[path = "ordinary_new_borrowed_compare_carrier_consumer_tests.rs"]
mod tests;
