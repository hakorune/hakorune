use super::super::ast::ExprV0;
use super::super::ast::StmtV0;
use super::{lower_stmt_list_with_vars, merge_var_maps, new_block, BridgeEnv, LoopContext};
use crate::mir::{BasicBlockId, MirFunction, ValueId};
use std::collections::BTreeMap;

pub(super) fn lower_if_stmt(
    f: &mut MirFunction,
    cur_bb: BasicBlockId,
    cond: &ExprV0,
    then_body: &[StmtV0],
    else_body: &Option<Vec<StmtV0>>,
    vars: &mut BTreeMap<String, ValueId>,
    loop_stack: &mut Vec<LoopContext>,
    env: &BridgeEnv,
) -> Result<BasicBlockId, String> {
    let (cval, cur) = super::expr::lower_expr_with_vars(env, f, cur_bb, cond, vars)?;
    let then_bb = new_block(f);
    let else_bb = new_block(f);
    let merge_bb = new_block(f);
    crate::mir::ssot::cf_common::set_branch(f, cur, cval, then_bb, else_bb);
    let base_vars = vars.clone();
    let mut then_vars = base_vars.clone();
    let tend = lower_stmt_list_with_vars(f, then_bb, then_body, &mut then_vars, loop_stack, env)?;
    let mut then_terminated = false;
    if let Some(bb) = f.get_block_mut(tend) {
        if !bb.is_terminated() {
            crate::mir::ssot::cf_common::set_jump(f, tend, merge_bb);
        } else {
            then_terminated = true;
        }
    }
    let mut else_vars = base_vars.clone();
    let (else_end_pred, else_terminated) = if let Some(elses) = else_body {
        let eend = lower_stmt_list_with_vars(f, else_bb, elses, &mut else_vars, loop_stack, env)?;
        let mut term = false;
        if let Some(bb) = f.get_block_mut(eend) {
            if !bb.is_terminated() {
                crate::mir::ssot::cf_common::set_jump(f, eend, merge_bb);
            } else {
                term = true;
            }
        }
        (eend, term)
    } else {
        crate::mir::ssot::cf_common::set_jump(f, else_bb, merge_bb);
        (else_bb, false)
    };
    // If both branches terminate (e.g., both return/throw), no merge or var-join is needed.
    if then_terminated && else_terminated {
        return Ok(merge_bb);
    }
    // If only one side reaches merge_bb, take that side's var map directly.
    // This avoids creating PHI inputs from non-predecessor blocks (e.g. continue/return branches).
    if then_terminated && !else_terminated {
        *vars = else_vars;
        return Ok(merge_bb);
    }
    if else_terminated && !then_terminated {
        *vars = then_vars;
        return Ok(merge_bb);
    }
    merge_var_maps(
        f,
        merge_bb,
        tend,
        else_end_pred,
        then_vars,
        else_vars,
        base_vars,
        vars,
    )?;
    Ok(merge_bb)
}
