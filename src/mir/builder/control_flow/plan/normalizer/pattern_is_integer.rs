use super::helpers::create_phi_bindings;
use super::{build_pattern1_coreloop, CoreEffectPlan, CorePlan, PlanNormalizer, LoweredRecipe};
use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
use crate::mir::builder::control_flow::joinir::patterns::router::LoopPatternContext;
use crate::mir::builder::control_flow::plan::facts::pattern_is_integer_facts::PatternIsIntegerFacts;
use crate::mir::builder::control_flow::plan::CoreExitPlan;
use crate::mir::builder::MirBuilder;
use crate::mir::{ConstValue, MirType};

pub(in crate::mir::builder) fn normalize_is_integer_minimal(
    builder: &mut MirBuilder,
    facts: &PatternIsIntegerFacts,
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

    let digit_check = ASTNode::BinaryOp {
        operator: BinaryOperator::Equal,
        left: Box::new(facts.digit_call.clone()),
        right: Box::new(ASTNode::Literal {
            value: LiteralValue::Bool(false),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    };
    let (lhs, op, rhs, mut cond_effects) =
        PlanNormalizer::lower_compare_ast(&digit_check, builder, &phi_bindings)?;

    let cond_not_digit = builder.alloc_typed(MirType::Bool);
    cond_effects.push(CoreEffectPlan::Compare {
        dst: cond_not_digit,
        lhs,
        op,
        rhs,
    });
    let return_value = if facts.return_zero_on_fail {
        let return_id = builder.alloc_typed(MirType::Integer);
        cond_effects.push(CoreEffectPlan::Const {
            dst: return_id,
            value: ConstValue::Integer(0),
        });
        return_id
    } else {
        rhs
    };
    cond_effects.push(CoreEffectPlan::ExitIf {
        cond: cond_not_digit,
        exit: CoreExitPlan::Return(Some(return_value)),
    });

    loop_plan.body = cond_effects.into_iter().map(CorePlan::Effect).collect();
    loop_plan
        .frag
        .wires
        .retain(|wire| wire.from != loop_plan.body_bb);
    Ok(CorePlan::Loop(loop_plan))
}
