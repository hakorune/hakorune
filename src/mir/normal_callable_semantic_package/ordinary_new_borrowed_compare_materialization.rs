//! Original checked comparison source and append; final SSA proof remains owed.
use super::super::super::borrowed_formal_uses::BorrowedFormalUseDraftKindV1 as Use;
use super::*;
use crate::mir::dynamic_operator_contract::{
    DynamicOperatorFamilyV1 as Family, DynamicOperatorNormalResultV1 as Normal,
    DynamicOperatorValueClassV1 as Class, VerifiedDynamicOperatorExecutionEnvelopeV1,
};
use crate::mir::resolved_semantics::ResolvedBinaryOperatorV1 as Operator;
use crate::mir::{BasicBlockId, CompareOp, MirInstruction};

#[derive(Debug, PartialEq, Eq)]
struct SourceWitness {
    binding: BindingRefV1,
    formal: BindingRefV1,
    envelope: &'static VerifiedDynamicOperatorExecutionEnvelopeV1,
}

/// Owns the existing source receipts across ordered child descent. Not replay
/// permission, final operand correspondence, or proof Normal has executed.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct BorrowedCompareSourceLoanV1 {
    binary: OwnedExprSiteV1,
    left: OwnedExprSiteV1,
    right: OwnedExprSiteV1,
    operator: CompareOp,
    witnesses: Box<[SourceWitness]>,
}
impl BorrowedCompareSourceLoanV1 {
    pub(crate) fn owner(&self) -> FunctionOwnerIdV1 {
        self.binary.owner()
    }
    pub(crate) fn site(&self) -> &OwnedExprSiteV1 {
        &self.binary
    }
    pub(in crate::mir) fn operand_sites(&self) -> (&OwnedExprSiteV1, &OwnedExprSiteV1) {
        (&self.left, &self.right)
    }
    pub(in crate::mir) fn operator(&self) -> CompareOp {
        self.operator
    }
}

#[derive(Debug)]
pub(super) struct BorrowedCompareMaterializationV1 {
    source: BorrowedCompareSourceLoanV1,
    children: (ValueId, ValueId),
    original: (BasicBlockId, MirInstruction),
}

impl OrdinaryNewClaimLedgerV1 {
    /// A probe selects original source inventory; it grants no validation.
    pub(crate) fn has_borrowed_compare_source_v1(&self, owner: FunctionOwnerIdV1) -> bool {
        self.borrowed_formal_source
            .as_ref()
            .and_then(|source| source.as_ref().ok())
            .and_then(|source| source.definitions.get(&owner))
            .is_some_and(|definition| {
                definition
                    .uses
                    .iter()
                    .any(|row| matches!(&row.kind, Use::CompareOperand { .. }))
            })
    }

    pub(crate) fn prepare_borrowed_compare_source_v1(
        &self,
        owner: FunctionOwnerIdV1,
        site: &OwnedExprSiteV1,
        operator: Option<CompareOp>,
    ) -> Result<Option<BorrowedCompareSourceLoanV1>, String> {
        if site.owner() != owner {
            return Err(freeze("borrowed-compare/source-owner"));
        }
        if !self.has_borrowed_compare_source_v1(owner) {
            return Ok(None);
        }
        let source = self
            .borrowed_formal_source
            .as_ref()
            .ok_or_else(|| freeze("borrowed-compare/source-missing"))?
            .as_ref()
            .map_err(Clone::clone)?;
        let definition = source
            .definitions
            .get(&owner)
            .ok_or_else(|| freeze("borrowed-compare/owner"))?;
        let mut selected: Option<BorrowedCompareSourceLoanV1> = None;
        let mut witnesses = Vec::new();
        for row in &definition.uses {
            let Use::CompareOperand { binary, source } = &row.kind else {
                continue;
            };
            if binary != site {
                continue;
            }
            let (source_operator, left, right, envelope) = source.comparison_parts();
            let (expected, family) = match source_operator {
                Operator::Greater => (CompareOp::Gt, Family::Greater),
                Operator::LessEqual => (CompareOp::Le, Family::LessEqual),
                _ => return Err(freeze("borrowed-compare/source-operator")),
            };
            let domain = envelope.domain();
            if operator != Some(expected)
                || left.owner() != owner
                || right.owner() != owner
                || definition.origins.get(&row.binding) != Some(&row.formal)
                || domain.family() != family
                || domain.left() != Class::NormalInteger
                || domain.right() != Class::NormalInteger
                || envelope.normal_result() != Normal::TrivialBool
            {
                return Err(freeze("borrowed-compare/source-identity"));
            }
            if let Some(previous) = &selected {
                if previous.left != *left
                    || previous.right != *right
                    || previous.operator != expected
                {
                    return Err(freeze("borrowed-compare/source-ambiguous"));
                }
            } else {
                selected = Some(BorrowedCompareSourceLoanV1 {
                    binary: binary.clone(),
                    left: left.clone(),
                    right: right.clone(),
                    operator: expected,
                    witnesses: Box::new([]),
                });
            }
            // Each operand row keeps its own original receipt. Each row
            // borrows the same original issuer reference without reissuing it.
            witnesses.push(SourceWitness {
                binding: row.binding,
                formal: row.formal,
                envelope: envelope,
            });
        }
        if let Some(selected) = &mut selected {
            let values = self.borrowed_ordinary_entry_values_v1(owner)?;
            self.check_borrowed_ordinary_entry_values_v1(owner, &Ok(values))?;
            selected.witnesses = witnesses.into_boxed_slice();
        }
        Ok(selected)
    }

    fn check_borrowed_compare_source_loan(
        &self,
        loan: &BorrowedCompareSourceLoanV1,
    ) -> Result<(), String> {
        let current = self
            .prepare_borrowed_compare_source_v1(loan.owner(), &loan.binary, Some(loan.operator))?
            .ok_or_else(|| freeze("borrowed-compare/source-membership"))?;
        if current != *loan
            || !current
                .witnesses
                .iter()
                .zip(loan.witnesses.iter())
                .all(|(current, original)| std::ptr::eq(current.envelope, original.envelope))
        {
            return Err(freeze("borrowed-compare/source-membership"));
        }
        Ok(())
    }

    /// Pre-finalization children are observations only; they are not equated
    /// to finalized Compare operands without the original LocalSSA lineage.
    pub(in crate::mir) fn record_borrowed_compare_v1(
        &self,
        loan: BorrowedCompareSourceLoanV1,
        children: (ValueId, ValueId),
        completed: &crate::mir::builder::ops::CompletedOrdinaryBinaryV1,
    ) -> Result<(), String> {
        self.check_borrowed_compare_source_loan(&loan)?;
        let value = completed.value();
        let original = completed
            .comparison_original()
            .ok_or_else(|| freeze("borrowed-compare/completion-not-compare"))?;
        if !matches!(&original.1, MirInstruction::Compare { dst, op, .. }
            if *dst == value && *op == loan.operator)
        {
            return Err(freeze("borrowed-compare/physical-identity"));
        }
        let mut entries = self.borrowed_entry_values.borrow_mut();
        let entry = entries
            .get_mut(&loan.owner())
            .ok_or_else(|| freeze("borrowed-compare/entry-missing"))?;
        if entry.comparisons.contains_key(&value) {
            return Err(freeze("borrowed-compare/duplicate-materialization"));
        }
        entry.comparisons.insert(
            value,
            BorrowedCompareMaterializationV1 {
                source: loan,
                children,
                original: original.clone(),
            },
        );
        Ok(())
    }

    /// Source + original append observations only. Callback child values are
    /// pre-finalization outcomes, not a proof of the final SSA operand lineage.
    pub(in crate::mir) fn with_borrowed_ordinary_compares_v1(
        &self,
        owner: FunctionOwnerIdV1,
        mut visit: impl FnMut(
            &BorrowedCompareSourceLoanV1,
            (ValueId, ValueId),
            &(BasicBlockId, MirInstruction),
        ) -> Result<(), String>,
    ) -> Result<(), String> {
        let entries = self.borrowed_entry_values.borrow();
        let entry = entries
            .get(&owner)
            .ok_or_else(|| freeze("borrowed-compare/entry-missing"))?;
        self.check_borrowed_ordinary_entry_values_v1(owner, &entry.values)?;
        for (value, record) in &entry.comparisons {
            self.check_borrowed_compare_source_loan(&record.source)?;
            if !matches!(&record.original.1, MirInstruction::Compare { dst, op, .. }
                if dst == value && *op == record.source.operator)
            {
                return Err(freeze("borrowed-compare/physical-identity"));
            }
            visit(&record.source, record.children, &record.original)?;
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "ordinary_new_borrowed_compare_materialization_tests.rs"]
mod tests;
