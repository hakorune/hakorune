//! Original Loop Header/body/exit walk through the existing recursive scanner.
//! This source summary is deliberately not Home After or physical permission.
use crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1;
use super::*;
use crate::mir::resolved_semantics::BodyChildRoleV1;

#[allow(clippy::too_many_arguments)]
pub(super) fn observe<'a, E>(
    input: ResolvedFunctionLoweringInputV1<'a>,
    statement: &crate::mir::compiler::located::LocatedStmtV1<'a>,
    source: &std::rc::Rc<crate::mir::builder::RetainedCallableLoopSourceV1>,
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
) -> Result<Option<super::local_flow::source_loop::SourceLoopScanSummaryV1>, E> {
    if unavailable.is_some() || source.owner() != input.owner() || source.site() != statement.site() {
        return Ok(None);
    }
    let mut child_locals = locals.clone();
    let Some(checkpoint) = child_locals.stage_source_loop_headers_v1(std::rc::Rc::clone(source), homes, local_calls)
        else {
            if crate::config::env::builder_debug_enabled() {
                eprintln!("[source-loop] owner={:?} site={:?} Header basis unavailable", input.owner(), source.site());
            }
            return Ok(None);
        };
    let mut child_calls = path_calls.clone();
    let Some(condition) = super::condition::observe_loop(input, source, &child_locals, homes,
        local_calls, &mut child_calls, local_field_read, borrowed_actuals)? else {
        if crate::config::env::builder_debug_enabled() {
            eprintln!("[source-loop] owner={:?} site={:?} original Loop condition unavailable", input.owner(), source.site());
        }
        return Ok(None);
    };
    let Ok(body) = input.source().child_body_from_stmt(statement, BodyChildRoleV1::LoopBody)
        else { return Ok(None); };
    let mut child_homes = homes.clone();
    let mut child_covered = covered_statements.clone();
    let mut child_selected = selected_seen.clone();
    let mut child_issue = None;
    let terminated = super::scan::scan_statement_flow(
        input,
        &body,
        exit_sites,
        selected,
        result_sites,
        results,
        exit_homes,
        maps,
        local_calls,
        &mut child_calls,
        terminal_relations,
        argument_observations,
        result_prefixes,
        &mut child_locals,
        &mut child_homes,
        &mut child_covered,
        &mut child_selected,
        &mut child_issue,
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
    if !terminated && !child_locals.observe_source_loop_backedge_v1(&child_homes, &child_covered,
        &child_calls, &child_issue) { child_locals.note_source_loop_availability_v1(false); }
    child_locals.note_source_loop_availability_v1(child_issue.is_none());
    let Some(mut summary) = child_locals.finish_source_loop_headers_v1(checkpoint, condition, local_calls) else {
        if crate::config::env::builder_debug_enabled() {
            eprintln!("[source-loop] owner={:?} site={:?} scoped Header finish unavailable path={child_issue:?}", input.owner(), source.site());
        }
        return Ok(None);
    };
    if let Err(reason) = summary.validate_for_recipe_v1(input) {
        if crate::config::env::builder_debug_enabled() {
            eprintln!("[source-loop] owner={:?} function={:?} site={:?} reason={reason} path={child_issue:?} incoming={:?} control={:?} bool={:?}",
                input.owner(), crate::mir::resolved_semantics::CallableHeaderSyntaxViewV1::from_function_ast(input.source().root()).map(|header| header.name()), source.site(),
                summary.incoming.as_ref().err(), summary.control.as_ref().err(),
                summary.bool_expressions.as_ref().err());
        }
        summary.refused = true;
    }
    // Retain failed ports as well: the whole-loop verifier needs the complete
    // source census and must never turn an omitted failed path into success.
    Ok(Some(summary))
}
