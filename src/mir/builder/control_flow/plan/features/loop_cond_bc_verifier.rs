//! Route-local verifier for `LoopCondBreakContinue`.
//!
//! Scope:
//! - verify route-specific PHI closure invariants after materialization
//! - keep break/fallthrough contract checks out of the pipeline body

use crate::mir::builder::control_flow::plan::features::loop_cond_bc_phi_materializer::LoopCondBreakContinuePhiClosure;
use crate::mir::builder::control_flow::plan::{CoreExitPlan, CorePlan, LoweredRecipe};
use crate::mir::{BasicBlockId, ValueId};
use std::collections::BTreeMap;

pub(in crate::mir::builder) fn verify_loop_cond_break_continue_phi_closure(
    phi_closure: &LoopCondBreakContinuePhiClosure,
    body_plans: &[LoweredRecipe],
    break_phi_dsts: &BTreeMap<String, ValueId>,
    continue_target: BasicBlockId,
    header_bb: BasicBlockId,
    step_bb: BasicBlockId,
    use_header_continue_target: bool,
    body_exits_all_paths: bool,
    carrier_count: usize,
    error_prefix: &str,
) -> Result<(), String> {
    let expected_continue_target = if use_header_continue_target {
        header_bb
    } else {
        step_bb
    };
    if continue_target != expected_continue_target {
        return Err(format!(
            "{error_prefix}: continue_target {:?} != expected {:?}",
            continue_target, expected_continue_target
        ));
    }

    if break_phi_dsts.len() != carrier_count {
        return Err(format!(
            "{error_prefix}: break_phi_dsts len {} != carrier_count {}",
            break_phi_dsts.len(),
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

    for (name, final_value) in phi_closure.final_values() {
        let Some(expected) = break_phi_dsts.get(name) else {
            return Err(format!(
                "{error_prefix}: final_value carrier {:?} missing from break_phi_dsts",
                name
            ));
        };
        if expected != final_value {
            return Err(format!(
                "{error_prefix}: final_value for carrier {:?} {:?} != after phi {:?}",
                name, final_value, expected
            ));
        }
    }

    let expected_phi_count = if use_header_continue_target || body_exits_all_paths {
        carrier_count * 2
    } else {
        carrier_count * 3
    };
    if phi_closure.phis().len() != expected_phi_count {
        return Err(format!(
            "{error_prefix}: phi count {} != expected {}",
            phi_closure.phis().len(),
            expected_phi_count
        ));
    }

    if !body_exits_all_paths {
        let Some(CorePlan::Exit(CoreExitPlan::ContinueWithPhiArgs { phi_args, .. })) =
            body_plans.last()
        else {
            return Err(format!(
                "{error_prefix}: fallthrough body must end with ContinueWithPhiArgs"
            ));
        };
        if phi_args.len() != carrier_count {
            return Err(format!(
                "{error_prefix}: fallthrough phi_args len {} != carrier_count {}",
                phi_args.len(),
                carrier_count
            ));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::control_flow::plan::CorePhiInfo;

    fn dummy_phi(dst: u32) -> CorePhiInfo {
        CorePhiInfo {
            block: BasicBlockId(1),
            dst: ValueId(dst),
            inputs: vec![],
            tag: format!("phi_{dst}"),
        }
    }

    #[test]
    fn verifier_accepts_fallthrough_step_target_shape() {
        let closure = LoopCondBreakContinuePhiClosure::new(
            vec![dummy_phi(2), dummy_phi(3), dummy_phi(4)],
            vec![("i".to_string(), ValueId(4))],
        );
        let body_plans = vec![CorePlan::Exit(CoreExitPlan::ContinueWithPhiArgs {
            depth: 1,
            phi_args: vec![(ValueId(3), ValueId(5))],
        })];
        let break_phi_dsts = BTreeMap::from([("i".to_string(), ValueId(4))]);

        verify_loop_cond_break_continue_phi_closure(
            &closure,
            &body_plans,
            &break_phi_dsts,
            BasicBlockId(12),
            BasicBlockId(11),
            BasicBlockId(12),
            false,
            false,
            1,
            "loop_cond_break_continue",
        )
        .expect("verifier should accept");
    }

    #[test]
    fn verifier_rejects_missing_fallthrough_continue() {
        let closure = LoopCondBreakContinuePhiClosure::new(
            vec![dummy_phi(2), dummy_phi(3), dummy_phi(4)],
            vec![("i".to_string(), ValueId(4))],
        );
        let break_phi_dsts = BTreeMap::from([("i".to_string(), ValueId(4))]);

        let err = verify_loop_cond_break_continue_phi_closure(
            &closure,
            &[],
            &break_phi_dsts,
            BasicBlockId(12),
            BasicBlockId(11),
            BasicBlockId(12),
            false,
            false,
            1,
            "loop_cond_break_continue",
        )
        .expect_err("verifier must reject missing continue exit");
        assert!(err.contains("fallthrough body must end with ContinueWithPhiArgs"));
    }

    #[test]
    fn verifier_rejects_non_after_phi_final_value() {
        let closure = LoopCondBreakContinuePhiClosure::new(
            vec![dummy_phi(2), dummy_phi(3), dummy_phi(4)],
            vec![("i".to_string(), ValueId(2))],
        );
        let break_phi_dsts = BTreeMap::from([("i".to_string(), ValueId(4))]);

        let err = verify_loop_cond_break_continue_phi_closure(
            &closure,
            &[],
            &break_phi_dsts,
            BasicBlockId(11),
            BasicBlockId(11),
            BasicBlockId(12),
            true,
            true,
            1,
            "loop_cond_break_continue",
        )
        .expect_err("verifier must reject non-after-phi final value");
        assert!(err.contains("final_value for carrier"));
    }
}
