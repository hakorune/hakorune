use super::{build_pattern1_coreloop, CoreEffectPlan, CorePlan, LoweredRecipe};
use crate::mir::basic_block::EdgeArgs;
use crate::mir::builder::control_flow::plan::edgecfg_facade::{BlockParams, Frag};
use crate::mir::builder::control_flow::plan::features::edgecfg_stubs;
use crate::mir::builder::control_flow::plan::features::loop_carriers::build_loop_phi_info;
use crate::mir::builder::control_flow::joinir::patterns::router::LoopPatternContext;
use crate::mir::builder::control_flow::plan::facts::pattern_escape_map_facts::{
    EscapeDefaultFacts, PatternEscapeMapFacts,
};
use crate::mir::builder::MirBuilder;
use crate::mir::join_ir::lowering::inline_boundary::JumpArgsLayout;
use crate::mir::{BinaryOp, CompareOp, ConstValue, Effect, EffectMask, MirType};
use std::collections::BTreeMap;

pub(in crate::mir::builder) fn normalize_escape_map_minimal(
    builder: &mut MirBuilder,
    facts: &PatternEscapeMapFacts,
    ctx: &LoopPatternContext,
) -> Result<LoweredRecipe, String> {
    if facts.cases.is_empty() {
        return Err("[normalizer] escape_map requires at least one case".to_string());
    }

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

    let out_lookup = builder.variable_ctx.variable_map.get(&facts.out_var).copied();
    let out_init = out_lookup.unwrap_or_else(|| builder.alloc_typed(MirType::String));

    let haystack = builder
        .variable_ctx
        .variable_map
        .get(&facts.haystack_var)
        .copied()
        .ok_or_else(|| {
            format!(
                "[normalizer] Haystack variable {} not found",
                facts.haystack_var
            )
        })?;

    let out_current = builder.alloc_typed(MirType::String);
    let out_param = builder.alloc_typed(MirType::String);
    let ch_value = builder.alloc_typed(MirType::String);
    let one_val = builder.alloc_typed(MirType::Integer);
    let loop_var_plus_one = builder.alloc_typed(MirType::Integer);

    loop_plan.phis.push(build_loop_phi_info(
        loop_plan.header_bb,
        loop_plan.preheader_bb,
        loop_plan.step_bb,
        out_current,
        out_init,
        out_param,
        format!("loop_acc_{}", facts.out_var),
    ));
    loop_plan
        .final_values
        .push((facts.out_var.clone(), out_current));

    if out_lookup.is_none() {
        if let Some((_, effects)) = loop_plan
            .block_effects
            .iter_mut()
            .find(|(block_id, _)| *block_id == loop_plan.preheader_bb)
        {
            effects.push(CoreEffectPlan::Const {
                dst: out_init,
                value: ConstValue::String("".to_string()),
            });
        }
    }

    let mut case_bbs = Vec::new();
    for _ in 0..facts.cases.len() {
        case_bbs.push(builder.next_block_id());
    }

    let mut check_bbs = Vec::new();
    for _ in 1..facts.cases.len() {
        check_bbs.push(builder.next_block_id());
    }

    let default_bb = builder.next_block_id();

    let mut cond_values = Vec::new();
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

    let cond0 = builder.alloc_typed(MirType::Bool);
    let match0 = builder.alloc_typed(MirType::String);
    body_plans.push(CorePlan::Effect(CoreEffectPlan::Const {
        dst: match0,
        value: ConstValue::String(facts.cases[0].match_lit.clone()),
    }));
    body_plans.push(CorePlan::Effect(CoreEffectPlan::Compare {
        dst: cond0,
        lhs: ch_value,
        op: CompareOp::Eq,
        rhs: match0,
    }));
    cond_values.push(cond0);

    let mut check_effects = Vec::new();
    for (idx, check_bb) in check_bbs.iter().enumerate() {
        let case_idx = idx + 1;
        let cond = builder.alloc_typed(MirType::Bool);
        let match_val = builder.alloc_typed(MirType::String);
        cond_values.push(cond);

        let effects = vec![
            CoreEffectPlan::Const {
                dst: match_val,
                value: ConstValue::String(facts.cases[case_idx].match_lit.clone()),
            },
            CoreEffectPlan::Compare {
                dst: cond,
                lhs: ch_value,
                op: CompareOp::Eq,
                rhs: match_val,
            },
        ];
        check_effects.push((*check_bb, effects));
    }

    let mut case_effects = Vec::new();
    let mut case_out_values = Vec::new();
    for (idx, case_bb) in case_bbs.iter().enumerate() {
        let replace_val = builder.alloc_typed(MirType::String);
        let out_case = builder.alloc_typed(MirType::String);
        case_out_values.push(out_case);

        let effects = vec![
            CoreEffectPlan::Const {
                dst: replace_val,
                value: ConstValue::String(facts.cases[idx].replace_lit.clone()),
            },
            CoreEffectPlan::BinOp {
                dst: out_case,
                lhs: out_current,
                op: BinaryOp::Add,
                rhs: replace_val,
            },
        ];
        case_effects.push((*case_bb, effects));
    }

    let default_out = builder.alloc_typed(MirType::String);
    let default_effects = match &facts.default_case {
        EscapeDefaultFacts::Literal(lit) => {
            let default_val = builder.alloc_typed(MirType::String);
            vec![
                CoreEffectPlan::Const {
                    dst: default_val,
                    value: ConstValue::String(lit.clone()),
                },
                CoreEffectPlan::BinOp {
                    dst: default_out,
                    lhs: out_current,
                    op: BinaryOp::Add,
                    rhs: default_val,
                },
            ]
        }
        EscapeDefaultFacts::Char => vec![CoreEffectPlan::BinOp {
            dst: default_out,
            lhs: out_current,
            op: BinaryOp::Add,
            rhs: ch_value,
        }],
    };

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

    let mut block_effects = Vec::new();
    block_effects.push((loop_plan.preheader_bb, preheader_effects));
    block_effects.push((loop_plan.header_bb, header_effects));
    block_effects.push((loop_plan.body_bb, Vec::new()));
    block_effects.extend(check_effects);
    block_effects.extend(case_effects);
    block_effects.push((default_bb, default_effects));
    block_effects.push((loop_plan.step_bb, step_effects));

    let empty_args = EdgeArgs {
        layout: JumpArgsLayout::CarriersOnly,
        values: vec![],
    };
    let mut branches = Vec::new();
    branches.push(edgecfg_stubs::build_loop_header_branch_with_args(
        loop_plan.header_bb,
        loop_plan.cond_loop,
        loop_plan.body_bb,
        empty_args.clone(),
        loop_plan.after_bb,
        empty_args.clone(),
    ));

    let first_else = check_bbs.first().copied().unwrap_or(default_bb);
    branches.push(edgecfg_stubs::build_branch_stub(
        loop_plan.body_bb,
        cond_values[0],
        case_bbs[0],
        empty_args.clone(),
        first_else,
        empty_args.clone(),
    ));

    for (idx, check_bb) in check_bbs.iter().enumerate() {
        let cond = cond_values[idx + 1];
        let then_target = case_bbs[idx + 1];
        let else_target = if idx + 1 < check_bbs.len() {
            check_bbs[idx + 1]
        } else {
            default_bb
        };
        branches.push(edgecfg_stubs::build_branch_stub(
            *check_bb,
            cond,
            then_target,
            empty_args.clone(),
            else_target,
            empty_args.clone(),
        ));
    }

    let mut wires = Vec::new();
    for (idx, case_bb) in case_bbs.iter().enumerate() {
        wires.push(edgecfg_stubs::build_loop_back_edge_with_args(
            *case_bb,
            loop_plan.step_bb,
            EdgeArgs {
                layout: JumpArgsLayout::ExprResultPlusCarriers,
                values: vec![case_out_values[idx]],
            },
        ));
    }
    wires.push(edgecfg_stubs::build_loop_back_edge_with_args(
        default_bb,
        loop_plan.step_bb,
        EdgeArgs {
            layout: JumpArgsLayout::ExprResultPlusCarriers,
            values: vec![default_out],
        },
    ));
    wires.push(edgecfg_stubs::build_loop_back_edge_with_args(
        loop_plan.step_bb,
        loop_plan.header_bb,
        empty_args.clone(),
    ));

    let mut block_params = BTreeMap::new();
    block_params.insert(
        loop_plan.step_bb,
        BlockParams {
            layout: JumpArgsLayout::ExprResultPlusCarriers,
            params: vec![out_param],
        },
    );

    loop_plan.body = body_plans;
    loop_plan.block_effects = block_effects;
    loop_plan.frag = Frag {
        entry: loop_plan.header_bb,
        block_params,
        exits: BTreeMap::new(),
        wires,
        branches,
    };

    Ok(CorePlan::Loop(loop_plan))
}
