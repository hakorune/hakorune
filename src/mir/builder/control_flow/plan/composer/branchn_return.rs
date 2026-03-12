//! Phase 29at P3: BranchN match-return minimal composer

use crate::ast::LiteralValue;
use crate::mir::builder::control_flow::plan::branchn::CoreBranchArmPlan;
use crate::mir::builder::control_flow::plan::facts::MatchReturnFacts;
use crate::mir::builder::control_flow::plan::features::exit_branch;
use crate::mir::builder::control_flow::plan::{
    CoreBranchNPlan, CoreEffectPlan, CorePlan, LoweredRecipe,
};
use crate::mir::builder::MirBuilder;
use crate::mir::{CompareOp, ConstValue, MirType, ValueId};

pub(in crate::mir::builder) struct MatchReturnPlan {
    pub core_plan: LoweredRecipe,
    pub return_value: ValueId,
}

pub(in crate::mir::builder) fn compose_match_return_branchn(
    builder: &mut MirBuilder,
    facts: &MatchReturnFacts,
) -> Result<MatchReturnPlan, String> {
    let mut effects = Vec::new();

    let scrutinee_id = match &facts.scrutinee {
        crate::mir::builder::control_flow::plan::facts::MatchReturnScrutinee::Var(name) => builder
            .variable_ctx
            .require(name, "match_return_scrutinee")?,
        crate::mir::builder::control_flow::plan::facts::MatchReturnScrutinee::Int(value) => {
            let (value_id, effect) = alloc_const_effect(builder, &LiteralValue::Integer(*value))?;
            effects.push(effect);
            value_id
        }
    };

    let mut arms = Vec::with_capacity(facts.arms.len());
    for arm in &facts.arms {
        let (label_id, label_effect) = alloc_const_effect(builder, &arm.label)?;
        effects.push(label_effect);

        let cond_id = builder.next_value_id();
        effects.push(CoreEffectPlan::Compare {
            dst: cond_id,
            lhs: scrutinee_id,
            op: CompareOp::Eq,
            rhs: label_id,
        });

        let (ret_id, ret_effect) = alloc_const_effect(builder, &arm.return_value)?;
        effects.push(ret_effect);

        arms.push(CoreBranchArmPlan {
            condition: cond_id,
            plans: vec![exit_branch::build_return_only(ret_id)],
        });
    }

    let (else_value_id, else_effect) = alloc_const_effect(builder, &facts.else_value)?;
    effects.push(else_effect);

    let branch_plan = CoreBranchNPlan {
        arms,
        else_plans: Some(vec![exit_branch::build_return_only(else_value_id)]),
    };

    let mut plans: Vec<LoweredRecipe> = effects.into_iter().map(CorePlan::Effect).collect();
    plans.push(CorePlan::BranchN(branch_plan));
    let core_plan = if plans.len() == 1 {
        plans.pop().expect("branch plan")
    } else {
        CorePlan::Seq(plans)
    };

    Ok(MatchReturnPlan {
        core_plan,
        return_value: else_value_id,
    })
}

fn alloc_const_effect(
    builder: &mut MirBuilder,
    literal: &LiteralValue,
) -> Result<(ValueId, CoreEffectPlan), String> {
    let dst = builder.next_value_id();
    let const_value = literal_to_const(literal)?;
    register_literal_type(builder, dst, literal);
    Ok((
        dst,
        CoreEffectPlan::Const {
            dst,
            value: const_value,
        },
    ))
}

fn literal_to_const(literal: &LiteralValue) -> Result<ConstValue, String> {
    match literal {
        LiteralValue::Integer(n) => Ok(ConstValue::Integer(*n)),
        LiteralValue::Bool(b) => Ok(ConstValue::Bool(*b)),
        LiteralValue::String(s) => Ok(ConstValue::String(s.clone())),
        LiteralValue::Float(f) => Ok(ConstValue::Float(*f)),
        LiteralValue::Null => Ok(ConstValue::Null),
        LiteralValue::Void => Ok(ConstValue::Void),
    }
}

fn register_literal_type(builder: &mut MirBuilder, dst: ValueId, literal: &LiteralValue) {
    match literal {
        LiteralValue::Integer(_) => {
            builder.type_ctx.set_type(dst, MirType::Integer);
        }
        LiteralValue::Bool(_) => {
            builder.type_ctx.set_type(dst, MirType::Bool);
        }
        LiteralValue::Float(_) => {
            builder.type_ctx.set_type(dst, MirType::Float);
        }
        LiteralValue::String(_) => {
            builder
                .type_ctx
                .set_type(dst, MirType::Box("StringBox".to_string()));
            builder
                .type_ctx
                .value_origin_newbox
                .insert(dst, "StringBox".to_string());
        }
        LiteralValue::Null | LiteralValue::Void => {
            builder.type_ctx.set_type(dst, MirType::Void);
        }
    }
}
