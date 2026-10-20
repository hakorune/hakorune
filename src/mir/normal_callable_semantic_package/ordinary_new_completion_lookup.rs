//! Borrow one already-issued Completion by its exact callable owner.

use super::OrdinaryNewClaimLedgerV1;
use crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1;
use crate::mir::resolved_semantics::FunctionOwnerIdV1;
use std::rc::Rc;

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn field_array_text_append_sources_v1(&self)
        -> Result<Vec<crate::mir::normal_callable_semantic_package::FieldArrayAppendSourceLoanV1>, String> {
        use crate::mir::resolved_semantics::home_new_prefix::NamedArrayHomeObservationV1;
        let mut sources = Vec::new();
        // Only successful original Completions contain source frames. The
        // owner's execution gate still rejects error-bearing Completions.
        let originals = self.completion_index.values().filter_map(|row| row.as_ref().ok())
            .chain(self.root_completion.iter().filter_map(|row| row.as_ref().ok())
                .filter(|root| !self.completion_index.contains_key(&root.owner())));
        for completion in originals {
            let Some(flow) = completion.cleanup().root_flow() else { continue };
            for row in flow.named_arrays() {
                if let NamedArrayHomeObservationV1::FieldTextAppend(frame) = row {
                    sources.push(self.field_array_text_append_source_v1(frame.source().call())?);
                }
            }
        }
        Ok(sources)
    }
    /// Presence selects the existing source-scoped consumer, never an empty
    /// cleanup inference. Error-bearing original completions remain errors.
    pub(in crate::mir) fn requires_source_home_scope_v1(&self, owner: FunctionOwnerIdV1) -> Result<bool, String> {
        if let Some(Err(issue)) = self.completion_index.get(&owner) {
            return Err(format!("[freeze:contract][callable-home/source-completion] {issue:?}"));
        }
        Ok(self.completion_for_owner(owner).is_some_and(|completion| completion.cleanup().root_flow().is_some()))
    }

    pub(in crate::mir) fn completion_for_owner(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> Option<&VerifiedFunctionCompletionV1> {
        self.completion_source_for_owner_v1(owner).map(Rc::as_ref)
    }

    pub(in crate::mir::normal_callable_semantic_package) fn completion_source_for_owner_v1(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> Option<&Rc<VerifiedFunctionCompletionV1>> {
        self.completion_index
            .get(&owner)
            .and_then(|row| row.as_ref().ok())
            .or_else(|| {
                self.root_completion.as_ref()
                    .and_then(|row| row.as_ref().ok())
                    .filter(|completion| completion.owner() == owner)
            })
    }
}
