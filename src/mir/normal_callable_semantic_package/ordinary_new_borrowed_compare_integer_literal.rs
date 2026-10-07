//! Exact original compare-literal materialization on the same borrowed entry.
use super::super::super::borrowed_formal_uses::BorrowedFormalUseDraftKindV1 as Use;
use super::*;
use crate::mir::builder::emission::constant::CompletedConstV1;
use crate::mir::{BasicBlockId, ConstValue, MirInstruction};

/// Source membership only, consumed once by one original literal append.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct BorrowedCompareIntegerLiteralLoanV1 {
    binary: OwnedExprSiteV1,
    site: OwnedExprSiteV1,
    value: i64,
}
impl BorrowedCompareIntegerLiteralLoanV1 {
    pub(crate) fn owner(&self) -> FunctionOwnerIdV1 {
        self.site.owner()
    }
    pub(crate) fn site(&self) -> &OwnedExprSiteV1 {
        &self.site
    }
}

#[derive(Debug)]
pub(super) struct BorrowedCompareIntegerLiteralMaterializationV1 {
    source: BorrowedCompareIntegerLiteralLoanV1,
    original: (BasicBlockId, MirInstruction),
}

impl OrdinaryNewClaimLedgerV1 {
    /// Selection probe only; a known descriptor still demands its entry below.
    pub(crate) fn has_borrowed_compare_integer_literal_source_v1(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> bool {
        self.borrowed_formal_source.as_ref().and_then(|source| source.as_ref().ok())
            .and_then(|source| source.definitions.get(&owner))
            .is_some_and(|definition| definition.uses.iter().any(|row| {
                matches!(&row.kind, Use::CompareOperand { source, .. } if source.integer_literal().is_some())
            }))
    }

    pub(crate) fn prepare_borrowed_compare_integer_literal_v1(
        &self,
        owner: FunctionOwnerIdV1,
        site: &OwnedExprSiteV1,
        value: i64,
    ) -> Result<Option<BorrowedCompareIntegerLiteralLoanV1>, String> {
        if site.owner() != owner {
            return Err(freeze("borrowed-literal/source-owner"));
        }
        if !self.has_borrowed_compare_integer_literal_source_v1(owner) {
            return Ok(None);
        }
        let source = self
            .borrowed_formal_source
            .as_ref()
            .ok_or_else(|| freeze("borrowed-literal/source-missing"))?
            .as_ref()
            .map_err(Clone::clone)?;
        let definition = source
            .definitions
            .get(&owner)
            .ok_or_else(|| freeze("borrowed-literal/owner"))?;
        let mut selected = None;
        for row in &definition.uses {
            let Use::CompareOperand { binary, source } = &row.kind else {
                continue;
            };
            let Some((literal_site, literal_value)) = source.integer_literal() else {
                continue;
            };
            if literal_site != site {
                continue;
            }
            if literal_value != value
                || binary.owner() != owner
                || definition.origins.get(&row.binding) != Some(&row.formal)
            {
                return Err(freeze("borrowed-literal/source-identity"));
            }
            let loan = BorrowedCompareIntegerLiteralLoanV1 {
                binary: binary.clone(),
                site: site.clone(),
                value,
            };
            if selected.as_ref().is_some_and(|previous| previous != &loan) {
                return Err(freeze("borrowed-literal/source-ambiguous"));
            }
            selected = Some(loan);
        }
        if selected.is_some() {
            let values = self.borrowed_ordinary_entry_values_v1(owner)?;
            self.check_borrowed_ordinary_entry_values_v1(owner, &Ok(values))?;
        }
        Ok(selected)
    }

    pub(crate) fn record_borrowed_compare_integer_literal_v1(
        &self,
        loan: BorrowedCompareIntegerLiteralLoanV1,
        completed: &CompletedConstV1,
    ) -> Result<(), String> {
        self.check_compare_integer_literal_loan(&loan)?;
        let original = completed.original();
        if !matches!(&original.1, MirInstruction::Const { dst, value: ConstValue::Integer(value) }
            if *dst == completed.value() && *value == loan.value)
        {
            return Err(freeze("borrowed-literal/physical-identity"));
        }
        let mut entries = self.borrowed_entry_values.borrow_mut();
        let entry = entries
            .get_mut(&loan.owner())
            .ok_or_else(|| freeze("borrowed-literal/entry-missing"))?;
        if entry.integer_literals.contains_key(&completed.value()) {
            return Err(freeze("borrowed-literal/duplicate-materialization"));
        }
        entry.integer_literals.insert(
            completed.value(),
            BorrowedCompareIntegerLiteralMaterializationV1 {
                source: loan,
                original: original.clone(),
            },
        );
        Ok(())
    }

    fn check_compare_integer_literal_loan(
        &self,
        loan: &BorrowedCompareIntegerLiteralLoanV1,
    ) -> Result<(), String> {
        if self
            .prepare_borrowed_compare_integer_literal_v1(loan.owner(), &loan.site, loan.value)?
            .as_ref()
            != Some(loan)
        {
            return Err(freeze("borrowed-literal/source-membership"));
        }
        Ok(())
    }

    /// Original append observations only; consumers still owe final SSA mapping.
    pub(in crate::mir) fn with_borrowed_ordinary_compare_integer_literals_v1(
        &self,
        owner: FunctionOwnerIdV1,
        mut visit: impl FnMut(
            &OwnedExprSiteV1,
            &OwnedExprSiteV1,
            ValueId,
            &(BasicBlockId, MirInstruction),
        ) -> Result<(), String>,
    ) -> Result<(), String> {
        let entries = self.borrowed_entry_values.borrow();
        let entry = entries
            .get(&owner)
            .ok_or_else(|| freeze("borrowed-literal/entry-missing"))?;
        self.check_borrowed_ordinary_entry_values_v1(owner, &entry.values)?;
        for (value, record) in &entry.integer_literals {
            self.check_compare_integer_literal_loan(&record.source)?;
            if record.source.owner() != owner
                || !matches!(&record.original.1, MirInstruction::Const { dst, value: ConstValue::Integer(integer) }
                    if dst == value && *integer == record.source.value)
            {
                return Err(freeze("borrowed-literal/physical-identity"));
            }
            visit(
                &record.source.binary,
                &record.source.site,
                *value,
                &record.original,
            )?;
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "ordinary_new_borrowed_compare_integer_literal_tests.rs"]
mod tests;
