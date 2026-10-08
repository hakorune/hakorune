//! Normal Integer call-child source lent by the existing static index issuer.
//! No entry, physical result value, argument proof or phase promotion is issued.
use super::*;
use crate::mir::normal_callable_semantic_package::qualified_static_call_claim::{
    incoming_source::StaticIncomingSourceV1, QualifiedStaticCallClaimIndexV1,
};
use crate::mir::normal_callable_semantic_package::selected_mapping::VerifiedSelectedCallableBatchMapV1;
use crate::mir::resolved_semantics::ResolvedMethodCallReceiverSourceV1;
use hakorune_mir_defs::CanonicalSameModuleCallableKeyV1;

pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call)
struct StaticOperandContextV1
<'a> {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) index:
        &'a QualifiedStaticCallClaimIndexV1,
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) selected:
        &'a VerifiedSelectedCallableBatchMapV1,
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) contracts:
        &'a [OwnedCallableParameterContractDeclarationV1],
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) caller:
        &'a CanonicalSameModuleCallableKeyV1,
}

#[derive(Debug, Clone)]
pub(super) struct IntegerCallOperandSourceV1 {
    original: Rc<StaticIncomingSourceV1>,
}

impl PartialEq for IntegerCallOperandSourceV1 {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.original, &other.original)
    }
}
impl Eq for IntegerCallOperandSourceV1 {}

impl IntegerCallOperandSourceV1 {
    pub(super) fn original(&self) -> &Rc<StaticIncomingSourceV1> {
        &self.original
    }
}

pub(super) fn integer_call_operand_source_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &SourceExprSiteV1,
    context: Option<&StaticOperandContextV1<'_>>,
) -> Result<Option<IntegerCallOperandSourceV1>, BorrowedFormalUseDraftErrorV1> {
    let Some(context) = context else {
        return Ok(None);
    };
    let mut calls = input
        .function()
        .method_calls()
        .filter(|(observed, _)| *observed == site);
    let Some((_, source)) = calls.next() else {
        return Ok(None);
    };
    if calls.next().is_some() || source.owner() != input.owner() {
        return Err(BorrowedFormalUseDraftErrorV1::SourceIdentity);
    }
    if source.receiver() != ResolvedMethodCallReceiverSourceV1::CurrentOwner
        || !source.arguments().is_empty()
    {
        return Ok(None);
    }
    let Some(row) = context.index.current_owner_source(context.caller, site) else {
        return Ok(None);
    };
    let crate::mir::callable_result_representation::VerifiedCallableResultDispositionV1::ExactI64 {
        required_i64_arguments,
    } = row.result()
    else {
        return Ok(None);
    };
    if !required_i64_arguments.is_empty() {
        return Ok(None);
    }
    let owned = OwnedExprSiteV1::new(input.owner(), site.clone());
    let Some(loan) = context
        .index
        .incoming_source(
            context.caller,
            &owned,
            source,
            context.selected,
            context.contracts,
            None,
        )
        .map_err(|_| BorrowedFormalUseDraftErrorV1::SourceIdentity)?
    else {
        return Ok(None);
    };
    if !loan.contract().parameters.is_empty() {
        return Err(BorrowedFormalUseDraftErrorV1::SourceIdentity);
    }
    let original = loan.retain();
    if !loan.corroborates_retained(&original)
        || original.call_site() != &owned
        || original.is_qualified()
    {
        return Err(BorrowedFormalUseDraftErrorV1::SourceIdentity);
    }
    Ok(Some(IntegerCallOperandSourceV1 { original }))
}
