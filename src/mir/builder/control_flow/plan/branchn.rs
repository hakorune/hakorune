use crate::mir::ValueId;
use super::{CoreIfPlan, CorePlan, LoweredRecipe};

/// Phase 29at P1: BranchN plan (scaffold only)
#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct CoreBranchNPlan {
    /// Branch arms (2+)
    pub arms: Vec<CoreBranchArmPlan>,

    /// Else branch plans (optional)
    pub else_plans: Option<Vec<LoweredRecipe>>,
}

/// Phase 29at P1: BranchN arm plan
#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct CoreBranchArmPlan {
    /// Arm condition (ValueId reference)
    pub condition: ValueId,

    /// Arm plans
    pub plans: Vec<LoweredRecipe>,
}

pub(in crate::mir::builder) fn branchn_to_if_chain(
    branch_plan: CoreBranchNPlan,
) -> Result<LoweredRecipe, String> {
    if branch_plan.arms.len() < 2 {
        return Err("[lowerer] CorePlan::BranchN requires at least 2 arms".to_string());
    }

    let mut iter = branch_plan.arms.into_iter().rev();
    let last = iter
        .next()
        .ok_or_else(|| "[lowerer] CorePlan::BranchN has no arms".to_string())?;

    let mut chain = CorePlan::If(CoreIfPlan {
        condition: last.condition,
        then_plans: last.plans,
        else_plans: branch_plan.else_plans,
        joins: Vec::new(),
    });

    for arm in iter {
        chain = CorePlan::If(CoreIfPlan {
            condition: arm.condition,
            then_plans: arm.plans,
            else_plans: Some(vec![chain]),
            joins: Vec::new(),
        });
    }

    Ok(chain)
}
