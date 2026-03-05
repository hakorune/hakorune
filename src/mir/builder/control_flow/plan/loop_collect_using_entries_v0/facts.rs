//! Facts for loop_collect_using_entries_v0 (one-shape, planner-required only).

use crate::ast::{ASTNode, BinaryOperator};
use crate::mir::builder::control_flow::plan::facts::no_exit_block::try_build_no_exit_block_recipe;
use crate::mir::builder::control_flow::plan::planner::Freeze;

use super::recipe::LoopCollectUsingEntriesV0Recipe;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct LoopCollectUsingEntriesV0Facts {
    pub loop_var: String,
    pub limit_var: String,
    pub condition: ASTNode,
    pub recipe: LoopCollectUsingEntriesV0Recipe,
}

fn release_enabled() -> bool {
    true
}

fn as_var_name(ast: &ASTNode) -> Option<&str> {
    match ast {
        ASTNode::Variable { name, .. } => Some(name),
        _ => None,
    }
}

fn is_loop_cond_var_lt_var(ast: &ASTNode) -> Option<(String, String)> {
    match ast {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left,
            right,
            ..
        } => Some((
            as_var_name(left.as_ref())?.to_string(),
            as_var_name(right.as_ref())?.to_string(),
        )),
        _ => None,
    }
}

fn declares_single_local(stmt: &ASTNode) -> Option<String> {
    let ASTNode::Local { variables, .. } = stmt else {
        return None;
    };
    if variables.len() != 1 {
        return None;
    }
    Some(variables[0].clone())
}

fn extract_step_var_from_tail(stmt: &ASTNode, loop_var: &str) -> Option<String> {
    let ASTNode::Assignment { target, value, .. } = stmt else {
        return None;
    };
    if as_var_name(target.as_ref()) != Some(loop_var) {
        return None;
    }
    Some(as_var_name(value.as_ref())?.to_string())
}

pub(in crate::mir::builder) fn try_extract_loop_collect_using_entries_v0_facts(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<Option<LoopCollectUsingEntriesV0Facts>, Freeze> {
    let debug = crate::config::env::joinir_dev::debug_enabled();
    let debug_reject = |reason: &str| {
        if debug {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[plan/reject_detail] box=loop_collect_using_entries_v0 reason={}",
                reason
            ));
        }
    };

    if !release_enabled() {
        debug_reject("planner_required_off");
        return Ok(None);
    }

    let Some((loop_var, limit_var)) = is_loop_cond_var_lt_var(condition) else {
        debug_reject("cond_not_var_lt_var");
        return Ok(None);
    };

    if body.len() < 4 {
        debug_reject("body_too_short");
        return Ok(None);
    }

    if body.iter().any(ASTNode::contains_non_local_exit) {
        debug_reject("contains_exit");
        return Ok(None);
    }

    let Some(step_var) = extract_step_var_from_tail(body.last().unwrap(), &loop_var) else {
        debug_reject("tail_not_loopvar_eq_stepvar");
        return Ok(None);
    };

    let Some(first_local) = declares_single_local(&body[0]) else {
        debug_reject("first_stmt_not_single_local");
        return Ok(None);
    };
    if first_local != step_var {
        debug_reject("step_var_not_first_local");
        return Ok(None);
    }

    let Some(second_local) = declares_single_local(&body[1]) else {
        debug_reject("second_stmt_not_single_local");
        return Ok(None);
    };
    if second_local == loop_var || second_local == step_var {
        debug_reject("second_local_conflict");
        return Ok(None);
    }

    if !matches!(body[2], ASTNode::If { else_body: Some(_), .. }) {
        debug_reject("missing_top_level_if_else");
        return Ok(None);
    }

    let Some(body_no_exit) = try_build_no_exit_block_recipe(body, true) else {
        debug_reject("no_exit_recipe_failed");
        return Ok(None);
    };

    Ok(Some(LoopCollectUsingEntriesV0Facts {
        loop_var,
        limit_var,
        condition: condition.clone(),
        recipe: LoopCollectUsingEntriesV0Recipe { body_no_exit },
    }))
}
