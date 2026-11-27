//! Original signature issuance precedes the same-ledger typed input closure.
use super::*;
use crate::mir::normal_callable_semantic_package::{
    physical_signature::VerifiedCallablePhysicalSignatureCohortV1, OrdinaryNewClaimLedgerV1,
};

pub(super) fn issue_signature_and_finish_inputs_v1(
    brand: crate::mir::builder::SameModuleCallableCatalogBrandV1,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &std::rc::Rc<VerifiedSelectedCallableBatchMapV1>,
    contracts: &crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractsV1,
    results: &crate::mir::normal_callable_semantic_package::result_contract::VerifiedCallableResultContractCohortV1,
    ledger: &mut OrdinaryNewClaimLedgerV1,
) -> Result<
    std::rc::Rc<VerifiedCallablePhysicalSignatureCohortV1>,
    NormalCallableSemanticPackageIssueV1,
> {
    let signature = issue_callable_physical_signature_v1(brand, batch, selected, contracts)
        .map_err(
            |error| NormalCallableSemanticPackageIssueV1::PhysicalSignature { _error: error },
        )?;
    let signature = std::rc::Rc::new(signature);
    ledger
        .retain_original_instance_input_issuance_v1(selected.clone(), signature.clone())
        .map_err(
            |error| NormalCallableSemanticPackageIssueV1::LexicalInstanceCall { _error: error },
        )?;
    ledger
        .finish_typed_object_input_actuals_v1(selected, contracts, &signature)
        .map_err(
            |error| NormalCallableSemanticPackageIssueV1::LexicalInstanceCall { _error: error },
        )?;
    ledger
        .finish_original_instance_input_actuals_v1(selected, contracts, &signature, results)
        .map_err(
            |error| NormalCallableSemanticPackageIssueV1::LexicalInstanceCall { _error: error },
        )?;
    ledger
        .finish_static_input_actuals_v1(batch, selected, contracts, &signature, results)
        .map_err(
            |error| NormalCallableSemanticPackageIssueV1::LexicalInstanceCall { _error: error },
        )?;
    ledger
        .select_completed_static_local_routes_v1()
        .map_err(
            |error| NormalCallableSemanticPackageIssueV1::LexicalInstanceCall { _error: error },
        )?;
    ledger
        .seal_object_return_dispositions_v1(selected, contracts, &signature)
        .map_err(
            |error| NormalCallableSemanticPackageIssueV1::LexicalInstanceCall { _error: error },
        )?;
    // The same finisher now demands producer Normal/End after its single seal.
    // This is dependency staging, never a Fault retry or a second issuer.
    ledger
        .finish_original_instance_input_actuals_v1(selected, contracts, &signature, results)
        .map_err(|error| NormalCallableSemanticPackageIssueV1::LexicalInstanceCall { _error: error })?;
    ledger.seal_static_local_sources_v1();
    Ok(signature)
}

pub(super) fn issue_and_finish_lexical_slots_v1(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    contracts: &crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractsV1,
    signatures: &VerifiedCallablePhysicalSignatureCohortV1,
    results: &crate::mir::normal_callable_semantic_package::result_contract::VerifiedCallableResultContractCohortV1,
    ledger: &mut OrdinaryNewClaimLedgerV1,
) -> Result<(), NormalCallableSemanticPackageIssueV1> {
    ledger
        .issue_lexical_instance_call_dispositions(batch, selected, signatures, results)
        .and_then(|_| {
            ledger.finish_object_lexical_slots_v1(selected, contracts, signatures, results)
        })
        .and_then(|_| ledger.finish_borrowed_nullable_slots_v1(batch, selected, contracts, signatures, results))
        .and_then(|_| ledger.finish_mixed_call_slots_v1(selected, contracts, signatures, results))
        .and_then(|_| ledger.co_seal_static_local_routes_v1())
        .map_err(
            |error| NormalCallableSemanticPackageIssueV1::LexicalInstanceCall { _error: error },
        )
}
