//! Per-statement source walk inside `scan_new_home_flow`: the loop owns no
//! separate authority — every observation still lands in the caller's rows.
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
    local_nullable_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<bool, E>,
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
    scalar_field: &mut impl FnMut(
        &OwnedExprSiteV1,
        &SourceExprSiteV1,
        BindingRefV1,
        BindingRefV1,
        &str,
    ) -> Result<bool, E>,
    // The issuer's receiver-side container-field proof for `me.<field>`
    // receivers of builtin container calls — a separate contract.
    container_field: &mut impl FnMut(
        &OwnedExprSiteV1,
        &SourceExprSiteV1,
        BindingRefV1,
        BindingRefV1,
        &str,
    ) -> Result<bool, E>,
) -> Result<bool, E> {
    let function = input.function();
    for index in 0..body.statements().len() {
        let Ok(statement) = input.source().body_stmt(body, index) else {
            *unavailable = Some(HomePrefixUnavailableV1::SourceMismatch);
            return Ok(false);
        };
        covered_statements.push(statement.site().clone());
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
            )?;
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
                local_nullable_call,
                argument_i64_field,
                scalar_field,
                container_field,
            )?;
            if terminated {
                return Ok(true);
            }
            continue;
        }
        let ASTNode::Local {
            variables,
            initial_values,
            ..
        } = statement.node()
        else {
            // A self-rooted `me.<field> = <rhs>` write with a Home-neutral
            // RHS is covered without ledger rows — the raw lane already
            // owns the plain `FieldSet` emission.
            if field_write::observe_receiver_field_write(
                input,
                &statement,
                locals,
                scalar_field,
            )? {
                continue;
            }
            // A `me.<ArrayBox field>.m(..)` statement whose manifest row
            // returns `NoValue` and whose arguments are Home-neutral is
            // covered without ledger rows — the raw lane already owns
            // `Callee::Method{RuntimeData}`/`ArrayElementWrite` emission.
            if field_call::observe_statement_field_call(
                input,
                &statement,
                locals,
                homes,
                container_field,
                scalar_field,
            )? {
                continue;
            }
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
        // Natural syntax permits one initialized local. Do not give synthetic
        // multiple-initializer carriers a premature sequential-install meaning.
        if variables.len() != 1 && initial_values.iter().any(Option::is_some) {
            unavailable.get_or_insert_with(|| {
                HomePrefixUnavailableV1::PrefixNotCovered(statement.site().clone())
            });
        }
        for ordinal in 0..variables.len() {
            let declaration = SourceBindingSiteV1::Local {
                statement: statement.site().clone(),
                ordinal: ordinal as u32,
            };
            let relation = function
                .expression_source()
                .initializers()
                .find(|row| row.declaration_site() == &declaration);
            let Some(relation) = relation else {
                unavailable.get_or_insert(HomePrefixUnavailableV1::SourceMismatch);
                continue;
            };
            let binding = relation.binding();
            if binding.owner() != input.owner()
                || function.declaration_binding(&declaration) != Some(binding)
            {
                unavailable.get_or_insert(HomePrefixUnavailableV1::SourceMismatch);
            }
            let Some(site) = relation.initializer_site() else {
                locals.install_uninitialized(binding);
                continue;
            };
            let owned = OwnedExprSiteV1::new(input.owner(), site.clone());
            if let Some(local_call) = local_call_flow::issue_local_call(
                input,
                statement.site(),
                &owned,
                declaration.clone(),
                binding,
                &homes,
                local_call_flow::LocalCallResultClassV1::I64,
                terminal_call,
            )? {
                path_calls.insert(local_call.site().clone());
                local_calls.push(local_call);
                locals.install_i64_call_result(binding);
                continue;
            }
            if let Some(local_call) = local_call_flow::issue_local_call(
                input,
                statement.site(),
                &owned,
                declaration.clone(),
                binding,
                &homes,
                local_call_flow::LocalCallResultClassV1::Map,
                local_map_call,
            )? {
                // A received map installs as a live map binding and joins
                // the caller's terminal Homes accounting so the owner's
                // exit can release it.
                path_calls.insert(local_call.site().clone());
                local_calls.push(local_call);
                homes.push(binding);
                locals.install_map(binding);
                continue;
            }
            let local_call = match local_call_flow::issue_local_call(
                input,
                statement.site(),
                &owned,
                declaration.clone(),
                binding,
                &homes,
                local_call_flow::LocalCallResultClassV1::Handle,
                &mut *local_handle_call,
            )? {
                Some(local_call) => Some(local_call),
                // Qualified receivers (`recv.make(...)`) sit in the
                // method-call inventory, never in direct-call
                // observations — try the lexical membership source.
                None => local_call_flow::issue_lexical_local_call(
                    input,
                    statement.site(),
                    &owned,
                    declaration.clone(),
                    binding,
                    &homes,
                    local_call_flow::LocalCallResultClassV1::Handle,
                    &mut *local_handle_call,
                )?,
            };
            if let Some(local_call) = local_call {
                // A received object handle installs as an owned caller
                // Home: the callee transferred ownership at the Return
                // edge, so this owner owes exactly one release at exit.
                // Its acquisition is the call site, never a `new` site.
                path_calls.insert(local_call.site().clone());
                local_calls.push(local_call);
                homes.push(binding);
                locals.install_received_handle(binding);
                continue;
            }
            if let Some(local_call) = local_call_flow::issue_receiver_local_call(
                input,
                statement.site(),
                &owned,
                declaration.clone(),
                binding,
                &homes,
                local_nullable_call,
            )? {
                // A nullable `me.m(..)` result joins the owned-Home ledger:
                // the caller owes a checked release at every exit and the
                // acquisition is this call site, never a `new` site.
                path_calls.insert(local_call.site().clone());
                local_calls.push(local_call);
                homes.push(binding);
                locals.install_received_nullable(binding);
                continue;
            }
            if let Some(destination) = selected.get(&owned) {
                if *destination != binding {
                    unavailable.get_or_insert(HomePrefixUnavailableV1::SourceMismatch);
                }
                let located = input.source().expr_at(&owned);
                let mut moved_arguments = Vec::new();
                match located {
                    Ok(new) => {
                        match new.node() {
                            ASTNode::New {
                                arguments,
                                field_initializers,
                                ..
                            } => {
                                let observed_arguments = {
                                    let mut rows = Vec::with_capacity(arguments.len());
                                    let mut row_error = None;
                                    for (ordinal, _) in arguments.iter().enumerate() {
                                        let Ok(ordinal) = u32::try_from(ordinal) else {
                                            row_error = Some(SelectedNewArgumentUnavailableV1::ArgumentOrdinalOverflow { new_site: owned.clone() });
                                            break;
                                        };
                                        let argument = match input.source().child_expr_from_expr(
                                            &new,
                                            ExprChildRoleV1::CallArgument(ordinal),
                                        ) {
                                            Ok(argument) => argument,
                                            Err(_) => {
                                                row_error = Some(SelectedNewArgumentUnavailableV1::SourceMismatch { new_site: owned.clone() });
                                                break;
                                            }
                                        };
                                        match locals.observe_selected_argument(
                                            argument.site(),
                                            argument_i64_field,
                                        )? {
                                            Some(kind) => rows.push(SelectedNewArgumentV1::new(
                                                ordinal,
                                                argument.site().clone(),
                                                kind,
                                            )),
                                            None => {
                                                row_error = Some(SelectedNewArgumentUnavailableV1::ArgumentNotTrivial { new_site: owned.clone(), site: argument.site().clone() });
                                                break;
                                            }
                                        }
                                    }
                                    match row_error {
                                        Some(error) => Err(error),
                                        None => Ok(rows.into_boxed_slice()),
                                    }
                                };
                                argument_observations.insert(
                                    owned.clone(),
                                    SelectedNewArgumentObservationV1::new(
                                        owned.clone(),
                                        observed_arguments,
                                    ),
                                );
                                if !field_initializers.is_empty() {
                                    unavailable.get_or_insert_with(|| {
                                        HomePrefixUnavailableV1::OverridesNotCovered(site.clone())
                                    });
                                }
                                for argument in 0..arguments.len() {
                                    let arg = input.source().child_expr_from_expr(
                                        &new,
                                        ExprChildRoleV1::CallArgument(argument as u32),
                                    );
                                    match arg {
                                        // Handle arguments require the selected parameter's
                                        // source demand, not merely a physical borrow ABI.
                                        Ok(arg) => {
                                            match locals.observe_selected_argument(
                                                arg.site(),
                                                argument_i64_field,
                                            )? {
                                                Some(kind) => {
                                                    match kind {
                                                        // An argument naming a live owned
                                                        // binding moves the lease into the
                                                        // constructed object; the frame's
                                                        // exit obligation ends at this
                                                        // edge.
                                                        SelectedNewArgumentKindV1::Local { binding }
                                                        | SelectedNewArgumentKindV1::Handle { binding }
                                                        | SelectedNewArgumentKindV1::BoundValue { binding }
                                                            if homes.contains(&binding) =>
                                                        {
                                                            moved_arguments.push(binding);
                                                        }
                                                        _ => {}
                                                    }
                                                }
                                                None => {
                                                    unavailable.get_or_insert_with(|| {
                                                        HomePrefixUnavailableV1::ArgumentNotCovered(
                                                            arg.site().clone(),
                                                        )
                                                    });
                                                }
                                            }
                                        }
                                        Err(_) => {
                                            unavailable.get_or_insert(
                                                HomePrefixUnavailableV1::SourceMismatch,
                                            );
                                        }
                                    }
                                }
                            }
                            _ => {
                                unavailable.get_or_insert(HomePrefixUnavailableV1::SourceMismatch);
                            }
                        }
                    }
                    Err(_) => {
                        unavailable.get_or_insert(HomePrefixUnavailableV1::SourceMismatch);
                    }
                }
                let result = match &*unavailable {
                    Some(issue) => Err(issue.clone()),
                    None => issue_new_fault_continuation_v1(input, &owned)
                        .map_err(|_| HomePrefixUnavailableV1::SourceMismatch)
                        .map(|outward_fault| CallerNewHomePrefixV1 {
                            destination: binding,
                            prior_homes: homes.iter().rev().copied().collect(),
                            outward_fault,
                            covered_statements: covered_statements.clone().into_boxed_slice(),
                        }),
                };
                results.insert(owned.clone(), result);
                selected_seen.push(owned.clone());
                // Argument moves complete on the invoke's Normal edge —
                // `prior_homes` above kept each binding so a Fault unwind
                // still discharges it.
                for moved in moved_arguments {
                    homes.retain(|home| *home != moved);
                    locals.consume_home(moved);
                }
                // This is the Normal successor only, after exact local commit.
                homes.push(binding);
                locals.install_selected_normal_home(binding, owned);
            } else if let Some(keys) = map_literal_keys(input, site) {
                if unavailable.is_none() {
                    let mut used = std::collections::BTreeSet::new();
                    let mut nested = Vec::new();
                    match map_flow::observe_map(
                        input,
                        &owned,
                        MapDestinationV1::LocalBinding(binding),
                        keys,
                        locals,
                        &homes,
                        &mut used,
                        &mut nested,
                        map_compatible,
                    )? {
                        Ok((map, remaining)) => {
                            *homes = remaining;
                            homes.push(binding);
                            locals.install_map(binding);
                            maps.push(map_flow::MapHomeObservation::Complete(map));
                        }
                        Err(issue) => {
                            *unavailable = Some(issue);
                            maps.push(map_flow::MapHomeObservation::Unavailable { site: owned });
                        }
                    }
                    maps.extend(nested);
                } else {
                    maps.push(map_flow::MapHomeObservation::Unavailable { site: owned });
                }
            } else if let Some(class) = locals.observe(site) {
                locals.install_observed(binding, class);
            } else if let Some(result_kind) = field_call::observe_local_field_call(
                input,
                site,
                locals,
                homes,
                container_field,
                scalar_field,
            )? {
                // A manifest-proven `me.<ArrayBox field>.m(..)` result
                // binds by its contract class — scalars are trivial,
                // dynamic results stay `BoundValue`.
                use crate::mir::core_method_result_kind::CoreMethodResultKindV1;
                match result_kind {
                    CoreMethodResultKindV1::I64Value => {
                        locals.install_i64_call_result(binding);
                    }
                    CoreMethodResultKindV1::BoolValue => {
                        locals.install_scalar_call_result(
                            binding,
                            local_flow::SourceScalarKind::Bool,
                        );
                    }
                    _ => locals.install_bound_value(binding),
                }
                continue;
            } else {
                locals.install_inventoried_call_result(binding, site);
                unavailable.get_or_insert_with(|| {
                    HomePrefixUnavailableV1::PrefixNotCovered(statement.site().clone())
                });
            }
        }
        // Contained map literals deeper inside initializer subtrees (array
        // elements, call arguments, nested containers) still evaluate at
        // this program point — observe them with the running state.
        map_descendant_flow::observe_descendant_maps(
            input,
            statement.site(),
            locals,
            homes,
            maps,
            map_compatible,
        )?;
    }
    Ok(false)
}
