//! Ground call-result composition from the original borrowed source rows.
//! Temporary readiness sets schedule proof; the existing result map owns it.
use super::*;
use crate::mir::resolved_semantics::{BindingKindV1, ResolvedAssignmentTargetV1};

pub(super) fn retain_call_dependency(
    input: ResolvedFunctionLoweringInputV1<'_>,
    source: &PreparedBorrowedFormalIngressV1,
    owner: FunctionOwnerIdV1,
    site: &SourceExprSiteV1,
    dependencies: &mut Vec<LexicalInstanceCallSourceTargetV1>,
) -> Result<bool, String> {
    let function = input.function();
    let call_site = if function.method_calls().any(|(actual, _)| actual == site) {
        site.clone()
    } else {
        let Some(ResolvedLexicalRefV1::Local(binding)) = function.variable_ref(site) else {
            return Ok(false);
        };
        if binding.owner() != owner
            || function
                .binding(binding)
                .is_none_or(|record| !matches!(record.kind(), BindingKindV1::Local { .. }))
            || function.assignment_targets().any(|(_, target)| {
                matches!(target,
                ResolvedAssignmentTargetV1::BindingRebind(actual) if *actual == binding)
            })
        {
            return Ok(false);
        }
        let mut initializers = function
            .expression_source()
            .initializers()
            .filter(|row| row.binding() == binding);
        let Some(initializer) = initializers.next() else {
            return Ok(false);
        };
        if initializers.next().is_some()
            || function.declaration_binding(initializer.declaration_site()) != Some(binding)
        {
            return Ok(false);
        }
        let Some(call_site) = initializer.initializer_site() else {
            return Ok(false);
        };
        call_site.clone()
    };
    let Some((_, call)) = function
        .method_calls()
        .find(|(site, _)| *site == &call_site)
    else {
        return Ok(false);
    };
    let owned_site = OwnedExprSiteV1::new(owner, call_site.clone());
    let mut incoming = source.incoming.iter().filter(|row| row.call == owned_site);
    let Some(row) = incoming.next() else {
        return Ok(false);
    };
    let target = &row.source;
    if incoming.next().is_some()
        || target.call_site() != &owned_site
        || target.callee_owner() != row.callee
        || !source.definitions.contains_key(&row.callee)
        || target.receiver_site() != call.receiver_site()
        || call.receiver()
            != crate::mir::resolved_semantics::ResolvedMethodCallReceiverSourceV1::Lexical(
                ResolvedLexicalRefV1::Local(target.receiver_binding()),
            )
        || target.argument_sites().len() != call.arity() as usize
        || target.target().arity() != call.arity()
        || !target
            .argument_sites()
            .iter()
            .zip(call.arguments())
            .all(|(site, argument)| site == argument.site())
    {
        return Err(freeze("borrowed-result/call-source-identity"));
    }
    dependencies.push(target.clone());
    Ok(true)
}

pub(super) fn ground_source_results(
    results: &mut BTreeMap<FunctionOwnerIdV1, Result<BorrowedI64ResultSourceV1, String>>,
) {
    let mut grounded = BTreeSet::new();
    loop {
        let next: Vec<_> = results
            .iter()
            .filter_map(|(owner, proof)| {
                let proof = proof.as_ref().ok()?;
                (!grounded.contains(owner)
                    && proof.dependencies.iter().all(|dependency| {
                        grounded.contains(&dependency.callee_owner())
                            && results
                                .get(&dependency.callee_owner())
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
                    proof
                        .dependencies
                        .iter()
                        .any(|dependency| {
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
                                            || !callee.contract_corroborated
                                    })
                        })
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
