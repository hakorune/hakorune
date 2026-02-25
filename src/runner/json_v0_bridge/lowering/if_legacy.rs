use super::super::ast::{ExprV0, StmtV0};
use super::{BridgeEnv, LoopContext};
use crate::mir::{BasicBlockId, MirFunction, ValueId};
use std::collections::BTreeMap;

/// Stage-B legacy encoding for `if !(cond) { ... }`:
///   1) `If { cond: Int(0), then: [] }`
///   2) `Expr Int(0)`
///   3) `Expr <cond>`
///   4) `Expr BlockExpr { prelude, tail }`
///
/// Re-associate the 4-statement sequence into a single normalized `If` statement.
pub(super) fn try_lower_stageb_legacy_if_not_stmt_quad(
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
    let Some(fourth) = stmts.get(idx + 3) else {
        return Ok(None);
    };

    let StmtV0::If {
        cond: ExprV0::Int { value },
        then,
        r#else,
    } = first
    else {
        return Ok(None);
    };
    if !is_int_zero(value) || !then.is_empty() {
        return Ok(None);
    }
    if let Some(else_body) = r#else {
        if !else_body.is_empty() {
            return Ok(None);
        }
    }

    let StmtV0::Expr {
        expr: ExprV0::Int { value: sentinel },
    } = second
    else {
        return Ok(None);
    };
    if !is_int_zero(sentinel) {
        return Ok(None);
    }

    let StmtV0::Expr { expr: cond_expr } = third else {
        return Ok(None);
    };

    let StmtV0::Expr {
        expr: ExprV0::BlockExpr { prelude, tail },
    } = fourth
    else {
        return Ok(None);
    };

    let mut then_body = prelude.clone();
    if let Ok(stmt) = serde_json::from_value::<StmtV0>(tail.clone()) {
        then_body.push(stmt);
    } else if let Ok(expr) = serde_json::from_value::<ExprV0>(tail.clone()) {
        then_body.push(StmtV0::Expr { expr });
    } else {
        return Err("stageb legacy if-not: invalid BlockExpr.tail".into());
    }

    let normalized_if = StmtV0::If {
        cond: ExprV0::Compare {
            op: "==".to_string(),
            lhs: Box::new(cond_expr.clone()),
            rhs: Box::new(ExprV0::Bool { value: false }),
        },
        then: then_body,
        r#else: None,
    };
    let next_bb = super::lower_stmt_with_vars(f, cur_bb, &normalized_if, vars, loop_stack, env)?;
    Ok(Some((next_bb, 4)))
}

fn is_int_zero(value: &serde_json::Value) -> bool {
    if let Some(n) = value.as_i64() {
        return n == 0;
    }
    value.as_str() == Some("0")
}
