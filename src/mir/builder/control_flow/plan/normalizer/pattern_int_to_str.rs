use super::{build_pattern1_coreloop, CoreEffectPlan, CorePlan, LoweredRecipe};
use crate::mir::builder::control_flow::joinir::patterns::router::LoopPatternContext;
use crate::mir::builder::control_flow::plan::facts::pattern_int_to_str_facts::PatternIntToStrFacts;
use crate::mir::builder::control_flow::plan::features::loop_carriers::build_loop_phi_info;
use crate::mir::builder::MirBuilder;
use crate::mir::{BinaryOp, ConstValue, Effect, EffectMask, MirType};

pub(in crate::mir::builder) fn normalize_int_to_str_minimal(
    builder: &mut MirBuilder,
    facts: &PatternIntToStrFacts,
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

    let out_lookup = builder.variable_ctx.variable_map.get(&facts.out_var).copied();
    let out_init = out_lookup.unwrap_or_else(|| builder.alloc_typed(MirType::String));
    let digits_lookup = builder.variable_ctx.variable_map.get(&facts.digits_var).copied();
    let digits = digits_lookup.unwrap_or_else(|| builder.alloc_typed(MirType::String));

    let out_current = builder.alloc_typed(MirType::String);
    let out_next = builder.alloc_typed(MirType::String);
    let digit_val = builder.alloc_typed(MirType::Integer);
    let digit_plus_one = builder.alloc_typed(MirType::Integer);
    let ch_val = builder.alloc_typed(MirType::String);
    let one_val = builder.alloc_typed(MirType::Integer);
    let ten_val = builder.alloc_typed(MirType::Integer);

    loop_plan.phis.push(build_loop_phi_info(
        loop_plan.header_bb,
        loop_plan.preheader_bb,
        loop_plan.step_bb,
        out_current,
        out_init,
        out_next,
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

    let Some((_, step_effects)) = loop_plan
        .block_effects
        .iter_mut()
        .find(|(block_id, _)| *block_id == loop_plan.step_bb)
    else {
        return Err("[normalizer] step_bb missing from block_effects".to_string());
    };

    let mut extra_effects = Vec::new();
    extra_effects.push(CoreEffectPlan::Const {
        dst: one_val,
        value: ConstValue::Integer(1),
    });
    extra_effects.push(CoreEffectPlan::Const {
        dst: ten_val,
        value: ConstValue::Integer(10),
    });
    if digits_lookup.is_none() {
        extra_effects.push(CoreEffectPlan::Const {
            dst: digits,
            value: ConstValue::String("0123456789".to_string()),
        });
    }
    extra_effects.push(CoreEffectPlan::BinOp {
        dst: digit_val,
        lhs: loop_var_current,
        op: BinaryOp::Mod,
        rhs: ten_val,
    });
    extra_effects.push(CoreEffectPlan::BinOp {
        dst: digit_plus_one,
        lhs: digit_val,
        op: BinaryOp::Add,
        rhs: one_val,
    });
    extra_effects.push(CoreEffectPlan::MethodCall {
        dst: Some(ch_val),
        object: digits,
        method: "substring".to_string(),
        args: vec![digit_val, digit_plus_one],
        effects: EffectMask::PURE.add(Effect::Io),
    });
    extra_effects.push(CoreEffectPlan::BinOp {
        dst: out_next,
        lhs: ch_val,
        op: BinaryOp::Add,
        rhs: out_current,
    });

    step_effects.splice(0..0, extra_effects);

    Ok(CorePlan::Loop(loop_plan))
}
