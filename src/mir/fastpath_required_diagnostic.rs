/*!
 * Required FastPath diagnostic contract.
 *
 * This is the source-syntax-free v0 of future `direct {}` semantics.  A
 * `RequiredFastPathRegion` asks the compiler to explain whether relevant
 * memory-like access sites have a MIR-owned FastPathPlan.  It does not imply
 * unsafe memory and it does not require unchecked bounds.
 */

use crate::mir::function::{FastPathObligation, RequiredFastPathRegion};
use crate::mir::MirFunction;

pub fn refresh_function_fastpath_obligations(function: &mut MirFunction) {
    let mut obligations = Vec::new();
    let mut obligation_id = 0u32;

    for region in &function.metadata.required_fastpath_regions {
        for route in &function.metadata.generic_method_routes {
            let Some((access_kind, op)) = relevant_access_kind(function, route) else {
                continue;
            };
            obligations.push(build_obligation(
                function,
                region,
                obligation_id,
                route.block(),
                route.instruction_index(),
                access_kind,
                op,
            ));
            obligation_id += 1;
        }
    }

    function.metadata.fastpath_obligations = obligations;
}

fn relevant_access_kind(
    function: &MirFunction,
    route: &crate::mir::generic_method_route_plan::GenericMethodRoute,
) -> Option<(&'static str, &'static str)> {
    let op = match route.route_kind_tag() {
        "array_slot_load_any" => "load",
        "array_store_any" => "store",
        _ => return None,
    };
    if span_plan_for_site(function, route.block(), route.instruction_index()).is_some() {
        return Some(match op {
            "load" => ("span_i64", op),
            _ => ("span_mut_i64", op),
        });
    }
    if route.receiver_origin_box() == Some("ArrayBox") {
        return Some(("direct_array_i64", op));
    }
    None
}

fn build_obligation(
    function: &MirFunction,
    region: &RequiredFastPathRegion,
    obligation_id: u32,
    block: crate::mir::BasicBlockId,
    instruction_index: usize,
    access_kind: &'static str,
    op: &'static str,
) -> FastPathObligation {
    if let Some(plan) = direct_array_plan_for_site(function, block, instruction_index) {
        return FastPathObligation {
            obligation_id,
            region_id: region.region_id,
            block,
            instruction_index,
            access_kind,
            op,
            expected: "FastPathPlanRequired",
            actual_plan_kind: Some("DirectArrayAccessPlan"),
            actual_route: Some(plan.route()),
            bounds_policy: Some(plan.bounds_policy().as_str()),
            proof_ids: plan.proof_ids().to_vec(),
            status: "passed",
            failure_code: None,
            failure_reason: None,
        };
    }

    if let Some(plan) = span_plan_for_site(function, block, instruction_index) {
        return FastPathObligation {
            obligation_id,
            region_id: region.region_id,
            block,
            instruction_index,
            access_kind,
            op,
            expected: "FastPathPlanRequired",
            actual_plan_kind: Some("SpanAccessPlan"),
            actual_route: Some(plan.route),
            bounds_policy: Some(plan.bounds_policy),
            proof_ids: plan.proof_ids.clone(),
            status: "passed",
            failure_code: None,
            failure_reason: None,
        };
    }

    FastPathObligation {
        obligation_id,
        region_id: region.region_id,
        block,
        instruction_index,
        access_kind,
        op,
        expected: "FastPathPlanRequired",
        actual_plan_kind: None,
        actual_route: None,
        bounds_policy: None,
        proof_ids: Vec::new(),
        status: "failed",
        failure_code: Some("DM006001"),
        failure_reason: Some("missing_fastpath_plan"),
    }
}

fn direct_array_plan_for_site(
    function: &MirFunction,
    block: crate::mir::BasicBlockId,
    instruction_index: usize,
) -> Option<&crate::mir::direct_array_access_plan::DirectArrayAccessPlan> {
    function
        .metadata
        .direct_array_access_plans
        .iter()
        .find(|plan| plan.block() == block && plan.instruction_index() == instruction_index)
}

fn span_plan_for_site(
    function: &MirFunction,
    block: crate::mir::BasicBlockId,
    instruction_index: usize,
) -> Option<&crate::mir::function::SpanAccessPlan> {
    function
        .metadata
        .span_access_plans
        .iter()
        .find(|plan| plan.block == block && plan.instruction_index == instruction_index)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
    use crate::mir::direct_array_access_plan::refresh_function_direct_array_access_plans;
    use crate::mir::generic_method_route_facts::GenericMethodValueDemand;
    use crate::mir::generic_method_route_plan::{
        GenericMethodRoute, GenericMethodRouteDecision, GenericMethodRouteEvidence,
        GenericMethodRouteKind, GenericMethodRouteOperands, GenericMethodRouteProof,
        GenericMethodRouteSite, GenericMethodRouteSurface,
    };
    use crate::mir::{
        BasicBlockId, Callee, EffectMask, FunctionSignature, MirFunction, MirInstruction, MirType,
    };

    fn make_function() -> MirFunction {
        MirFunction::new(
            FunctionSignature {
                name: "main".to_string(),
                params: vec![],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        )
    }

    fn region() -> RequiredFastPathRegion {
        RequiredFastPathRegion {
            region_id: 0,
            source_kind: "diagnostic_mode",
            relevant_access_policy: "direct_memory",
            route_requirement: "fastpath_plan_required",
            bounds_requirement: "checked_allowed",
            fallback_policy: "fail_fast",
        }
    }

    fn array_set_route(
        block: u32,
        instruction_index: usize,
        receiver: u32,
        key: u32,
        result: u32,
    ) -> GenericMethodRoute {
        GenericMethodRoute::new(
            GenericMethodRouteSite::new(BasicBlockId::new(block), instruction_index),
            GenericMethodRouteSurface::new("ArrayBox", "set", 2),
            GenericMethodRouteEvidence::new(Some("ArrayBox".to_string()), None),
            GenericMethodRouteOperands::new(
                crate::mir::ValueId::new(receiver),
                Some(crate::mir::ValueId::new(key)),
                Some(crate::mir::ValueId::new(result)),
            ),
            GenericMethodRouteDecision::new(
                GenericMethodRouteKind::ArrayStoreAny,
                GenericMethodRouteProof::SetSurfacePolicy,
                None,
                None,
                GenericMethodValueDemand::WriteAny,
                None,
            ),
        )
    }

    fn array_set_call() -> MirInstruction {
        MirInstruction::Call {
            dst: Some(crate::mir::ValueId::new(30)),
            func: crate::mir::ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "ArrayBox".to_string(),
                method: "set".to_string(),
                receiver: Some(crate::mir::ValueId::new(1)),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![
                crate::mir::ValueId::new(1),
                crate::mir::ValueId::new(2),
                crate::mir::ValueId::new(3),
            ],
            effects: EffectMask::PURE,
        }
    }

    #[test]
    fn refresh_records_failed_obligation_when_plan_is_missing() {
        let mut function = make_function();
        function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry")
            .instructions
            .push(array_set_call());
        function
            .metadata
            .generic_method_routes
            .push(array_set_route(0, 0, 1, 2, 30));
        function.metadata.required_fastpath_regions.push(region());

        refresh_function_fastpath_obligations(&mut function);

        assert_eq!(function.metadata.fastpath_obligations.len(), 1);
        let obligation = &function.metadata.fastpath_obligations[0];
        assert_eq!(obligation.status, "failed");
        assert_eq!(obligation.failure_code, Some("DM006001"));
        assert_eq!(obligation.failure_reason, Some("missing_fastpath_plan"));
    }

    #[test]
    fn refresh_records_passed_obligation_when_direct_array_plan_exists() {
        let mut function = make_function();
        function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry")
            .instructions
            .push(array_set_call());
        function
            .metadata
            .generic_method_routes
            .push(array_set_route(0, 0, 1, 2, 30));
        function.metadata.required_fastpath_regions.push(region());
        refresh_function_direct_array_access_plans(&mut function);

        refresh_function_fastpath_obligations(&mut function);

        assert_eq!(function.metadata.fastpath_obligations.len(), 1);
        let obligation = &function.metadata.fastpath_obligations[0];
        assert_eq!(obligation.status, "passed");
        assert_eq!(obligation.actual_plan_kind, Some("DirectArrayAccessPlan"));
        assert_eq!(obligation.actual_route, Some("direct_array_i64_store"));
        assert_eq!(obligation.bounds_policy, Some("checked"));
        assert_eq!(obligation.failure_code, None);
    }
}
