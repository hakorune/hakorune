use super::*;

pub(in crate::mir::builder) struct CallableGenericLoopSourceFactsIssuerV1;

impl CallableGenericLoopSourceFactsIssuerV1 {
    /// Issue the retained checked loop source product.  The raw Ready branch
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
        let retained = retained.map(|retained| {
            parent_source.site().ok_or_else(|| "[freeze:contract][preissued-loop/parent-missing]".to_owned())
                .and_then(|site| retained.into_issuance(owner, site, function_origin, source_kind, policy))
        }).transpose();
        match retained {
            Ok(Some(Some(product))) => {
                return match loop_cond::issue(owner, parent_source, condition_source, body_source,
                    condition, body, binding_product, function_origin, source_kind,
                    source_projection, source_items, source_target_probe, call_free, product) {
                    Ok(source_facts) => CallableGenericLoopSourceFactsDispositionV1::LoopCondReady(source_facts),
                    Err(error) => CallableGenericLoopSourceFactsDispositionV1::RouteNotFrontSelected(error),
                };
            }
            Ok(_) => {}
            Err(error) => return CallableGenericLoopSourceFactsDispositionV1::FactsRejected(error.into_boxed_str()),
        }
        CallableGenericLoopSourceFactsDispositionV1::CheckedSourceRouteUnavailable
    }
}
