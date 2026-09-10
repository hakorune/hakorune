//! Split from composer.rs (behavior-preserving module split).

use super::RecipeComposer;
use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::lower::normalize::CanonicalLoopFacts;
use crate::mir::builder::control_flow::plan::features::generic_loop_body;
use crate::mir::builder::control_flow::plan::features::generic_loop_context::GenericLoopV1LoweringContext;
use crate::mir::builder::control_flow::plan::features::generic_loop_pipeline;
use crate::mir::builder::control_flow::plan::parts::var_map_scope::with_saved_variable_map_typed;
use crate::mir::builder::control_flow::plan::planner::Freeze;
use crate::mir::builder::control_flow::plan::skeletons::generic_loop::alloc_generic_loop_v0_skeleton;
use crate::mir::builder::control_flow::plan::{CorePlan, LoopPlanExpressionPortV1, LoweredRecipe};
use crate::mir::builder::MirBuilder;
use crate::mir::policies::BodyLoweringPolicy;
use crate::mir::resolved_semantics::ExprChildRoleV1;

impl RecipeComposer {
    /// Compose generic_loop_v0 facts into LoweredRecipe without the normalizer.
    ///
    /// Used only in strict/dev + planner_required routing.
    pub fn compose_generic_loop_v0_recipe(
        builder: &mut MirBuilder,
        facts: &CanonicalLoopFacts,
        ctx: &LoopRouteContext,
    ) -> Result<LoweredRecipe, Freeze> {
        use crate::config::env::joinir_dev;

        let generic_loop_v0 = facts.facts.generic_loop_v0.as_ref().ok_or_else(|| {
            Freeze::contract("generic_loop_v0 facts missing in compose_generic_loop_v0_recipe")
        })?;

        if joinir_dev::debug_enabled() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0
                .log
                .debug("[recipe:compose] route=generic_loop_v0 path=direct_pipeline");
        }

        with_saved_variable_map_typed(builder, |builder| {
            let mut skeleton = alloc_generic_loop_v0_skeleton(
                builder,
                &generic_loop_v0.loop_var,
                generic_loop_v0.carrier_role,
            )
            .map_err(|e| Freeze::contract(&format!("generic_loop_v0 skeleton failed: {}", e)))?;

            generic_loop_pipeline::apply_generic_loop_v0_pipeline(
                builder,
                generic_loop_v0,
                ctx,
                &mut skeleton,
            )
            .map_err(|e| Freeze::contract(&format!("generic_loop_v0 pipeline failed: {}", e)))?;

            Ok(CorePlan::Loop(skeleton.plan))
        })
    }

    /// Compose generic_loop_v1 facts into LoweredRecipe without the normalizer.
    ///
    /// Used only in strict/dev + planner_required routing.
    pub fn compose_generic_loop_v1_recipe(
        builder: &mut MirBuilder,
        facts: &CanonicalLoopFacts,
        ctx: &LoopRouteContext,
    ) -> Result<LoweredRecipe, Freeze> {
        Self::compose_generic_loop_v1_recipe_with_context(builder, facts, ctx)
    }

    /// Compose a source-backed GenericLoopV1 Recipe through the route-neutral
    /// physical context.  The caller has already selected GenericLoopV1; this
    /// method never constructs or reclassifies a `LoopRouteContext`.
    pub(in crate::mir::builder) fn compose_source_generic_loop_v1_recipe(
        builder: &mut MirBuilder,
        facts: &CanonicalLoopFacts,
        ctx: &dyn GenericLoopV1LoweringContext,
    ) -> Result<LoweredRecipe, Freeze> {
        Self::compose_generic_loop_v1_recipe_with_context(builder, facts, ctx)
    }

    /// Compose the first source-aware GenericLoopV1 slice through the same
    /// carrier/skeleton owners as the raw route.  The port is the only source
    /// lookup capability; no name map or legacy expression normalizer is
    /// admitted on this path.
    #[allow(clippy::too_many_arguments)]
    pub(in crate::mir::builder) fn compose_source_generic_loop_v1_recipe_with_port<'input, P>(
        builder: &mut MirBuilder,
        facts: &CanonicalLoopFacts,
        port: &P,
        condition: P::ExprInput<'input>,
        body: P::BodyInput<'input>,
    ) -> Result<LoweredRecipe, Freeze>
    where
        P: LoopPlanExpressionPortV1 + 'input,
    {
        let generic_loop_v1 = facts.facts.generic_loop_v1.as_ref().ok_or_else(|| {
            Freeze::contract("generic_loop_v1 facts missing in source port composer")
        })?;
        if !matches!(
            generic_loop_v1.body_lowering_policy,
            BodyLoweringPolicy::RecipeOnly
        ) {
            return Err(Freeze::unsupported(
                "callable-loop source port requires RecipeOnly body",
            ));
        }

        let increment_index = generic_loop_v1
            .body
            .body
            .iter()
            .position(|statement| {
                source_loop_increment_matches(
                    statement,
                    &generic_loop_v1.loop_var,
                    &generic_loop_v1.loop_increment,
                )
            })
            .ok_or_else(|| Freeze::contract("callable-loop source increment site missing"))?;
        let increment_statement = port
            .body_stmt(&body, increment_index)
            .map_err(|error| Freeze::contract(&error.render()))?;
        let increment_target = port
            .child_expr_from_stmt(&increment_statement, ExprChildRoleV1::AssignmentTarget)
            .map_err(|error| Freeze::contract(&error.render()))?;
        let increment_input = port
            .child_expr_from_stmt(&increment_statement, ExprChildRoleV1::AssignmentValue)
            .map_err(|error| Freeze::contract(&error.render()))?;

        with_saved_variable_map_typed(builder, |builder| {
            let pre_body_map = builder.function_state.variable_ctx.variable_map.clone();
            let mut skeleton = alloc_generic_loop_v0_skeleton(
                builder,
                &generic_loop_v1.loop_var,
                generic_loop_v1.carrier_role,
            )
            .map_err(|error| {
                Freeze::contract(&format!("generic_loop_v1 skeleton failed: {error}"))
            })?;

            let carrier_state = generic_loop_body::prepare_generic_loop_v1_carriers(
                builder,
                generic_loop_v1,
                skeleton.loop_var_current,
                &skeleton.carrier_representation,
            );
            let mut carrier_orchestration =
                generic_loop_body::orchestrate_generic_loop_v1_carriers_with_body(
                    builder,
                    carrier_state,
                    |builder, phi_bindings, carrier_step_phis| {
                        let mut current_bindings = phi_bindings.clone();
                        for (name, value_id) in phi_bindings {
                            crate::mir::builder::control_flow::plan::parts::var_map_scope::publish_emission_cache(
                                builder,
                                name.clone(),
                                *value_id,
                            );
                        }
                        let mut reject = |_builder: &mut MirBuilder,
                                          _bindings: &mut std::collections::BTreeMap<
                                              String,
                                              crate::mir::ValueId,
                                          >,
                                          _port: &P,
                                          _statement: P::StmtInput<'input>| {
                            Err("[freeze:contract][callable-loop/source-unsupported-body-statement]".to_owned())
                        };
                        let mut body_plans = generic_loop_body::lower_direct_body_input_with_policy(
                            builder,
                            &mut current_bindings,
                            port,
                            body,
                            carrier_step_phis,
                            &generic_loop_v1.loop_var,
                            "[callable-loop/source-body]",
                            &mut reject,
                            &|statement| {
                                source_loop_increment_matches(
                                    statement,
                                    &generic_loop_v1.loop_var,
                                    &generic_loop_v1.loop_increment,
                                )
                            },
                        )?;

                        if !generic_loop_body::body_plans_exit_on_all_paths(&body_plans) {
                            let (next, effects) = crate::mir::builder::control_flow::plan::normalizer::PlanNormalizer::lower_value_input(
                                port,
                                increment_input,
                                builder,
                                &current_bindings,
                            )?;
                            body_plans.extend(
                                crate::mir::builder::control_flow::plan::steps::effects_to_plans(
                                    effects,
                                ),
                            );
                            port.exact_source_assignment_rebind(&increment_target, next)?;
                            let mut fallthrough_bindings = current_bindings.clone();
                            fallthrough_bindings.insert(generic_loop_v1.loop_var.clone(), next);
                            let exit = crate::mir::builder::control_flow::plan::parts::exit::build_continue_with_phi_args(
                                builder,
                                carrier_step_phis,
                                &fallthrough_bindings,
                                "[callable-loop/source-body]",
                            )?;
                            body_plans.push(CorePlan::Exit(exit));
                        }
                        Ok(body_plans)
                    },
                )
                .map_err(|error| Freeze::contract(&format!("callable-loop source body: {error}")))?;
            skeleton.plan.body = carrier_orchestration.take_body_plans();

            builder.function_state.variable_ctx.variable_map = pre_body_map;
            crate::mir::builder::control_flow::plan::features::generic_loop_step::apply_generic_loop_condition_input(
                builder,
                &mut skeleton,
                port,
                condition,
                &generic_loop_v1.loop_var,
                "[callable-loop/source-condition]",
            )
            .map_err(|error| Freeze::contract(&error))?;
            builder.function_state.variable_ctx.variable_map =
                carrier_orchestration.post_body_map().clone();
            carrier_orchestration.finalize(
                builder,
                &mut skeleton.plan,
                &generic_loop_v1.loop_var,
                skeleton.loop_var_init,
                skeleton.loop_var_current,
            );
            Ok(CorePlan::Loop(skeleton.plan))
        })
    }

    fn compose_generic_loop_v1_recipe_with_context(
        builder: &mut MirBuilder,
        facts: &CanonicalLoopFacts,
        ctx: &dyn GenericLoopV1LoweringContext,
    ) -> Result<LoweredRecipe, Freeze> {
        use crate::config::env::joinir_dev;

        let generic_loop_v1 = facts.facts.generic_loop_v1.as_ref().ok_or_else(|| {
            Freeze::contract("generic_loop_v1 facts missing in compose_generic_loop_v1_recipe")
        })?;

        if joinir_dev::debug_enabled() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0
                .log
                .debug("[recipe:compose] route=generic_loop_v1 path=direct_pipeline");
        }

        // ExitAllowed lowering needs its prebuilt recipe. RecipeOnly v1 shapes
        // may still lower through the v1 pipeline without an ExitAllowed recipe.
        if matches!(
            generic_loop_v1.body_lowering_policy,
            BodyLoweringPolicy::ExitAllowed { .. }
        ) && generic_loop_v1.body_exit_allowed.is_none()
        {
            return Err(Freeze::contract(
                "generic_loop_v1 ExitAllowed route requires body_exit_allowed",
            ));
        }

        with_saved_variable_map_typed(builder, |builder| {
            let mut skeleton = alloc_generic_loop_v0_skeleton(
                builder,
                &generic_loop_v1.loop_var,
                generic_loop_v1.carrier_role,
            )
            .map_err(|e| Freeze::contract(&format!("generic_loop_v1 skeleton failed: {}", e)))?;

            generic_loop_pipeline::apply_generic_loop_v1_pipeline(
                builder,
                generic_loop_v1,
                ctx,
                &mut skeleton,
            )
            .map_err(|e| Freeze::contract(&format!("generic_loop_v1 pipeline failed: {}", e)))?;

            Ok(CorePlan::Loop(skeleton.plan))
        })
    }
}

fn source_loop_increment_matches(
    statement: &crate::ast::ASTNode,
    loop_var: &str,
    increment: &crate::ast::ASTNode,
) -> bool {
    let crate::ast::ASTNode::Assignment { target, value, .. } = statement else {
        return false;
    };
    matches!(target.as_ref(), crate::ast::ASTNode::Variable { name, .. } if name == loop_var)
        && value.as_ref() == increment
}
