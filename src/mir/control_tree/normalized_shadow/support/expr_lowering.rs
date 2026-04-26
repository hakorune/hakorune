//! Shared expression lowering facade for normalized-shadow routes.
//!
//! Route lowerers and the legacy entry path import this module instead of
//! owning duplicate assignment/compare lowering logic.

use std::collections::BTreeMap;

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
use crate::mir::control_tree::step_tree::AstNodeHandle;
use crate::mir::join_ir::lowering::error_tags;
use crate::mir::join_ir::{BinOpKind, CompareOp, ConstValue, JoinInst, MirLikeInst};
use crate::mir::ValueId;

pub(crate) fn lower_assign_stmt(
    target: &Option<String>,
    value_ast: &Option<AstNodeHandle>,
    body: &mut Vec<JoinInst>,
    next_value_id: &mut u32,
    env: &mut BTreeMap<String, ValueId>,
) -> Result<(), String> {
    let target_name = target
        .as_ref()
        .ok_or_else(|| "[phase128/assign/target] Assign target must be a variable".to_string())?;

    let value_ast = value_ast
        .as_ref()
        .ok_or_else(|| "[phase128/assign/value] Assign value AST is missing".to_string())?;

    match value_ast.0.as_ref() {
        ASTNode::Literal {
            value: LiteralValue::Integer(i),
            ..
        } => {
            let dst_vid = ValueId(*next_value_id);
            *next_value_id += 1;

            body.push(JoinInst::Compute(MirLikeInst::Const {
                dst: dst_vid,
                value: ConstValue::Integer(*i),
            }));

            env.insert(target_name.clone(), dst_vid);
            Ok(())
        }
        ASTNode::Variable { name, .. } => {
            let src_vid = env.get(name).copied().ok_or_else(|| {
                error_tags::freeze_with_hint(
                    "phase130/assign/var/rhs_missing",
                    &format!("RHS variable '{name}' not found in env"),
                    "ensure the variable is defined before assignment (in writes or inputs)",
                )
            })?;

            env.insert(target_name.clone(), src_vid);
            Ok(())
        }
        ASTNode::BinaryOp {
            operator,
            left,
            right,
            ..
        } => {
            if !matches!(operator, BinaryOperator::Add) {
                return Err(error_tags::freeze_with_hint(
                    "phase130/assign/add/unsupported_op",
                    &format!("Phase 130 only supports Add operator, got {:?}", operator),
                    "use x = x + <literal> pattern or wait for future phases",
                ));
            }

            let lhs_var = match &**left {
                ASTNode::Variable { name, .. } => name.clone(),
                _ => {
                    return Err(error_tags::freeze_with_hint(
                        "phase130/assign/add/lhs_not_var",
                        "Phase 130 Add: LHS must be a variable",
                        "use pattern x = x + <literal>",
                    ));
                }
            };

            let rhs_int = match &**right {
                ASTNode::Literal {
                    value: LiteralValue::Integer(i),
                    ..
                } => *i,
                _ => {
                    return Err(error_tags::freeze_with_hint(
                        "phase130/assign/add/rhs_not_int_literal",
                        "Phase 130 Add: RHS must be integer literal",
                        "use pattern x = x + <literal>",
                    ));
                }
            };

            if target_name != &lhs_var {
                return Err(error_tags::freeze_with_hint(
                    "phase130/assign/add/dst_neq_lhs",
                    &format!(
                        "Phase 130 Add: dst '{}' must equal lhs '{}' (x = x + 3 pattern)",
                        target_name, lhs_var
                    ),
                    "use pattern x = x + <literal> where dst == lhs",
                ));
            }

            let lhs_vid = env.get(&lhs_var).copied().ok_or_else(|| {
                error_tags::freeze_with_hint(
                    "phase130/assign/add/lhs_missing",
                    &format!("Add LHS variable '{}' not found in env", lhs_var),
                    "ensure the variable is defined before the add operation",
                )
            })?;

            let rhs_vid = ValueId(*next_value_id);
            *next_value_id += 1;
            body.push(JoinInst::Compute(MirLikeInst::Const {
                dst: rhs_vid,
                value: ConstValue::Integer(rhs_int),
            }));

            let result_vid = ValueId(*next_value_id);
            *next_value_id += 1;
            body.push(JoinInst::Compute(MirLikeInst::BinOp {
                dst: result_vid,
                op: BinOpKind::Add,
                lhs: lhs_vid,
                rhs: rhs_vid,
            }));

            env.insert(target_name.clone(), result_vid);
            Ok(())
        }
        _ => Err(format!(
            "[phase130/assign/unsupported] Phase 130 supports: int literal, variable, or x = x + <int literal>  Hint: Use supported pattern or wait for future phases"
        )),
    }
}

pub(crate) fn parse_minimal_compare(ast: &ASTNode) -> Result<(String, CompareOp, i64), String> {
    match ast {
        ASTNode::BinaryOp {
            operator,
            left,
            right,
            ..
        } => {
            let var_name = match &**left {
                ASTNode::Variable { name, .. } => name.clone(),
                _ => {
                    return Err(format!(
                        "[phase123/if/compare_lhs_unsupported] Phase 123 only supports Variable on left side of comparison. Hint: Use simple variable comparison or wait for Phase 124"
                    ));
                }
            };

            let int_value = match &**right {
                ASTNode::Literal {
                    value: LiteralValue::Integer(i),
                    ..
                } => *i,
                _ => {
                    return Err(format!(
                        "[phase123/if/compare_rhs_unsupported] Phase 123 only supports Integer literal on right side of comparison. Hint: Use integer literal or wait for Phase 124"
                    ));
                }
            };

            let compare_op = match operator {
                BinaryOperator::Equal => CompareOp::Eq,
                BinaryOperator::NotEqual => CompareOp::Ne,
                BinaryOperator::Less => CompareOp::Lt,
                BinaryOperator::LessEqual => CompareOp::Le,
                BinaryOperator::Greater => CompareOp::Gt,
                BinaryOperator::GreaterEqual => CompareOp::Ge,
                _ => {
                    return Err(format!(
                        "[phase123/if/compare_op_unsupported] Phase 123 only supports comparison operators (==, !=, <, <=, >, >=). Hint: Use comparison operator or wait for Phase 124"
                    ));
                }
            };

            Ok((var_name, compare_op, int_value))
        }
        _ => Err(format!(
            "[phase123/if/cond_unsupported] Phase 123 only supports binary comparisons. Hint: Use simple comparison (var == literal) or wait for Phase 124"
        )),
    }
}
