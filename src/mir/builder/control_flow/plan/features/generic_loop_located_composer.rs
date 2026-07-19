//! Disconnected same-call composer for the first located GenericLoopV1 slice.
//!
//! The composer consumes one O0 representation, performs the strict Parts
//! preflight before Builder effects, threads the same expression port through
//! body, cleanup, and condition lowering, then seals the completed CorePlan.
//! It has no production route or claim consumer.

use std::collections::BTreeMap;

use crate::mir::builder::control_flow::plan::features::{generic_loop_body, generic_loop_step};
use crate::mir::builder::control_flow::plan::generic_loop::located_representation::VerifiedLocatedGenericLoopBodyRepresentationV1;
use crate::mir::builder::control_flow::plan::parts::prepare_located_generic_loop_parts_execution_v1;
use crate::mir::builder::control_flow::plan::parts::var_map_scope::with_saved_variable_map_typed;
use crate::mir::builder::control_flow::plan::skeletons::generic_loop::alloc_generic_loop_v0_skeleton;
use crate::mir::builder::control_flow::plan::{
    CorePlan, LocatedLoopPlanExpressionPortV1, LoopPlanExpressionPortV1,
    VerifiedLocatedCoreLoopPlanV1,
};
use crate::mir::builder::{CanonicalSameModuleCallableKeyV1, MirBuilder};
use crate::mir::callable_result_representation::VerifiedCallableResultActivationPlanV1;
use crate::mir::resolved_semantics::ExprChildRoleV1;

const LOCATED_GENERIC_LOOP_ERR: &str = "[located-generic-loop-v1]";

pub(in crate::mir::builder) fn compose_located_generic_loop_v1<'plan>(
    builder: &mut MirBuilder,
    representation: VerifiedLocatedGenericLoopBodyRepresentationV1<'plan>,
    port: &LocatedLoopPlanExpressionPortV1<'plan>,
    activation_plan: &'plan VerifiedCallableResultActivationPlanV1,
    caller: &CanonicalSameModuleCallableKeyV1,
) -> Result<VerifiedLocatedCoreLoopPlanV1<'plan>, String> {
    let plan = {
        let lowering = representation
            .bind_lowering_port(port)
            .map_err(|error| format!("{LOCATED_GENERIC_LOOP_ERR}: bind failed: {error:?}"))?;
        let condition = lowering.condition();
        let cleanup = lowering.cleanup();
        let cleanup_value = port
            .child_expr_from_stmt(&cleanup, ExprChildRoleV1::AssignmentValue)
            .map_err(|error| error.render())?;
        let loop_var = lowering.loop_var().to_string();
        let carrier_role = lowering.carrier_role();
        let execution = prepare_located_generic_loop_parts_execution_v1(&lowering)?;
        let (carrier_targets, parts_body) = execution.into_parts();

        with_saved_variable_map_typed(builder, |builder| {
            let pre_body_map = builder.variable_ctx.variable_map.clone();
            let mut skeleton = alloc_generic_loop_v0_skeleton(builder, &loop_var, carrier_role)?;
            let mut carrier_orchestration =
                generic_loop_body::orchestrate_generic_loop_v1_carriers_from_targets(
                    builder,
                    &loop_var,
                    &carrier_targets,
                    skeleton.loop_var_current,
                    &skeleton.carrier_representation,
                    |builder, phi_bindings, carrier_step_phis| {
                        let mut current_bindings = phi_bindings.clone();
                        for (name, value) in phi_bindings {
                            builder
                                .variable_ctx
                                .variable_map
                                .insert(name.clone(), *value);
                        }
                        let mut body = parts_body.lower_body(
                            builder,
                            &mut current_bindings,
                            carrier_step_phis,
                            &BTreeMap::new(),
                            LOCATED_GENERIC_LOOP_ERR,
                        )?;
                        generic_loop_body::apply_generic_loop_v1_fallthrough_cleanup_input(
                            builder,
                            &mut body,
                            carrier_step_phis,
                            &current_bindings,
                            &loop_var,
                            port,
                            cleanup_value,
                            LOCATED_GENERIC_LOOP_ERR,
                        )?;
                        Ok(body)
                    },
                )?;
            skeleton.plan.body = carrier_orchestration.take_body_plans();

            builder.variable_ctx.variable_map = pre_body_map;
            generic_loop_step::apply_generic_loop_condition_input(
                builder,
                &mut skeleton,
                port,
                condition,
                &loop_var,
                LOCATED_GENERIC_LOOP_ERR,
            )?;
            builder.variable_ctx.variable_map = carrier_orchestration.post_body_map().clone();
            carrier_orchestration.finalize(
                builder,
                &mut skeleton.plan,
                &loop_var,
                skeleton.loop_var_init,
                skeleton.loop_var_current,
            );
            Ok::<CorePlan, String>(CorePlan::Loop(skeleton.plan))
        })?
    };

    let loop_statement = representation.into_loop_statement();
    VerifiedLocatedCoreLoopPlanV1::verify(plan, activation_plan, caller, loop_statement)
        .map_err(|error| format!("{LOCATED_GENERIC_LOOP_ERR}: final seal failed: {error:?}"))
}

#[cfg(test)]
#[path = "generic_loop_located_composer_tests.rs"]
mod tests;
