//! Route-local cleanup for `LoopCondContinueWithReturn`.
//!
//! Scope:
//! - append the route-local final `ContinueWithPhiArgs` closure

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
    use crate::mir::builder::control_flow::plan::{CoreEffectPlan, CoreExitPlan};
    use crate::mir::MirType;

    #[test]
    fn cleanup_appends_final_continue_exit() {
        let mut builder = MirBuilder::new();
        let header_phi = builder.alloc_typed(MirType::Integer);
        let step_phi = builder.alloc_typed(MirType::Integer);
        let current_value = builder.alloc_typed(MirType::Integer);
        let mut body_plans = vec![CorePlan::Effect(CoreEffectPlan::Copy {
            dst: current_value,
            src: header_phi,
        })];

        apply_fallthrough_continue_exit(
            &mut builder,
            &mut body_plans,
            &BTreeMap::from([("i".to_string(), step_phi)]),
            &BTreeMap::from([("i".to_string(), current_value)]),
            "loop_cond_continue_with_return",
        )
        .expect("cleanup should append continue exit");

        assert!(matches!(
            body_plans.last(),
            Some(CorePlan::Exit(CoreExitPlan::ContinueWithPhiArgs { .. }))
        ));
    }
}
