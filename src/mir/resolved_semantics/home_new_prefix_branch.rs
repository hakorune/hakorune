//! Bounded `If` branch walk inside `scan_statement_flow`: each branch forks
//! the running Home/local state, a branch that reaches an explicit exit
//! terminates its path, and fall-through branches join by ordered
//! intersection. Divergent surviving Homes become a named
//! `HomeFlowBranchDivergent` — never a union, never an empty substitution.
//! Only a verified `ResolvedIfRegionBundleV1` admits the walk; any other `If`
//! keeps `PrefixNotCovered`.
use super::*;
use crate::mir::resolved_semantics::{
    BodyChildRoleV1, BodyExpressionShapeV1, ResolvedBinaryOperatorV1, ResolvedLexicalRefV1,
    ResolvedLiteralSourceV1,
};

/// One path's Home/local state during a branch walk. A terminated path
/// (one that reached an explicit Return) does not join the continuation.
struct BranchPath<'a> {
    locals: PrefixLocalFlow<'a>,
    homes: Vec<BindingRefV1>,
    covered: Vec<SourceStmtSiteV1>,
    selected_seen: Vec<OwnedExprSiteV1>,
    calls: BTreeSet<OwnedExprSiteV1>,
    unavailable: Option<HomePrefixUnavailableV1>,
    terminated: bool,
}

#[allow(clippy::too_many_arguments)]
fn walk_branch<'a, E>(
    input: ResolvedFunctionLoweringInputV1<'a>,
    body: &crate::mir::compiler::located::LocatedBodyV1<'a>,
    exit_sites: &BTreeSet<SourceStmtSiteV1>,
    selected: &BTreeMap<OwnedExprSiteV1, BindingRefV1>,
    result_sites: &BTreeSet<OwnedExprSiteV1>,
    results: &mut BTreeMap<OwnedExprSiteV1, Result<CallerNewHomePrefixV1, HomePrefixUnavailableV1>>,
    exit_homes: &mut BTreeMap<SourceStmtSiteV1, Result<RootHomeExitV1, HomePrefixUnavailableV1>>,
    maps: &mut Vec<MapHomeObservation>,
    local_calls: &mut Vec<LocalCallObservationV1>,
    terminal_relations: &mut BTreeMap<SourceStmtSiteV1, TerminalRelationV1>,
    argument_observations: &mut BTreeMap<OwnedExprSiteV1, SelectedNewArgumentObservationV1>,
    result_prefixes: &mut BTreeMap<
        OwnedExprSiteV1,
        Result<ResultNewHomePrefixV1, HomePrefixUnavailableV1>,
    >,
    mut path: BranchPath<'a>,
    field_is_integer: &mut impl FnMut(
        &OwnedExprSiteV1,
        &SourceExprSiteV1,
        BindingRefV1,
        BindingRefV1,
        &str,
    ) -> Result<bool, E>,
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
    scalar_field: &mut impl FnMut(
        &OwnedExprSiteV1,
        &SourceExprSiteV1,
        BindingRefV1,
        BindingRefV1,
        &str,
    ) -> Result<bool, E>,
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
        bool,
    ) -> Result<Option<Vec<LocalFieldReadResultV1>>, E>,
    borrowed_actuals: &mut impl FnMut(
        &OwnedExprSiteV1,
        BorrowedCallActualRequestV1<'_>,
    ) -> Result<Option<BorrowedCallArgumentsV1>, E>,
    view_use: &mut impl FnMut(&OwnedExprSiteV1, BorrowedViewUseRequestV1<'_>) -> Result<bool, E>,

    object_return: &mut impl FnMut(
        &OwnedExprSiteV1,
    ) -> Result<Option<ObjectReturnCallQualificationV1>, E>,
) -> Result<BranchPath<'a>, E> {
    path.terminated = super::scan::scan_statement_flow(
        input,
        body,
        exit_sites,
        selected,
        result_sites,
        results,
        exit_homes,
        maps,
        local_calls,
        &mut path.calls,
        terminal_relations,
        argument_observations,
        result_prefixes,
        &mut path.locals,
        &mut path.homes,
        &mut path.covered,
        &mut path.selected_seen,
        &mut path.unavailable,
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
        object_return,
    )?;
    Ok(path)
}

/// Stage borrowed call actuals for every source callsite inside a
/// statement subtree that never joined the covered walk — a
/// prefix-failed or structurally unadmitted `If`/`Loop` still contains
/// real call edges that incoming coverage can name, and a named edge
/// without staged actuals freezes `selected-incoming-unobserved`.
/// This records source facts only: no claims, terminal relations or
/// Home joins. `prefix_known` must be the caller's pre-statement value —
/// the same basis the condition-subtree staging used — so a repeated
/// stage of the same site stays identical instead of drifting.
/// Covered `If`s must not call this for their interiors: the branch
/// walk already stages those sites with real flow state, and a
/// divergent re-staging freezes `repeated-walk-drift`.
pub(super) fn stage_unobserved_statement_actuals<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &crate::mir::compiler::located::LocatedStmtV1<'_>,
    locals: &PrefixLocalFlow<'_>,
    prefix_known: bool,
    borrowed_actuals: &mut impl FnMut(
        &OwnedExprSiteV1,
        BorrowedCallActualRequestV1<'_>,
    ) -> Result<Option<BorrowedCallArgumentsV1>, E>,
) -> Result<(), E> {
    stage_subtree_call_actuals(
        input,
        statement.site().node(),
        locals,
        prefix_known,
        borrowed_actuals,
    )
}

/// Stage every outermost call under a source site prefix — a call
/// nested inside another call's argument subtree rides the enclosing
/// observation; identical re-staging is a no-op.
fn stage_subtree_call_actuals<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &crate::mir::resolved_semantics::SourceNodeSiteV1,
    locals: &PrefixLocalFlow<'_>,
    prefix_known: bool,
    borrowed_actuals: &mut impl FnMut(
        &OwnedExprSiteV1,
        BorrowedCallActualRequestV1<'_>,
    ) -> Result<Option<BorrowedCallArgumentsV1>, E>,
) -> Result<(), E> {
    let prefix = site.segments();
    let subtree_calls: Vec<SourceExprSiteV1> = input
        .function()
        .method_calls()
        .map(|(site, _)| site.clone())
        .filter(|site| site.node().segments().starts_with(prefix))
        .collect();
    for site in subtree_calls.iter().filter(|site| {
        !subtree_calls.iter().any(|outer| {
            outer != *site && site.node().segments().starts_with(outer.node().segments())
        })
    }) {
        let owned = OwnedExprSiteV1::new(input.owner(), site.clone());
        for (call_site, actuals) in
            local_call_flow::observe_borrowed_call_actuals(input, &owned, locals, prefix_known)
        {
            borrowed_actuals(&call_site, BorrowedCallActualRequestV1::Observe(&actuals))?;
        }
    }
    Ok(())
}

/// True when any sealed `MapLiteral` row sits under `statement`'s subtree.
/// Branch-local map literals need per-path Home state that this bounded
/// admission does not carry — the whole `If` keeps `PrefixNotCovered` and
/// the tail pass issues their `Unavailable` rows.
fn subtree_has_map_literal(
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &crate::mir::compiler::located::LocatedStmtV1<'_>,
) -> bool {
    let Some(shape) = input.body_shape() else {
        return false;
    };
    let prefix = statement.site().node().segments();
    shape.expressions().iter().any(|row| {
        matches!(
            row,
            BodyExpressionShapeV1::MapLiteral { site, .. }
                if site.node().segments().starts_with(prefix)
        )
    })
}

#[allow(clippy::too_many_arguments)]
pub(super) fn observe_if_statement<'a, E>(
    input: ResolvedFunctionLoweringInputV1<'a>,
    statement: &crate::mir::compiler::located::LocatedStmtV1<'a>,
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
    homes: &mut Vec<BindingRefV1>,
    covered_statements: &mut Vec<SourceStmtSiteV1>,
    selected_seen: &mut Vec<OwnedExprSiteV1>,
    unavailable: &mut Option<HomePrefixUnavailableV1>,
    field_is_integer: &mut impl FnMut(
        &OwnedExprSiteV1,
        &SourceExprSiteV1,
        BindingRefV1,
        BindingRefV1,
        &str,
    ) -> Result<bool, E>,
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
    scalar_field: &mut impl FnMut(
        &OwnedExprSiteV1,
        &SourceExprSiteV1,
        BindingRefV1,
        BindingRefV1,
        &str,
    ) -> Result<bool, E>,
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
        bool,
    ) -> Result<Option<Vec<LocalFieldReadResultV1>>, E>,
    borrowed_actuals: &mut impl FnMut(
        &OwnedExprSiteV1,
        BorrowedCallActualRequestV1<'_>,
    ) -> Result<Option<BorrowedCallArgumentsV1>, E>,
    view_use: &mut impl FnMut(&OwnedExprSiteV1, BorrowedViewUseRequestV1<'_>) -> Result<bool, E>,

    object_return: &mut impl FnMut(
        &OwnedExprSiteV1,
    ) -> Result<Option<ObjectReturnCallQualificationV1>, E>,
) -> Result<bool, E> {
    // The pre-`If` path basis: every staging this statement performs —
    // condition calls now, uncovered interiors on the early returns —
    // shares it, so a repeated stage of the same site stays identical.
    let prefix_known = unavailable.is_none();
    // A call inside the condition subtree is a real call edge even when
    // the `If` itself stays uncovered — incoming coverage can still name
    // it, so its borrowed actuals stage from the sealed call inventory
    // here. Statement `call_root` observation never visits this
    // position, and the gates below decide walk admission, not whether
    // the call exists.
    if let Ok(condition) = input
        .source()
        .child_expr_from_stmt(statement, ExprChildRoleV1::IfCondition)
    {
        stage_subtree_call_actuals(
            input,
            condition.site().node(),
            locals,
            prefix_known,
            borrowed_actuals,
        )?;
    }
    let bundle = match input.function().if_region_bundle(statement.site()) {
        Ok(bundle) => bundle,
        Err(_) => {
            // An `If` with no verified region row cannot carry a join —
            // keep the named unavailability rather than inferring one,
            // and still stage the interior call edges.
            unavailable.get_or_insert_with(|| {
                HomePrefixUnavailableV1::PrefixNotCovered(statement.site().clone())
            });
            stage_unobserved_statement_actuals(
                input,
                statement,
                locals,
                prefix_known,
                borrowed_actuals,
            )?;
            return Ok(false);
        }
    };
    let then_body = input
        .source()
        .child_body_from_stmt(statement, BodyChildRoleV1::IfThen);
    let else_body = input
        .source()
        .child_body_from_stmt(statement, BodyChildRoleV1::IfElse);
    let else_expected = matches!(
        statement.node(),
        ASTNode::If {
            else_body: Some(_),
            ..
        }
    );
    if then_body.is_err()
        || else_body.is_ok() != else_expected
        || bundle.else_pair().is_some() != else_expected
    {
        unavailable.get_or_insert(HomePrefixUnavailableV1::SourceMismatch);
        return Ok(false);
    }
    if subtree_has_map_literal(input, statement) {
        unavailable.get_or_insert_with(|| {
            HomePrefixUnavailableV1::PrefixNotCovered(statement.site().clone())
        });
        // The per-path Home state this admission cannot carry stays out,
        // but the interior call edges are still real — stage them.
        stage_unobserved_statement_actuals(
            input,
            statement,
            locals,
            prefix_known,
            borrowed_actuals,
        )?;
        return Ok(false);
    }
    let Ok(condition) = input
        .source()
        .child_expr_from_stmt(statement, ExprChildRoleV1::IfCondition)
    else {
        unavailable.get_or_insert(HomePrefixUnavailableV1::SourceMismatch);
        return Ok(false);
    };
    if scalar_expression::contains_source_request(
        input,
        condition.site(),
        locals,
        local_static_call,
    )? {
        if let Some((_, calls)) = scalar_expression::observe_scalar_expression(
            input,
            condition.site(),
            locals,
            Some(SourceScalarKind::Bool),
            local_field_read,
            statement.site(),
            homes,
            local_static_call,
            &mut |operand, binary| {
                view_use(
                    operand,
                    BorrowedViewUseRequestV1::CheckedCompareOperand { binary },
                )
            },
            borrowed_actuals,
        )? {
            path_calls.extend(calls.iter().map(|call| call.site().clone()));
            local_calls.extend(calls);
        } else {
            unavailable.get_or_insert_with(|| {
                HomePrefixUnavailableV1::PrefixNotCovered(statement.site().clone())
            });
            stage_unobserved_statement_actuals(
                input,
                statement,
                locals,
                prefix_known,
                borrowed_actuals,
            )?;
            return Ok(false);
        }
    }
    let then_body = then_body.unwrap();

    let base_covered = covered_statements.len();
    let base_selected = selected_seen.len();
    let fork = |locals: &PrefixLocalFlow<'a>,
                homes: &Vec<BindingRefV1>,
                covered: &Vec<SourceStmtSiteV1>,
                seen: &Vec<OwnedExprSiteV1>,
                calls: &BTreeSet<OwnedExprSiteV1>,
                issue: &Option<HomePrefixUnavailableV1>|
     -> BranchPath<'a> {
        BranchPath {
            locals: locals.clone(),
            homes: homes.clone(),
            covered: covered.clone(),
            selected_seen: seen.clone(),
            calls: calls.clone(),
            unavailable: issue.clone(),
            terminated: false,
        }
    };
    let then_path = walk_branch(
        input,
        &then_body,
        exit_sites,
        selected,
        result_sites,
        results,
        exit_homes,
        maps,
        local_calls,
        terminal_relations,
        argument_observations,
        result_prefixes,
        fork(
            locals,
            homes,
            covered_statements,
            selected_seen,
            path_calls,
            unavailable,
        ),
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
        object_return,
    )?;
    let else_path = match else_body {
        Ok(else_body) => walk_branch(
            input,
            &else_body,
            exit_sites,
            selected,
            result_sites,
            results,
            exit_homes,
            maps,
            local_calls,
            terminal_relations,
            argument_observations,
            result_prefixes,
            fork(
                locals,
                homes,
                covered_statements,
                selected_seen,
                path_calls,
                unavailable,
            ),
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
            object_return,
        )?,
        // A missing `else` joins the entry snapshot unchanged.
        Err(_) => BranchPath {
            locals: locals.clone(),
            homes: homes.clone(),
            covered: covered_statements.clone(),
            selected_seen: selected_seen.clone(),
            calls: path_calls.clone(),
            unavailable: unavailable.clone(),
            terminated: false,
        },
    };

    if then_path.terminated && else_path.terminated {
        // Every path exits inside this `If` — the enclosing path ends here.
        return Ok(true);
    }

    let (join_locals, join_homes, join_covered, join_selected, join_calls, branch_issue) =
        if then_path.terminated {
            (
                else_path.locals,
                else_path.homes,
                else_path.covered,
                else_path.selected_seen,
                else_path.calls,
                else_path.unavailable,
            )
        } else if else_path.terminated {
            (
                then_path.locals,
                then_path.homes,
                then_path.covered,
                then_path.selected_seen,
                then_path.calls,
                then_path.unavailable,
            )
        } else {
            // Ordered intersection: a Home survives the join only when both
            // fall-through paths still hold it, in the shared order.
            let joined: Vec<BindingRefV1> = then_path
                .homes
                .iter()
                .filter(|home| else_path.homes.contains(home))
                .copied()
                .collect();
            let mut join_locals = then_path.locals.clone();
            let divergent = then_path.homes != joined
                || else_path.homes != joined
                || !join_locals.join_branch(&else_path.locals);
            let mut join_covered = covered_statements.clone();
            for site in then_path.covered[base_covered..]
                .iter()
                .chain(else_path.covered[base_covered..].iter())
            {
                if !join_covered.contains(site) {
                    join_covered.push(site.clone());
                }
            }
            let mut join_selected = selected_seen.clone();
            for site in then_path.selected_seen[base_selected..]
                .iter()
                .chain(else_path.selected_seen[base_selected..].iter())
            {
                if !join_selected.contains(site) {
                    join_selected.push(site.clone());
                }
            }
            // Covered local calls may run on either branch, so they union —
            // the exit row records coverage, not exclusivity.
            let mut join_calls = then_path.calls.clone();
            join_calls.extend(else_path.calls.iter().cloned());
            let issue = if divergent {
                // Named divergence retains both branch states; the joined
                // Home list is never emitted as an exit obligation.
                unavailable.get_or_insert(HomePrefixUnavailableV1::HomeFlowBranchDivergent {
                    site: statement.site().clone(),
                    then_homes: then_path.homes.into_boxed_slice(),
                    else_homes: else_path.homes.into_boxed_slice(),
                });
                unavailable.clone()
            } else {
                then_path.unavailable.or(else_path.unavailable)
            };
            (
                join_locals,
                joined,
                join_covered,
                join_selected,
                join_calls,
                issue,
            )
        };

    // `branch_issue` already carries the entry issue (each branch forked
    // from this path's unavailable) or the divergence named above.
    // A terminated `binding == null` arm proves the surviving fall-through
    // carries a live object; `binding != null` narrows when the terminated
    // arm is the `else`. The joined state alone records the mark — an
    // arm's interior never sees it.
    let mut join_locals = join_locals;
    let guarded = null_guarded_local(input.function(), condition.site());
    if let Some((binding, operator)) = guarded {
        let narrowed = matches!(operator, ResolvedBinaryOperatorV1::Equal) && then_path.terminated
            || matches!(operator, ResolvedBinaryOperatorV1::NotEqual) && else_path.terminated;
        if narrowed {
            join_locals.mark_nonnull(binding);
        }
    }
    *locals = join_locals;
    *homes = join_homes;
    *covered_statements = join_covered;
    *selected_seen = join_selected;
    *path_calls = join_calls;
    *unavailable = branch_issue;
    Ok(false)
}

/// The `binding == null` (or `null == binding`) compare guard at one
/// exact condition site — operator and binding, nothing else. Only the
/// sealed binary/literal/variable source rows answer here; the caller
/// alone decides which terminated arm proves non-null.
fn null_guarded_local(
    function: &crate::mir::resolved_semantics::VerifiedResolvedFunctionV1,
    site: &SourceExprSiteV1,
) -> Option<(BindingRefV1, ResolvedBinaryOperatorV1)> {
    let binary = function.expression_source().binary(site)?;
    if !matches!(
        binary.operator(),
        ResolvedBinaryOperatorV1::Equal | ResolvedBinaryOperatorV1::NotEqual
    ) {
        return None;
    }
    for (value, other) in [(binary.lhs(), binary.rhs()), (binary.rhs(), binary.lhs())] {
        if let Some(ResolvedLexicalRefV1::Local(binding)) = function.variable_ref(value) {
            if matches!(
                function.expression_source().literal(other),
                Some(ResolvedLiteralSourceV1::Null)
            ) {
                return Some((binding, binary.operator()));
            }
        }
    }
    None
}
