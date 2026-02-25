use super::helpers::create_phi_bindings;
use super::{CoreEffectPlan, CoreLoopPlan, CorePlan, LoweredRecipe};
use crate::mir::builder::control_flow::plan::Pattern1ArrayJoinPlan;
use crate::mir::basic_block::EdgeArgs;
use crate::mir::builder::control_flow::plan::edgecfg_facade::{BlockParams, Frag};
use crate::mir::builder::control_flow::plan::features::edgecfg_stubs;
use crate::mir::builder::control_flow::plan::features::loop_carriers::build_loop_phi_info;
use crate::mir::builder::control_flow::plan::step_mode::extract_to_step_bb_explicit_step;
use crate::mir::builder::control_flow::joinir::patterns::router::LoopPatternContext;
use crate::mir::builder::MirBuilder;
use crate::mir::join_ir::lowering::inline_boundary::JumpArgsLayout;
use crate::mir::{BinaryOp, Effect, EffectMask, MirType};
use std::collections::BTreeMap;

impl super::PlanNormalizer {
    /// Phase 29ap P3: Pattern1ArrayJoin → CorePlan conversion
    ///
    /// Expands a stdlib-style StringUtils.join loop into a generic CorePlan:
    /// - CFG structure: preheader → header → body → sep?/step → header (back-edge)
    /// - 2 PHIs in header: loop index + result accumulator
    /// - separator branch merges via block_params on step_bb
    pub(in crate::mir::builder) fn normalize_pattern1_array_join(
        builder: &mut MirBuilder,
        parts: Pattern1ArrayJoinPlan,
        ctx: &LoopPatternContext,
    ) -> Result<LoweredRecipe, String> {
        use crate::mir::builder::control_flow::joinir::trace;

        let trace_logger = trace::trace();
        let debug = ctx.debug;

        if debug {
            trace_logger.debug(
                "normalizer/pattern1_array_join",
                &format!(
                    "Phase 29ap P3: Normalizing Pattern1ArrayJoin for {} (loop_var: {}, result: {})",
                    ctx.func_name, parts.loop_var, parts.result_var
                ),
            );
        }

        let loop_var_init = builder
            .variable_ctx
            .variable_map
            .get(&parts.loop_var)
            .copied()
            .ok_or_else(|| format!("[normalizer] Loop variable {} not found", parts.loop_var))?;

        let result_init = builder
            .variable_ctx
            .variable_map
            .get(&parts.result_var)
            .copied()
            .ok_or_else(|| format!("[normalizer] Result variable {} not found", parts.result_var))?;

        let array_id = builder
            .variable_ctx
            .variable_map
            .get(&parts.array_var)
            .copied()
            .ok_or_else(|| format!("[normalizer] Array {} not found", parts.array_var))?;

        let separator_id = builder
            .variable_ctx
            .variable_map
            .get(&parts.separator_var)
            .copied()
            .ok_or_else(|| format!("[normalizer] Separator {} not found", parts.separator_var))?;

        let preheader_bb = builder
            .current_block
            .ok_or_else(|| "[normalizer] No current block for loop entry".to_string())?;
        let header_bb = builder.next_block_id();
        let body_bb = builder.next_block_id();
        let sep_bb = builder.next_block_id();
        let step_bb = builder.next_block_id();
        let after_bb = builder.next_block_id();

        let loop_var_current = builder.alloc_typed(MirType::Integer);
        let result_current = builder.alloc_typed(MirType::String);
        let cond_loop = builder.alloc_typed(MirType::Bool);
        let cond_sep = builder.alloc_typed(MirType::Bool);
        let loop_var_next = builder.alloc_typed(MirType::Integer);
        let result_after_sep = builder.alloc_typed(MirType::String);
        let result_base = builder.alloc_typed(MirType::String);
        let result_next = builder.alloc_typed(MirType::String);
        let elem_value = builder.alloc_typed(MirType::String);

        let phi_bindings = create_phi_bindings(&[
            (&parts.loop_var, loop_var_current),
            (&parts.result_var, result_current),
        ]);

        let (loop_cond_lhs, loop_cond_op, loop_cond_rhs, loop_cond_consts) =
            Self::lower_compare_ast(&parts.condition, builder, &phi_bindings)?;

        let (if_cond_lhs, if_cond_op, if_cond_rhs, if_cond_consts) =
            Self::lower_compare_ast(&parts.if_condition, builder, &phi_bindings)?;

        let (loop_inc_lhs, loop_inc_op, loop_inc_rhs, loop_inc_consts) =
            Self::lower_binop_ast(&parts.loop_increment, builder, &phi_bindings)?;

        let mut header_effects = loop_cond_consts;
        header_effects.push(CoreEffectPlan::Compare {
            dst: cond_loop,
            lhs: loop_cond_lhs,
            op: loop_cond_op,
            rhs: loop_cond_rhs,
        });

        let mut body_plans = Vec::new();
        for effect in if_cond_consts {
            body_plans.push(CorePlan::Effect(effect));
        }
        body_plans.push(CorePlan::Effect(CoreEffectPlan::Compare {
            dst: cond_sep,
            lhs: if_cond_lhs,
            op: if_cond_op,
            rhs: if_cond_rhs,
        }));

        let sep_effects = vec![CoreEffectPlan::BinOp {
            dst: result_after_sep,
            lhs: result_current,
            op: BinaryOp::Add,
            rhs: separator_id,
        }];

        let mut step_effects = loop_inc_consts;
        step_effects.push(CoreEffectPlan::BinOp {
            dst: loop_var_next,
            lhs: loop_inc_lhs,
            op: loop_inc_op,
            rhs: loop_inc_rhs,
        });
        step_effects.push(CoreEffectPlan::MethodCall {
            dst: Some(elem_value),
            object: array_id,
            method: "get".to_string(),
            args: vec![loop_var_current],
            effects: EffectMask::PURE.add(Effect::Io),
        });
        step_effects.push(CoreEffectPlan::BinOp {
            dst: result_next,
            lhs: result_base,
            op: BinaryOp::Add,
            rhs: elem_value,
        });

        let block_effects = vec![
            (preheader_bb, vec![]),
            (header_bb, header_effects),
            (body_bb, vec![]),
            (sep_bb, sep_effects),
            (step_bb, step_effects),
        ];

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
                result_current,
                result_init,
                result_next,
                format!("result_var_{}", parts.result_var),
            ),
        ];

        let empty_args = EdgeArgs {
            layout: JumpArgsLayout::CarriersOnly,
            values: vec![],
        };
        let join_args_body = EdgeArgs {
            layout: JumpArgsLayout::ExprResultPlusCarriers,
            values: vec![result_current],
        };
        let join_args_sep = EdgeArgs {
            layout: JumpArgsLayout::ExprResultPlusCarriers,
            values: vec![result_after_sep],
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
            edgecfg_stubs::build_branch_stub(
                body_bb,
                cond_sep,
                sep_bb,
                empty_args.clone(),
                step_bb,
                join_args_body,
            ),
        ];

        let wires = vec![
            edgecfg_stubs::build_loop_back_edge_with_args(sep_bb, step_bb, join_args_sep),
            edgecfg_stubs::build_loop_back_edge_with_args(step_bb, header_bb, empty_args.clone()),
        ];

        let mut block_params = BTreeMap::new();
        block_params.insert(
            step_bb,
            BlockParams {
                layout: JumpArgsLayout::ExprResultPlusCarriers,
                params: vec![result_base],
            },
        );

        let frag = Frag {
            entry: header_bb,
            block_params,
            exits: BTreeMap::new(),
            wires,
            branches,
        };

        let final_values = vec![
            (parts.loop_var.clone(), loop_var_current),
            (parts.result_var.clone(), result_current),
        ];
        let (step_mode, has_explicit_step) = extract_to_step_bb_explicit_step();

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
                "normalizer/pattern1_array_join",
                "CorePlan construction complete (6 blocks, 2 PHIs, join at step_bb)",
            );
        }

        Ok(CorePlan::Loop(loop_plan))
    }
}
