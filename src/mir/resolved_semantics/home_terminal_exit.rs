//! Apply the selected terminal's ownership transfer and site-bound exit snapshot.
use super::*;

#[allow(clippy::too_many_arguments)]
pub(super) fn finish_terminal_v1<'a, E>(
    input: ResolvedFunctionLoweringInputV1<'a>,
    statement: &crate::mir::compiler::located::LocatedStmtV1<'a>,
    results: &BTreeMap<OwnedExprSiteV1, Result<CallerNewHomePrefixV1, HomePrefixUnavailableV1>>,
    exit_homes: &mut BTreeMap<SourceStmtSiteV1, Result<RootHomeExitV1, HomePrefixUnavailableV1>>,
    path_calls: &BTreeSet<OwnedExprSiteV1>,
    maps: &mut Vec<MapHomeObservation>,
    terminal_relations: &mut BTreeMap<SourceStmtSiteV1, TerminalRelationV1>,
    locals: &mut PrefixLocalFlow<'a>,
    homes: &mut HomesV1,
    covered_statements: &Vec<SourceStmtSiteV1>,
    selected_seen: &[OwnedExprSiteV1],
    unavailable: &mut Option<HomePrefixUnavailableV1>,
    map_compatible: &mut impl FnMut(&OwnedExprSiteV1, BindingRefV1) -> Result<bool, E>,
    borrowed_actuals: &mut impl FnMut(
        &OwnedExprSiteV1,
        BorrowedCallActualRequestV1<'_>,
    ) -> Result<Option<BorrowedCallArgumentsV1>, E>,
    local_calls: &mut Vec<LocalCallObservationV1>,
    scalar_return: bool,
    mut relation: Option<TerminalRelationV1>,
    committed_expression: Option<
        std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionEvaluationV1>,
    >,
) -> Result<(), E> {
    // Contained map literals inside the return value consume Homes
    // during expression evaluation — observe them before the
    // terminal transfer reads the remaining state.
    map_descendant_flow::observe_descendant_maps(
        input,
        statement.site(),
        locals,
        homes,
        maps,
        map_compatible,
    )?;
    if let Some(evaluation) = &committed_expression {
        if let Err(issue) = locals.commit_expression_end_v1(evaluation, homes) {
            unavailable.get_or_insert(issue);
        }
    } else {
        locals.commit_statement_takes_v1(
            statement.site(),
            local_calls,
            homes,
            unavailable,
            borrowed_actuals,
        )?;
    }
    let homes_row = match &*unavailable {
        Some(issue) => Err(issue.clone()),
        None if !scalar_return => Err(HomePrefixUnavailableV1::ReturnValueNotCovered(
            statement.site().clone(),
        )),
        None if selected_seen.iter().any(|site| !results.contains_key(site)) => {
            Err(HomePrefixUnavailableV1::SourceMismatch)
        }
        // Exit-boundary obligation: every surviving caller-owned Home is
        // released here, newest first, together with the local-call sites
        // evaluation on this exit's path could have covered.
        None => Ok(RootHomeExitV1::from_live_v1(
            homes,
            path_calls.iter().cloned().collect(),
            covered_statements.clone().into_boxed_slice(),
        )),
    };
    if let Err(issue) = &homes_row {
        note_terminal_unavailable(
            input.owner(),
            statement,
            "terminal homes (the relation is dropped)",
            issue,
        );
    }
    if homes_row.is_err() {
        // `return new <class>` is exact-site evidence: its own
        // `ResultNewHomePrefixV1` — and thus the retained result
        // claim — already records whatever flow gap made the
        // terminal homes unavailable. Receiver-entry demands and
        // other prefix gaps must not erase the construction proof
        // a caller's handle-result edge reads from the child
        // contract; every other returned source depends on this
        // frame's local flow, so those still drop.
        relation = relation.filter(|relation| {
            matches!(
                relation,
                TerminalRelationV1::Value(row)
                    if matches!(
                        row.returned(),
                        TerminalReturnedSourceV1::Construction(_)
                    )
            )
        });
    }
    // Site-bound consumption: this Return's Home obligation and relation
    // live under its own site, never a function-level slot.
    exit_homes.insert(statement.site().clone(), homes_row);
    if let Some(relation) = relation {
        if crate::config::env::builder_debug_enabled() {
            eprintln!(
                "[terminal-relation] stored owner={:?} statement={:?}",
                input.owner(),
                statement.site()
            );
        }
        terminal_relations.insert(statement.site().clone(), relation);
    }
    Ok(())
}

/// Default-off (builder debug flag): which terminal classification made a return unavailable.
#[inline(never)]
pub(super) fn note_terminal_unavailable(
    owner: FunctionOwnerIdV1,
    statement: &crate::mir::compiler::located::LocatedStmtV1<'_>,
    which: &str,
    issue: &dyn std::fmt::Debug,
) {
    if crate::config::env::builder_debug_enabled() {
        let issue: String = format!("{issue:?}").chars().take(300).collect();
        eprintln!(
            "[terminal-relation] owner={owner:?} statement={:?}: {which} unavailable: {issue}",
            statement.site()
        );
    }
}
