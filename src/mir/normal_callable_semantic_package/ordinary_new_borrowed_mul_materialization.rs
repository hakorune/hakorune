//! SAME original Mul source and actual append, retained by the entry owner.
//! Child values are observations; producer/Copy/finishing proof remains owed.
use super::super::super::borrowed_formal_uses::{
    BorrowedFormalUseDraftKindV1 as Use, BorrowedMulSideV1, BorrowedMulSourceV1,
};
use super::*;
use crate::mir::{BasicBlockId, BinaryOp as BinOp, MirInstruction};
use std::rc::Rc;

#[derive(Debug)]
pub(in crate::mir) struct BorrowedMulSourceLoanV1 {
    original: Rc<BorrowedMulSourceV1>,
}

impl BorrowedMulSourceLoanV1 {
    pub(in crate::mir) fn owner(&self) -> FunctionOwnerIdV1 {
        self.original.binary().owner()
    }
    pub(in crate::mir) fn site(&self) -> &OwnedExprSiteV1 {
        self.original.binary()
    }
    pub(in crate::mir) fn operand_sites(&self) -> (&OwnedExprSiteV1, &OwnedExprSiteV1) {
        self.original.operand_sites()
    }
    pub(super) fn original(&self) -> &Rc<BorrowedMulSourceV1> {
        &self.original
    }
    pub(super) fn view_binding(
        &self,
        side: BorrowedMulSideV1,
    ) -> Option<(BindingRefV1, BindingRefV1, &OwnedExprSiteV1)> {
        self.original.view_binding(side)
    }
}

#[derive(Debug)]
pub(in crate::mir) struct BorrowedMulMaterializationV1 {
    source: BorrowedMulSourceLoanV1,
    children: (ValueId, ValueId),
    original: (BasicBlockId, MirInstruction),
}

impl BorrowedMulMaterializationV1 {
    pub(super) fn source(&self) -> &BorrowedMulSourceLoanV1 {
        &self.source
    }
    pub(super) fn children(&self) -> (ValueId, ValueId) {
        self.children
    }
    pub(in crate::mir) fn owner(&self) -> FunctionOwnerIdV1 {
        self.source.owner()
    }
    pub(in crate::mir) fn value(&self) -> ValueId {
        self.original.1.dst_value().expect("sealed Mul destination")
    }
    pub(in crate::mir) fn original(&self) -> &(BasicBlockId, MirInstruction) {
        &self.original
    }
}

impl OrdinaryNewClaimLedgerV1 {
    /// Preserve each issued Mul append across the physical finishing boundary.
    /// Operand Copies are not admitted by this binding alone.
    pub(in crate::mir::normal_callable_semantic_package) fn borrowed_mul_bindings_v1(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> Result<Vec<(BasicBlockId, MirInstruction)>, String> {
        let entries = self.borrowed_entry_values.borrow();
        self.verify_borrowed_mul_reuse_v1(
            owner,
            entries
                .get(&owner)
                .into_iter()
                .flat_map(|entry| entry.multiplications.values()),
        )?;
        Ok(entries
            .get(&owner)
            .into_iter()
            .flat_map(|entry| entry.multiplications.values())
            .map(|record| record.original().clone())
            .collect())
    }

    /// Match the SAME original append to a mandatory finished instruction.
    /// Final operand lineage and guard dominance remain separate obligations.
    pub(in crate::mir::normal_callable_semantic_package) fn verify_finished_borrowed_muls_v1(
        &self,
        owner: FunctionOwnerIdV1,
        function: &crate::mir::MirFunction,
        mut project: impl FnMut(
            &(BasicBlockId, MirInstruction),
        ) -> Result<(BasicBlockId, MirInstruction), String>,
    ) -> Result<(), String> {
        for original in self.borrowed_mul_bindings_v1(owner)? {
            let finished = project(&original)?;
            if !matches!(&finished.1, MirInstruction::BinOp { op: BinOp::Mul, .. })
                || function.blocks.get(&finished.0).is_none_or(|block| {
                    block
                        .all_instructions()
                        .filter(|row| *row == &finished.1)
                        .count()
                        != 1
                })
            {
                return Err(freeze("borrowed-mul/finished-binding"));
            }
        }
        Ok(())
    }

    /// Both executable and source-only Mul inventory select the raw site;
    /// preparation alone decides whether execution is permitted.
    pub(crate) fn has_borrowed_mul_source_v1(&self, owner: FunctionOwnerIdV1) -> bool {
        self.borrowed_formal_source
            .as_ref()
            .and_then(|source| source.as_ref().ok())
            .is_some_and(|source| {
                source
                    .definitions
                    .get(&owner)
                    .into_iter()
                    .chain(source.source_only_definitions.get(&owner))
                    .any(|definition| {
                        definition
                            .uses
                            .iter()
                            .any(|row| matches!(&row.kind, Use::MulOperand { .. }))
                    })
            })
    }

    /// Only executable selected definitions may acquire this entry loan.
    pub(in crate::mir) fn prepare_borrowed_mul_source_v1(
        &self,
        owner: FunctionOwnerIdV1,
        site: &OwnedExprSiteV1,
        operator: Option<BinOp>,
    ) -> Result<Option<BorrowedMulSourceLoanV1>, String> {
        if site.owner() != owner {
            return Err(freeze("borrowed-mul/source-owner"));
        }
        let Some(source) = self.borrowed_formal_source.as_ref() else {
            return Ok(None);
        };
        let source = source.as_ref().map_err(Clone::clone)?;
        let Some(definition) = source.definitions.get(&owner) else {
            if source
                .source_only_definitions
                .get(&owner)
                .is_some_and(|definition| {
                    definition.uses.iter().any(
                        |row| matches!(&row.kind, Use::MulOperand { binary, .. } if binary == site),
                    )
                })
            {
                return Err(freeze("borrowed-mul/source-only-entry"));
            }
            return Ok(None);
        };
        let mut selected: Option<&Rc<BorrowedMulSourceV1>> = None;
        for row in &definition.uses {
            let Use::MulOperand {
                binary,
                source,
                side,
            } = &row.kind
            else {
                continue;
            };
            if binary != site {
                continue;
            }
            if operator != Some(BinOp::Mul)
                || source.binary() != site
                || !source.corroborates_retained_rows(definition)
                || !source
                    .view_binding(*side)
                    .is_some_and(|(binding, formal, original)| {
                        binding == row.binding && formal == row.formal && original == &row.site
                    })
                || selected.is_some_and(|original| !Rc::ptr_eq(original, source))
            {
                return Err(freeze("borrowed-mul/source-identity"));
            }
            selected = Some(source);
        }
        let Some(original) = selected else {
            return Ok(None);
        };
        let values = self.borrowed_ordinary_entry_values_v1(owner)?;
        self.check_borrowed_ordinary_entry_values_v1(owner, &Ok(values))?;
        Ok(Some(BorrowedMulSourceLoanV1 {
            original: Rc::clone(original),
        }))
    }

    pub(super) fn check_borrowed_mul_source_loan_v1(
        &self,
        loan: &BorrowedMulSourceLoanV1,
    ) -> Result<(), String> {
        let current = self
            .prepare_borrowed_mul_source_v1(loan.owner(), loan.site(), Some(BinOp::Mul))?
            .ok_or_else(|| freeze("borrowed-mul/source-membership"))?;
        if !Rc::ptr_eq(current.original(), loan.original()) {
            return Err(freeze("borrowed-mul/source-membership"));
        }
        Ok(())
    }

    /// No raw-child/final-operand equality is fabricated: LocalSSA may append
    /// mandatory Copies between them. Their actual lineage is checked later.
    pub(in crate::mir) fn record_borrowed_mul_v1(
        &self,
        loan: BorrowedMulSourceLoanV1,
        children: (ValueId, ValueId),
        completed: &crate::mir::builder::ops::CompletedOrdinaryBinaryV1,
    ) -> Result<Rc<BorrowedMulMaterializationV1>, String> {
        self.check_borrowed_mul_source_loan_v1(&loan)?;
        let value = completed.value();
        let original = completed
            .arithmetic_original()
            .ok_or_else(|| freeze("borrowed-mul/completion-not-arithmetic"))?;
        if !matches!(&original.1, MirInstruction::BinOp { dst, op: BinOp::Mul, .. } if *dst == value)
        {
            return Err(freeze("borrowed-mul/physical-identity"));
        }
        let mut entries = self.borrowed_entry_values.borrow_mut();
        let entry = entries
            .get_mut(&loan.owner())
            .ok_or_else(|| freeze("borrowed-mul/entry-missing"))?;
        if entry.multiplications.contains_key(&value)
            || entry
                .multiplications
                .values()
                .any(|record| record.source.site() == loan.site())
        {
            return Err(freeze("borrowed-mul/duplicate-materialization"));
        }
        let record = Rc::new(BorrowedMulMaterializationV1 {
            source: loan,
            children,
            original: original.clone(),
        });
        entry.multiplications.insert(value, Rc::clone(&record));
        Ok(record)
    }

    /// The original source inventory remains required even if observations are
    /// removed. Equal replacement records cannot substitute for the installed Rc.
    pub(in crate::mir) fn verify_borrowed_mul_reuse_v1<'a>(
        &self,
        owner: FunctionOwnerIdV1,
        installed: impl Iterator<Item = &'a Rc<BorrowedMulMaterializationV1>>,
    ) -> Result<(), String> {
        let installed: Vec<_> = installed.collect();
        let expected: BTreeSet<_> = match &self.borrowed_formal_source {
            Some(source) => source
                .as_ref()
                .map_err(Clone::clone)?
                .definitions
                .get(&owner)
                .map(|definition| {
                    definition
                        .uses
                        .iter()
                        .filter_map(|row| match &row.kind {
                            Use::MulOperand { binary, .. } => Some(binary.clone()),
                            _ => None,
                        })
                        .collect()
                })
                .unwrap_or_default(),
            None => BTreeSet::new(),
        };
        if expected.is_empty()
            && installed.is_empty()
            && self
                .borrowed_entry_values
                .borrow()
                .get(&owner)
                .is_none_or(|entry| entry.multiplications.is_empty())
        {
            return Ok(());
        }
        let entries = self.borrowed_entry_values.borrow();
        let entry = entries
            .get(&owner)
            .ok_or_else(|| freeze("borrowed-mul/entry-missing"))?;
        self.check_borrowed_ordinary_entry_values_v1(owner, &entry.values)?;
        let observed: BTreeSet<_> = entry
            .multiplications
            .values()
            .map(|record| record.source.site().clone())
            .collect();
        if expected != observed
            || observed.len() != entry.multiplications.len()
            || installed.len() != entry.multiplications.len()
        {
            return Err(freeze("borrowed-mul/reuse-coverage"));
        }
        let mut values = BTreeSet::new();
        for record in installed {
            self.check_borrowed_mul_source_loan_v1(&record.source)?;
            if record.owner() != owner
                || !values.insert(record.value())
                || !matches!(&record.original.1, MirInstruction::BinOp { dst, op: BinOp::Mul, .. } if *dst == record.value())
                || !entry
                    .multiplications
                    .get(&record.value())
                    .is_some_and(|original| Rc::ptr_eq(original, record))
            {
                return Err(freeze("borrowed-mul/reuse-identity"));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "ordinary_new_borrowed_mul_materialization_tests.rs"]
mod tests;
