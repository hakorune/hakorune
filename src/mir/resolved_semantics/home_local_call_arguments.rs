//! Ordered lexical I64 argument sealing inside the existing source-flow owner.
//! Borrowed demand, strict scalar sealing and nested calls share one recursion.
use super::*;

pub(super) fn seal_lexical_i64_arguments_at<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &OwnedExprSiteV1,
    prior_homes: &[BindingRefV1],
    locals: &PrefixLocalFlow<'_>,
    allow_strict: bool,
    request: BorrowedCallActualRequestV1<'_>,
    is_selected_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<bool, E>,
    borrowed_arguments: &mut impl FnMut(
        &OwnedExprSiteV1,
        crate::mir::resolved_semantics::home_new_prefix::BorrowedCallActualRequestV1<'_>,
    ) -> Result<
        Option<crate::mir::resolved_semantics::home_new_prefix::BorrowedCallArgumentsV1>,
        E,
    >,
) -> Result<Option<Vec<LocalCallArgumentV1>>, E> {
    let Some((observed_site, call)) = input
        .function()
        .method_calls()
        .find(|(observed_site, _)| *observed_site == site.site())
    else {
        return Ok(None);
    };
    if observed_site != site.site() {
        return Ok(None);
    }
    if !matches!(call.receiver(),
        ResolvedMethodCallReceiverSourceV1::Lexical(ResolvedLexicalRefV1::Local(binding))
            if binding.owner() == input.owner())
    {
        // Membership comes only from the selected package callback. The
        // original field/Me spelling is corroborated here, never a class guess.
        let stored = input.body_shape().is_some_and(|shape| {
            let Some(crate::mir::resolved_semantics::BodyExpressionShapeV1::FieldAccess {
                object,
                ..
            }) = shape.expression_shape(call.receiver_site())
            else {
                return false;
            };
            matches!(shape.expression_shape(object),
                Some(crate::mir::resolved_semantics::BodyExpressionShapeV1::Me {
                    receiver: crate::mir::resolved_semantics::BodyMeReceiverV1::Lexical(binding), ..
                }) if binding.owner() == input.owner())
        });
        if !stored {
            return Ok(None);
        }
        return Ok(borrowed_arguments(site, request)?
            .and_then(|row| match row {
                BorrowedCallArgumentsV1::Scalar(arguments) => Some(arguments),
                BorrowedCallArgumentsV1::StaticSource(_)
                | BorrowedCallArgumentsV1::Object { .. } => None,
            })
            .filter(|arguments| super::borrowed_actuals::contains_borrowed_actual_v1(arguments))
            .map(|arguments| arguments.into_vec()));
    }
    seal_i64_call_arguments(
        input,
        locals,
        call,
        prior_homes,
        allow_strict,
        request,
        is_selected_call,
        borrowed_arguments,
    )
}

/// Demand the selected borrowed projection first, without re-observing locals.
/// Only positively unselected sites may use strict-I64 sealing. Its Integer/
/// Scalar arguments and direct argument-call recursion retain source order
/// and the enclosing live-Home set. No other subtree is accepted here.
fn seal_i64_call_arguments<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    locals: &PrefixLocalFlow<'_>,
    call: &crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1,
    prior_homes: &[BindingRefV1],
    allow_strict: bool,
    request: BorrowedCallActualRequestV1<'_>,
    is_selected_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<bool, E>,
    borrowed_arguments: &mut impl FnMut(
        &OwnedExprSiteV1,
        crate::mir::resolved_semantics::home_new_prefix::BorrowedCallActualRequestV1<'_>,
    ) -> Result<
        Option<crate::mir::resolved_semantics::home_new_prefix::BorrowedCallArgumentsV1>,
        E,
    >,
) -> Result<Option<Vec<LocalCallArgumentV1>>, E> {
    let owned = OwnedExprSiteV1::new(input.owner(), call.site().clone());
    match borrowed_arguments(&owned, request)? {
        Some(BorrowedCallArgumentsV1::Scalar(arguments)) => return Ok(Some(arguments.into_vec())),
        Some(BorrowedCallArgumentsV1::StaticSource(_))
        | Some(BorrowedCallArgumentsV1::Object { .. }) => return Ok(None),
        None => {}
    }
    if !allow_strict || !is_selected_call(&owned)? {
        return Ok(None);
    }
    let mut arguments = Vec::with_capacity(call.arguments().len());
    for argument in call.arguments() {
        let row = match locals.observe(argument.site()) {
            Some(OrdinaryObservation::Integer(value)) => LocalCallArgumentV1::Integer(value),
            Some(OrdinaryObservation::TrivialLocal(binding, Some(SourceScalarKind::Integer))) => {
                LocalCallArgumentV1::Scalar(binding)
            }
            _ => {
                let Some(inner) = seal_argument_call(
                    input,
                    locals,
                    argument.site(),
                    prior_homes,
                    is_selected_call,
                    borrowed_arguments,
                )?
                else {
                    return Ok(None);
                };
                LocalCallArgumentV1::CallResult(Box::new(inner))
            }
        };
        arguments.push(row);
    }
    Ok(Some(arguments))
}

/// Seal one argument-position method call — the inner site must be a
/// lexical-receiver `Lexical(Local)` method call whose selected callee is
/// i64-proven by the same package predicate the outer claim used. Nested
/// deeper expression shapes (binary operands, condition operands) never
/// reach `method_calls()` here and stay unclaimed.
fn seal_argument_call<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    locals: &PrefixLocalFlow<'_>,
    site: &SourceExprSiteV1,
    prior_homes: &[BindingRefV1],
    is_selected_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<bool, E>,
    borrowed_arguments: &mut impl FnMut(
        &OwnedExprSiteV1,
        crate::mir::resolved_semantics::home_new_prefix::BorrowedCallActualRequestV1<'_>,
    ) -> Result<
        Option<crate::mir::resolved_semantics::home_new_prefix::BorrowedCallArgumentsV1>,
        E,
    >,
) -> Result<Option<ArgumentCallObservationV1>, E> {
    let owned = OwnedExprSiteV1::new(input.owner(), site.clone());
    let Some((observed_site, call)) = input
        .function()
        .method_calls()
        .find(|(observed_site, _)| *observed_site == site)
    else {
        return Ok(None);
    };
    if observed_site != site
        || !matches!(
            call.receiver(),
            ResolvedMethodCallReceiverSourceV1::Lexical(ResolvedLexicalRefV1::Local(binding))
                if binding.owner() == input.owner()
        )
    {
        return Ok(None);
    }
    let Some(inner) = seal_i64_call_arguments(
        input,
        locals,
        call,
        prior_homes,
        true,
        BorrowedCallActualRequestV1::I64ResultArguments,
        is_selected_call,
        borrowed_arguments,
    )?
    else {
        return Ok(None);
    };
    Ok(Some(ArgumentCallObservationV1::issue(
        owned,
        prior_homes.iter().copied().collect(),
        inner.into_boxed_slice(),
    )))
}
