#[cfg(feature = "rc-insertion-minimal")]
use std::collections::{HashMap, HashSet};

#[cfg(feature = "rc-insertion-minimal")]
use crate::mir::{BasicBlockId, MirFunction, MirInstruction, ValueId};

#[cfg(feature = "rc-insertion-minimal")]
use std::fmt;

#[cfg(feature = "rc-insertion-minimal")]
const RC_PHI_EDGE_MISMATCH_TAG: &str = "[freeze:contract][rc_insertion/phi_edge_mismatch]";

#[cfg(feature = "rc-insertion-minimal")]
#[derive(Debug, Clone)]
pub(super) struct RcPhiEdgeMismatch {
    func_name: String,
    cleanup_kind: &'static str,
    pred: BasicBlockId,
    target: BasicBlockId,
    reason: &'static str,
    value: Option<ValueId>,
}

#[cfg(feature = "rc-insertion-minimal")]
impl fmt::Display for RcPhiEdgeMismatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(value) = self.value {
            write!(
                f,
                "{} fn={} cleanup={} pred={} target={} reason={} value={}",
                RC_PHI_EDGE_MISMATCH_TAG,
                self.func_name,
                self.cleanup_kind,
                self.pred,
                self.target,
                self.reason,
                value
            )
        } else {
            write!(
                f,
                "{} fn={} cleanup={} pred={} target={} reason={}",
                RC_PHI_EDGE_MISMATCH_TAG,
                self.func_name,
                self.cleanup_kind,
                self.pred,
                self.target,
                self.reason
            )
        }
    }
}

#[cfg(feature = "rc-insertion-minimal")]
pub(super) fn fail_fast_rc_phi_edge_mismatch(err: RcPhiEdgeMismatch) -> ! {
    panic!("{}", err);
}

#[cfg(feature = "rc-insertion-minimal")]
pub(super) fn verify_rc_phi_edge_contracts(
    func_name: &str,
    func: &MirFunction,
    break_cleanup_values_by_block: &HashMap<BasicBlockId, Vec<ValueId>>,
    continue_cleanup_values_by_block: &HashMap<BasicBlockId, Vec<ValueId>>,
) -> Result<(), RcPhiEdgeMismatch> {
    verify_rc_phi_edge_contract_for_kind(func_name, func, "break", break_cleanup_values_by_block)?;
    verify_rc_phi_edge_contract_for_kind(
        func_name,
        func,
        "continue",
        continue_cleanup_values_by_block,
    )?;
    Ok(())
}

#[cfg(feature = "rc-insertion-minimal")]
fn verify_rc_phi_edge_contract_for_kind(
    func_name: &str,
    func: &MirFunction,
    cleanup_kind: &'static str,
    cleanup_values_by_block: &HashMap<BasicBlockId, Vec<ValueId>>,
) -> Result<(), RcPhiEdgeMismatch> {
    for (pred_bid, release_values) in cleanup_values_by_block {
        let release_set: HashSet<ValueId> = release_values.iter().copied().collect();
        if release_set.is_empty() {
            continue;
        }

        let Some(pred_block) = func.blocks.get(pred_bid) else {
            continue;
        };
        let Some(MirInstruction::Jump { target, edge_args }) = pred_block.terminator.as_ref()
        else {
            return Err(RcPhiEdgeMismatch {
                func_name: func_name.to_string(),
                cleanup_kind,
                pred: *pred_bid,
                target: *pred_bid,
                reason: "cleanup_pred_not_jump",
                value: None,
            });
        };

        let Some(target_block) = func.blocks.get(target) else {
            continue;
        };
        match cleanup_kind {
            "break" => {
                if !matches!(
                    target_block.terminator.as_ref(),
                    Some(MirInstruction::Return { .. })
                ) {
                    return Err(RcPhiEdgeMismatch {
                        func_name: func_name.to_string(),
                        cleanup_kind,
                        pred: *pred_bid,
                        target: *target,
                        reason: "break_target_not_return",
                        value: None,
                    });
                }
            }
            "continue" => {
                if !matches!(
                    target_block.terminator.as_ref(),
                    Some(MirInstruction::Branch { .. })
                ) {
                    return Err(RcPhiEdgeMismatch {
                        func_name: func_name.to_string(),
                        cleanup_kind,
                        pred: *pred_bid,
                        target: *target,
                        reason: "continue_target_not_branch",
                        value: None,
                    });
                }
            }
            _ => {}
        }

        if let Some(args) = edge_args.as_ref() {
            if !args.values.is_empty() {
                let conflict = args
                    .values
                    .iter()
                    .copied()
                    .find(|v| release_set.contains(v));
                return Err(RcPhiEdgeMismatch {
                    func_name: func_name.to_string(),
                    cleanup_kind,
                    pred: *pred_bid,
                    target: *target,
                    reason: "cleanup_edge_args_present",
                    value: conflict,
                });
            }
        }

        for inst in &target_block.instructions {
            let MirInstruction::Phi { inputs, .. } = inst else {
                // MIR convention: Phi instructions are block head contiguous.
                break;
            };
            for (incoming_pred, incoming_value) in inputs {
                if incoming_pred != pred_bid {
                    continue;
                }
                if release_set.contains(incoming_value) {
                    return Err(RcPhiEdgeMismatch {
                        func_name: func_name.to_string(),
                        cleanup_kind,
                        pred: *pred_bid,
                        target: *target,
                        reason: "cleanup_phi_input_released",
                        value: Some(*incoming_value),
                    });
                }
            }
        }
    }

    Ok(())
}
