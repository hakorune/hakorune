use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::generic_loop::facts_types::GenericLoopV1Facts;
use crate::mir::builder::control_flow::plan::{CoreLoopPlan, LoweredRecipe};
use crate::mir::builder::MirBuilder;
use crate::mir::ValueId;
use std::collections::BTreeMap;

use super::carriers::{
    finalize_generic_loop_v1_carriers, prepare_generic_loop_v1_carriers,
    GenericLoopV1CarrierState,
};
use super::{lower_generic_loop_v1_body, plans_require_continue_edge};

pub(in crate::mir::builder) struct GenericLoopV1CarrierOrchestration {
    body_plans: Vec<LoweredRecipe>,
    carrier_state: GenericLoopV1CarrierState,
    post_body_map: BTreeMap<String, ValueId>,
    body_has_continue_edge: bool,
}

impl GenericLoopV1CarrierOrchestration {
    pub(in crate::mir::builder) fn body_has_continue_edge(&self) -> bool {
        self.body_has_continue_edge
    }

    pub(in crate::mir::builder) fn post_body_map(&self) -> &BTreeMap<String, ValueId> {
        &self.post_body_map
    }

    pub(in crate::mir::builder) fn take_body_plans(&mut self) -> Vec<LoweredRecipe> {
        std::mem::take(&mut self.body_plans)
    }

    pub(in crate::mir::builder) fn loop_var_step_src(
        &self,
        loop_var: &str,
        fallback: ValueId,
    ) -> ValueId {
        self.post_body_map.get(loop_var).copied().unwrap_or(fallback)
    }

    pub(in crate::mir::builder) fn finalize(
        self,
        builder: &mut MirBuilder,
        loop_plan: &mut CoreLoopPlan,
        loop_var: &str,
        loop_var_init: ValueId,
        loop_var_current: ValueId,
    ) {
        finalize_generic_loop_v1_carriers(
            builder,
            loop_plan,
            self.carrier_state,
            loop_var,
            loop_var_init,
            loop_var_current,
            &self.post_body_map,
            self.body_has_continue_edge,
        );
    }
}

pub(in crate::mir::builder) fn orchestrate_generic_loop_v1_carriers(
    builder: &mut MirBuilder,
    facts: &GenericLoopV1Facts,
    loop_var_current: ValueId,
    ctx: &LoopRouteContext,
) -> Result<GenericLoopV1CarrierOrchestration, String> {
    let carrier_state = prepare_generic_loop_v1_carriers(builder, facts, loop_var_current);
    crate::mir::builder::control_flow::joinir::trace::trace()
        .varmap("generic_loop_v1_phi_bindings", &carrier_state.phi_bindings);
    let body_plans = lower_generic_loop_v1_body(
        builder,
        facts,
        &carrier_state.phi_bindings,
        &carrier_state.carrier_step_phis,
        ctx,
    )?;
    crate::mir::builder::control_flow::joinir::trace::trace().varmap(
        "generic_loop_v1_post_body",
        &builder.variable_ctx.variable_map,
    );
    let post_body_map = builder.variable_ctx.variable_map.clone();
    let body_has_continue_edge = plans_require_continue_edge(&body_plans);
    Ok(GenericLoopV1CarrierOrchestration {
        body_plans,
        carrier_state,
        post_body_map,
        body_has_continue_edge,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::ValueId;

    #[test]
    fn generic_loop_v1_carrier_orchestration_uses_post_body_step_src() {
        let orchestration = GenericLoopV1CarrierOrchestration {
            body_plans: Vec::new(),
            carrier_state: GenericLoopV1CarrierState {
                phi_bindings: BTreeMap::new(),
                carrier_step_phis: BTreeMap::new(),
                loop_var_step_phi: ValueId::new(0),
                carrier_infos: Vec::new(),
            },
            post_body_map: BTreeMap::from([("i".to_string(), ValueId::new(7))]),
            body_has_continue_edge: true,
        };

        assert_eq!(orchestration.loop_var_step_src("i", ValueId::new(3)), ValueId::new(7));
    }

    #[test]
    fn generic_loop_v1_carrier_orchestration_falls_back_to_current_value() {
        let orchestration = GenericLoopV1CarrierOrchestration {
            body_plans: Vec::new(),
            carrier_state: GenericLoopV1CarrierState {
                phi_bindings: BTreeMap::new(),
                carrier_step_phis: BTreeMap::new(),
                loop_var_step_phi: ValueId::new(0),
                carrier_infos: Vec::new(),
            },
            post_body_map: BTreeMap::new(),
            body_has_continue_edge: false,
        };

        assert_eq!(orchestration.loop_var_step_src("i", ValueId::new(3)), ValueId::new(3));
        assert!(!orchestration.body_has_continue_edge());
    }
}
