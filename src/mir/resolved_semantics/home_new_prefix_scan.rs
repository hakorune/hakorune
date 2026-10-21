//! Per-statement source walk inside `scan_new_home_flow`: the loop owns no
//! separate authority — every observation still lands in the caller's rows.
use crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1;
use super::SelectedLoopHomeLoanV1;
use super::*;

/// Returns `true` when this path terminated at an explicit exit site —
/// statements past it are unreachable on this path and are never walked.
#[allow(clippy::too_many_arguments)]
pub(super) fn scan_statement_flow<'a, E>(
    input: ResolvedFunctionLoweringInputV1<'a>,
    body: &crate::mir::compiler::located::LocatedBodyV1<'a>,
    exit_sites: &BTreeSet<SourceStmtSiteV1>,
    selected: &BTreeMap<OwnedExprSiteV1, BindingRefV1>,
    result_sites: &BTreeSet<OwnedExprSiteV1>,
    results: &mut BTreeMap<OwnedExprSiteV1, Result<CallerNewHomePrefixV1, HomePrefixUnavailableV1>>,
    exit_homes: &mut BTreeMap<SourceStmtSiteV1, Result<RootHomeExitV1, HomePrefixUnavailableV1>>,
    maps: &mut Vec<MapHomeObservation>,
    local_calls: &mut Vec<LocalCallObservationV1>,
    // The local-call sites observed on this path — cloned per branch,
    // unioned at a fall-through join, and snapshotted into each exit row
    // so the consumer can attribute binding groups per exit.
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
    // The issuer's lexical instance-call membership for the exact-i64
    // result lane — `local x = recv.m(..)` sites whose uniquely selected
    // callee returns only literals carry an `I64` local-call claim.
    local_lexical_i64_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<bool, E>,
    // The issuer's lexical instance-call membership for the nullable-result
    // lane — `local x = recv.m(..)` sites whose uniquely selected callee
    // carries the sealed `NullableObject` claim join the checked-release
    // lane instead of the scalar lane.
    local_lexical_nullable_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<bool, E>,
    local_nullable_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<bool, E>,
    // The issuer's qualified static-box call membership — `local x =
    // Alias.m(..)` sites sealed `ExactI64` join the I64 lane as ordinary
    // non-lifecycle local-call claims.
    local_static_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<Option<StaticI64CallClaimV1>, E>,
    // The issuer's argument-position i64-field proof: the scanner supplies
    // the exact read site, receiver site, receiver binding, and observed
    // handle root; the predicate alone decides the field class.
    argument_i64_field: &mut impl FnMut(
        &OwnedExprSiteV1,
        &SourceExprSiteV1,
        BindingRefV1,
        BindingRefV1,
        &str,
    ) -> Result<bool, E>,
    // The issuer's receiver-side scalar-field proof for `me.<field>` reads
    // inside a field-write RHS — same evidence shape, separate contract.
    scalar_field: &mut impl HomeFieldSourceConsultV1<E>,
    // The issuer's receiver-side container-field proof for `me.<field>`
    // receivers of builtin container calls — a separate contract.
    container_field: &mut impl FnMut(
        &OwnedExprSiteV1,
        &SourceExprSiteV1,
        BindingRefV1,
        BindingRefV1,
        &str,
    ) -> Result<bool, E>,
    // The issuer's receiver-side `ArrayBox` field proof — the
    // predicate alone consults the sealed whole-Box element-integer
    // census; the scanner carries no element facts.
    array_i64_field: &mut impl FnMut(
        &OwnedExprSiteV1,
        &SourceExprSiteV1,
        BindingRefV1,
        BindingRefV1,
        &str,
    ) -> Result<bool, E>,
    // The issuer's `formal.<field>` index proof — the scanner supplies
    // the exact read site and the receiver's resolved parameter binding;
    // the predicate alone decides the unique-declaration proof.
    formal_i64_field: &mut impl FnMut(
        &OwnedExprSiteV1,
        &SourceExprSiteV1,
        BindingRefV1,
        BindingRefV1,
        &str,
    ) -> Result<bool, E>,
    // The issuer's local-initializer `receiver.field` read membership —
    // the scanner supplies the exact read site, receiver site, receiver
    // binding, movable root, and any field-read alias class; the
    // predicate alone decides the field declaration and result class.
    local_field_read: &mut impl FnMut(
        &[LocalFieldReadRequestV1],
        LocalFieldReadDemandV1,
    ) -> Result<Option<Vec<LocalFieldReadResultV1>>, E>,
    borrowed_actuals: &mut impl FnMut(
        &OwnedExprSiteV1,
        BorrowedCallActualRequestV1<'_>,
    ) -> Result<Option<BorrowedCallArgumentsV1>, E>,
    // The issuer's dominated-view use membership — `true` only when the
    // sealed borrowed-formal draft admits an `ArrayElementValue`,
    // `AddOperand`, or `NewArgument` value use at this exact leaf site.
    // Coverage consult only; the draft stays the sole admission authority.
    view_use: &mut impl FnMut(&OwnedExprSiteV1, BorrowedViewUseRequestV1<'_>) -> Result<bool, E>,
    loop_prestate: &mut impl FnMut(
        LoopI64PreStateRequestV1,
    ) -> Result<Option<SelectedLoopHomeLoanV1>, E>,

    object_return: &mut impl FnMut(
        &OwnedExprSiteV1,
    ) -> Result<Option<ObjectReturnCallQualificationV1>, E>,
) -> Result<bool, E> {
    let function = input.function();
    locals.begin_source_loop_body_v1(body);
    for index in 0..body.statements().len() {
        let Ok(statement) = input.source().body_stmt(body, index) else {
            *unavailable = Some(HomePrefixUnavailableV1::SourceMismatch);
            return Ok(false);
        };
        covered_statements.push(statement.site().clone());
        locals.visit_source_loop_statement_v1(body, statement.site(), index);
        locals.invalidate_dynamic_field_sources(&statement);
        local_call_flow::stage_statement_actuals(input, &statement, locals,
            unavailable.is_none(), homes, local_calls, local_field_read, borrowed_actuals)?;

        if let Some(available) = locals.observe_source_loop_exit_v1(statement.site(), homes,
            covered_statements, path_calls, unavailable) {
            if !available { unavailable.get_or_insert(HomePrefixUnavailableV1::SourceMismatch); }
            locals.note_source_loop_availability_v1(available && unavailable.is_none());
            locals.end_source_loop_body_v1(body, true);
            return Ok(true);
        }

        if exit_sites.contains(statement.site()) {
            super::terminal::observe_terminal_statement(
                input,
                &statement,
                result_sites,
                results,
                exit_homes,
                path_calls,
                maps,
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
                argument_i64_field,
                local_lexical_i64_call,
                borrowed_actuals,
                view_use,
                object_return,
                local_calls,
                local_static_call,
                local_field_read,
            )?;
            locals.record_source_loop_return_v1(statement.site(), terminal_relations.get(statement.site()),
                exit_homes.get(statement.site()), local_calls);
            locals.note_source_loop_availability_v1(unavailable.is_none());
            locals.end_source_loop_body_v1(body, true);
            return Ok(true);
        }
        if matches!(statement.node(), ASTNode::If { .. }) {
            let terminated = super::branch::observe_if_statement(
                input,
                &statement,
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
            if terminated {
                locals.end_source_loop_body_v1(body, true);
                return Ok(true);
            }
            continue;
        }
        if matches!(statement.node(), ASTNode::Print { .. }) {
            if !super::print::observe(input, &statement, locals, homes, unavailable.is_none(),
                local_calls, path_calls, local_field_read, borrowed_actuals)? {
                unavailable.get_or_insert_with(|| HomePrefixUnavailableV1::PrefixNotCovered(statement.site().clone()));
            } else {
                locals.record_source_loop_expression_v1(statement.site());
            }
            continue;
        }
        let ASTNode::Local { .. } = statement.node()
        else {
            if super::binding_rebind::observe(input, &statement, locals, homes, unavailable.is_none(),
                local_calls, path_calls, local_field_read, borrowed_actuals)? {
                locals.record_source_loop_expression_v1(statement.site());
                continue;
            }
            if map_flow::observe_field_install_v1(input, &statement, locals, homes, unavailable.is_none(),
                local_calls, maps, borrowed_actuals)? {
                continue;
            }
            if super::named_array::observe_append(
                input,
                &statement,
                locals,
                homes,
                covered_statements,
                unavailable,
                borrowed_actuals,
            )? {
                continue;
            }
            if matches!(statement.node(), ASTNode::MethodCall { .. }) {
                if let Some(call) = local_call_flow::issue_lexical_i64_discard_call(
                    input,
                    &statement,
                    homes,
                    locals,
                    local_lexical_i64_call,
                    borrowed_actuals,
                )? {
                    locals.record_source_loop_call_v1(statement.site(), &call);
                    path_calls.insert(call.site().clone());
                    local_calls.push(call);
                    continue;
                }
            }
            // The source effect and its original RHS plan share this Home
            // walk. The existing raw writer remains the sole FieldSet emitter.
            if field_write::observe(
                input,
                &statement,
                locals,
                homes, unavailable.is_none(), local_calls,
                scalar_field,
                view_use,
                local_field_read,
            )? {
                continue;
            }
            // A `me.<ArrayBox field>.m(..)` statement whose manifest row
            // returns `NoValue` and whose arguments are Home-neutral is
            // covered without ledger rows — the raw lane already owns
            // `Callee::Method{RuntimeData}`/`ArrayElementWrite` emission.
            if let Some(ready) = field_call::observe_text_append(input, &statement, locals, homes, unavailable.is_none(),
                local_calls, scalar_field, borrowed_actuals)? {
                if !ready { unavailable.get_or_insert_with(|| HomePrefixUnavailableV1::PrefixNotCovered(statement.site().clone())); }
                continue;
            }
            if field_call::observe_statement_field_call(
                input,
                &statement,
                locals,
                homes,
                container_field,
                array_i64_field,
                formal_i64_field,
                scalar_field,
                view_use,
            )? {
                continue;
            }
            // The current Home prefix lends proven scalar classes at the
            // exact Loop point. Only a complete source-bound After loan can
            // advance this walk; a typed refusal keeps the old first-stop.
            if matches!(statement.node(), ASTNode::Loop { .. }) && unavailable.is_none() {
                if let Some(request) = locals.loop_i64_prestate(statement.site(), homes) {
                    match loop_prestate(request)? {
                        Some(SelectedLoopHomeLoanV1::NoExitArrayAfter(loan)) => {
                            if let Some(current) = locals.loop_i64_prestate(statement.site(), homes) {
                                if locals.install_no_exit_array_after(current, loan) { continue; }
                            }
                        }
                        Some(SelectedLoopHomeLoanV1::VariableAccumAfter(loan)) => {
                            // Re-observe at this same point: the callback does
                            // not mutate Home state before installing carriers.
                            if let Some(current) = locals.loop_i64_prestate(statement.site(), homes) {
                                if locals.install_loop_i64_after(current, loan) {
                                    continue;
                                }
                            }
                        }
                        Some(SelectedLoopHomeLoanV1::StaticI64Search(source)) => {
                            if let Some(current) = locals.loop_i64_prestate(statement.site(), homes) {
                                if current.proves_integer(source.bin_binding())
                                    && current.proves_integer(source.n_binding())
                                    && super::search_loop::observe_static_i64_search_loop(
                                        input,
                                        &statement,
                                        &source,
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
                                    )?
                                {
                                    continue;
                                }
                            }
                        }
                        Some(SelectedLoopHomeLoanV1::SourceObservation(row)) => {
                            let Some(source) = row.observation_source_v1(input) else {
                                unavailable.get_or_insert_with(|| HomePrefixUnavailableV1::PrefixNotCovered(statement.site().clone()));
                                continue;
                            };
                            let summary = super::source_loop::observe(
                                input,
                                &statement,
                                &source,
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
                            if let Some(summary) = summary {
                                // Retain the exact whole-source logical product; no retry or
                                // second scan is permitted after selected failure.
                                if let Ok(product) = row.retain_source_recipe_v2(input, summary) {
                                    if locals.apply_source_loop_after_v2(product, homes) {
                                        // This is source Home continuation only. The same row's
                                        // product must still be fulfilled by the physical owner.
                                        continue;
                                    }
                                    unavailable.get_or_insert(HomePrefixUnavailableV1::SourceMismatch);
                                }
                            }
                            // WIP: the complete V2/JoinSig + physical product
                            // must consume this summary before granting After.
                            // A selected source walk never retries or stages
                            // its interior again using the pre-loop snapshot.
                            unavailable.get_or_insert_with(|| HomePrefixUnavailableV1::PrefixNotCovered(statement.site().clone()));
                            locals.note_source_loop_availability_v1(false);
                            continue;
                        }
                        None => {}
                    }
                }
            }
            // Statement kinds this lane does not admit (loop, assignment,
            // match, ...) still contain real call edges — stage their
            // actuals from the sealed call inventory on the pre-statement
            // basis before the prefix marker. Facts only, no Home joins.
            super::branch::stage_unobserved_statement_actuals(
                input,
                &statement,
                locals,
                unavailable.is_none(),
                homes,
                borrowed_actuals,
            )?;
            unavailable.get_or_insert_with(|| {
                HomePrefixUnavailableV1::PrefixNotCovered(statement.site().clone())
            });
            // Expression statements still evaluate — contained map literals
            // under the subtree consume Homes and need one flow row each.
            map_descendant_flow::observe_descendant_maps(
                input,
                statement.site(),
                locals,
                homes,
                maps,
                map_compatible,
            )?;
            continue;
        };
        super::local_statement::observe(
            input, &statement,
            selected,
            results,
            maps,
            local_calls,
            path_calls,
            argument_observations,
            locals,
            homes,
            covered_statements,
            selected_seen,
            unavailable,
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
        )?;
        let mut calls = local_calls.iter().filter(|call| call.statement() == statement.site() && !call.is_expression_value());
        if let Some(call) = calls.next() {
            if calls.next().is_none() { locals.record_source_loop_call_v1(statement.site(), call); }
            else { locals.note_source_loop_availability_v1(false); }
        } else { locals.record_source_loop_expression_v1(statement.site()); }
    }
    locals.end_source_loop_body_v1(body, false);
    Ok(false)
}
