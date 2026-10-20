//! Existing Local statement walk, shared by root, branch and Loop scans.
//! The running scanner owns ordering; this owner retains the same initializer,
//! Call, field, map and Home installation algorithms without a second evaluator.
use crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1;
use super::*;

#[allow(clippy::too_many_arguments)]
pub(super) fn observe<'a, E>(
    input: ResolvedFunctionLoweringInputV1<'a>,
    statement: &crate::mir::compiler::located::LocatedStmtV1<'a>,
    selected: &BTreeMap<OwnedExprSiteV1, BindingRefV1>,
    results: &mut BTreeMap<OwnedExprSiteV1, Result<CallerNewHomePrefixV1, HomePrefixUnavailableV1>>,
    maps: &mut Vec<MapHomeObservation>,
    local_calls: &mut Vec<LocalCallObservationV1>,
    path_calls: &mut BTreeSet<OwnedExprSiteV1>,
    argument_observations: &mut BTreeMap<OwnedExprSiteV1, SelectedNewArgumentObservationV1>,
    locals: &mut PrefixLocalFlow<'a>,
    homes: &mut Vec<LiveHomeV1>,
    covered_statements: &mut Vec<SourceStmtSiteV1>,
    selected_seen: &mut Vec<OwnedExprSiteV1>,
    unavailable: &mut Option<HomePrefixUnavailableV1>,
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
) -> Result<(), E> {
    let function = input.function();
    let ASTNode::Local { variables, initial_values, .. } = statement.node() else {
        unavailable.get_or_insert(HomePrefixUnavailableV1::SourceMismatch);
        return Ok(());
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
        local_call_flow::stage_argument_expressions(input, &statement, &owned, locals,
            unavailable.is_none(), homes, local_calls, local_field_read, borrowed_actuals)?;
        if mixed_call::observe(input, statement.site(), &owned, &declaration, binding,
            homes, locals, local_calls, path_calls, unavailable, borrowed_actuals)? {
            continue;
        }
        if let Some(selected) = local_call_flow::issue_borrowed_nullable_local_call(
            input,
            statement.site(),
            &owned,
            declaration.clone(),
            binding,
            &homes,
            borrowed_actuals,
        )? {
            match selected {
                Ok(local_call) => {
                    let loan = local_call
                        .borrowed_nullable()
                        .expect("source issuer retains its loan");
                    locals.install_borrowed_nullable(loan.clone());
                    path_calls.insert(local_call.site().clone());
                    local_calls.push(local_call);
                }
                Err(issue) => {
                    unavailable.get_or_insert(issue);
                }
            }
            continue;
        }
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
        // Qualified `Alias.m(..)` static calls live in the method-call
        // inventory — the package membership index admits only sealed
        // `ExactI64` targets and the flow seals each argument's class.
        if let Some(local_call) = local_call_flow::issue_static_i64_local_call(
            input,
            statement.site(),
            &owned,
            declaration.clone(),
            binding,
            &homes,
            locals,
            local_static_call,
            borrowed_actuals,
        )? {
            path_calls.insert(local_call.site().clone());
            local_calls.push(local_call);
            locals.install_i64_call_result(binding);
            continue;
        }
        if let Some(selected) = local_call_flow::issue_received_producer_local_call(
            input,
            statement.site(),
            &owned,
            declaration.clone(),
            binding,
            &homes,
            borrowed_actuals,
        )? {
            match selected {
                Ok(local_call) => {
                    match local_call.result() {
                        local_call_flow::LocalCallResultClassV1::Handle => {
                            locals.install_received_handle(binding, &owned)
                        }
                        local_call_flow::LocalCallResultClassV1::Nullable => {
                            locals.install_received_nullable(binding, &owned)
                        }
                        _ => unreachable!("producer result corroborated before observation"),
                    }
                    path_calls.insert(local_call.site().clone());
                    local_calls.push(local_call);
                    homes.push(LiveHomeV1::Stable(binding));
                }
                Err(issue) => {
                    unavailable.get_or_insert(issue);
                }
            }
            continue;
        }
        // Lexical-receiver `recv.m(..)` calls whose selected callee
        // carries the sealed `NullableObject` claim join the
        // checked-release lane before the i64 lane — the same sealed
        // argument evidence, but the received binding is an owned
        // nullable Home the caller releases only when live.
        if let Some(local_call) = local_call_flow::issue_lexical_nullable_local_call(
            input,
            statement.site(),
            &owned,
            declaration.clone(),
            binding,
            &homes,
            locals,
            local_lexical_nullable_call,
            local_lexical_i64_call,
            borrowed_actuals,
        )? {
            path_calls.insert(local_call.site().clone());
            local_calls.push(local_call);
            homes.push(LiveHomeV1::Stable(binding));
            locals.install_received_nullable(binding, &owned);
            continue;
        }
        // Lexical-receiver `recv.m(..)` calls whose selected callee
        // returns only literals join the same I64 lane — the package
        // predicate names the membership, the flow seals the arguments.
        if let Some(local_call) = local_call_flow::issue_lexical_i64_local_call(
            input,
            statement.site(),
            &owned,
            declaration.clone(),
            binding,
            &homes,
            locals,
            local_lexical_i64_call,
            borrowed_actuals,
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
            homes.push(LiveHomeV1::Stable(binding));
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
                borrowed_actuals,
            )?,
        };
        if let Some(local_call) = local_call {
            // A received object handle installs as an owned caller
            // Home: the callee transferred ownership at the Return
            // edge, so this owner owes exactly one release at exit.
            // Its acquisition is the call site, never a `new` site.
            path_calls.insert(local_call.site().clone());
            local_calls.push(local_call);
            homes.push(LiveHomeV1::Stable(binding));
            locals.install_received_handle(binding, &owned);
            continue;
        }
        if let Some(local_call) = local_call_flow::issue_receiver_local_call(
            input,
            statement.site(),
            &owned,
            declaration.clone(),
            binding,
            &homes,
            local_call_flow::LocalCallResultClassV1::Nullable,
            local_nullable_call,
        )? {
            // A nullable `me.m(..)` result joins the owned-Home ledger:
            // the caller owes a checked release at every exit and the
            // acquisition is this call site, never a `new` site.
            path_calls.insert(local_call.site().clone());
            local_calls.push(local_call);
            homes.push(LiveHomeV1::Stable(binding));
            locals.install_received_nullable(binding, &owned);
            continue;
        }
        if let Some(destination) = selected.get(&owned) {
            super::new_local::observe_selected_new_local(
                input,
                site,
                &owned,
                binding,
                *destination,
                results,
                argument_observations,
                locals,
                homes,
                covered_statements,
                selected_seen,
                unavailable,
                scalar_field,
                argument_i64_field,
            )?;
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
                        homes.push(LiveHomeV1::Stable(binding));
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
        } else if super::named_array::observe_initializer(
            input,
            &owned,
            binding,
            locals,
            homes,
            covered_statements,
            unavailable,
            borrowed_actuals,
        )? {
            // Source obligation retained; no producer execution is inferred.
        } else if super::initializer_expression::observe(input, &statement, relation, locals, homes,
            unavailable.is_none(), local_calls, path_calls, scalar_field, local_field_read, borrowed_actuals)? {
            // The original Normal Text residence retains its acquisition.
        } else if let Some(class) = locals.observe(site) {
            locals.install_observed(binding, class);
        } else if let Some(result_kind) = field_call::observe_local_field_call(
            input,
            site,
            locals,
            homes,
            container_field,
            array_i64_field,
            formal_i64_field,
            scalar_field,
            view_use,
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
        } else if let Some(result) =
            field_read::observe_local_field_read(input, site, locals, homes, local_field_read)?
        {
            field_read::install_local_field_read_result(
                input, site, binding, locals, unavailable, result,
            );
            continue;
        } else if let Some((kind, calls)) = {
            let mut operation = None;
            let observed = scalar_expression::observe_local_initializer(
                &mut operation,
                input,
                site,
                locals,
                local_field_read,
                statement.site(),
                homes, unavailable.is_none(),
                local_static_call,
                view_use,
                borrowed_actuals,
            )?;
            if let Some(operation) = operation {
                if operation.source().root().initializer().map(|row| row.binding()) != Some(binding)
                    || !locals.retain_expression_plan(operation)
                {
                    unavailable.get_or_insert(HomePrefixUnavailableV1::SourceMismatch);
                    continue;
                }
            }
            observed
        } {
            path_calls.extend(calls.iter().map(|call| call.site().clone()));
            local_calls.extend(calls);
            locals.install_scalar_call_result(binding, kind);
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
    Ok(())
}
