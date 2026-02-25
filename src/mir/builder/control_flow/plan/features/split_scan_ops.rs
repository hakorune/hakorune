//! SplitScan feature ops (no AST re-parse, no skeleton allocation).

use crate::mir::builder::control_flow::plan::edgecfg_facade::{
    compose, BlockParams, Frag,
};
use crate::mir::builder::control_flow::joinir::patterns::router::LoopPatternContext;
use crate::mir::builder::control_flow::plan::features::{
    carrier_merge, edgecfg_stubs, loop_carriers, split_emit, step_mode,
};
use crate::mir::builder::control_flow::plan::normalizer::common::empty_args;
use crate::mir::builder::control_flow::plan::skeletons::split_scan::SplitScanSkeleton;
use crate::mir::builder::control_flow::plan::{
    CoreEffectPlan, CoreLoopPlan, SplitScanPlan,
};
use crate::mir::builder::MirBuilder;
use crate::mir::join_ir::lowering::inline_boundary::JumpArgsLayout;
use crate::mir::{BinaryOp, CompareOp, ConstValue, Effect, EffectMask};
use std::collections::BTreeMap;

pub(in crate::mir::builder) fn build_split_scan_plan(
    builder: &mut MirBuilder,
    parts: &SplitScanPlan,
    ctx: &LoopPatternContext,
    skeleton: &SplitScanSkeleton,
) -> Result<CoreLoopPlan, String> {
    use crate::mir::builder::control_flow::joinir::trace;

    let trace_logger = trace::trace();
    let debug = ctx.debug;

    if debug {
        trace_logger.debug(
            "normalizer/split_scan",
            &format!(
                "Phase 29bv: SplitScan pipeline for {}",
                ctx.func_name
            ),
        );
    }

    let s_host = builder
        .variable_ctx
        .variable_map
        .get(&parts.s_var)
        .copied()
        .ok_or_else(|| format!("[normalizer] Variable {} not found", parts.s_var))?;

    let sep_host = builder
        .variable_ctx
        .variable_map
        .get(&parts.sep_var)
        .copied()
        .ok_or_else(|| format!("[normalizer] Variable {} not found", parts.sep_var))?;

    let result_host = builder
        .variable_ctx
        .variable_map
        .get(&parts.result_var)
        .copied()
        .ok_or_else(|| format!("[normalizer] Variable {} not found", parts.result_var))?;

    let i_init_val = builder
        .variable_ctx
        .variable_map
        .get(&parts.i_var)
        .copied()
        .ok_or_else(|| format!("[normalizer] Variable {} not found", parts.i_var))?;

    let start_init_val = builder
        .variable_ctx
        .variable_map
        .get(&parts.start_var)
        .copied()
        .ok_or_else(|| format!("[normalizer] Variable {} not found", parts.start_var))?;

    let header_effects = vec![
        CoreEffectPlan::MethodCall {
            dst: Some(skeleton.sep_len),
            object: sep_host,
            method: "length".to_string(),
            args: vec![],
            effects: EffectMask::PURE.add(Effect::Io),
        },
        CoreEffectPlan::MethodCall {
            dst: Some(skeleton.s_len),
            object: s_host,
            method: "length".to_string(),
            args: vec![],
            effects: EffectMask::PURE.add(Effect::Io),
        },
        CoreEffectPlan::BinOp {
            dst: skeleton.limit,
            lhs: skeleton.s_len,
            op: BinaryOp::Sub,
            rhs: skeleton.sep_len,
        },
        CoreEffectPlan::Compare {
            dst: skeleton.cond_loop,
            lhs: skeleton.i_current,
            op: CompareOp::Le,
            rhs: skeleton.limit,
        },
    ];

    let body = split_emit::build_match_body(
        s_host,
        sep_host,
        skeleton.i_current,
        skeleton.sep_len,
        skeleton.i_plus_sep,
        skeleton.chunk,
        skeleton.cond_match,
    );

    let then_effects = split_emit::build_then_effects(
        s_host,
        result_host,
        skeleton.start_current,
        skeleton.i_current,
        skeleton.sep_len,
        skeleton.segment,
        skeleton.start_next_then,
    );

    let else_effects = vec![
        CoreEffectPlan::Const {
            dst: skeleton.one,
            value: ConstValue::Integer(1),
        },
        CoreEffectPlan::BinOp {
            dst: skeleton.i_next_else,
            lhs: skeleton.i_current,
            op: BinaryOp::Add,
            rhs: skeleton.one,
        },
    ];

    let empty_args = empty_args();

    let then_join_args = loop_carriers::build_expr_carrier_join_args(vec![
        skeleton.start_next_then,
        skeleton.start_next_then,
    ]);
    let else_join_args = loop_carriers::build_expr_carrier_join_args(vec![
        skeleton.i_next_else,
        skeleton.start_current,
    ]);

    let then_frag = Frag {
        entry: skeleton.then_bb,
        block_params: BTreeMap::new(),
        exits: edgecfg_stubs::build_single_normal_exit_map(skeleton.then_bb, then_join_args),
        wires: vec![],
        branches: vec![],
    };

    let else_frag = Frag {
        entry: skeleton.else_bb,
        block_params: BTreeMap::new(),
        exits: edgecfg_stubs::build_single_normal_exit_map(skeleton.else_bb, else_join_args),
        wires: vec![],
        branches: vec![],
    };

    let mut join_block_params = BTreeMap::new();
    join_block_params.insert(
        skeleton.step_bb,
        BlockParams {
            layout: JumpArgsLayout::ExprResultPlusCarriers,
            params: vec![skeleton.i_next, skeleton.start_next],
        },
    );
    let join_frag = Frag {
        entry: skeleton.step_bb,
        block_params: join_block_params,
        exits: BTreeMap::new(),
        wires: vec![],
        branches: vec![],
    };

    let body_if_frag = compose::if_(
        skeleton.body_bb,
        skeleton.cond_match,
        then_frag,
        empty_args.clone(),
        else_frag,
        empty_args.clone(),
        join_frag,
    );

    let block_effects = vec![
        (skeleton.preheader_bb, vec![]),
        (skeleton.header_bb, header_effects.clone()),
        (skeleton.body_bb, vec![]),
        (skeleton.then_bb, then_effects),
        (skeleton.else_bb, else_effects),
        (skeleton.step_bb, vec![]),
    ];

    let loop_carriers = loop_carriers::build_loop_carriers(
        skeleton.header_bb,
        skeleton.preheader_bb,
        skeleton.step_bb,
        vec![
            loop_carriers::LoopCarrierSpec {
                dst: skeleton.i_current,
                init: i_init_val,
                next: skeleton.i_next,
                tag: format!("loop_carrier_i_{}", parts.i_var),
            },
            loop_carriers::LoopCarrierSpec {
                dst: skeleton.start_current,
                init: start_init_val,
                next: skeleton.start_next,
                tag: format!("loop_carrier_start_{}", parts.start_var),
            },
        ],
    );

    let mut branches = vec![edgecfg_stubs::build_loop_header_branch(
        skeleton.header_bb,
        skeleton.cond_loop,
        skeleton.body_bb,
        skeleton.after_bb,
    )];

    let Frag {
        block_params,
        branches: body_branches,
        wires: body_wires,
        exits: body_exits,
        ..
    } = body_if_frag;

    branches.extend(body_branches);

    let mut wires = Vec::new();
    wires.extend(body_wires);
    wires.push(edgecfg_stubs::build_loop_back_edge(
        skeleton.step_bb,
        skeleton.header_bb,
    ));

    let mut exits = BTreeMap::new();
    for (kind, stubs) in body_exits {
        exits.insert(kind, stubs);
    }

    let frag = Frag {
        entry: skeleton.header_bb,
        block_params,
        exits,
        wires,
        branches,
    };

    let final_values = carrier_merge::build_final_values(vec![
        (parts.i_var.as_str(), skeleton.i_current),
        (parts.start_var.as_str(), skeleton.start_current),
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
        body,
        cond_loop: skeleton.cond_loop,
        cond_match: skeleton.cond_match,
        block_effects,
        phis: Vec::new(),
        frag,
        final_values,
        step_mode,
        has_explicit_step,
    };

    Ok(loop_carriers::with_loop_carriers(loop_plan, loop_carriers))
}
