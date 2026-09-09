//! Owner-indexed Completion retention for the existing ordinary-New ledger.
//!
//! The index borrows the issuer's `Rc` products after S6C has consumed its
//! exclusive seed. It does not verify, copy, or reclassify source meaning.

use super::super::completion_seed::VerifiedCallableCompletionSeedCohortV1;
use super::OrdinaryNewClaimLedgerV1;
use std::rc::Rc;

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn retain_completion_index(
        &mut self,
        seeds: &VerifiedCallableCompletionSeedCohortV1,
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
