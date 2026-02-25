use super::{build_pattern1_coreloop, CoreEffectPlan, CorePlan, LoweredRecipe};
use crate::mir::basic_block::EdgeArgs;
use crate::mir::builder::control_flow::plan::edgecfg_facade::{BlockParams, Frag};
use crate::mir::builder::control_flow::plan::features::edgecfg_stubs;
use crate::mir::builder::control_flow::joinir::patterns::router::LoopPatternContext;
use crate::mir::builder::control_flow::plan::facts::pattern_split_lines_facts::PatternSplitLinesFacts;
use crate::mir::builder::control_flow::plan::features::loop_carriers::build_loop_phi_info;
use crate::mir::builder::MirBuilder;
use crate::mir::join_ir::lowering::inline_boundary::JumpArgsLayout;
use crate::mir::{BinaryOp, CompareOp, ConstValue, Effect, EffectMask, MirType};
use std::collections::BTreeMap;

pub(in crate::mir::builder) fn normalize_split_lines_minimal(
    builder: &mut MirBuilder,
    facts: &PatternSplitLinesFacts,
    ctx: &LoopPatternContext,
) -> Result<LoweredRecipe, String> {
    let mut loop_plan = build_pattern1_coreloop(
        builder,
        &facts.loop_var,
        &facts.loop_condition,
        &facts.loop_increment,
        ctx,
    )?;

    let loop_var_current = loop_plan
        .final_values
        .iter()
        .find(|(name, _)| name == &facts.loop_var)
        .map(|(_, value)| *value)
        .ok_or_else(|| {
            format!(
                "[normalizer] loop var {} missing from final_values",
                facts.loop_var
            )
        })?;

    let start_init = builder
        .variable_ctx
        .variable_map
        .get(&facts.start_var)
        .copied()
        .ok_or_else(|| format!("[normalizer] Start var {} not found", facts.start_var))?;

    let result_host = builder
        .variable_ctx
        .variable_map
        .get(&facts.result_var)
        .copied()
        .ok_or_else(|| format!("[normalizer] Result var {} not found", facts.result_var))?;

    let haystack = builder
        .variable_ctx
        .variable_map
        .get(&facts.haystack_var)
        .copied()
        .ok_or_else(|| {
            format!(
                "[normalizer] Haystack var {} not found",
                facts.haystack_var
            )
        })?;

    let start_current = builder.alloc_typed(MirType::Integer);
    let start_param = builder.alloc_typed(MirType::Integer);
    let ch_value = builder.alloc_typed(MirType::String);
    let loop_var_plus_one = builder.alloc_typed(MirType::Integer);
    let one_val = builder.alloc_typed(MirType::Integer);
    let cond_match = builder.alloc_typed(MirType::Bool);
    let delim_val = builder.alloc_typed(MirType::String);
    let segment = builder.alloc_typed(MirType::String);

    loop_plan.phis.push(build_loop_phi_info(
        loop_plan.header_bb,
        loop_plan.preheader_bb,
        loop_plan.step_bb,
        start_current,
        start_init,
        start_param,
        format!("loop_start_{}", facts.start_var),
    ));
    loop_plan
        .final_values
        .push((facts.start_var.clone(), start_current));

    let mut body_plans = Vec::new();
    body_plans.push(CorePlan::Effect(CoreEffectPlan::Const {
        dst: one_val,
        value: ConstValue::Integer(1),
    }));
    body_plans.push(CorePlan::Effect(CoreEffectPlan::BinOp {
        dst: loop_var_plus_one,
        lhs: loop_var_current,
        op: BinaryOp::Add,
        rhs: one_val,
    }));
    body_plans.push(CorePlan::Effect(CoreEffectPlan::MethodCall {
        dst: Some(ch_value),
        object: haystack,
        method: "substring".to_string(),
        args: vec![loop_var_current, loop_var_plus_one],
        effects: EffectMask::PURE.add(Effect::Io),
    }));
    body_plans.push(CorePlan::Effect(CoreEffectPlan::Const {
        dst: delim_val,
        value: ConstValue::String(facts.delimiter_lit.clone()),
    }));
    body_plans.push(CorePlan::Effect(CoreEffectPlan::Compare {
        dst: cond_match,
        lhs: ch_value,
        op: CompareOp::Eq,
        rhs: delim_val,
    }));

    let then_bb = builder.next_block_id();
    let then_effects = vec![
        CoreEffectPlan::MethodCall {
            dst: Some(segment),
            object: haystack,
            method: "substring".to_string(),
            args: vec![start_current, loop_var_current],
            effects: EffectMask::PURE.add(Effect::Io),
        },
        CoreEffectPlan::MethodCall {
            dst: None,
            object: result_host,
            method: "push".to_string(),
            args: vec![segment],
            effects: EffectMask::MUT,
        },
    ];

    let mut preheader_effects = Vec::new();
    let mut header_effects = Vec::new();
    let mut step_effects = Vec::new();
    for (block_id, effects) in &loop_plan.block_effects {
        if *block_id == loop_plan.preheader_bb {
            preheader_effects = effects.clone();
        } else if *block_id == loop_plan.header_bb {
            header_effects = effects.clone();
        } else if *block_id == loop_plan.step_bb {
            step_effects = effects.clone();
        }
    }

    let block_effects = vec![
        (loop_plan.preheader_bb, preheader_effects),
        (loop_plan.header_bb, header_effects),
        (loop_plan.body_bb, Vec::new()),
        (then_bb, then_effects),
        (loop_plan.step_bb, step_effects),
    ];

    let empty_args = EdgeArgs {
        layout: JumpArgsLayout::CarriersOnly,
        values: vec![],
    };
    let join_args_no_match = EdgeArgs {
        layout: JumpArgsLayout::ExprResultPlusCarriers,
        values: vec![start_current],
    };
    let join_args_then = EdgeArgs {
        layout: JumpArgsLayout::ExprResultPlusCarriers,
        values: vec![loop_var_plus_one],
    };

    let branches = vec![
        edgecfg_stubs::build_loop_header_branch_with_args(
            loop_plan.header_bb,
            loop_plan.cond_loop,
            loop_plan.body_bb,
            empty_args.clone(),
            loop_plan.after_bb,
            empty_args.clone(),
        ),
        edgecfg_stubs::build_branch_stub(
            loop_plan.body_bb,
            cond_match,
            then_bb,
            empty_args.clone(),
            loop_plan.step_bb,
            join_args_no_match,
        ),
    ];

    let wires = vec![
        edgecfg_stubs::build_loop_back_edge_with_args(then_bb, loop_plan.step_bb, join_args_then),
        edgecfg_stubs::build_loop_back_edge_with_args(
            loop_plan.step_bb,
            loop_plan.header_bb,
            empty_args.clone(),
        ),
    ];

    let mut block_params = BTreeMap::new();
    block_params.insert(
        loop_plan.step_bb,
        BlockParams {
            layout: JumpArgsLayout::ExprResultPlusCarriers,
            params: vec![start_param],
        },
    );

    loop_plan.body = body_plans;
    loop_plan.block_effects = block_effects;
    loop_plan.cond_match = cond_match;
    loop_plan.frag = Frag {
        entry: loop_plan.header_bb,
        block_params,
        exits: BTreeMap::new(),
        wires,
        branches,
    };

    Ok(CorePlan::Loop(loop_plan))
}
