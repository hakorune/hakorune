//! Main-only canonical pre-draft batch manifest.

use crate::mir::compiler::normal_source_plan::VerifiedNormalMainThunkPlanV1;

use super::rejection::NormalModuleTransactionSchemaErrorV1;
use super::schema::{
    NormalModuleDraftExpectationV1, NormalModuleEntryRelationV1, NormalModuleTransactionDraftV1,
    NormalModuleTransactionSchemaV1,
};

pub(in crate::mir::builder) struct NormalCanonicalModuleBatchV1;

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum NormalCanonicalModuleBatchErrorV1 {
    Schema(NormalModuleTransactionSchemaErrorV1),
}

#[derive(Debug)]
pub(in crate::mir::builder) struct PreparedNormalCanonicalModuleBatchV1<'unit> {
    thunk: VerifiedNormalMainThunkPlanV1<'unit>,
    schema: NormalModuleTransactionSchemaV1,
    _seal: PreparedNormalCanonicalModuleBatchSealV1,
}

#[derive(Debug)]
struct PreparedNormalCanonicalModuleBatchSealV1;

#[derive(Debug)]
pub(in crate::mir::builder) struct RejectedNormalCanonicalModuleBatchV1<'unit> {
    owner: VerifiedNormalMainThunkPlanV1<'unit>,
    error: NormalCanonicalModuleBatchErrorV1,
}

impl NormalCanonicalModuleBatchV1 {
    pub(in crate::mir::builder) fn prepare(
        thunk: VerifiedNormalMainThunkPlanV1<'_>,
    ) -> Result<PreparedNormalCanonicalModuleBatchV1<'_>, RejectedNormalCanonicalModuleBatchV1<'_>>
    {
        let header = thunk.source_header();
        let entry = thunk.entry();
        let draft = NormalModuleTransactionDraftV1::new(
            vec![
                NormalModuleDraftExpectationV1::source_main(
                    header.owner(),
                    header.symbol().as_mir_name(),
                    header.arity(),
                ),
                NormalModuleDraftExpectationV1::physical_entry(
                    entry.physical_symbol(),
                    entry.physical_arity(),
                ),
            ],
            NormalModuleEntryRelationV1::new(
                header.owner(),
                header.symbol().as_mir_name(),
                header.arity(),
                entry.physical_symbol(),
                entry.physical_arity(),
            ),
        );
        prepare_draft(thunk, draft)
    }
}

fn prepare_draft<'unit>(
    thunk: VerifiedNormalMainThunkPlanV1<'unit>,
    draft: NormalModuleTransactionDraftV1,
) -> Result<PreparedNormalCanonicalModuleBatchV1<'unit>, RejectedNormalCanonicalModuleBatchV1<'unit>>
{
    match NormalModuleTransactionSchemaV1::seal(draft) {
        Ok(schema) => Ok(PreparedNormalCanonicalModuleBatchV1 {
            thunk,
            schema,
            _seal: PreparedNormalCanonicalModuleBatchSealV1,
        }),
        Err(rejected) => Err(RejectedNormalCanonicalModuleBatchV1 {
            owner: thunk,
            error: NormalCanonicalModuleBatchErrorV1::Schema(rejected.into_error()),
        }),
    }
}

impl<'unit> PreparedNormalCanonicalModuleBatchV1<'unit> {
    pub(in crate::mir::builder) fn into_parts(
        self,
    ) -> (
        VerifiedNormalMainThunkPlanV1<'unit>,
        NormalModuleTransactionSchemaV1,
    ) {
        (self.thunk, self.schema)
    }

    #[cfg(test)]
    pub(super) fn thunk(&self) -> &VerifiedNormalMainThunkPlanV1<'_> {
        &self.thunk
    }

    #[cfg(test)]
    pub(super) fn schema(&self) -> &NormalModuleTransactionSchemaV1 {
        &self.schema
    }
}

impl RejectedNormalCanonicalModuleBatchV1<'_> {
    pub(in crate::mir::builder) fn error(&self) -> &NormalCanonicalModuleBatchErrorV1 {
        &self.error
    }

    pub(in crate::mir::builder) fn discard(self) {
        drop(self);
    }

    #[cfg(test)]
    pub(super) fn owner(&self) -> &VerifiedNormalMainThunkPlanV1<'_> {
        &self.owner
    }
}

#[cfg(test)]
pub(super) fn prepare_draft_for_test<'unit>(
    thunk: VerifiedNormalMainThunkPlanV1<'unit>,
    draft: NormalModuleTransactionDraftV1,
) -> Result<PreparedNormalCanonicalModuleBatchV1<'unit>, RejectedNormalCanonicalModuleBatchV1<'unit>>
{
    prepare_draft(thunk, draft)
}
