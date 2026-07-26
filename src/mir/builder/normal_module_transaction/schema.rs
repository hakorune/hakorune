//! Canonical role/key/entry correspondence before any draft exists.

use std::collections::BTreeSet;

use crate::mir::builder::module_draft_collector::FunctionDraftKeyV1;
use crate::mir::canonical_physical_drain::CanonicalInsertedDispositionV1;
use crate::mir::resolved_semantics::{CanonicalCallableKeyV1, FunctionOwnerIdV1};

use super::rejection::{
    NormalModuleTransactionSchemaErrorV1, RejectedNormalModuleTransactionSchemaV1,
};

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum NormalModuleDraftRoleV1 {
    SourceMain { owner: FunctionOwnerIdV1 },
    Helper { key: CanonicalCallableKeyV1 },
    PhysicalEntry,
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) struct NormalModuleDraftExpectationV1 {
    role: NormalModuleDraftRoleV1,
    key: FunctionDraftKeyV1,
    symbol: Box<str>,
    arity: usize,
    disposition: CanonicalInsertedDispositionV1,
}

impl NormalModuleDraftExpectationV1 {
    pub(in crate::mir::builder) fn source_main(
        owner: FunctionOwnerIdV1,
        symbol: impl Into<Box<str>>,
        arity: usize,
    ) -> Self {
        Self::new(
            NormalModuleDraftRoleV1::SourceMain { owner },
            FunctionDraftKeyV1::CanonicalResolvedOwner(owner),
            symbol,
            arity,
        )
    }

    pub(in crate::mir::builder) fn helper(
        key: CanonicalCallableKeyV1,
        symbol: impl Into<Box<str>>,
        arity: usize,
    ) -> Self {
        Self::new(
            NormalModuleDraftRoleV1::Helper { key: key.clone() },
            FunctionDraftKeyV1::CanonicalCallable(key),
            symbol,
            arity,
        )
    }

    pub(in crate::mir::builder) fn physical_entry(
        symbol: impl Into<Box<str>>,
        arity: usize,
    ) -> Self {
        Self::new(
            NormalModuleDraftRoleV1::PhysicalEntry,
            FunctionDraftKeyV1::Main,
            symbol,
            arity,
        )
    }

    fn new(
        role: NormalModuleDraftRoleV1,
        key: FunctionDraftKeyV1,
        symbol: impl Into<Box<str>>,
        arity: usize,
    ) -> Self {
        Self {
            role,
            key,
            symbol: symbol.into(),
            arity,
            disposition: CanonicalInsertedDispositionV1::from_canonical_source(),
        }
    }

    pub(in crate::mir::builder) fn role(&self) -> &NormalModuleDraftRoleV1 {
        &self.role
    }

    pub(in crate::mir::builder) fn key(&self) -> &FunctionDraftKeyV1 {
        &self.key
    }

    pub(in crate::mir::builder) fn symbol(&self) -> &str {
        &self.symbol
    }

    pub(in crate::mir::builder) const fn arity(&self) -> usize {
        self.arity
    }

    #[cfg(test)]
    pub(super) fn from_unchecked_parts_for_test(
        role: NormalModuleDraftRoleV1,
        key: FunctionDraftKeyV1,
        symbol: impl Into<Box<str>>,
        arity: usize,
    ) -> Self {
        Self::new(role, key, symbol, arity)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) struct NormalModuleEntryRelationV1 {
    source_main_owner: FunctionOwnerIdV1,
    source_main_symbol: Box<str>,
    source_main_arity: usize,
    physical_symbol: Box<str>,
    physical_arity: usize,
}

impl NormalModuleEntryRelationV1 {
    pub(in crate::mir::builder) fn new(
        source_main_owner: FunctionOwnerIdV1,
        source_main_symbol: impl Into<Box<str>>,
        source_main_arity: usize,
        physical_symbol: impl Into<Box<str>>,
        physical_arity: usize,
    ) -> Self {
        Self {
            source_main_owner,
            source_main_symbol: source_main_symbol.into(),
            source_main_arity,
            physical_symbol: physical_symbol.into(),
            physical_arity,
        }
    }
}

#[derive(Debug)]
pub(in crate::mir::builder) struct NormalModuleTransactionDraftV1 {
    rows: Vec<NormalModuleDraftExpectationV1>,
    source_entry: NormalModuleEntryRelationV1,
}

impl NormalModuleTransactionDraftV1 {
    pub(in crate::mir::builder) fn new(
        rows: Vec<NormalModuleDraftExpectationV1>,
        source_entry: NormalModuleEntryRelationV1,
    ) -> Self {
        Self { rows, source_entry }
    }

    #[cfg(test)]
    pub(super) fn rows(&self) -> &[NormalModuleDraftExpectationV1] {
        &self.rows
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) struct NormalModuleTransactionSchemaV1 {
    rows: Box<[NormalModuleDraftExpectationV1]>,
    source_entry: NormalModuleEntryRelationV1,
    _seal: NormalModuleTransactionSchemaSealV1,
}

#[derive(Debug, PartialEq, Eq)]
struct NormalModuleTransactionSchemaSealV1;

impl NormalModuleTransactionSchemaV1 {
    pub(in crate::mir::builder) fn seal(
        draft: NormalModuleTransactionDraftV1,
    ) -> Result<Self, RejectedNormalModuleTransactionSchemaV1> {
        match validate(&draft) {
            Ok(()) => {
                let NormalModuleTransactionDraftV1 {
                    mut rows,
                    source_entry,
                } = draft;
                rows.sort_by(compare_rows);
                Ok(Self {
                    rows: rows.into_boxed_slice(),
                    source_entry,
                    _seal: NormalModuleTransactionSchemaSealV1,
                })
            }
            Err(error) => Err(RejectedNormalModuleTransactionSchemaV1 {
                owner: draft,
                error,
            }),
        }
    }

    pub(in crate::mir::builder) fn rows(&self) -> &[NormalModuleDraftExpectationV1] {
        &self.rows
    }

    pub(in crate::mir::builder) const fn source_entry(&self) -> &NormalModuleEntryRelationV1 {
        &self.source_entry
    }
}

fn validate(
    draft: &NormalModuleTransactionDraftV1,
) -> Result<(), NormalModuleTransactionSchemaErrorV1> {
    let source_main_count = draft
        .rows
        .iter()
        .filter(|row| matches!(row.role, NormalModuleDraftRoleV1::SourceMain { .. }))
        .count();
    match source_main_count {
        0 => return Err(NormalModuleTransactionSchemaErrorV1::MissingSourceMain),
        1 => {}
        _ => return Err(NormalModuleTransactionSchemaErrorV1::DuplicateSourceMain),
    }
    let physical_entry_count = draft
        .rows
        .iter()
        .filter(|row| matches!(row.role, NormalModuleDraftRoleV1::PhysicalEntry))
        .count();
    match physical_entry_count {
        0 => return Err(NormalModuleTransactionSchemaErrorV1::MissingPhysicalEntry),
        1 => {}
        _ => return Err(NormalModuleTransactionSchemaErrorV1::DuplicatePhysicalEntry),
    }

    let mut keys = BTreeSet::new();
    let mut symbols = BTreeSet::new();
    for row in &draft.rows {
        validate_role_key(row)?;
        if !keys.insert(row.key.clone()) {
            return Err(NormalModuleTransactionSchemaErrorV1::DuplicateKey(
                row.key.clone(),
            ));
        }
        if !symbols.insert(row.symbol.clone()) {
            return Err(NormalModuleTransactionSchemaErrorV1::DuplicateSymbol(
                row.symbol.clone(),
            ));
        }
    }
    validate_entry_relation(draft)
}

fn validate_role_key(
    row: &NormalModuleDraftExpectationV1,
) -> Result<(), NormalModuleTransactionSchemaErrorV1> {
    match (&row.role, &row.key) {
        (
            NormalModuleDraftRoleV1::SourceMain { owner },
            FunctionDraftKeyV1::CanonicalResolvedOwner(actual),
        ) => {
            if owner != actual {
                return Err(NormalModuleTransactionSchemaErrorV1::RoleKeyMismatch);
            }
            if row.arity != 0 {
                return Err(NormalModuleTransactionSchemaErrorV1::ArityMismatch);
            }
        }
        (
            NormalModuleDraftRoleV1::Helper { key },
            FunctionDraftKeyV1::CanonicalCallable(actual),
        ) => {
            if key != actual {
                return Err(NormalModuleTransactionSchemaErrorV1::RoleKeyMismatch);
            }
        }
        (NormalModuleDraftRoleV1::PhysicalEntry, FunctionDraftKeyV1::Main) => {
            if row.arity != 0 {
                return Err(NormalModuleTransactionSchemaErrorV1::ArityMismatch);
            }
        }
        _ => return Err(NormalModuleTransactionSchemaErrorV1::RoleKeyMismatch),
    }
    Ok(())
}

fn validate_entry_relation(
    draft: &NormalModuleTransactionDraftV1,
) -> Result<(), NormalModuleTransactionSchemaErrorV1> {
    let source_main = draft
        .rows
        .iter()
        .find(|row| matches!(row.role, NormalModuleDraftRoleV1::SourceMain { .. }))
        .expect("source Main cardinality was checked");
    let physical_entry = draft
        .rows
        .iter()
        .find(|row| matches!(row.role, NormalModuleDraftRoleV1::PhysicalEntry))
        .expect("physical entry cardinality was checked");
    let owner = match &source_main.role {
        NormalModuleDraftRoleV1::SourceMain { owner } => *owner,
        _ => unreachable!("source Main row was selected by role"),
    };
    let relation = &draft.source_entry;
    if relation.source_main_owner != owner
        || relation.source_main_symbol.as_ref() != source_main.symbol()
        || relation.source_main_arity != source_main.arity
        || relation.physical_symbol.as_ref() != physical_entry.symbol()
        || relation.physical_arity != physical_entry.arity
    {
        return Err(NormalModuleTransactionSchemaErrorV1::EntryRelationMismatch);
    }
    Ok(())
}

fn compare_rows(
    left: &NormalModuleDraftExpectationV1,
    right: &NormalModuleDraftExpectationV1,
) -> std::cmp::Ordering {
    row_rank(&left.role)
        .cmp(&row_rank(&right.role))
        .then_with(|| left.key.cmp(&right.key))
}

fn row_rank(role: &NormalModuleDraftRoleV1) -> u8 {
    match role {
        NormalModuleDraftRoleV1::SourceMain { .. } => 0,
        NormalModuleDraftRoleV1::Helper { .. } => 1,
        NormalModuleDraftRoleV1::PhysicalEntry => 2,
    }
}
