//! Tail call policy helpers (SSOT)
//!
//! Responsibilities:
//! - Identify entry-like source blocks (LoopEntry vs BackEdge)
//! - Record latch incoming in one place for BackEdge

use crate::mir::builder::control_flow::joinir::merge::boundary_carrier_layout::BoundaryCarrierLayout;
use crate::mir::builder::control_flow::joinir::merge::contract_checks::is_entry_like_source;
use crate::mir::builder::control_flow::joinir::merge::loop_header_phi_info::LoopHeaderPhiInfo;
use crate::mir::builder::control_flow::joinir::merge::rewriter::latch_incoming_recorder;
use crate::mir::builder::control_flow::joinir::merge::tail_call_classifier::TailCallKind;
use crate::mir::join_ir::lowering::inline_boundary::JoinInlineBoundary;
use crate::mir::{BasicBlock, BasicBlockId, MirInstruction, ValueId};

/// Entry-like source is MAIN's blocks.
pub(super) fn is_loop_entry_source(
    func_name: &str,
    _old_block_id: BasicBlockId,
    _func_entry_block: BasicBlockId,
) -> bool {
    is_entry_like_source(func_name)
}

/// Record latch incoming for BackEdge using copies in the rewritten block.
pub(super) fn record_latch_incoming_if_backedge(
    tail_call_kind: TailCallKind,
    boundary: Option<&JoinInlineBoundary>,
    new_block_id: BasicBlockId,
    args: &[ValueId],
    new_block: &BasicBlock,
    loop_header_phi_info: &mut LoopHeaderPhiInfo,
) {
    if tail_call_kind != TailCallKind::BackEdge {
        return;
    }

    let Some(boundary) = boundary else { return };

    let mut latch_args: Vec<ValueId> = Vec::new();
    let mut loop_var_updated = false;

    let mut other_phi_dsts: std::collections::BTreeSet<ValueId> = std::collections::BTreeSet::new();
    if let Some(loop_var) = boundary.loop_var_name.as_deref() {
        for (name, entry) in loop_header_phi_info.carrier_phis.iter() {
            if name.as_str() != loop_var {
                other_phi_dsts.insert(entry.phi_dst);
            }
        }
    }
    let layout = BoundaryCarrierLayout::from_boundary(boundary);
    let ordered_carriers = layout.ordered_names();

    for (idx, carrier_name) in ordered_carriers.iter().enumerate() {
        let phi_dst = match loop_header_phi_info.get_carrier_phi(carrier_name) {
            Some(dst) => dst,
            None => continue,
        };

        let mut chosen = None;
        for inst in new_block.instructions.iter().rev() {
            if let MirInstruction::Copy { dst, src } = inst {
                if *dst == phi_dst {
                    chosen = Some(*src);
                    if *src != *dst {
                        if boundary.loop_var_name.as_deref() == Some(*carrier_name) {
                            loop_var_updated = !other_phi_dsts.contains(src);
                        }
                        break;
                    }
                }
            }
        }

        if let Some(val) = chosen {
            latch_args.push(val);
        } else if let Some(arg) = args.get(idx) {
            if boundary.loop_var_name.as_deref() == Some(*carrier_name)
                && *arg != phi_dst
                && !other_phi_dsts.contains(arg)
            {
                loop_var_updated = true;
            }
            latch_args.push(*arg);
        }
    }

    if !loop_var_updated && boundary.loop_var_name.is_some() {
        return;
    }

    latch_incoming_recorder::record_if_backedge(
        tail_call_kind,
        Some(boundary),
        new_block_id,
        &latch_args,
        loop_header_phi_info,
    );
}
