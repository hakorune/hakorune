//! Disconnected same-call composer for the first located GenericLoopV1 slice.
//!
//! The composer consumes one O0 representation, performs the strict Parts
//! preflight before Builder effects, threads the same expression port through
//! body, cleanup, and condition lowering, then seals the completed CorePlan.
//! It has no production route or claim consumer.

use std::collections::BTreeMap;

use crate::mir::builder::control_flow::plan::features::{generic_loop_body, generic_loop_step};
use crate::mir::builder::control_flow::plan::generic_loop::located_representation::{
    PreparedLocatedGenericLoopDirectExecutionV1, VerifiedLocatedGenericLoopBodyRepresentationV1,
    VerifiedLocatedGenericLoopDirectPreflightV1, VerifiedLocatedGenericLoopLoweringViewV1,
};
use crate::mir::builder::control_flow::plan::parts::var_map_scope::with_saved_variable_map_typed;
use crate::mir::builder::control_flow::plan::parts::{
    prepare_located_generic_loop_parts_execution_v1, PreparedLocatedGenericLoopPartsBodyV1,
    PreparedLocatedGenericLoopPartsExecutionV1,
};
use crate::mir::builder::control_flow::plan::skeletons::generic_loop::alloc_generic_loop_v0_skeleton;
use crate::mir::builder::control_flow::plan::{
    CorePlan, LocatedLoopPlanExpressionPortV1, LoopPlanExpressionPortV1,
    VerifiedLocatedCoreLoopPlanV1,
};
use crate::mir::builder::{CanonicalSameModuleCallableKeyV1, MirBuilder};
use crate::mir::callable_result_representation::VerifiedCallableResultActivationPlanV1;
use crate::mir::resolved_semantics::ExprChildRoleV1;
use crate::mir::ValueId;

const LOCATED_GENERIC_LOOP_ERR: &str = "[located-generic-loop-v1]";

enum PreparedLocatedGenericLoopBodyExecutionV1<'seal, 'view, 'plan> {
    Direct(PreparedLocatedGenericLoopDirectBodyV1<'seal, 'view, 'plan>),
    ExitAllowed(PreparedLocatedGenericLoopPartsBodyV1<'seal, 'view, 'plan>),
}

struct PreparedLocatedGenericLoopDirectBodyV1<'seal, 'view, 'plan> {
    lowering: &'seal VerifiedLocatedGenericLoopLoweringViewV1<'view, 'plan>,
    _seal: DirectBodyExecutionSealV1,
}

struct DirectBodyExecutionSealV1;

impl<'seal, 'view, 'plan> PreparedLocatedGenericLoopBodyExecutionV1<'seal, 'view, 'plan> {
    fn lower_body(
        self,
        builder: &mut MirBuilder,
        current_bindings: &mut BTreeMap<String, ValueId>,
        carrier_step_phis: &BTreeMap<String, ValueId>,
        loop_var: &str,
    ) -> Result<Vec<crate::mir::builder::control_flow::plan::LoweredRecipe>, String> {
        match self {
            Self::Direct(body) => {
                let lowering = body.lowering;
                let crate::mir::builder::control_flow::plan::generic_loop::located_representation::VerifiedLocatedGenericLoopLoweringModeV1::DirectRecipeOnly { body } = lowering.mode()
                else {
                    return Err(format!(
                        "{LOCATED_GENERIC_LOOP_ERR}: direct execution mode drift"
                    ));
                };
                let port = body.expression_port();
                let statements = (0..body.len()).map(|index| {
                    body.statement(index).ok_or_else(|| {
                        format!(
                            "{LOCATED_GENERIC_LOOP_ERR}: direct prefix carrier missing: index={index}"
                        )
                    })
                });
                let mut reject = |_builder: &mut MirBuilder,
                                  _bindings: &mut BTreeMap<String, ValueId>,
                                  _port: &LocatedLoopPlanExpressionPortV1<'plan>,
                                  _statement| {
                    Err(format!(
                        "{LOCATED_GENERIC_LOOP_ERR}: unsupported DirectRecipeOnly statement"
                    ))
                };
                let statements = statements.collect::<Result<Vec<_>, _>>()?;
                generic_loop_body::lower_direct_statement_inputs(
                    builder,
                    current_bindings,
                    port,
                    statements,
                    carrier_step_phis,
                    loop_var,
                    LOCATED_GENERIC_LOOP_ERR,
                    &mut reject,
                    &|_| false,
                )
            }
            Self::ExitAllowed(body) => body.lower_body(
                builder,
                current_bindings,
                carrier_step_phis,
                &BTreeMap::new(),
                LOCATED_GENERIC_LOOP_ERR,
            ),
        }
    }
}

enum PreparedLocatedGenericLoopExecutionV1<'seal, 'view, 'plan> {
    Direct(PreparedLocatedGenericLoopDirectExecutionV1<'seal, 'view, 'plan>),
    ExitAllowed(PreparedLocatedGenericLoopPartsExecutionV1<'seal, 'view, 'plan>),
}

impl<'seal, 'view, 'plan> PreparedLocatedGenericLoopExecutionV1<'seal, 'view, 'plan> {
    fn prepare(
        lowering: &'seal VerifiedLocatedGenericLoopLoweringViewV1<'view, 'plan>,
    ) -> Result<Self, String> {
        match lowering.mode() {
            crate::mir::builder::control_flow::plan::generic_loop::located_representation::VerifiedLocatedGenericLoopLoweringModeV1::DirectRecipeOnly { .. } => {
                let preflight = VerifiedLocatedGenericLoopDirectPreflightV1::verify(lowering)
                    .map_err(|error| format!("{LOCATED_GENERIC_LOOP_ERR}: direct preflight failed: {error:?}"))?;
                Ok(Self::Direct(preflight.into_execution()))
            }
            crate::mir::builder::control_flow::plan::generic_loop::located_representation::VerifiedLocatedGenericLoopLoweringModeV1::ExitAllowedRecipe { .. } => {
                Ok(Self::ExitAllowed(prepare_located_generic_loop_parts_execution_v1(lowering)?))
            }
        }
    }

    fn into_parts(
        self,
    ) -> (
        Box<[String]>,
        PreparedLocatedGenericLoopBodyExecutionV1<'seal, 'view, 'plan>,
    ) {
        match self {
            Self::Direct(token) => {
                let (lowering, targets) = token.into_components();
                (
                    targets,
                    PreparedLocatedGenericLoopBodyExecutionV1::Direct(
                        PreparedLocatedGenericLoopDirectBodyV1 {
                            lowering,
                            _seal: DirectBodyExecutionSealV1,
                        },
                    ),
                )
            }
            Self::ExitAllowed(token) => {
                let (targets, body) = token.into_parts();
                (
                    targets,
                    PreparedLocatedGenericLoopBodyExecutionV1::ExitAllowed(body),
                )
            }
        }
    }
}

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
        let execution = PreparedLocatedGenericLoopExecutionV1::prepare(&lowering)?;
        let (carrier_targets, body_execution) = execution.into_parts();

        with_saved_variable_map_typed(builder, |builder| {
            let pre_body_map = builder.function_state.variable_ctx.variable_map.clone();
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
                                .function_state
                                .variable_ctx
                                .variable_map
                                .insert(name.clone(), *value);
                        }
                        let mut body = body_execution.lower_body(
                            builder,
                            &mut current_bindings,
                            carrier_step_phis,
                            &loop_var,
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

            builder.function_state.variable_ctx.variable_map = pre_body_map;
            generic_loop_step::apply_generic_loop_condition_input(
                builder,
                &mut skeleton,
                port,
                condition,
                &loop_var,
                LOCATED_GENERIC_LOOP_ERR,
            )?;
            builder.function_state.variable_ctx.variable_map =
                carrier_orchestration.post_body_map().clone();
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
