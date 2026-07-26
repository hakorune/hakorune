//! Retained preparation failure for the passive normal-module schema.

use super::schema::NormalModuleTransactionDraftV1;
use crate::mir::builder::module_draft_collector::FunctionDraftKeyV1;

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum NormalModuleTransactionSchemaErrorV1 {
    MissingSourceMain,
    DuplicateSourceMain,
    MissingPhysicalEntry,
    DuplicatePhysicalEntry,
    DuplicateKey(FunctionDraftKeyV1),
    DuplicateSymbol(Box<str>),
    RoleKeyMismatch,
    ArityMismatch,
    EntryRelationMismatch,
}

#[derive(Debug)]
pub(in crate::mir::builder) struct RejectedNormalModuleTransactionSchemaV1 {
    pub(super) owner: NormalModuleTransactionDraftV1,
    pub(super) error: NormalModuleTransactionSchemaErrorV1,
}

impl RejectedNormalModuleTransactionSchemaV1 {
    pub(in crate::mir::builder) fn error(&self) -> &NormalModuleTransactionSchemaErrorV1 {
        &self.error
    }

    pub(in crate::mir::builder) fn discard(self) {
        drop(self);
    }

    pub(super) fn into_error(self) -> NormalModuleTransactionSchemaErrorV1 {
        self.error
    }

    #[cfg(test)]
    pub(super) fn owner(&self) -> &NormalModuleTransactionDraftV1 {
        &self.owner
    }
}
