use crate::ast::{ASTNode, BinaryOperator, UnaryOperator};
use crate::mir::join_ir::{BinOpKind, CompareOp, JoinInst, MirLikeInst, UnaryOp};
use crate::mir::ValueId;

use super::super::condition_env::ConditionEnv;
use super::super::loop_body_local_env::LoopBodyLocalEnv; // Phase 92 P2-2: Body-local support
use super::super::user_method_policy::UserMethodPolicy;
use super::value_expr::{lower_literal, lower_value_expression};

/// Recursive helper for condition lowering
///
/// Handles all supported AST node types and emits appropriate JoinIR instructions.
///
/// # Phase 92 P2-2
///
/// Added `body_local_env` parameter to support body-local variable resolution.
///
/// # Phase 252
///
/// Added `current_static_box_name` parameter to support `this.method(...)` calls
/// in static box method conditions.
pub(super) fn lower_condition_recursive(
    cond_ast: &ASTNode,
    alloc_value: &mut dyn FnMut() -> ValueId,
    env: &ConditionEnv,
    body_local_env: Option<&LoopBodyLocalEnv>, // Phase 92 P2-2
    current_static_box_name: Option<&str>, // Phase 252
    instructions: &mut Vec<JoinInst>,
) -> Result<ValueId, String> {
    match cond_ast {
        // Comparison operations: <, ==, !=, <=, >=, >
        ASTNode::BinaryOp {
            operator,
            left,
            right,
            ..
        } => match operator {
            BinaryOperator::Less
            | BinaryOperator::Equal
            | BinaryOperator::NotEqual
            | BinaryOperator::LessEqual
            | BinaryOperator::GreaterEqual
            | BinaryOperator::Greater => lower_comparison(
                operator,
                left,
                right,
                alloc_value,
                env,
                body_local_env,
                current_static_box_name,
                instructions,
            ),
            BinaryOperator::And => lower_logical_and(
                left,
                right,
                alloc_value,
                env,
                body_local_env,
                current_static_box_name,
                instructions,
            ),
            BinaryOperator::Or => lower_logical_or(
                left,
                right,
                alloc_value,
                env,
                body_local_env,
                current_static_box_name,
                instructions,
            ),
            _ => Err(format!(
                "Unsupported binary operator in condition: {:?}",
                operator
            )),
        },

        // Unary NOT operator
        ASTNode::UnaryOp {
            operator: UnaryOperator::Not,
            operand,
            ..
        } => lower_not_operator(
            operand,
            alloc_value,
            env,
            body_local_env,
            current_static_box_name,
            instructions,
        ),

        // Phase 92 P2-2: Variables - resolve from ConditionEnv or LoopBodyLocalEnv
        ASTNode::Variable { name, .. } => {
            // Priority 1: ConditionEnv (loop parameters, captured variables)
            if let Some(value_id) = env.get(name) {
                return Ok(value_id);
            }
            // Priority 2: LoopBodyLocalEnv (body-local variables like `ch`)
            if let Some(body_env) = body_local_env {
                if let Some(value_id) = body_env.get(name) {
                    return Ok(value_id);
                }
            }
            Err(format!(
                "Variable '{}' not found in ConditionEnv or LoopBodyLocalEnv",
                name
            ))
        }

        // Literals - emit as constants
        ASTNode::Literal { value, .. } => lower_literal(value, alloc_value, instructions),

        // Phase 252: MethodCall support (this.method or builtin methods)
        ASTNode::MethodCall {
            object,
            method,
            arguments,
            ..
        } => {
            // Check if this is a me/this.method(...) call
            match object.as_ref() {
                ASTNode::Me { .. } | ASTNode::This { .. } => {
                    // me/this.method(...) - requires current_static_box_name
                    let box_name = current_static_box_name.ok_or_else(|| {
                        format!(
                            "this.{}(...) requires current_static_box_name (not in static box context)",
                            method
                        )
                    })?;

                    // Check if method is allowed in condition context via UserMethodPolicy
                    if !UserMethodPolicy::allowed_in_condition(box_name, method) {
                        return Err(format!(
                            "User-defined method not allowed in loop condition: {}.{}() (not whitelisted)",
                            box_name, method
                        ));
                    }

                    // Lower arguments using lower_for_init whitelist
                    // (Arguments are value expressions, not conditions, so we use init whitelist)
                    let mut arg_vals = Vec::new();
                    for arg_ast in arguments {
                        let arg_val = lower_value_expression(
                            arg_ast,
                            alloc_value,
                            env,
                            body_local_env,
                            current_static_box_name,
                            instructions,
                        )?;
                        arg_vals.push(arg_val);
                    }

                    // Emit BoxCall instruction
                    let dst = alloc_value();
                    instructions.push(JoinInst::Compute(MirLikeInst::BoxCall {
                        dst: Some(dst),
                        box_name: box_name.to_string(),
                        method: method.clone(),
                        args: arg_vals,
                    }));

                    Ok(dst)
                }
                _ => {
                    // Not this.method - treat as value expression (builtin methods via CoreMethodId)
                    lower_value_expression(
                        object,
                        alloc_value,
                        env,
                        body_local_env,
                        current_static_box_name,
                        instructions,
                    )?;
                    Err(format!(
                        "MethodCall on non-this object not yet supported in condition: {:?}",
                        cond_ast
                    ))
                }
            }
        }

        _ => Err(format!("Unsupported AST node in condition: {:?}", cond_ast)),
    }
}

/// Lower a comparison operation (e.g., `i < end`)
fn lower_comparison(
    operator: &BinaryOperator,
    left: &ASTNode,
    right: &ASTNode,
    alloc_value: &mut dyn FnMut() -> ValueId,
    env: &ConditionEnv,
    body_local_env: Option<&LoopBodyLocalEnv>, // Phase 92 P2-2
    current_static_box_name: Option<&str>, // Phase 252
    instructions: &mut Vec<JoinInst>,
) -> Result<ValueId, String> {
    // Lower left and right sides
    let lhs = lower_value_expression(
        left,
        alloc_value,
        env,
        body_local_env,
        current_static_box_name,
        instructions,
    )?;
    let rhs = lower_value_expression(
        right,
        alloc_value,
        env,
        body_local_env,
        current_static_box_name,
        instructions,
    )?;
    let dst = alloc_value();

    let cmp_op = match operator {
        BinaryOperator::Less => CompareOp::Lt,
        BinaryOperator::Equal => CompareOp::Eq,
        BinaryOperator::NotEqual => CompareOp::Ne,
        BinaryOperator::LessEqual => CompareOp::Le,
        BinaryOperator::GreaterEqual => CompareOp::Ge,
        BinaryOperator::Greater => CompareOp::Gt,
        _ => unreachable!(),
    };

    // Emit Compare instruction
    instructions.push(JoinInst::Compute(MirLikeInst::Compare {
        dst,
        op: cmp_op,
        lhs,
        rhs,
    }));

    Ok(dst)
}

/// Lower logical AND operation (e.g., `a && b`)
fn lower_logical_and(
    left: &ASTNode,
    right: &ASTNode,
    alloc_value: &mut dyn FnMut() -> ValueId,
    env: &ConditionEnv,
    body_local_env: Option<&LoopBodyLocalEnv>, // Phase 92 P2-2
    current_static_box_name: Option<&str>, // Phase 252
    instructions: &mut Vec<JoinInst>,
) -> Result<ValueId, String> {
    // Logical AND: evaluate both sides and combine
    let lhs = lower_condition_recursive(
        left,
        alloc_value,
        env,
        body_local_env,
        current_static_box_name,
        instructions,
    )?;
    let rhs = lower_condition_recursive(
        right,
        alloc_value,
        env,
        body_local_env,
        current_static_box_name,
        instructions,
    )?;
    let dst = alloc_value();

    // Emit BinOp And instruction
    instructions.push(JoinInst::Compute(MirLikeInst::BinOp {
        dst,
        op: BinOpKind::And,
        lhs,
        rhs,
    }));

    Ok(dst)
}

/// Lower logical OR operation (e.g., `a || b`)
fn lower_logical_or(
    left: &ASTNode,
    right: &ASTNode,
    alloc_value: &mut dyn FnMut() -> ValueId,
    env: &ConditionEnv,
    body_local_env: Option<&LoopBodyLocalEnv>, // Phase 92 P2-2
    current_static_box_name: Option<&str>, // Phase 252
    instructions: &mut Vec<JoinInst>,
) -> Result<ValueId, String> {
    // Logical OR: evaluate both sides and combine
    let lhs = lower_condition_recursive(
        left,
        alloc_value,
        env,
        body_local_env,
        current_static_box_name,
        instructions,
    )?;
    let rhs = lower_condition_recursive(
        right,
        alloc_value,
        env,
        body_local_env,
        current_static_box_name,
        instructions,
    )?;
    let dst = alloc_value();

    // Emit BinOp Or instruction
    instructions.push(JoinInst::Compute(MirLikeInst::BinOp {
        dst,
        op: BinOpKind::Or,
        lhs,
        rhs,
    }));

    Ok(dst)
}

/// Lower NOT operator (e.g., `!cond`)
fn lower_not_operator(
    operand: &ASTNode,
    alloc_value: &mut dyn FnMut() -> ValueId,
    env: &ConditionEnv,
    body_local_env: Option<&LoopBodyLocalEnv>, // Phase 92 P2-2
    current_static_box_name: Option<&str>, // Phase 252
    instructions: &mut Vec<JoinInst>,
) -> Result<ValueId, String> {
    let operand_val = lower_condition_recursive(
        operand,
        alloc_value,
        env,
        body_local_env,
        current_static_box_name,
        instructions,
    )?;
    let dst = alloc_value();

    // Emit UnaryOp Not instruction
    instructions.push(JoinInst::Compute(MirLikeInst::UnaryOp {
        dst,
        op: UnaryOp::Not,
        operand: operand_val,
    }));

    Ok(dst)
}
