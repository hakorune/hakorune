use super::*;

pub(in crate::mir::builder) struct CallableGenericLoopSourceFactsIssuerV1;

impl CallableGenericLoopSourceFactsIssuerV1 {
    /// Issue exactly one source-aware planner outcome.  The raw Ready branch
    /// is the sole production caller; no old-route fallback is owned here.
    pub(in crate::mir::builder) fn issue_once<'source>(
        payload: PreparedCallableGenericLoopSourceFactsPayloadV1<'source>,
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
        let trace_name = std::env::var_os("NYASH_DEBUG_LOCAL_CALLS").is_some()
            .then(|| function_name.clone());

        let retained = retained.map(|retained| {
            parent_source.site().ok_or_else(|| "[freeze:contract][preissued-loop/parent-missing]".to_owned())
                .and_then(|site| retained.into_issuance(owner, site, function_origin, source_kind, policy))
        }).transpose();
        let (planned, source_recipe) = match retained {
            Ok(retained) => retained.unwrap_or((None, None)),
            Err(error) => return CallableGenericLoopSourceFactsDispositionV1::FactsRejected(error.into_boxed_str()),
        };
        if let Some(product) = source_recipe {
            if planned.is_some() {
                return CallableGenericLoopSourceFactsDispositionV1::FactsRejected(
                    "[freeze:contract][source-loop/product-route-drift]".into());
            }
            return match loop_cond::issue(owner, parent_source, condition_source, body_source,
                condition, body, binding_product, function_origin, source_kind, None,
                source_projection, source_items, source_target_probe, call_free, Some(product)) {
                Ok(source_facts) => CallableGenericLoopSourceFactsDispositionV1::LoopCondReady(source_facts),
                Err(error) => CallableGenericLoopSourceFactsDispositionV1::RouteNotFrontSelected(error),
            };
        }
        let planned = match planned {
            Some(planned) => planned,
            None => match PlannedSourceLoopV1::issue(&condition, &body, policy, function_name, debug) {
                Ok(planned) => planned,
                Err(error) => return CallableGenericLoopSourceFactsDispositionV1::FactsRejected(error.into_boxed_str()),
            },
        };
        let PlannedSourceLoopV1 { outcome, selection } = planned;
        if let Some(function_name) = trace_name {
            eprintln!("[local-call-bindings] loop-route owner={owner:?} function={function_name} family={:?}",
                selection.as_ref().and_then(|selection| selection.sole_family()));
        }
        let Some(selection) = selection else {
            return CallableGenericLoopSourceFactsDispositionV1::FactsAbsent;
        };
        if selection.sole_family() == Some(CallableLoopSoleFamilyV1::LoopCondBreakContinue) {
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
                Some((outcome, selection)),
                source_projection,
                source_items,
                source_target_probe,
                call_free,
                None,
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
            return CallableGenericLoopSourceFactsDispositionV1::FactsAbsent;
        }
        CallableGenericLoopSourceFactsDispositionV1::RouteNotFrontSelected(
            CallableGenericLoopSourceFactsRouteErrorV1::NonGenericOrOverlapping {
                routes: selection.matched_routes().into(),
            },
        )
    }
}
