use super::*;

pub(in crate::mir::builder) struct CallableGenericLoopSourceFactsIssuerV1;

impl CallableGenericLoopSourceFactsIssuerV1 {
    /// Issue exactly one source-aware planner outcome.  The raw Ready branch
    /// is the sole production caller; no old-route fallback is owned here.
    pub(in crate::mir::builder) fn issue_once<'source>(
        payload: PreparedCallableGenericLoopSourceFactsPayloadV1<'source>,
        variable_accum_recurrence: Option<
            crate::mir::loop_recipe_contract::VerifiedVariableAccumRecurrenceRecipeProductV1,
        >,
        retained: Option<PreissuedCallableLoopSourcePlanV1>,
    ) -> CallableGenericLoopSourceFactsDispositionV1<'source> {
        let PreparedCallableGenericLoopSourceFactsPayloadV1 {
            parent_source,
            condition_source,
            body_source,
            condition,
            body,
            owner,
            binding_product,
            function_name,
            debug,
            in_static_box: _,
            policy,
            function_origin,
            source_kind,
            source_projection,
            source_items,
            source_target_probe,
            call_free,
        } = payload;

        if let Err(error) = validate_source_input(
            parent_source,
            &condition_source,
            &body_source,
            owner,
            &binding_product,
        ) {
            return CallableGenericLoopSourceFactsDispositionV1::SourceUnavailable(error);
        }

        let planned = match retained {
            Some(retained) => parent_source.site().ok_or_else(|| "[freeze:contract][preissued-loop/parent-missing]".to_owned())
                .and_then(|site| retained.into_planned(owner, site, function_origin, source_kind, policy)),
            None => PlannedSourceLoopV1::issue(&condition, &body, policy, function_name, debug).map(|plan| (plan, None)),
        };
        let (PlannedSourceLoopV1 { outcome, selection }, source_recipe) = match planned {
            Ok(planned) => planned,
            Err(error) => return CallableGenericLoopSourceFactsDispositionV1::FactsRejected(error.into_boxed_str()),
        };
        // A retained verified source product cannot be dropped into another
        // family, absent selection or the unretained planner/accumulation path.
        if source_recipe.is_some() && (variable_accum_recurrence.is_some()
            || !matches!(selection.as_ref().and_then(|selection| selection.sole_family()),
                Some(CallableLoopSoleFamilyV1::LoopCondBreakContinue | CallableLoopSoleFamilyV1::LoopCondReturnInBody))) {
            return CallableGenericLoopSourceFactsDispositionV1::FactsRejected(
                "[freeze:contract][source-loop/product-route-drift]".into());
        }
        let Some(selection) = selection else {
            return match variable_accum_recurrence {
                Some(product) => CallableGenericLoopSourceFactsDispositionV1::VariableAccumRecurrenceReady(product),
                None => CallableGenericLoopSourceFactsDispositionV1::FactsAbsent,
            };
        };
        if let Some(product) = variable_accum_recurrence {
            if !selection.matched_routes().is_empty() {
                return CallableGenericLoopSourceFactsDispositionV1::RouteNotFrontSelected(
                    CallableGenericLoopSourceFactsRouteErrorV1::VariableAccumRecurrenceOverlap {
                        routes: selection.matched_routes().into(),
                    },
                );
            }
            return CallableGenericLoopSourceFactsDispositionV1::VariableAccumRecurrenceReady(
                product,
            );
        }
        if selection.sole_family() == Some(CallableLoopSoleFamilyV1::LoopCondBreakContinue)
            || selection.sole_family() == Some(CallableLoopSoleFamilyV1::LoopCondReturnInBody) && source_recipe.is_some() {
            return match loop_cond::issue(
                owner,
                parent_source,
                condition_source,
                body_source,
                condition,
                body,
                binding_product,
                function_origin,
                source_kind,
                outcome,
                selection,
                source_projection,
                source_items,
                source_target_probe,
                call_free,
                source_recipe,
            ) {
                Ok(source_facts) => {
                    CallableGenericLoopSourceFactsDispositionV1::LoopCondReady(source_facts)
                }
                Err(error) => {
                    CallableGenericLoopSourceFactsDispositionV1::RouteNotFrontSelected(error)
                }
            };
        }
        if selection.sole_family() == Some(CallableLoopSoleFamilyV1::LoopTrueBreakContinue) {
            return match loop_true::issue(
                owner,
                parent_source,
                condition_source,
                body_source,
                condition,
                body,
                binding_product,
                function_origin,
                source_kind,
                outcome,
                selection,
                source_projection,
                source_items,
                source_target_probe,
            ) {
                Ok(source_facts) => {
                    CallableGenericLoopSourceFactsDispositionV1::LoopTrueReady(source_facts)
                }
                Err(error) => {
                    CallableGenericLoopSourceFactsDispositionV1::RouteNotFrontSelected(error)
                }
            };
        }
        CallableGenericLoopSourceFactsDispositionV1::RouteNotFrontSelected(
            CallableGenericLoopSourceFactsRouteErrorV1::NonGenericOrOverlapping {
                routes: selection.matched_routes().into(),
            },
        )
    }
}

pub(in crate::mir::builder) fn issue_callable_variable_accum_recurrence(
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    expected_owner: FunctionOwnerIdV1,
    parent_source: &RawInvocationSourceContextV1,
) -> Result<
    Option<crate::mir::loop_recipe_contract::VerifiedVariableAccumRecurrenceRecipeProductV1>,
    CallableGenericLoopSourceFactsRouteErrorV1,
> {
    use crate::mir::loop_structural_facts::VariableAccumRecurrenceSourceAttemptOutcomeV1 as Outcome;
    use CallableGenericLoopSourceFactsRouteErrorV1 as Reject;

    let Some(site) = parent_source.site() else {
        return Err(Reject::VariableAccumRecurrenceSourceUnavailable(
            "parent-source-site-missing".into(),
        ));
    };
    let site = crate::mir::resolved_semantics::SourceStmtSiteV1::from_node(site.clone());
    if input.owner() != expected_owner {
        return Err(Reject::VariableAccumRecurrenceSourceUnavailable(
            "resolved-input-owner-mismatch".into(),
        ));
    }
    let ledger = input
        .forest()
        .callable_source_ledger(expected_owner)
        .map_err(|error| {
            Reject::VariableAccumRecurrenceSourceUnavailable(format!("{error:?}").into())
        })?;
    let membership = ledger.resolved_loop_source(&site).map_err(|error| {
        Reject::VariableAccumRecurrenceSourceUnavailable(format!("{error:?}").into())
    })?;
    let attempt = crate::mir::compiler::variable_accum_recurrence_projection::
        issue_variable_accum_recurrence_source_attempt_v1(
            input,
            &ledger,
            membership,
            crate::mir::loop_structural_facts::VariableAccumRecurrenceObservationCoverageV1::Complete,
        );
    let (outcome, identity, coverage) = attempt.into_parts();
    if identity.owner() != expected_owner
        || identity.site() != &site
        || identity.function_origin() != input.function().function_origin()
        || identity.source_kind() != input.function().source_kind()
    {
        return Err(Reject::VariableAccumRecurrenceSiteMismatch);
    }
    if coverage
        != crate::mir::loop_structural_facts::VariableAccumRecurrenceObservationCoverageV1::Complete
    {
        return Err(Reject::VariableAccumRecurrenceUnresolved(
            crate::mir::loop_structural_facts::VariableAccumRecurrenceSourceUnresolvedV1::IncompleteCoverage,
        ));
    }
    match outcome {
        Outcome::Candidate(facts) => {
            crate::mir::loop_recipe_contract::produce_variable_accum_recurrence_recipe_v1(facts)
                .map(Some)
                .map_err(|error| {
                    Reject::VariableAccumRecurrenceProducer(format!("{error:?}").into())
                })
        }
        Outcome::Declined(_) => Ok(None),
        Outcome::Unresolved(reason) => Err(Reject::VariableAccumRecurrenceUnresolved(reason)),
        Outcome::Rejected(reason) => Err(Reject::VariableAccumRecurrenceRejected(reason)),
    }
}
