use serde_json::{json, Value};
use syn::{Expr, Pat, Stmt};

use crate::exprs::{expr_to_json, unsupported_expr};
use crate::types::{item_kind, pat_name, type_name};

pub(crate) fn block_stmts_to_json(block: &syn::Block, tail_expr_returns: bool) -> Vec<Value> {
    block
        .stmts
        .iter()
        .enumerate()
        .filter_map(|(index, stmt)| {
            stmt_to_json(stmt, tail_expr_returns && index + 1 == block.stmts.len())
        })
        .collect::<Vec<_>>()
}

fn stmt_to_json(stmt: &Stmt, is_tail: bool) -> Option<Value> {
    match stmt {
        Stmt::Local(local) => {
            let name = pat_name(&local.pat).unwrap_or_else(|| "unsupported_pattern".to_string());
            let value = local
                .init
                .as_ref()
                .map(|init| expr_to_json(init.expr.as_ref()))
                .unwrap_or_else(|| unsupported_expr("let without initializer"));
            Some(json!({
                "kind": "Let",
                "name": name,
                "type": local_type(local),
                "value": value,
            }))
        }
        Stmt::Expr(Expr::Return(ret), _) => {
            if let Some(expr) = &ret.expr {
                Some(json!({"kind": "Return", "value": expr_to_json(expr.as_ref())}))
            } else {
                Some(json!({"kind": "Return"}))
            }
        }
        Stmt::Expr(Expr::If(if_expr), _) => Some(if_to_json(if_expr, is_tail)),
        Stmt::Expr(Expr::While(while_expr), _) => Some(while_to_json(while_expr)),
        Stmt::Expr(Expr::Loop(loop_expr), _) => Some(loop_to_json(loop_expr)),
        Stmt::Expr(Expr::ForLoop(_), _) => Some(json!({
            "kind": "Expr",
            "value": unsupported_expr("Rust for loop expression is out of v0 scope"),
        })),
        Stmt::Expr(Expr::Assign(assign), _) => Some(assign_to_json(assign)),
        Stmt::Expr(expr, None) if is_tail => Some(json!({
            "kind": "Return",
            "value": expr_to_json(expr),
        })),
        Stmt::Expr(expr, _) => Some(json!({
            "kind": "Expr",
            "value": expr_to_json(expr),
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

fn while_to_json(expr: &syn::ExprWhile) -> Value {
    json!({
        "kind": "While",
        "cond": expr_to_json(expr.cond.as_ref()),
        "body": block_stmts_to_json(&expr.body, false),
    })
}

fn loop_to_json(expr: &syn::ExprLoop) -> Value {
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
        "body": block_stmts_to_json(&expr.body, false),
    })
}

fn if_to_json(expr: &syn::ExprIf, tail_expr_returns: bool) -> Value {
    let else_body = match &expr.else_branch {
        Some((_, else_expr)) => match else_expr.as_ref() {
            Expr::Block(block) => block_stmts_to_json(&block.block, tail_expr_returns),
            Expr::If(nested_if) => vec![if_to_json(nested_if, tail_expr_returns)],
            _ => vec![json!({
                "kind": "Unsupported",
                "reason": "non-block else branch is out of v0 scope",
            })],
        },
        None => Vec::new(),
    };

    json!({
        "kind": "If",
        "cond": expr_to_json(expr.cond.as_ref()),
        "then": block_stmts_to_json(&expr.then_branch, tail_expr_returns),
        "else": else_body,
    })
}

fn assign_to_json(expr: &syn::ExprAssign) -> Value {
    json!({
        "kind": "Assign",
        "target": expr_to_json(expr.left.as_ref()),
        "value": expr_to_json(expr.right.as_ref()),
    })
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
