//! Source-selected carrier identity, borrowed from the original entry/alias.
use super::*;
use std::rc::Rc;

/// No Integer projection or new Copy permission: this loan keeps the original
/// source alias record and formal root until the physical consumer verifies it.
#[derive(Debug)]
pub(in crate::mir) struct BorrowedCompareCarrierOperandLoanV1 {
    binary: OwnedExprSiteV1,
    site: OwnedExprSiteV1,
    binding: BindingRefV1,
    formal: BindingRefV1,
    entry_value: ValueId,
    value: ValueId,
    alias: Option<Rc<BorrowedAliasMaterializationV1>>,
}
impl BorrowedCompareCarrierOperandLoanV1 {
    pub(in crate::mir) fn owner(&self) -> FunctionOwnerIdV1 {
        self.binary.owner()
    }
    pub(in crate::mir) fn site(&self) -> &OwnedExprSiteV1 {
        &self.site
    }
    pub(in crate::mir) fn binary(&self) -> &OwnedExprSiteV1 {
        &self.binary
    }
    pub(in crate::mir) fn value(&self) -> ValueId {
        self.value
    }
    pub(in crate::mir) fn entry_value(&self) -> ValueId {
        self.entry_value
    }
    pub(in crate::mir) fn original_copies(
        &self,
    ) -> Result<&[(BasicBlockId, MirInstruction)], String> {
        match &self.alias {
            Some(alias) => alias
                .proof
                .loan_copies(self.formal, self.entry_value, self.value),
            None if self.binding == self.formal && self.value == self.entry_value => Ok(&[]),
            None => Err(freeze("borrowed-compare/carrier-root")),
        }
    }
}
impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir) fn verify_compare_carrier_operand_loan_v1(
        &self,
        compare: &super::super::compare_materialization::BorrowedCompareSourceLoanV1,
        loan: &BorrowedCompareCarrierOperandLoanV1,
    ) -> Result<(), String> {
        let current = self.borrow_compare_carrier_operands_v1(compare)?;
        let current = current
            .iter()
            .find(|row| row.site == loan.site)
            .ok_or_else(|| freeze("borrowed-compare/carrier-membership"))?;
        if current.binary != loan.binary
            || current.binding != loan.binding
            || current.formal != loan.formal
            || current.entry_value != loan.entry_value
            || current.value != loan.value
            || match (&current.alias, &loan.alias) {
                (Some(current), Some(original)) => !Rc::ptr_eq(current, original),
                (None, None) => false,
                _ => true,
            }
        {
            return Err(freeze("borrowed-compare/carrier-identity"));
        }
        loan.original_copies()?;
        Ok(())
    }
    pub(in crate::mir) fn borrow_compare_carrier_operands_v1(
        &self,
        compare: &super::super::compare_materialization::BorrowedCompareSourceLoanV1,
    ) -> Result<Vec<BorrowedCompareCarrierOperandLoanV1>, String> {
        self.check_borrowed_compare_source_loan(compare)?;
        let owner = compare.owner();
        let source = self
            .borrowed_formal_source
            .as_ref()
            .ok_or_else(|| freeze("borrowed-compare/carrier-source"))?
            .as_ref()
            .map_err(Clone::clone)?;
        let definition = source
            .definitions
            .get(&owner)
            .ok_or_else(|| freeze("borrowed-compare/carrier-owner"))?;
        let entries = self.borrowed_entry_values.borrow();
        let entry = entries
            .get(&owner)
            .ok_or_else(|| freeze("borrowed-compare/carrier-entry"))?;
        self.check_borrowed_ordinary_entry_values_v1(owner, &entry.values)?;
        let values = entry.values.as_ref().map_err(Clone::clone)?;
        let mut result = Vec::new();
        for row in &definition.uses {
            let Use::CompareOperand { binary, .. } = &row.kind else {
                continue;
            };
            if binary != compare.site() {
                continue;
            }
            let (left, right) = compare.operand_sites();
            if &row.site != left && &row.site != right {
                return Err(freeze("borrowed-compare/carrier-side"));
            }
            let formal_value = values
                .iter()
                .find(|(_, binding, _)| *binding == row.formal)
                .ok_or_else(|| freeze("borrowed-compare/carrier-formal"))?
                .2;
            let alias = if row.binding == row.formal {
                None
            } else {
                let alias = entry
                    .aliases
                    .get(&row.binding)
                    .ok_or_else(|| freeze("borrowed-compare/carrier-alias-missing"))?;
                self.check_borrowed_alias_source_v1(
                    owner,
                    &alias.site,
                    &alias.declaration,
                    row.binding,
                    alias.source_binding,
                    alias.formal,
                )?;
                if alias.formal != row.formal {
                    return Err(freeze("borrowed-compare/carrier-formal"));
                }
                Some(Rc::clone(alias))
            };
            let value = alias.as_ref().map_or(formal_value, |alias| alias.value);
            let loan = BorrowedCompareCarrierOperandLoanV1 {
                binary: binary.clone(),
                site: row.site.clone(),
                binding: row.binding,
                formal: row.formal,
                entry_value: formal_value,
                value,
                alias,
            };
            loan.original_copies()?;
            if result
                .iter()
                .any(|previous: &BorrowedCompareCarrierOperandLoanV1| previous.site == loan.site)
            {
                return Err(freeze("borrowed-compare/carrier-duplicate"));
            }
            result.push(loan);
        }
        if result.is_empty() {
            return Err(freeze("borrowed-compare/carrier-missing"));
        }
        Ok(result)
    }
}
