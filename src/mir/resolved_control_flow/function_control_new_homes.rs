//! Completion forwarding for source-issued ordinary-New Home/terminal relations.
//! The source scanner owns selection; this owner only binds its cleanup to Completion.
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::BorrowedViewUseRequestV1;

pub(crate) fn verify_function_completion_with_new_homes_and_argument_observations_v1<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    base_completion: Option<Result<VerifiedFunctionCompletionV1, FunctionCompletionVerificationErrorV1>>,
    selected: &std::collections::BTreeMap<
        crate::mir::resolved_semantics::OwnedExprSiteV1,
        crate::mir::resolved_semantics::BindingRefV1,
    >,
    parameters: impl IntoIterator<
        Item = (
            u32,
            crate::mir::resolved_semantics::BindingRefV1,
            crate::mir::callable_parameter_contract::CallableParameterContractKindV1,
            crate::mir::resolved_semantics::HomeDemandV1,
        ),
    >,
    entry_home: Option<&crate::mir::resolved_semantics::VerifiedInstanceEntryHomeLoanV1>,
    source_value_class_facts: Option<std::rc::Rc<crate::mir::normal_callable_semantic_package::SourceValueClassFactsV1>>,
    map_compatible: &mut impl FnMut(
        &crate::mir::resolved_semantics::OwnedExprSiteV1,
        crate::mir::resolved_semantics::BindingRefV1,
    ) -> Result<bool, E>,
    terminal_call: &mut impl FnMut(&crate::mir::resolved_semantics::OwnedExprSiteV1) -> Result<bool, E>,
    local_call_route: &mut impl FnMut(
        &crate::mir::resolved_semantics::OwnedExprSiteV1,
    ) -> Result<
        Option<crate::mir::resolved_semantics::home_new_prefix::LocalCallRouteV1>,
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
    scalar_field: &mut impl crate::mir::resolved_semantics::home_new_prefix::HomeFieldSourceConsultV1<E>,
    container_field: &mut impl FnMut(
        &crate::mir::resolved_semantics::OwnedExprSiteV1,
        &crate::mir::resolved_semantics::SourceExprSiteV1,
        crate::mir::resolved_semantics::BindingRefV1,
        crate::mir::resolved_semantics::BindingRefV1,
        &str,
    ) -> Result<bool, E>,
    array_i64_field: &mut impl FnMut(
        &crate::mir::resolved_semantics::OwnedExprSiteV1,
        &crate::mir::resolved_semantics::SourceExprSiteV1,
        crate::mir::resolved_semantics::BindingRefV1,
        crate::mir::resolved_semantics::BindingRefV1,
        &str,
    ) -> Result<bool, E>,
    // The issuer's `formal.<field>` index proof — the scanner supplies
    // the exact read site and the receiver's resolved parameter binding;
    // the predicate alone decides the unique-declaration proof.
    formal_i64_field: &mut impl FnMut(
        &crate::mir::resolved_semantics::OwnedExprSiteV1,
        &crate::mir::resolved_semantics::SourceExprSiteV1,
        crate::mir::resolved_semantics::BindingRefV1,
        crate::mir::resolved_semantics::BindingRefV1,
        &str,
    ) -> Result<bool, E>,
    local_field_read: &mut impl FnMut(
        &[crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadRequestV1],
        crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadDemandV1,
    ) -> Result<
        Option<Vec<crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1>>,
        E,
    >,
    borrowed_actuals: &mut impl FnMut(
        &crate::mir::resolved_semantics::OwnedExprSiteV1,
        crate::mir::resolved_semantics::home_new_prefix::BorrowedCallActualRequestV1<'_>,
    ) -> Result<
        Option<crate::mir::resolved_semantics::home_new_prefix::BorrowedCallArgumentsV1>,
        E,
    >,
    // The issuer's dominated-view use membership — `true` only when the
    // sealed borrowed-formal draft admits an `ArrayElementValue`,
    // `AddOperand`, or `NewArgument` value use at this exact leaf site.
    // Coverage consult only; the draft stays the sole admission authority.
    view_use: &mut impl FnMut(
        &crate::mir::resolved_semantics::OwnedExprSiteV1,
        BorrowedViewUseRequestV1<'_>,
    ) -> Result<bool, E>,
    loop_prestate: &mut impl FnMut(
        crate::mir::resolved_semantics::home_new_prefix::LoopI64PreStateRequestV1,
    ) -> Result<
        Option<crate::mir::resolved_semantics::home_new_prefix::SelectedLoopHomeLoanV1>,
        E,
    >,

    object_return: &mut impl FnMut(
        &crate::mir::resolved_semantics::OwnedExprSiteV1,
    ) -> Result<
        Option<crate::mir::normal_callable_semantic_package::ObjectReturnCallQualificationV1>,
        E,
    >,
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
    let mut completion = match base_completion.unwrap_or_else(|| verify_function_completion_v1(input)) {
        Ok(completion) => completion,
        Err(error) => return Ok(Err(error)),
    };
    if completion.owner() != input.owner() {
        return Ok(Err(FunctionCompletionVerificationErrorV1::OwnerClosureMismatch));
    }
    let (prefixes, homes, terminal_relations, argument_observations, result_prefixes) =
        crate::mir::resolved_semantics::home_new_prefix::scan_new_home_flow(
            input,
            selected,
            parameters,
            entry_home,
            source_value_class_facts,
            completion.explicit_sites(),
            completion.implicit_body_end().is_some(),
            result_sites,

            map_compatible,
            terminal_call,
            local_call_route,
            argument_i64_field,
            scalar_field,
            container_field,
            array_i64_field,
            formal_i64_field,
            local_field_read,
            borrowed_actuals,
            view_use,
            loop_prestate,
            object_return,
        )?;
    completion.attach_root_home_flow(homes);
    Ok(Ok((
        completion,
        prefixes,
        terminal_relations,
        argument_observations,
        result_prefixes,
    )))
}
