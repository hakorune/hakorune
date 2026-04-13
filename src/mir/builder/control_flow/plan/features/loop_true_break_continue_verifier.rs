//! Route-local verifier for `LoopTrueBreakContinue`.
//!
//! Scope:
//! - verify route-specific PHI closure invariants after materialization
//! - keep break/fallthrough contract checks out of the pipeline body

use crate::mir::builder::control_flow::plan::features::loop_true_break_continue_phi_materializer::LoopTrueBreakContinuePhiClosure;
use crate::mir::builder::control_flow::plan::{CoreExitPlan, CorePlan, LoweredRecipe};
use crate::mir::ValueId;
use std::collections::BTreeMap;

pub(in crate::mir::builder) fn verify_loop_true_break_continue_phi_closure(
    phi_closure: &LoopTrueBreakContinuePhiClosure,
    body_plans: &[LoweredRecipe],
    body_break_phi_dsts: Option<&BTreeMap<String, ValueId>>,
    body_after_phi_count: usize,
    carrier_count: usize,
    requires_fallthrough_continue: bool,
    error_prefix: &str,
) -> Result<(), String> {
    if phi_closure.final_values().len() != carrier_count {
        return Err(format!(
            "{error_prefix}: final_values len {} != carrier_count {}",
            phi_closure.final_values().len(),
            carrier_count
        ));
    }

    match body_break_phi_dsts {
        Some(break_phi_dsts) => {
            if break_phi_dsts.len() != carrier_count {
                return Err(format!(
                    "{error_prefix}: break_phi_dsts len {} != carrier_count {}",
                    break_phi_dsts.len(),
                    carrier_count
                ));
            }
            if body_after_phi_count != carrier_count {
                return Err(format!(
                    "{error_prefix}: after_phi_count {} != carrier_count {}",
                    body_after_phi_count,
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
                        "{error_prefix}: final_value for carrier {:?} {:?} != break-after phi {:?}",
                        name,
                        final_value,
                        expected
                    ));
                }
            }
        }
        None => {
            if body_after_phi_count != 0 {
                return Err(format!(
                    "{error_prefix}: after_phi_count {} without break lane",
                    body_after_phi_count
                ));
            }
        }
    }

    let expected_phi_count = (carrier_count * 2) + body_after_phi_count;
    if phi_closure.phis().len() != expected_phi_count {
        return Err(format!(
            "{error_prefix}: phi count {} != expected {}",
            phi_closure.phis().len(),
            expected_phi_count
        ));
    }

    if requires_fallthrough_continue {
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
    use crate::mir::BasicBlockId;

    fn dummy_phi(dst: u32) -> CorePhiInfo {
        CorePhiInfo {
            block: BasicBlockId(1),
            dst: ValueId(dst),
            inputs: vec![],
            tag: format!("phi_{dst}"),
        }
    }

    #[test]
    fn verifier_accepts_break_lane_with_fallthrough_continue() {
        let break_phi_dsts = BTreeMap::from([("i".to_string(), ValueId(8))]);
        let closure = LoopTrueBreakContinuePhiClosure::new(
            vec![dummy_phi(2), dummy_phi(3), dummy_phi(8)],
            vec![("i".to_string(), ValueId(8))],
        );
        let body_plans = vec![CorePlan::Exit(CoreExitPlan::ContinueWithPhiArgs {
            depth: 1,
            phi_args: vec![(ValueId(3), ValueId(4))],
        })];

        verify_loop_true_break_continue_phi_closure(
            &closure,
            &body_plans,
            Some(&break_phi_dsts),
            1,
            1,
            true,
            "loop_true_break_continue",
        )
        .expect("verifier should accept");
    }

    #[test]
    fn verifier_rejects_missing_fallthrough_continue() {
        let closure = LoopTrueBreakContinuePhiClosure::new(
            vec![dummy_phi(2), dummy_phi(3)],
            vec![("i".to_string(), ValueId(2))],
        );
        let body_plans = vec![];

        let err = verify_loop_true_break_continue_phi_closure(
            &closure,
            &body_plans,
            None,
            0,
            1,
            true,
            "loop_true_break_continue",
        )
        .expect_err("verifier must reject missing fallthrough continue");
        assert!(err.contains("fallthrough body must end with ContinueWithPhiArgs"));
    }

    #[test]
    fn verifier_rejects_non_break_final_value_on_break_lane() {
        let break_phi_dsts = BTreeMap::from([("i".to_string(), ValueId(8))]);
        let closure = LoopTrueBreakContinuePhiClosure::new(
            vec![dummy_phi(2), dummy_phi(3), dummy_phi(8)],
            vec![("i".to_string(), ValueId(2))],
        );
        let body_plans = vec![];

        let err = verify_loop_true_break_continue_phi_closure(
            &closure,
            &body_plans,
            Some(&break_phi_dsts),
            1,
            1,
            false,
            "loop_true_break_continue",
        )
        .expect_err("verifier must reject non-break final value");
        assert!(err.contains("final_value for carrier"));
    }
}
