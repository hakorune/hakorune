//! `.set(index, value)` element-value admission for the borrowed-view draft.
//!
//! This classifier is not ABI authority. It consumes the proven `me.<ArrayBox>`
//! receiver field and the operation owner's checked-compare dominance; the
//! physical closure still proves the routed `ArrayElementWrite` coordinate.

use std::collections::BTreeMap;

use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1;
use crate::mir::resolved_semantics::{
    BindingRefV1, BodyExpressionShapeV1, BodyMeReceiverV1, OwnedExprSiteV1,
    ResolvedAssignmentTargetV1, ResolvedLexicalRefV1, SourceExprSiteV1, SourceNodeSiteV1,
};

use super::{
    normal_integer_operand, use_dominated_by_if, BorrowedFormalUseDraftErrorV1,
    BorrowedFormalUseDraftKindV1,
};

/// `site` is the value argument of a `.set(index, value)` element write only
/// when the receiver is a proven `me.<ArrayBox>` field, the index argument
/// proves the Normal-Integer class, and an admitted checked compare of the
/// same formal owns a region dominating this use. The index operand is the
/// envelope's integer sibling; a `.set` on any other receiver surface or a
/// different argument position stays `UnresolvedArgument`.
pub(super) fn array_element_value_kind(
    input: ResolvedFunctionLoweringInputV1<'_>,
    origins: &BTreeMap<BindingRefV1, BindingRefV1>,
    constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    receiver: Option<(BindingRefV1, &crate::parser::ParserOrdinaryBoxSourceRowV1)>,
    formal: BindingRefV1,
    guards: &BTreeMap<BindingRefV1, Vec<SourceNodeSiteV1>>,
    call: &crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1,
    site: &SourceExprSiteV1,
) -> Result<Option<BorrowedFormalUseDraftKindV1>, BorrowedFormalUseDraftErrorV1> {
    let function = input.function();
    if call.owner() != input.owner() || call.selector() != "set" || call.arity() != 2 {
        return Ok(None);
    }
    let Some(shape) = input.body_shape() else {
        return Ok(None);
    };
    let Some(BodyExpressionShapeV1::FieldAccess { object, field, .. }) =
        shape.expression_shape(call.receiver_site())
    else {
        return Ok(None);
    };
    let me = match function.variable_ref(object) {
        Some(ResolvedLexicalRefV1::Local(binding)) => binding,
        _ => match shape.expression_shape(object) {
            Some(BodyExpressionShapeV1::Me {
                receiver: BodyMeReceiverV1::Lexical(binding),
                ..
            }) => *binding,
            _ => return Ok(None),
        },
    };
    if crate::mir::normal_callable_semantic_package::ordinary_new_coseal::receiver_array_field(
        constructors,
        receiver,
        &OwnedExprSiteV1::new(input.owner(), call.receiver_site().clone()),
        me,
        field,
    )
    .map_err(|_| BorrowedFormalUseDraftErrorV1::SourceIdentity)?
    .is_none()
    {
        return Ok(None);
    }
    let index = call
        .arguments()
        .iter()
        .find(|argument| argument.ordinal() == 0);
    let Some(index) = index else {
        return Ok(None);
    };
    let integer = normal_integer_operand(input, origins, constructors, receiver, index.site())?;
    let get_result = if integer {
        false
    } else {
        get_result_index(input, constructors, receiver, index.site())?
    };
    if !integer && !get_result {
        return Ok(None);
    }
    let dominated = guards
        .get(&formal)
        .into_iter()
        .flatten()
        .any(|guard| use_dominated_by_if(site.node(), guard));
    if !dominated {
        return Ok(None);
    }
    Ok(Some(BorrowedFormalUseDraftKindV1::ArrayElementValue {
        call: OwnedExprSiteV1::new(input.owner(), call.site().clone()),
    }))
}

/// `site` is a `.get`-result local for the `.set` index arm only when the
/// binding's sole initializer is a `me.<ArrayBox field>.get`/1 — the same
/// `receiver_array_field` + `CORE_METHOD_CONTRACT_ROWS_V2` authorities the
/// statement coverage consumes — and the binding is never rebound. Any
/// other initializer shape, a rebound local, a receiver outside the entry
/// proof, or a non-`get` call keeps the index outside the draft. This arm
/// is not a `normal_integer_operand` widening: it names only the exact
/// proven `.get` initializer, never a `Dynamic` element type.
fn get_result_index(
    input: ResolvedFunctionLoweringInputV1<'_>,
    constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    receiver: Option<(BindingRefV1, &crate::parser::ParserOrdinaryBoxSourceRowV1)>,
    site: &SourceExprSiteV1,
) -> Result<bool, BorrowedFormalUseDraftErrorV1> {
    let function = input.function();
    let Some(ResolvedLexicalRefV1::Local(binding)) = function.variable_ref(site) else {
        return Ok(false);
    };
    for (_, target) in function.assignment_targets() {
        if matches!(
            target,
            ResolvedAssignmentTargetV1::BindingRebind(rebound) if *rebound == binding
        ) {
            return Ok(false);
        }
    }
    let mut initializers = function
        .expression_source()
        .initializers()
        .filter(|row| row.binding() == binding);
    let Some(initializer) = initializers.next() else {
        return Ok(false);
    };
    if initializers.next().is_some() {
        return Err(BorrowedFormalUseDraftErrorV1::SourceIdentity);
    }
    let Some(call_site) = initializer.initializer_site() else {
        return Ok(false);
    };
    let Some(call) = function.method_call(call_site) else {
        return Ok(false);
    };
    if call.owner() != input.owner() || call.selector() != "get" || call.arity() != 1 {
        return Ok(false);
    }
    if crate::mir::core_method_result_kind::lookup_core_method_result_row_v2(
        "ArrayBox",
        call.selector(),
        call.arity(),
    )
    .is_none()
    {
        return Ok(false);
    }
    let Some(shape) = input.body_shape() else {
        return Ok(false);
    };
    let Some(BodyExpressionShapeV1::FieldAccess { object, field, .. }) =
        shape.expression_shape(call.receiver_site())
    else {
        return Ok(false);
    };
    let me = match function.variable_ref(object) {
        Some(ResolvedLexicalRefV1::Local(binding)) => binding,
        _ => match shape.expression_shape(object) {
            Some(BodyExpressionShapeV1::Me {
                receiver: BodyMeReceiverV1::Lexical(binding),
                ..
            }) => *binding,
            _ => return Ok(false),
        },
    };
    let proven = crate::mir::normal_callable_semantic_package::ordinary_new_coseal::receiver_array_field(
        constructors,
        receiver,
        &OwnedExprSiteV1::new(input.owner(), call.receiver_site().clone()),
        me,
        field,
    )
    .map_err(|_| BorrowedFormalUseDraftErrorV1::SourceIdentity)?;
    Ok(proven.is_some())
}
