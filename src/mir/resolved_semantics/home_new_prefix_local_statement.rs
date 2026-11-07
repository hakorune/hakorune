//! Existing Local statement walk, shared by root, branch and Loop scans.
//! The running scanner owns ordering; this owner retains the same initializer,
//! Call, field, map and Home installation algorithms without a second evaluator.
use super::*;
use super::local_call_route as route_flow;
use crate::mir::resolved_semantics::home_new_prefix::{LiveHomeV1, SharedPayloadSourceV1};

#[allow(clippy::too_many_arguments)]
pub(super) fn observe<'a, E>(
    input: ResolvedFunctionLoweringInputV1<'a>,
    statement: &crate::mir::compiler::located::LocatedStmtV1<'a>,
    selected: &BTreeMap<OwnedExprSiteV1, BindingRefV1>,
    result_sites: &BTreeSet<OwnedExprSiteV1>,
    result_prefixes: &mut BTreeMap<OwnedExprSiteV1, Result<ResultNewHomePrefixV1, HomePrefixUnavailableV1>>,
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
    local_call_route: &mut impl FnMut(&OwnedExprSiteV1) -> Result<Option<LocalCallRouteV1>, E>,
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
    if crate::config::env::builder_debug_enabled() {
        if let Some(issue) = unavailable.as_ref() {
            eprintln!(
                "[home-prefix/local] owner={:?} statement={:?} prior_issue={issue:?}",
                input.owner(),
                statement.site()
            );
        }
    }
    let ASTNode::Local {
        variables,
        initial_values,
        ..
    } = statement.node()
    else {
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
        if let Some(ResolvedLexicalRefV1::Local(source)) = function.variable_ref(site) {
            if locals.shared_acquisition_v1(source).is_some() {
                unavailable.get_or_insert_with(|| HomePrefixUnavailableV1::SharedOperandUnsupported {
                    site: site.clone(), guidance: "Shared handle read borrowing is not connected; use share s for an owned clone or move s to transfer ownership",
                });
                continue;
            }
        }
        if let Some(unary) = function.expression_source().unary(site) {
            let proves_original_move = |read_site: &SourceExprSiteV1, source: BindingRefV1, acquisition: &OwnedExprSiteV1| {
                let Some(flow) = function.expression_source().value_flow() else { return false };
                let Some(value) = flow.read(read_site, source) else { return false };
                let terminal = flow.alias_terminal_with_read_site(value, |origin| {
                    let read = function.expression_source().stored_value_operand(origin);
                    function.variable_ref(read).map(|reference| (read.clone(), reference))
                }, |origin| origin == acquisition.site());
                terminal.and_then(|value| flow.origin(value)).is_some_and(|origin|
                    matches!(origin, super::super::source_value_flow::SourceValueOriginV1::Expression(site) if site == acquisition.site()))
            };
            if unary.operator() == super::super::ResolvedUnaryOperatorV1::Shared {
                if let Some(ResolvedLexicalRefV1::Local(source)) = function.variable_ref(unary.operand()) {
                    if locals.shared_acquisition_v1(source).is_some() {
                        if !locals.retain_shared_clone_initializer_v1(relation, source, unary.clone(), homes) {
                            unavailable.get_or_insert_with(|| HomePrefixUnavailableV1::ArgumentNotCovered(site.clone()));
                        }
                        continue;
                    }
                }
                let payload = OwnedExprSiteV1::new(input.owner(), unary.operand().clone());
                if result_sites.contains(&payload) {
                    let Some(construction) = function.expression_source().construction(unary.operand()) else {
                        unavailable.get_or_insert(HomePrefixUnavailableV1::SourceMismatch); continue;
                    };
                    let Some(construction_source) = scalar_field.shared_payload_construction_source(&payload)?
                        .filter(|source| source.shared_trivial_payload_v1()) else {
                        unavailable.get_or_insert(HomePrefixUnavailableV1::SourceMismatch); continue;
                    };
                    let (observation, moved, issue) = super::new_local::observe_new_arguments_v1(
                        input, &payload, construction.arguments().len(), locals, homes, local_calls, argument_i64_field, borrowed_actuals)?;
                    argument_observations.insert(payload.clone(), observation);
                    if unavailable.is_some() || issue.is_some() || !moved.is_empty() || !construction.field_initializers().is_empty() {
                        unavailable.get_or_insert_with(|| issue.unwrap_or(HomePrefixUnavailableV1::SourceMismatch)); continue;
                    }
                    let prior_homes = homes.to_vec();
                    let shared_site = owned.clone();
                    let prefix = issue_shared_new_fault_continuation_v1(input, &payload, &shared_site, binding)
                        .map_err(|_| HomePrefixUnavailableV1::SourceMismatch)
                        .map(|outward_fault| ResultNewHomePrefixV1 {
                            purpose: ResultNewPurposeV1::SharedPayload { shared_site: shared_site.clone(), binding },
                            prior_homes: homes.iter().rev().cloned().collect(),
                            normal_homes: homes.iter().rev().cloned().collect(), outward_fault,
                            covered_statements: covered_statements.clone().into_boxed_slice(),
                        });
                    if prefix.is_err() { unavailable.get_or_insert(HomePrefixUnavailableV1::SourceMismatch); continue; }
                    let plan = input.forest().callable_source_ledger(input.owner()).ok()
                        .and_then(|ledger| crate::mir::resolved_value_profile::expression_plan::local_expression_source_plan_v1(&ledger, input.body_shape(), relation).ok())
                        .and_then(|source| super::expression_plan::HomeExpressionPlanV1::shared_initializer(source, SharedPayloadSourceV1::Fresh { site: payload.clone() }, unary.clone(), None, prior_homes.into()));
                    if !plan.is_some_and(|plan| locals.retain_expression_plan(plan))
                        || !locals.install_shared_fresh_v1(binding, shared_site, payload.clone(), construction_source, homes) {
                        unavailable.get_or_insert(HomePrefixUnavailableV1::SourceMismatch); continue;
                    }
                    result_prefixes.insert(payload, prefix);
                    continue;
                }
                let Some(moved) = function.expression_source().moved(unary.operand()) else {
                    unavailable.get_or_insert_with(|| HomePrefixUnavailableV1::SharedOperandUnsupported {
                        site: site.clone(), guidance: "share requires an owned Shared local or a fresh/moved plain Box; for an owned plain Box use share move x; borrowed Box, values, Dynamic and nullable operands are unsupported",
                    });
                    continue;
                };
                let Some(ResolvedLexicalRefV1::Local(source)) = function.variable_ref(moved.operand()) else {
                    unavailable.get_or_insert_with(|| HomePrefixUnavailableV1::MoveUnsupported(moved.site().clone()));
                    continue;
                };
                if locals.was_moved_out(source) {
                    unavailable.get_or_insert_with(|| HomePrefixUnavailableV1::UseAfterMove(moved.operand().clone()));
                    continue;
                }
                if locals.shared_acquisition_v1(source).is_some() {
                    unavailable.get_or_insert_with(|| HomePrefixUnavailableV1::SharedOperandUnsupported {
                        site: site.clone(), guidance: "share move h cannot start sharing an existing Shared handle; use move h to transfer it, or share h to add one handle",
                    });
                    continue;
                }
                let scope = function.binding(binding).map(|row| row.owner_scope());
                if scope.is_none() || function.binding(source).map(|row| row.owner_scope()) != scope
                    || function.exact_scope_containing(statement.site().node()) != scope {
                    unavailable.get_or_insert_with(|| HomePrefixUnavailableV1::MoveUnsupported(moved.site().clone()));
                    continue;
                }
                let acquisition = locals.home_acquisition(source).cloned();
                let proven = acquisition.as_ref().is_some_and(|row| proves_original_move(moved.operand(), source, row));
                let Some(acquisition) = acquisition.filter(|_| proven) else {
                    unavailable.get_or_insert_with(|| HomePrefixUnavailableV1::MoveUnsupported(moved.site().clone()));
                    continue;
                };
                if locals.shared_move_preflight_v1(source, binding, &owned, homes).is_none() {
                    unavailable.get_or_insert_with(|| HomePrefixUnavailableV1::ArgumentNotCovered(site.clone()));
                    continue;
                }
                let prior_homes = homes.to_vec().into();
                let plan = input.forest().callable_source_ledger(input.owner()).ok()
                    .and_then(|ledger| crate::mir::resolved_value_profile::expression_plan::local_expression_source_plan_v1(&ledger, input.body_shape(), relation).ok())
                    .and_then(|source_plan| super::expression_plan::HomeExpressionPlanV1::shared_initializer(source_plan, SharedPayloadSourceV1::Existing { from: source, acquisition, is_clone: false }, unary.clone(), Some(moved.clone()), prior_homes));
                if !plan.is_some_and(|plan| locals.retain_expression_plan(plan))
                    || !locals.install_shared_move_v1(source, binding, owned, homes) {
                    unavailable.get_or_insert_with(|| HomePrefixUnavailableV1::ArgumentNotCovered(site.clone()));
                }
                continue;
            }
            if unary.is_move() {
                let source = match function.variable_ref(unary.operand()) {
                    Some(ResolvedLexicalRefV1::Local(source)) => source,
                    _ => {
                        unavailable.get_or_insert_with(|| HomePrefixUnavailableV1::MoveUnsupported(site.clone()));
                        continue;
                    }
                };
                if locals.was_moved_out(source) {
                    unavailable.get_or_insert_with(|| HomePrefixUnavailableV1::UseAfterMove(unary.operand().clone()));
                    continue;
                }
                let scope = function.binding(binding).map(|row| row.owner_scope());
                if scope.is_none()
                    || function.binding(source).map(|row| row.owner_scope()) != scope
                    || function.exact_scope_containing(statement.site().node()) != scope
                {
                    unavailable.get_or_insert_with(|| HomePrefixUnavailableV1::MoveUnsupported(site.clone()));
                    continue;
                }
                let acquisition = locals.movable_acquisition_v1(source).cloned();
                let proven = acquisition.as_ref().is_some_and(|row| if locals.shared_acquisition_v1(source).is_some() {
                    function.expression_source().value_flow().is_some_and(|flow| flow.read(unary.operand(), source).is_some())
                } else { proves_original_move(unary.operand(), source, row) });
                if !proven {
                    unavailable.get_or_insert_with(|| HomePrefixUnavailableV1::MoveUnsupported(site.clone()));
                    continue;
                }
                let Some(acquisition) = acquisition else {
                    unavailable.get_or_insert_with(|| HomePrefixUnavailableV1::MoveUnsupported(site.clone()));
                    continue;
                };
                let plan = input.forest().callable_source_ledger(input.owner()).ok()
                    .and_then(|ledger| crate::mir::resolved_value_profile::expression_plan::local_expression_source_plan_v1(&ledger, input.body_shape(), relation).ok())
                    .and_then(|source_plan| super::expression_plan::HomeExpressionPlanV1::move_initializer(source_plan, unary, source, acquisition));
                if !plan.is_some_and(|plan| locals.retain_expression_plan(plan))
                    || !locals.transfer_home_v1(source, binding, homes)
                {
                    unavailable.get_or_insert_with(|| HomePrefixUnavailableV1::MoveUnsupported(site.clone()));
                }
                continue;
            }
        }
        if let Some(result) = map_flow::observe_field_get_local_v1(
            input,
            &owned,
            &declaration,
            binding,
            locals,
            homes,
            unavailable.is_none(),
            local_calls,
            maps,
            borrowed_actuals,
        )? {
            if let Err(issue) = result {
                unavailable.get_or_insert(issue);
            }
            continue;
        }
        // A Call below an operation of the initializer is not a direct argument
        // of the root Call; stage every Call in source order, as the other
        // whole-expression consumers do.
        let whole = input.forest().callable_source_ledger(input.owner()).ok()
            .and_then(|ledger| crate::mir::resolved_value_profile::expression_plan::local_expression_source_plan_v1(&ledger, input.body_shape(), relation).ok())
            .filter(|plan| plan.has_call_below_operation_v1());
        let staged_whole = match &whole {
            Some(plan) => local_call_flow::stage_expression_actuals_v1(
                input, &statement, plan, locals, unavailable.is_none(), homes,
                local_calls, path_calls, local_field_read, borrowed_actuals,
            )?,
            None => false,
        };
        if !staged_whole {
            local_call_flow::stage_argument_expressions(
                input,
                &statement,
                &owned,
                locals,
                unavailable.is_none(),
                homes,
                local_calls,
                path_calls,
                local_field_read,
                borrowed_actuals,
            )?;
        }
        if mixed_call::observe(
            input,
            statement.site(),
            &owned,
            &declaration,
            binding,
            homes,
            locals,
            local_calls,
            path_calls,
            unavailable,
            borrowed_actuals,
        )? {
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
                    if !locals.install_borrowed_nullable(loan.clone()) {
                        unavailable.get_or_insert_with(|| HomePrefixUnavailableV1::PrefixNotCovered(statement.site().clone()));
                        continue;
                    }
                    path_calls.insert(local_call.site().clone());
                    local_calls.push(local_call);
                }
                Err(issue) => {
                    unavailable.get_or_insert(issue);
                }
            }
            continue;
        }
        let receiver_alias = input
            .function()
            .method_call(owned.site())
            .and_then(|call| match call.receiver() {
                crate::mir::resolved_semantics::ResolvedMethodCallReceiverSourceV1::Lexical(
                    crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(receiver),
                ) => locals
                    .field_alias_receiver_class_v1(receiver)
                    .and_then(|class| {
                        locals
                            .received_field_alias_call_path_v1(receiver)
                            .map(|path| (receiver, class, path))
                    }),
                _ => None,
            });
        if let Some(selected) = local_call_flow::issue_received_producer_local_call_with_alias_v1(
            input,
            statement.site(),
            &owned,
            declaration.clone(),
            binding,
            &homes,
            receiver_alias,
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
        // One question per site: the package names the call kind from name
        // resolution and the result class from the callee's own class source.
        // The issuer seals structure and arguments; the install table is keyed
        // by the class. No route is a named rejection below, never a retry.
        let mut tried_lexical_i64 = false;
        if let Some(route) = local_call_route(&owned)? {
            tried_lexical_i64 = route.is_lexical_i64();
            if let Some(local_call) = route_flow::issue_routed_local_call(
                input,
                statement.site(),
                &owned,
                declaration.clone(),
                binding,
                &homes,
                locals,
                route,
                &mut route_flow::lexical_i64_of(&mut *local_call_route),
                borrowed_actuals,
            )? {
                route_flow::install_routed_local_call(
                    local_call, binding, &owned, locals, homes, path_calls, local_calls,
                );
                continue;
            }
        }
        // The package's borrowed-formal selection is a second class source for
        // a lexical call: it lends the call's own source result or arguments.
        // A cohort-routed lexical I64 call has already asked it.
        if !tried_lexical_i64 {
            if let Some(local_call) = local_call_flow::issue_lexical_i64_local_call(
                input,
                statement.site(),
                &owned,
                declaration.clone(),
                binding,
                &homes,
                locals,
                &mut route_flow::lexical_i64_of(&mut *local_call_route),
                borrowed_actuals,
            )? {
                route_flow::install_routed_local_call(
                    local_call, binding, &owned, locals, homes, path_calls, local_calls,
                );
                continue;
            }
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
                local_calls,
                borrowed_actuals,
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
        } else if super::initializer_expression::observe(
            input,
            &statement,
            relation,
            locals,
            homes,
            unavailable.is_none(),
            local_calls,
            path_calls,
            scalar_field,
            local_field_read,
            borrowed_actuals,
        )? {
            // The original Normal Text residence retains its acquisition.
        } else if locals.install_declared_read_alias_v1(binding, site) {
            // Same non-owning result and current validity; no new Home or Call.
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
                    locals.install_scalar_call_result(binding, SourceScalarKind::Integer);
                }
                CoreMethodResultKindV1::BoolValue => {
                    locals.install_scalar_call_result(binding, local_flow::SourceScalarKind::Bool);
                }
                _ => locals.install_bound_value(binding),
            }
            continue;
        } else if let Some(result) =
            field_read::observe_local_field_read(input, site, locals, homes, local_field_read)?
        {
            field_read::install_local_field_read_result(
                input,
                site,
                binding,
                locals,
                unavailable,
                result,
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
                homes,
                unavailable.is_none(),
                &mut super::local_call_route::static_claim_of(&mut *local_call_route),
                view_use,
                borrowed_actuals,
            )?;
            if let Some(operation) = operation {
                if operation
                    .source()
                    .root()
                    .initializer()
                    .map(|row| row.binding())
                    != Some(binding)
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
