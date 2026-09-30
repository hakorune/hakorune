//! Completion forwarding for source-issued ordinary-New Home/terminal relations.
//! The source scanner owns selection; this owner only binds its cleanup to Completion.
use super::*;

/// First Completion issuance for the parameter-free selected New loan. Prefix and
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
        std::iter::empty(),
        None,
        field_is_integer,
        &mut |_, _| Ok(false),
        &mut |_| Ok(false),
        &mut |_| Ok(false),
        &mut |_| Ok(false),
        &mut |_| Ok(false),
        &mut |_| Ok(None),
        &std::collections::BTreeSet::new(),
        &mut |_, _, _, _, _| Ok(false),
        &mut |_, _, _, _, _| Ok(false),
        &mut |_, _, _, _, _| Ok(false),
    )?;
    Ok(
        result.map(|(completion, prefixes, terminal_relations, _, _)| {
            // This compat wrapper keeps the single-exit shape: a relation row
            // surfaces only when the completion sealed exactly one exit site.
            let terminal = (terminal_relations.len() == 1)
                .then(|| terminal_relations.values().next())
                .flatten()
                .and_then(|relation| match relation {
                    crate::mir::resolved_semantics::home_new_prefix::TerminalRelationV1::I64Add(
                        row,
                    ) => Some(row.clone()),
                    _ => None,
                });
            (completion, prefixes, terminal)
        }),
    )
}

pub(crate) fn verify_function_completion_with_new_homes_and_argument_observations_v1<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    selected: &std::collections::BTreeMap<
        crate::mir::resolved_semantics::OwnedExprSiteV1,
        crate::mir::resolved_semantics::BindingRefV1,
    >,
    parameters: impl IntoIterator<
        Item = (
            u32,
            crate::mir::resolved_semantics::BindingRefV1,
            crate::mir::callable_parameter_contract::CallableParameterContractKindV1,
        ),
    >,
    entry_home: Option<&crate::mir::resolved_semantics::VerifiedInstanceEntryHomeLoanV1>,
    field_is_integer: &mut impl FnMut(
        &crate::mir::resolved_semantics::OwnedExprSiteV1,
        &crate::mir::resolved_semantics::SourceExprSiteV1,
        crate::mir::resolved_semantics::BindingRefV1,
        crate::mir::resolved_semantics::BindingRefV1,
        &str,
    ) -> Result<bool, E>,
    map_compatible: &mut impl FnMut(
        &crate::mir::resolved_semantics::OwnedExprSiteV1,
        crate::mir::resolved_semantics::BindingRefV1,
    ) -> Result<bool, E>,
    terminal_call: &mut impl FnMut(&crate::mir::resolved_semantics::OwnedExprSiteV1) -> Result<bool, E>,
    local_map_call: &mut impl FnMut(&crate::mir::resolved_semantics::OwnedExprSiteV1) -> Result<bool, E>,
    local_handle_call: &mut impl FnMut(
        &crate::mir::resolved_semantics::OwnedExprSiteV1,
    ) -> Result<bool, E>,
    local_nullable_call: &mut impl FnMut(
        &crate::mir::resolved_semantics::OwnedExprSiteV1,
    ) -> Result<bool, E>,
    local_static_call: &mut impl FnMut(
        &crate::mir::resolved_semantics::OwnedExprSiteV1,
    ) -> Result<
        Option<
            crate::mir::resolved_semantics::home_new_prefix::QualifiedStaticCallClaimV1,
        >,
        E,
    >,
    result_sites: &std::collections::BTreeSet<crate::mir::resolved_semantics::OwnedExprSiteV1>,
    argument_i64_field: &mut impl FnMut(
        &crate::mir::resolved_semantics::OwnedExprSiteV1,
        &crate::mir::resolved_semantics::SourceExprSiteV1,
        crate::mir::resolved_semantics::BindingRefV1,
        crate::mir::resolved_semantics::BindingRefV1,
        &str,
    ) -> Result<bool, E>,
    scalar_field: &mut impl FnMut(
        &crate::mir::resolved_semantics::OwnedExprSiteV1,
        &crate::mir::resolved_semantics::SourceExprSiteV1,
        crate::mir::resolved_semantics::BindingRefV1,
        crate::mir::resolved_semantics::BindingRefV1,
        &str,
    ) -> Result<bool, E>,
    container_field: &mut impl FnMut(
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
            std::collections::BTreeMap<
                crate::mir::resolved_semantics::SourceStmtSiteV1,
                crate::mir::resolved_semantics::home_new_prefix::TerminalRelationV1,
            >,
            std::collections::BTreeMap<
                crate::mir::resolved_semantics::OwnedExprSiteV1,
                crate::mir::resolved_semantics::home_new_prefix::SelectedNewArgumentObservationV1,
            >,
            std::collections::BTreeMap<
                crate::mir::resolved_semantics::OwnedExprSiteV1,
                Result<
                    crate::mir::resolved_semantics::home_new_prefix::ResultNewHomePrefixV1,
                    crate::mir::resolved_semantics::home_new_prefix::HomePrefixUnavailableV1,
                >,
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
    let (prefixes, homes, terminal_relations, argument_observations, result_prefixes) =
        crate::mir::resolved_semantics::home_new_prefix::scan_new_home_flow(
            input,
            selected,
            parameters,
            entry_home,
            completion.explicit_sites(),
            completion.implicit_body_end().is_some(),
            result_sites,
            field_is_integer,
            map_compatible,
            terminal_call,
            local_map_call,
            local_handle_call,
            local_nullable_call,
            local_static_call,
            argument_i64_field,
            scalar_field,
            container_field,
        )?;
    match &mut completion {
        VerifiedFunctionCompletionV1::ExplicitReturn(row) => row.cleanup.attach_root_flow(homes),
        VerifiedFunctionCompletionV1::ExplicitReturns(row) => row.cleanup.attach_root_flow(homes),
        VerifiedFunctionCompletionV1::ExplicitUnitSetWithImplicitEnd(row) => {
            row.cleanup.attach_root_flow(homes)
        }
        VerifiedFunctionCompletionV1::ImplicitVoid(row) => row.cleanup.attach_root_flow(homes),
    }
    Ok(Ok((
        completion,
        prefixes,
        terminal_relations,
        argument_observations,
        result_prefixes,
    )))
}
