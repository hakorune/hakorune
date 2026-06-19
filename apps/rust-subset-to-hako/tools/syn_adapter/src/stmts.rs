use serde_json::{json, Value};
use syn::{BinOp, Expr, Pat, Stmt};

use crate::exprs::{expr_to_json_with_context, unsupported_expr, ExprContext};
use crate::types::{insert_pat_name_metadata, item_kind, pat_name, type_name};

pub(crate) fn block_stmts_to_json_with_context(
    block: &syn::Block,
    tail_expr_returns: bool,
    context: &ExprContext,
) -> Vec<Value> {
    block
        .stmts
        .iter()
        .enumerate()
        .filter_map(|(index, stmt)| {
            stmt_to_json(
                stmt,
                tail_expr_returns && index + 1 == block.stmts.len(),
                context,
            )
        })
        .collect::<Vec<_>>()
}

fn stmt_to_json(stmt: &Stmt, is_tail: bool, context: &ExprContext) -> Option<Value> {
    match stmt {
        Stmt::Local(local) => {
            let Some(_name) = pat_name(&local.pat) else {
                return Some(json!({
                    "kind": "Unsupported",
                    "reason": "unsupported let pattern out of v0 scope",
                }));
            };
            let value = local
                .init
                .as_ref()
                .map(|init| expr_to_json_with_context(init.expr.as_ref(), context))
                .unwrap_or_else(|| unsupported_expr("let without initializer"));
            let mut value = json!({
                "kind": "Let",
                "type": local_type(local),
                "value": value,
            });
            insert_pat_name_metadata(&mut value, &local.pat);
            Some(value)
        }
        Stmt::Expr(Expr::Return(ret), _) => {
            if let Some(expr) = &ret.expr {
                Some(
                    json!({"kind": "Return", "value": expr_to_json_with_context(expr.as_ref(), context)}),
                )
            } else {
                Some(json!({"kind": "Return"}))
            }
        }
        Stmt::Expr(Expr::If(if_expr), _) => Some(if_to_json(if_expr, is_tail, context)),
        Stmt::Expr(Expr::While(while_expr), _) => Some(while_to_json(while_expr, context)),
        Stmt::Expr(Expr::Loop(loop_expr), _) => Some(loop_to_json(loop_expr, context)),
        Stmt::Expr(Expr::ForLoop(_), _) => Some(json!({
            "kind": "Expr",
            "value": unsupported_expr("Rust for loop expression is out of v0 scope"),
        })),
        Stmt::Expr(Expr::Binary(binary), _) if compound_assign_op(&binary.op).is_some() => {
            Some(json!({
                "kind": "Unsupported",
                "reason": format!(
                    "Rust compound assignment expression is out of v0 scope: {}",
                    compound_assign_op(&binary.op).unwrap_or("unknown"),
                ),
            }))
        }
        Stmt::Expr(Expr::Assign(assign), _) => Some(assign_to_json(assign, context)),
        Stmt::Expr(expr, None) if is_tail => Some(json!({
            "kind": "Return",
            "value": expr_to_json_with_context(expr, context),
        })),
        Stmt::Expr(expr, _) => Some(json!({
            "kind": "Expr",
            "value": expr_to_json_with_context(expr, context),
        })),
        Stmt::Item(item) => Some(json!({
            "kind": "Unsupported",
            "reason": format!("nested item out of v0 scope: {}", item_kind(item)),
        })),
        Stmt::Macro(_) => Some(json!({
            "kind": "Unsupported",
            "reason": "statement macro out of v0 scope",
        })),
    }
}

fn while_to_json(expr: &syn::ExprWhile, context: &ExprContext) -> Value {
    json!({
        "kind": "While",
        "cond": expr_to_json_with_context(expr.cond.as_ref(), context),
        "body": block_stmts_to_json_with_context(&expr.body, false, context),
    })
}

fn loop_to_json(expr: &syn::ExprLoop, context: &ExprContext) -> Value {
    if block_has_break_or_continue(&expr.body) {
        return json!({
            "kind": "Unsupported",
            "reason": "loop with break/continue belongs to compiler Recipe/CorePlan backlog",
        });
    }

    json!({
        "kind": "While",
        "cond": {
            "kind": "Literal",
            "type": "bool",
            "value": true,
        },
        "body": block_stmts_to_json_with_context(&expr.body, false, context),
    })
}

fn if_to_json(expr: &syn::ExprIf, tail_expr_returns: bool, context: &ExprContext) -> Value {
    let else_body = match &expr.else_branch {
        Some((_, else_expr)) => match else_expr.as_ref() {
            Expr::Block(block) => {
                block_stmts_to_json_with_context(&block.block, tail_expr_returns, context)
            }
            Expr::If(nested_if) => vec![if_to_json(nested_if, tail_expr_returns, context)],
            _ => vec![json!({
                "kind": "Unsupported",
                "reason": "non-block else branch is out of v0 scope",
            })],
        },
        None => Vec::new(),
    };

    json!({
        "kind": "If",
        "cond": expr_to_json_with_context(expr.cond.as_ref(), context),
        "then": block_stmts_to_json_with_context(&expr.then_branch, tail_expr_returns, context),
        "else": else_body,
    })
}

fn assign_to_json(expr: &syn::ExprAssign, context: &ExprContext) -> Value {
    json!({
        "kind": "Assign",
        "target": expr_to_json_with_context(expr.left.as_ref(), context),
        "value": expr_to_json_with_context(expr.right.as_ref(), context),
    })
}

fn compound_assign_op(op: &BinOp) -> Option<&'static str> {
    match op {
        BinOp::AddAssign(_) => Some("+="),
        BinOp::SubAssign(_) => Some("-="),
        BinOp::MulAssign(_) => Some("*="),
        BinOp::DivAssign(_) => Some("/="),
        BinOp::RemAssign(_) => Some("%="),
        BinOp::BitXorAssign(_) => Some("^="),
        BinOp::BitAndAssign(_) => Some("&="),
        BinOp::BitOrAssign(_) => Some("|="),
        BinOp::ShlAssign(_) => Some("<<="),
        BinOp::ShrAssign(_) => Some(">>="),
        _ => None,
    }
}

fn local_type(local: &syn::Local) -> String {
    match &local.pat {
        Pat::Type(pat_type) => type_name(pat_type.ty.as_ref()),
        _ => "Unknown".to_string(),
    }
}

fn block_has_break_or_continue(block: &syn::Block) -> bool {
    block.stmts.iter().any(stmt_has_break_or_continue)
}

fn stmt_has_break_or_continue(stmt: &Stmt) -> bool {
    match stmt {
        Stmt::Local(local) => local
            .init
            .as_ref()
            .map(|init| expr_has_break_or_continue(init.expr.as_ref()))
            .unwrap_or(false),
        Stmt::Expr(expr, _) => expr_has_break_or_continue(expr),
        Stmt::Item(_) | Stmt::Macro(_) => false,
    }
}

fn expr_has_break_or_continue(expr: &Expr) -> bool {
    match expr {
        Expr::Break(_) | Expr::Continue(_) => true,
        Expr::Block(block) => block_has_break_or_continue(&block.block),
        Expr::If(if_expr) => {
            block_has_break_or_continue(&if_expr.then_branch)
                || if_expr
                    .else_branch
                    .as_ref()
                    .map(|(_, else_expr)| expr_has_break_or_continue(else_expr.as_ref()))
                    .unwrap_or(false)
        }
        Expr::Loop(loop_expr) => block_has_break_or_continue(&loop_expr.body),
        Expr::While(while_expr) => block_has_break_or_continue(&while_expr.body),
        _ => false,
    }
}
