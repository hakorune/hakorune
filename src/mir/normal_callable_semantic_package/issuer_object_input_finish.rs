//! Original signature issuance precedes the same-ledger typed input closure.
use super::*;
use crate::mir::normal_callable_semantic_package::{
    physical_signature::VerifiedCallablePhysicalSignatureCohortV1, OrdinaryNewClaimLedgerV1,
};

pub(super) fn issue_signature_and_finish_inputs_v1(
    brand: crate::mir::builder::SameModuleCallableCatalogBrandV1,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    ledger: &mut OrdinaryNewClaimLedgerV1,
) -> Result<VerifiedCallablePhysicalSignatureCohortV1, NormalCallableSemanticPackageIssueV1> {
    let signature = issue_callable_physical_signature_v1(brand, batch, selected, contracts)
        .map_err(
            |error| NormalCallableSemanticPackageIssueV1::PhysicalSignature { _error: error },
        )?;
    ledger
        .finish_typed_object_input_actuals_v1(selected, contracts, &signature)
        .map_err(
            |error| NormalCallableSemanticPackageIssueV1::LexicalInstanceCall { _error: error },
        )?;
    ledger
        .seal_object_return_dispositions_v1(selected, contracts, &signature)
        .map_err(
            |error| NormalCallableSemanticPackageIssueV1::LexicalInstanceCall { _error: error },
        )?;
    Ok(signature)
}

pub(super) fn issue_and_finish_lexical_slots_v1(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    signatures: &VerifiedCallablePhysicalSignatureCohortV1,
    results: &crate::mir::normal_callable_semantic_package::result_contract::VerifiedCallableResultContractCohortV1,
    ledger: &mut OrdinaryNewClaimLedgerV1,
) -> Result<(), NormalCallableSemanticPackageIssueV1> {
    ledger
        .issue_lexical_instance_call_dispositions(batch, selected, signatures, results)
        .and_then(|_| {
            ledger.finish_object_lexical_slots_v1(selected, contracts, signatures, results)
        })
        .and_then(|_| ledger.co_seal_static_local_routes_v1())
        .map_err(
            |error| NormalCallableSemanticPackageIssueV1::LexicalInstanceCall { _error: error },
        )
}
