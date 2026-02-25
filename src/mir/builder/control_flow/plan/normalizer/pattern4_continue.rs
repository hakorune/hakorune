use super::helpers::create_phi_bindings;
use super::{CoreEffectPlan, CoreLoopPlan, CorePlan, LoweredRecipe};
use crate::mir::builder::control_flow::plan::Pattern4ContinuePlan;
use crate::mir::basic_block::EdgeArgs;
use crate::mir::builder::control_flow::plan::edgecfg_facade::Frag;
use crate::mir::builder::control_flow::plan::features::edgecfg_stubs;
use crate::mir::builder::control_flow::plan::features::loop_carriers::build_phi_info;
use crate::mir::builder::control_flow::plan::step_mode::extract_to_step_bb_explicit_step;
use crate::mir::builder::control_flow::joinir::patterns::router::LoopPatternContext;
use crate::mir::builder::MirBuilder;
use crate::mir::join_ir::lowering::inline_boundary::JumpArgsLayout;
use crate::mir::MirType;
use std::collections::BTreeMap;

impl super::PlanNormalizer {
    /// Phase 286 P2: Pattern4Continue → CorePlan 変換
    ///
    /// Expands Pattern4 (Loop with Continue) semantics into generic CorePlan:
    /// - CFG structure: 2-step branching + header PHI merge
    /// - NO Select instruction (not in CoreEffectPlan)
    /// - NO after PHI (header PHI handles all paths)
    pub(in crate::mir::builder) fn normalize_pattern4_continue(
        builder: &mut MirBuilder,
        parts: Pattern4ContinuePlan,
        ctx: &LoopPatternContext,
    ) -> Result<LoweredRecipe, String> {
        use crate::mir::builder::control_flow::joinir::trace;

        let trace_logger = trace::trace();
        let debug = ctx.debug;

        if debug {
            trace_logger.debug(
                "normalizer/pattern4_continue",
                &format!(
                    "Phase 286 P2: Normalizing Pattern4Continue for {} (loop_var: {}, carriers: {:?})",
                    ctx.func_name, parts.loop_var, parts.carrier_vars
                ),
            );
        }

        // P2 PoC Scope: Single carrier only
        if parts.carrier_vars.len() != 1 {
            return Err(format!(
                "[normalizer] P2 PoC scope: only single carrier supported (found: {})",
                parts.carrier_vars.len()
            ));
        }

        let carrier_var = &parts.carrier_vars[0];

        // Step 1: Get host ValueIds for variables
        let loop_var_init = builder
            .variable_ctx
            .variable_map
            .get(&parts.loop_var)
            .copied()
            .ok_or_else(|| format!("[normalizer] Loop variable {} not found", parts.loop_var))?;

        let carrier_init = builder
            .variable_ctx
            .variable_map
            .get(carrier_var)
            .copied()
            .ok_or_else(|| format!("[normalizer] Carrier variable {} not found", carrier_var))?;

        // Step 2: Capture preheader block
        let preheader_bb = builder
            .current_block
            .ok_or_else(|| "[normalizer] No current block for loop entry".to_string())?;

        // Step 3: Allocate BasicBlockIds for 8 blocks (2-step branching architecture)
        let header_bb = builder.next_block_id();
        let body_bb = builder.next_block_id();
        let continue_path_bb = builder.next_block_id();
        let normal_path_bb = builder.next_block_id();
        let step_continue_bb = builder.next_block_id();
        let step_normal_bb = builder.next_block_id();
        let after_bb = builder.next_block_id();

        if debug {
            trace_logger.debug(
                "normalizer/pattern4_continue",
                &format!(
                    "Allocated: preheader={:?}, header={:?}, body={:?}, continue_path={:?}, normal_path={:?}, step_continue={:?}, step_normal={:?}, after={:?}",
                    preheader_bb,
                    header_bb,
                    body_bb,
                    continue_path_bb,
                    normal_path_bb,
                    step_continue_bb,
                    step_normal_bb,
                    after_bb
                ),
            );
        }

        // Step 4: Allocate ValueIds for loop control and carriers
        let loop_var_current = builder.alloc_typed(MirType::Integer);
        let carrier_current = builder.alloc_typed(MirType::Integer);

        // Step 4.5: Build phi_bindings
        let phi_bindings = create_phi_bindings(&[
            (&parts.loop_var, loop_var_current),
            (carrier_var, carrier_current),
        ]);

        let cond_loop = builder.alloc_typed(MirType::Bool);
        let cond_continue = builder.alloc_typed(MirType::Bool);
        let loop_var_cont_next = builder.alloc_typed(MirType::Integer);
        let carrier_updated = builder.alloc_typed(MirType::Integer);
        let loop_var_norm_next = builder.alloc_typed(MirType::Integer);

        // Step 5: Lower AST expressions to get operands and const definitions
        let (loop_cond_lhs, loop_cond_op, loop_cond_rhs, loop_cond_consts) =
            Self::lower_compare_ast(&parts.condition, builder, &phi_bindings)?;

        let (cont_cond_lhs, cont_cond_op, cont_cond_rhs, cont_cond_consts) =
            Self::lower_compare_ast(&parts.continue_condition, builder, &phi_bindings)?;

        let carrier_update_ast = parts
            .carrier_updates
            .get(carrier_var)
            .ok_or_else(|| format!("[normalizer] Carrier update for {} not found", carrier_var))?;
        let (carrier_update_lhs, carrier_update_op, carrier_update_rhs, carrier_update_consts) =
            Self::lower_binop_ast(carrier_update_ast, builder, &phi_bindings)?;

        let (loop_inc_lhs, loop_inc_op, loop_inc_rhs, loop_inc_consts) =
            Self::lower_binop_ast(&parts.loop_increment, builder, &phi_bindings)?;

        // Step 6: Build header_effects (const definitions + loop condition check)
        let mut header_effects = loop_cond_consts;
        header_effects.push(CoreEffectPlan::Compare {
            dst: cond_loop,
            lhs: loop_cond_lhs,
            op: loop_cond_op,
            rhs: loop_cond_rhs,
        });

        // Step 7: Build body (as CorePlan items, NOT block_effects)
        let mut body = Vec::new();
        for const_effect in cont_cond_consts {
            body.push(CorePlan::Effect(const_effect));
        }
        body.push(CorePlan::Effect(CoreEffectPlan::Compare {
            dst: cond_continue,
            lhs: cont_cond_lhs,
            op: cont_cond_op,
            rhs: cont_cond_rhs,
        }));

        // Step 8: Build continue_path effects
        let mut continue_path_effects = loop_inc_consts.clone();
        continue_path_effects.push(CoreEffectPlan::BinOp {
            dst: loop_var_cont_next,
            lhs: loop_inc_lhs,
            op: loop_inc_op,
            rhs: loop_inc_rhs,
        });

        // Step 9: Build normal_path effects
        let mut normal_path_effects = carrier_update_consts;
        normal_path_effects.push(CoreEffectPlan::BinOp {
            dst: carrier_updated,
            lhs: carrier_update_lhs,
            op: carrier_update_op,
            rhs: carrier_update_rhs,
        });
        normal_path_effects.extend(loop_inc_consts);
        normal_path_effects.push(CoreEffectPlan::BinOp {
            dst: loop_var_norm_next,
            lhs: loop_inc_lhs,
            op: loop_inc_op,
            rhs: loop_inc_rhs,
        });

        // Step 10: Build block_effects
        let block_effects = vec![
            (preheader_bb, vec![]),
            (header_bb, header_effects),
            (body_bb, vec![]),
            (continue_path_bb, continue_path_effects),
            (normal_path_bb, normal_path_effects),
            (step_continue_bb, vec![]),
            (step_normal_bb, vec![]),
        ];

        // Step 11: Build PHIs
        let phis = vec![
            build_phi_info(
                header_bb,
                loop_var_current,
                vec![
                    (preheader_bb, loop_var_init),
                    (step_continue_bb, loop_var_cont_next),
                    (step_normal_bb, loop_var_norm_next),
                ],
                format!("loop_var_{}", parts.loop_var),
            ),
            build_phi_info(
                header_bb,
                carrier_current,
                vec![
                    (preheader_bb, carrier_init),
                    (step_continue_bb, carrier_current),
                    (step_normal_bb, carrier_updated),
                ],
                format!("carrier_{}", carrier_var),
            ),
        ];

        // Step 12: Build Frag
        let empty_args = EdgeArgs {
            layout: JumpArgsLayout::CarriersOnly,
            values: vec![],
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
            edgecfg_stubs::build_loop_cond_branch(
                body_bb,
                cond_continue,
                continue_path_bb,
                normal_path_bb,
            ),
        ];

        let wires = vec![
            edgecfg_stubs::build_loop_back_edge_with_args(
                continue_path_bb,
                step_continue_bb,
                empty_args.clone(),
            ),
            edgecfg_stubs::build_loop_back_edge_with_args(
                step_continue_bb,
                header_bb,
                empty_args.clone(),
            ),
            edgecfg_stubs::build_loop_back_edge_with_args(
                normal_path_bb,
                step_normal_bb,
                empty_args.clone(),
            ),
            edgecfg_stubs::build_loop_back_edge_with_args(
                step_normal_bb,
                header_bb,
                empty_args.clone(),
            ),
        ];

        let frag = Frag {
            entry: header_bb,
            block_params: BTreeMap::new(),
            exits: BTreeMap::new(),
            wires,
            branches,
        };

        // Step 13: Build final_values
        let final_values = vec![
            (parts.loop_var.clone(), loop_var_current),
            (carrier_var.clone(), carrier_current),
        ];
        let (step_mode, has_explicit_step) = extract_to_step_bb_explicit_step();

        // Step 14: Build CoreLoopPlan
        let loop_plan = CoreLoopPlan {
            preheader_bb,
            preheader_is_fresh: false,
            header_bb,
            body_bb,
            step_bb: step_normal_bb,
            continue_target: step_normal_bb,
            after_bb,
            found_bb: after_bb,
            body,
            cond_loop,
            cond_match: cond_continue,
            block_effects,
            phis,
            frag,
            final_values,
            step_mode,
            has_explicit_step,
        };

        if debug {
            trace_logger.debug(
                "normalizer/pattern4_continue",
                "CorePlan construction complete (2-step branching with header PHI merge)",
            );
        }

        Ok(CorePlan::Loop(loop_plan))
    }
}
