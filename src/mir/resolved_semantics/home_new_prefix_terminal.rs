//! Terminal-statement observation inside `scan_statement_flow`: each
//! explicit exit site owns its own relation, result co-seal, and Home
//! snapshot — every row lands in the site-keyed caller maps.
use super::*;

#[allow(clippy::too_many_arguments)]
pub(super) fn observe_terminal_statement<'a, E>(
    input: ResolvedFunctionLoweringInputV1<'a>,
    statement: &crate::mir::compiler::located::LocatedStmtV1<'a>,
    result_sites: &BTreeSet<OwnedExprSiteV1>,
    results: &mut BTreeMap<OwnedExprSiteV1, Result<CallerNewHomePrefixV1, HomePrefixUnavailableV1>>,
    exit_homes: &mut BTreeMap<SourceStmtSiteV1, Result<RootHomeExitV1, HomePrefixUnavailableV1>>,
    path_calls: &mut BTreeSet<OwnedExprSiteV1>,
    maps: &mut Vec<MapHomeObservation>,
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
    argument_i64_field: &mut impl FnMut(
        &OwnedExprSiteV1,
        &SourceExprSiteV1,
        BindingRefV1,
        BindingRefV1,
        &str,
    ) -> Result<bool, E>,
    local_lexical_i64_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<bool, E>,
    borrowed_actuals: &mut impl FnMut(
        &OwnedExprSiteV1,
        BorrowedCallActualRequestV1<'_>,
    ) -> Result<Option<BorrowedCallArgumentsV1>, E>,

    view_use: &mut impl FnMut(&OwnedExprSiteV1, BorrowedViewUseRequestV1<'_>) -> Result<bool, E>,
    object_return: &mut impl FnMut(
        &OwnedExprSiteV1,
    ) -> Result<Option<ObjectReturnCallQualificationV1>, E>,
    local_calls: &mut Vec<LocalCallObservationV1>,
    static_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<Option<StaticI64CallClaimV1>, E>,
) -> Result<(), E> {
    let mut relation: Option<TerminalRelationV1> = None;
    // Qualified object returns are exclusive, including unavailable support.
    // They must never reach the scalar borrowed-argument shortcut.
    let object_terminal = if matches!(statement.node(), ASTNode::Return { value: Some(_), .. }) {
        if let Ok(value) = input
            .source()
            .child_expr_from_stmt(statement, ExprChildRoleV1::ReturnValue)
        {
            let site = OwnedExprSiteV1::new(input.owner(), value.site().clone());
            if let Some(loan) = object_return(&site)? {
                Some(super::object_return::observe_object_return_source(
                    input,
                    statement,
                    loan,
                    locals,
                    local_calls,
                    path_calls,
                    homes,
                    borrowed_actuals,
                )?)
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };
    let mut static_terminal = None;
    if object_terminal.is_none() {
        if let Ok(value) = input
            .source()
            .child_expr_from_stmt(statement, ExprChildRoleV1::ReturnValue)
        {
            // The same scalar preflight observes the original call children.
            // Field returns keep their existing field issuer and relations.
            let borrowed_mul = scalar_expression::observe_guarded_mul_return(
                input,
                value.site(),
                statement.site(),
                homes,
                view_use,
                static_call,
                borrowed_actuals,
            )?;
            use scalar_expression::GuardedMulReturnV1;
            let mul = match borrowed_mul {
                GuardedMulReturnV1::Unavailable => {
                    match scalar_expression::observe_local_mul_return(
                        input,
                        value.site(),
                        statement.site(),
                        locals,
                        homes,
                        static_call,
                        borrowed_actuals,
                    )? {
                        GuardedMulReturnV1::Unselected => GuardedMulReturnV1::Unavailable,
                        local => local,
                    }
                }
                other => other,
            };
            let selected_mul = !matches!(mul, GuardedMulReturnV1::Unselected);
            let scalar = match mul {
                GuardedMulReturnV1::Observed(calls) => Some((SourceScalarKind::Integer, calls)),
                GuardedMulReturnV1::Unavailable => {
                    let issue = unavailable
                        .get_or_insert(HomePrefixUnavailableV1::ReturnValueNotCovered(
                            statement.site().clone(),
                        ))
                        .clone();
                    exit_homes.insert(statement.site().clone(), Err(issue));
                    return Ok(());
                }
                GuardedMulReturnV1::Unselected => scalar_expression::observe_scalar_expression(
                    input,
                    value.site(),
                    locals,
                    Some(SourceScalarKind::Integer),
                    false,
                    &mut |_, _| Ok(None),
                    statement.site(),
                    homes,
                    static_call,
                    &mut |_, _| Ok(false),
                    borrowed_actuals,
                )?,
            };
            if let Some((_, calls)) = scalar {
                if selected_mul || !calls.is_empty() {
                    static_terminal = Some(
                        if calls.len() == 1 && calls[0].site().site() == value.site() {
                            TerminalRelationV1::Call(TerminalI64CallReturnV1::issue(
                                input.owner(),
                                statement.site().clone(),
                                value.site().clone(),
                                Box::new([]),
                            ))
                        } else {
                            TerminalRelationV1::I64Scalar(TerminalI64ScalarReturnV1::issue(
                                input.owner(),
                                statement.site().clone(),
                                value.site().clone(),
                            ))
                        },
                    );
                    path_calls.extend(calls.iter().map(|call| call.site().clone()));
                    local_calls.extend(calls);
                }
            }
        }
    }
    let borrowed_terminal = if object_terminal.is_none() && static_terminal.is_none() {
        local_call_flow::issue_borrowed_i64_terminal_call(
            input,
            statement,
            homes,
            locals,
            local_lexical_i64_call,
            borrowed_actuals,
        )?
    } else {
        None
    };
    let scalar_return = if let Some(obligation) = object_terminal {
        match obligation {
            Ok(value) => {
                relation = Some(TerminalRelationV1::Value(value));
                true
            }
            Err(issue) => {
                unavailable.get_or_insert(issue);
                false
            }
        }
    } else if let Some(terminal) = static_terminal {
        relation = Some(terminal);
        true
    } else if let Some(terminal) = borrowed_terminal {
        relation = Some(TerminalRelationV1::Call(terminal));
        true
    } else {
        match statement.node() {
            ASTNode::Return { value: None, .. } => {
                relation = Some(TerminalRelationV1::Unit(TerminalUnitReturnV1::issue(
                    input.owner(),
                    statement.site().clone(),
                )));
                true
            }
            ASTNode::Return { value: Some(_), .. } => match input
                .source()
                .child_expr_from_stmt(statement, ExprChildRoleV1::ReturnValue)
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
                            relation =
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
                            maps.push(map_flow::MapHomeObservation::Unavailable { site: owned });
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
                                    call: OwnedExprSiteV1::new(input.owner(), value.site().clone()),
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
                            relation = Some(TerminalRelationV1::Unit(TerminalUnitReturnV1::issue(
                                input.owner(),
                                statement.site().clone(),
                            )));
                            true
                        }
                        Some(ResolvedLiteralSourceV1::Integer(number)) => {
                            relation = Some(TerminalRelationV1::IntegerLiteral(
                                TerminalIntegerLiteralReturnV1::issue(
                                    input.owner(),
                                    statement.site().clone(),
                                    value.site().clone(),
                                    *number,
                                ),
                            ));
                            true
                        }
                        Some(ResolvedLiteralSourceV1::Bool(value_bool)) => {
                            relation = Some(TerminalRelationV1::BoolLiteral(
                                TerminalBoolLiteralReturnV1::issue(
                                    input.owner(),
                                    statement.site().clone(),
                                    value.site().clone(),
                                    *value_bool,
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
                            let argument_class =
                                |ordinal: u32,
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
                                        .map(|(ordinal, site)| argument_class(ordinal as u32, site))
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
                                relation =
                                    Some(TerminalRelationV1::Call(TerminalI64CallReturnV1::issue(
                                        input.owner(),
                                        statement.site().clone(),
                                        value.site().clone(),
                                        arguments.into_boxed_slice(),
                                    )));
                                true
                            } else {
                                false
                            }
                        }
                        _ => {
                            let checked_return = match input.function().variable_ref(value.site()) {
                                Some(ResolvedLexicalRefV1::Local(binding)) => view_use(
                                    &OwnedExprSiteV1::new(input.owner(), value.site().clone()),
                                    BorrowedViewUseRequestV1::IntegerReturn {
                                        exit: statement.site(),
                                        binding,
                                    },
                                )?,
                                _ => false,
                            };
                            let scalar = if checked_return {
                                Some(ReturnScalar::Integer)
                            } else {
                                return_scalar(input, value.site(), &locals, field_is_integer)?
                            };
                            match scalar {
                                Some(ReturnScalar::I64Add { site, field_reads }) => {
                                    relation = Some(TerminalRelationV1::I64Add(
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
                                    relation = Some(TerminalRelationV1::I64Field(
                                        TerminalI64FieldReturnV1::issue(
                                            input.owner(),
                                            statement.site().clone(),
                                            value.site().clone(),
                                            field_read_site,
                                        ),
                                    ));
                                    true
                                }
                                // A top-level proven-i64 scalar — a bound
                                // local or trivial integer expression — is
                                // its own source relation. Literal and Call
                                // exits were classified above; this row only
                                // records the exact return/value sites.
                                Some(ReturnScalar::Integer) => {
                                    relation = Some(TerminalRelationV1::I64Scalar(
                                        TerminalI64ScalarReturnV1::issue(
                                            input.owner(),
                                            statement.site().clone(),
                                            value.site().clone(),
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
                                        relation = Some(TerminalRelationV1::MapGet(row));
                                        true
                                    } else {
                                        match terminal_returned_source(input, value.site(), &locals)
                                        {
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
                                                            Ok(located) => match located.node() {
                                                                ASTNode::New {
                                                                    arguments,
                                                                    field_initializers,
                                                                    ..
                                                                } => {
                                                                    let observed_arguments = {
                                                                        let mut rows =
                                                                            Vec::with_capacity(
                                                                                arguments.len(),
                                                                            );
                                                                        let mut row_error = None;
                                                                        for (ordinal, _) in
                                                                            arguments
                                                                                .iter()
                                                                                .enumerate()
                                                                        {
                                                                            let Ok(ordinal) =
                                                                                u32::try_from(
                                                                                    ordinal,
                                                                                )
                                                                            else {
                                                                                row_error = Some(SelectedNewArgumentUnavailableV1::ArgumentOrdinalOverflow { new_site: owned.clone() });
                                                                                break;
                                                                            };
                                                                            let argument = match input.source().child_expr_from_expr(
                                                                &located, ExprChildRoleV1::CallArgument(ordinal),
                                                            ) {
                                                                Ok(argument) => argument,
                                                                Err(_) => {
                                                                    row_error = Some(SelectedNewArgumentUnavailableV1::SourceMismatch { new_site: owned.clone() });
                                                                    break;
                                                                }
                                                            };
                                                                            match locals.observe_selected_argument(argument.site(), argument_i64_field)? {
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
                                                                            Some(error) => {
                                                                                Err(error)
                                                                            }
                                                                            None => Ok(rows
                                                                                .into_boxed_slice(
                                                                                )),
                                                                        }
                                                                    };
                                                                    argument_observations.insert(
                                                            owned.clone(),
                                                            SelectedNewArgumentObservationV1::new(
                                                                owned.clone(),
                                                                observed_arguments,
                                                            ),
                                                        );
                                                                    if !field_initializers
                                                                        .is_empty()
                                                                    {
                                                                        unavailable.get_or_insert_with(|| {
                                                                HomePrefixUnavailableV1::OverridesNotCovered(owned.site().clone())
                                                            });
                                                                    }
                                                                    let mut moved_arguments =
                                                                        Vec::new();
                                                                    for argument in
                                                                        0..arguments.len()
                                                                    {
                                                                        let arg = input.source().child_expr_from_expr(
                                                                &located,
                                                                ExprChildRoleV1::CallArgument(argument as u32),
                                                            );
                                                                        match arg {
                                                                            Ok(arg) => {
                                                                                match locals
                                                                        .observe_selected_argument(
                                                                            arg.site(),
                                                                            argument_i64_field,
                                                                        )? {
                                                                        Some(kind) => {
                                                                            match kind {
                                                                                // An argument
                                                                                // naming a live
                                                                                // owned binding
                                                                                // moves the lease
                                                                                // into the
                                                                                // constructed
                                                                                // object; the
                                                                                // frame's exit
                                                                                // obligation ends
                                                                                // at this edge.
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
                                                                    result_prefixes.insert(
                                                                        owned.clone(),
                                                                        result_prefix,
                                                                    );
                                                                    // The arg
                                                                    // move
                                                                    // completes
                                                                    // on the
                                                                    // invoke's
                                                                    // Normal
                                                                    // edge —
                                                                    // `prior_homes`
                                                                    // above kept
                                                                    // the binding
                                                                    // so a Fault
                                                                    // unwind still
                                                                    // discharges
                                                                    // it.
                                                                    for moved in moved_arguments {
                                                                        homes.retain(|home| {
                                                                            *home != moved
                                                                        });
                                                                        locals.consume_home(moved);
                                                                    }
                                                                }
                                                                _ => {
                                                                    unavailable.get_or_insert(HomePrefixUnavailableV1::SourceMismatch);
                                                                }
                                                            },
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
                                                    TerminalReturnedSourceV1::MapLocal(binding)
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
                                                relation = Some(TerminalRelationV1::Value(
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
                                            relation = Some(
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
        }
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
        None => Ok(RootHomeExitV1::issue(
            homes.iter().rev().copied().collect(),
            path_calls.iter().cloned().collect(),
        )),
    };
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
        terminal_relations.insert(statement.site().clone(), relation);
    }
    Ok(())
}
