use super::helpers::{create_phi_bindings, LoopBlocksStandard5};
use super::{CoreEffectPlan, CoreLoopPlan, CorePlan, Pattern9AccumConstLoopPlan, LoweredRecipe};
use crate::mir::basic_block::EdgeArgs;
use crate::mir::builder::control_flow::plan::edgecfg_facade::Frag;
use crate::mir::builder::control_flow::plan::features::edgecfg_stubs;
use crate::mir::builder::control_flow::plan::features::loop_carriers::build_loop_phi_info;
use crate::mir::builder::control_flow::plan::step_mode::extract_to_step_bb_explicit_step;
use crate::mir::builder::control_flow::joinir::patterns::router::LoopPatternContext;
use crate::mir::builder::MirBuilder;
use crate::mir::join_ir::lowering::inline_boundary::JumpArgsLayout;
use crate::mir::MirType;
use std::collections::BTreeMap;

impl super::PlanNormalizer {
    /// Phase 286 P2.3: Pattern9AccumConstLoop → CorePlan 変換
    ///
    /// Expands Pattern9 (Accumulator Const Loop) semantics into generic CorePlan:
    /// - CFG structure: preheader → header → body → step → header (back-edge)
    /// - 2 PHIs in header: loop variable (i) and accumulator (sum)
    /// - Similar to Pattern1 but with an additional carrier
    pub(in crate::mir::builder) fn normalize_pattern9_accum_const_loop(
        builder: &mut MirBuilder,
        parts: Pattern9AccumConstLoopPlan,
        ctx: &LoopPatternContext,
    ) -> Result<LoweredRecipe, String> {
        use crate::mir::builder::control_flow::joinir::trace;

        let trace_logger = trace::trace();
        let debug = ctx.debug;

        if debug {
            trace_logger.debug(
                "normalizer/pattern9_accum_const_loop",
                &format!(
                    "Phase 286 P2.3: Normalizing Pattern9AccumConstLoop for {} (loop_var: {}, acc_var: {})",
                    ctx.func_name, parts.loop_var, parts.acc_var
                ),
            );
        }

        // Step 1: Get host ValueIds for variables
        let loop_var_init = builder
            .variable_ctx
            .variable_map
            .get(&parts.loop_var)
            .copied()
            .ok_or_else(|| format!("[normalizer] Loop variable {} not found", parts.loop_var))?;

        let acc_var_init = builder
            .variable_ctx
            .variable_map
            .get(&parts.acc_var)
            .copied()
            .ok_or_else(|| format!("[normalizer] Accumulator variable {} not found", parts.acc_var))?;

        // Step 2-3: Allocate BasicBlockIds
        let blocks = LoopBlocksStandard5::allocate(builder)?;
        let LoopBlocksStandard5 {
            preheader_bb,
            header_bb,
            body_bb,
            step_bb,
            after_bb,
        } = blocks;

        if debug {
            trace_logger.debug(
                "normalizer/pattern9_accum_const_loop",
                &format!(
                    "Allocated: preheader={:?}, header={:?}, body={:?}, step={:?}, after={:?}",
                    preheader_bb, header_bb, body_bb, step_bb, after_bb
                ),
            );
        }

        // Step 4: Allocate ValueIds for loop control
        let loop_var_current = builder.alloc_typed(MirType::Integer);
        let acc_var_current = builder.alloc_typed(MirType::Integer);
        let cond_loop = builder.alloc_typed(MirType::Bool);
        let acc_var_next = builder.alloc_typed(MirType::Integer);
        let loop_var_next = builder.alloc_typed(MirType::Integer);

        // Step 5: Build phi_bindings
        let phi_bindings = create_phi_bindings(&[
            (&parts.loop_var, loop_var_current),
            (&parts.acc_var, acc_var_current),
        ]);

        // Step 6: Lower AST expressions
        let (loop_cond_lhs, loop_cond_op, loop_cond_rhs, loop_cond_consts) =
            Self::lower_compare_ast(&parts.condition, builder, &phi_bindings)?;

        let (acc_update_lhs, acc_update_op, acc_update_rhs, acc_update_consts) =
            Self::lower_binop_ast(&parts.acc_update, builder, &phi_bindings)?;

        let (loop_inc_lhs, loop_inc_op, loop_inc_rhs, loop_inc_consts) =
            Self::lower_binop_ast(&parts.loop_increment, builder, &phi_bindings)?;

        // Step 7: Build header_effects
        let mut header_effects = loop_cond_consts;
        header_effects.push(CoreEffectPlan::Compare {
            dst: cond_loop,
            lhs: loop_cond_lhs,
            op: loop_cond_op,
            rhs: loop_cond_rhs,
        });

        // Step 8: Build step_effects
        let mut step_effects = acc_update_consts;
        step_effects.push(CoreEffectPlan::BinOp {
            dst: acc_var_next,
            lhs: acc_update_lhs,
            op: acc_update_op,
            rhs: acc_update_rhs,
        });
        step_effects.extend(loop_inc_consts);
        step_effects.push(CoreEffectPlan::BinOp {
            dst: loop_var_next,
            lhs: loop_inc_lhs,
            op: loop_inc_op,
            rhs: loop_inc_rhs,
        });

        // Step 9: Build block_effects
        let block_effects = vec![
            (preheader_bb, vec![]),
            (header_bb, header_effects),
            (body_bb, vec![]),
            (step_bb, step_effects),
        ];

        // Step 10: Build PHIs
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
                acc_var_current,
                acc_var_init,
                acc_var_next,
                format!("acc_var_{}", parts.acc_var),
            ),
        ];

        // Step 11: Build Frag
        let empty_args = EdgeArgs {
            layout: JumpArgsLayout::CarriersOnly,
            values: vec![],
        };

        let branches = vec![edgecfg_stubs::build_loop_header_branch_with_args(
            header_bb,
            cond_loop,
            body_bb,
            empty_args.clone(),
            after_bb,
            empty_args.clone(),
        )];

        let wires = vec![
            edgecfg_stubs::build_loop_back_edge_with_args(body_bb, step_bb, empty_args.clone()),
            edgecfg_stubs::build_loop_back_edge_with_args(step_bb, header_bb, empty_args.clone()),
        ];

        let frag = Frag {
            entry: header_bb,
            block_params: BTreeMap::new(),
            exits: BTreeMap::new(),
            wires,
            branches,
        };

        // Step 12: Build final_values
        let final_values = vec![
            (parts.loop_var.clone(), loop_var_current),
            (parts.acc_var.clone(), acc_var_current),
        ];
        let (step_mode, has_explicit_step) = extract_to_step_bb_explicit_step();

        // Step 13: Build CoreLoopPlan
        let loop_plan = CoreLoopPlan {
            preheader_bb,
            preheader_is_fresh: false,
            header_bb,
            body_bb,
            step_bb,
            continue_target: step_bb,
            after_bb,
            found_bb: after_bb,
            body: vec![],
            cond_loop,
            cond_match: cond_loop,
            block_effects,
            phis,
            frag,
            final_values,
            step_mode,
            has_explicit_step,
        };

        if debug {
            trace_logger.debug(
                "normalizer/pattern9_accum_const_loop",
                "CorePlan construction complete (4 blocks, 2 PHIs)",
            );
        }

        Ok(CorePlan::Loop(loop_plan))
    }
}
