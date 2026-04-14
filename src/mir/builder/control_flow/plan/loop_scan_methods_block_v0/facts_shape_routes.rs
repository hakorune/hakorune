use crate::ast::ASTNode;

use super::facts_helpers::{
    declares_local_var, extract_next_i_from_tail, match_next_i_guard, match_scan_window_block,
    scan_window_substring_receiver,
};

pub(super) struct LoopScanMethodsBlockShapeMatch {
    pub next_i_var: String,
}

pub(super) fn try_match_loop_scan_methods_block_shape(
    body: &[ASTNode],
    loop_var: &str,
    limit_var: &str,
) -> Result<LoopScanMethodsBlockShapeMatch, String> {
    if body.len() < 6 {
        return Err("body_too_short".to_string());
    }

    if !declares_local_var(&body[0], "next_i")
        || !declares_local_var(&body[1], "k")
        || !declares_local_var(&body[2], "name_start")
    {
        return Err("missing_required_locals".to_string());
    }

    let Some(last) = body.last() else {
        return Err("body_last_missing".to_string());
    };
    let Some(next_i_var) = extract_next_i_from_tail(last, loop_var) else {
        return Err("tail_not_i_eq_next_i".to_string());
    };

    let Some(prev) = body.get(body.len().saturating_sub(2)) else {
        return Err("body_too_short_for_tail_guard".to_string());
    };
    if !match_next_i_guard(prev, &next_i_var, loop_var) {
        return Err("tail_guard_shape".to_string());
    }

    if match_scan_window_block(&body[3], limit_var).is_none() {
        if let Some(recv) = scan_window_substring_receiver(&body[3]) {
            return Err(format!("scan_window_block_shape receiver={recv}"));
        }
        return Err("scan_window_block_shape".to_string());
    }

    Ok(LoopScanMethodsBlockShapeMatch { next_i_var })
}
