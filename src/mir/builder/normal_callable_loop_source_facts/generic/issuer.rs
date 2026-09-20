use super::*;

pub(in crate::mir::builder) struct CallableGenericLoopSourceFactsIssuerV1;

impl CallableGenericLoopSourceFactsIssuerV1 {
    /// Issue exactly one source-aware planner outcome.  The raw Ready branch
    /// is the sole production caller; no old-route fallback is owned here.
    pub(in crate::mir::builder) fn issue_once<'source>(
        payload: PreparedCallableGenericLoopSourceFactsPayloadV1<'source>,
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
            in_static_box,
            policy,
            function_origin,
            source_kind,
            source_projection,
            source_items,
            source_target_probe,
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

        let planner_input =
            CallableLoopFactsPlannerInputV1::new(&condition, &body, policy, function_name, debug);
        let outcome = match single_planner::try_build_source_outcome(planner_input) {
            Ok(outcome) => outcome,
            Err(error) => {
                return CallableGenericLoopSourceFactsDispositionV1::FactsRejected(
                    error.into_boxed_str(),
                )
            }
        };
        if outcome.facts.is_none() {
            return CallableGenericLoopSourceFactsDispositionV1::FactsAbsent;
        }

        let selection = select_recipe_first_routes(outcome.facts.as_ref());
        let selected = match selection.verify_located_generic_loop_v1() {
            Ok(selected) => selected,
            Err(error) => {
                if selection
                    .verify_located_loop_cond_break_continue_v1()
                    .is_ok()
                {
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
                    ) {
                        Ok(source_facts) => {
                            CallableGenericLoopSourceFactsDispositionV1::LoopCondReady(source_facts)
                        }
                        Err(error) => {
                            CallableGenericLoopSourceFactsDispositionV1::RouteNotFrontSelected(
                                error,
                            )
                        }
                    };
                }
                if selection
                    .verify_located_loop_true_break_continue_v1()
                    .is_ok()
                {
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
                            CallableGenericLoopSourceFactsDispositionV1::RouteNotFrontSelected(
                                error,
                            )
                        }
                    };
                }
                return CallableGenericLoopSourceFactsDispositionV1::RouteNotFrontSelected(
                    route_error(error),
                );
            }
        };

        CallableGenericLoopSourceFactsDispositionV1::Ready(CallableGenericLoopSourceFactsV1 {
            owner,
            parent_source,
            condition_source,
            body_source,
            condition,
            body,
            binding_product,
            policy,
            debug,
            in_static_box,
            outcome,
            selection,
            selected,
        })
    }
}
