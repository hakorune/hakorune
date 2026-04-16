//! Shared helpers for loop_cond* facts extraction.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::extractors::common_helpers::walk_stmt_list;

pub(in crate::mir::builder) fn planner_required_for_loop_cond() -> bool {
    let strict_or_dev = crate::config::env::joinir_dev::strict_enabled()
        || crate::config::env::joinir_dev_enabled();
    strict_or_dev && crate::config::env::joinir_dev::planner_required_enabled()
}

pub(in crate::mir::builder) fn branch_tail_is_continue(body: &[ASTNode]) -> bool {
    matches!(body.last(), Some(ASTNode::Continue { .. }))
}

pub(in crate::mir::builder) fn branch_tail_is_continue_flattened(body: &[ASTNode]) -> bool {
    let mut last_stmt = None;
    walk_stmt_list(body, |stmt| {
        last_stmt = Some(stmt);
        false
    });
    matches!(last_stmt, Some(ASTNode::Continue { .. }))
}
