//! Family dispatch for a retained aggregate refusal. Membership comes from
//! original Instance arguments and the existing construction issuer, not AST names.
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::BorrowedCallActualRequestV1;
use crate::mir::resolved_semantics::{
    NamedArrayConstructionSourceV1, NamedArrayRequirementIssueV1,
    ResolvedLexicalRefV1, ResolvedMethodCallReceiverSourceV1,
};

pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::coseal_issue) fn select_failed_array_initializer_v1(
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    brands: Option<&crate::analysis::brand_program_declaration_catalog::VerifiedBrandProgramDeclarationCatalogV1>,
    targets: &super::super::super::lexical_instance_call::PreparedLexicalInstanceCallSourceTargetsV1,
    site: &OwnedExprSiteV1,
    request: BorrowedCallActualRequestV1<'_>,
) -> Result<bool, OrdinaryNewCoSealIssueV1> {
    let BorrowedCallActualRequestV1::NamedArrayHolderInitializer(binding) = request else {
        return Ok(true);
    };
    let Some(brands) = brands else { return Ok(false) };
    let refuse = |issue: String| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
        site: site.clone(), issue,
    };
    let targets = targets.as_ref().map_err(|issue| refuse(issue.clone()))?;
    let ledger = input.forest().callable_source_ledger(input.owner())
        .map_err(|issue| refuse(format!("[freeze:contract][array-holder/source-ledger]: {issue:?}")))?;
    for target in targets.iter().filter_map(|row| row.as_ref().ok().and_then(Option::as_ref))
        .filter(|row| row.call_site().owner() == input.owner())
    {
        for argument in target.argument_sites() {
            let Some(read) = ledger.method_call(argument) else { continue };
            if read.receiver() != ResolvedMethodCallReceiverSourceV1::Lexical(ResolvedLexicalRefV1::Local(binding))
                || crate::mir::core_method_result_kind::lookup_core_method_result_row_v2("ArrayBox", read.selector(), read.arity())
                    .is_none_or(|row| row.op != crate::mir::core_method_op::CoreMethodOp::ArrayGet)
            { continue; }
            let identity = match NamedArrayConstructionSourceV1::issue(
                &ledger, read, batch.ordinary_box_coverage(), brands,
            ) {
                Ok(identity) => identity,
                Err(NamedArrayRequirementIssueV1::NotNamedArray) => continue,
                Err(issue) => return Err(refuse(format!("[freeze:contract][array-holder/source-membership]: {issue:?}"))),
            };
            if identity.binding() == binding && identity.owner() == site.owner()
                && identity.construction() == site.site()
            { return Ok(true); }
        }
    }
    Ok(false)
}

/// Borrow the existing selected element row first. A bare construction can
/// retain an initializer obligation, but cannot escape a selected row's refusal.
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::coseal_issue) fn named_array_initializer_source_v1(
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    brands: Option<&crate::analysis::brand_program_declaration_catalog::VerifiedBrandProgramDeclarationCatalogV1>,
    source: &super::super::super::lexical_instance_call::PreparedBorrowedFormalIngressV1,
    site: &OwnedExprSiteV1, binding: BindingRefV1,
) -> Result<Option<crate::mir::normal_callable_semantic_package::NamedArrayHolderSourceLoanV1>, OrdinaryNewCoSealIssueV1> {
    let refuse = |issue: String| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress { site: site.clone(), issue };
    if site.owner() != input.owner() || binding.owner() != input.owner() {
        return Err(refuse("[freeze:contract][array-holder/source-membership]".into()));
    }
    if let Some(loan) = source.named_array_holder_source_v1(site, binding, true).map_err(&refuse)? {
        return Ok(Some(loan));
    }
    let Some(brands) = brands else { return Ok(None) };
    let ledger = input.forest().callable_source_ledger(input.owner())
        .map_err(|issue| refuse(format!("[freeze:contract][array-holder/source-ledger]: {issue:?}")))?;
    let identity = match NamedArrayConstructionSourceV1::issue_for_binding(&ledger, binding, batch.ordinary_box_coverage(), brands) {
        Ok(identity) => identity,
        Err(NamedArrayRequirementIssueV1::NotNamedArray | NamedArrayRequirementIssueV1::InitializerMissing
            | NamedArrayRequirementIssueV1::ConstructionMissing | NamedArrayRequirementIssueV1::DeclarationCollision
            | NamedArrayRequirementIssueV1::ReassignedReceiver | NamedArrayRequirementIssueV1::ConstructorShape) => return Ok(None),
        Err(issue) => return Err(refuse(format!("[freeze:contract][array-holder/source-membership]: {issue:?}"))),
    };
    if identity.construction() != site.site() { return Err(refuse("[freeze:contract][array-holder/source-membership]".into())); }
    let empty_transfer = input.body_shape().and_then(|shape|
        crate::mir::resolved_semantics::NamedArrayEmptyTransferSourceV1::issue(&ledger, shape, &identity));
    Ok(Some(crate::mir::normal_callable_semantic_package::NamedArrayHolderSourceLoanV1::from_construction(identity, empty_transfer)))
}
