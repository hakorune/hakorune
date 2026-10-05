//! Ground call-result composition from the original borrowed source rows.
//! Temporary readiness sets schedule proof; the existing result map owns it.
use super::*;

/// The same finite dependency fold grounds Pending requirements before promotion
/// and sealed dependencies afterwards. It never erases a pre-ingress candidate.
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn grounded_source_results_v1(
    results: &BTreeMap<FunctionOwnerIdV1, Result<BorrowedI64ResultSourceV1, String>>,
) -> BTreeSet<FunctionOwnerIdV1> {
    let mut grounded = BTreeSet::new();
    loop {
        let next: Vec<_> = results
            .iter()
            .filter_map(|(owner, proof)| {
                let proof = proof.as_ref().ok()?;
                (!grounded.contains(owner)
                    && pending::pending_ready_v1(proof)
                    && pending::dependency_owners_v1(proof).iter().all(|callee| {
                        grounded.contains(callee)
                            && results
                                .get(callee)
                                .and_then(|row| row.as_ref().ok())
                                .is_some_and(|row| {
                                    row.class == BorrowedResultClassV1::I64
                                        && !row.returns.is_empty()
                                })
                    }))
                .then_some(*owner)
            })
            .collect();
        if next.is_empty() {
            break;
        }
        grounded.extend(next);
    }
    grounded
}

pub(super) fn ground_source_results(
    results: &mut BTreeMap<FunctionOwnerIdV1, Result<BorrowedI64ResultSourceV1, String>>,
) {
    let grounded = grounded_source_results_v1(results);
    for (owner, proof) in results {
        if proof.is_ok() && !grounded.contains(owner) {
            *proof = Err(freeze("borrowed-result/source-not-i64"));
        }
    }
}

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn corroborate_borrowed_result_cohort_v1(
        &mut self,
        prepared: &[Result<Option<LexicalInstanceCallSourceTargetV1>, String>],
        results: &crate::mir::normal_callable_semantic_package::result_contract::VerifiedCallableResultContractCohortV1,
    ) {
        for source in prepared
            .iter()
            .filter_map(|row| row.as_ref().ok().and_then(Option::as_ref))
        {
            self.corroborate_borrowed_i64_result_v1(source, results);
        }
        loop {
            let invalid: Vec<_> = self
                .borrowed_i64_results
                .iter()
                .filter_map(|(owner, proof)| {
                    let proof = proof.as_ref().ok()?;
                    (proof.require_source_sealed_v1().is_err()
                        || proof.dependencies.iter().any(|dependency| {
                            let exact = prepared
                                .iter()
                                .filter_map(|row| row.as_ref().ok().and_then(Option::as_ref))
                                .filter(|row| row.call_site() == dependency.call_site())
                                .collect::<Vec<_>>();
                            exact.len() != 1
                                || exact[0] != dependency
                                || self
                                    .borrowed_i64_results
                                    .get(&dependency.callee_owner())
                                    .and_then(|row| row.as_ref().ok())
                                    .is_none_or(|callee| {
                                        callee.class != BorrowedResultClassV1::I64
                                            || callee.returns.is_empty()
                                            || callee.require_source_sealed_v1().is_err()
                                            || !callee.contract_corroborated
                                    })
                        }))
                    .then_some(*owner)
                })
                .collect();
            if invalid.is_empty() {
                break;
            }
            for owner in invalid {
                self.borrowed_i64_results.insert(
                    owner,
                    Err(freeze("borrowed-result/result-contract-mismatch")),
                );
            }
        }
    }
}

#[cfg(test)]
#[path = "ordinary_new_borrowed_formal_result_composition_tests.rs"]
mod tests;
