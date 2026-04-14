//! Route-local cleanup for `LoopCondContinueOnly`.
//!
//! Scope:
//! - append the route-local fallthrough continue exit

use crate::mir::builder::control_flow::plan::{CorePlan, LoweredRecipe};
use crate::mir::builder::MirBuilder;
use crate::mir::ValueId;
use std::collections::BTreeMap;

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
    use crate::mir::builder::control_flow::plan::CoreExitPlan;

    #[test]
    fn cleanup_appends_continue_with_phi_args() {
        let mut builder = MirBuilder::new();
        let mut body_plans = vec![];
        let carrier_step_phis = BTreeMap::from([("i".to_string(), ValueId(1))]);
        let current_bindings = BTreeMap::from([("i".to_string(), ValueId(2))]);

        apply_fallthrough_continue_exit(
            &mut builder,
            &mut body_plans,
            &carrier_step_phis,
            &current_bindings,
            "[test] loop_cond_continue_only",
        )
        .expect("cleanup should append continue");

        assert!(matches!(
            body_plans.last(),
            Some(CorePlan::Exit(CoreExitPlan::ContinueWithPhiArgs { .. }))
        ));
    }
}
