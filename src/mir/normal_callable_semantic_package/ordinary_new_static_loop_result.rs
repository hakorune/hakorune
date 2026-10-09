//! Retain the original Static result-catalog I64 row for one selected Loop.
//! This is source/result ABI evidence, not a physical return or call packet.

use std::collections::{BTreeMap, BTreeSet};

use crate::mir::builder::{SameModuleCallableNamespaceV1, SelectedNormalCallableKeyV1};
use crate::mir::callable_result_representation::VerifiedCallableResultDispositionV1;
use crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_control_flow::DeclaredFunctionResultContractV1;
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, SourceStmtSiteV1};
use hakorune_mir_defs::CanonicalSameModuleCallableKeyV1;

use super::super::qualified_static_call_claim::QualifiedStaticCallClaimIndexV1;
use super::super::result_contract::{
    VerifiedCallableResultContractCohortV1, VerifiedCallableResultContractRowV1,
};
use super::super::selected_mapping::VerifiedSelectedCallableBatchMapV1;
use super::{OrdinaryNewClaimLedgerV1, OrdinaryNewCoSealIssueV1};

#[derive(Debug)]
pub(in crate::mir) struct VerifiedStaticLoopI64ResultSourceV1 {
    owner: FunctionOwnerIdV1,
    caller: CanonicalSameModuleCallableKeyV1,
    loop_site: SourceStmtSiteV1,
    returns: Box<[SourceStmtSiteV1]>,
}

impl VerifiedStaticLoopI64ResultSourceV1 {
    pub(in crate::mir) fn corroborates(
        &self,
        product: &crate::mir::builder::VerifiedStaticI64LoopSemanticV2,
    ) -> bool {
        let (entry, _, _) = product.source_calls();
        let actual: BTreeSet<_> = self.returns.iter().collect();
        let expected = [
            &product.roles().return_site,
            product.tail_call().return_site(),
        ];
        self.owner == product.roles().n_binding.owner()
            && self.loop_site == product.roles().loop_site
            && entry.original().caller() == &self.caller
            && self.returns.len() == 2
            && actual.len() == 2
            && expected.into_iter().collect::<BTreeSet<_>>() == actual
    }
}

fn issue_result(
    input: ResolvedFunctionLoweringInputV1<'_>,
    loop_site: &SourceStmtSiteV1,
    caller: &CanonicalSameModuleCallableKeyV1,
    claim: Option<&VerifiedCallableResultDispositionV1>,
    contract: Option<&VerifiedCallableResultContractRowV1>,
    identity: &crate::parser::CallableDeclarationIdentityV1,
) -> Result<VerifiedStaticLoopI64ResultSourceV1, String> {
    let reject = || "[freeze:contract][callable-loop/static-result-source-unavailable]".to_owned();
    let contract = contract.ok_or_else(reject)?;
    let borrowed = contract.borrow();
    let completion = borrowed.completion();
    if caller.namespace() != SameModuleCallableNamespaceV1::StaticBoxMethod
        || !matches!(
            claim,
            Some(VerifiedCallableResultDispositionV1::ExactI64 { .. })
        )
        || contract.owner() != input.owner()
        || !contract.identity().same_as(identity)
        || borrowed.result().is_some()
        || !matches!(
            completion.function_exit_contract().declared_result(),
            DeclaredFunctionResultContractV1::Unannotated
        )
        || completion.owner() != input.owner()
        || !completion.returns_value()
    {
        return Err(reject());
    }
    Ok(VerifiedStaticLoopI64ResultSourceV1 {
        owner: input.owner(),
        caller: caller.clone(),
        loop_site: loop_site.clone(),
        returns: completion.explicit_sites().to_vec().into_boxed_slice(),
    })
}

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn retain_static_loop_i64_result_v1(
        &mut self,
        batch: &VerifiedResolvedCallableSemanticBatchV1,
        selected: &VerifiedSelectedCallableBatchMapV1,
        claims: &QualifiedStaticCallClaimIndexV1,
        contracts: &VerifiedCallableResultContractCohortV1,
    ) -> Result<(), OrdinaryNewCoSealIssueV1> {
        let mut rows = BTreeMap::new();
        for declaration in batch.declarations() {
            let slot = declaration.batch_slot();
            let Some(SelectedNormalCallableKeyV1::Cataloged(caller)) =
                selected.key_for_batch_slot(slot)
            else {
                continue;
            };
            batch
                .with_lowering_input(slot, |input| {
                    for loop_site in input.function().loop_sites() {
                        if !self.expects_loop_static_source_loan_v1(input.owner(), loop_site) {
                            continue;
                        }
                        let result = issue_result(
                            input,
                            loop_site,
                            &caller,
                            claims.result_for_key(&caller),
                            contracts.row(slot),
                            declaration.identity(),
                        );
                        if rows.insert(loop_site.clone(), result).is_some() {
                            return Err(OrdinaryNewCoSealIssueV1::BatchLoan);
                        }
                    }
                    Ok(())
                })
                .map_err(|_| OrdinaryNewCoSealIssueV1::BatchLoan)??;
        }
        *self.loop_static_i64_results.get_mut() = rows;
        Ok(())
    }

    pub(in crate::mir) fn take_static_loop_i64_result_v1(
        &self,
        loop_site: &SourceStmtSiteV1,
    ) -> Option<Result<VerifiedStaticLoopI64ResultSourceV1, String>> {
        self.loop_static_i64_results.borrow_mut().remove(loop_site)
    }
}
