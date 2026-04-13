//! Route-local verifier for `LoopCondReturnInBody`.
//!
//! Scope:
//! - verify route-specific PHI closure invariants after materialization
//! - keep these checks out of the pipeline body and out of the generic loop verifier

use crate::mir::builder::control_flow::plan::features::loop_cond_return_in_body_phi_materializer::LoopCondReturnInBodyPhiClosure;
use crate::mir::builder::control_flow::plan::CoreExitPlan;
use crate::mir::BasicBlockId;

pub(in crate::mir::builder) fn verify_loop_cond_return_in_body_phi_closure(
    phi_closure: &LoopCondReturnInBodyPhiClosure,
    continue_target: BasicBlockId,
    use_header_continue_target: bool,
    body_exits_all_paths: bool,
    carrier_count: usize,
    error_prefix: &str,
) -> Result<(), String> {
    if phi_closure.continue_target() != continue_target {
        return Err(format!(
            "{error_prefix}: continue_target {:?} != expected {:?}",
            phi_closure.continue_target(),
            continue_target
        ));
    }

    match (body_exits_all_paths, phi_closure.continue_exit()) {
        (true, Some(_)) => {
            return Err(format!(
                "{error_prefix}: all-exit body must not emit continue_exit"
            ));
        }
        (false, None) => {
            return Err(format!(
                "{error_prefix}: fallthrough body must emit continue_exit"
            ));
        }
        (false, Some(CoreExitPlan::ContinueWithPhiArgs { phi_args, .. })) => {
            if phi_args.len() != carrier_count {
                return Err(format!(
                    "{error_prefix}: continue_exit phi_args len {} != carrier_count {}",
                    phi_args.len(),
                    carrier_count
                ));
            }
        }
        (false, Some(other)) => {
            return Err(format!(
                "{error_prefix}: unexpected continue_exit shape {:?}",
                other
            ));
        }
        (true, None) => {}
    }

    if phi_closure.final_values().len() != carrier_count {
        return Err(format!(
            "{error_prefix}: final_values len {} != carrier_count {}",
            phi_closure.final_values().len(),
            carrier_count
        ));
    }

    let expected_phi_count = if body_exits_all_paths || use_header_continue_target {
        carrier_count
    } else {
        carrier_count * 2
    };
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
    use crate::mir::builder::control_flow::plan::{CoreExitPlan, CorePhiInfo};
    use crate::mir::ValueId;

    fn closure(
        continue_exit: Option<CoreExitPlan>,
        phis: Vec<CorePhiInfo>,
        final_values: Vec<(String, ValueId)>,
        continue_target: BasicBlockId,
    ) -> LoopCondReturnInBodyPhiClosure {
        LoopCondReturnInBodyPhiClosure::new(continue_exit, phis, final_values, continue_target)
    }

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
        let closure = closure(
            Some(CoreExitPlan::ContinueWithPhiArgs {
                depth: 1,
                phi_args: vec![(ValueId(3), ValueId(4))],
            }),
            vec![dummy_phi(2), dummy_phi(3)],
            vec![("i".to_string(), ValueId(2))],
            BasicBlockId(12),
        );

        verify_loop_cond_return_in_body_phi_closure(
            &closure,
            BasicBlockId(12),
            false,
            false,
            1,
            "loop_cond_return_in_body",
        )
        .expect("verifier should accept");
    }

    #[test]
    fn verifier_rejects_missing_fallthrough_continue_exit() {
        let closure = closure(
            None,
            vec![dummy_phi(2), dummy_phi(3)],
            vec![("i".to_string(), ValueId(2))],
            BasicBlockId(12),
        );

        let err = verify_loop_cond_return_in_body_phi_closure(
            &closure,
            BasicBlockId(12),
            false,
            false,
            1,
            "loop_cond_return_in_body",
        )
        .expect_err("verifier must reject missing continue_exit");
        assert!(err.contains("fallthrough body must emit continue_exit"));
    }

    #[test]
    fn verifier_rejects_all_exit_continue_edge() {
        let closure = closure(
            Some(CoreExitPlan::ContinueWithPhiArgs {
                depth: 1,
                phi_args: vec![(ValueId(3), ValueId(4))],
            }),
            vec![dummy_phi(2)],
            vec![("i".to_string(), ValueId(2))],
            BasicBlockId(10),
        );

        let err = verify_loop_cond_return_in_body_phi_closure(
            &closure,
            BasicBlockId(10),
            true,
            true,
            1,
            "loop_cond_return_in_body",
        )
        .expect_err("verifier must reject all-exit continue");
        assert!(err.contains("all-exit body must not emit continue_exit"));
    }
}
