//! Home walk for the sole verified Static I64 search-loop source role.
//! Source topology and the I64 step were verified by the existing V2
//! product; this walk observes calls and site-specific exits, not Recipe IDs.
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1;
use crate::mir::resolved_semantics::BodyChildRoleV1;

#[allow(clippy::too_many_arguments)]
pub(super) fn observe_static_i64_search_loop<'a, E>(
    input: ResolvedFunctionLoweringInputV1<'a>,
    statement: &crate::mir::compiler::located::LocatedStmtV1<'a>,
    source: &crate::mir::builder::VerifiedStaticI64LoopHomeSourceV2,
    exit_sites: &BTreeSet<SourceStmtSiteV1>,
    selected: &BTreeMap<OwnedExprSiteV1, BindingRefV1>,
    result_sites: &BTreeSet<OwnedExprSiteV1>,
    results: &mut BTreeMap<OwnedExprSiteV1, Result<CallerNewHomePrefixV1, HomePrefixUnavailableV1>>,
    exit_homes: &mut BTreeMap<SourceStmtSiteV1, Result<RootHomeExitV1, HomePrefixUnavailableV1>>,
    maps: &mut Vec<MapHomeObservation>,
    local_calls: &mut Vec<LocalCallObservationV1>,
    path_calls: &mut BTreeSet<OwnedExprSiteV1>,
    terminal_relations: &mut BTreeMap<SourceStmtSiteV1, TerminalRelationV1>,
    argument_observations: &mut BTreeMap<OwnedExprSiteV1, SelectedNewArgumentObservationV1>,
    result_prefixes: &mut BTreeMap<
        OwnedExprSiteV1,
        Result<ResultNewHomePrefixV1, HomePrefixUnavailableV1>,
    >,
    locals: &mut PrefixLocalFlow<'a>,
    homes: &mut Vec<LiveHomeV1>,
    covered_statements: &mut Vec<SourceStmtSiteV1>,
    selected_seen: &mut Vec<OwnedExprSiteV1>,
    unavailable: &mut Option<HomePrefixUnavailableV1>,
    field_is_integer: &mut impl crate::mir::resolved_semantics::home_new_prefix::HomeTerminalFieldSourceConsultV1<E>,
    map_compatible: &mut impl FnMut(&OwnedExprSiteV1, BindingRefV1) -> Result<bool, E>,
    terminal_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<bool, E>,
    local_map_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<bool, E>,
    local_handle_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<bool, E>,
    local_lexical_i64_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<bool, E>,
    local_lexical_nullable_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<bool, E>,
    local_nullable_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<bool, E>,
    local_static_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<Option<StaticI64CallClaimV1>, E>,
    argument_i64_field: &mut impl FnMut(
        &OwnedExprSiteV1,
        &SourceExprSiteV1,
        BindingRefV1,
        BindingRefV1,
        &str,
    ) -> Result<bool, E>,
    scalar_field: &mut impl HomeFieldSourceConsultV1<E>,
    container_field: &mut impl FnMut(
        &OwnedExprSiteV1,
        &SourceExprSiteV1,
        BindingRefV1,
        BindingRefV1,
        &str,
    ) -> Result<bool, E>,
    array_i64_field: &mut impl FnMut(
        &OwnedExprSiteV1,
        &SourceExprSiteV1,
        BindingRefV1,
        BindingRefV1,
        &str,
    ) -> Result<bool, E>,
    formal_i64_field: &mut impl FnMut(
        &OwnedExprSiteV1,
        &SourceExprSiteV1,
        BindingRefV1,
        BindingRefV1,
        &str,
    ) -> Result<bool, E>,
    local_field_read: &mut impl FnMut(
        &[LocalFieldReadRequestV1],
        LocalFieldReadDemandV1,
    ) -> Result<Option<Vec<LocalFieldReadResultV1>>, E>,
    borrowed_actuals: &mut impl FnMut(
        &OwnedExprSiteV1,
        BorrowedCallActualRequestV1<'_>,
    ) -> Result<Option<BorrowedCallArgumentsV1>, E>,
    view_use: &mut impl FnMut(&OwnedExprSiteV1, BorrowedViewUseRequestV1<'_>) -> Result<bool, E>,
    loop_prestate: &mut impl FnMut(
        LoopI64PreStateRequestV1,
    ) -> Result<Option<SelectedLoopHomeLoanV1>, E>,
    object_return: &mut impl FnMut(
        &OwnedExprSiteV1,
    ) -> Result<Option<ObjectReturnCallQualificationV1>, E>,
) -> Result<bool, E> {
    if source.owner() != input.owner()
        || source.loop_site() != statement.site()
        || !exit_sites.contains(source.early_return_site())
        || !exit_sites.contains(source.tail_return_site())
    {
        return Ok(false);
    }
    let Ok(condition) = input
        .source()
        .child_expr_from_stmt(statement, ExprChildRoleV1::LoopCondition)
    else {
        return Ok(false);
    };
    let Some((SourceScalarKind::Bool, header_calls)) =
        scalar_expression::observe_scalar_expression(
            input,
            condition.site(),
            locals,
            Some(SourceScalarKind::Bool),
            false,
            local_field_read,
            statement.site(),
            homes,
            unavailable.is_none(),
            local_static_call,
            &mut |operand, binary| {
                view_use(
                    operand,
                    BorrowedViewUseRequestV1::CheckedCompareOperand { binary },
                )
            },
            borrowed_actuals,
        )?
    else {
        return Ok(false);
    };
    if header_calls.len() != 1 || header_calls[0].site().site() != source.header_call_site() {
        return Ok(false);
    }
    path_calls.extend(header_calls.iter().map(|call| call.site().clone()));
    local_calls.extend(header_calls);
    let Ok(body) = input
        .source()
        .child_body_from_stmt(statement, BodyChildRoleV1::LoopBody)
    else {
        return Ok(false);
    };
    if body.statements().len() != 2 {
        return Ok(false);
    }
    let Ok(if_statement) = input.source().body_stmt(&body, 0) else {
        return Ok(false);
    };
    let Ok(step_statement) = input.source().body_stmt(&body, 1) else {
        return Ok(false);
    };
    // This call is evaluated by the If condition on both the early-return
    // and fall-through paths. The V2 product proved its exact compare shape;
    // the existing direct-value issuer supplies its source/actual evidence.
    let Some(body_call) = local_call_flow::issue_current_owner_i64_direct_value_call(
        input,
        if_statement.site(),
        source.body_call_site(),
        homes,
        locals,
        unavailable.is_none(),
        local_static_call,
        borrowed_actuals,
    )?
    else {
        return Ok(false);
    };
    path_calls.insert(body_call.site().clone());
    local_calls.push(body_call);
    covered_statements.push(if_statement.site().clone());
    let terminated = branch::observe_if_statement(
        input,
        &if_statement,
        exit_sites,
        selected,
        result_sites,
        results,
        exit_homes,
        maps,
        local_calls,
        path_calls,
        terminal_relations,
        argument_observations,
        result_prefixes,
        locals,
        homes,
        covered_statements,
        selected_seen,
        unavailable,
        field_is_integer,
        map_compatible,
        terminal_call,
        local_map_call,
        local_handle_call,
        local_lexical_i64_call,
        local_lexical_nullable_call,
        local_nullable_call,
        local_static_call,
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
    if terminated
        || unavailable.is_some()
        || !exit_homes
            .get(source.early_return_site())
            .is_some_and(Result::is_ok)
    {
        return Ok(false);
    }
    // The product proved this exact assignment is `bin = bin + 1` with no
    // owned effect. The current I64 class and Homes therefore survive both
    // the backedge and the zero-iteration path.
    covered_statements.push(step_statement.site().clone());
    Ok(true)
}
