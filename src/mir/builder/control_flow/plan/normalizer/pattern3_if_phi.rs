use super::helpers::{create_phi_bindings, LoopBlocksWithIfPhi};
use super::{CoreEffectPlan, CoreLoopPlan, CorePlan, LoweredRecipe};
use crate::mir::builder::control_flow::plan::Pattern3IfPhiPlan;
use crate::mir::basic_block::EdgeArgs;
use crate::mir::builder::control_flow::plan::edgecfg_facade::{BlockParams, Frag};
use crate::mir::builder::control_flow::plan::features::edgecfg_stubs;
use crate::mir::builder::control_flow::plan::features::loop_carriers::build_loop_phi_info;
use crate::mir::builder::control_flow::plan::step_mode::extract_to_step_bb_explicit_step;
use crate::mir::builder::control_flow::joinir::patterns::router::LoopPatternContext;
use crate::mir::builder::MirBuilder;
use crate::mir::join_ir::lowering::inline_boundary::JumpArgsLayout;
use crate::mir::MirType;
use std::collections::BTreeMap;

impl super::PlanNormalizer {
    /// Phase 286 P2.6.1: Pattern3IfPhi → CorePlan 変換
    ///
    /// Expands Pattern3 (Loop with If-Phi) semantics into generic CorePlan:
    /// - CFG structure: preheader → header → body → then/else → merge → step → header
    /// - 2 PHIs: in header (loop_var, carrier)
    /// - merge join (carrier_next) is expressed via `Frag.block_params + EdgeArgs`
    /// - If-else branching with PHI merge (no Select instruction)
    pub(in crate::mir::builder) fn normalize_pattern3_if_phi(
        builder: &mut MirBuilder,
        parts: Pattern3IfPhiPlan,
        ctx: &LoopPatternContext,
    ) -> Result<LoweredRecipe, String> {
        use crate::mir::builder::control_flow::joinir::trace;

        let trace_logger = trace::trace();
        let debug = ctx.debug;
        let debug_enabled = crate::config::env::joinir_dev::debug_enabled();

        if debug {
            trace_logger.debug(
                "normalizer/pattern3_if_phi",
                &format!(
                    "Phase 286 P2.6.1: Normalizing Pattern3IfPhi for {} (loop_var: {}, carrier_var: {})",
                    ctx.func_name, parts.loop_var, parts.carrier_var
                ),
            );
        }

        let locals_count = builder.variable_ctx.variable_map.len();
        let ring0 = crate::runtime::get_global_ring0();
        let log_var_not_found = |name: &str| {
            if debug_enabled {
                ring0.log.debug(&format!(
                    "[plan/trace:if_phi_normalize] ctx=normalizer/pattern3_if_phi var={} locals_count={} scope={}",
                    name, locals_count, ctx.func_name
                ));
            }
        };

        // Step 1: Get host ValueIds for variables
        let loop_var_init = builder
            .variable_ctx
            .variable_map
            .get(&parts.loop_var)
            .copied()
            .ok_or_else(|| {
                log_var_not_found(&parts.loop_var);
                format!("[normalizer] Loop variable {} not found", parts.loop_var)
            })?;

        let carrier_init = builder
            .variable_ctx
            .variable_map
            .get(&parts.carrier_var)
            .copied()
            .ok_or_else(|| {
                log_var_not_found(&parts.carrier_var);
                format!("[normalizer] Carrier variable {} not found", parts.carrier_var)
            })?;

        // Step 2-3: Allocate BasicBlockIds
        let blocks = LoopBlocksWithIfPhi::allocate(builder)?;
        let LoopBlocksWithIfPhi {
            preheader_bb,
            header_bb,
            body_bb,
            then_bb,
            else_bb,
            merge_bb,
            step_bb,
            after_bb,
        } = blocks;

        if debug {
            trace_logger.debug(
                "normalizer/pattern3_if_phi",
                &format!(
                    "Allocated: preheader={:?}, header={:?}, body={:?}, then={:?}, else={:?}, merge={:?}, step={:?}, after={:?}",
                    preheader_bb,
                    header_bb,
                    body_bb,
                    then_bb,
                    else_bb,
                    merge_bb,
                    step_bb,
                    after_bb
                ),
            );
        }

        // Step 4: Allocate ValueIds for loop control
        let loop_var_current = builder.alloc_typed(MirType::Integer);
        let carrier_current = builder.alloc_typed(MirType::Integer);
        let cond_loop = builder.alloc_typed(MirType::Bool);
        let cond_if = builder.alloc_typed(MirType::Bool);
        let carrier_then = builder.alloc_typed(MirType::Integer);
        let carrier_else = builder.alloc_typed(MirType::Integer);
        let carrier_next = builder.alloc_typed(MirType::Integer);
        let loop_var_next = builder.alloc_typed(MirType::Integer);

        // Step 5: Build phi_bindings
        let phi_bindings = create_phi_bindings(&[
            (&parts.loop_var, loop_var_current),
            (&parts.carrier_var, carrier_current),
        ]);

        // Step 6: Lower AST expressions
        let (loop_cond_lhs, loop_cond_op, loop_cond_rhs, loop_cond_consts) =
            Self::lower_compare_ast(&parts.condition, builder, &phi_bindings).map_err(|e| {
                if let Some(var) = e
                    .strip_prefix("[normalizer] Variable ")
                    .and_then(|rest| rest.strip_suffix(" not found"))
                {
                    log_var_not_found(var);
                }
                e
            })?;

        let (if_cond_lhs, if_cond_op, if_cond_rhs, if_cond_consts) =
            Self::lower_compare_ast(&parts.if_condition, builder, &phi_bindings).map_err(|e| {
                if let Some(var) = e
                    .strip_prefix("[normalizer] Variable ")
                    .and_then(|rest| rest.strip_suffix(" not found"))
                {
                    log_var_not_found(var);
                }
                e
            })?;

        let (then_lhs, then_op, then_rhs, then_consts) =
            Self::lower_binop_ast(&parts.then_update, builder, &phi_bindings).map_err(|e| {
                if let Some(var) = e
                    .strip_prefix("[normalizer] Variable ")
                    .and_then(|rest| rest.strip_suffix(" not found"))
                {
                    log_var_not_found(var);
                }
                e
            })?;

        let (else_lhs, else_op, else_rhs, else_consts) =
            Self::lower_binop_ast(&parts.else_update, builder, &phi_bindings).map_err(|e| {
                if let Some(var) = e
                    .strip_prefix("[normalizer] Variable ")
                    .and_then(|rest| rest.strip_suffix(" not found"))
                {
                    log_var_not_found(var);
                }
                e
            })?;

        let (loop_inc_lhs, loop_inc_op, loop_inc_rhs, loop_inc_consts) =
            Self::lower_binop_ast(&parts.loop_increment, builder, &phi_bindings).map_err(|e| {
                if let Some(var) = e
                    .strip_prefix("[normalizer] Variable ")
                    .and_then(|rest| rest.strip_suffix(" not found"))
                {
                    log_var_not_found(var);
                }
                e
            })?;

        // Step 7: Build header_effects
        let mut header_effects = loop_cond_consts;
        header_effects.push(CoreEffectPlan::Compare {
            dst: cond_loop,
            lhs: loop_cond_lhs,
            op: loop_cond_op,
            rhs: loop_cond_rhs,
        });

        // Step 8: Build body plans
        let mut body_plans: Vec<LoweredRecipe> = Vec::new();
        for const_effect in if_cond_consts {
            body_plans.push(CorePlan::Effect(const_effect));
        }
        body_plans.push(CorePlan::Effect(CoreEffectPlan::Compare {
            dst: cond_if,
            lhs: if_cond_lhs,
            op: if_cond_op,
            rhs: if_cond_rhs,
        }));

        // Step 9: Build then_effects
        let mut then_effects = then_consts;
        then_effects.push(CoreEffectPlan::BinOp {
            dst: carrier_then,
            lhs: then_lhs,
            op: then_op,
            rhs: then_rhs,
        });

        // Step 10: Build else_effects
        let mut else_effects = else_consts;
        else_effects.push(CoreEffectPlan::BinOp {
            dst: carrier_else,
            lhs: else_lhs,
            op: else_op,
            rhs: else_rhs,
        });

        // Step 11: Build step_effects
        let mut step_effects = loop_inc_consts;
        step_effects.push(CoreEffectPlan::BinOp {
            dst: loop_var_next,
            lhs: loop_inc_lhs,
            op: loop_inc_op,
            rhs: loop_inc_rhs,
        });

        // Step 12: Build block_effects
        let block_effects = vec![
            (preheader_bb, vec![]),
            (header_bb, header_effects),
            (body_bb, vec![]),
            (then_bb, then_effects),
            (else_bb, else_effects),
            (merge_bb, vec![]),
            (step_bb, step_effects),
        ];

        // Step 13: Build PHIs (2 PHIs in header only)
        let phis = vec![
            build_loop_phi_info(
                header_bb,
                preheader_bb,
                step_bb,
                loop_var_current,
                loop_var_init,
                loop_var_next,
                format!("loop_var_{}", parts.loop_var),
            ),
            build_loop_phi_info(
                header_bb,
                preheader_bb,
                step_bb,
                carrier_current,
                carrier_init,
                carrier_next,
                format!("carrier_{}", parts.carrier_var),
            ),
        ];

        // Step 14: Build Frag (merge join via block_params + edge args)
        let empty_args = EdgeArgs {
            layout: JumpArgsLayout::CarriersOnly,
            values: vec![],
        };

        let merge_join_args_then = EdgeArgs {
            layout: JumpArgsLayout::ExprResultPlusCarriers,
            values: vec![carrier_then],
        };
        let merge_join_args_else = EdgeArgs {
            layout: JumpArgsLayout::ExprResultPlusCarriers,
            values: vec![carrier_else],
        };

        let branches = vec![
            edgecfg_stubs::build_loop_header_branch_with_args(
                header_bb,
                cond_loop,
                body_bb,
                empty_args.clone(),
                after_bb,
                empty_args.clone(),
            ),
            edgecfg_stubs::build_loop_cond_branch(body_bb, cond_if, then_bb, else_bb),
        ];

        let wires = vec![
            edgecfg_stubs::build_loop_back_edge_with_args(then_bb, merge_bb, merge_join_args_then),
            edgecfg_stubs::build_loop_back_edge_with_args(else_bb, merge_bb, merge_join_args_else),
            edgecfg_stubs::build_loop_back_edge_with_args(merge_bb, step_bb, empty_args.clone()),
            edgecfg_stubs::build_loop_back_edge_with_args(step_bb, header_bb, empty_args.clone()),
        ];

        let mut block_params = BTreeMap::new();
        block_params.insert(
            merge_bb,
            BlockParams {
                layout: JumpArgsLayout::ExprResultPlusCarriers,
                params: vec![carrier_next],
            },
        );

        let frag = Frag {
            entry: header_bb,
            block_params,
            exits: BTreeMap::new(),
            wires,
            branches,
        };

        // Step 15: Build final_values
        let final_values = vec![
            (parts.loop_var.clone(), loop_var_current),
            (parts.carrier_var.clone(), carrier_current),
        ];
        let (step_mode, has_explicit_step) = extract_to_step_bb_explicit_step();

        // Step 16: Build CoreLoopPlan
        let loop_plan = CoreLoopPlan {
            preheader_bb,
            preheader_is_fresh: false,
            header_bb,
            body_bb,
            step_bb,
            continue_target: step_bb,
            after_bb,
            found_bb: after_bb,
            body: body_plans,
            cond_loop,
            cond_match: cond_if,
            block_effects,
            phis,
            frag,
            final_values,
            step_mode,
            has_explicit_step,
        };

        if debug {
            trace_logger.debug(
                "normalizer/pattern3_if_phi",
                "CorePlan construction complete (7 blocks, 2 PHIs + merge join via block_params)",
            );
        }

        Ok(CorePlan::Loop(loop_plan))
    }
}
