//! Step: Effects to LoweredRecipe conversion (plan::steps SSOT).

use crate::mir::builder::control_flow::plan::{CoreEffectPlan, CorePlan, LoweredRecipe};

/// Convert a Vec of CoreEffectPlan into a Vec of LoweredRecipe.
#[inline]
pub fn effects_to_plans(effects: Vec<CoreEffectPlan>) -> Vec<LoweredRecipe> {
    effects.into_iter().map(CorePlan::Effect).collect()
}
