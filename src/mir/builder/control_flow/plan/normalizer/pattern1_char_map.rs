use super::helpers::{create_phi_bindings, LoopBlocksStandard5};
use super::{CoreEffectPlan, CoreLoopPlan, CorePlan, LoweredRecipe};
use crate::mir::builder::control_flow::plan::Pattern1CharMapPlan;
use crate::mir::basic_block::EdgeArgs;
use crate::mir::builder::control_flow::plan::edgecfg_facade::Frag;
use crate::mir::builder::control_flow::plan::features::edgecfg_stubs;
use crate::mir::builder::control_flow::plan::features::loop_carriers::build_loop_phi_info;
use crate::mir::builder::control_flow::plan::step_mode::extract_to_step_bb_explicit_step;
use crate::mir::builder::control_flow::joinir::patterns::router::LoopPatternContext;
use crate::mir::builder::MirBuilder;
use crate::mir::join_ir::lowering::inline_boundary::JumpArgsLayout;
use crate::mir::{BinaryOp, Effect, EffectMask, MirType};
use std::collections::BTreeMap;

impl super::PlanNormalizer {
    /// Phase 29ap P2: Pattern1CharMap → CorePlan conversion
    ///
    /// Expands a stdlib-style char-map loop into a generic CorePlan:
    /// - CFG structure: preheader → header → body → step → header (back-edge)
    /// - 2 PHIs in header: loop index + result accumulator
    /// - All body effects are emitted in the step block (body stays empty)
    pub(in crate::mir::builder) fn normalize_pattern1_char_map(
        builder: &mut MirBuilder,
        parts: Pattern1CharMapPlan,
        ctx: &LoopPatternContext,
    ) -> Result<LoweredRecipe, String> {
        use crate::mir::builder::control_flow::joinir::trace;

        let trace_logger = trace::trace();
        let debug = ctx.debug;

        if debug {
            trace_logger.debug(
                "normalizer/pattern1_char_map",
                &format!(
                    "Phase 29ap P2: Normalizing Pattern1CharMap for {} (loop_var: {}, result: {})",
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

        let haystack = builder
            .variable_ctx
            .variable_map
            .get(&parts.haystack_var)
            .copied()
            .ok_or_else(|| format!("[normalizer] Haystack {} not found", parts.haystack_var))?;

        let mut static_box_name: Option<String> = None;
        let receiver = if let Some(&value) = builder
            .variable_ctx
            .variable_map
            .get(&parts.receiver_var)
        {
            Some(value)
        } else if parts.receiver_var == "me" && ctx.in_static_box {
            let Some(box_name) = builder.comp_ctx.current_static_box.clone() else {
                return Err("[normalizer] Static receiver missing current_static_box".to_string());
            };
            static_box_name = Some(box_name);
            None
        } else {
            return Err(format!(
                "[normalizer] Receiver {} not found",
                parts.receiver_var
            ));
        };

        let blocks = LoopBlocksStandard5::allocate(builder)?;
        let LoopBlocksStandard5 {
            preheader_bb,
            header_bb,
            body_bb,
            step_bb,
            after_bb,
        } = blocks;

        let loop_var_current = builder.alloc_typed(MirType::Integer);
        let result_current = builder.alloc_typed(MirType::String);
        let cond_loop = builder.alloc_typed(MirType::Bool);
        let loop_var_next = builder.alloc_typed(MirType::Integer);
        let result_next = builder.alloc_typed(MirType::String);
        let ch_value = builder.alloc_typed(MirType::String);
        let mapped_value = builder.alloc_typed(MirType::String);

        let phi_bindings = create_phi_bindings(&[
            (&parts.loop_var, loop_var_current),
            (&parts.result_var, result_current),
        ]);

        let (loop_cond_lhs, loop_cond_op, loop_cond_rhs, loop_cond_consts) =
            Self::lower_compare_ast(&parts.condition, builder, &phi_bindings)?;

        let (loop_inc_lhs, loop_inc_op, loop_inc_rhs, loop_inc_consts) =
            Self::lower_binop_ast(&parts.loop_increment, builder, &phi_bindings)?;

        let preheader_effects = Vec::new();

        let mut header_effects = loop_cond_consts;
        header_effects.push(CoreEffectPlan::Compare {
            dst: cond_loop,
            lhs: loop_cond_lhs,
            op: loop_cond_op,
            rhs: loop_cond_rhs,
        });

        let mut step_effects = loop_inc_consts;
        step_effects.push(CoreEffectPlan::BinOp {
            dst: loop_var_next,
            lhs: loop_inc_lhs,
            op: loop_inc_op,
            rhs: loop_inc_rhs,
        });
        step_effects.push(CoreEffectPlan::MethodCall {
            dst: Some(ch_value),
            object: haystack,
            method: "substring".to_string(),
            args: vec![loop_var_current, loop_var_next],
            effects: EffectMask::PURE.add(Effect::Io),
        });
        if let Some(box_name) = static_box_name {
            let func_name = format!("{}.{}/{}", box_name, parts.transform_method, 1);
            step_effects.push(CoreEffectPlan::GlobalCall {
                dst: Some(mapped_value),
                func: func_name,
                args: vec![ch_value],
            });
        } else {
            let receiver_id = receiver.ok_or_else(|| "[normalizer] Missing receiver".to_string())?;
            step_effects.push(CoreEffectPlan::MethodCall {
                dst: Some(mapped_value),
                object: receiver_id,
                method: parts.transform_method.clone(),
                args: vec![ch_value],
                effects: EffectMask::PURE.add(Effect::Io),
            });
        }
        step_effects.push(CoreEffectPlan::BinOp {
            dst: result_next,
            lhs: result_current,
            op: BinaryOp::Add,
            rhs: mapped_value,
        });

        let block_effects = vec![
            (preheader_bb, preheader_effects),
            (header_bb, header_effects),
            (body_bb, vec![]),
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
                "normalizer/pattern1_char_map",
                "CorePlan construction complete (5 blocks, 2 PHIs)",
            );
        }

        Ok(CorePlan::Loop(loop_plan))
    }
}
