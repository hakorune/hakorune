use super::helpers::create_phi_bindings;
use super::{build_pattern1_coreloop, CoreEffectPlan, CorePlan, PlanNormalizer, LoweredRecipe};
use crate::mir::builder::control_flow::joinir::patterns::router::LoopPatternContext;
use crate::mir::builder::control_flow::plan::facts::pattern_starts_with_facts::PatternStartsWithFacts;
use crate::mir::builder::control_flow::plan::CoreExitPlan;
use crate::mir::builder::MirBuilder;
use crate::mir::{ConstValue, MirType};

pub(in crate::mir::builder) fn normalize_starts_with_minimal(
    builder: &mut MirBuilder,
    facts: &PatternStartsWithFacts,
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
    let phi_bindings = create_phi_bindings(&[(&facts.loop_var, loop_var_current)]);

    let (lhs, op, rhs, mut cond_effects) =
        PlanNormalizer::lower_compare_ast(&facts.mismatch_condition, builder, &phi_bindings)?;

    let cond_mismatch = builder.alloc_typed(MirType::Bool);
    cond_effects.push(CoreEffectPlan::Compare {
        dst: cond_mismatch,
        lhs,
        op,
        rhs,
    });

    let zero_val = builder.alloc_typed(MirType::Integer);
    cond_effects.push(CoreEffectPlan::Const {
        dst: zero_val,
        value: ConstValue::Integer(0),
    });
    cond_effects.push(CoreEffectPlan::ExitIf {
        cond: cond_mismatch,
        exit: CoreExitPlan::Return(Some(zero_val)),
    });

    loop_plan.body = cond_effects.into_iter().map(CorePlan::Effect).collect();
    loop_plan
        .frag
        .wires
        .retain(|wire| wire.from != loop_plan.body_bb);
    Ok(CorePlan::Loop(loop_plan))
}
