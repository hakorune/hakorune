//! Completion forwarding for source-issued ordinary-New Home/terminal relations.
//! The source scanner owns selection; this owner only binds its cleanup to Completion.
use super::*;

/// First Completion issuance for the selected App Main New loan. Prefix and
/// terminal obligations come from the same input and one ownership walk.
pub(crate) fn verify_function_completion_with_new_homes_v1<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    selected: &std::collections::BTreeMap<
        crate::mir::resolved_semantics::OwnedExprSiteV1,
        crate::mir::resolved_semantics::BindingRefV1,
    >,
    field_is_integer: &mut impl FnMut(
        &crate::mir::resolved_semantics::OwnedExprSiteV1,
        &crate::mir::resolved_semantics::SourceExprSiteV1,
        crate::mir::resolved_semantics::BindingRefV1,
        crate::mir::resolved_semantics::BindingRefV1,
        &str,
    ) -> Result<bool, E>,
) -> Result<
    Result<
        (
            VerifiedFunctionCompletionV1,
            std::collections::BTreeMap<
                crate::mir::resolved_semantics::OwnedExprSiteV1,
                Result<
                    crate::mir::resolved_semantics::home_new_prefix::CallerNewHomePrefixV1,
                    crate::mir::resolved_semantics::home_new_prefix::HomePrefixUnavailableV1,
                >,
            >,
            Option<crate::mir::resolved_semantics::home_new_prefix::TerminalI64AddReturnV1>,
        ),
        FunctionCompletionVerificationErrorV1,
    >,
    E,
> {
    let result = verify_function_completion_with_new_homes_and_argument_observations_v1(
        input,
        selected,
        field_is_integer,
    )?;
    Ok(result.map(|(completion, prefixes, terminal, _)| {
        let terminal = match terminal {
            Some(crate::mir::resolved_semantics::home_new_prefix::TerminalRelationV1::I64Add(
                row,
            )) => Some(row),
            _ => None,
        };
        (completion, prefixes, terminal)
    }))
}

pub(crate) fn verify_function_completion_with_new_homes_and_argument_observations_v1<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    selected: &std::collections::BTreeMap<
        crate::mir::resolved_semantics::OwnedExprSiteV1,
        crate::mir::resolved_semantics::BindingRefV1,
    >,
    field_is_integer: &mut impl FnMut(
        &crate::mir::resolved_semantics::OwnedExprSiteV1,
        &crate::mir::resolved_semantics::SourceExprSiteV1,
        crate::mir::resolved_semantics::BindingRefV1,
        crate::mir::resolved_semantics::BindingRefV1,
        &str,
    ) -> Result<bool, E>,
) -> Result<
    Result<
        (
            VerifiedFunctionCompletionV1,
            std::collections::BTreeMap<
                crate::mir::resolved_semantics::OwnedExprSiteV1,
                Result<
                    crate::mir::resolved_semantics::home_new_prefix::CallerNewHomePrefixV1,
                    crate::mir::resolved_semantics::home_new_prefix::HomePrefixUnavailableV1,
                >,
            >,
            Option<crate::mir::resolved_semantics::home_new_prefix::TerminalRelationV1>,
            std::collections::BTreeMap<
                crate::mir::resolved_semantics::OwnedExprSiteV1,
                crate::mir::resolved_semantics::home_new_prefix::SelectedNewArgumentObservationV1,
            >,
        ),
        FunctionCompletionVerificationErrorV1,
    >,
    E,
> {
    let mut completion = match verify_function_completion_v1(input) {
        Ok(completion) => completion,
        Err(error) => return Ok(Err(error)),
    };
    let (prefixes, homes, terminal_relation, argument_observations) =
        crate::mir::resolved_semantics::home_new_prefix::scan_new_home_flow(
            input,
            selected,
            completion.explicit_site(),
            field_is_integer,
        )?;
    let cleanup = ResolvedCleanupObligationsV1::explicit_empty().with_terminal_homes(homes);
    match &mut completion {
        VerifiedFunctionCompletionV1::ExplicitReturn(row) => row.cleanup = cleanup,
        VerifiedFunctionCompletionV1::ExplicitReturns(row) => row.cleanup = cleanup,
        VerifiedFunctionCompletionV1::ImplicitVoid(row) => row.cleanup = cleanup,
    }
    Ok(Ok((
        completion,
        prefixes,
        terminal_relation,
        argument_observations,
    )))
}
