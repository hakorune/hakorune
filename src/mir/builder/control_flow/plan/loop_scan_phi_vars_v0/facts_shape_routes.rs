use crate::ast::ASTNode;
use crate::mir::builder::control_flow::recipes::loop_scan_phi_vars_v0::LoopScanPhiVarsV0Recipe;

use super::facts_helpers::{
    is_if_stmt, is_inc_stmt, is_local_decl, is_local_init_zero, is_loop_with_break,
    is_loop_without_exit, is_var_step_stmt_nonconst,
};

pub(in crate::mir::builder) struct LoopScanPhiVarsShapeMatch {
    pub prefix_end: usize,
    pub nested_idx: usize,
    pub step_start: usize,
    pub recipe: LoopScanPhiVarsV0Recipe,
}

pub(in crate::mir::builder) fn try_match_loop_scan_phi_vars_len7_shape(
    body: &[ASTNode],
    loop_var: &str,
) -> Result<LoopScanPhiVarsShapeMatch, &'static str> {
    if !is_local_decl(&body[0]) {
        return Err("stmt0_not_local");
    }
    if !is_local_init_zero(&body[1]) {
        return Err("stmt1_not_local_init_zero");
    }
    if !is_local_decl(&body[2]) {
        return Err("stmt2_not_local_m");
    }
    if !is_local_init_zero(&body[3]) {
        return Err("stmt3_not_local_found_init_zero");
    }
    if !is_loop_with_break(&body[4]) {
        return Err("stmt4_not_loop_with_break");
    }
    if !is_if_stmt(&body[5]) {
        return Err("stmt5_not_if");
    }
    if !is_inc_stmt(&body[6], loop_var) {
        return Err("stmt6_not_inc");
    }

    Ok(LoopScanPhiVarsShapeMatch {
        prefix_end: 4,
        nested_idx: 4,
        step_start: 6,
        recipe: LoopScanPhiVarsV0Recipe {
            local_var_name_stmt: Some(body[0].clone()),
            local_j_stmt: body[1].clone(),
            local_m_stmt: body[2].clone(),
            local_found_stmt: Some(body[3].clone()),
            inner_loop_search: body[4].clone(),
            found_if_stmt: Some(body[5].clone()),
            step_inc_stmt: body[6].clone(),
        },
    })
}

pub(in crate::mir::builder) fn try_match_loop_scan_phi_vars_ext_shape01(
    body: &[ASTNode],
    loop_var: &str,
) -> Result<LoopScanPhiVarsShapeMatch, &'static str> {
    if !is_local_init_zero(&body[0]) {
        return Err("stmt0_not_local_j_init_zero_ext_shape01");
    }
    if !is_local_decl(&body[1]) {
        return Err("stmt1_not_local_m_ext_shape01");
    }
    if !is_loop_without_exit(&body[2]) {
        return Err("stmt2_not_loop_no_exit_ext_shape01");
    }
    if !is_var_step_stmt_nonconst(&body[3], loop_var) {
        return Err("stmt3_not_nonconst_var_step_ext_shape01");
    }

    Ok(LoopScanPhiVarsShapeMatch {
        prefix_end: 2,
        nested_idx: 2,
        step_start: 3,
        recipe: LoopScanPhiVarsV0Recipe {
            local_var_name_stmt: None,
            local_j_stmt: body[0].clone(),
            local_m_stmt: body[1].clone(),
            local_found_stmt: None,
            inner_loop_search: body[2].clone(),
            found_if_stmt: None,
            step_inc_stmt: body[3].clone(),
        },
    })
}
