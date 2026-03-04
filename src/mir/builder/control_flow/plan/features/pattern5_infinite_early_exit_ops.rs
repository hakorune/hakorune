//! Pattern5InfiniteEarlyExit feature ops (no AST re-parse, no skeleton allocation).

use crate::mir::builder::control_flow::plan::edgecfg_facade::{BlockParams, Frag};
use crate::mir::builder::control_flow::joinir::patterns::router::LoopPatternContext;
use crate::mir::builder::control_flow::plan::domain::{
    Pattern5ExitKind, Pattern5InfiniteEarlyExitPlan,
};
use crate::mir::builder::control_flow::plan::features::{
    carrier_merge, edgecfg_stubs, loop_carriers, step_mode,
};
use crate::mir::builder::control_flow::plan::normalizer::common::empty_args;
use crate::mir::builder::control_flow::plan::skeletons::loop_true::LoopTrueSkeleton;
use crate::mir::builder::control_flow::plan::{CoreEffectPlan, CoreLoopPlan};
use crate::mir::builder::control_flow::plan::normalizer::PlanNormalizer;
use crate::mir::builder::MirBuilder;
use crate::mir::join_ir::lowering::inline_boundary::JumpArgsLayout;
use crate::mir::{ConstValue, MirType, ValueId};
use std::collections::BTreeMap;

pub(in crate::mir::builder) fn build_pattern5_infinite_early_exit_plan(
    builder: &mut MirBuilder,
    parts: &Pattern5InfiniteEarlyExitPlan,
    ctx: &LoopPatternContext,
    skeleton: &LoopTrueSkeleton,
) -> Result<CoreLoopPlan, String> {
    use crate::mir::builder::control_flow::joinir::trace;

    let trace_logger = trace::trace();
    let debug = ctx.debug;

    if debug {
        trace_logger.debug(
            "normalizer/pattern5_infinite_early_exit",
            &format!(
                "Phase 29bw: Pattern5 pipeline for {} (loop_var={}, exit_kind={:?})",
                ctx.func_name, parts.loop_var, parts.exit_kind
            ),
        );
    }

    match parts.exit_kind {
        Pattern5ExitKind::Return => {
            build_pattern5_return(builder, parts, skeleton, debug, &trace_logger)
        }
        Pattern5ExitKind::Break => {
            build_pattern5_break(builder, parts, skeleton, debug, &trace_logger)
        }
    }
}

fn build_pattern5_return(
    builder: &mut MirBuilder,
    parts: &Pattern5InfiniteEarlyExitPlan,
    skeleton: &LoopTrueSkeleton,
    debug: bool,
    trace_logger: &crate::mir::builder::control_flow::joinir::trace::JoinLoopTrace,
) -> Result<CoreLoopPlan, String> {
    let found_bb = builder.next_block_id();

    if debug {
        trace_logger.debug(
            "normalizer/pattern5_return",
            &format!(
                "Blocks: preheader={:?}, header={:?}, body={:?}, found={:?}, step={:?}, after={:?}",
                skeleton.preheader_bb,
                skeleton.header_bb,
                skeleton.body_bb,
                found_bb,
                skeleton.step_bb,
                skeleton.after_bb
            ),
        );
    }

    let loop_var_init = builder
        .variable_ctx
        .variable_map
        .get(&parts.loop_var)
        .copied()
        .ok_or_else(|| {
            format!(
                "Pattern5Return: loop_var '{}' not in variable_map",
                parts.loop_var
            )
        })?;

    let loop_var_current = builder.alloc_typed(MirType::Integer);
    let cond_exit = builder.alloc_typed(MirType::Bool);
    let loop_var_next = builder.alloc_typed(MirType::Integer);

    let loop_bindings = loop_carriers::build_loop_bindings(&[(
        parts.loop_var.as_str(),
        loop_var_current,
    )]);

    let (exit_cond_lhs, exit_cond_op, exit_cond_rhs, exit_cond_consts) =
        PlanNormalizer::lower_compare_ast(&parts.exit_condition, builder, &loop_bindings)?;

    let return_value: Option<(ValueId, crate::ast::ASTNode)> =
        if let Some(ref exit_val_ast) = parts.exit_value {
            let return_val_id = builder.alloc_typed(MirType::Integer);
            let return_val_ast = exit_val_ast.clone();
            Some((return_val_id, return_val_ast))
        } else {
            None
        };

    let (inc_lhs, inc_op, inc_rhs, inc_consts) =
        PlanNormalizer::lower_binop_ast(&parts.loop_increment, builder, &loop_bindings)?;

    let mut body_plans = Vec::new();
    for eff in exit_cond_consts {
        body_plans.push(crate::mir::builder::control_flow::plan::CorePlan::Effect(eff));
    }
    body_plans.push(crate::mir::builder::control_flow::plan::CorePlan::Effect(
        CoreEffectPlan::Compare {
            dst: cond_exit,
            lhs: exit_cond_lhs,
            op: exit_cond_op,
            rhs: exit_cond_rhs,
        },
    ));

    let mut found_effects: Vec<CoreEffectPlan> = Vec::new();
    let return_val_id = if let Some((ret_val_id, ref ret_val_ast)) = return_value {
        if let crate::ast::ASTNode::Literal {
            value: crate::ast::LiteralValue::Integer(n),
            ..
        } = ret_val_ast
        {
            found_effects.push(CoreEffectPlan::Const {
                dst: ret_val_id,
                value: ConstValue::Integer(*n),
            });
        } else {
            let (lhs, op, rhs, consts) =
                PlanNormalizer::lower_binop_ast(ret_val_ast, builder, &loop_bindings)?;
            for c in consts {
                found_effects.push(c);
            }
            found_effects.push(CoreEffectPlan::BinOp {
                dst: ret_val_id,
                lhs,
                op,
                rhs,
            });
        }
        Some(ret_val_id)
    } else {
        None
    };

    let mut step_effects = inc_consts;
    step_effects.push(CoreEffectPlan::BinOp {
        dst: loop_var_next,
        lhs: inc_lhs,
        op: inc_op,
        rhs: inc_rhs,
    });

    let loop_carriers = loop_carriers::build_loop_carriers(
        skeleton.header_bb,
        skeleton.preheader_bb,
        skeleton.step_bb,
        vec![loop_carriers::LoopCarrierSpec {
            dst: loop_var_current,
            init: loop_var_init,
            next: loop_var_next,
            tag: format!("loop_carrier_{}", parts.loop_var),
        }],
    );

    let header_effects = vec![CoreEffectPlan::Const {
        dst: skeleton.cond_loop,
        value: ConstValue::Bool(true),
    }];

    let block_effects = vec![
        (skeleton.preheader_bb, vec![]),
        (skeleton.header_bb, header_effects),
        (skeleton.body_bb, vec![]),
        (found_bb, found_effects),
        (skeleton.step_bb, step_effects),
        (skeleton.after_bb, vec![]),
    ];

    let empty = empty_args();
    let branches = vec![
        edgecfg_stubs::build_loop_header_branch(
            skeleton.header_bb,
            skeleton.cond_loop,
            skeleton.body_bb,
            skeleton.after_bb,
        ),
        edgecfg_stubs::build_loop_cond_branch(
            skeleton.body_bb,
            cond_exit,
            found_bb,
            skeleton.step_bb,
        ),
    ];

    let return_args = edgecfg_stubs::build_return_args(
        return_val_id.into_iter().collect(),
    );

    let wires = vec![
        edgecfg_stubs::build_loop_back_edge(skeleton.step_bb, skeleton.header_bb),
        edgecfg_stubs::build_return_exit_stub(found_bb, return_args),
        edgecfg_stubs::build_return_exit_stub(skeleton.after_bb, empty.clone()),
    ];

    let frag = Frag {
        entry: skeleton.header_bb,
        block_params: BTreeMap::new(),
        exits: BTreeMap::new(),
        wires,
        branches,
    };

    let final_values = carrier_merge::build_final_values(vec![
        (parts.loop_var.as_str(), loop_var_current),
    ]);

    let (step_mode, has_explicit_step) = step_mode::extract_to_step_bb_explicit_step();

    let loop_plan = CoreLoopPlan {
        preheader_bb: skeleton.preheader_bb,
        preheader_is_fresh: false,
        header_bb: skeleton.header_bb,
        body_bb: skeleton.body_bb,
        step_bb: skeleton.step_bb,
        continue_target: skeleton.step_bb,
        after_bb: skeleton.after_bb,
        found_bb,
        body: body_plans,
        cond_loop: skeleton.cond_loop,
        cond_match: cond_exit,
        block_effects,
        phis: Vec::new(),
        frag,
        final_values,
        step_mode,
        has_explicit_step,
    };

    if debug {
        trace_logger.debug(
            "normalizer/pattern5_return",
            "CorePlan construction complete (pipeline)",
        );
    }

    Ok(loop_carriers::with_loop_carriers(loop_plan, loop_carriers))
}

fn build_pattern5_break(
    builder: &mut MirBuilder,
    parts: &Pattern5InfiniteEarlyExitPlan,
    skeleton: &LoopTrueSkeleton,
    debug: bool,
    trace_logger: &crate::mir::builder::control_flow::joinir::trace::JoinLoopTrace,
) -> Result<CoreLoopPlan, String> {
    let break_then_bb = builder.next_block_id();

    if debug {
        trace_logger.debug(
            "normalizer/pattern5_break",
            &format!(
                "Blocks: preheader={:?}, header={:?}, body={:?}, break_then={:?}, step={:?}, after={:?}",
                skeleton.preheader_bb,
                skeleton.header_bb,
                skeleton.body_bb,
                break_then_bb,
                skeleton.step_bb,
                skeleton.after_bb
            ),
        );
    }

    let carrier_var = parts
        .carrier_var
        .as_ref()
        .ok_or_else(|| "Pattern5Break: carrier_var required".to_string())?;
    let carrier_update_ast = parts
        .carrier_update
        .as_ref()
        .ok_or_else(|| "Pattern5Break: carrier_update required".to_string())?;

    let loop_var_init = builder
        .variable_ctx
        .variable_map
        .get(&parts.loop_var)
        .copied()
        .ok_or_else(|| {
            format!(
                "Pattern5Break: loop_var '{}' not in variable_map",
                parts.loop_var
            )
        })?;

    let carrier_init = builder
        .variable_ctx
        .variable_map
        .get(carrier_var)
        .copied()
        .ok_or_else(|| {
            format!(
                "Pattern5Break: carrier_var '{}' not in variable_map",
                carrier_var
            )
        })?;

    let loop_var_current = builder.alloc_typed(MirType::Integer);
    let carrier_current = builder.alloc_typed(MirType::Integer);
    let cond_exit = builder.alloc_typed(MirType::Bool);
    let carrier_step = builder.alloc_typed(MirType::Integer);
    let loop_var_next = builder.alloc_typed(MirType::Integer);
    let carrier_out = builder.alloc_typed(MirType::Integer);

    let loop_bindings = loop_carriers::build_loop_bindings(&[
        (parts.loop_var.as_str(), loop_var_current),
        (carrier_var.as_str(), carrier_current),
    ]);

    let (exit_cond_lhs, exit_cond_op, exit_cond_rhs, exit_cond_consts) =
        PlanNormalizer::lower_compare_ast(&parts.exit_condition, builder, &loop_bindings)?;

    let (carrier_lhs, carrier_op, carrier_rhs, carrier_consts) =
        PlanNormalizer::lower_binop_ast(carrier_update_ast, builder, &loop_bindings)?;

    let (inc_lhs, inc_op, inc_rhs, inc_consts) =
        PlanNormalizer::lower_binop_ast(&parts.loop_increment, builder, &loop_bindings)?;

    let mut body_plans = Vec::new();
    for eff in exit_cond_consts {
        body_plans.push(crate::mir::builder::control_flow::plan::CorePlan::Effect(eff));
    }
    body_plans.push(crate::mir::builder::control_flow::plan::CorePlan::Effect(
        CoreEffectPlan::Compare {
            dst: cond_exit,
            lhs: exit_cond_lhs,
            op: exit_cond_op,
            rhs: exit_cond_rhs,
        },
    ));

    let mut step_effects = carrier_consts;
    step_effects.push(CoreEffectPlan::BinOp {
        dst: carrier_step,
        lhs: carrier_lhs,
        op: carrier_op,
        rhs: carrier_rhs,
    });
    for c in inc_consts {
        step_effects.push(c);
    }
    step_effects.push(CoreEffectPlan::BinOp {
        dst: loop_var_next,
        lhs: inc_lhs,
        op: inc_op,
        rhs: inc_rhs,
    });

    let loop_carriers = loop_carriers::build_loop_carriers(
        skeleton.header_bb,
        skeleton.preheader_bb,
        skeleton.step_bb,
        vec![
            loop_carriers::LoopCarrierSpec {
                dst: loop_var_current,
                init: loop_var_init,
                next: loop_var_next,
                tag: format!("loop_carrier_{}", parts.loop_var),
            },
            loop_carriers::LoopCarrierSpec {
                dst: carrier_current,
                init: carrier_init,
                next: carrier_step,
                tag: format!("loop_carrier_{}", carrier_var),
            },
        ],
    );

    let header_effects = vec![CoreEffectPlan::Const {
        dst: skeleton.cond_loop,
        value: ConstValue::Bool(true),
    }];

    let block_effects = vec![
        (skeleton.preheader_bb, vec![]),
        (skeleton.header_bb, header_effects),
        (skeleton.body_bb, vec![]),
        (break_then_bb, vec![]),
        (skeleton.step_bb, step_effects),
    ];

    let after_join_args = loop_carriers::build_expr_carrier_join_args(vec![carrier_current]);
    let break_join_args = loop_carriers::build_expr_carrier_join_args(vec![carrier_current]);
    let empty = empty_args();

    let branches = vec![
        edgecfg_stubs::build_loop_header_branch_with_args(
            skeleton.header_bb,
            skeleton.cond_loop,
            skeleton.body_bb,
            empty.clone(),
            skeleton.after_bb,
            after_join_args,
        ),
        edgecfg_stubs::build_loop_cond_branch(
            skeleton.body_bb,
            cond_exit,
            break_then_bb,
            skeleton.step_bb,
        ),
    ];

    let loop_id = crate::mir::control_form::LoopId(0);

    let wires = vec![
        edgecfg_stubs::build_break_exit_stub(
            break_then_bb,
            loop_id,
            skeleton.after_bb,
            break_join_args,
        ),
        edgecfg_stubs::build_loop_back_edge(skeleton.step_bb, skeleton.header_bb),
    ];

    let mut block_params = BTreeMap::new();
    block_params.insert(
        skeleton.after_bb,
        BlockParams {
            layout: JumpArgsLayout::ExprResultPlusCarriers,
            params: vec![carrier_out],
        },
    );

    let frag = Frag {
        entry: skeleton.header_bb,
        block_params,
        exits: BTreeMap::new(),
        wires,
        branches,
    };

    let final_values = carrier_merge::build_final_values(vec![
        (parts.loop_var.as_str(), loop_var_current),
        (carrier_var.as_str(), carrier_out),
    ]);

    let (step_mode, has_explicit_step) = step_mode::extract_to_step_bb_explicit_step();

    let loop_plan = CoreLoopPlan {
        preheader_bb: skeleton.preheader_bb,
        preheader_is_fresh: false,
        header_bb: skeleton.header_bb,
        body_bb: skeleton.body_bb,
        step_bb: skeleton.step_bb,
        continue_target: skeleton.step_bb,
        after_bb: skeleton.after_bb,
        found_bb: skeleton.after_bb,
        body: body_plans,
        cond_loop: skeleton.cond_loop,
        cond_match: cond_exit,
        block_effects,
        phis: Vec::new(),
        frag,
        final_values,
        step_mode,
        has_explicit_step,
    };

    if debug {
        trace_logger.debug(
            "normalizer/pattern5_break",
            "CorePlan construction complete (pipeline)",
        );
    }

    Ok(loop_carriers::with_loop_carriers(loop_plan, loop_carriers))
}
