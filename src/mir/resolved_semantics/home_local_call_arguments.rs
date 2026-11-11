//! Ordered lexical I64 argument sealing inside the existing source-flow owner.
//! Borrowed demand, strict scalar sealing and nested calls share one recursion.
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1;

pub(super) fn seal_lexical_i64_arguments_at<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &OwnedExprSiteV1,
    prior_homes: &[LiveHomeV1],
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
                BorrowedCallArgumentsV1::StoredMapGetSource(_)
                | BorrowedCallArgumentsV1::StoredMapSource(_)
                | BorrowedCallArgumentsV1::ExpressionCoreSource(_)
                | BorrowedCallArgumentsV1::FiniteEntryRead(_)
                | BorrowedCallArgumentsV1::DynamicEntryRead(_)
                | BorrowedCallArgumentsV1::TextEntryRead(_)
                | BorrowedCallArgumentsV1::ArgumentExpressionSource(_)
                | BorrowedCallArgumentsV1::ArgumentExpressionFrame(_)
                | BorrowedCallArgumentsV1::NamedArrayHolderSource(_)
                | BorrowedCallArgumentsV1::SourceInstance(_)
                | BorrowedCallArgumentsV1::StaticSource(_)
                | BorrowedCallArgumentsV1::HandleSource(_)
                | BorrowedCallArgumentsV1::SourceObject { .. }
                | BorrowedCallArgumentsV1::SourceUnitDiscard { .. }
                | BorrowedCallArgumentsV1::SourceResult { .. }
                | BorrowedCallArgumentsV1::DynamicInvocationSource { .. }
                | BorrowedCallArgumentsV1::BorrowedNullableLocalSource { .. }
                | BorrowedCallArgumentsV1::BorrowedNullableReturnSource { .. }
                | BorrowedCallArgumentsV1::MixedLocalSource { .. }
                | BorrowedCallArgumentsV1::Object { .. }
                | BorrowedCallArgumentsV1::CallEntryHomes { .. } => None,
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
    prior_homes: &[LiveHomeV1],
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
        Some(
            BorrowedCallArgumentsV1::StoredMapGetSource(_)
            | BorrowedCallArgumentsV1::StoredMapSource(_)
            | BorrowedCallArgumentsV1::ExpressionCoreSource(_)
            | BorrowedCallArgumentsV1::FiniteEntryRead(_)
            | BorrowedCallArgumentsV1::DynamicEntryRead(_),
        )
        | Some(BorrowedCallArgumentsV1::TextEntryRead(_))
        | Some(
            BorrowedCallArgumentsV1::ArgumentExpressionSource(_)
            | BorrowedCallArgumentsV1::ArgumentExpressionFrame(_),
        )
        | Some(BorrowedCallArgumentsV1::NamedArrayHolderSource(_))
        | Some(BorrowedCallArgumentsV1::SourceInstance(_))
        | Some(BorrowedCallArgumentsV1::StaticSource(_))
        | Some(BorrowedCallArgumentsV1::HandleSource(_))
        | Some(BorrowedCallArgumentsV1::SourceObject { .. })
        | Some(BorrowedCallArgumentsV1::SourceUnitDiscard { .. })
        | Some(BorrowedCallArgumentsV1::SourceResult { .. })
        | Some(BorrowedCallArgumentsV1::DynamicInvocationSource { .. })
        | Some(BorrowedCallArgumentsV1::BorrowedNullableLocalSource { .. }
                | BorrowedCallArgumentsV1::BorrowedNullableReturnSource { .. })
        | Some(BorrowedCallArgumentsV1::MixedLocalSource { .. })
        | Some(BorrowedCallArgumentsV1::Object { .. })
        | Some(BorrowedCallArgumentsV1::CallEntryHomes { .. }) => return Ok(None),
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
    prior_homes: &[LiveHomeV1],
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
    let entry = crate::mir::resolved_semantics::home_new_prefix::expression_entry_facts_at_v1(
        input, &owned, &inner, false, borrowed_arguments,
    )?;
    let mut observation = ArgumentCallObservationV1::issue(
        owned, prior_homes.iter().cloned().collect(), inner.into_boxed_slice(),
    );
    observation.entry_ownership = entry;
    Ok(Some(observation))
}

/// Issue a source-proved static scalar (I64 or Bool) continuation, preserving its original route.
///
/// The route's claim is the sole membership authority — the package returns a
/// sealed Qualified or bounded CurrentOwner claim. Both require the
/// original `StaticBoxMethod` target and `ExactI64` disposition. Arity-bearing
/// CurrentOwner arguments must project from the same original Static source;
/// qualified scalar fallback keeps its existing separate boundary.
pub(crate) fn issue_static_scalar_local_call<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &SourceStmtSiteV1,
    site: &OwnedExprSiteV1,
    declaration: SourceBindingSiteV1,
    destination: BindingRefV1,
    prior_homes: &[LiveHomeV1],
    locals: &PrefixLocalFlow<'_>,
    claim: super::super::StaticScalarCallClaimV1,
    borrowed_arguments: &mut impl FnMut(
        &OwnedExprSiteV1,
        BorrowedCallActualRequestV1<'_>,
    ) -> Result<Option<BorrowedCallArgumentsV1>, E>,
) -> Result<Option<LocalCallObservationV1>, E> {
    let Some((observed_site, call)) = input
        .function()
        .method_calls()
        .find(|(observed_site, _)| *observed_site == site.site())
    else {
        return Ok(None);
    };
    if input.owner() != site.owner()
        || call.owner() != input.owner()
        || observed_site != site.site()
        || !claim.corroborates_source(site, call.receiver(), call.arity())
    {
        return Ok(None);
    }
    let qualified_claim = claim.qualified_claim();
    let source_request = qualified_claim
        .map(BorrowedCallActualRequestV1::QualifiedStaticSourceArguments)
        .or_else(|| {
            claim
                .current_owner_source_required_i64_arguments()
                .map(|_| BorrowedCallActualRequestV1::CurrentOwnerStaticSourceArguments(&claim))
        });
    let arguments = if let Some(request) = source_request {
        match borrowed_arguments(site, request)? {
            Some(BorrowedCallArgumentsV1::StaticSource(arguments)) => {
                if arguments.len() != call.arguments().len()
                    || arguments
                        .iter()
                        .zip(call.arguments())
                        .any(|(argument, original)| match argument {
                            LocalCallArgumentV1::BorrowedActual { ordinal, site }
                            | LocalCallArgumentV1::Expression { ordinal, site } => {
                                *ordinal != original.ordinal() || site != original.site()
                            }
                            LocalCallArgumentV1::Integer(_) => false,
                            LocalCallArgumentV1::Scalar(binding) => {
                                binding.owner() != input.owner()
                            }
                            _ => true,
                        })
                {
                    return Ok(None);
                }
                arguments.into_vec()
            }
            Some(
                BorrowedCallArgumentsV1::StoredMapGetSource(_)
                | BorrowedCallArgumentsV1::StoredMapSource(_)
                | BorrowedCallArgumentsV1::ExpressionCoreSource(_)
                | BorrowedCallArgumentsV1::FiniteEntryRead(_)
                | BorrowedCallArgumentsV1::DynamicEntryRead(_),
            )
            | Some(BorrowedCallArgumentsV1::TextEntryRead(_))
            | Some(
                BorrowedCallArgumentsV1::ArgumentExpressionSource(_)
                | BorrowedCallArgumentsV1::ArgumentExpressionFrame(_),
            )
            | Some(BorrowedCallArgumentsV1::NamedArrayHolderSource(_))
            | Some(BorrowedCallArgumentsV1::SourceInstance(_))
            | Some(BorrowedCallArgumentsV1::Scalar(_))
            | Some(BorrowedCallArgumentsV1::HandleSource(_))
            | Some(BorrowedCallArgumentsV1::SourceUnitDiscard { .. })
            | Some(BorrowedCallArgumentsV1::SourceResult { .. })
            | Some(BorrowedCallArgumentsV1::DynamicInvocationSource { .. })
            | Some(BorrowedCallArgumentsV1::SourceObject { .. })
            | Some(BorrowedCallArgumentsV1::BorrowedNullableLocalSource { .. }
                | BorrowedCallArgumentsV1::BorrowedNullableReturnSource { .. })
            | Some(BorrowedCallArgumentsV1::MixedLocalSource { .. })
            | Some(BorrowedCallArgumentsV1::Object { .. })
            | Some(BorrowedCallArgumentsV1::CallEntryHomes { .. }) => return Ok(None),
            None => {
                let Some(claim) = qualified_claim else {
                    // CurrentOwner input never borrows a fallback local seal:
                    // the projected original Static Rc is mandatory.
                    return Ok(None);
                };
                let mut arguments = Vec::with_capacity(call.arguments().len());
                for argument in call.arguments() {
                    let (row, i64_evidence) = match locals.observe(argument.site()) {
                        Some(OrdinaryObservation::Integer(value)) => {
                            (LocalCallArgumentV1::Integer(value), true)
                        }
                        Some(OrdinaryObservation::Bool(value)) => {
                            (LocalCallArgumentV1::Bool(value), false)
                        }
                        Some(OrdinaryObservation::TrivialLocal(binding, Some(kind))) => (
                            LocalCallArgumentV1::Scalar(binding),
                            kind == SourceScalarKind::Integer,
                        ),
                        _ => return Ok(None),
                    };
                    if claim.required_i64_arguments().contains(&argument.ordinal()) && !i64_evidence
                    {
                        return Ok(None);
                    }
                    arguments.push(row);
                }
                arguments
            }
        }
    } else {
        if !call.arguments().is_empty() {
            return Ok(None);
        }
        Vec::new()
    };
    Ok(Some(LocalCallObservationV1::issue(
        input.owner(),
        statement.clone(),
        site.clone(),
        declaration,
        destination,
        prior_homes.iter().cloned().collect(),
        arguments.into_boxed_slice(),
        claim.result(),
    )))
}
