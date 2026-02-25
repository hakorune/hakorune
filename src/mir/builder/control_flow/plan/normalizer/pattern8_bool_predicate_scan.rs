use super::helpers::create_phi_bindings;
use super::{CoreEffectPlan, CoreLoopPlan, CorePlan, Pattern8BoolPredicateScanPlan, LoweredRecipe};
use crate::mir::basic_block::EdgeArgs;
use crate::mir::builder::control_flow::plan::edgecfg_facade::{ExitKind, Frag};
use crate::mir::builder::control_flow::plan::features::edgecfg_stubs;
use crate::mir::builder::control_flow::plan::features::loop_carriers::build_loop_phi_info;
use crate::mir::builder::control_flow::plan::step_mode::extract_to_step_bb_explicit_step;
use crate::mir::builder::control_flow::joinir::patterns::router::LoopPatternContext;
use crate::mir::builder::MirBuilder;
use crate::mir::join_ir::lowering::inline_boundary::JumpArgsLayout;
use crate::mir::{BinaryOp, CompareOp, ConstValue, Effect, EffectMask, MirType};
use std::collections::BTreeMap;

impl super::PlanNormalizer {
    /// Pattern8BoolPredicateScan → CorePlan 変換 (Phase 286 P2.4.1)
    ///
    /// Expands Pattern8 (Bool Predicate Scan) semantics into generic CorePlan:
    /// - CFG structure: preheader → header → body → found (return false) / step → header
    /// - 1 PHI for loop variable (i)
    /// - Early exit: return false in found_bb
    /// - Post-loop: return true in after_bb
    pub(in crate::mir::builder) fn normalize_pattern8_bool_predicate_scan(
        builder: &mut MirBuilder,
        parts: Pattern8BoolPredicateScanPlan,
        ctx: &LoopPatternContext,
    ) -> Result<LoweredRecipe, String> {
        use crate::mir::builder::control_flow::plan::edgecfg_facade::compose;
        use crate::mir::builder::control_flow::joinir::trace;

        let trace_logger = trace::trace();
        let debug = ctx.debug;

        if debug {
            trace_logger.debug(
                "normalizer/pattern8_bool_predicate_scan",
                &format!(
                    "Phase 286 P2.4.1: Normalizing Pattern8 for {} (loop_var: {}, predicate: {}.{})",
                    ctx.func_name, parts.loop_var, parts.predicate_receiver, parts.predicate_method
                ),
            );
        }

        // P0 Scope: Forward scan (step=1) only
        if parts.step_lit != 1 {
            return Err(format!(
                "[normalizer/pattern8] P0 scope: only forward scan supported (step={})",
                parts.step_lit
            ));
        }

        // Step 1: Get host ValueIds for variables
        let haystack_host = builder
            .variable_ctx
            .variable_map
            .get(&parts.haystack)
            .copied()
            .ok_or_else(|| format!("[normalizer/pattern8] Variable {} not found", parts.haystack))?;

        let mut static_box_name: Option<String> = None;
        let predicate_receiver_host = if let Some(&value) = builder
            .variable_ctx
            .variable_map
            .get(&parts.predicate_receiver)
        {
            Some(value)
        } else if parts.predicate_receiver == "me" && ctx.in_static_box {
            let Some(box_name) = builder.comp_ctx.current_static_box.clone() else {
                return Err(
                    "[normalizer/pattern8] Static receiver missing current_static_box".to_string(),
                );
            };
            static_box_name = Some(box_name);
            None
        } else {
            return Err(format!(
                "[normalizer/pattern8] Variable {} not found",
                parts.predicate_receiver
            ));
        };

        let loop_var_init = builder
            .variable_ctx
            .variable_map
            .get(&parts.loop_var)
            .copied()
            .ok_or_else(|| format!("[normalizer/pattern8] Loop variable {} not found", parts.loop_var))?;

        // Step 2: Capture preheader block
        let preheader_bb = builder
            .current_block
            .ok_or_else(|| "[normalizer/pattern8] No current block for loop entry".to_string())?;

        // Step 3: Allocate BasicBlockIds for 6 blocks
        let header_bb = builder.next_block_id();
        let body_bb = builder.next_block_id();
        let found_bb = builder.next_block_id();
        let step_bb = builder.next_block_id();
        let after_bb = builder.next_block_id();

        if debug {
            trace_logger.debug(
                "normalizer/pattern8_bool_predicate_scan",
                &format!(
                    "Allocated: preheader={:?}, header={:?}, body={:?}, found={:?}, step={:?}, after={:?}",
                    preheader_bb, header_bb, body_bb, found_bb, step_bb, after_bb
                ),
            );
        }

        // Step 4: Allocate ValueIds
        let loop_var_current = builder.alloc_typed(MirType::Integer);
        let cond_loop = builder.alloc_typed(MirType::Bool);
        let one_val = builder.alloc_typed(MirType::Integer);
        let i_plus_one = builder.alloc_typed(MirType::Integer);
        let ch = builder.alloc_typed(MirType::String);
        let cond_predicate = builder.alloc_typed(MirType::Bool);
        let cond_not_predicate = builder.alloc_typed(MirType::Bool);
        let loop_var_next = builder.alloc_typed(MirType::Integer);
        let true_val = builder.alloc_typed(MirType::Bool);
        let false_val = builder.alloc_typed(MirType::Bool);

        // Step 5: Build phi_bindings
        let phi_bindings = create_phi_bindings(&[(&parts.loop_var, loop_var_current)]);

        // Step 6: Lower loop condition
        let (loop_cond_lhs, loop_cond_op, loop_cond_rhs, loop_cond_consts) =
            Self::lower_compare_ast(&parts.condition, builder, &phi_bindings)?;

        // Step 7: Build header_effects
        let mut header_effects = loop_cond_consts;
        header_effects.push(CoreEffectPlan::Compare {
            dst: cond_loop,
            lhs: loop_cond_lhs,
            op: loop_cond_op,
            rhs: loop_cond_rhs,
        });

        // Step 8: Build body (predicate check)
        let predicate_call = if let Some(box_name) = static_box_name {
            let func_name = format!("{}.{}/{}", box_name, parts.predicate_method, 1);
            CorePlan::Effect(CoreEffectPlan::GlobalCall {
                dst: Some(cond_predicate),
                func: func_name,
                args: vec![ch],
            })
        } else {
            let receiver_id = predicate_receiver_host
                .ok_or_else(|| "[normalizer/pattern8] Missing predicate receiver".to_string())?;
            CorePlan::Effect(CoreEffectPlan::MethodCall {
                dst: Some(cond_predicate),
                object: receiver_id,
                method: parts.predicate_method.clone(),
                args: vec![ch],
                effects: EffectMask::PURE.add(Effect::Io),
            })
        };

        let body = vec![
            CorePlan::Effect(CoreEffectPlan::Const {
                dst: one_val,
                value: ConstValue::Integer(1),
            }),
            CorePlan::Effect(CoreEffectPlan::Const {
                dst: false_val,
                value: ConstValue::Bool(false),
            }),
            CorePlan::Effect(CoreEffectPlan::BinOp {
                dst: i_plus_one,
                lhs: loop_var_current,
                op: BinaryOp::Add,
                rhs: one_val,
            }),
            CorePlan::Effect(CoreEffectPlan::MethodCall {
                dst: Some(ch),
                object: haystack_host,
                method: "substring".to_string(),
                args: vec![loop_var_current, i_plus_one],
                effects: EffectMask::PURE.add(Effect::Io),
            }),
            predicate_call,
            CorePlan::Effect(CoreEffectPlan::Compare {
                dst: cond_not_predicate,
                lhs: cond_predicate,
                op: CompareOp::Eq,
                rhs: false_val,
            }),
        ];

        // Step 9: Build step_effects
        let step_effects = vec![CoreEffectPlan::BinOp {
            dst: loop_var_next,
            lhs: loop_var_current,
            op: BinaryOp::Add,
            rhs: one_val,
        }];

        // Step 10: Build found_bb effects (empty)
        let found_effects = vec![];

        // Step 11: Build after_bb effects
        let after_effects = vec![CoreEffectPlan::Const {
            dst: true_val,
            value: ConstValue::Bool(true),
        }];

        // Step 12: Build block_effects
        let block_effects = vec![
            (preheader_bb, vec![]),
            (header_bb, header_effects),
            (body_bb, vec![]),
            (found_bb, found_effects),
            (step_bb, step_effects),
            (after_bb, after_effects),
        ];

        // Step 13: Build PHI
        let phis = vec![build_loop_phi_info(
            header_bb,
            preheader_bb,
            step_bb,
            loop_var_current,
            loop_var_init,
            loop_var_next,
            format!("loop_var_{}", parts.loop_var),
        )];

        // Step 14: Build Frag using compose::cleanup for early exit
        let empty_args = EdgeArgs {
            layout: JumpArgsLayout::CarriersOnly,
            values: vec![],
        };

        let ret_false_args = EdgeArgs {
            layout: JumpArgsLayout::CarriersOnly,
            values: vec![false_val],
        };

        let ret_true_args = EdgeArgs {
            layout: JumpArgsLayout::CarriersOnly,
            values: vec![true_val],
        };

        let main_branches = vec![
            edgecfg_stubs::build_loop_header_branch_with_args(
                header_bb,
                cond_loop,
                body_bb,
                empty_args.clone(),
                after_bb,
                empty_args.clone(),
            ),
            edgecfg_stubs::build_loop_cond_branch(body_bb, cond_not_predicate, found_bb, step_bb),
        ];

        let main_wires = vec![edgecfg_stubs::build_loop_back_edge_with_args(
            step_bb,
            header_bb,
            empty_args.clone(),
        )];

        let main_frag = Frag {
            entry: header_bb,
            block_params: BTreeMap::new(),
            exits: BTreeMap::new(),
            wires: main_wires,
            branches: main_branches,
        };

        let cleanup_exits = vec![
            edgecfg_stubs::build_return_exit_stub(found_bb, ret_false_args),
            edgecfg_stubs::build_return_exit_stub(after_bb, ret_true_args),
        ];

        let cleanup_frag = Frag {
            entry: found_bb,
            block_params: BTreeMap::new(),
            exits: BTreeMap::from([(ExitKind::Return, cleanup_exits)]),
            wires: vec![],
            branches: vec![],
        };

        let frag = compose::cleanup(main_frag, cleanup_frag, None, None)
            .expect("compose::cleanup() failed in normalize_pattern8_bool_predicate_scan");

        // Step 15: Build final_values
        let final_values = vec![(parts.loop_var.clone(), loop_var_current)];
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
            found_bb,
            body,
            cond_loop,
            cond_match: cond_not_predicate,
            block_effects,
            phis,
            frag,
            final_values,
            step_mode,
            has_explicit_step,
        };

        if debug {
            trace_logger.debug(
                "normalizer/pattern8_bool_predicate_scan",
                "CorePlan construction complete (5 blocks, 1 PHI, 2 Return exits)",
            );
        }

        Ok(CorePlan::Loop(loop_plan))
    }
}
