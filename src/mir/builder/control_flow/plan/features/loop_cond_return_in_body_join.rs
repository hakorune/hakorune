//! Route-specific join contract for `LoopCondReturnInBody`.
//!
//! Scope:
//! - keep facts/recipe descriptive
//! - keep pipeline lowering mechanical
//! - isolate continue-join + loop-phi planning for this one route family

use crate::mir::builder::control_flow::plan::features::loop_carriers;
use crate::mir::builder::control_flow::plan::{CoreExitPlan, CorePhiInfo};
use crate::mir::{BasicBlockId, ValueId};
use std::collections::BTreeMap;

pub(in crate::mir::builder) struct LoopCondReturnInBodyJoinSig {
    continue_phi_args: Option<Vec<(ValueId, ValueId)>>,
    phis: Vec<CorePhiInfo>,
    final_values: Vec<(String, ValueId)>,
}

impl LoopCondReturnInBodyJoinSig {
    pub(in crate::mir::builder) fn continue_exit(&self) -> Option<CoreExitPlan> {
        self.continue_phi_args
            .clone()
            .map(|phi_args| CoreExitPlan::ContinueWithPhiArgs { depth: 1, phi_args })
    }

    pub(in crate::mir::builder) fn phis(&self) -> &[CorePhiInfo] {
        &self.phis
    }

    pub(in crate::mir::builder) fn final_values(&self) -> &[(String, ValueId)] {
        &self.final_values
    }
}

pub(in crate::mir::builder) fn build_loop_cond_return_in_body_join_sig(
    header_bb: BasicBlockId,
    preheader_bb: BasicBlockId,
    step_bb: BasicBlockId,
    use_header_continue_target: bool,
    body_exits_all_paths: bool,
    carrier_inits: &BTreeMap<String, ValueId>,
    carrier_phis: &BTreeMap<String, ValueId>,
    carrier_step_phis: &BTreeMap<String, ValueId>,
    current_bindings: &BTreeMap<String, ValueId>,
    error_prefix: &str,
) -> Result<LoopCondReturnInBodyJoinSig, String> {
    let continue_phi_args = if body_exits_all_paths {
        None
    } else {
        Some(collect_continue_phi_args(
            carrier_step_phis,
            current_bindings,
            error_prefix,
        )?)
    };

    let mut phis = Vec::new();
    let mut final_values = Vec::new();
    for (var, header_phi_dst) in carrier_phis {
        let Some(&init_val) = carrier_inits.get(var) else {
            continue;
        };
        if use_header_continue_target || body_exits_all_paths {
            phis.push(loop_carriers::build_preheader_only_phi_info(
                header_bb,
                preheader_bb,
                *header_phi_dst,
                init_val,
                format!("loop_cond_return_in_body_carrier_{}", var),
            ));
        } else {
            let Some(step_phi_dst) = carrier_step_phis.get(var).copied() else {
                continue;
            };
            phis.push(loop_carriers::build_step_join_phi_info(
                step_bb,
                step_phi_dst,
                format!("loop_cond_return_in_body_step_join_{}", var),
            ));
            phis.push(loop_carriers::build_loop_phi_info(
                header_bb,
                preheader_bb,
                step_bb,
                *header_phi_dst,
                init_val,
                step_phi_dst,
                format!("loop_cond_return_in_body_carrier_{}", var),
            ));
        }
        final_values.push((var.clone(), *header_phi_dst));
    }

    Ok(LoopCondReturnInBodyJoinSig {
        continue_phi_args,
        phis,
        final_values,
    })
}

fn collect_continue_phi_args(
    carrier_step_phis: &BTreeMap<String, ValueId>,
    current_bindings: &BTreeMap<String, ValueId>,
    error_prefix: &str,
) -> Result<Vec<(ValueId, ValueId)>, String> {
    let mut phi_args = Vec::new();
    for (name, phi_dst) in carrier_step_phis {
        let Some(&val) = current_bindings.get(name) else {
            return Err(format!(
                "{error_prefix}: step join value {} not found",
                name
            ));
        };
        phi_args.push((*phi_dst, val));
    }
    Ok(phi_args)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn singleton(name: &str, value: ValueId) -> BTreeMap<String, ValueId> {
        BTreeMap::from([(name.to_string(), value)])
    }

    #[test]
    fn join_sig_keeps_continue_and_step_phi_for_fallthrough_loop() {
        let sig = build_loop_cond_return_in_body_join_sig(
            BasicBlockId(10),
            BasicBlockId(11),
            BasicBlockId(12),
            false,
            false,
            &singleton("i", ValueId(1)),
            &singleton("i", ValueId(2)),
            &singleton("i", ValueId(3)),
            &singleton("i", ValueId(4)),
            "loop_cond_return_in_body",
        )
        .expect("join sig");

        assert!(matches!(
            sig.continue_exit(),
            Some(CoreExitPlan::ContinueWithPhiArgs {
                depth: 1,
                phi_args
            }) if phi_args == vec![(ValueId(3), ValueId(4))]
        ));
        assert_eq!(sig.phis().len(), 2);
        assert_eq!(sig.phis()[0].block, BasicBlockId(12));
        assert_eq!(sig.phis()[0].dst, ValueId(3));
        assert!(sig.phis()[0].inputs.is_empty());
        assert_eq!(sig.phis()[1].block, BasicBlockId(10));
        assert_eq!(
            sig.phis()[1].inputs,
            vec![
                (BasicBlockId(11), ValueId(1)),
                (BasicBlockId(12), ValueId(3))
            ]
        );
        assert_eq!(sig.final_values(), [("i".to_string(), ValueId(2))]);
    }

    #[test]
    fn join_sig_skips_continue_for_all_exit_body() {
        let sig = build_loop_cond_return_in_body_join_sig(
            BasicBlockId(10),
            BasicBlockId(11),
            BasicBlockId(12),
            false,
            true,
            &singleton("i", ValueId(1)),
            &singleton("i", ValueId(2)),
            &singleton("i", ValueId(3)),
            &singleton("i", ValueId(4)),
            "loop_cond_return_in_body",
        )
        .expect("join sig");

        assert!(sig.continue_exit().is_none());
        assert_eq!(sig.phis().len(), 1);
        assert_eq!(sig.phis()[0].block, BasicBlockId(10));
        assert_eq!(sig.phis()[0].inputs, vec![(BasicBlockId(11), ValueId(1))]);
        assert_eq!(sig.final_values(), [("i".to_string(), ValueId(2))]);
    }
}
