//! Selected local `new` source, argument and Normal/Fault Home accounting.
//! This is the existing scanner branch, kept separate from ordinary locals.
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1;

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
    homes: &mut HomesV1,
    covered_statements: &[SourceStmtSiteV1],
    selected_seen: &mut Vec<OwnedExprSiteV1>,
    unavailable: &mut Option<HomePrefixUnavailableV1>,
    scalar_field: &mut impl HomeFieldSourceConsultV1<E>,
    calls: &[LocalCallObservationV1],
    actuals: &mut impl FnMut(
        &OwnedExprSiteV1,
        BorrowedCallActualRequestV1<'_>,
    ) -> Result<Option<BorrowedCallArgumentsV1>, E>,
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
        Ok(new) => match new.node() {
            ASTNode::New {
                arguments,
                field_initializers,
                ..
            } => {
                let (observation, moved, issue) = observe_new_arguments_v1(
                    input,
                    owned,
                    arguments.len(),
                    locals,
                    homes,
                    calls,
                    argument_i64_field,
                    actuals,
                )?;
                argument_observations.insert(owned.clone(), observation);
                moved_arguments = moved;
                if let Some(issue) = issue {
                    unavailable.get_or_insert(issue);
                }
                if !field_initializers.is_empty() {
                    unavailable.get_or_insert_with(|| {
                        HomePrefixUnavailableV1::OverridesNotCovered(site.clone())
                    });
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
    let result = match &*unavailable {
        Some(issue) => Err(issue.clone()),
        None => issue_new_fault_continuation_v1(input, &owned)
            .map_err(|_| HomePrefixUnavailableV1::SourceMismatch)
            .map(|outward_fault| {
                CallerNewHomePrefixV1::from_transition(binding, homes, &moved_arguments, outward_fault, covered_statements)
            }),
    };
    results.insert(owned.clone(), result);
    selected_seen.push(owned.clone());
    // Argument moves complete on the invoke's Normal edge —
    // `prior_homes` above kept each binding so a Fault unwind
    // still discharges it.
    if crate::config::env::builder_debug_enabled() {
        eprintln!("[new-local] site={:?} moved={:?} homes_before={:?}", owned, moved_arguments, homes);
    }
    locals.release_to_callee(homes, &moved_arguments);
    // This is the Normal successor only, after exact local commit.
    let construction_source = scalar_field.construction_source(owned, binding)?;
    locals.acquire_selected_home(homes, binding, owned.clone(), construction_source);
    Ok(())
}

/// One source-order observation shared by local and return-New. Home transfer
/// is derived from those same rows; qualified Text expressions borrow inputs.
#[allow(clippy::too_many_arguments)]
pub(super) fn observe_new_arguments_v1<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    new_site: &OwnedExprSiteV1,
    arity: usize,
    locals: &PrefixLocalFlow<'_>,
    homes: &[LiveHomeV1],
    calls: &[LocalCallObservationV1],
    argument_i64_field: &mut impl FnMut(
        &OwnedExprSiteV1,
        &SourceExprSiteV1,
        BindingRefV1,
        BindingRefV1,
        &str,
    ) -> Result<bool, E>,
    actuals: &mut impl FnMut(
        &OwnedExprSiteV1,
        BorrowedCallActualRequestV1<'_>,
    ) -> Result<Option<BorrowedCallArgumentsV1>, E>,
) -> Result<
    (
        SelectedNewArgumentObservationV1,
        Vec<BindingRefV1>,
        Option<HomePrefixUnavailableV1>,
    ),
    E,
> {
    let source = input
        .function()
        .expression_source()
        .construction(new_site.site());
    let Some(source) = source.filter(|source| {
        input.owner() == new_site.owner()
            && source.site() == new_site.site()
            && source.arguments().len() == arity
    }) else {
        return Ok((
            SelectedNewArgumentObservationV1::new(
                new_site.clone(),
                Err(SelectedNewArgumentUnavailableV1::SourceMismatch {
                    new_site: new_site.clone(),
                }),
            ),
            Vec::new(),
            Some(HomePrefixUnavailableV1::SourceMismatch),
        ));
    };
    let mut rows = Vec::with_capacity(arity);
    let mut moved = Vec::new();
    let mut transfer_sites = Vec::new();
    let mut error = None;
    let mut unavailable = None;
    for (index, site) in source.arguments().iter().enumerate() {
        let Ok(ordinal) = u32::try_from(index) else {
            error.get_or_insert(SelectedNewArgumentUnavailableV1::ArgumentOrdinalOverflow {
                new_site: new_site.clone(),
            });
            unavailable.get_or_insert(HomePrefixUnavailableV1::SourceMismatch);
            break;
        };
        let expected =
            crate::mir::resolved_semantics::SourcePathV1::from_node(new_site.site().node())
                .child(crate::mir::resolved_semantics::SourcePathSegmentV1::Argument(ordinal))
                .expr();
        if site != &expected {
            error.get_or_insert(SelectedNewArgumentUnavailableV1::SourceMismatch {
                new_site: new_site.clone(),
            });
            unavailable.get_or_insert(HomePrefixUnavailableV1::SourceMismatch);
            continue;
        }
        let explicit_move = input.function().expression_source().moved(site).is_some();
        let observed = locals.observe_selected_argument(site, argument_i64_field)?;
        // Already-proved scalar leaves keep their exact old domain. Opaque
        // bindings/otherwise uncovered Text need their own original evaluation.
        let needs_expression = matches!(
            &observed,
            None | Some(
                SelectedNewArgumentKindV1::Handle { .. }
                    | SelectedNewArgumentKindV1::BoundValue { .. }
            )
        );
        let mut declined = None;
        let evaluation = if needs_expression {
            match crate::mir::resolved_value_profile::expression_plan::new_argument_expression_source_plan_v1(input, new_site, ordinal) {
                Ok(plan) => match super::expression_plan::qualify_text_evaluation_v1(input, plan, new_site, locals, homes, calls, actuals, None)? {
                    Ok(evaluation) => Some(evaluation),
                    Err(reason) => { declined = Some(reason); None }
                },
                Err(_) => {
                    error.get_or_insert(SelectedNewArgumentUnavailableV1::SourceMismatch { new_site: new_site.clone() });
                    unavailable.get_or_insert(HomePrefixUnavailableV1::SourceMismatch);
                    continue;
                }
            }
        } else {
            None
        };
        let kind = evaluation
            .map(|evaluation| SelectedNewArgumentKindV1::Expression { evaluation })
            .or(observed);
        match kind {
            Some(kind) => {
                match &kind {
                    SelectedNewArgumentKindV1::Local { binding }
                    | SelectedNewArgumentKindV1::Handle { binding }
                    | SelectedNewArgumentKindV1::BoundValue { binding }
                        if homes.iter().any(|home| home.binding() == *binding) =>
                    {
                        moved.push(*binding);
                        // Only a local name or an explicit `move` names its own
                        // Home; other expressions merely bind a value to one.
                        if explicit_move || input.function().variable_ref(site).is_some() {
                            transfer_sites.push(site.clone());
                        }
                    }
                    _ => {}
                }
                rows.push(SelectedNewArgumentV1::new(ordinal, site.clone(), kind).with_explicit_move(explicit_move));
            }
            None => {
                error.get_or_insert(SelectedNewArgumentUnavailableV1::ArgumentNotTrivial {
                    new_site: new_site.clone(),
                    site: site.clone(),
                    reason: declined,
                });
                unavailable.get_or_insert_with(|| {
                    HomePrefixUnavailableV1::ArgumentNotCovered(site.clone())
                });
            }
        }
    }
    // Same source-order use-after-move walk as a method-call entry transfer;
    // every argument whose Home leaves the frame is a transfer site.
    if !transfer_sites.is_empty() || locals.has_moved_out() {
        if let Err(issue) = locals.validate_pending_call_uses_v1(new_site, &transfer_sites) {
            unavailable.get_or_insert(issue);
        }
    }
    Ok((
        SelectedNewArgumentObservationV1::new(
            new_site.clone(),
            match error {
                Some(error) => Err(error),
                None => Ok(rows.into_boxed_slice()),
            },
        ),
        moved,
        unavailable,
    ))
}
