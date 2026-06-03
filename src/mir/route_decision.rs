/*!
 * Planner-owned route decisions.
 *
 * RouteDecisionV0 is a report-only outcome view: planners may prefer fast
 * routes, but MIRBuilder must only preserve origins/spans/types and lowering
 * must consume the selected route instead of re-deciding policy.
 */

use crate::mir::direct_array_access_plan::{DirectArrayAccessOp, DirectArrayAccessPlan};
use crate::mir::direct_exact_hotcore_call_plan::DirectExactHotCoreCallPlan;
use crate::mir::{BasicBlockId, MirFunction, MirModule};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RouteDecision {
    pub site_id: String,
    pub block: BasicBlockId,
    pub instruction_index: usize,
    pub semantic_op: &'static str,
    pub access_kind: &'static str,
    pub preferred_route: &'static str,
    pub selected_route: &'static str,
    pub fallback_route: &'static str,
    pub fallback_policy: &'static str,
    pub proof_ids: Vec<&'static str>,
    pub miss_reason: Option<&'static str>,
    pub source_plan_kind: &'static str,
}

impl RouteDecision {
    fn from_direct_array_access_plan(
        plan: &DirectArrayAccessPlan,
        fallback_policy: &'static str,
    ) -> Self {
        let (semantic_op, access_kind, fallback_route) = match plan.op() {
            DirectArrayAccessOp::Load => (
                "ArrayGet",
                "direct_array_i64_load",
                "generic_array_get_helper",
            ),
            DirectArrayAccessOp::Store => (
                "ArraySet",
                "direct_array_i64_store",
                "generic_array_set_helper",
            ),
        };
        Self {
            site_id: format!("b{}.i{}", plan.block().as_u32(), plan.instruction_index()),
            block: plan.block(),
            instruction_index: plan.instruction_index(),
            semantic_op,
            access_kind,
            preferred_route: plan.route(),
            selected_route: plan.route(),
            fallback_route,
            fallback_policy,
            proof_ids: plan.proof_ids().to_vec(),
            miss_reason: None,
            source_plan_kind: "DirectArrayAccessPlan",
        }
    }

    fn from_direct_exact_hotcore_call_plan(plan: &DirectExactHotCoreCallPlan) -> Self {
        let selected_route = if plan.lowering_consumer_enabled {
            "static_exact_call"
        } else {
            "generic_method_dispatch"
        };
        let proof_ids = if plan.lowering_consumer_enabled {
            vec!["same_module", "static_exact", "scalar_return"]
        } else {
            Vec::new()
        };
        Self {
            site_id: format!("b{}.i{}", plan.block.as_u32(), plan.instruction_index),
            block: plan.block,
            instruction_index: plan.instruction_index,
            semantic_op: "MethodCall",
            access_kind: "hotcore_call",
            preferred_route: "static_exact_call",
            selected_route,
            fallback_route: "generic_method_dispatch",
            fallback_policy: "opportunistic",
            proof_ids,
            miss_reason: plan.failure_reason,
            source_plan_kind: "DirectExactHotCoreCallPlan",
        }
    }
}

pub fn refresh_function_route_decisions(function: &mut MirFunction) {
    let direct_memory_required = function
        .metadata
        .required_fastpath_regions
        .iter()
        .any(|region| {
            region.relevant_access_policy == "direct_memory"
                && region.route_requirement == "fastpath_plan_required"
                && region.fallback_policy == "fail_fast"
        });
    let fallback_policy = if direct_memory_required {
        "require_fastpath"
    } else {
        "opportunistic"
    };

    function.metadata.route_decisions = function
        .metadata
        .direct_array_access_plans
        .iter()
        .map(|plan| RouteDecision::from_direct_array_access_plan(plan, fallback_policy))
        .collect();
}

pub fn refresh_module_hotcore_route_decisions(module: &mut MirModule) {
    for function in module.functions.values_mut() {
        function
            .metadata
            .route_decisions
            .retain(|decision| decision.source_plan_kind != "DirectExactHotCoreCallPlan");
        function.metadata.route_decisions.extend(
            function
                .metadata
                .direct_exact_hotcore_call_plans
                .iter()
                .map(RouteDecision::from_direct_exact_hotcore_call_plan),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
    use crate::mir::direct_array_access_plan::refresh_function_direct_array_access_plans;
    use crate::mir::function::RequiredFastPathRegion;
    use crate::mir::generic_method_route_plan::refresh_function_generic_method_routes;
    use crate::mir::{
        BasicBlock, Callee, EffectMask, FunctionSignature, MirFunction, MirInstruction, MirModule,
        MirType, ValueId,
    };

    fn method_call(
        dst: Option<u32>,
        box_name: &str,
        method: &str,
        receiver: u32,
        args: Vec<u32>,
    ) -> MirInstruction {
        MirInstruction::Call {
            dst: dst.map(ValueId::new),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: box_name.to_string(),
                method: method.to_string(),
                receiver: Some(ValueId::new(receiver)),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: args.into_iter().map(ValueId::new).collect(),
            effects: EffectMask::PURE,
        }
    }

    #[test]
    fn route_decision_reports_direct_array_selected_route() {
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "main".to_string(),
                params: vec![],
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        let mut block = BasicBlock::new(BasicBlockId::new(0));
        block.add_instruction(method_call(Some(5), "ArrayBox", "get", 2, vec![1]));
        block.set_terminator(MirInstruction::Return { value: None });
        function.add_block(block);

        refresh_function_generic_method_routes(&mut function);
        refresh_function_direct_array_access_plans(&mut function);
        refresh_function_route_decisions(&mut function);

        assert_eq!(function.metadata.route_decisions.len(), 1);
        let decision = &function.metadata.route_decisions[0];
        assert_eq!(decision.site_id, "b0.i0");
        assert_eq!(decision.semantic_op, "ArrayGet");
        assert_eq!(decision.preferred_route, "direct_array_i64_load");
        assert_eq!(decision.selected_route, "direct_array_i64_load");
        assert_eq!(decision.fallback_route, "generic_array_get_helper");
        assert_eq!(decision.fallback_policy, "opportunistic");
        assert_eq!(decision.miss_reason, None);
        assert_eq!(decision.source_plan_kind, "DirectArrayAccessPlan");
    }

    #[test]
    fn required_fastpath_region_marks_route_decision_required() {
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "main".to_string(),
                params: vec![],
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        function
            .metadata
            .required_fastpath_regions
            .push(RequiredFastPathRegion {
                region_id: 0,
                source_kind: "diagnostic_mode",
                relevant_access_policy: "direct_memory",
                route_requirement: "fastpath_plan_required",
                bounds_requirement: "checked_allowed",
                fallback_policy: "fail_fast",
            });
        let mut block = BasicBlock::new(BasicBlockId::new(0));
        block.add_instruction(method_call(Some(5), "ArrayBox", "get", 2, vec![1]));
        block.set_terminator(MirInstruction::Return { value: None });
        function.add_block(block);

        refresh_function_generic_method_routes(&mut function);
        refresh_function_direct_array_access_plans(&mut function);
        refresh_function_route_decisions(&mut function);

        assert_eq!(function.metadata.route_decisions.len(), 1);
        assert_eq!(
            function.metadata.route_decisions[0].fallback_policy,
            "require_fastpath"
        );
    }

    #[test]
    fn hotcore_call_plan_appends_route_decision() {
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "Main.runOne/2".to_string(),
                params: vec![],
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        function
            .metadata
            .direct_exact_hotcore_call_plans
            .push(DirectExactHotCoreCallPlan {
                block: BasicBlockId::new(3),
                instruction_index: 7,
                caller: "Main.runOne/2".to_string(),
                callee: "HakoAllocObjectLifecycleHotCore.objectLifecycleSmallAlloc/1".to_string(),
                box_name: "HakoAllocObjectLifecycleHotCore".to_string(),
                method: "objectLifecycleSmallAlloc".to_string(),
                receiver_value: ValueId::new(1),
                result_value: Some(ValueId::new(9)),
                receiver_exact: true,
                same_module: true,
                dispatch_policy: "static_exact",
                call_boundary_policy: "thin_direct_call_candidate",
                return_shape: Some("scalar_i64"),
                value_demand: "read_scalar",
                callee_summary_status: "ok",
                lowering_consumer_enabled: true,
                generic_method_dispatch: false,
                dynamic_route: false,
                boxed_fallback: false,
                summary: "ok",
                failure_reason: None,
            });
        let mut module = MirModule::new("route-decision-hotcore-test".to_string());
        module.add_function(function);

        refresh_module_hotcore_route_decisions(&mut module);

        let function = module.functions.get("Main.runOne/2").expect("function");
        assert_eq!(function.metadata.route_decisions.len(), 1);
        let decision = &function.metadata.route_decisions[0];
        assert_eq!(decision.site_id, "b3.i7");
        assert_eq!(decision.semantic_op, "MethodCall");
        assert_eq!(decision.access_kind, "hotcore_call");
        assert_eq!(decision.preferred_route, "static_exact_call");
        assert_eq!(decision.selected_route, "static_exact_call");
        assert_eq!(decision.fallback_route, "generic_method_dispatch");
        assert_eq!(decision.source_plan_kind, "DirectExactHotCoreCallPlan");
        assert_eq!(
            decision.proof_ids,
            vec!["same_module", "static_exact", "scalar_return"]
        );
    }
}
