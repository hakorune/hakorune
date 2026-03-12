use super::super::ast::{ExprV0, StmtV0};
use super::{BridgeEnv, LoopContext};
use crate::ast::{ASTNode, LiteralValue, Span};
use crate::mir::{BasicBlockId, MirFunction, MirInstruction, ValueId};
use std::collections::BTreeMap;

/// Stage-B legacy encoding: `fn(x) { ... }` is emitted as two adjacent statements:
///   1) `return fn(x)`
///   2) `{ ... }` as a standalone BlockExpr statement
///
/// This helper re-associates that pair into a single lambda literal value.
pub(super) fn try_lower_stageb_legacy_fn_literal_stmt_pair(
    f: &mut MirFunction,
    cur_bb: BasicBlockId,
    stmts: &[StmtV0],
    idx: usize,
    _vars: &mut BTreeMap<String, ValueId>,
    _loop_stack: &mut Vec<LoopContext>,
    _env: &BridgeEnv,
) -> Result<Option<(BasicBlockId, usize)>, String> {
    let Some(first) = stmts.get(idx) else {
        return Ok(None);
    };
    let Some(second) = stmts.get(idx + 1) else {
        return Ok(None);
    };

    let StmtV0::Return { expr } = first else {
        return Ok(None);
    };
    let ExprV0::Call { name, args } = expr else {
        return Ok(None);
    };
    if name != "fn" {
        return Ok(None);
    }

    let StmtV0::Expr { expr: be } = second else {
        return Ok(None);
    };
    let ExprV0::BlockExpr { prelude, tail } = be else {
        return Ok(None);
    };

    let params = stageb_fn_call_args_to_params(args)?;
    let mut body_stmts: Vec<StmtV0> = prelude.clone();
    let tail_stmt: StmtV0 = serde_json::from_value(tail.clone())
        .map_err(|_| "stageb legacy fn literal: invalid BlockExpr.tail stmt json")?;
    body_stmts.push(tail_stmt);
    let body = stageb_stmt_list_to_ast_lambda_body(&body_stmts)?;

    let dst = f.next_value_id();
    if let Some(bb) = f.get_block_mut(cur_bb) {
        bb.add_instruction(MirInstruction::NewClosure {
            dst,
            params,
            body_id: None,
            body,
            captures: vec![],
            me: None,
        });
        bb.set_terminator(MirInstruction::Return { value: Some(dst) });
    }

    Ok(Some((cur_bb, 2)))
}

fn stageb_fn_call_args_to_params(args: &[ExprV0]) -> Result<Vec<String>, String> {
    let mut params: Vec<String> = Vec::new();
    for a in args {
        match a {
            ExprV0::Var { name } => params.push(name.clone()),
            _ => {
                return Err(
                    "stageb legacy fn literal: expected param list as Vars in fn(...)".into(),
                );
            }
        }
    }
    Ok(params)
}

fn stageb_stmt_list_to_ast_lambda_body(stmts: &[StmtV0]) -> Result<Vec<ASTNode>, String> {
    let mut out: Vec<ASTNode> = Vec::new();
    for s in stmts {
        out.push(stageb_stmt_to_ast(s)?);
    }
    Ok(out)
}

fn stageb_stmt_to_ast(s: &StmtV0) -> Result<ASTNode, String> {
    match s {
        StmtV0::Return { expr } => Ok(ASTNode::Return {
            value: Some(Box::new(stageb_expr_to_ast(expr)?)),
            span: Span::unknown(),
        }),
        StmtV0::Expr { expr } => stageb_expr_to_ast(expr),
        StmtV0::Local { name, expr } => Ok(ASTNode::Local {
            variables: vec![name.clone()],
            initial_values: vec![Some(Box::new(stageb_expr_to_ast(expr)?))],
            span: Span::unknown(),
        }),
        _ => Err("stageb legacy fn literal: unsupported stmt in lambda body".into()),
    }
}

fn stageb_expr_to_ast(e: &ExprV0) -> Result<ASTNode, String> {
    match e {
        ExprV0::Var { name } => Ok(ASTNode::Variable {
            name: name.clone(),
            span: Span::unknown(),
        }),
        ExprV0::Int { value } => {
            let ival: i64 = if let Some(n) = value.as_i64() {
                n
            } else if let Some(s) = value.as_str() {
                s.parse()
                    .map_err(|_| "stageb legacy fn literal: invalid int literal")?
            } else {
                return Err("stageb legacy fn literal: invalid int literal".into());
            };
            Ok(ASTNode::Literal {
                value: LiteralValue::Integer(ival),
                span: Span::unknown(),
            })
        }
        ExprV0::Str { value } => Ok(ASTNode::Literal {
            value: LiteralValue::String(value.clone()),
            span: Span::unknown(),
        }),
        ExprV0::Bool { value } => Ok(ASTNode::Literal {
            value: LiteralValue::Bool(*value),
            span: Span::unknown(),
        }),
        ExprV0::Null => Ok(ASTNode::Literal {
            value: LiteralValue::Null,
            span: Span::unknown(),
        }),
        // Keep minimal: only the forms used by current selfhost fixtures.
        _ => Err("stageb legacy fn literal: unsupported expr in lambda body".into()),
    }
}
