//! CurrentOwner Static source-only expression-value observations.
use super::super::StaticI64CallClaimV1;
use super::*;

/// Source-only value observation from the original zeroarg CurrentOwner loan.
/// This shares the statement's live Homes and never invents a local destination.
pub(in crate::mir::resolved_semantics::home_new_prefix) fn issue_static_i64_value_call<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &SourceStmtSiteV1,
    site: &SourceExprSiteV1,
    prior_homes: &[BindingRefV1],
    static_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<Option<StaticI64CallClaimV1>, E>,
) -> Result<Option<LocalCallObservationV1>, E> {
    if !site
        .node()
        .segments()
        .starts_with(statement.node().segments())
    {
        return Ok(None);
    }
    let owned = OwnedExprSiteV1::new(input.owner(), site.clone());
    let Some(claim) = static_call(&owned)? else {
        return Ok(None);
    };
    // Qualified expression contexts retain their existing admission boundary.
    if !claim.is_current_owner_zeroarg() {
        return Ok(None);
    }
    let Some((_, call)) = input
        .function()
        .method_calls()
        .find(|(observed, _)| *observed == site)
    else {
        return Ok(None);
    };
    if call.owner() != input.owner()
        || !call.arguments().is_empty()
        || !claim.corroborates_source(&owned, call.receiver(), call.arity())
    {
        return Ok(None);
    }
    Ok(Some(LocalCallObservationV1 {
        owner: input.owner(),
        statement: statement.clone(),
        site: owned,
        destination: LocalCallDestinationV1::ExpressionValue,
        prior_homes: prior_homes.iter().copied().collect(),
        arguments: Box::new([]),
        result: LocalCallResultClassV1::I64,
    }))
}

/// A direct value call with inputs uses the same original Static source row as
/// local initialization. It is source-only until the whole callee is admitted.
pub(in crate::mir::resolved_semantics::home_new_prefix) fn issue_current_owner_i64_direct_value_call<
    E,
>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &SourceStmtSiteV1,
    site: &SourceExprSiteV1,
    prior_homes: &[BindingRefV1],
    locals: &PrefixLocalFlow<'_>,
    static_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<Option<StaticI64CallClaimV1>, E>,
    borrowed_arguments: &mut impl FnMut(
        &OwnedExprSiteV1,
        BorrowedCallActualRequestV1<'_>,
    ) -> Result<Option<BorrowedCallArgumentsV1>, E>,
) -> Result<Option<LocalCallObservationV1>, E> {
    if !site
        .node()
        .segments()
        .starts_with(statement.node().segments())
    {
        return Ok(None);
    }
    let owned = OwnedExprSiteV1::new(input.owner(), site.clone());
    let Some(claim) = static_call(&owned)? else {
        return Ok(None);
    };
    if claim
        .current_owner_source_required_i64_arguments()
        .is_none()
    {
        return Ok(None);
    }
    let mut calls = input
        .function()
        .method_calls()
        .filter(|(observed, _)| *observed == site);
    let Some((_, call)) = calls.next() else {
        return Ok(None);
    };
    if calls.next().is_some()
        || call.owner() != input.owner()
        || !claim.corroborates_source(&owned, call.receiver(), call.arity())
    {
        return Ok(None);
    }
    for (call_site, actuals) in observe_borrowed_call_actuals(input, &owned, locals, true) {
        borrowed_arguments(&call_site, BorrowedCallActualRequestV1::Observe(&actuals))?;
    }
    project_current_owner_i64_staged_value_call(
        input,
        statement,
        site,
        prior_homes,
        static_call,
        borrowed_arguments,
    )
}

/// Reuse the direct-value issuer after an enclosing expression has already
/// staged the original call actual. This projects source evidence only; it
/// never observes a second actual or publishes a physical packet.
pub(in crate::mir::resolved_semantics::home_new_prefix) fn project_current_owner_i64_staged_value_call<
    E,
>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &SourceStmtSiteV1,
    site: &SourceExprSiteV1,
    prior_homes: &[BindingRefV1],
    static_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<Option<StaticI64CallClaimV1>, E>,
    borrowed_arguments: &mut impl FnMut(
        &OwnedExprSiteV1,
        BorrowedCallActualRequestV1<'_>,
    ) -> Result<Option<BorrowedCallArgumentsV1>, E>,
) -> Result<Option<LocalCallObservationV1>, E> {
    if !site
        .node()
        .segments()
        .starts_with(statement.node().segments())
    {
        return Ok(None);
    }
    let owned = OwnedExprSiteV1::new(input.owner(), site.clone());
    let Some(claim) = static_call(&owned)? else {
        return Ok(None);
    };
    if claim
        .current_owner_source_required_i64_arguments()
        .is_none()
    {
        return Ok(None);
    }
    let mut calls = input
        .function()
        .method_calls()
        .filter(|(observed, _)| *observed == site);
    let Some((_, call)) = calls.next() else {
        return Ok(None);
    };
    if calls.next().is_some()
        || call.owner() != input.owner()
        || !claim.corroborates_source(&owned, call.receiver(), call.arity())
    {
        return Ok(None);
    }
    let Some(BorrowedCallArgumentsV1::StaticSource(arguments)) = borrowed_arguments(
        &owned,
        BorrowedCallActualRequestV1::CurrentOwnerStaticSourceArguments(&claim),
    )?
    else {
        return Ok(None);
    };
    if arguments.len() != call.arguments().len()
        || arguments
            .iter()
            .zip(call.arguments())
            .any(|(argument, original)| match argument {
                LocalCallArgumentV1::BorrowedActual { ordinal, site } => {
                    *ordinal != original.ordinal() || site != original.site()
                }
                LocalCallArgumentV1::Integer(_) => false,
                LocalCallArgumentV1::Scalar(binding) => binding.owner() != input.owner(),
                _ => true,
            })
    {
        return Ok(None);
    }
    Ok(Some(LocalCallObservationV1 {
        owner: input.owner(),
        statement: statement.clone(),
        site: owned,
        destination: LocalCallDestinationV1::ExpressionValue,
        prior_homes: prior_homes.iter().copied().collect(),
        arguments,
        result: LocalCallResultClassV1::I64,
    }))
}
