//! Latch incoming recorder (SSOT)
//!
//! Phase 287 P2: Centralize "when is it legal to record latch_incoming" policy.
//!
//! Contract:
//! - Only record latch_incoming for `TailCallKind::BackEdge`
//! - Never record for LoopEntry (main → loop_step), to avoid overwriting the true latch

use crate::mir::builder::control_flow::joinir::merge::loop_header_phi_info::LoopHeaderPhiInfo;
use crate::mir::builder::control_flow::joinir::merge::tail_call_classifier::TailCallKind;
use crate::mir::builder::control_flow::joinir::merge::boundary_carrier_layout::BoundaryCarrierLayout;
use crate::mir::join_ir::lowering::inline_boundary::JoinInlineBoundary;
use crate::mir::{BasicBlockId, ValueId};
use crate::mir::builder::control_flow::joinir::merge::dev_log;
use crate::mir::builder::control_flow::joinir::trace;

pub(in crate::mir::builder::control_flow::joinir::merge) fn record_if_backedge(
    tail_call_kind: TailCallKind,
    boundary: Option<&JoinInlineBoundary>,
    new_block_id: BasicBlockId,
    args: &[ValueId],
    loop_header_phi_info: &mut LoopHeaderPhiInfo,
) {
    if tail_call_kind != TailCallKind::BackEdge {
        return;
    }

    let Some(boundary) = boundary else { return };

    if loop_header_phi_info
        .carrier_phis
        .values()
        .any(|entry| entry.latch_incoming.is_some())
    {
        return;
    }

    let layout = BoundaryCarrierLayout::from_boundary(boundary);
    let ordered_carriers = layout.ordered_names();

    if let Some(loop_var_name) = &boundary.loop_var_name {
        debug_assert!(
            !args.is_empty(),
            "Phase 29ae Fail-Fast: BackEdge latch args empty for loop var '{}'",
            loop_var_name
        );
        if dev_log::dev_enabled(false) {
            if let Some(entry) = loop_header_phi_info.carrier_phis.get(loop_var_name) {
                if let Some(&arg0) = args.first() {
                    if arg0 == entry.entry_incoming.1 {
                        trace::trace().stderr_if(
                            &format!(
                                "[joinir/latch] warn: loop_var '{}' latch arg matches entry_incoming {:?}",
                                loop_var_name, arg0
                            ),
                            true,
                        );
                    }
                }
            }
        }
        if let Some(idx) = layout.ordered_arg_index(loop_var_name) {
            if let Some(&latch_value) = args.get(idx) {
                loop_header_phi_info.set_latch_incoming(loop_var_name, new_block_id, latch_value);
            }
        }
    }

    // Other carriers (excluding loop_var)
    for (idx, carrier_name) in ordered_carriers.iter().enumerate() {
        if boundary.loop_var_name.as_deref() == Some(*carrier_name) {
            continue;
        }
        if let Some(&latch_value) = args.get(idx) {
            loop_header_phi_info.set_latch_incoming(carrier_name, new_block_id, latch_value);
        }
    }

    // Loop invariants: latch incoming is the PHI destination itself (preserve value).
    for (inv_name, _inv_host_id) in boundary.loop_invariants.iter() {
        if let Some(phi_dst) = loop_header_phi_info.get_carrier_phi(inv_name) {
            loop_header_phi_info.set_latch_incoming(inv_name, new_block_id, phi_dst);
        }
    }
}
