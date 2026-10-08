//! Exact original Integer Return and SAME Compare through the existing finishing owner.
use super::super::root_home::{RootHomeExitEntry, RootHomeExitProgress};
use super::*;

impl FinalizedRootSourceHandoffV1 {
    pub(in crate::mir) fn with_borrowed_ordinary_integer_returns_v1(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
        mut visit: impl FnMut(
            BindingRefV1,
            BindingRefV1,
            &OwnedExprSiteV1,
            &(BasicBlockId, MirInstruction),
            (BasicBlockId, usize),
            (BasicBlockId, usize),
            &(BasicBlockId, MirInstruction),
        ) -> Result<(), String>,
    ) -> Result<(), String> {
        let source = self.borrowed_ordinary_entry_source_for_function_v1(owner, function)?;
        for (binding, formal, exit, value, binary) in source.integer_return_uses()? {
            let mut guard_compare = None;
            self.with_borrowed_ordinary_compares_v1(owner, function, |loan, _, finished| {
                if loan.site() == &binary {
                    if !loan.operand_formals().any(|(operand, _)| operand == formal)
                        || guard_compare.is_some()
                    {
                        return Err(freeze("borrowed-return/compare-correspondence"));
                    }
                    guard_compare = Some((
                        find_finished_producer(&function.signature.name, finished, function)?,
                        finished.clone(),
                    ));
                }
                Ok(())
            })?;
            let guard_compare =
                guard_compare.ok_or_else(|| freeze("borrowed-return/compare-missing"))?;
            // Independent expected operand from the original source binding, not Return itself.
            let expected_value = if binding == formal {
                self.borrowed_ordinary_entry_values_v1(owner)?
                    .iter()
                    .find(|(_, root, _)| *root == formal)
                    .map(|(_, _, value)| *value)
                    .ok_or_else(|| freeze("borrowed-return/entry-formal-missing"))?
            } else {
                let mut value = None;
                self.with_borrowed_ordinary_alias_copies_v1(
                    owner,
                    function,
                    |_, _, alias, root, original_value, _| {
                        if alias == binding {
                            if root != formal || value.replace(original_value).is_some() {
                                return Err(freeze("borrowed-return/alias-correspondence"));
                            }
                        }
                        Ok(())
                    },
                )?;
                value.ok_or_else(|| freeze("borrowed-return/alias-missing"))?
            };
            let original = {
                let exits = self.ledger.root_exits.borrow();
                let Some(RootHomeExitProgress::Emitted {
                    bindings,
                    entry: RootHomeExitEntry::Plain { .. },
                    ..
                }) = exits.get(&(owner, exit.clone()))
                else {
                    return Err(freeze("borrowed-return/original-exit-missing"));
                };
                let mut returns = bindings.iter().filter(|(_, instruction)| {
                    matches!(instruction, MirInstruction::Return { value: Some(_) })
                });
                let original = returns
                    .next()
                    .ok_or_else(|| freeze("borrowed-return/original-return-missing"))?;
                if returns.next().is_some() {
                    return Err(freeze("borrowed-return/original-return-duplicate"));
                }
                original.clone()
            };
            if !matches!(original.1, MirInstruction::Return { value: Some(value) } if value == expected_value)
            {
                return Err(freeze("borrowed-return/source-binding-operand"));
            }
            let (symbol, finished) = self.ledger.finished_binding_for_owner(owner, &original)?;
            let coordinate = find_finished_producer(&symbol, &finished, function)?;
            if !matches!(finished.1, MirInstruction::Return { value: Some(_) }) {
                return Err(freeze("borrowed-return/finished-return-kind"));
            }
            visit(
                binding,
                formal,
                &value,
                &finished,
                coordinate,
                guard_compare.0,
                &guard_compare.1,
            )?;
        }
        Ok(())
    }
}
