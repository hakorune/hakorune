//! Source observation of the existing bounded lexical local call.
use super::*;

/// Issue one exact literal-argument lexical instance Call (`recv.m(...)`)
/// from already-resolved source. The caller supplies the same
/// selected-call predicate; the receiver must be a resolver-sealed lexical
/// local — this helper never resolves a target or guesses a receiver class.
pub(crate) fn issue_lexical_local_call<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &SourceStmtSiteV1,
    site: &OwnedExprSiteV1,
    declaration: SourceBindingSiteV1,
    destination: BindingRefV1,
    prior_homes: &[BindingRefV1],
    result: LocalCallResultClassV1,
    is_selected_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<bool, E>,
) -> Result<Option<LocalCallObservationV1>, E> {
    let self_receiver = input.function().method_call(site.site()).is_some_and(|call| {
        matches!(call.receiver(), ResolvedMethodCallReceiverSourceV1::Lexical(ResolvedLexicalRefV1::Local(binding))
            if input.function().binding(binding).is_some_and(|row|
                matches!(row.kind(), crate::mir::resolved_semantics::BindingKindV1::Receiver)))
    });
    if self_receiver {
        return issue_receiver_local_call(
            input,
            statement,
            site,
            declaration,
            destination,
            prior_homes,
            result,
            is_selected_call,
        );
    }
    if !is_selected_call(site)? {
        return Ok(None);
    }
    let Some((observed_site, call)) = input
        .function()
        .method_calls()
        .find(|(observed_site, _)| *observed_site == site.site())
    else {
        return Ok(None);
    };
    if observed_site != site.site()
        || !matches!(
            call.receiver(),
            super::ResolvedMethodCallReceiverSourceV1::Lexical(
                super::ResolvedLexicalRefV1::Local(binding)
            ) if binding.owner() == input.owner()
        )
    {
        return Ok(None);
    }
    let Some(arguments) = call
        .arguments()
        .iter()
        .map(|argument| {
            match input
                .function()
                .expression_source()
                .literal(argument.site())
            {
                Some(ResolvedLiteralSourceV1::Integer(value)) => {
                    Some(LocalCallArgumentV1::Integer(*value))
                }
                _ => None,
            }
        })
        .collect::<Option<Vec<_>>>()
    else {
        return Ok(None);
    };
    Ok(Some(LocalCallObservationV1::issue(
        input.owner(),
        statement.clone(),
        site.clone(),
        declaration,
        destination,
        prior_homes.iter().copied().collect(),
        arguments.into_boxed_slice(),
        result,
    )))
}
