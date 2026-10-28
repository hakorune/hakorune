//! Source observation of the existing bounded lexical local call.
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1;
use crate::mir::resolved_semantics::home_new_prefix::{
    FieldAliasPathV1, HomePrefixUnavailableV1, ObjectCallSourceSupportV1,
};

/// Record the original qualified Handle source row, or the existing positively
/// unselected literal lane. The package owns target/result/argument proof.
pub(crate) fn issue_lexical_local_call<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &SourceStmtSiteV1,
    site: &OwnedExprSiteV1,
    declaration: SourceBindingSiteV1,
    destination: BindingRefV1,
    prior_homes: &[LiveHomeV1],
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
                    prior_homes.iter().cloned().collect(),
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
        prior_homes.iter().cloned().collect(),
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
    prior_homes: &[LiveHomeV1],
    borrowed_arguments: &mut impl FnMut(
        &OwnedExprSiteV1,
        BorrowedCallActualRequestV1<'_>,
    ) -> Result<Option<BorrowedCallArgumentsV1>, E>,
) -> Result<Option<Result<LocalCallObservationV1, HomePrefixUnavailableV1>>, E> {
    issue_received_producer_local_call_with_alias_v1(
        input,
        statement,
        site,
        declaration,
        destination,
        prior_homes,
        None,
        borrowed_arguments,
    )
}

/// A declared field alias may receive an owned result without becoming an
/// owned receiver Home. Its exact alias class is checked against the original
/// resolved call target by the package source callback.
pub(crate) fn issue_received_producer_local_call_with_alias_v1<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &SourceStmtSiteV1,
    site: &OwnedExprSiteV1,
    declaration: SourceBindingSiteV1,
    destination: BindingRefV1,
    prior_homes: &[LiveHomeV1],
    receiver_alias: Option<(BindingRefV1, Box<str>, FieldAliasPathV1)>,
    borrowed_arguments: &mut impl FnMut(
        &OwnedExprSiteV1,
        BorrowedCallActualRequestV1<'_>,
    ) -> Result<Option<BorrowedCallArgumentsV1>, E>,
) -> Result<Option<Result<LocalCallObservationV1, HomePrefixUnavailableV1>>, E> {
    let receiver_binding = input
        .function()
        .method_call(site.site())
        .and_then(|call| match call.receiver() {
            ResolvedMethodCallReceiverSourceV1::Lexical(ResolvedLexicalRefV1::Local(binding)) => {
                Some(binding)
            }
            _ => None,
        });
    if receiver_alias.as_ref().is_some_and(|(binding, class, path)| {
        Some(*binding) != receiver_binding
            || binding.owner() != input.owner()
            || class.is_empty()
            || path.edges.last().map(|edge| edge.0) != Some(*binding)
    }) {
        return Ok(Some(Err(HomePrefixUnavailableV1::SourceMismatch)));
    }
    let claim_local = receiver_binding.is_some_and(|binding| {
        input.function().binding(binding).is_some_and(|row|
            matches!(row.kind(), crate::mir::resolved_semantics::BindingKindV1::Local { .. }))
            && (prior_homes.iter().any(|home| home.binding() == binding)
                || receiver_alias.as_ref().is_some_and(|(alias, _, _)| *alias == binding))
    });
    // Structural candidate only. The package must affirm the original Nullable
    // input loan before a self receiver can issue SourceObject below.
    let self_receiver = input.function().method_call(site.site()).is_some_and(|call|
        matches!(call.receiver(), ResolvedMethodCallReceiverSourceV1::Lexical(ResolvedLexicalRefV1::Local(binding))
            if binding.owner() == input.owner() && input.function().binding(binding)
                .is_some_and(|row| row.kind() == crate::mir::resolved_semantics::BindingKindV1::Receiver)));
    let formal_receiver = receiver_binding.is_some_and(|binding| binding.owner() == input.owner()
        && input.function().binding(binding).is_some_and(|row|
            matches!(row.kind(), crate::mir::resolved_semantics::BindingKindV1::Parameter { .. })));
    // Structural candidate only: `me.<field>.m(..)`. The package must affirm the
    // stored-child source and the Nullable input loan; the child is never a Home.
    let stored_receiver = receiver_alias.is_none() && input.body_shape().is_some_and(|shape| {
        use crate::mir::resolved_semantics::BodyExpressionShapeV1 as Shape;
        input.function().method_call(site.site()).is_some_and(|call|
            matches!(shape.expression_shape(call.receiver_site()),
                Some(Shape::FieldAccess { object, .. })
                    if matches!(shape.expression_shape(&object), Some(Shape::Me { .. }))))
    });
    if !claim_local && !self_receiver && !formal_receiver && !stored_receiver {
        return Ok(None);
    }
    let exact_initializer = if self_receiver || formal_receiver || stored_receiver {
        corroborate_local_initializer_v1(
            input,
            statement,
            site,
            &declaration,
            destination,
            prior_homes,
        )
    } else {
        corroborate_received_initializer_v1(
            input,
            statement,
            site,
            &declaration,
            destination,
            prior_homes,
        )
    };
    if !exact_initializer {
        return Ok(Some(Err(HomePrefixUnavailableV1::SourceMismatch)));
    }
    for requested in [
        LocalCallResultClassV1::Nullable,
        LocalCallResultClassV1::Handle,
    ] {
        let request = match &receiver_alias {
            Some((receiver, receiver_class, path)) => {
                BorrowedCallActualRequestV1::ReceivedFieldAliasObjectArguments {
                    receiver: *receiver,
                    receiver_class: receiver_class.as_ref(),
                    alias_path: path,
                    destination,
                    result: requested,
                }
            }
            None => BorrowedCallActualRequestV1::ReceivedObjectArguments(destination, requested),
        };
        let Some(response) = borrowed_arguments(
            site,
            request,
        )? else {
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
        let mut observation = LocalCallObservationV1::issue(
            input.owner(),
            statement.clone(),
            site.clone(),
            declaration,
            destination,
            prior_homes.iter().cloned().collect(),
            arguments,
            result,
        );
        observation.field_alias = receiver_alias.as_ref().map(|(_, _, path)| path.clone());
        return Ok(Some(Ok(observation)));
    }
    Ok(None)
}

#[cfg(test)]
#[path = "home_local_lexical_call_tests.rs"]
mod tests;

pub(super) fn corroborate_received_initializer_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &SourceStmtSiteV1,
    site: &OwnedExprSiteV1,
    declaration: &SourceBindingSiteV1,
    destination: BindingRefV1,
    prior_homes: &[LiveHomeV1],
) -> bool {
    corroborate_local_initializer_v1(input, statement, site, declaration, destination, prior_homes)
        && input.function().method_call(site.site()).is_some_and(|call|
            matches!(call.receiver(), ResolvedMethodCallReceiverSourceV1::Lexical(ResolvedLexicalRefV1::Local(binding))
                if binding.owner() == input.owner()
                && input.function().binding(binding).is_some_and(|row| matches!(row.kind(), crate::mir::resolved_semantics::BindingKindV1::Local { .. }))))
}

pub(super) fn corroborate_local_initializer_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &SourceStmtSiteV1,
    site: &OwnedExprSiteV1,
    declaration: &SourceBindingSiteV1,
    destination: BindingRefV1,
    prior_homes: &[LiveHomeV1],
) -> bool {
    let function = input.function();
    let Some(initializer) = function.expression_source().initializer(declaration) else {
        return false;
    };
    if function.method_call(site.site()).is_none() {
        return false;
    }
    site.owner() == input.owner()
        && destination.owner() == input.owner()
        && matches!(declaration, SourceBindingSiteV1::Local { statement: original, .. } if original == statement)
        && initializer.binding() == destination
        && initializer.initializer_site() == Some(site.site())
        && function.declaration_binding(declaration) == Some(destination)
        && prior_homes
            .iter()
            .all(|binding| binding.owner() == input.owner() && binding.binding() != destination)
}

pub(super) fn corroborate_received_arguments_v1(
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
            LocalCallArgumentV1::Expression { ordinal, site } => *ordinal == original.ordinal() && site == original.site(),
            LocalCallArgumentV1::Integer(value) => matches!(function.expression_source().literal(original.site()), Some(ResolvedLiteralSourceV1::Integer(actual)) if actual == value),
            LocalCallArgumentV1::Scalar(binding) => binding.owner() == input.owner() && function.variable_ref(original.site()) == Some(ResolvedLexicalRefV1::Local(*binding)),
            _ => false,
        })
}
