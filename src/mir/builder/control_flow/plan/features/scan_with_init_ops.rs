//! ScanWithInit feature ops (no AST re-parse, no skeleton allocation).

use crate::mir::builder::control_flow::plan::edgecfg_facade::{compose, Frag};
use crate::mir::builder::control_flow::joinir::patterns::router::LoopPatternContext;
use crate::mir::builder::control_flow::plan::features::{
    carrier_merge, edgecfg_stubs, loop_carriers, step_mode,
};
use crate::mir::builder::control_flow::plan::skeletons::scan_with_init::ScanWithInitSkeleton;
use crate::mir::builder::control_flow::plan::{
    CoreEffectPlan, CoreLoopPlan, CorePlan, ScanDirection,
    ScanWithInitPlan,
};
use crate::mir::builder::MirBuilder;
use crate::mir::{BinaryOp, CompareOp, ConstValue, Effect, EffectMask};
use std::collections::BTreeMap;

pub(in crate::mir::builder) fn build_scan_with_init_plan(
    builder: &mut MirBuilder,
    parts: &ScanWithInitPlan,
    ctx: &LoopPatternContext,
    skeleton: &ScanWithInitSkeleton,
) -> Result<CoreLoopPlan, String> {
    use crate::mir::builder::control_flow::joinir::trace;

    let trace_logger = trace::trace();
    let debug = ctx.debug;

    if debug {
        trace_logger.debug(
            "normalizer/scan_with_init",
            &format!(
                "Phase 29bv: ScanWithInit pipeline for {}",
                ctx.func_name
            ),
        );
    }

    match parts.scan_direction {
        ScanDirection::Forward if parts.step_lit != 1 => {
            return Err(format!(
                "[normalizer] scan_with_init: forward scan requires step=1 (step={})",
                parts.step_lit
            ));
        }
        ScanDirection::Reverse if parts.step_lit != -1 => {
            return Err(format!(
                "[normalizer] scan_with_init: reverse scan requires step=-1 (step={})",
                parts.step_lit
            ));
        }
        _ => {}
    }

    let s_host = builder
        .variable_ctx
        .variable_map
        .get(&parts.haystack)
        .copied()
        .ok_or_else(|| format!("[normalizer] Variable {} not found", parts.haystack))?;

    let needle_host = builder
        .variable_ctx
        .variable_map
        .get(&parts.needle)
        .copied()
        .ok_or_else(|| format!("[normalizer] Variable {} not found", parts.needle))?;

    let i_init_val = builder
        .variable_ctx
        .variable_map
        .get(&parts.loop_var)
        .copied()
        .ok_or_else(|| format!("[normalizer] Loop variable {} not found", parts.loop_var))?;

    let mut header_effects = vec![CoreEffectPlan::Const {
        dst: skeleton.one_val,
        value: ConstValue::Integer(1),
    }];

    if parts.dynamic_needle {
        header_effects.push(CoreEffectPlan::MethodCall {
            dst: Some(skeleton.needle_len_val),
            object: needle_host,
            method: "length".to_string(),
            args: vec![],
            effects: EffectMask::PURE.add(Effect::Io),
        });
    }

    match parts.scan_direction {
        ScanDirection::Forward => {
            header_effects.push(CoreEffectPlan::MethodCall {
                dst: Some(skeleton.len_val),
                object: s_host,
                method: "length".to_string(),
                args: vec![],
                effects: EffectMask::PURE.add(Effect::Io),
            });
            header_effects.push(CoreEffectPlan::BinOp {
                dst: skeleton.bound_val,
                lhs: skeleton.len_val,
                op: BinaryOp::Sub,
                rhs: skeleton.needle_len_val,
            });
            header_effects.push(CoreEffectPlan::Compare {
                dst: skeleton.cond_loop,
                lhs: skeleton.i_current,
                op: CompareOp::Le,
                rhs: skeleton.bound_val,
            });
        }
        ScanDirection::Reverse => {
            let zero_val = skeleton
                .zero_val
                .ok_or_else(|| "[normalizer] reverse scan requires zero value".to_string())?;
            header_effects.push(CoreEffectPlan::Const {
                dst: zero_val,
                value: ConstValue::Integer(0),
            });
            header_effects.push(CoreEffectPlan::Compare {
                dst: skeleton.cond_loop,
                lhs: skeleton.i_current,
                op: CompareOp::Ge,
                rhs: zero_val,
            });
        }
    }

    let body = vec![
        CorePlan::Effect(CoreEffectPlan::BinOp {
            dst: skeleton.i_plus_needle_len,
            lhs: skeleton.i_current,
            op: BinaryOp::Add,
            rhs: skeleton.needle_len_val,
        }),
        CorePlan::Effect(CoreEffectPlan::MethodCall {
            dst: Some(skeleton.window_val),
            object: s_host,
            method: "substring".to_string(),
            args: vec![skeleton.i_current, skeleton.i_plus_needle_len],
            effects: EffectMask::PURE.add(Effect::Io),
        }),
        CorePlan::Effect(CoreEffectPlan::Compare {
            dst: skeleton.cond_match,
            lhs: skeleton.window_val,
            op: CompareOp::Eq,
            rhs: needle_host,
        }),
    ];

    let step_effects = match parts.scan_direction {
        ScanDirection::Forward => vec![CoreEffectPlan::BinOp {
            dst: skeleton.i_next,
            lhs: skeleton.i_current,
            op: BinaryOp::Add,
            rhs: skeleton.one_val,
        }],
        ScanDirection::Reverse => vec![CoreEffectPlan::BinOp {
            dst: skeleton.i_next,
            lhs: skeleton.i_current,
            op: BinaryOp::Sub,
            rhs: skeleton.one_val,
        }],
    };

    let block_effects = vec![
        (skeleton.preheader_bb, vec![]),
        (skeleton.header_bb, header_effects.clone()),
        (skeleton.body_bb, vec![]),
        (skeleton.step_bb, step_effects.clone()),
    ];

    let loop_carriers = loop_carriers::build_loop_carriers(
        skeleton.header_bb,
        skeleton.preheader_bb,
        skeleton.step_bb,
        vec![loop_carriers::LoopCarrierSpec {
            dst: skeleton.i_current,
            init: i_init_val,
            next: skeleton.i_next,
            tag: format!("loop_carrier_{}", parts.loop_var),
        }],
    );

    let ret_found_args =
        edgecfg_stubs::build_return_args(vec![skeleton.i_current]);

    let main_branches = vec![
        edgecfg_stubs::build_loop_header_branch(
            skeleton.header_bb,
            skeleton.cond_loop,
            skeleton.body_bb,
            skeleton.after_bb,
        ),
        edgecfg_stubs::build_loop_cond_branch(
            skeleton.body_bb,
            skeleton.cond_match,
            skeleton.found_bb,
            skeleton.step_bb,
        ),
    ];

    let main_wires =
        vec![edgecfg_stubs::build_loop_back_edge(skeleton.step_bb, skeleton.header_bb)];

    let main_frag = Frag {
        entry: skeleton.header_bb,
        block_params: BTreeMap::new(),
        exits: BTreeMap::new(),
        wires: main_wires,
        branches: main_branches,
    };

    let cleanup_frag = Frag {
        entry: skeleton.found_bb,
        block_params: BTreeMap::new(),
        exits: edgecfg_stubs::build_single_return_exit_map(skeleton.found_bb, ret_found_args),
        wires: vec![],
        branches: vec![],
    };

    let frag = compose::cleanup(main_frag, cleanup_frag, None, None)
        .expect("compose::cleanup() failed in scan_with_init ops");

    let final_values = carrier_merge::build_final_values(vec![(
        parts.loop_var.as_str(),
        skeleton.i_current,
    )]);

    let (step_mode, has_explicit_step) = step_mode::extract_to_step_bb_explicit_step();

    let loop_plan = CoreLoopPlan {
        preheader_bb: skeleton.preheader_bb,
        preheader_is_fresh: false,
        header_bb: skeleton.header_bb,
        body_bb: skeleton.body_bb,
        step_bb: skeleton.step_bb,
        continue_target: skeleton.step_bb,
        after_bb: skeleton.after_bb,
        found_bb: skeleton.found_bb,
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
