//! Pattern7 Split Scan - EdgeCFG Frag Emission (Phase 272 P0.2)
//!
//! Purpose: Construct and emit Frag for Pattern7 (split scan with 2 carriers + side effect)
//!
//! CFG Structure (6 blocks):
//! ```
//! preheader_bb
//!     ↓ Jump
//! header_bb (PHI: i_current, start_current)
//!     ↓ Branch(cond_loop: i <= s.len - sep.len)
//!     ├─ true → body_bb
//!     │   ↓ Branch(cond_match: substring == sep)
//!     │   ├─ true → then_bb (result.push + updates)
//!     │   │   ↓ Jump → step_bb
//!     │   └─ false → else_bb (i++)
//!     │       ↓ Jump → step_bb
//!     └─ false → after_bb (post-loop)
//!
//! step_bb (PHI: i_next, start_next)
//!     ↓ Jump → header_bb (latch)
//! ```
//!
//! Frag Components:
//! - **2 branches**: header (loop condition), body (match condition)
//! - **3 wires**: then→step, else→step, step→header
//! - **0 exits**: after_bb handled by AST (no terminator)
//!
//! Phase 272 P0.2: Pattern7 Frag migration from JoinIRConversionPipeline

use crate::mir::builder::control_flow::edgecfg::api::{
    BranchStub, EdgeStub, ExitKind, Frag, FragEmitSession,
};
use crate::mir::basic_block::{BasicBlockId, EdgeArgs};
use crate::mir::join_ir::lowering::inline_boundary::JumpArgsLayout;
use crate::mir::{MirFunction, ValueId};

/// Emit EdgeCFG Frag for Pattern7 split scan loop
///
/// # CFG Invariants
/// - header_bb: 2 PHI nodes (i_current, start_current) must be inserted by caller
/// - step_bb: 2 PHI nodes (i_next, start_next) must be inserted by caller
/// - after_bb: No terminator (AST will handle post-loop code)
///
/// # Terminators Generated (SSOT via emit_frag)
/// - header_bb: Branch(cond_loop) → body_bb / after_bb
/// - body_bb: Branch(cond_match) → then_bb / else_bb
/// - then_bb: Jump → step_bb
/// - else_bb: Jump → step_bb
/// - step_bb: Jump → header_bb
///
/// # Side Effects
/// - result.push(segment) must be emitted in then_bb by caller (before calling this function)
///
/// # Arguments
/// - `func`: Current MirFunction (for emit_frag)
/// - `header_bb`: Loop header block (PHI merge point)
/// - `body_bb`: Match check block
/// - `then_bb`: Match found block (segment push + start update)
/// - `else_bb`: Match not found block (i increment)
/// - `step_bb`: Increment merge block (PHI for i_next, start_next)
/// - `after_bb`: Post-loop block (no terminator)
/// - `cond_loop`: Loop condition value (i <= limit)
/// - `cond_match`: Match condition value (chunk == sep)
#[allow(dead_code)]
pub(in crate::mir::builder) fn emit_split_scan_edgecfg(
    session: &mut FragEmitSession,
    func: &mut MirFunction,
    header_bb: BasicBlockId,
    body_bb: BasicBlockId,
    then_bb: BasicBlockId,
    else_bb: BasicBlockId,
    step_bb: BasicBlockId,
    after_bb: BasicBlockId,
    cond_loop: ValueId,
    cond_match: ValueId,
) -> Result<(), String> {
    // EdgeArgs: CarriersOnly layout (no explicit values, PHI handles propagation)
    let empty_args = EdgeArgs {
        layout: JumpArgsLayout::CarriersOnly,
        values: vec![],
    };

    // 2 branches: header (loop condition), body (match condition)
    let branches = vec![
        BranchStub::new(
            header_bb,
            cond_loop,
            body_bb,
            empty_args.clone(),
            after_bb,
            empty_args.clone(),
        ),
        BranchStub::new(
            body_bb,
            cond_match,
            then_bb,
            empty_args.clone(),
            else_bb,
            empty_args.clone(),
        ),
    ];

    // 3 wires (resolved internal edges): then→step, else→step, step→header
    let wires = vec![
        EdgeStub::new(then_bb, ExitKind::Normal, Some(step_bb), empty_args.clone()),
        EdgeStub::new(else_bb, ExitKind::Normal, Some(step_bb), empty_args.clone()),
        EdgeStub::new(step_bb, ExitKind::Normal, Some(header_bb), empty_args),
    ];

    // Construct Frag (no exits - after_bb is handled by AST)
    let mut frag = Frag::new(header_bb);
    frag.branches = branches;
    frag.wires = wires;
    // frag.exits remains empty (after_bb has no terminator)

    // Emit terminators (SSOT, Phase 29bq+: session 経由)
    session.emit_and_seal(func, &frag)?;

    Ok(())
}
