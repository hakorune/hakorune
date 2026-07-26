//! Schema-only batch sealing for completed normal callable drafts.

use crate::mir::builder::normal_module_transaction::{
    NormalModuleDraftExpectationV1, NormalModuleEntryRelationV1, NormalModuleTransactionDraftV1,
    NormalModuleTransactionSchemaErrorV1, NormalModuleTransactionSchemaV1,
};

use super::callable_main_physical::PreparedNormalCallableMainPhysicalV1;

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir) enum NormalCallableBatchErrorV1 {
    Schema(NormalModuleTransactionSchemaErrorV1),
}

/// One unpublished schema and the exact prepared drafts it describes.
#[derive(Debug)]
pub(in crate::mir) struct PreparedNormalCallableBatchV1 {
    drafts: PreparedNormalCallableMainPhysicalV1,
    schema: NormalModuleTransactionSchemaV1,
}

impl PreparedNormalCallableBatchV1 {
    pub(in crate::mir) const fn drafts(&self) -> &PreparedNormalCallableMainPhysicalV1 {
        &self.drafts
    }

    pub(in crate::mir) const fn schema(&self) -> &NormalModuleTransactionSchemaV1 {
        &self.schema
    }
}

/// Schema rejection retains every already-prepared helper/Main/physical draft.
#[derive(Debug)]
pub(in crate::mir) struct RejectedNormalCallableBatchV1 {
    drafts: PreparedNormalCallableMainPhysicalV1,
    error: NormalCallableBatchErrorV1,
}

impl RejectedNormalCallableBatchV1 {
    pub(in crate::mir) fn error(&self) -> &NormalCallableBatchErrorV1 {
        &self.error
    }

    pub(in crate::mir) fn discard(self) {
        drop(self);
    }

    #[cfg(test)]
    pub(super) fn retained_helper_count(&self) -> usize {
        self.drafts.helpers().drafts().len()
    }
}

impl PreparedNormalCallableMainPhysicalV1 {
    /// Sole BATCH0 terminal. It borrows prepared drafts only to construct the
    /// pre-existing schema input, then retains them unchanged on either exit.
    pub(in crate::mir) fn seal_normal_callable_batch_v1(
        self,
    ) -> Result<PreparedNormalCallableBatchV1, RejectedNormalCallableBatchV1> {
        let relation = self.relation();
        let mut rows = Vec::with_capacity(self.helpers().drafts().len() + 2);
        rows.push(NormalModuleDraftExpectationV1::source_main(
            relation.source_header().owner(),
            relation.source_header().symbol().as_mir_name(),
            relation.source_header().arity(),
        ));
        for helper in self.helpers().drafts() {
            rows.push(NormalModuleDraftExpectationV1::helper(
                helper.key().clone(),
                helper.draft().signature.name.clone(),
                helper.draft().signature.params.len(),
            ));
        }
        rows.push(NormalModuleDraftExpectationV1::physical_entry(
            relation.entry().physical_symbol(),
            relation.entry().physical_arity(),
        ));
        let draft = NormalModuleTransactionDraftV1::new(
            rows,
            NormalModuleEntryRelationV1::new(
                relation.source_header().owner(),
                relation.source_header().symbol().as_mir_name(),
                relation.source_header().arity(),
                relation.entry().physical_symbol(),
                relation.entry().physical_arity(),
            ),
        );
        match NormalModuleTransactionSchemaV1::seal(draft) {
            Ok(schema) => Ok(PreparedNormalCallableBatchV1 {
                drafts: self,
                schema,
            }),
            Err(rejected) => Err(RejectedNormalCallableBatchV1 {
                drafts: self,
                error: NormalCallableBatchErrorV1::Schema(rejected.into_error()),
            }),
        }
    }
}

#[cfg(test)]
pub(crate) fn reject_normal_callable_batch_for_test(
    prepared: PreparedNormalCallableMainPhysicalV1,
) -> RejectedNormalCallableBatchV1 {
    let relation = prepared.relation();
    let draft = NormalModuleTransactionDraftV1::new(
        vec![
            NormalModuleDraftExpectationV1::source_main(
                relation.source_header().owner(),
                relation.source_header().symbol().as_mir_name(),
                relation.source_header().arity(),
            ),
            NormalModuleDraftExpectationV1::source_main(
                relation.source_header().owner(),
                relation.source_header().symbol().as_mir_name(),
                relation.source_header().arity(),
            ),
            NormalModuleDraftExpectationV1::physical_entry(
                relation.entry().physical_symbol(),
                relation.entry().physical_arity(),
            ),
        ],
        NormalModuleEntryRelationV1::new(
            relation.source_header().owner(),
            relation.source_header().symbol().as_mir_name(),
            relation.source_header().arity(),
            relation.entry().physical_symbol(),
            relation.entry().physical_arity(),
        ),
    );
    let rejected = NormalModuleTransactionSchemaV1::seal(draft)
        .expect_err("test injection requires duplicate source Main rejection");
    RejectedNormalCallableBatchV1 {
        drafts: prepared,
        error: NormalCallableBatchErrorV1::Schema(rejected.into_error()),
    }
}
