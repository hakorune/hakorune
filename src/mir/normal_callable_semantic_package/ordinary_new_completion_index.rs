//! Owner-indexed Completion retention for the existing ordinary-New ledger.
//!
//! The index borrows the issuer's `Rc` products after S6C has consumed its
//! exclusive seed. It does not verify, copy, or reclassify source meaning.

use super::super::result_contract::VerifiedCallableResultContractBuilderV1;
use super::{OrdinaryNewClaimLedgerV1, OrdinaryNewCoSealIssueV1};
use std::rc::Rc;

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn retain_completion_index(
        &mut self,
        seeds: &VerifiedCallableResultContractBuilderV1,
    ) -> Result<(), OrdinaryNewCoSealIssueV1> {
        if self
            .root_completion
            .as_ref()
            .is_some_and(|root| root.is_ok())
            && (!self.completion_index.is_empty() || !self.terminal_relation_index.is_empty())
        {
            return Err(retention_issue("duplicate-install"));
        }
        let mut completion_index = seeds
            .completion_index()
            .into_iter()
            .map(|(owner, completion)| (owner, Ok(completion)))
            .collect::<std::collections::BTreeMap<_, _>>();
        let mut terminal_relation_index = seeds.terminal_relation_index();
        if let Some(Ok(completion)) = &self.root_completion {
            let owner = completion.owner();
            if completion_index.contains_key(&owner) || terminal_relation_index.contains_key(&owner)
            {
                return Err(retention_issue("root-owner-collision"));
            }
            for (site, relation) in self.terminal_relation.iter() {
                if relation.owner() != owner {
                    return Err(retention_issue("root-terminal-owner"));
                }
                if relation.return_site() != site {
                    return Err(retention_issue("root-terminal-site"));
                }
                if !completion.explicit_sites().contains(site) {
                    return Err(retention_issue("root-terminal-exit"));
                }
            }
            completion_index.insert(owner, Ok(Rc::clone(completion)));
            terminal_relation_index.insert(owner, Rc::clone(&self.terminal_relation));
        }
        self.completion_index = completion_index;
        self.terminal_relation_index = terminal_relation_index;
        Ok(())
    }
}

fn retention_issue(reason: &'static str) -> OrdinaryNewCoSealIssueV1 {
    OrdinaryNewCoSealIssueV1::CompletionIndexRetention { reason }
}

#[cfg(test)]
#[path = "ordinary_new_completion_index_tests.rs"]
mod tests;

#[path = "ordinary_new_return_leaf.rs"]
mod return_leaf;
pub(super) use return_leaf::ObjectReturnTeardownDescriptorV1;

#[path = "ordinary_new_return_callee.rs"]
mod return_callee;

#[path = "ordinary_new_return_acquisition.rs"]
mod return_acquisition;

#[path = "ordinary_new_return_handoff.rs"]
mod return_handoff;
pub(super) use return_handoff::ObjectReturnTeardownAvailabilityV1;

#[path = "ordinary_new_normal_return.rs"]
mod normal_return;
pub(super) use normal_return::NormalReturnDispositionV1;

#[path = "ordinary_new_return_result.rs"]
mod return_result;

#[path = "ordinary_new_direct_return_source.rs"]
mod direct_return_source;

#[path = "ordinary_new_direct_return_cleanup.rs"]
mod direct_return_cleanup;
pub(super) use direct_return_cleanup::DirectRootCleanupSourceV1;

#[path = "ordinary_new_terminal_call_source.rs"]
mod terminal_call_source;

#[path = "ordinary_new_return_construction.rs"]
mod return_construction;

#[path = "ordinary_new_root_object_result.rs"]
mod root_object_result;
