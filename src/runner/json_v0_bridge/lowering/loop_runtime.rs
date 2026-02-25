use super::super::ast::{ExprV0, StmtV0};
use crate::mir::{BasicBlockId, MirFunction, ValueId};
use std::cell::RefCell;
use std::collections::BTreeMap;

// Snapshot stacks for loop break/continue (per-nested-loop frame)
thread_local! {
    static EXIT_SNAPSHOT_STACK: RefCell<Vec<Vec<(BasicBlockId, BTreeMap<String, ValueId>)>>> = RefCell::new(Vec::new());
    static CONT_SNAPSHOT_STACK: RefCell<Vec<Vec<(BasicBlockId, BTreeMap<String, ValueId>)>>> = RefCell::new(Vec::new());
    // Optional increment hint for current loop frame: (var_name, step)
    static INCR_HINT_STACK: RefCell<Vec<Option<(String, i64)>>> = RefCell::new(Vec::new());
}

pub(super) fn push_loop_snapshot_frames() {
    EXIT_SNAPSHOT_STACK.with(|s| s.borrow_mut().push(Vec::new()));
    CONT_SNAPSHOT_STACK.with(|s| s.borrow_mut().push(Vec::new()));
}

pub(super) fn pop_exit_snapshots() -> Vec<(BasicBlockId, BTreeMap<String, ValueId>)> {
    EXIT_SNAPSHOT_STACK.with(|s| s.borrow_mut().pop().unwrap_or_default())
}

pub(super) fn pop_continue_snapshots() -> Vec<(BasicBlockId, BTreeMap<String, ValueId>)> {
    CONT_SNAPSHOT_STACK.with(|s| s.borrow_mut().pop().unwrap_or_default())
}

pub(super) fn record_exit_snapshot(cur_bb: BasicBlockId, vars: &BTreeMap<String, ValueId>) {
    EXIT_SNAPSHOT_STACK.with(|s| {
        if let Some(top) = s.borrow_mut().last_mut() {
            top.push((cur_bb, vars.clone()));
        }
    });
}

pub(super) fn record_continue_snapshot(cur_bb: BasicBlockId, vars: &BTreeMap<String, ValueId>) {
    CONT_SNAPSHOT_STACK.with(|s| {
        if let Some(top) = s.borrow_mut().last_mut() {
            top.push((cur_bb, vars.clone()));
        }
    });
}

pub(super) fn detect_and_push_increment_hint(body: &[StmtV0]) {
    let mut hint: Option<(String, i64)> = None;
    for stmt in body.iter().rev() {
        if let StmtV0::Local { name, expr } = stmt.clone() {
            if let ExprV0::Binary { op, lhs, rhs } = expr {
                if let ExprV0::Var { name: vname } = *lhs {
                    if vname == name {
                        if let ExprV0::Int { value } = *rhs {
                            if let Some(v) = value.as_i64() {
                                let step = match op.as_str() {
                                    "+" => v,
                                    "-" => -v,
                                    _ => 0,
                                };
                                if step != 0 {
                                    hint = Some((name.clone(), step));
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    INCR_HINT_STACK.with(|s| s.borrow_mut().push(hint));
}

pub(super) fn pop_increment_hint() -> Option<(String, i64)> {
    INCR_HINT_STACK.with(|s| s.borrow_mut().pop().unwrap_or(None))
}

pub(super) fn peek_increment_hint() -> Option<(String, i64)> {
    INCR_HINT_STACK.with(|s| s.borrow().last().cloned().unwrap_or(None))
}

/// Small helper: set Jump terminator and record predecessor on the target.
fn jump_with_pred(f: &mut MirFunction, cur_bb: BasicBlockId, target: BasicBlockId) {
    // Delegate to SSOT CF helper for consistency
    crate::mir::ssot::cf_common::set_jump(f, cur_bb, target);
}

pub(super) fn lower_break_stmt(f: &mut MirFunction, cur_bb: BasicBlockId, exit_bb: BasicBlockId) {
    jump_with_pred(f, cur_bb, exit_bb);
    // ARCHIVED: JIT events moved to archive/jit-cranelift/ during Phase 15
    // crate::jit::events::emit_lower(
    //     serde_json::json!({ "id": "loop_break","exit_bb": exit_bb.0,"decision": "lower" }),
    //     "loop",
    //     "<json_v0>",
    // );
}

pub(super) fn lower_continue_stmt(
    f: &mut MirFunction,
    cur_bb: BasicBlockId,
    target_bb: BasicBlockId,
) {
    // target_bb は header か canonical continue_merge_bb のいずれか
    jump_with_pred(f, cur_bb, target_bb);
    // ARCHIVED: JIT events moved to archive/jit-cranelift/ during Phase 15
    // crate::jit::events::emit_lower(
    //     serde_json::json!({ "id": "loop_continue","cond_bb": cond_bb.0,"decision": "lower" }),
    //     "loop",
    //     "<json_v0>",
    // );
}
