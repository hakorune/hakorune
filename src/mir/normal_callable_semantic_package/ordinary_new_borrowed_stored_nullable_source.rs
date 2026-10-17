//! Retain the original stored LOCAL result demand before receiver/input issuance.
//! This projects the existing result loan; it grants no execution or owning Home.
use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::AppMainSourceRefV1;
use super::super::super::result_class_claim::{
    BorrowedNullableLocalSourceLoanV1, OrdinaryNewResultClassClaimsV1, OrdinaryNewResultClassV1,
};
use super::*;

pub(super) fn original_local_loan_v1(
    need: &PreparedSourceCallNeedV1,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    fields: &super::super::super::field_write_claim::OrdinaryNewFieldWriteClaimsV1,
    facts: &OrdinaryNewResultClassClaimsV1,
 app_main: Option<AppMainSourceRefV1<'_>>) -> Result<Option<BorrowedNullableLocalSourceLoanV1>, String> {
    let reference = need.reference();
    if need.stored().is_none()
        || !matches!(
            facts.get(&reference.target),
            Some(OrdinaryNewResultClassV1::NullableForwarded { .. })
        )
    {
        return Ok(None);
    }
    let destination = crate::mir::normal_callable_semantic_package::original_call_source::with_original_source_input_v1(
        batch, constructors, reference.call_site.owner(), |context, input| {
            let original = input.function().method_call(reference.call_site.site())
                .ok_or_else(|| freeze("caller-source-call"))?;
            context.constructor_key_for_call(&reference.call_site, original)?;
            let mut initializers = input.function().expression_source().initializers()
                .filter(|row| row.initializer_site() == Some(reference.call_site.site()));
            let destination = initializers.next().map(|row| row.binding());
            if initializers.next().is_some() { return Err(freeze("initializer-not-unique")); }
            Ok(destination)
        })??;
    let Some(destination) = destination else {
        return Ok(None);
    };
    let loan = facts
        .borrowed_nullable_local_source_v1(
            batch,
            selected,
            fields,
            contracts,
            &reference.call_site,
            destination,
         app_main)
        .map_err(freeze)?
        .ok_or_else(|| freeze("local-result-missing"))?;
    recheck_original_local_loan_v1(need, &loan, batch, selected, contracts, fields, facts, app_main)?;
    Ok(Some(loan))
}

pub(super) fn recheck_original_local_loan_v1(
    need: &PreparedSourceCallNeedV1,
    loan: &BorrowedNullableLocalSourceLoanV1,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    fields: &super::super::super::field_write_claim::OrdinaryNewFieldWriteClaimsV1,
    facts: &OrdinaryNewResultClassClaimsV1,
 app_main: Option<AppMainSourceRefV1<'_>>) -> Result<(), String> {
    let reference = need.reference();
    let stored = need
        .stored()
        .ok_or_else(|| freeze("stored-source-missing"))?;
    if loan.call() != &reference.call_site
        || loan.receiver() != &reference.receiver_site
        || loan.key() != &reference.target
        || loan.callee_owner() != reference.callee_owner
        || stored.child_class.as_ref() != reference.target.owner()
        || selected.batch_slot(&SelectedNormalCallableKeyV1::Cataloged(
            reference.target.clone(),
        )) != Some(reference.target_batch_slot)
    {
        return Err(freeze("target-identity"));
    }
    let mut declarations = contracts
        .iter()
        .filter(|row| row.owner == reference.callee_owner);
    let declaration = declarations
        .next()
        .ok_or_else(|| freeze("callee-contract"))?;
    if declarations.next().is_some() || declaration.batch_slot != reference.target_batch_slot
        || declaration.mode != crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1::InstanceBoxMethod
        || declaration.parameters.len() != reference.argument_sites.len()
        || reference.argument_sites.len() != reference.target.arity() as usize
        || declaration.parameters.iter().enumerate().any(|(ordinal, row)|
            row.ordinal as usize != ordinal || row.binding.owner() != reference.callee_owner) {
        return Err(freeze("callee-contract"));
    }
    facts
        .recheck_borrowed_nullable_local_source_v1(batch, selected, fields, contracts, loan, app_main)
        .map_err(freeze)
}
fn freeze(reason: &str) -> String {
    format!("[freeze:contract][stored-child/borrowed-nullable/{reason}]")
}
