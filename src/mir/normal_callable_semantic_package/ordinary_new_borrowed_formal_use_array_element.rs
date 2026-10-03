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
    ResolvedLexicalRefV1, SourceExprSiteV1, SourceNodeSiteV1,
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
    if !normal_integer_operand(input, origins, constructors, receiver, index.site())? {
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
