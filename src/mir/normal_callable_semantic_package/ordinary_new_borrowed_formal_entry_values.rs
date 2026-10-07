//! Physical formal correspondence retained by the original ordinary ledger.
use super::*;
use crate::mir::ValueId;

type EntryValues = Box<[(u32, BindingRefV1, ValueId)]>;

/// Physical observations of this already-selected entry, not source products.
#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package) struct BorrowedOrdinaryEntryPhysicalV1 {
    pub(in crate::mir::normal_callable_semantic_package) values: Result<EntryValues, String>,
    aliases: BTreeMap<BindingRefV1, alias_materialization::BorrowedAliasMaterializationV1>,
    comparisons: BTreeMap<ValueId, std::rc::Rc<compare_materialization::BorrowedCompareMaterializationV1>>,
    comparison_consumers: Option<BTreeMap<ValueId, (Vec<(crate::mir::BasicBlockId, crate::mir::MirInstruction)>, Vec<(crate::mir::BasicBlockId, crate::mir::MirInstruction)>)>>,
    literal_consumers: Option<BTreeMap<ValueId, Vec<(crate::mir::BasicBlockId, crate::mir::MirInstruction)>>>,
    integer_literals:
        BTreeMap<ValueId, std::rc::Rc<integer_literal::BorrowedCompareIntegerLiteralMaterializationV1>>,
}

#[path = "ordinary_new_borrowed_alias_materialization.rs"]
mod alias_materialization;

#[path = "ordinary_new_borrowed_compare_integer_literal.rs"]
mod integer_literal;
pub(crate) use integer_literal::BorrowedCompareIntegerLiteralLoanV1;
pub(in crate::mir) use integer_literal::BorrowedCompareIntegerLiteralMaterializationV1;

#[path = "ordinary_new_borrowed_compare_materialization.rs"]
mod compare_materialization;
pub(crate) use compare_materialization::BorrowedCompareSourceLoanV1;
pub(in crate::mir) use compare_materialization::BorrowedCompareMaterializationV1;

#[path = "ordinary_new_borrowed_compare_consumers.rs"]
mod compare_consumers;
#[path = "ordinary_new_borrowed_compare_literal_consumers.rs"]
mod literal_consumers;

impl OrdinaryNewClaimLedgerV1 {
    /// Loan the original declared classes for exactly the validated pre-entry
    /// rows. Inferred opaque object views grant no declared-header authority.
    pub(crate) fn borrowed_ordinary_entry_declared_classes_v1(
        &self,
        owner: FunctionOwnerIdV1,
        values: &Result<EntryValues, String>,
    ) -> Result<BTreeMap<BindingRefV1, &str>, String> {
        self.validate_borrowed_ordinary_entry_values_v1(owner, values)?;
        let rows = values.as_ref().map_err(Clone::clone)?;
        let source = self
            .borrowed_formal_source
            .as_ref()
            .ok_or_else(|| freeze("borrowed-entry/source-missing"))?
            .as_ref()
            .map_err(Clone::clone)?;
        Ok(rows
            .iter()
            .filter_map(|(_, binding, _)| {
                source
                    .formal_object_view(*binding)
                    .filter(|view| view.is_declared())
                    .map(|view| (*binding, view.class()))
            })
            .collect())
    }

    pub(crate) fn record_borrowed_ordinary_entry_values_v1(
        &self,
        owner: FunctionOwnerIdV1,
        values: Result<EntryValues, String>,
    ) -> Result<(), String> {
        self.validate_borrowed_ordinary_entry_values_v1(owner, &values)?;
        let mut recorded = self.borrowed_entry_values.borrow_mut();
        if recorded.contains_key(&owner) {
            return Err(freeze("borrowed-entry/duplicate-entry-values"));
        }
        recorded.insert(
            owner,
            BorrowedOrdinaryEntryPhysicalV1 {
                values,
                aliases: BTreeMap::new(),
                comparisons: BTreeMap::new(),
                comparison_consumers: None,
                literal_consumers: None,
                integer_literals: BTreeMap::new(),
            },
        );
        Ok(())
    }

    pub(crate) fn validate_borrowed_ordinary_entry_values_v1(
        &self,
        owner: FunctionOwnerIdV1,
        values: &Result<EntryValues, String>,
    ) -> Result<(), String> {
        if self.borrowed_entry_values.borrow().contains_key(&owner) {
            return Err(freeze("borrowed-entry/duplicate-entry-values"));
        }
        self.check_borrowed_ordinary_entry_values_v1(owner, values)
    }

    // The final source loan checks the same correspondence without repeating
    // the record-only duplicate-state guard.
    pub(super) fn check_borrowed_ordinary_entry_values_v1(
        &self,
        owner: FunctionOwnerIdV1,
        values: &Result<EntryValues, String>,
    ) -> Result<(), String> {
        if let Ok(rows) = values {
            let source = self
                .borrowed_formal_source
                .as_ref()
                .ok_or_else(|| freeze("borrowed-entry/source-missing"))?
                .as_ref()
                .map_err(Clone::clone)?;
            let definition = source
                .definitions
                .get(&owner)
                .ok_or_else(|| freeze("borrowed-entry/entry-owner"))?;
            let roots: BTreeSet<_> = definition.origins.values().copied().collect();
            if roots != rows.iter().map(|(_, binding, _)| *binding).collect()
                || rows.len() != roots.len()
                || rows.iter().any(|(_, binding, _)| binding.owner() != owner)
                || rows.windows(2).any(|pair| pair[0].0 >= pair[1].0)
                || rows
                    .iter()
                    .map(|(_, _, value)| *value)
                    .collect::<BTreeSet<_>>()
                    .len()
                    != rows.len()
            {
                return Err(freeze("borrowed-entry/entry-values-identity"));
            }
            self.checked_borrowed_entry_incoming(source, owner)?;
            for call in source.incoming.iter().filter(|call| call.callee == owner) {
                if call.arguments.len() != rows.len()
                    || call.arguments.iter().zip(rows.iter()).any(
                        |((ordinal, _, binding), (observed, formal, _))| {
                            ordinal != observed || binding != formal
                        },
                    )
                {
                    return Err(freeze("borrowed-entry/entry-values-source-drift"));
                }
            }
        }
        Ok(())
    }

    /// An actual borrowed consumer must demand this Result. Absence and Err
    /// never mean an untagged ABI, and metadata does not prove correspondence.
    pub(crate) fn borrowed_ordinary_entry_values_v1(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> Result<EntryValues, String> {
        self.borrowed_entry_values
            .borrow()
            .get(&owner)
            .ok_or_else(|| freeze("borrowed-entry/entry-values-missing"))?
            .values
            .clone()
    }

    /// Membership probe for the closed transport profile. Consumers that
    /// enumerate every birth caller — including owners outside the
    /// profile — use this to skip non-borrowed owners instead of decoding
    /// the `entry-values-missing` freeze.
    pub(crate) fn has_borrowed_ordinary_entry_v1(&self, owner: FunctionOwnerIdV1) -> bool {
        self.borrowed_entry_values.borrow().contains_key(&owner)
    }
}
