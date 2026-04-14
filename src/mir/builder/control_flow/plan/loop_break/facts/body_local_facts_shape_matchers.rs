//! loop_break body-local promotion facts shape-specific matchers.

use crate::ast::ASTNode;

use super::body_local_facts::LoopBodyLocalShape;
mod body_local_common;
mod body_local_digit_matcher;
mod body_local_trim_matcher;

pub(super) fn try_match_trim_seg(
    break_condition: &ASTNode,
    body: &[ASTNode],
    break_idx: usize,
    loop_var: &str,
) -> Option<(String, LoopBodyLocalShape)> {
    body_local_trim_matcher::try_match_trim_seg(break_condition, body, break_idx, loop_var)
}

pub(super) fn try_match_digit_pos(
    break_condition: &ASTNode,
    body: &[ASTNode],
    break_idx: usize,
    loop_var: &str,
) -> Option<(String, LoopBodyLocalShape)> {
    body_local_digit_matcher::try_match_digit_pos(break_condition, body, break_idx, loop_var)
}
