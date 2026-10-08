//! Owner-indexed Completion retention for the existing ordinary-New ledger.
//!
//! The index borrows the issuer's `Rc` products after S6C has consumed its
//! exclusive seed. It does not verify, copy, or reclassify source meaning.

use super::super::result_contract::VerifiedCallableResultContractBuilderV1;
use super::OrdinaryNewClaimLedgerV1;
use std::rc::Rc;

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn retain_completion_index(
        &mut self,
        seeds: &VerifiedCallableResultContractBuilderV1,
    ) {
        self.completion_index = seeds
            .completion_index()
            .into_iter()
            .map(|(owner, completion)| (owner, Ok(completion)))
            .collect();
        self.terminal_relation_index = seeds.terminal_relation_index();
        if let Some(Ok(completion)) = &self.root_completion {
            self.completion_index
                .entry(completion.owner())
                .or_insert_with(|| Ok(Rc::clone(completion)));
        }
    }
}

#[path = "ordinary_new_return_leaf.rs"]
mod return_leaf;

#[path = "ordinary_new_return_callee.rs"]
mod return_callee;

#[path = "ordinary_new_return_acquisition.rs"]
mod return_acquisition;

#[path = "ordinary_new_return_handoff.rs"]
mod return_handoff;

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
