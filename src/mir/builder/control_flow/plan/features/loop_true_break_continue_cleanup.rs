//! Route-local cleanup for `LoopTrueBreakContinue`.
//!
//! Scope:
//! - detect whether normal fallthrough still needs a continue exit
//! - append the route-local `ContinueWithPhiArgs` closure

use crate::mir::builder::control_flow::plan::{CorePlan, LoweredRecipe};
use crate::mir::builder::MirBuilder;
use crate::mir::ValueId;
use std::collections::BTreeMap;

pub(in crate::mir::builder) fn requires_fallthrough_continue(
    body_plans: &[LoweredRecipe],
    tail_is_return: bool,
) -> bool {
    !tail_is_return && !matches!(body_plans.last(), Some(CorePlan::Exit(_)))
}

pub(in crate::mir::builder) fn apply_fallthrough_continue_exit(
    builder: &mut MirBuilder,
    body_plans: &mut Vec<LoweredRecipe>,
    carrier_step_phis: &BTreeMap<String, ValueId>,
    current_bindings: &BTreeMap<String, ValueId>,
    error_prefix: &str,
) -> Result<(), String> {
    let exit = crate::mir::builder::control_flow::plan::parts::exit::build_continue_with_phi_args(
        builder,
        carrier_step_phis,
        current_bindings,
        error_prefix,
    )?;
    body_plans.push(CorePlan::Exit(exit));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::control_flow::plan::{CoreExitPlan, CoreEffectPlan};

    #[test]
    fn cleanup_requires_fallthrough_when_tail_is_not_exit() {
        let plans = vec![CorePlan::Effect(CoreEffectPlan::Copy {
            dst: ValueId(1),
            src: ValueId(2),
        })];

        assert!(requires_fallthrough_continue(&plans, false));
    }

    #[test]
    fn cleanup_skips_fallthrough_for_tail_return() {
        let plans = vec![CorePlan::Exit(CoreExitPlan::Return(Some(ValueId(1))))];

        assert!(!requires_fallthrough_continue(&plans, true));
    }
}
