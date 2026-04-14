//! Route-local verifier for `LoopCondContinueOnly`.
//!
//! Scope:
//! - verify route-specific PHI closure invariants after materialization
//! - keep continue-only contract checks out of the pipeline body

use crate::mir::builder::control_flow::plan::features::loop_cond_co_phi_materializer::LoopCondContinueOnlyPhiClosure;
use crate::mir::builder::control_flow::plan::{CoreExitPlan, CorePlan, LoweredRecipe};
use crate::mir::BasicBlockId;

pub(in crate::mir::builder) fn verify_loop_cond_continue_only_phi_closure(
    phi_closure: &LoopCondContinueOnlyPhiClosure,
    body_plans: &[LoweredRecipe],
    continue_target: BasicBlockId,
    step_bb: BasicBlockId,
    carrier_count: usize,
    error_prefix: &str,
) -> Result<(), String> {
    if continue_target != step_bb {
        return Err(format!(
            "{error_prefix}: continue_target {:?} != step_bb {:?}",
            continue_target, step_bb
        ));
    }

    let Some(CorePlan::Exit(CoreExitPlan::ContinueWithPhiArgs { phi_args, .. })) =
        body_plans.last()
    else {
        return Err(format!(
            "{error_prefix}: continue-only body must end with ContinueWithPhiArgs"
        ));
    };
    if phi_args.len() != carrier_count {
        return Err(format!(
            "{error_prefix}: continue_exit phi_args len {} != carrier_count {}",
            phi_args.len(),
            carrier_count
        ));
    }

    if phi_closure.final_values().len() != carrier_count {
        return Err(format!(
            "{error_prefix}: final_values len {} != carrier_count {}",
            phi_closure.final_values().len(),
            carrier_count
        ));
    }

    let expected_phi_count = carrier_count * 2;
    if phi_closure.phis().len() != expected_phi_count {
        return Err(format!(
            "{error_prefix}: phi count {} != expected {}",
            phi_closure.phis().len(),
            expected_phi_count
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::control_flow::plan::CorePhiInfo;
    use crate::mir::ValueId;

    fn dummy_phi(dst: u32) -> CorePhiInfo {
        CorePhiInfo {
            block: BasicBlockId(1),
            dst: ValueId(dst),
            inputs: vec![],
            tag: format!("phi_{dst}"),
        }
    }

    #[test]
    fn loop_cond_continue_only_verifier_accepts_min_shape() {
        let closure = LoopCondContinueOnlyPhiClosure::new(
            vec![dummy_phi(2), dummy_phi(3)],
            vec![("i".to_string(), ValueId(2))],
        );
        let body_plans = vec![CorePlan::Exit(CoreExitPlan::ContinueWithPhiArgs {
            depth: 1,
            phi_args: vec![(ValueId(3), ValueId(4))],
        })];

        verify_loop_cond_continue_only_phi_closure(
            &closure,
            &body_plans,
            BasicBlockId(12),
            BasicBlockId(12),
            1,
            "loop_cond_continue_only",
        )
        .expect("verifier should accept");
    }

    #[test]
    fn loop_cond_continue_only_verifier_rejects_missing_continue_exit() {
        let closure = LoopCondContinueOnlyPhiClosure::new(
            vec![dummy_phi(2), dummy_phi(3)],
            vec![("i".to_string(), ValueId(2))],
        );
        let body_plans = vec![];

        let err = verify_loop_cond_continue_only_phi_closure(
            &closure,
            &body_plans,
            BasicBlockId(12),
            BasicBlockId(12),
            1,
            "loop_cond_continue_only",
        )
        .expect_err("verifier must reject missing continue exit");
        assert!(err.contains("must end with ContinueWithPhiArgs"));
    }

    #[test]
    fn loop_cond_continue_only_verifier_rejects_wrong_phi_count() {
        let closure =
            LoopCondContinueOnlyPhiClosure::new(vec![dummy_phi(2)], vec![("i".to_string(), ValueId(2))]);
        let body_plans = vec![CorePlan::Exit(CoreExitPlan::ContinueWithPhiArgs {
            depth: 1,
            phi_args: vec![(ValueId(3), ValueId(4))],
        })];

        let err = verify_loop_cond_continue_only_phi_closure(
            &closure,
            &body_plans,
            BasicBlockId(12),
            BasicBlockId(12),
            1,
            "loop_cond_continue_only",
        )
        .expect_err("verifier must reject wrong phi count");
        assert!(err.contains("phi count"));
    }
}
