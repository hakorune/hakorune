use super::super::ast::{ExprV0, StmtV0};
use super::{BridgeEnv, LoopContext};
use crate::mir::{BasicBlockId, MirFunction, ValueId};
use std::collections::BTreeMap;

pub(super) fn try_lower_stageb_legacy_while_stmt_triplet(
    f: &mut MirFunction,
    cur_bb: BasicBlockId,
    stmts: &[StmtV0],
    idx: usize,
    vars: &mut BTreeMap<String, ValueId>,
    loop_stack: &mut Vec<LoopContext>,
    env: &BridgeEnv,
) -> Result<Option<(BasicBlockId, usize)>, String> {
    let Some(first) = stmts.get(idx) else {
        return Ok(None);
    };
    let Some(second) = stmts.get(idx + 1) else {
        return Ok(None);
    };
    let Some(third) = stmts.get(idx + 2) else {
        return Ok(None);
    };

    let StmtV0::Expr {
        expr: ExprV0::Var { name },
    } = first
    else {
        return Ok(None);
    };
    if name != "while" {
        return Ok(None);
    }

    let StmtV0::Expr { expr: cond } = second else {
        return Ok(None);
    };

    let StmtV0::Expr {
        expr: ExprV0::BlockExpr { prelude, tail },
    } = third
    else {
        return Ok(None);
    };

    let mut body = prelude.clone();
    if let Ok(stmt) = serde_json::from_value::<StmtV0>(tail.clone()) {
        body.push(stmt);
    } else if let Ok(expr) = serde_json::from_value::<ExprV0>(tail.clone()) {
        body.push(StmtV0::Expr { expr });
    } else {
        return Err("stageb legacy while: invalid BlockExpr.tail".into());
    }

    let loop_stmt = StmtV0::Loop {
        cond: cond.clone(),
        body,
    };
    let next_bb = super::lower_stmt_with_vars(f, cur_bb, &loop_stmt, vars, loop_stack, env)?;
    Ok(Some((next_bb, 3)))
}
