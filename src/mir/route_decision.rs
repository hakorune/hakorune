/*!
 * Planner-owned route decisions.
 *
 * RouteDecisionV0 is a report-only outcome view: planners may prefer fast
 * routes, but MIRBuilder must only preserve origins/spans/types and lowering
 * must consume the selected route instead of re-deciding policy.
 */

use crate::mir::direct_array_access_plan::{DirectArrayAccessOp, DirectArrayAccessPlan};
use crate::mir::{BasicBlockId, MirFunction};

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
    fn from_direct_array_access_plan(plan: &DirectArrayAccessPlan) -> Self {
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
            fallback_policy: match plan.fallback_policy().as_str() {
                "fail_fast" => "require_fastpath",
                _ => "opportunistic",
            },
            proof_ids: plan.proof_ids().to_vec(),
            miss_reason: None,
            source_plan_kind: "DirectArrayAccessPlan",
        }
    }
}

pub fn refresh_function_route_decisions(function: &mut MirFunction) {
    function.metadata.route_decisions = function
        .metadata
        .direct_array_access_plans
        .iter()
        .map(RouteDecision::from_direct_array_access_plan)
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
    use crate::mir::direct_array_access_plan::refresh_function_direct_array_access_plans;
    use crate::mir::generic_method_route_plan::refresh_function_generic_method_routes;
    use crate::mir::{
        BasicBlock, Callee, EffectMask, FunctionSignature, MirFunction, MirInstruction, MirType,
        ValueId,
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
}
