use crate::ast::ASTNode;
use crate::mir::join_ir::lowering::condition_env::ConditionEnv;
use crate::mir::join_ir::lowering::condition_lowerer::lower_value_expression;
use crate::mir::join_ir::{CompareOp, ConstValue, JoinInst, MirLikeInst};
use crate::mir::ValueId;

pub(super) fn extract_loop_condition<F>(
    cond: &ASTNode,
    alloc_value: &mut F,
    cond_env: &ConditionEnv,
) -> Result<(String, CompareOp, Option<ValueId>, ValueId, Vec<JoinInst>), String>
where
    F: FnMut() -> ValueId,
{
    use crate::mir::join_ir::lowering::condition_pattern::{normalize_comparison, ConditionValue};

    if let Some(norm) = normalize_comparison(cond) {
        let var_name = norm.left_var;
        let op = match norm.op {
            crate::mir::CompareOp::Lt => CompareOp::Lt,
            crate::mir::CompareOp::Gt => CompareOp::Gt,
            crate::mir::CompareOp::Le => CompareOp::Le,
            crate::mir::CompareOp::Ge => CompareOp::Ge,
            crate::mir::CompareOp::Eq => CompareOp::Eq,
            crate::mir::CompareOp::Ne => CompareOp::Ne,
        };

        let mut limit_instructions = Vec::new();
        let limit_value = match norm.right {
            ConditionValue::Literal(lit) => {
                let val_id = alloc_value();
                limit_instructions.push(JoinInst::Compute(MirLikeInst::Const {
                    dst: val_id,
                    value: ConstValue::Integer(lit),
                }));
                val_id
            }
            ConditionValue::Variable(var_name) => {
                let var_node = ASTNode::Variable {
                    name: var_name,
                    span: crate::ast::Span {
                        start: 0,
                        end: 0,
                        line: 1,
                        column: 1,
                    },
                };
                lower_value_expression(
                    &var_node,
                    alloc_value,
                    cond_env,
                    None,
                    None,
                    &mut limit_instructions,
                )?
            }
        };

        return Ok((var_name, op, None, limit_value, limit_instructions));
    }

    match cond {
        ASTNode::BinaryOp {
            operator,
            left,
            right,
            ..
        } => {
            use crate::ast::BinaryOperator;

            let op = match operator {
                BinaryOperator::Less => CompareOp::Lt,
                BinaryOperator::Greater => CompareOp::Gt,
                BinaryOperator::LessEqual => CompareOp::Le,
                BinaryOperator::GreaterEqual => CompareOp::Ge,
                BinaryOperator::Equal => CompareOp::Eq,
                BinaryOperator::NotEqual => CompareOp::Ne,
                _ => {
                    return Err(format!(
                        "[if-sum] Unsupported operator in condition: {:?}",
                        operator
                    ))
                }
            };

            let var_name = extract_base_variable(left);

            let (lhs_val_opt, mut instructions) = match left.as_ref() {
                ASTNode::Variable { name, .. } if name == &var_name => (None, Vec::new()),
                _ => {
                    let mut insts = Vec::new();
                    let lhs = lower_value_expression(
                        left,
                        alloc_value,
                        cond_env,
                        None,
                        None,
                        &mut insts,
                    )?;
                    (Some(lhs), insts)
                }
            };

            let rhs_val = lower_value_expression(
                right,
                alloc_value,
                cond_env,
                None,
                None,
                &mut instructions,
            )?;

            Ok((var_name, op, lhs_val_opt, rhs_val, instructions))
        }
        _ => Err("[if-sum] Expected comparison in condition".to_string()),
    }
}

fn extract_base_variable(expr: &ASTNode) -> String {
    match expr {
        ASTNode::Variable { name, .. } => name.clone(),
        ASTNode::BinaryOp { left, .. } => extract_base_variable(left),
        _ => String::new(),
    }
}

pub(super) fn extract_if_condition<F>(
    if_stmt: &ASTNode,
    alloc_value: &mut F,
    cond_env: &ConditionEnv,
) -> Result<(String, CompareOp, Option<ValueId>, ValueId, Vec<JoinInst>), String>
where
    F: FnMut() -> ValueId,
{
    match if_stmt {
        ASTNode::If { condition, .. } => extract_loop_condition(condition, alloc_value, cond_env),
        _ => Err("[if-sum] Expected If statement".to_string()),
    }
}

pub(super) fn extract_then_update(if_stmt: &ASTNode) -> Result<(String, ASTNode), String> {
    match if_stmt {
        ASTNode::If { then_body, .. } => {
            for stmt in then_body {
                if let ASTNode::Assignment { target, value, .. } = stmt {
                    let target_name = extract_variable_name(&**target)?;
                    if let ASTNode::BinaryOp {
                        operator: crate::ast::BinaryOperator::Add,
                        left,
                        right,
                        ..
                    } = value.as_ref()
                    {
                        let lhs_name = extract_variable_name(left)?;
                        if lhs_name == target_name {
                            return Ok((target_name, right.as_ref().clone()));
                        }
                    }
                }
            }
            Err("[if-sum] No valid accumulator update found in then block".to_string())
        }
        _ => Err("[if-sum] Expected If statement".to_string()),
    }
}

pub(super) fn extract_unconditional_update(body: &[ASTNode], update_var: &str) -> Option<ASTNode> {
    for stmt in body {
        if matches!(stmt, ASTNode::If { .. }) {
            continue;
        }

        if let ASTNode::Assignment { target, value, .. } = stmt {
            if let Ok(target_name) = extract_variable_name(&**target) {
                if target_name == update_var {
                    if let ASTNode::BinaryOp {
                        operator: crate::ast::BinaryOperator::Add,
                        left,
                        right,
                        ..
                    } = value.as_ref()
                    {
                        if let Ok(lhs_name) = extract_variable_name(left) {
                            if lhs_name == target_name {
                                return Some(right.as_ref().clone());
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

pub(super) fn extract_counter_update(
    body: &[ASTNode],
    loop_var: &str,
) -> Result<(String, i64), String> {
    for stmt in body {
        if let ASTNode::Assignment { target, value, .. } = stmt {
            if let Ok(target_name) = extract_variable_name(&**target) {
                if target_name == loop_var {
                    if let ASTNode::BinaryOp {
                        operator: crate::ast::BinaryOperator::Add,
                        left,
                        right,
                        ..
                    } = value.as_ref()
                    {
                        let lhs_name = extract_variable_name(left)?;
                        if lhs_name == target_name {
                            let step = extract_integer_literal(right)?;
                            return Ok((target_name, step));
                        }
                    }
                }
            }
        }
    }
    Err(format!(
        "[if-sum] No counter update found for '{}'",
        loop_var
    ))
}

fn extract_variable_name(node: &ASTNode) -> Result<String, String> {
    match node {
        ASTNode::Variable { name, .. } => Ok(name.clone()),
        _ => Err(format!("[if-sum] Expected variable, got {:?}", node)),
    }
}

fn extract_integer_literal(node: &ASTNode) -> Result<i64, String> {
    match node {
        ASTNode::Literal {
            value: crate::ast::LiteralValue::Integer(n),
            ..
        } => Ok(*n),
        _ => Err(format!("[if-sum] Expected integer literal, got {:?}", node)),
    }
}
