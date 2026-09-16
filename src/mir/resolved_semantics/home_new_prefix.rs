//! Caller-prefix Home availability, issued from one resolved source loan.
//!
//! A successful prefix describes Normal-path installations, not runtime
//! cleanup readiness. Construction/argument unwind remains a required
//! dependency; unknown prefix meaning never becomes an empty Home list.

use super::{
    BindingRefV1, ExprChildRoleV1, FunctionOwnerIdV1, HomeDemandV1, OwnedExprSiteV1,
    ResolvedLexicalRefV1, ResolvedLiteralSourceV1, SourceBindingSiteV1, SourceExprSiteV1,
    SourcePathSegmentV1, SourceStmtSiteV1,
};
use crate::ast::ASTNode;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_control_flow::{issue_new_fault_continuation_v1, NewFaultContinuationV1};
use std::collections::BTreeMap;

#[path = "selected_new_arguments.rs"]
mod selected_new_arguments;
pub(crate) use selected_new_arguments::{
    SelectedNewArgumentKindV1, SelectedNewArgumentObservationV1, SelectedNewArgumentUnavailableV1,
    SelectedNewArgumentV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum HomePrefixUnavailableV1 {
    EntryDemandMissing,
    SourceMismatch,
    PrefixNotCovered(SourceStmtSiteV1),
    ArgumentNotCovered(SourceExprSiteV1),
    OverridesNotCovered(SourceExprSiteV1),
    TerminalNotCovered,
    MapCandidateNotCovered(SourceExprSiteV1),
    ReturnValueNotCovered(SourceStmtSiteV1),
}

/// Immutable source facts. Cloning preserves the same owner/site identities;
/// no physical value, storage policy, or cleanup implementation is issued.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CallerNewHomePrefixV1 {
    destination: BindingRefV1,
    prior_homes: Box<[BindingRefV1]>,
    outward_fault: NewFaultContinuationV1,
    covered_statements: Box<[SourceStmtSiteV1]>,
}

impl CallerNewHomePrefixV1 {
    pub(crate) fn destination(&self) -> BindingRefV1 {
        self.destination
    }
    pub(crate) fn prior_homes(&self) -> &[BindingRefV1] {
        &self.prior_homes
    }
    pub(crate) fn required_unwind(&self) -> &OwnedExprSiteV1 {
        self.outward_fault.site()
    }
    pub(crate) fn outward_fault(&self) -> &NewFaultContinuationV1 {
        &self.outward_fault
    }
    pub(crate) fn covered_statements(&self) -> &[SourceStmtSiteV1] {
        &self.covered_statements
    }
}

#[path = "home_prefix_local_flow.rs"]
mod local_flow;
pub(crate) use local_flow::SourceScalarKind;
use local_flow::{OrdinaryObservation, PrefixLocalFlow};
#[path = "home_local_call_flow.rs"]
mod local_call_flow;
pub(crate) use local_call_flow::{LocalCallObservationV1, LocalCallResultClassV1};
#[path = "home_map_descendant_flow.rs"]
mod map_descendant_flow;
#[path = "home_map_flow.rs"]
mod map_flow;
#[path = "home_terminal_relation.rs"]
mod terminal_relation;
pub(crate) use map_flow::{
    ArrayElementSource, MapDestinationV1, MapEntryStoreClassV1, MapHomeEntry, MapHomeFlow,
    MapHomeObservation, MapValueSource, RootHomeFlow,
};
use terminal_relation::{map_literal_keys, return_scalar, terminal_returned_source, ReturnScalar};
pub(crate) use terminal_relation::{
    TerminalI64AddReturnV1, TerminalI64CallReturnV1, TerminalI64FieldReturnV1,
    TerminalIntegerLiteralReturnV1, TerminalRelationV1, TerminalReturnedSourceV1,
    TerminalUnitReturnV1, TerminalValueReturnV1,
};

pub(crate) fn issue_new_home_prefixes_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    selected: &BTreeMap<OwnedExprSiteV1, BindingRefV1>,
) -> BTreeMap<OwnedExprSiteV1, Result<CallerNewHomePrefixV1, HomePrefixUnavailableV1>> {
    scan_new_home_flow(
        input,
        selected,
        std::iter::empty(),
        None,
        &mut |_, _, _, _, _| Ok::<_, std::convert::Infallible>(false),
        &mut |_, _| Ok(false),
        &mut |_| Ok(false),
        &mut |_| Ok(false),
    )
    .unwrap_or_else(|never| match never {})
    .0
}

/// One source walk supplies both New-failure prefixes and terminal ownership.
/// The caller must take the terminal from the Completion verified on this input.
pub(crate) fn scan_new_home_flow<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    selected: &BTreeMap<OwnedExprSiteV1, BindingRefV1>,
    parameters: impl IntoIterator<
        Item = (
            u32,
            BindingRefV1,
            crate::mir::callable_parameter_contract::CallableParameterContractKindV1,
        ),
    >,
    terminal: Option<&SourceStmtSiteV1>,
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
) -> Result<
    (
        BTreeMap<OwnedExprSiteV1, Result<CallerNewHomePrefixV1, HomePrefixUnavailableV1>>,
        RootHomeFlow,
        Option<TerminalRelationV1>,
        BTreeMap<OwnedExprSiteV1, SelectedNewArgumentObservationV1>,
    ),
    E,
> {
    let mut results = BTreeMap::new();
    let mut terminal_homes = Err(HomePrefixUnavailableV1::TerminalNotCovered);
    let mut maps = Vec::new();
    let mut local_calls = Vec::new();
    let mut terminal_relation = None;
    let mut argument_observations = BTreeMap::new();
    let function = input.function();
    let mut unavailable = (function
        .declaration_sites()
        .any(|site| matches!(site, SourceBindingSiteV1::Receiver))
        || !input
            .forest()
            .ordered_capture_demands(input.owner())
            .is_empty())
    .then_some(HomePrefixUnavailableV1::EntryDemandMissing);
    let Ok(body) = input.source().root_body() else {
        return Ok((
            selected
                .keys()
                .map(|site| (site.clone(), Err(HomePrefixUnavailableV1::SourceMismatch)))
                .collect(),
            RootHomeFlow {
                terminal: Err(HomePrefixUnavailableV1::SourceMismatch),
                maps,
                local_calls,
            },
            terminal_relation,
            argument_observations,
        ));
    };
    let mut locals = PrefixLocalFlow::new(input);
    if !locals.install_parameters(parameters) {
        unavailable = Some(HomePrefixUnavailableV1::EntryDemandMissing);
    }
    let mut homes = Vec::new();
    let mut covered_statements = Vec::new();
    for index in 0..body.statements().len() {
        let Ok(statement) = input.source().body_stmt(&body, index) else {
            unavailable = Some(HomePrefixUnavailableV1::SourceMismatch);
            break;
        };
        covered_statements.push(statement.site().clone());
        if terminal == Some(statement.site()) {
            let scalar_return = match statement.node() {
                ASTNode::Return { value: None, .. } => {
                    terminal_relation = Some(TerminalRelationV1::Unit(
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
                            &mut locals,
                            &homes,
                            &mut used,
                            &mut nested,
                            map_compatible,
                        )? {
                            Ok((map, remaining)) => {
                                homes = remaining;
                                terminal_relation =
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
                                    &mut locals,
                                    &homes,
                                    &mut used,
                                    &mut nested,
                                    map_compatible,
                                )? {
                                    Ok((map, remaining)) => {
                                        homes = remaining;
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
                                terminal_relation =
                                    Some(TerminalRelationV1::Unit(TerminalUnitReturnV1::issue(
                                        input.owner(),
                                        statement.site().clone(),
                                    )));
                                true
                            }
                            Some(ResolvedLiteralSourceV1::Integer(number)) => {
                                terminal_relation = Some(TerminalRelationV1::IntegerLiteral(
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
                                let arguments = input
                                    .function()
                                    .direct_call_observations()
                                    .find(|(site, _)| *site == value.site())
                                    .and_then(|(_, row)| {
                                        row.argument_sites()
                                            .iter()
                                            .map(|site| {
                                                match input
                                                    .function()
                                                    .expression_source()
                                                    .literal(site)
                                                {
                                                    Some(ResolvedLiteralSourceV1::Integer(
                                                        value,
                                                    )) => Some(*value),
                                                    _ => None,
                                                }
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
                                                .map(|argument| {
                                                    match input
                                                        .function()
                                                        .expression_source()
                                                        .literal(argument.site())
                                                    {
                                                        Some(ResolvedLiteralSourceV1::Integer(
                                                            value,
                                                        )) => Some(*value),
                                                        _ => None,
                                                    }
                                                })
                                                .collect::<Option<Vec<_>>>()
                                        })
                                });
                                if let Some(arguments) = arguments {
                                    terminal_relation = Some(TerminalRelationV1::Call(
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
                                        terminal_relation = Some(TerminalRelationV1::I64Add(
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
                                        terminal_relation = Some(TerminalRelationV1::I64Field(
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
                                        match terminal_returned_source(input, value.site(), &locals)
                                        {
                                            Some(returned) => {
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
                                                terminal_relation =
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
                                            None => false,
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
                &mut locals,
                &mut homes,
                &mut maps,
                map_compatible,
            )?;
            terminal_homes = match &unavailable {
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
                terminal_relation = None;
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
                &mut locals,
                &mut homes,
                &mut maps,
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
                let result = match &unavailable {
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
                        &mut locals,
                        &homes,
                        &mut used,
                        &mut nested,
                        map_compatible,
                    )? {
                        Ok((map, remaining)) => {
                            homes = remaining;
                            homes.push(binding);
                            locals.install_map(binding);
                            maps.push(map_flow::MapHomeObservation::Complete(map));
                        }
                        Err(issue) => {
                            unavailable = Some(issue);
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
            &mut locals,
            &mut homes,
            &mut maps,
            map_compatible,
        )?;
    }
    // Statements after the terminal are never walked; their sealed map
    // literals still owe loop1 one row each — issue Unavailable rows.
    map_descendant_flow::issue_unobserved_descendants(input, &mut maps);
    for site in selected.keys() {
        results.entry(site.clone()).or_insert_with(|| {
            Err(unavailable
                .clone()
                .unwrap_or(HomePrefixUnavailableV1::SourceMismatch))
        });
    }
    Ok((
        results,
        RootHomeFlow {
            terminal: terminal_homes,
            maps,
            local_calls,
        },
        terminal_relation,
        argument_observations,
    ))
}
