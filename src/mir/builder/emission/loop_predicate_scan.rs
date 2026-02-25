//! Phase 269 P1: Pattern8 Bool Predicate Scan - Emission Entrypoint
//!
//! ## Purpose
//! Thin entrypoint for Pattern8 Frag construction and MIR terminator emission.
//! This module only handles terminator wiring via EdgeCFG Frag API.
//! Block allocation and value computation (len, substring, predicate call) are done by Pattern8.
//!
//! ## Critical Corrections (5 SSOT)
//! 1. Return in wires (not exits) - emit_frag() generates terminators from wires/branches only
//! 2. after_bb has no terminator - let subsequent AST lowering handle "return true"
//! 3. Frag assembly is direct field access (no with_* API)
//! 4. BranchStub/EdgeStub field names match current implementation
//! 5. Return Void (loop as statement, not expression)

use crate::mir::builder::MirBuilder;
use crate::mir::builder::control_flow::edgecfg::api::{
    BranchStub, EdgeStub, ExitKind, Frag,
};
use crate::mir::basic_block::{BasicBlockId, EdgeArgs};
use crate::mir::join_ir::lowering::inline_boundary::JumpArgsLayout;
use crate::mir::ValueId;

/// Emit Bool Predicate Scan EdgeCFG Fragment
///
/// ## Arguments
/// - `b`: MirBuilder (for emit_frag access to current_function)
/// - `header_bb`: Loop condition check block (i < len)
/// - `body_bb`: Substring + predicate call + fail branch
/// - `step_bb`: Increment i and jump back to header
/// - `after_bb`: Normal loop exit (no terminator - subsequent AST lowering handles it)
/// - `ret_false_bb`: Early exit Return(false) block
/// - `cond_loop`: ValueId for (i < len)
/// - `cond_fail`: ValueId for (not ok)
/// - `ret_false_val`: ValueId for false literal
///
/// ## Frag Structure
/// - **branches**:
///   1. header: cond_loop true→body, false→after
///   2. body: cond_fail true→ret_false, false→step
/// - **wires**:
///   - step → header (Normal Jump)
///   - ret_false_bb → Return(false) - **IN WIRES, NOT EXITS**
/// - **exits**: empty (no upward propagation in P1)
///
/// ## Returns
/// `Ok(())` - Frag emitted successfully
/// `Err` - emit_frag failed or current_function is None
pub(in crate::mir::builder) fn emit_bool_predicate_scan_edgecfg(
    b: &mut MirBuilder,
    header_bb: BasicBlockId,
    body_bb: BasicBlockId,
    step_bb: BasicBlockId,
    after_bb: BasicBlockId,
    ret_false_bb: BasicBlockId,
    cond_loop: ValueId,
    cond_fail: ValueId,
    ret_false_val: ValueId,
) -> Result<(), String> {
    // EdgeArgs::empty() helper
    let empty_args = EdgeArgs {
        layout: JumpArgsLayout::CarriersOnly,
        values: vec![],
    };

    // Return(false) arguments (contains value)
    let ret_false_args = EdgeArgs {
        layout: JumpArgsLayout::CarriersOnly,
        values: vec![ret_false_val],
    };

    // branches (BranchStub)
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
            cond_fail,
            ret_false_bb,
            empty_args.clone(),
            step_bb,
            empty_args.clone(),
        ),
    ];

    // wires (EdgeStub)
    let wires = vec![
        // step_bb → header_bb Jump (Normal)
        EdgeStub::new(step_bb, ExitKind::Normal, Some(header_bb), empty_args.clone()),
        // ret_false_bb Return(false) - THIS GOES IN WIRES!
        EdgeStub::new(ret_false_bb, ExitKind::Return, None, ret_false_args),
    ];

    // Frag assembly (direct field access - no with_* API exists)
    let mut frag = Frag::new(header_bb);
    frag.branches = branches;
    frag.wires = wires;
    // exits is empty (no upward propagation in P1)

    // emit_frag generates MIR terminators (Phase 29bq+: session 経由)
    if let Some(ref mut func) = b.scope_ctx.current_function {
        b.frag_emit_session.emit_and_seal(func, &frag)?;
    } else {
        return Err("[emit_bool_predicate_scan_edgecfg] current_function is None".to_string());
    }

    Ok(())
}
