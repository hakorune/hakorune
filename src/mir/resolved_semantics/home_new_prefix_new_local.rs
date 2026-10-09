//! Selected local `new` source, argument and Normal/Fault Home accounting.
//! This is the existing scanner branch, kept separate from ordinary locals.
use super::*;

#[allow(clippy::too_many_arguments)]
pub(super) fn observe_selected_new_local<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &SourceExprSiteV1,
    owned: &OwnedExprSiteV1,
    binding: BindingRefV1,
    destination: BindingRefV1,
    results: &mut BTreeMap<OwnedExprSiteV1, Result<CallerNewHomePrefixV1, HomePrefixUnavailableV1>>,
    argument_observations: &mut BTreeMap<OwnedExprSiteV1, SelectedNewArgumentObservationV1>,
    locals: &mut PrefixLocalFlow<'_>,
    homes: &mut Vec<BindingRefV1>,
    covered_statements: &[SourceStmtSiteV1],
    selected_seen: &mut Vec<OwnedExprSiteV1>,
    unavailable: &mut Option<HomePrefixUnavailableV1>,
    argument_i64_field: &mut impl FnMut(
        &OwnedExprSiteV1,
        &SourceExprSiteV1,
        BindingRefV1,
        BindingRefV1,
        &str,
    ) -> Result<bool, E>,
) -> Result<(), E> {
    if destination != binding {
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
                                row_error = Some(
                                    SelectedNewArgumentUnavailableV1::ArgumentOrdinalOverflow {
                                        new_site: owned.clone(),
                                    },
                                );
                                break;
                            };
                            let argument = match input
                                .source()
                                .child_expr_from_expr(&new, ExprChildRoleV1::CallArgument(ordinal))
                            {
                                Ok(argument) => argument,
                                Err(_) => {
                                    row_error =
                                        Some(SelectedNewArgumentUnavailableV1::SourceMismatch {
                                            new_site: owned.clone(),
                                        });
                                    break;
                                }
                            };
                            match locals
                                .observe_selected_argument(argument.site(), argument_i64_field)?
                            {
                                Some(kind) => rows.push(SelectedNewArgumentV1::new(
                                    ordinal,
                                    argument.site().clone(),
                                    kind,
                                )),
                                None => {
                                    row_error = Some(
                                        SelectedNewArgumentUnavailableV1::ArgumentNotTrivial {
                                            new_site: owned.clone(),
                                            site: argument.site().clone(),
                                        },
                                    );
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
                        SelectedNewArgumentObservationV1::new(owned.clone(), observed_arguments),
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
                                match locals
                                    .observe_selected_argument(arg.site(), argument_i64_field)?
                                {
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
                                unavailable.get_or_insert(HomePrefixUnavailableV1::SourceMismatch);
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
                covered_statements: covered_statements.to_vec().into_boxed_slice(),
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
    locals.install_selected_normal_home(binding, owned.clone());
    Ok(())
}
