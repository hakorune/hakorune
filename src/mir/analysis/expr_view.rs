//! Analysis-only expression/statement views (no AST rewrite).
//!
//! Goal: provide conservative, reusable matchers for policies / planner facts.
//! - Do not transform the AST.
//! - Reject shapes that would require control-flow reasoning.

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Commute {
    AsWritten,
    Swapped,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelfUpdateOp {
    Add,
    Sub,
}

#[derive(Debug, Clone)]
pub struct BinOpView<'a> {
    pub operator: BinaryOperator,
    pub left: &'a ASTNode,
    pub right: &'a ASTNode,
}

#[derive(Debug, Clone)]
pub struct BlockExprView<'a> {
    pub prelude_stmts: &'a [ASTNode],
    pub tail_expr: &'a ASTNode,
}

#[derive(Debug, Clone)]
pub struct AssignmentView<'a> {
    pub target: &'a ASTNode,
    pub value: &'a ASTNode,
}

#[derive(Debug, Clone)]
pub struct SelfUpdateByConstView<'a> {
    pub var_name: &'a str,
    pub op: SelfUpdateOp,
    pub step: i64,
    pub commute: Commute,
}

#[derive(Debug, Clone)]
pub struct SelfUpdateAssignByConstView<'a> {
    pub target_var: &'a str,
    pub rhs: SelfUpdateByConstView<'a>,
}

pub fn match_var(expr: &ASTNode) -> Option<&str> {
    match expr {
        ASTNode::Variable { name, .. } => Some(name.as_str()),
        _ => None,
    }
}

pub fn match_int(expr: &ASTNode) -> Option<i64> {
    match expr {
        ASTNode::Literal {
            value: LiteralValue::Integer(n),
            ..
        } => Some(*n),
        _ => None,
    }
}

pub fn match_binop(expr: &ASTNode, operator: BinaryOperator) -> Option<BinOpView<'_>> {
    match expr {
        ASTNode::BinaryOp {
            operator: op,
            left,
            right,
            ..
        } if op == &operator => Some(BinOpView {
            operator: operator.clone(),
            left: left.as_ref(),
            right: right.as_ref(),
        }),
        _ => None,
    }
}

pub fn match_blockexpr(expr: &ASTNode) -> Option<BlockExprView<'_>> {
    match expr {
        ASTNode::BlockExpr {
            prelude_stmts,
            tail_expr,
            ..
        } => Some(BlockExprView {
            prelude_stmts: prelude_stmts.as_slice(),
            tail_expr: tail_expr.as_ref(),
        }),
        _ => None,
    }
}

pub fn match_assignment(node: &ASTNode) -> Option<AssignmentView<'_>> {
    match node {
        ASTNode::Assignment { target, value, .. } => Some(AssignmentView {
            target: target.as_ref(),
            value: value.as_ref(),
        }),
        _ => None,
    }
}

pub fn match_assignment_to_var(node: &ASTNode) -> Option<(&str, &ASTNode)> {
    let assign = match_assignment(node)?;
    let name = match_var(assign.target)?;
    Some((name, assign.value))
}

/// Match `var_name + step` OR (if allowed) `step + var_name`.
/// This is analysis-only: it does not rewrite/normalize the expression.
pub fn match_add_by_const<'expr, 'name>(
    expr: &'expr ASTNode,
    var_name: &'name str,
    step: i64,
    allow_commutative_add: bool,
) -> Option<SelfUpdateByConstView<'name>> {
    let bin = match_binop(expr, BinaryOperator::Add)?;

    if matches!(match_var(bin.left), Some(name) if name == var_name) && match_int(bin.right) == Some(step) {
        return Some(SelfUpdateByConstView {
            var_name,
            op: SelfUpdateOp::Add,
            step,
            commute: Commute::AsWritten,
        });
    }

    if allow_commutative_add
        && match_int(bin.left) == Some(step)
        && matches!(match_var(bin.right), Some(name) if name == var_name)
    {
        return Some(SelfUpdateByConstView {
            var_name,
            op: SelfUpdateOp::Add,
            step,
            commute: Commute::Swapped,
        });
    }

    None
}

/// Match `var_name - step` only (non-commutative).
pub fn match_sub_by_const<'expr, 'name>(
    expr: &'expr ASTNode,
    var_name: &'name str,
    step: i64,
) -> Option<SelfUpdateByConstView<'name>> {
    let bin = match_binop(expr, BinaryOperator::Subtract)?;

    if matches!(match_var(bin.left), Some(name) if name == var_name) && match_int(bin.right) == Some(step) {
        return Some(SelfUpdateByConstView {
            var_name,
            op: SelfUpdateOp::Sub,
            step,
            commute: Commute::AsWritten,
        });
    }

    None
}

/// Match `var_name + step` (commutative if allowed) or `var_name - step` (non-commutative).
pub fn match_self_update_by_const<'expr, 'name>(
    expr: &'expr ASTNode,
    var_name: &'name str,
    step: i64,
    allow_commutative_add: bool,
) -> Option<SelfUpdateByConstView<'name>> {
    match_add_by_const(expr, var_name, step, allow_commutative_add)
        .or_else(|| match_sub_by_const(expr, var_name, step))
}

fn blockexpr_prelude_is_allowed(prelude_stmts: &[ASTNode]) -> bool {
    prelude_stmts.iter().all(|stmt| {
        matches!(stmt, ASTNode::Local { .. } | ASTNode::Assignment { .. } | ASTNode::BlockExpr { .. })
    })
}

fn collect_self_update_assign_matches<'a>(
    node: &'a ASTNode,
    target_var: &'a str,
    step: i64,
    allow_commutative_add: bool,
    out: &mut Vec<SelfUpdateAssignByConstView<'a>>,
) {
    if let Some((name, rhs)) = match_assignment_to_var(node) {
        if name == target_var {
            if let Some(rhs_view) =
                match_self_update_by_const(rhs, target_var, step, allow_commutative_add)
            {
                out.push(SelfUpdateAssignByConstView {
                    target_var,
                    rhs: rhs_view,
                });
                return;
            }
        }
    }

    if let Some(block) = match_blockexpr(node) {
        if !blockexpr_prelude_is_allowed(block.prelude_stmts) {
            return;
        }
        for stmt in block.prelude_stmts {
            collect_self_update_assign_matches(stmt, target_var, step, allow_commutative_add, out);
        }
        collect_self_update_assign_matches(block.tail_expr, target_var, step, allow_commutative_add, out);
    }
}

/// Find exactly one `target_var = target_var ± step` update in `stmts`.
///
/// - Supports `BlockExpr` wrappers conservatively (see `blockexpr_prelude_is_allowed`).
/// - Does not recurse into general control-flow nodes (If/Loop) except for BlockExpr parts.
pub fn find_single_self_update_assign_by_const<'a>(
    stmts: &'a [ASTNode],
    target_var: &'a str,
    step: i64,
    allow_commutative_add: bool,
) -> Option<SelfUpdateAssignByConstView<'a>> {
    let mut matches: Vec<SelfUpdateAssignByConstView<'a>> = Vec::new();
    for stmt in stmts {
        collect_self_update_assign_matches(stmt, target_var, step, allow_commutative_add, &mut matches);
        if matches.len() > 1 {
            return None;
        }
    }
    matches.pop()
}

fn collect_self_update_assign_matches_any_target<'a>(
    node: &'a ASTNode,
    delta: i64,
    allow_commutative_add: bool,
    out: &mut Vec<SelfUpdateAssignByConstView<'a>>,
) {
    if let Some((target_var, rhs)) = match_assignment_to_var(node) {
        let rhs_view = match delta {
            1 => match_add_by_const(rhs, target_var, 1, allow_commutative_add),
            -1 => match_sub_by_const(rhs, target_var, 1),
            _ => None,
        };
        if let Some(rhs_view) = rhs_view {
            out.push(SelfUpdateAssignByConstView {
                target_var,
                rhs: rhs_view,
            });
            return;
        }
    }

    if let Some(block) = match_blockexpr(node) {
        if !blockexpr_prelude_is_allowed(block.prelude_stmts) {
            return;
        }
        for stmt in block.prelude_stmts {
            collect_self_update_assign_matches_any_target(stmt, delta, allow_commutative_add, out);
        }
        collect_self_update_assign_matches_any_target(block.tail_expr, delta, allow_commutative_add, out);
    }
}

/// Find exactly one `x = x + 1` (delta=1) or `x = x - 1` (delta=-1) update in `stmts`,
/// where the target variable name is discovered from the assignment itself.
pub fn find_single_self_update_assign_by_const_any_target<'a>(
    stmts: &'a [ASTNode],
    delta: i64,
    allow_commutative_add: bool,
) -> Option<SelfUpdateAssignByConstView<'a>> {
    let mut matches: Vec<SelfUpdateAssignByConstView<'a>> = Vec::new();
    for stmt in stmts {
        collect_self_update_assign_matches_any_target(stmt, delta, allow_commutative_add, &mut matches);
        if matches.len() > 1 {
            return None;
        }
    }
    matches.pop()
}
