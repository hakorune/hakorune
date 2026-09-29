//! Per-statement source walk inside `scan_new_home_flow`: the loop owns no
//! separate authority — every observation still lands in the caller's rows.
use super::*;

#[allow(clippy::too_many_arguments)]
pub(super) fn scan_statement_flow<'a, E>(
    input: ResolvedFunctionLoweringInputV1<'a>,
    body: &crate::mir::compiler::located::LocatedBodyV1<'a>,
    terminal: Option<&SourceStmtSiteV1>,
    selected: &BTreeMap<OwnedExprSiteV1, BindingRefV1>,
    result_sites: &BTreeSet<OwnedExprSiteV1>,
    results: &mut BTreeMap<OwnedExprSiteV1, Result<CallerNewHomePrefixV1, HomePrefixUnavailableV1>>,
    terminal_homes: &mut Result<Box<[BindingRefV1]>, HomePrefixUnavailableV1>,
    maps: &mut Vec<MapHomeObservation>,
    local_calls: &mut Vec<LocalCallObservationV1>,
    terminal_relation: &mut Option<TerminalRelationV1>,
    argument_observations: &mut BTreeMap<OwnedExprSiteV1, SelectedNewArgumentObservationV1>,
    result_prefixes: &mut BTreeMap<
        OwnedExprSiteV1,
        Result<ResultNewHomePrefixV1, HomePrefixUnavailableV1>,
    >,
    locals: &mut PrefixLocalFlow<'a>,
    homes: &mut Vec<BindingRefV1>,
    covered_statements: &mut Vec<SourceStmtSiteV1>,
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
) -> Result<(), E> {
    let function = input.function();
    for index in 0..body.statements().len() {
        let Ok(statement) = input.source().body_stmt(body, index) else {
            *unavailable = Some(HomePrefixUnavailableV1::SourceMismatch);
            break;
        };
        covered_statements.push(statement.site().clone());
        if terminal == Some(statement.site()) {
            let scalar_return = match statement.node() {
                ASTNode::Return { value: None, .. } => {
                    *terminal_relation = Some(TerminalRelationV1::Unit(
                        TerminalUnitReturnV1::issue(input.owner(), statement.site().clone()),
                    ));
                    true
                }
                ASTNode::Return { value: Some(_), .. } => match input
                    .source()
                    .child_expr_from_stmt(&statement, ExprChildRoleV1::ReturnValue)
                {
                    Ok(value) if map_literal_keys(input, value.site()).is_some() => {
                        // `return %{...}` transfers construction to the caller;
                        // the Value relation records the exact returned site.
                        // The ownership transfer itself belongs to the
                        // lifecycle contract, not to this row.
                        let keys = map_literal_keys(input, value.site()).unwrap();
                        let owned = OwnedExprSiteV1::new(input.owner(), value.site().clone());
                        let mut used = std::collections::BTreeSet::new();
                        let mut nested = Vec::new();
                        match map_flow::observe_map(
                            input,
                            &owned,
                            MapDestinationV1::ReturnBoundary(statement.site().clone()),
                            keys,
                            locals,
                            &homes,
                            &mut used,
                            &mut nested,
                            map_compatible,
                        )? {
                            Ok((map, remaining)) => {
                                *homes = remaining;
                                *terminal_relation =
                                    Some(TerminalRelationV1::Value(TerminalValueReturnV1::issue(
                                        input.owner(),
                                        statement.site().clone(),
                                        value.site().clone(),
                                        TerminalReturnedSourceV1::MapLiteral(owned.clone()),
                                    )));
                                maps.push(map_flow::MapHomeObservation::Complete(map));
                            }
                            Err(issue) => {
                                unavailable.get_or_insert(issue);
                                maps.push(map_flow::MapHomeObservation::Unavailable {
                                    site: owned,
                                });
                            }
                        }
                        maps.extend(nested);
                        true
                    }
                    Ok(value) => {
                        // `return foo(%{...})` — argument-position map literals
                        // need flow rows regardless of how the return value
                        // itself classifies. Siblings share one `used` set so a
                        // Home cannot transfer into two argument maps.
                        if let Some(shape) = input.body_shape() {
                            let mut arguments: Vec<(u32, SourceExprSiteV1)> = shape
                                .relations()
                                .iter()
                                .filter_map(|row| match row.role() {
                                    SourcePathSegmentV1::Argument(ordinal)
                                        if row.parent() == value.site().node() =>
                                    {
                                        Some((*ordinal, row.child().clone()))
                                    }
                                    _ => None,
                                })
                                .collect();
                            arguments.sort_by_key(|(ordinal, _)| *ordinal);
                            let mut used = std::collections::BTreeSet::new();
                            let mut nested = Vec::new();
                            for (ordinal, child) in arguments {
                                let Some(keys) = map_literal_keys(input, &child) else {
                                    continue;
                                };
                                let owned = OwnedExprSiteV1::new(input.owner(), child);
                                match map_flow::observe_map(
                                    input,
                                    &owned,
                                    MapDestinationV1::CallArgument {
                                        call: OwnedExprSiteV1::new(
                                            input.owner(),
                                            value.site().clone(),
                                        ),
                                        ordinal,
                                    },
                                    keys,
                                    locals,
                                    &homes,
                                    &mut used,
                                    &mut nested,
                                    map_compatible,
                                )? {
                                    Ok((map, remaining)) => {
                                        *homes = remaining;
                                        maps.push(map_flow::MapHomeObservation::Complete(map));
                                    }
                                    Err(issue) => {
                                        unavailable.get_or_insert(issue);
                                        maps.push(map_flow::MapHomeObservation::Unavailable {
                                            site: owned,
                                        });
                                    }
                                }
                            }
                            maps.extend(nested);
                        }
                        match input.function().expression_source().literal(value.site()) {
                            // `return void` spells the explicit-unit terminal;
                            // completion classifies it like a bare return.
                            Some(ResolvedLiteralSourceV1::Void) => {
                                *terminal_relation =
                                    Some(TerminalRelationV1::Unit(TerminalUnitReturnV1::issue(
                                        input.owner(),
                                        statement.site().clone(),
                                    )));
                                true
                            }
                            Some(ResolvedLiteralSourceV1::Integer(number)) => {
                                *terminal_relation = Some(TerminalRelationV1::IntegerLiteral(
                                    TerminalIntegerLiteralReturnV1::issue(
                                        input.owner(),
                                        statement.site().clone(),
                                        value.site().clone(),
                                        *number,
                                    ),
                                ));
                                true
                            }
                            _ if terminal_call(&OwnedExprSiteV1::new(
                                input.owner(),
                                value.site().clone(),
                            ))? =>
                            {
                                // Every argument seals as its own source
                                // class: an integer literal carries its
                                // value; a `%{...}` literal names its exact
                                // `CallArgument` flow row (already observed
                                // above for this return value). Neither arm
                                // re-reads a type or resolves a name.
                                let call_owned =
                                    OwnedExprSiteV1::new(input.owner(), value.site().clone());
                                let argument_class = |ordinal: u32,
                                                      site: &SourceExprSiteV1|
                                 -> Option<TerminalCallArgumentV1> {
                                    match input.function().expression_source().literal(site) {
                                        Some(ResolvedLiteralSourceV1::Integer(value)) => {
                                            return Some(TerminalCallArgumentV1::I64(*value));
                                        }
                                        _ => {}
                                    }
                                    let owned = OwnedExprSiteV1::new(input.owner(), site.clone());
                                    let mut candidates = maps.iter().filter(|row| {
                                        row.site() == &owned
                                            && matches!(
                                                row.complete().map(|flow| flow.destination()),
                                                Some(MapDestinationV1::CallArgument {
                                                    call,
                                                    ordinal: expected,
                                                }) if call == &call_owned && *expected == ordinal
                                            )
                                    });
                                    let row = candidates.next()?;
                                    if candidates.next().is_some() {
                                        return None;
                                    }
                                    row.complete()?;
                                    Some(TerminalCallArgumentV1::Map(owned))
                                };
                                let arguments = input
                                    .function()
                                    .direct_call_observations()
                                    .find(|(site, _)| *site == value.site())
                                    .and_then(|(_, row)| {
                                        row.argument_sites()
                                            .iter()
                                            .enumerate()
                                            .map(|(ordinal, site)| {
                                                argument_class(ordinal as u32, site)
                                            })
                                            .collect::<Option<Vec<_>>>()
                                    });
                                let arguments = arguments.or_else(|| {
                                    input
                                        .function()
                                        .method_calls()
                                        .find(|(site, _)| *site == value.site())
                                        .and_then(|(_, row)| {
                                            row.arguments()
                                                .iter()
                                                .enumerate()
                                                .map(|(ordinal, argument)| {
                                                    argument_class(ordinal as u32, argument.site())
                                                })
                                                .collect::<Option<Vec<_>>>()
                                        })
                                });
                                if let Some(arguments) = arguments {
                                    *terminal_relation = Some(TerminalRelationV1::Call(
                                        TerminalI64CallReturnV1::issue(
                                            input.owner(),
                                            statement.site().clone(),
                                            value.site().clone(),
                                            arguments.into_boxed_slice(),
                                        ),
                                    ));
                                    true
                                } else {
                                    false
                                }
                            }
                            _ => {
                                match return_scalar(input, value.site(), &locals, field_is_integer)?
                                {
                                    Some(ReturnScalar::I64Add { site, field_reads }) => {
                                        *terminal_relation = Some(TerminalRelationV1::I64Add(
                                            TerminalI64AddReturnV1::issue(
                                                input.owner(),
                                                statement.site().clone(),
                                                site,
                                                field_reads,
                                            ),
                                        ));
                                        true
                                    }
                                    Some(ReturnScalar::IntegerField(field_read_site)) => {
                                        *terminal_relation = Some(TerminalRelationV1::I64Field(
                                            TerminalI64FieldReturnV1::issue(
                                                input.owner(),
                                                statement.site().clone(),
                                                value.site().clone(),
                                                field_read_site,
                                            ),
                                        ));
                                        true
                                    }
                                    Some(_) => true,
                                    None => {
                                        // `return <map>.get("<key>")` — the
                                        // bounded readable-Map terminal. The
                                        // receiver stays live: owned locals
                                        // still owe their End and borrowed
                                        // formals keep caller ownership.
                                        if let Some(row) = terminal_map_get(
                                            input,
                                            statement.site(),
                                            value.site(),
                                            &locals,
                                        ) {
                                            *terminal_relation =
                                                Some(TerminalRelationV1::MapGet(row));
                                            true
                                        } else {
                                            match terminal_returned_source(
                                                input,
                                                value.site(),
                                                &locals,
                                            ) {
                                                Some(returned) => {
                                                    // `return new <class>(...)` —
                                                    // the fresh object transfers
                                                    // to the caller; prior Homes
                                                    // remain this frame's exit
                                                    // obligation. Claim-member
                                                    // sites co-seal their
                                                    // destination-less result
                                                    // prefix and argument
                                                    // observation at this point.
                                                    if let TerminalReturnedSourceV1::Construction(
                                                        owned,
                                                    ) = &returned
                                                    {
                                                        if result_sites.contains(owned) {
                                                            match input.source().expr_at(owned) {
                                                                Ok(located) => {
                                                                    match located.node() {
                                                            ASTNode::New {
                                                                arguments,
                                                                field_initializers,
                                                                ..
                                                            } => {
                                                                let observed_arguments = arguments.iter().enumerate().map(|(ordinal, _)| {
                                                                let ordinal = u32::try_from(ordinal).map_err(|_| {
                                                                    SelectedNewArgumentUnavailableV1::ArgumentOrdinalOverflow { new_site: owned.clone() }
                                                                })?;
                                                                let argument = input.source().child_expr_from_expr(
                                                                    &located, ExprChildRoleV1::CallArgument(ordinal),
                                                                ).map_err(|_| SelectedNewArgumentUnavailableV1::SourceMismatch { new_site: owned.clone() })?;
                                                                let kind = locals.observe(argument.site()).and_then(OrdinaryObservation::into_selected_argument).ok_or_else(|| {
                                                                    SelectedNewArgumentUnavailableV1::ArgumentNotTrivial { new_site: owned.clone(), site: argument.site().clone() }
                                                                })?;
                                                                Ok(SelectedNewArgumentV1::new(ordinal, argument.site().clone(), kind))
                                                            }).collect::<Result<Vec<_>, _>>().map(|rows| rows.into_boxed_slice());
                                                                argument_observations.insert(
                                                                    owned.clone(),
                                                                    SelectedNewArgumentObservationV1::new(
                                                                        owned.clone(),
                                                                        observed_arguments,
                                                                    ),
                                                                );
                                                                if !field_initializers.is_empty() {
                                                                    unavailable.get_or_insert_with(|| {
                                                                        HomePrefixUnavailableV1::OverridesNotCovered(owned.site().clone())
                                                                    });
                                                                }
                                                                for argument in 0..arguments.len() {
                                                                    let arg = input.source().child_expr_from_expr(
                                                                        &located,
                                                                        ExprChildRoleV1::CallArgument(argument as u32),
                                                                    );
                                                                    match arg {
                                                                        Ok(arg)
                                                                            if locals
                                                                                .observe(arg.site())
                                                                                .and_then(
                                                                                    OrdinaryObservation::into_selected_argument,
                                                                                )
                                                                                .is_some() => {}
                                                                        Ok(arg) => {
                                                                            unavailable.get_or_insert_with(|| {
                                                                                HomePrefixUnavailableV1::ArgumentNotCovered(
                                                                                    arg.site().clone(),
                                                                                )
                                                                            });
                                                                        }
                                                                        Err(_) => {
                                                                            unavailable.get_or_insert(
                                                                                HomePrefixUnavailableV1::SourceMismatch,
                                                                            );
                                                                        }
                                                                    }
                                                                }
                                                                let result_prefix = match &*unavailable {
                                                                    Some(issue) => Err(issue.clone()),
                                                                    None => issue_result_new_fault_continuation_v1(input, owned)
                                                                        .map_err(|_| HomePrefixUnavailableV1::SourceMismatch)
                                                                        .map(|outward_fault| ResultNewHomePrefixV1 {
                                                                            prior_homes: homes.iter().rev().copied().collect(),
                                                                            outward_fault,
                                                                            covered_statements: covered_statements.clone().into_boxed_slice(),
                                                                        }),
                                                                };
                                                                result_prefixes.insert(owned.clone(), result_prefix);
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
                                                        }
                                                    }
                                                    // A returned local/Home leaves
                                                    // with the caller: it is no
                                                    // longer this function's
                                                    // terminal cleanup.
                                                    let binding = match &returned {
                                                        TerminalReturnedSourceV1::MapLocal(
                                                            binding,
                                                        )
                                                        | TerminalReturnedSourceV1::Home {
                                                            binding,
                                                            ..
                                                        } => Some(*binding),
                                                        _ => None,
                                                    };
                                                    if let Some(binding) = binding {
                                                        homes.retain(|home| *home != binding);
                                                        locals.consume_home(binding);
                                                    }
                                                    *terminal_relation =
                                                        Some(TerminalRelationV1::Value(
                                                            TerminalValueReturnV1::issue(
                                                                input.owner(),
                                                                statement.site().clone(),
                                                                value.site().clone(),
                                                                returned,
                                                            ),
                                                        ));
                                                    true
                                                }
                                                None => {
                                                    // `return <qualified call>`
                                                    // — the sealed method-call
                                                    // row proves a qualified
                                                    // receiver; the relation
                                                    // records the call site only
                                                    // and owns no callee, result
                                                    // class, or handoff authority.
                                                    if input.function().method_calls().any(
                                                    |(site, row)| {
                                                        *site == *value.site()
                                                            && matches!(
                                                                row.receiver(),
                                                                ResolvedMethodCallReceiverSourceV1::QualifiedUnbound
                                                            )
                                                    },
                                                ) {
                                                    *terminal_relation = Some(
                                                        TerminalRelationV1::OpaqueCall(
                                                            TerminalOpaqueCallReturnV1::issue(
                                                                input.owner(),
                                                                statement.site().clone(),
                                                                value.site().clone(),
                                                            ),
                                                        ),
                                                    );
                                                    true
                                                } else {
                                                    false
                                                }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(_) => false,
                },
                _ => false,
            };
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
            *terminal_homes = match &*unavailable {
                Some(issue) => Err(issue.clone()),
                None if !scalar_return => Err(HomePrefixUnavailableV1::ReturnValueNotCovered(
                    statement.site().clone(),
                )),
                None if results.len() != selected.len() => {
                    Err(HomePrefixUnavailableV1::SourceMismatch)
                }
                None => Ok(homes.iter().rev().copied().collect()),
            };
            if terminal_homes.is_err() {
                // `return new <class>` is exact-site evidence: its own
                // `ResultNewHomePrefixV1` — and thus the retained result
                // claim — already records whatever flow gap made the
                // terminal homes unavailable. Receiver-entry demands and
                // other prefix gaps must not erase the construction proof
                // a caller's handle-result edge reads from the child
                // contract; every other returned source depends on this
                // frame's local flow, so those still drop.
                *terminal_relation = terminal_relation.take().filter(|relation| {
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
            break;
        }
        let ASTNode::Local {
            variables,
            initial_values,
            ..
        } = statement.node()
        else {
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
                local_calls.push(local_call);
                homes.push(binding);
                locals.install_received_handle(binding);
                continue;
            }
            if let Some(destination) = selected.get(&owned) {
                if *destination != binding {
                    unavailable.get_or_insert(HomePrefixUnavailableV1::SourceMismatch);
                }
                let located = input.source().expr_at(&owned);
                match located {
                    Ok(new) => {
                        match new.node() {
                            ASTNode::New {
                                arguments,
                                field_initializers,
                                ..
                            } => {
                                let observed_arguments = arguments.iter().enumerate().map(|(ordinal, _)| {
                                let ordinal = u32::try_from(ordinal).map_err(|_| {
                                    SelectedNewArgumentUnavailableV1::ArgumentOrdinalOverflow { new_site: owned.clone() }
                                })?;
                                let argument = input.source().child_expr_from_expr(
                                    &new, ExprChildRoleV1::CallArgument(ordinal),
                                ).map_err(|_| SelectedNewArgumentUnavailableV1::SourceMismatch { new_site: owned.clone() })?;
                                let kind = locals.observe(argument.site()).and_then(OrdinaryObservation::into_selected_argument).ok_or_else(|| {
                                    SelectedNewArgumentUnavailableV1::ArgumentNotTrivial { new_site: owned.clone(), site: argument.site().clone() }
                                })?;
                                Ok(SelectedNewArgumentV1::new(ordinal, argument.site().clone(), kind))
                            }).collect::<Result<Vec<_>, _>>().map(|rows| rows.into_boxed_slice());
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
                                        Ok(arg)
                                            if locals
                                                .observe(arg.site())
                                                .and_then(
                                                    OrdinaryObservation::into_selected_argument,
                                                )
                                                .is_some() => {}
                                        Ok(arg) => {
                                            unavailable.get_or_insert_with(|| {
                                                HomePrefixUnavailableV1::ArgumentNotCovered(
                                                    arg.site().clone(),
                                                )
                                            });
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
    Ok(())
}
