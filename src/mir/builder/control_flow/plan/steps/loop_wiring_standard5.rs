//! Step: Standard5 loop CFG wiring
//! (plan::steps SSOT)
//!
//! Input: CoreLoopFrame (from coreloop_skeleton) + cond_loop ValueId + edge_args
//! Output: Frag with branches/wires configured
//! Fail-Fast: None (pure construction)

use crate::mir::builder::control_flow::edgecfg::api::EdgeStub;
use crate::mir::builder::control_flow::plan::features::coreloop_frame::CoreLoopFrame;
use crate::mir::builder::control_flow::plan::features::edgecfg_stubs;
use crate::mir::EdgeArgs;

/// Helper: Create empty EdgeArgs with CarriersOnly layout.
///
/// 通常ケース用の便利関数。ContinueWithPhiArgs 等で値を積む場合は caller が作る。
pub fn empty_carriers_args() -> EdgeArgs {
    use crate::mir::join_ir::lowering::inline_boundary::JumpArgsLayout;
    EdgeArgs {
        layout: JumpArgsLayout::CarriersOnly,
        values: vec![],
    }
}

// =============================================================================
// Phase 2b-1: Internal Wires Only (for short-circuit header branches)
// =============================================================================

/// Extract Standard5 internal wires (body→step→header) for reuse.
///
/// Use this when header branches are generated externally (e.g., short-circuit).
/// Pipeline assembles Frag directly with custom branches + these wires.
///
/// # Contract
/// - Returns only the internal wires (body→step, step→header)
/// - Branches are NOT included (caller provides from lower_loop_header_cond)
/// - edge_args は caller が指定（通常は empty_carriers_args()）
pub fn build_standard5_internal_wires(frame: &CoreLoopFrame, edge_args: EdgeArgs) -> Vec<EdgeStub> {
    vec![
        edgecfg_stubs::build_loop_back_edge_with_args(
            frame.body_bb,
            frame.step_bb,
            edge_args.clone(),
        ),
        edgecfg_stubs::build_loop_back_edge_with_args(frame.step_bb, frame.header_bb, edge_args),
    ]
}
