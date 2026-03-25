//! Test-only loop_break normalizer harness.
//!
//! This module now uses the semantic `loop_break.rs` file name.

use super::helpers_layout::create_phi_bindings;
use crate::ast::ASTNode;
use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::edgecfg_facade::{BlockParams, Frag};
use crate::mir::builder::control_flow::plan::features::edgecfg_stubs;
use crate::mir::builder::control_flow::plan::features::loop_carriers::build_loop_phi_info;
use crate::mir::builder::control_flow::plan::step_mode::extract_to_step_bb_explicit_step;
use crate::mir::builder::control_flow::plan::{
    CoreEffectPlan, CoreLoopPlan, CorePlan, LoopBreakStepPlacement, LoweredRecipe,
};
use crate::mir::builder::MirBuilder;
use crate::mir::join_ir::lowering::inline_boundary::JumpArgsLayout;
use crate::mir::EdgeArgs;
use crate::mir::{BinaryOp, ConstValue, MirType, ValueId};
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct LoopBreakPlan {
    loop_var: String,
    carrier_var: String,
    loop_condition: ASTNode,
    break_condition: ASTNode,
    carrier_update_in_break: Option<ASTNode>,
    carrier_update_in_body: ASTNode,
    loop_increment: ASTNode,
    step_placement: LoopBreakStepPlacement,
}

impl super::PlanNormalizer {
    /// LoopBreakPlan → CorePlan 変換
    ///
    /// CFG structure (6 blocks):
    /// ```
    /// preheader → header(PHI: i_current, carrier_current)
    ///               ↓
    ///            body(break_cond check)
    ///               ↓
    ///          ┌────┴────┐
    ///     break_then    step
    ///     (optional      ↓
    ///      update)    header (back-edge)
    ///          ↓
    ///        after_bb(join: carrier_out)
    ///          ↑
    ///        header (natural exit when !cond_loop)
    /// ```
    ///
    /// Key: after_bb PHI merges break path and natural exit path carrier values.
    pub(in crate::mir::builder) fn normalize_loop_break(
        builder: &mut MirBuilder,
        parts: LoopBreakPlan,
        ctx: &LoopRouteContext,
    ) -> Result<LoweredRecipe, String> {
        use crate::mir::builder::control_flow::joinir::trace;

        let trace_logger = trace::trace();
        let debug = ctx.debug;

        if debug {
            trace_logger.debug(
                "normalizer/loop_break",
                &format!(
                    "Phase 286 P3.1: Normalizing loop_break for {} (loop_var={}, carrier_var={})",
                    ctx.func_name, parts.loop_var, parts.carrier_var
                ),
            );
        }

        let step_before_break = matches!(parts.step_placement, LoopBreakStepPlacement::BeforeBreak);

        // Step 1: Block allocation (6 blocks)
        let preheader_bb = builder
            .current_block
            .ok_or_else(|| "loop_break: no current block".to_string())?;
        let header_bb = builder.next_block_id();
        let body_bb = builder.next_block_id();
        let break_then_bb = builder.next_block_id();
        let step_bb = builder.next_block_id();
        let after_bb = builder.next_block_id();

        if debug {
            trace_logger.debug(
                "normalizer/loop_break",
                &format!(
                    "Block allocation: preheader={:?}, header={:?}, body={:?}, break_then={:?}, step={:?}, after={:?}",
                    preheader_bb, header_bb, body_bb, break_then_bb, step_bb, after_bb
                ),
            );
        }

        // Step 2: Get initial values from variable_map
        let loop_var_init = builder
            .variable_ctx
            .variable_map
            .get(&parts.loop_var)
            .copied()
            .ok_or_else(|| {
                format!(
                    "loop_break: loop_var '{}' not in variable_map",
                    parts.loop_var
                )
            })?;

        let carrier_init = builder
            .variable_ctx
            .variable_map
            .get(&parts.carrier_var)
            .copied()
            .ok_or_else(|| {
                format!(
                    "loop_break: carrier_var '{}' not in variable_map",
                    parts.carrier_var
                )
            })?;

        // Step 3: ValueId allocation
        let loop_var_current = builder.alloc_typed(MirType::Integer);
        let carrier_type = builder
            .type_ctx
            .value_types
            .get(&carrier_init)
            .cloned()
            .unwrap_or(MirType::Integer);
        let carrier_current = builder.alloc_typed(carrier_type.clone());
        let cond_loop = builder.alloc_typed(MirType::Bool);
        let cond_break = builder.alloc_typed(MirType::Bool);
        let carrier_break = builder.alloc_typed(carrier_type.clone());
        let carrier_step = builder.alloc_typed(carrier_type.clone());
        let loop_var_next = builder.alloc_typed(MirType::Integer);
        let loop_var_out = if step_before_break {
            builder.alloc_typed(MirType::Integer)
        } else {
            loop_var_current
        };
        let carrier_out = builder.alloc_typed(carrier_type);

        // Step 4: phi_bindings for AST lowering
        let phi_bindings = create_phi_bindings(&[
            (&parts.loop_var, loop_var_current),
            (&parts.carrier_var, carrier_current),
        ]);
        let phi_bindings_after_step = if step_before_break {
            create_phi_bindings(&[
                (&parts.loop_var, loop_var_next),
                (&parts.carrier_var, carrier_current),
            ])
        } else {
            phi_bindings.clone()
        };
        let bindings_for_body = if step_before_break {
            &phi_bindings_after_step
        } else {
            &phi_bindings
        };

        // Step 5: Lower AST expressions
        let (loop_cond_lhs, loop_cond_op, loop_cond_rhs, loop_cond_consts) =
            Self::lower_compare_ast(&parts.loop_condition, builder, &phi_bindings)?;

        let (carrier_lhs, carrier_op, carrier_rhs, carrier_consts) =
            Self::lower_binop_ast(&parts.carrier_update_in_body, builder, bindings_for_body)?;

        let (loop_inc_lhs, loop_inc_op, loop_inc_rhs, loop_inc_consts) =
            Self::lower_binop_ast(&parts.loop_increment, builder, &phi_bindings)?;

        let break_then_effects = if let Some(ref break_update_ast) = parts.carrier_update_in_break {
            let (lhs, op, rhs, consts) =
                Self::lower_binop_ast(break_update_ast, builder, bindings_for_body)?;
            let mut effects = consts;
            effects.push(CoreEffectPlan::BinOp {
                dst: carrier_break,
                lhs,
                op,
                rhs,
            });
            effects
        } else {
            let zero = builder.alloc_typed(MirType::Integer);
            vec![
                CoreEffectPlan::Const {
                    dst: zero,
                    value: ConstValue::Integer(0),
                },
                CoreEffectPlan::BinOp {
                    dst: carrier_break,
                    lhs: carrier_current,
                    op: BinaryOp::Add,
                    rhs: zero,
                },
            ]
        };

        // Step 6: Build header_effects
        let mut header_effects = loop_cond_consts;
        header_effects.push(CoreEffectPlan::Compare {
            dst: cond_loop,
            lhs: loop_cond_lhs,
            op: loop_cond_op,
            rhs: loop_cond_rhs,
        });

        // Step 7: Build body plans (break condition check)
        let mut body_plans: Vec<LoweredRecipe> = Vec::new();
        if step_before_break {
            for effect in loop_inc_consts.clone() {
                body_plans.push(CorePlan::Effect(effect));
            }
            body_plans.push(CorePlan::Effect(CoreEffectPlan::BinOp {
                dst: loop_var_next,
                lhs: loop_inc_lhs,
                op: loop_inc_op,
                rhs: loop_inc_rhs,
            }));
        }
        let break_cond_effects = Self::lower_break_condition_effects(
            &parts.break_condition,
            builder,
            bindings_for_body,
            cond_break,
        )?;
        for effect in break_cond_effects {
            body_plans.push(CorePlan::Effect(effect));
        }

        // Step 8: Build step_effects
        let mut step_effects = carrier_consts;
        step_effects.push(CoreEffectPlan::BinOp {
            dst: carrier_step,
            lhs: carrier_lhs,
            op: carrier_op,
            rhs: carrier_rhs,
        });
        if !step_before_break {
            step_effects.extend(loop_inc_consts);
            step_effects.push(CoreEffectPlan::BinOp {
                dst: loop_var_next,
                lhs: loop_inc_lhs,
                op: loop_inc_op,
                rhs: loop_inc_rhs,
            });
        }

        // Step 9: Build block_effects
        let block_effects = vec![
            (preheader_bb, vec![]),
            (header_bb, header_effects),
            (body_bb, vec![]),
            (break_then_bb, break_then_effects),
            (step_bb, step_effects),
        ];

        // Step 10: Build PHIs (header only)
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
                carrier_step,
                format!("carrier_{}", parts.carrier_var),
            ),
        ];

        // Step 11: Build Frag
        let empty_args = EdgeArgs {
            layout: JumpArgsLayout::CarriersOnly,
            values: vec![],
        };
        let after_join_args = EdgeArgs {
            layout: JumpArgsLayout::ExprResultPlusCarriers,
            values: if step_before_break {
                vec![carrier_current, loop_var_current]
            } else {
                vec![carrier_current]
            },
        };
        let break_join_args = EdgeArgs {
            layout: JumpArgsLayout::ExprResultPlusCarriers,
            values: if step_before_break {
                vec![carrier_break, loop_var_next]
            } else {
                vec![carrier_break]
            },
        };

        let branches = vec![
            edgecfg_stubs::build_branch_stub(
                header_bb,
                cond_loop,
                body_bb,
                empty_args.clone(),
                after_bb,
                after_join_args,
            ),
            edgecfg_stubs::build_loop_cond_branch(body_bb, cond_break, break_then_bb, step_bb),
        ];

        let wires = vec![
            edgecfg_stubs::build_loop_back_edge_with_args(break_then_bb, after_bb, break_join_args),
            edgecfg_stubs::build_loop_back_edge_with_args(step_bb, header_bb, empty_args.clone()),
        ];

        let mut block_params = BTreeMap::new();
        block_params.insert(
            after_bb,
            BlockParams {
                layout: JumpArgsLayout::ExprResultPlusCarriers,
                params: if step_before_break {
                    vec![carrier_out, loop_var_out]
                } else {
                    vec![carrier_out]
                },
            },
        );

        let frag = Frag {
            entry: header_bb,
            block_params,
            exits: BTreeMap::new(),
            wires,
            branches,
        };

        // Step 12: Build final_values
        let final_values = vec![
            (parts.loop_var.clone(), loop_var_out),
            (parts.carrier_var.clone(), carrier_out),
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
            body: body_plans,
            cond_loop,
            cond_match: cond_break,
            block_effects,
            phis,
            frag,
            final_values,
            step_mode,
            has_explicit_step,
        };

        if debug {
            trace_logger.debug(
                "normalizer/loop_break",
                "CorePlan construction complete (6 blocks, 2 PHIs, after_bb uses block_params)",
            );
        }

        Ok(CorePlan::Loop(loop_plan))
    }

    fn lower_break_condition_effects(
        ast: &crate::ast::ASTNode,
        builder: &mut MirBuilder,
        phi_bindings: &BTreeMap<String, ValueId>,
        dst: ValueId,
    ) -> Result<Vec<CoreEffectPlan>, String> {
        use crate::ast::{ASTNode, BinaryOperator};

        fn lower_bool_expr(
            ast: &ASTNode,
            builder: &mut MirBuilder,
            phi_bindings: &BTreeMap<String, ValueId>,
            dst: ValueId,
        ) -> Result<Vec<CoreEffectPlan>, String> {
            match ast {
                ASTNode::BinaryOp {
                    operator: BinaryOperator::Or,
                    left,
                    right,
                    ..
                } => {
                    let left_dst = builder.alloc_typed(MirType::Bool);
                    let right_dst = builder.alloc_typed(MirType::Bool);
                    let mut effects =
                        lower_bool_expr(left.as_ref(), builder, phi_bindings, left_dst)?;
                    let mut right_effects =
                        lower_bool_expr(right.as_ref(), builder, phi_bindings, right_dst)?;
                    effects.append(&mut right_effects);
                    effects.push(CoreEffectPlan::BinOp {
                        dst,
                        lhs: left_dst,
                        op: BinaryOp::Or,
                        rhs: right_dst,
                    });
                    Ok(effects)
                }
                ASTNode::BinaryOp {
                    operator: BinaryOperator::And,
                    left,
                    right,
                    ..
                } => {
                    let left_dst = builder.alloc_typed(MirType::Bool);
                    let right_dst = builder.alloc_typed(MirType::Bool);
                    let mut effects =
                        lower_bool_expr(left.as_ref(), builder, phi_bindings, left_dst)?;
                    let mut right_effects =
                        lower_bool_expr(right.as_ref(), builder, phi_bindings, right_dst)?;
                    effects.append(&mut right_effects);
                    effects.push(CoreEffectPlan::BinOp {
                        dst,
                        lhs: left_dst,
                        op: BinaryOp::And,
                        rhs: right_dst,
                    });
                    Ok(effects)
                }
                _ => {
                    let (lhs, op, rhs, consts) =
                        super::PlanNormalizer::lower_compare_ast(ast, builder, phi_bindings)?;
                    let mut effects = consts;
                    effects.push(CoreEffectPlan::Compare { dst, lhs, op, rhs });
                    Ok(effects)
                }
            }
        }

        lower_bool_expr(ast, builder, phi_bindings, dst)
    }
}
