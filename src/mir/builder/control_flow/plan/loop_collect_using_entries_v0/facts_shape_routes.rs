use crate::ast::ASTNode;

use super::facts_helpers::{declares_single_local, extract_step_var_from_tail};

pub(super) fn try_match_loop_collect_using_entries_v0_shape(
    body: &[ASTNode],
    loop_var: &str,
) -> Result<(), String> {
    if body.len() < 4 {
        return Err("body_too_short".to_string());
    }

    if body.iter().any(ASTNode::contains_non_local_exit) {
        return Err("contains_exit".to_string());
    }

    let Some(last) = body.last() else {
        return Err("body_last_missing".to_string());
    };
    let Some(step_var) = extract_step_var_from_tail(last, loop_var) else {
        return Err("tail_not_loopvar_eq_stepvar".to_string());
    };

    let Some(first_local) = declares_single_local(&body[0]) else {
        return Err("first_stmt_not_single_local".to_string());
    };
    if first_local != step_var {
        return Err("step_var_not_first_local".to_string());
    }

    let Some(second_local) = declares_single_local(&body[1]) else {
        return Err("second_stmt_not_single_local".to_string());
    };
    if second_local == loop_var || second_local == step_var {
        return Err("second_local_conflict".to_string());
    }

    if !matches!(
        body[2],
        ASTNode::If {
            else_body: Some(_),
            ..
        }
    ) {
        return Err("missing_top_level_if_else".to_string());
    }

    Ok(())
}
