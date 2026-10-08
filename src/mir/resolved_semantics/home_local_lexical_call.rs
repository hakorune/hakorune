//! Source observation of the existing bounded lexical local call.
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::{
    HomePrefixUnavailableV1, ObjectCallSourceSupportV1,
};

/// Record the original qualified Handle source row, or the existing positively
/// unselected literal lane. The package owns target/result/argument proof.
pub(crate) fn issue_lexical_local_call<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &SourceStmtSiteV1,
    site: &OwnedExprSiteV1,
    declaration: SourceBindingSiteV1,
    destination: BindingRefV1,
    prior_homes: &[BindingRefV1],
    result: LocalCallResultClassV1,
    is_selected_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<bool, E>,
    borrowed_arguments: &mut impl FnMut(
        &OwnedExprSiteV1,
        BorrowedCallActualRequestV1<'_>,
    ) -> Result<Option<BorrowedCallArgumentsV1>, E>,
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
    let claim_local = input.function().method_call(site.site()).is_some_and(|call| {
        matches!(call.receiver(), ResolvedMethodCallReceiverSourceV1::Lexical(ResolvedLexicalRefV1::Local(binding))
            if binding.owner() == input.owner()
            && input.function().binding(binding).is_some_and(|row| matches!(row.kind(), crate::mir::resolved_semantics::BindingKindV1::Local { .. })))
    });
    if result == LocalCallResultClassV1::Handle && claim_local {
        if !corroborate_received_initializer_v1(
            input,
            statement,
            site,
            &declaration,
            destination,
            prior_homes,
        ) {
            return Ok(None);
        }
        match borrowed_arguments(
            site,
            BorrowedCallActualRequestV1::ReceivedHandleArguments(destination),
        )? {
            Some(BorrowedCallArgumentsV1::HandleSource(arguments)) => {
                if !corroborate_received_arguments_v1(input, site, &arguments) {
                    return Ok(None);
                }
                return Ok(Some(LocalCallObservationV1::issue(
                    input.owner(),
                    statement.clone(),
                    site.clone(),
                    declaration,
                    destination,
                    prior_homes.iter().copied().collect(),
                    arguments,
                    result,
                )));
            }
            Some(_) => return Ok(None),
            None => {}
        }
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

/// A selected producer failure is retained by scan and cannot retry old lanes.
/// Nullable and Handle use the same source witness and independent refusal type.
pub(crate) fn issue_received_producer_local_call<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &SourceStmtSiteV1,
    site: &OwnedExprSiteV1,
    declaration: SourceBindingSiteV1,
    destination: BindingRefV1,
    prior_homes: &[BindingRefV1],
    borrowed_arguments: &mut impl FnMut(
        &OwnedExprSiteV1,
        BorrowedCallActualRequestV1<'_>,
    ) -> Result<Option<BorrowedCallArgumentsV1>, E>,
) -> Result<Option<Result<LocalCallObservationV1, HomePrefixUnavailableV1>>, E> {
    let claim_local = input.function().method_call(site.site()).is_some_and(|call| {
        matches!(call.receiver(), ResolvedMethodCallReceiverSourceV1::Lexical(ResolvedLexicalRefV1::Local(binding))
            if prior_homes.contains(&binding)
            && input.function().binding(binding).is_some_and(|row|
                matches!(row.kind(), crate::mir::resolved_semantics::BindingKindV1::Local { .. })))
    });
    if !claim_local {
        return Ok(None);
    }
    if !corroborate_received_initializer_v1(
        input,
        statement,
        site,
        &declaration,
        destination,
        prior_homes,
    ) {
        return Ok(Some(Err(HomePrefixUnavailableV1::SourceMismatch)));
    }
    for requested in [
        LocalCallResultClassV1::Nullable,
        LocalCallResultClassV1::Handle,
    ] {
        let Some(response) = borrowed_arguments(
            site,
            BorrowedCallActualRequestV1::ReceivedObjectArguments(destination, requested),
        )?
        else {
            continue;
        };
        let refusal = || Some(Err(HomePrefixUnavailableV1::SourceMismatch));
        let BorrowedCallArgumentsV1::SourceObject { result, arguments } = response else {
            return Ok(refusal());
        };
        if result != requested {
            return Ok(refusal());
        }
        let arguments = match arguments {
            ObjectCallSourceSupportV1::Observed(arguments)
            | ObjectCallSourceSupportV1::SourceOnly(arguments) => arguments,
            ObjectCallSourceSupportV1::Unavailable => return Ok(refusal()),
        };
        if !corroborate_received_arguments_v1(input, site, &arguments) {
            return Ok(refusal());
        }
        return Ok(Some(Ok(LocalCallObservationV1::issue(
            input.owner(),
            statement.clone(),
            site.clone(),
            declaration,
            destination,
            prior_homes.iter().copied().collect(),
            arguments,
            result,
        ))));
    }
    Ok(None)
}

#[cfg(test)]
#[path = "home_local_lexical_call_tests.rs"]
mod tests;

fn corroborate_received_initializer_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &SourceStmtSiteV1,
    site: &OwnedExprSiteV1,
    declaration: &SourceBindingSiteV1,
    destination: BindingRefV1,
    prior_homes: &[BindingRefV1],
) -> bool {
    let function = input.function();
    let Some(initializer) = function.expression_source().initializer(declaration) else {
        return false;
    };
    let Some(call) = function.method_call(site.site()) else {
        return false;
    };
    site.owner() == input.owner()
        && destination.owner() == input.owner()
        && matches!(declaration, SourceBindingSiteV1::Local { statement: original, .. } if original == statement)
        && initializer.binding() == destination
        && initializer.initializer_site() == Some(site.site())
        && function.declaration_binding(declaration) == Some(destination)
        && matches!(call.receiver(), ResolvedMethodCallReceiverSourceV1::Lexical(ResolvedLexicalRefV1::Local(binding))
            if binding.owner() == input.owner()
            && function.binding(binding).is_some_and(|row| matches!(row.kind(), crate::mir::resolved_semantics::BindingKindV1::Local { .. })))
        && prior_homes
            .iter()
            .all(|binding| binding.owner() == input.owner() && *binding != destination)
}

fn corroborate_received_arguments_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &OwnedExprSiteV1,
    arguments: &[LocalCallArgumentV1],
) -> bool {
    let function = input.function();
    let Some(call) = function.method_call(site.site()) else {
        return false;
    };
    arguments.len() == call.arguments().len()
        && arguments.iter().zip(call.arguments()).all(|(argument, original)| match argument {
            LocalCallArgumentV1::BorrowedActual { ordinal, site } => *ordinal == original.ordinal() && site == original.site(),
            LocalCallArgumentV1::Integer(value) => matches!(function.expression_source().literal(original.site()), Some(ResolvedLiteralSourceV1::Integer(actual)) if actual == value),
            LocalCallArgumentV1::Scalar(binding) => binding.owner() == input.owner() && function.variable_ref(original.site()) == Some(ResolvedLexicalRefV1::Local(*binding)),
            _ => false,
        })
}
