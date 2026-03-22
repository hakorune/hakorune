//! Step: Standard5 loop CFG wiring
//! (plan::steps SSOT)
//!
//! Input: CoreLoopFrame (from coreloop_skeleton) + cond_loop ValueId + edge_args
//! Output: Frag with branches/wires configured
//! Fail-Fast: None (pure construction)

use crate::mir::{BasicBlockId, EdgeArgs};
use crate::mir::builder::control_flow::plan::edgecfg_facade::{BranchStub, EdgeStub, Frag};
use crate::mir::builder::control_flow::plan::features::coreloop_frame::CoreLoopFrame;
use crate::mir::builder::control_flow::plan::features::edgecfg_stubs;
use crate::mir::ValueId;
use std::collections::BTreeMap;

/// Build Standard5 loop wiring (branches + internal wires).
///
/// # Contract
/// - Creates header → body/after branch
/// - Creates body → step → header internal wires
/// - Returns (branches, wires) for Frag construction
/// - Does NOT emit terminators (caller's responsibility)
/// - edge_args は caller が指定（将来 ContinueWithPhiArgs 等で値を積む場合に対応）
pub fn build_standard5_wiring(
    frame: &CoreLoopFrame,
    cond_loop: ValueId,
    edge_args: EdgeArgs,
) -> (Vec<BranchStub>, Vec<EdgeStub>) {
    // Header → body (then) / after (else)
    let branches = vec![edgecfg_stubs::build_loop_header_branch_with_args(
        frame.header_bb,
        cond_loop,
        frame.body_bb,
        edge_args.clone(),
        frame.after_bb,
        edge_args.clone(),
    )];

    // Internal wires: body → step → header
    let wires = vec![
        edgecfg_stubs::build_loop_back_edge_with_args(
            frame.body_bb,
            frame.step_bb,
            edge_args.clone(),
        ),
        edgecfg_stubs::build_loop_back_edge_with_args(frame.step_bb, frame.header_bb, edge_args),
    ];

    (branches, wires)
}

/// Build Standard5 Frag from wiring.
///
/// # Contract
/// - entry = header_bb
/// - No exits (all wired internally or via branches)
/// - No block_params (PHI handled separately)
pub fn build_standard5_frag(
    header_bb: BasicBlockId,
    branches: Vec<BranchStub>,
    wires: Vec<EdgeStub>,
) -> Frag {
    Frag {
        entry: header_bb,
        block_params: BTreeMap::new(),
        exits: BTreeMap::new(),
        wires,
        branches,
    }
}

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
