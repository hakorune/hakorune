//! Borrow one already-issued Completion by its exact callable owner.

use super::OrdinaryNewClaimLedgerV1;
use crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1;
use crate::mir::resolved_semantics::FunctionOwnerIdV1;
use std::rc::Rc;

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn completion_for_owner(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> Option<&VerifiedFunctionCompletionV1> {
        self.completion_index
            .get(&owner)
            .and_then(|row| row.as_ref().ok())
            .map(Rc::as_ref)
            .or_else(|| {
                self.root_completion
                    .as_ref()
                    .and_then(|row| row.as_ref().ok())
                    .filter(|completion| completion.owner() == owner)
                    .map(Rc::as_ref)
            })
    }
}
